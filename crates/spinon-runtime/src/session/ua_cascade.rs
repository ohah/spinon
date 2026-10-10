use spinon_core::{EnvironmentRevision, HostDocumentSnapshot, StyleRevision};
use spinon_style::{CascadeDiagnostic, ComputedStyleSnapshot, CssMediaEnvironment, CssViewport};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

mod author_stylesheets;
#[cfg(test)]
mod author_stylesheets_edge_tests;
#[cfg(test)]
mod author_stylesheets_tests;
#[cfg(test)]
mod block_paint_tests;
mod cache;
#[cfg(test)]
mod cache_benchmark;
#[cfg(test)]
mod cache_tests;
mod calculation;
mod environment;
#[cfg(test)]
mod incremental_benchmark;
mod invalidation;
#[cfg(test)]
mod invalidation_runtime_tests;
#[cfg(test)]
mod invalidation_test_support;
#[cfg(test)]
mod invalidation_tests;
#[cfg(test)]
mod registered_properties_tests;
mod runtime_layout;
use cache::RuntimeCalculationCacheEntry;
#[cfg(test)]
use cache::RuntimeCalculationCacheKey;
use cache::RuntimeStyleCacheEntry;
use calculation::{
    RuntimeCalculation, compute_request, compute_request_for_block_paint,
    compute_request_for_registered_properties_gpu, compute_request_for_runtime_gpu,
};
use environment::RuntimeCssEnvironment;
use runtime_layout::layout_failure;
pub use runtime_layout::{
    RuntimeLayoutCompleted, RuntimeLayoutFailure, RuntimeLayoutFrame, RuntimeLayoutSnapshot,
    RuntimeLayoutState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeUaCascadeState {
    NotConfigured,
    Pending,
    Ready,
    Empty,
    Failed,
}

impl RuntimeUaCascadeState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotConfigured => "not_configured",
            Self::Pending => "pending",
            Self::Ready => "ready",
            Self::Empty => "empty",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeUaCascadeError {
    InvalidEnvironment,
    Closed,
    WorkerUnavailable,
    RevisionExhausted,
}

pub(super) fn elapsed_microseconds(started_at: Instant) -> u128 {
    duration_microseconds(started_at.elapsed())
}

fn duration_microseconds(duration: Duration) -> u128 {
    duration.as_nanos().div_ceil(1_000)
}

impl RuntimeUaCascadeError {
    pub const fn status_code(self) -> i32 {
        match self {
            Self::InvalidEnvironment => -1,
            Self::RevisionExhausted => -1,
            Self::Closed => -6,
            Self::WorkerUnavailable => -7,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeUaCascadeKey {
    pub generation: u64,
    pub document_revision: u64,
    pub render_tree_revision: u64,
    pub style_revision: u64,
    pub environment_revision: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeUaCascadeRoot {
    pub root_node_id: u64,
    pub styles: ComputedStyleSnapshot,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeUaCascadeCompleted {
    pub key: RuntimeUaCascadeKey,
    pub roots: Arc<[RuntimeUaCascadeRoot]>,
    pub computation_duration_us: u128,
    pub cache_hit: bool,
    pub cascade_recomputed_style_elements: u64,
    pub cascade_reused_style_elements: u64,
    pub cascade_context_style_elements: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeUaCascadeSnapshot {
    pub state: RuntimeUaCascadeState,
    pub requested: Option<RuntimeUaCascadeKey>,
    pub completed: Option<Arc<RuntimeUaCascadeCompleted>>,
    pub error: Option<String>,
}

struct WorkRequest {
    key: RuntimeUaCascadeKey,
    snapshot: Arc<HostDocumentSnapshot>,
    viewport: CssViewport,
    previous_snapshot: Option<Arc<HostDocumentSnapshot>>,
    force_full: bool,
    invalidation: Option<invalidation::RuntimeStyleInvalidation>,
    previous_styles: Option<Arc<[RuntimeUaCascadeRoot]>>,
}

type CascadeComputer = dyn Fn(&WorkRequest) -> Result<RuntimeCalculation, String> + Send + Sync;

struct State {
    closing: bool,
    worker_available: bool,
    environment: Option<RuntimeCssEnvironment>,
    document: Option<Arc<HostDocumentSnapshot>>,
    requested: Option<RuntimeUaCascadeKey>,
    pending: Option<WorkRequest>,
    status: RuntimeUaCascadeState,
    completed: Option<Arc<RuntimeUaCascadeCompleted>>,
    error: Option<String>,
    layout_status: RuntimeLayoutState,
    layout_completed: Option<Arc<RuntimeLayoutCompleted>>,
    layout_diagnostics: Vec<CascadeDiagnostic>,
    layout_error: Option<RuntimeLayoutFailure>,
}

impl State {
    fn new() -> Self {
        Self {
            closing: false,
            worker_available: true,
            environment: None,
            document: None,
            requested: None,
            pending: None,
            status: RuntimeUaCascadeState::NotConfigured,
            completed: None,
            error: None,
            layout_status: RuntimeLayoutState::NotConfigured,
            layout_completed: None,
            layout_diagnostics: Vec::new(),
            layout_error: None,
        }
    }
}

struct Shared {
    state: Mutex<State>,
    wake: Condvar,
}

#[derive(Clone)]
pub(super) struct RuntimeUaCascadeHandle {
    shared: Arc<Shared>,
}

pub(super) struct RuntimeUaCascadeCoordinator {
    handle: RuntimeUaCascadeHandle,
    worker: Mutex<Option<JoinHandle<()>>>,
    startup_duration_us: u128,
}

impl RuntimeUaCascadeCoordinator {
    pub(super) fn new() -> Result<Self, String> {
        Self::new_with_computer(Arc::new(compute_request))
    }

    pub(super) fn new_runtime_gpu() -> Result<Self, String> {
        Self::new_with_computer(Arc::new(compute_request_for_runtime_gpu))
    }

    pub(super) fn new_runtime_gpu_registered_properties() -> Result<Self, String> {
        Self::new_with_computer(Arc::new(compute_request_for_registered_properties_gpu))
    }

    pub(super) fn new_runtime_gpu_block_paint() -> Result<Self, String> {
        Self::new_with_computer(Arc::new(compute_request_for_block_paint))
    }

    fn new_with_computer(compute: Arc<CascadeComputer>) -> Result<Self, String> {
        let startup_started = Instant::now();
        let shared = Arc::new(Shared {
            state: Mutex::new(State::new()),
            wake: Condvar::new(),
        });
        let worker_shared = Arc::clone(&shared);
        let (ready_sender, ready_receiver) = mpsc::sync_channel(1);
        let worker = thread::Builder::new()
            .name("spinon-css-cascade".to_owned())
            .spawn(move || worker_loop(worker_shared, ready_sender, compute))
            .map_err(|error| format!("CSS cascade worker 생성 실패: {error}"))?;
        if ready_receiver.recv().is_err() {
            let _ = worker.join();
            return Err("CSS cascade worker 준비 확인에 실패했습니다".to_owned());
        }
        Ok(Self {
            handle: RuntimeUaCascadeHandle { shared },
            worker: Mutex::new(Some(worker)),
            startup_duration_us: elapsed_microseconds(startup_started),
        })
    }

    pub(super) fn handle(&self) -> RuntimeUaCascadeHandle {
        self.handle.clone()
    }

    pub(super) const fn startup_duration_us(&self) -> u128 {
        self.startup_duration_us
    }

    pub(super) fn shutdown(&self) -> Result<(), &'static str> {
        {
            let mut state = lock(&self.handle.shared.state);
            state.closing = true;
            state.pending = None;
        }
        self.handle.shared.wake.notify_all();
        if let Some(worker) = lock(&self.worker).take()
            && worker.join().is_err()
        {
            return Err("CSS cascade worker가 정상 종료되지 않았습니다");
        }
        Ok(())
    }
}

impl Drop for RuntimeUaCascadeCoordinator {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

impl RuntimeUaCascadeHandle {
    pub(super) fn register_document_snapshot(
        &self,
        snapshot: Arc<HostDocumentSnapshot>,
    ) -> Result<(), RuntimeUaCascadeError> {
        let mut state = lock(&self.shared.state);
        if state.closing {
            return Err(RuntimeUaCascadeError::Closed);
        }
        let previous = state.document.replace(Arc::clone(&snapshot));
        request_latest(&self.shared, &mut state, previous, false);
        Ok(())
    }

    pub(super) fn set_environment(
        &self,
        width_css_px: f32,
        height_css_px: f32,
        device_scale_factor: f32,
        media_environment: CssMediaEnvironment,
    ) -> Result<EnvironmentRevision, RuntimeUaCascadeError> {
        let candidate = RuntimeCssEnvironment {
            width_css_px,
            height_css_px,
            device_scale_factor,
            media_environment,
            revision: EnvironmentRevision::INITIAL,
        };
        candidate
            .viewport()
            .validate()
            .map_err(|_| RuntimeUaCascadeError::InvalidEnvironment)?;

        let mut state = lock(&self.shared.state);
        if state.closing {
            return Err(RuntimeUaCascadeError::Closed);
        }
        if !state.worker_available {
            return Err(RuntimeUaCascadeError::WorkerUnavailable);
        }
        if let Some(current) = state.environment
            && current.same_input(candidate)
        {
            return Ok(current.revision);
        }
        let revision = match state.environment {
            None => EnvironmentRevision::INITIAL,
            Some(previous) => previous
                .revision
                .checked_next()
                .ok_or(RuntimeUaCascadeError::RevisionExhausted)?,
        };
        state.environment = Some(RuntimeCssEnvironment {
            revision,
            ..candidate
        });
        request_latest(&self.shared, &mut state, None, true);
        Ok(revision)
    }

    pub(super) fn snapshot(&self) -> RuntimeUaCascadeSnapshot {
        let state = lock(&self.shared.state);
        RuntimeUaCascadeSnapshot {
            state: state.status,
            requested: state.requested,
            completed: state.completed.clone(),
            error: state.error.clone(),
        }
    }
}

fn request_latest(
    shared: &Shared,
    state: &mut State,
    previous_snapshot: Option<Arc<HostDocumentSnapshot>>,
    force_full: bool,
) {
    let (Some(environment), Some(snapshot)) = (state.environment, state.document.as_ref()) else {
        state.status = RuntimeUaCascadeState::NotConfigured;
        state.requested = None;
        state.pending = None;
        state.completed = None;
        state.error = None;
        state.layout_status = RuntimeLayoutState::NotConfigured;
        state.layout_completed = None;
        state.layout_diagnostics.clear();
        state.layout_error = None;
        return;
    };
    let key = key_for(snapshot, environment.revision);
    state.requested = Some(key);
    state.completed = None;
    state.error = None;
    state.layout_status = RuntimeLayoutState::Pending;
    state.layout_completed = None;
    state.layout_diagnostics.clear();
    state.layout_error = None;
    if !state.worker_available {
        state.status = RuntimeUaCascadeState::Failed;
        state.error = Some("CSS cascade worker를 사용할 수 없습니다".to_owned());
        state.layout_status = RuntimeLayoutState::Failed;
        state.layout_error = Some(layout_failure("worker_unavailable", None, None));
        state.pending = None;
        return;
    }
    state.status = RuntimeUaCascadeState::Pending;
    let (previous_snapshot, force_full) = match state.pending.take() {
        Some(pending) => {
            let contiguous = previous_snapshot.as_ref().is_some_and(|previous| {
                pending.snapshot.generation() == previous.generation()
                    && pending.snapshot.document_revision() == previous.document_revision()
            });
            if contiguous {
                (pending.previous_snapshot, force_full || pending.force_full)
            } else {
                (None, true)
            }
        }
        None => (previous_snapshot, force_full),
    };
    state.pending = Some(WorkRequest {
        key,
        snapshot: Arc::clone(snapshot),
        viewport: environment.viewport(),
        previous_snapshot,
        force_full,
        invalidation: None,
        previous_styles: None,
    });
    shared.wake.notify_all();
}

fn key_for(
    snapshot: &HostDocumentSnapshot,
    environment_revision: EnvironmentRevision,
) -> RuntimeUaCascadeKey {
    RuntimeUaCascadeKey {
        generation: snapshot.generation().get(),
        document_revision: snapshot.document_revision().get(),
        render_tree_revision: snapshot.render_tree_revision().get(),
        style_revision: StyleRevision::INITIAL.get(),
        environment_revision: environment_revision.get(),
    }
}

fn worker_loop(shared: Arc<Shared>, ready: mpsc::SyncSender<()>, compute: Arc<CascadeComputer>) {
    let _ = ready.send(());
    let mut cache: Option<RuntimeCalculationCacheEntry> = None;
    let mut style_cache: Option<RuntimeStyleCacheEntry> = None;
    loop {
        let mut request = {
            let mut state = lock(&shared.state);
            while state.pending.is_none() && !state.closing {
                state = shared
                    .wake
                    .wait(state)
                    .unwrap_or_else(PoisonError::into_inner);
            }
            if state.closing {
                return;
            }
            state
                .pending
                .take()
                .expect("대기 중인 cascade 요청이 있어야 합니다")
        };
        request.invalidation = Some(if request.force_full {
            invalidation::RuntimeStyleInvalidation::Full
        } else {
            invalidation::classify_style_invalidation(
                request.previous_snapshot.as_deref(),
                &request.snapshot,
            )
        });
        request.previous_snapshot = None;
        request.previous_styles = style_cache
            .as_ref()
            .and_then(|entry| entry.styles_for_request(&request));
        let cached = cache.as_ref().and_then(|entry| entry.rekey_for(&request));
        if cached.is_none() {
            cache = None;
        }
        if let Some(calculation) = cached {
            publish_result(
                &shared,
                &request,
                Ok(calculation),
                true,
                &mut cache,
                &mut style_cache,
            );
            continue;
        }
        let result = catch_unwind(AssertUnwindSafe(|| compute(&request)));
        match result {
            Ok(computed) => publish_result(
                &shared,
                &request,
                computed,
                false,
                &mut cache,
                &mut style_cache,
            ),
            Err(_) => {
                fail_worker(&shared, "CSS runtime worker 내부에서 panic이 발생했습니다");
                return;
            }
        }
    }
}

fn publish_result(
    shared: &Shared,
    request: &WorkRequest,
    computed: Result<RuntimeCalculation, String>,
    cache_hit: bool,
    cache: &mut Option<RuntimeCalculationCacheEntry>,
    style_cache: &mut Option<RuntimeStyleCacheEntry>,
) {
    let key = request.key;
    let mut state = lock(&shared.state);
    if state.closing || state.requested != Some(key) {
        drop(state);
        shared.wake.notify_all();
        return;
    }
    match computed {
        Ok(calculation) => {
            if !cache_hit && calculation.layout.is_ok() {
                *cache = Some(RuntimeCalculationCacheEntry::new(
                    request,
                    calculation.clone(),
                ));
            }
            *style_cache = RuntimeStyleCacheEntry::new(request, &calculation);
            state.status = if calculation.roots.is_empty() {
                RuntimeUaCascadeState::Empty
            } else {
                RuntimeUaCascadeState::Ready
            };
            state.completed = Some(Arc::new(RuntimeUaCascadeCompleted {
                key,
                roots: Arc::clone(&calculation.roots),
                computation_duration_us: calculation.cascade_duration_us,
                cache_hit,
                cascade_recomputed_style_elements: calculation.cascade_recomputed_style_elements,
                cascade_reused_style_elements: calculation.cascade_reused_style_elements,
                cascade_context_style_elements: calculation.cascade_context_style_elements,
            }));
            state.error = None;
            state.layout_diagnostics = calculation.layout_diagnostics;
            match calculation.layout {
                Ok(layout) => {
                    state.layout_status = if layout.frames.is_empty() {
                        RuntimeLayoutState::Empty
                    } else {
                        RuntimeLayoutState::Ready
                    };
                    let mut layout = layout.clone();
                    layout.cache_hit = cache_hit;
                    state.layout_completed = Some(Arc::new(layout));
                    state.layout_error = None;
                }
                Err(error) => {
                    state.layout_status = RuntimeLayoutState::Failed;
                    state.layout_completed = None;
                    state.layout_error = Some(error);
                }
            }
        }
        Err(error) => {
            *style_cache = None;
            state.status = RuntimeUaCascadeState::Failed;
            state.completed = None;
            state.error = Some(error);
            state.layout_status = RuntimeLayoutState::Failed;
            state.layout_completed = None;
            state.layout_error = Some(layout_failure("cascade_failed", None, None));
        }
    }
    drop(state);
    shared.wake.notify_all();
}

fn fail_worker(shared: &Shared, message: &str) {
    let mut state = lock(&shared.state);
    state.worker_available = false;
    state.pending = None;
    if !state.closing && state.requested.is_some() {
        state.status = RuntimeUaCascadeState::Failed;
        state.completed = None;
        state.error = Some(message.to_owned());
        state.layout_status = RuntimeLayoutState::Failed;
        state.layout_completed = None;
        state.layout_error = Some(layout_failure("worker_failed", None, None));
    }
    drop(state);
    shared.wake.notify_all();
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

#[cfg(test)]
#[path = "ua_cascade/tests.rs"]
mod tests;

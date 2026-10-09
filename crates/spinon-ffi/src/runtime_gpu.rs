use spinon_render::RuntimeRenderSnapshot;
use spinon_render_wgpu::{PendingRuntimeSurface, RuntimeRenderer, RuntimeRendererError};
use spinon_runtime::{RuntimeLayoutState, RuntimeSession, TaskPriority};
use spinon_style::{CssColorScheme, CssMediaEnvironment};
use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

const ERR_ARGUMENT: i32 = -1;
const ERR_TIMEOUT: i32 = -10;
const ERR_LAYOUT: i32 = -11;
const ERR_STALE: i32 = -12;
const RUNTIME_GPU_FIXTURE_SOURCE: &str =
    include_str!("../../../tests/fixtures/css/c04/runtime-css-to-gpu-resize.js");
const RUNTIME_AUTHOR_STYLESHEETS_FIXTURE_SOURCE: &str =
    include_str!("../../../tests/fixtures/css/c04/runtime-author-stylesheets.js");
const RUNTIME_CSS_CUSTOM_PROPERTIES_FIXTURE_SOURCE: &str =
    include_str!("../../../tests/fixtures/css/c05/runtime-custom-properties-app.js");
const RUNTIME_CSS_REGISTERED_PROPERTIES_FIXTURE_SOURCE: &str =
    include_str!("../../../tests/fixtures/css/c05/runtime-registered-properties.js");
const RUNTIME_CSS_RESULT_CACHE_FIXTURE_SOURCE: &str =
    include_str!("../../../tests/fixtures/css/c05/runtime-result-cache.js");

#[repr(C)]
pub struct SpinonRuntimeGpuHost {
    _private: [u8; 0],
}

struct RuntimeGpuHost {
    session: RuntimeSession,
    operation: Mutex<()>,
    presentation: PresentationScenes,
    renderer: Mutex<Option<RuntimeRenderer>>,
    pending_uikit_surface: Mutex<Option<PendingRuntimeSurface>>,
}

struct PublishedRuntimeScene {
    sequence: u64,
    snapshot: Arc<RuntimeRenderSnapshot>,
}

struct PresentationScenes {
    sequence: AtomicU64,
    scene: Mutex<Option<PublishedRuntimeScene>>,
}

impl PresentationScenes {
    fn new() -> Self {
        Self {
            sequence: AtomicU64::new(0),
            scene: Mutex::new(None),
        }
    }

    fn invalidate(&self) -> Result<u64, String> {
        self.sequence
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
                value.checked_add(1)
            })
            .map(|value| value + 1)
            .map_err(|_| "화면 presentation sequence 값이 소진됐습니다".to_owned())
    }

    fn sequence(&self) -> u64 {
        self.sequence.load(Ordering::Acquire)
    }

    fn publish(&self, sequence: u64, snapshot: RuntimeRenderSnapshot) -> bool {
        let mut scene = lock(&self.scene);
        if self.sequence() != sequence {
            return false;
        }
        *scene = Some(PublishedRuntimeScene {
            sequence,
            snapshot: Arc::new(snapshot),
        });
        true
    }

    fn lock_current(&self) -> (u64, MutexGuard<'_, Option<PublishedRuntimeScene>>) {
        let sequence = self.sequence();
        let mut scene = lock(&self.scene);
        if scene
            .as_ref()
            .is_some_and(|published| published.sequence != sequence)
        {
            *scene = None;
        }
        (sequence, scene)
    }
}

impl RuntimeGpuHost {
    fn new() -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu()?;
        Ok((Self::from_session(session), report))
    }

    fn new_registered_properties_fixture() -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu_registered_properties_fixture()?;
        Ok((Self::from_session(session), report))
    }

    fn from_session(session: RuntimeSession) -> Self {
        Self {
            session,
            operation: Mutex::new(()),
            presentation: PresentationScenes::new(),
            renderer: Mutex::new(None),
            pending_uikit_surface: Mutex::new(None),
        }
    }

    fn invalidate_scene(&self) -> Result<u64, String> {
        self.presentation.invalidate()
    }

    fn presentation_sequence(&self) -> u64 {
        self.presentation.sequence()
    }

    /// 지정 시간은 CSS worker 대기에만 적용하며 호출한 V8 작업을 취소하지 않습니다.
    fn wait_and_publish(
        &self,
        sequence: u64,
        layout_timeout_millis: u64,
    ) -> Result<String, (i32, String)> {
        let snapshot = self
            .session
            .wait_for_layout_snapshot(Duration::from_millis(layout_timeout_millis));
        if snapshot.state == RuntimeLayoutState::Pending
            || snapshot.state == RuntimeLayoutState::NotConfigured
        {
            return Err((
                ERR_TIMEOUT,
                format!(
                    "CSS runtime 계산 제한 시간 초과 state={}",
                    snapshot.state.as_str()
                ),
            ));
        }
        if snapshot.state == RuntimeLayoutState::Failed {
            let error = snapshot
                .error
                .map(|error| {
                    format!(
                        "{} node={:?} property={:?}",
                        error.code, error.node_id, error.property
                    )
                })
                .unwrap_or_else(|| "원인을 보고하지 않았습니다".to_owned());
            return Err((ERR_LAYOUT, format!("CSS runtime 장면 생성 실패: {error}")));
        }
        let Some(completed) = snapshot.completed else {
            return Err((
                ERR_LAYOUT,
                "CSS runtime 완료 snapshot이 없습니다".to_owned(),
            ));
        };
        if snapshot.requested != Some(completed.key) {
            return Err((
                ERR_STALE,
                "요청 key와 완료 key가 달라 오래된 장면을 거부했습니다".to_owned(),
            ));
        }
        let Some(scene) = completed.render_snapshot.clone() else {
            return Err((
                ERR_LAYOUT,
                "runtime GPU profile의 장면이 없습니다".to_owned(),
            ));
        };
        if !scene_matches_layout_key(&scene, completed.key) {
            return Err((
                ERR_STALE,
                "runtime render scene revision tuple이 완료 key와 다릅니다".to_owned(),
            ));
        }
        let box_count = scene.boxes().len();
        let render_viewport = scene.viewport_css_px();
        let render_root = scene
            .boxes()
            .first()
            .map(|render_box| {
                let frame = render_box.frame_css_px();
                format!(
                    " render_viewport_css_px={}x{} render_root_css_px={}x{}",
                    render_viewport.width(),
                    render_viewport.height(),
                    frame.width(),
                    frame.height(),
                )
            })
            .unwrap_or_default();
        if !self.presentation.publish(sequence, scene) {
            return Err((
                ERR_STALE,
                "새로운 화면 요청이 도착해 오래된 장면을 거부했습니다".to_owned(),
            ));
        }
        let root_frame = completed
            .frames
            .first()
            .map(|frame| format!(" root_frame_css_px={}x{}", frame.width, frame.height))
            .unwrap_or_default();
        Ok(format!(
            "layout={} boxes={} generation={} document_revision={} render_tree_revision={} style_revision={} environment_revision={} cacheHit={}{}{}",
            snapshot.state.as_str(),
            box_count,
            completed.key.generation,
            completed.key.document_revision,
            completed.key.render_tree_revision,
            completed.key.style_revision,
            completed.key.environment_revision,
            completed.cache_hit,
            root_frame,
            render_root,
        ))
    }

    fn set_environment(
        &self,
        width_css_px: f32,
        height_css_px: f32,
        device_scale_factor: f32,
        dark: bool,
        layout_timeout_millis: u64,
    ) -> Result<String, (i32, String)> {
        if !width_css_px.is_finite()
            || width_css_px <= 0.0
            || !height_css_px.is_finite()
            || height_css_px <= 0.0
            || !device_scale_factor.is_finite()
            || device_scale_factor <= 0.0
            || layout_timeout_millis == 0
        {
            return Err((
                ERR_ARGUMENT,
                "viewport·scale·layout timeout은 유한한 양수여야 합니다".to_owned(),
            ));
        }
        let _operation = lock(&self.operation);
        let sequence = self
            .invalidate_scene()
            .map_err(|error| (ERR_STALE, error))?;
        let mut environment = CssMediaEnvironment::MOBILE;
        environment.color_scheme = if dark {
            CssColorScheme::Dark
        } else {
            CssColorScheme::Light
        };
        let revision = self
            .session
            .set_ua_cascade_environment(
                width_css_px,
                height_css_px,
                device_scale_factor,
                environment,
            )
            .map_err(|error| (ERR_LAYOUT, format!("CSS runtime 환경 갱신 실패: {error:?}")))?;
        let ready = self.wait_and_publish(sequence, layout_timeout_millis)?;
        Ok(format!("environment_revision={} {ready}", revision.get()))
    }

    fn eval(&self, source: &str, layout_timeout_millis: u64) -> Result<String, (i32, String)> {
        if layout_timeout_millis == 0 {
            return Err((
                ERR_ARGUMENT,
                "layout timeout은 0보다 커야 합니다".to_owned(),
            ));
        }
        let _operation = lock(&self.operation);
        let sequence = self
            .invalidate_scene()
            .map_err(|error| (ERR_STALE, error))?;
        let response = self.session.eval(source, TaskPriority::UserVisible);
        let ready = self.wait_and_publish(sequence, layout_timeout_millis);
        match (response.status, ready) {
            (0, Ok(ready)) => Ok(format!("{} {ready}", response.report)),
            (0, Err((status, error))) => Err((status, format!("{} {error}", response.report))),
            (status, Ok(ready)) => Err((status, format!("{} {ready}", response.report))),
            (status, Err((_, error))) => Err((status, format!("{} {error}", response.report))),
        }
    }

    unsafe fn create_android(
        &self,
        native_window: *mut c_void,
        width: u32,
        height: u32,
        backend: u32,
    ) -> Result<String, RuntimeRendererError> {
        let mut renderer_slot = lock(&self.renderer);
        if renderer_slot.is_some() || lock(&self.pending_uikit_surface).is_some() {
            return Err(RuntimeRendererError::InvalidArgument(
                "runtime GPU renderer가 이미 생성되었습니다",
            ));
        }
        let (renderer, report) =
            unsafe { RuntimeRenderer::new_android(native_window, width, height, backend) }?;
        *renderer_slot = Some(renderer);
        Ok(report)
    }

    unsafe fn prepare_uikit_surface(
        &self,
        view: *mut c_void,
        backend: u32,
    ) -> Result<String, RuntimeRendererError> {
        let renderer_slot = match self.renderer.try_lock() {
            Ok(renderer) => renderer,
            Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => {
                return Err(RuntimeRendererError::InvalidArgument(
                    "runtime GPU renderer가 현재 surface를 사용 중입니다",
                ));
            }
        };
        if renderer_slot.is_some() {
            return Err(RuntimeRendererError::InvalidArgument(
                "runtime GPU renderer가 이미 생성되었습니다",
            ));
        }
        drop(renderer_slot);
        let mut pending_slot = lock(&self.pending_uikit_surface);
        if pending_slot.is_some() {
            return Err(RuntimeRendererError::InvalidArgument(
                "UIKit surface가 이미 준비되었습니다",
            ));
        }
        let surface = unsafe { RuntimeRenderer::prepare_uikit_surface(view, backend) }?;
        *pending_slot = Some(surface);
        Ok("UIKit surface가 준비되었습니다".to_owned())
    }

    fn create_uikit(&self, width: u32, height: u32) -> Result<String, RuntimeRendererError> {
        let mut renderer_slot = lock(&self.renderer);
        if renderer_slot.is_some() {
            return Err(RuntimeRendererError::InvalidArgument(
                "runtime GPU renderer가 이미 생성되었습니다",
            ));
        }
        let pending = lock(&self.pending_uikit_surface).take().ok_or(
            RuntimeRendererError::InvalidArgument(
                "UIKit surface가 메인 스레드에서 준비되지 않았습니다",
            ),
        )?;
        let (renderer, report) = RuntimeRenderer::initialize_uikit_surface(pending, width, height)?;
        *renderer_slot = Some(renderer);
        Ok(report)
    }

    fn configure_uikit_surface(&self) -> Result<String, RuntimeRendererError> {
        let mut renderer_slot = lock(&self.renderer);
        let renderer = renderer_slot
            .as_mut()
            .ok_or(RuntimeRendererError::InvalidArgument(
                "runtime GPU renderer가 없습니다",
            ))?;
        renderer.configure_surface();
        Ok("UIKit surface가 메인 스레드에서 구성되었습니다".to_owned())
    }

    fn resize(&self, width: u32, height: u32) -> Result<(), (i32, String)> {
        self.invalidate_scene()
            .map_err(|error| (ERR_STALE, error))?;
        lock(&self.renderer)
            .as_mut()
            .ok_or_else(|| (ERR_LAYOUT, "runtime GPU renderer가 없습니다".to_owned()))?
            .resize(width, height)
            .map_err(|error| {
                let status = if matches!(error, RuntimeRendererError::InvalidArgument(_)) {
                    ERR_ARGUMENT
                } else {
                    ERR_LAYOUT
                };
                (status, error.to_string())
            })
    }

    fn draw(&self) -> Result<String, (i32, String)> {
        let (sequence, scene) = self.presentation.lock_current();
        let snapshot = scene.as_ref().map(|published| published.snapshot.as_ref());
        let mut renderer_slot = lock(&self.renderer);
        let renderer = renderer_slot
            .as_mut()
            .ok_or_else(|| (ERR_LAYOUT, "runtime GPU renderer가 없습니다".to_owned()))?;
        let presented = renderer
            .draw_if(snapshot, || self.presentation_sequence() == sequence)
            .map_err(|error| (ERR_LAYOUT, error.to_string()))?;
        let surface_texture_size = renderer.last_surface_texture_size();
        drop(renderer_slot);
        if !presented {
            return Err((
                ERR_STALE,
                "새 화면 요청이 도착해 이전 장면 제출을 막았습니다".to_owned(),
            ));
        }
        Ok(match scene.as_ref() {
            Some(scene) => {
                let viewport = scene.snapshot.viewport_css_px();
                let root = scene.snapshot.boxes().first().map(|render_box| {
                    let frame = render_box.frame_css_px();
                    format!(" root_css_px={}x{}", frame.width(), frame.height())
                });
                format!(
                    "presented boxes={} environment_revision={} viewport_css_px={}x{} surface_texture={}x{}{}",
                    scene.snapshot.boxes().len(),
                    scene.snapshot.key().environment_revision().get(),
                    viewport.width(),
                    viewport.height(),
                    surface_texture_size.0,
                    surface_texture_size.1,
                    root.unwrap_or_default(),
                )
            }
            None => "presented empty scene while runtime calculation is pending".to_owned(),
        })
    }

    #[cfg(feature = "c04-runtime-gpu-test-hooks")]
    fn inject_next_draw_failure_for_test(&self) -> Result<String, (i32, String)> {
        let mut renderer_slot = lock(&self.renderer);
        let renderer = renderer_slot
            .as_mut()
            .ok_or_else(|| (ERR_LAYOUT, "runtime GPU renderer가 없습니다".to_owned()))?;
        renderer.inject_next_draw_failure_for_test();
        Ok("시험용 다음 draw 실패를 설정했습니다".to_owned())
    }

    fn destroy_renderer(&self) -> Result<(), (i32, String)> {
        self.invalidate_scene()
            .map_err(|error| (ERR_STALE, error))?;
        let renderer = lock(&self.renderer).take();
        let pending = lock(&self.pending_uikit_surface).take();
        drop(renderer);
        drop(pending);
        Ok(())
    }
}

fn scene_matches_layout_key(
    scene: &RuntimeRenderSnapshot,
    key: spinon_runtime::RuntimeUaCascadeKey,
) -> bool {
    let scene_key = scene.key();
    scene_key.generation().get() == key.generation
        && scene_key.document_revision().get() == key.document_revision
        && scene_key.render_tree_revision().get() == key.render_tree_revision
        && scene_key.style_revision().get() == key.style_revision
        && scene_key.environment_revision().get() == key.environment_revision
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

fn raw_host<'a>(host: *mut SpinonRuntimeGpuHost) -> Option<&'a RuntimeGpuHost> {
    if host.is_null() {
        None
    } else {
        Some(unsafe { &*host.cast::<RuntimeGpuHost>() })
    }
}

mod ffi;
#[cfg(test)]
mod tests;

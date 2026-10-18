use crate::root_html::{
    ParsedRootHtml, RootHtmlEnvironmentMetadata, RootHtmlEnvironmentSnapshot,
    resolve_root_html_canvas_background, resolve_root_html_environment,
};
#[cfg(test)]
use crate::v8::{NodeCallback, TextCallback};
use crate::v8::{
    SpinonDocumentCollectionStats, SpinonV8Runtime, spinon_v8_current_thread_id,
    spinon_v8_runtime_notify_memory_pressure, spinon_v8_runtime_terminate,
};
#[cfg(feature = "c14-1-live-v8-fixture-test")]
use spinon_core::HostDocumentSnapshot;
pub use spinon_core::TaskPriority;
use spinon_style::CssColorScheme;
use std::ffi::CString;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

mod actor;
mod priority_probe;
mod report;
mod scheduler;
mod shutdown;
mod trace;
mod ua_cascade;

#[cfg(test)]
use actor::CallbackState;
use actor::actor_loop;
#[cfg(any(target_os = "android", target_os = "ios", test))]
pub use priority_probe::run_priority_fairness_probe;
pub use priority_probe::run_priority_probe;
#[cfg(test)]
use report::{OperationReport, operation_report};
use scheduler::{EnqueueError, TaskScheduler};
pub use shutdown::run_shutdown_probe;
use trace::{TraceSection, finish_reply_handoff};
use ua_cascade::RuntimeUaCascadeCoordinator;
pub use ua_cascade::{
    RuntimeLayoutCompleted, RuntimeLayoutFailure, RuntimeLayoutFrame, RuntimeLayoutSnapshot,
    RuntimeLayoutState, RuntimeUaCascadeCompleted, RuntimeUaCascadeError, RuntimeUaCascadeKey,
    RuntimeUaCascadeRoot, RuntimeUaCascadeSnapshot, RuntimeUaCascadeState,
};

const QUEUE_CAPACITY: usize = 64;
static NEXT_REPLY_TRACE_COOKIE: AtomicU32 = AtomicU32::new(1);
const OK: i32 = 0;
const ERR_ARGUMENT: i32 = -1;
const ERR_JAVASCRIPT: i32 = -4;
const ERR_QUEUE_FULL: i32 = -5;
const ERR_CLOSED: i32 = -6;
const ERR_WORKER: i32 = -7;
const ERR_CANCELLED: i32 = -8;

struct RuntimeControl {
    runtime: Option<usize>,
    active: bool,
    cancel_generation: u64,
    closing: bool,
}

impl RuntimeControl {
    fn new() -> Self {
        Self {
            runtime: None,
            active: false,
            cancel_generation: 0,
            closing: false,
        }
    }
}

pub struct RuntimeSession {
    scheduler: Arc<TaskScheduler>,
    worker: Mutex<Option<JoinHandle<()>>>,
    control: Arc<Mutex<RuntimeControl>>,
    submission: Mutex<()>,
    shutdown_gate: Mutex<()>,
    next_sequence: AtomicU64,
    ua_cascade: RuntimeUaCascadeCoordinator,
    root_html_environment: Option<RootHtmlEnvironmentMetadata>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootHtmlCanvasError {
    Environment(RuntimeUaCascadeError),
    DuplicateColorSchemeMetadata,
    CascadeTimeout,
    CascadeFailed,
    StaleSnapshot,
    MissingRoot,
    MissingRootStyle,
    UnsupportedComputedColorScheme,
    UnsupportedComputedBackgroundColor,
}

impl RootHtmlCanvasError {
    pub const fn status_code(self) -> i32 {
        match self {
            Self::Environment(error) => error.status_code(),
            Self::DuplicateColorSchemeMetadata => -13,
            Self::CascadeTimeout => -10,
            Self::CascadeFailed => -7,
            Self::StaleSnapshot => -12,
            Self::MissingRoot | Self::MissingRootStyle => -11,
            Self::UnsupportedComputedColorScheme => -14,
            Self::UnsupportedComputedBackgroundColor => -15,
        }
    }

    pub const fn reason_code(self) -> &'static str {
        match self {
            Self::Environment(_) => "environment_error",
            Self::DuplicateColorSchemeMetadata => "duplicate_color_scheme_metadata",
            Self::CascadeTimeout => "cascade_timeout",
            Self::CascadeFailed => "cascade_failed",
            Self::StaleSnapshot => "stale_snapshot",
            Self::MissingRoot => "missing_root",
            Self::MissingRootStyle => "missing_root_style",
            Self::UnsupportedComputedColorScheme => "unsupported_computed_color_scheme",
            Self::UnsupportedComputedBackgroundColor => "unsupported_computed_background_color",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RootHtmlCanvasSnapshot {
    pub environment: RootHtmlEnvironmentSnapshot,
    /// HTML root CSS와 metadata에서 선택한 scheme입니다. html/body 배경이 투명할 때 canvas fallback으로 쓰입니다.
    pub used_color_scheme: CssColorScheme,
    /// computed html/body background propagation 또는 scheme fallback으로 선택한 canvas RGBA8입니다.
    pub color_rgba: [u8; 4],
    pub cascade_key: RuntimeUaCascadeKey,
}

#[derive(Clone, Copy)]
enum RuntimeCssProfile {
    Default,
    RootHtmlDocument,
    RootHtmlDefaultRender,
    RuntimeGpu,
    RegisteredPropertiesFixture,
    BlockPaintFixture,
    #[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
    RootHtmlBlockPaintFixture,
    #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
    RootHtmlTextPaintFixture,
    BlockImageFixture,
    BlockFormattingFixture,
    BlockPositioningFixture,
    BlockFlexStackingFixture,
    BlockFlexOverflowClipFixture,
    #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
    C19BoxPaintOpacityFixture,
    #[cfg(feature = "c13-3-gesture-fixture-test")]
    BlockFlexScrollGestureFixture,
    BlockFlexGridFixture,
    BlockFlexGridPlacementFixture,
    BlockFlexGridAutoPlacementFixture,
    BlockFlexGridDenseAutoPlacementFixture,
    BlockFlexGridTemplateFixture,
}

enum Command {
    Eval {
        sequence: u64,
        trace_cookie: u32,
        source: CString,
        caller_thread_id: u64,
        reply: SyncSender<OperationResponse>,
    },
    Dispatch {
        sequence: u64,
        trace_cookie: u32,
        node_id: i32,
        caller_thread_id: u64,
        reply: SyncSender<OperationResponse>,
    },
    #[cfg(feature = "c14-1-live-v8-fixture-test")]
    SnapshotDocumentForC14Fixture {
        reply: SyncSender<Result<HostDocumentSnapshot, String>>,
    },
}

pub struct OperationResponse {
    pub status: i32,
    pub report: String,
}

/// 호스트가 명시적으로 V8에 전달하는 메모리 압박 단계입니다.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryPressureLevel {
    /// 압박이 없다는 상태를 전달합니다. 즉시 GC를 요청하지 않습니다.
    None = 0,
    /// V8의 점진적 회수 휴리스틱을 앞당기도록 알립니다.
    Moderate = 1,
    /// V8에 가능한 빠른 메모리 회수가 필요하다고 알립니다.
    Critical = 2,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn current_thread_id() -> u64 {
    unsafe { spinon_v8_current_thread_id() }
}

fn submit(
    session: &RuntimeSession,
    priority: TaskPriority,
    command: impl FnOnce(u64, u64, u32, SyncSender<OperationResponse>) -> Command,
) -> OperationResponse {
    let _submit_trace = TraceSection::new(c"SpinonR05:runtime-submit");
    let submit_started_at = Instant::now();
    let enqueue_result = {
        let _enqueue_trace = TraceSection::new(c"SpinonR05:runtime-enqueue");
        enqueue(session, priority, command)
    };
    let (receiver, trace_cookie) = match enqueue_result {
        Ok(response) => response,
        Err(response) => return response,
    };
    let response_started_at = Instant::now();
    let enqueue_total_us = response_started_at
        .duration_since(submit_started_at)
        .as_micros();
    let mut response = {
        let _receive_trace = TraceSection::new(c"SpinonR05:reply-receive");
        let received = receiver.recv();
        if received.is_ok() {
            finish_reply_handoff(trace_cookie);
        }
        received.unwrap_or_else(|_| OperationResponse {
            status: ERR_WORKER,
            report: "V8 실행기가 응답하기 전에 종료되었습니다".to_owned(),
        })
    };
    let response_wait_us = response_started_at.elapsed().as_micros();
    let submit_total_us = submit_started_at.elapsed().as_micros();
    let response_report_started_at = Instant::now();
    response.report.push_str(&format!(
        " enqueue_total_us={enqueue_total_us} response_wait_us={response_wait_us} submit_total_us={submit_total_us}"
    ));
    let response_report_append_us = response_report_started_at.elapsed().as_micros();
    response.report.push_str(&format!(
        " response_report_append_us={response_report_append_us}"
    ));
    response
}

fn enqueue(
    session: &RuntimeSession,
    priority: TaskPriority,
    command: impl FnOnce(u64, u64, u32, SyncSender<OperationResponse>) -> Command,
) -> Result<(Receiver<OperationResponse>, u32), OperationResponse> {
    let (reply, response) = {
        let _channel_trace = TraceSection::new(c"SpinonR05:reply-channel-create");
        mpsc::sync_channel(1)
    };
    let submission_lock_started_at = Instant::now();
    let _submission = lock(&session.submission);
    let submission_lock_wait_us = submission_lock_started_at.elapsed().as_micros();
    let control_lock_started_at = Instant::now();
    let control_lock_wait_us;
    {
        let control = lock(&session.control);
        control_lock_wait_us = control_lock_started_at.elapsed().as_micros();
        if control.closing {
            return Err(OperationResponse {
                status: ERR_CLOSED,
                report: "세션이 종료되었습니다".to_owned(),
            });
        }
    }
    let sequence = session.next_sequence.fetch_add(1, Ordering::Relaxed) + 1;
    let trace_cookie = NEXT_REPLY_TRACE_COOKIE.fetch_add(1, Ordering::Relaxed);
    let caller_thread_id = current_thread_id();
    match session.scheduler.try_enqueue(
        priority,
        command(sequence, caller_thread_id, trace_cookie, reply),
        submission_lock_wait_us,
        control_lock_wait_us,
    ) {
        Ok(()) => {}
        Err(EnqueueError::Full) => {
            return Err(OperationResponse {
                status: ERR_QUEUE_FULL,
                report: format!("명령 큐가 가득 찼습니다 capacity={QUEUE_CAPACITY}"),
            });
        }
        Err(EnqueueError::Stopped) => {
            return Err(OperationResponse {
                status: ERR_CLOSED,
                report: "V8 실행기 큐가 종료되었습니다".to_owned(),
            });
        }
    }
    drop(_submission);
    Ok((response, trace_cookie))
}

fn cancel_control(control: &Mutex<RuntimeControl>) -> i32 {
    let mut state = lock(control);
    if !state.active {
        return 1;
    }
    let Some(runtime) = state.runtime else {
        return ERR_WORKER;
    };
    state.cancel_generation = state.cancel_generation.wrapping_add(1);
    unsafe { spinon_v8_runtime_terminate(runtime as *mut SpinonV8Runtime) };
    OK
}

impl RuntimeSession {
    /// V8 Isolate를 만들고 소유 전용 OS 스레드의 준비를 기다립니다.
    pub fn new() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::Default)
    }

    /// UTF-8 루트 HTML을 document mode로 파싱한 뒤 V8 세션을 만듭니다.
    ///
    /// HTML 안의 script는 실행하지 않습니다. 기존 [`Self::new`]는 빈 HostDocument를 유지합니다.
    pub fn new_with_root_html(source: &str) -> Result<(Self, String), String> {
        let parsed = ParsedRootHtml::parse(source)?;
        let environment = RootHtmlEnvironmentMetadata::from_document(&parsed);
        Self::new_with_css_profile_and_root_html(
            RuntimeCssProfile::RootHtmlDocument,
            Some(parsed),
            Some(environment),
        )
    }

    /// 일반 root HTML을 기본 GPU runtime layout·paint 경로에 연결합니다.
    pub fn new_runtime_gpu_with_root_html(source: &str) -> Result<(Self, String), String> {
        let parsed = ParsedRootHtml::parse(source)?;
        let environment = RootHtmlEnvironmentMetadata::from_document(&parsed);
        Self::new_with_css_profile_and_root_html(
            RuntimeCssProfile::RootHtmlDefaultRender,
            Some(parsed),
            Some(environment),
        )
    }

    /// 고정 문서형 Block paint fixture 전용 runtime을 만듭니다.
    #[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
    pub fn new_runtime_gpu_root_html_block_paint_fixture(
        source: &str,
    ) -> Result<(Self, String), String> {
        let parsed = ParsedRootHtml::parse(source)?;
        let environment = RootHtmlEnvironmentMetadata::from_document(&parsed);
        Self::new_with_css_profile_and_root_html(
            RuntimeCssProfile::RootHtmlBlockPaintFixture,
            Some(parsed),
            Some(environment),
        )
    }

    /// 실제 HTML5 HostDocument에서 direct-text glyph paint를 실행하는 opt-in fixture runtime입니다.
    #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
    pub fn new_runtime_gpu_root_html_text_paint_fixture(
        source: &str,
    ) -> Result<(Self, String), String> {
        let parsed = ParsedRootHtml::parse(source)?;
        let environment = RootHtmlEnvironmentMetadata::from_document(&parsed);
        Self::new_with_css_profile_and_root_html(
            RuntimeCssProfile::RootHtmlTextPaintFixture,
            Some(parsed),
            Some(environment),
        )
    }

    /// C04.10 내부 GPU renderer 전용 Flex paint profile로 런타임을 만듭니다.
    pub fn new_runtime_gpu() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::RuntimeGpu)
    }

    /// C05.2 등록 사용자 지정 속성을 사용하는 내부 검증 runtime을 만듭니다.
    pub fn new_runtime_gpu_registered_properties_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::RegisteredPropertiesFixture)
    }

    /// C08 제한 Block·기본 페인트 fixture 전용 runtime을 만듭니다.
    pub fn new_runtime_gpu_block_paint_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockPaintFixture)
    }

    /// C18.3 object-fit fixture 전용 runtime을 만듭니다.
    pub fn new_runtime_gpu_block_image_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockImageFixture)
    }

    /// C09.1 일반 Block 흐름 fixture 전용 runtime을 만듭니다.
    pub fn new_runtime_gpu_block_formatting_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockFormattingFixture)
    }

    /// C12.2 Block absolute positioning fixture 전용 runtime을 만듭니다.
    pub fn new_runtime_gpu_block_positioning_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockPositioningFixture)
    }

    /// C12.4 Block/Flex 쌓임 맥락 fixture 전용 runtime을 만듭니다.
    pub fn new_runtime_gpu_block_flex_stacking_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockFlexStackingFixture)
    }

    /// C13.1 overflow clip fixture 전용 runtime을 만듭니다.
    pub fn new_runtime_gpu_block_flex_overflow_clip_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockFlexOverflowClipFixture)
    }

    /// C19.2 부분 alpha·그룹 투명도 fixture 전용 runtime을 만듭니다.
    #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
    pub fn new_runtime_gpu_c19_2_box_paint_opacity_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::C19BoxPaintOpacityFixture)
    }

    /// C13.3 style와 scroll input policy를 연결하는 내부 fixture runtime을 만듭니다.
    #[cfg(feature = "c13-3-gesture-fixture-test")]
    pub fn new_runtime_gpu_block_flex_scroll_gesture_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockFlexScrollGestureFixture)
    }

    /// C11.1 Grid runtime fixture 전용 CSS profile로 V8 세션을 만듭니다.
    pub fn new_runtime_gpu_block_flex_grid_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockFlexGridFixture)
    }

    /// C11.2.1 numeric Grid placement fixture 전용 CSS profile로 V8 세션을 만듭니다.
    pub fn new_runtime_gpu_block_flex_grid_placement_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockFlexGridPlacementFixture)
    }

    /// C11.2.2 row sparse auto-placement fixture 전용 runtime을 만듭니다.
    pub fn new_runtime_gpu_block_flex_grid_auto_placement_fixture() -> Result<(Self, String), String>
    {
        Self::new_with_css_profile(RuntimeCssProfile::BlockFlexGridAutoPlacementFixture)
    }

    /// C11.2.3 dense·column Grid auto-placement fixture 전용 runtime을 만듭니다.
    pub fn new_runtime_gpu_block_flex_grid_dense_auto_placement_fixture()
    -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockFlexGridDenseAutoPlacementFixture)
    }

    /// C11.3.1 정수 `repeat()` Grid fixture 전용 runtime을 만듭니다.
    pub fn new_runtime_gpu_block_flex_grid_template_fixture() -> Result<(Self, String), String> {
        Self::new_with_css_profile(RuntimeCssProfile::BlockFlexGridTemplateFixture)
    }

    fn new_with_css_profile(profile: RuntimeCssProfile) -> Result<(Self, String), String> {
        Self::new_with_css_profile_and_root_html(profile, None, None)
    }

    fn new_with_css_profile_and_root_html(
        profile: RuntimeCssProfile,
        root_html: Option<ParsedRootHtml>,
        root_html_environment: Option<RootHtmlEnvironmentMetadata>,
    ) -> Result<(Self, String), String> {
        let startup_started = Instant::now();
        let ua_cascade = match profile {
            RuntimeCssProfile::Default => RuntimeUaCascadeCoordinator::new()?,
            RuntimeCssProfile::RootHtmlDocument => {
                RuntimeUaCascadeCoordinator::new_root_html_document()?
            }
            RuntimeCssProfile::RootHtmlDefaultRender => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_root_html_default_render()?
            }
            RuntimeCssProfile::RuntimeGpu => RuntimeUaCascadeCoordinator::new_runtime_gpu()?,
            RuntimeCssProfile::RegisteredPropertiesFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_registered_properties()?
            }
            RuntimeCssProfile::BlockPaintFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_paint()?
            }
            #[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
            RuntimeCssProfile::RootHtmlBlockPaintFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_root_html_block_paint()?
            }
            #[cfg(feature = "c04-12-root-html-text-paint-fixture-test")]
            RuntimeCssProfile::RootHtmlTextPaintFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_root_html_text_paint()?
            }
            RuntimeCssProfile::BlockImageFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_image()?
            }
            RuntimeCssProfile::BlockFormattingFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_formatting()?
            }
            RuntimeCssProfile::BlockPositioningFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_positioning()?
            }
            RuntimeCssProfile::BlockFlexStackingFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_flex_stacking()?
            }
            RuntimeCssProfile::BlockFlexOverflowClipFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_flex_overflow_clip()?
            }
            #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
            RuntimeCssProfile::C19BoxPaintOpacityFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_c19_2_box_paint_opacity()?
            }
            #[cfg(feature = "c13-3-gesture-fixture-test")]
            RuntimeCssProfile::BlockFlexScrollGestureFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_flex_scroll_gesture()?
            }
            RuntimeCssProfile::BlockFlexGridFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_flex_grid()?
            }
            RuntimeCssProfile::BlockFlexGridPlacementFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_flex_grid_placement()?
            }
            RuntimeCssProfile::BlockFlexGridAutoPlacementFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_flex_grid_auto_placement()?
            }
            RuntimeCssProfile::BlockFlexGridDenseAutoPlacementFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_flex_grid_dense_auto_placement()?
            }
            RuntimeCssProfile::BlockFlexGridTemplateFixture => {
                RuntimeUaCascadeCoordinator::new_runtime_gpu_block_flex_grid_template()?
            }
        };
        let css_worker_ready_us = ua_cascade.startup_duration_us();
        let actor_ua_cascade = ua_cascade.handle();
        let control = Arc::new(Mutex::new(RuntimeControl::new()));
        let worker_control = Arc::clone(&control);
        let scheduler = Arc::new(TaskScheduler::new());
        let worker_scheduler = Arc::clone(&scheduler);
        let (ready_sender, ready_receiver) = mpsc::channel();
        let worker = thread::Builder::new()
            .name("spinon-js-runtime".to_owned())
            .spawn(move || {
                actor_loop(
                    worker_scheduler,
                    worker_control,
                    actor_ua_cascade,
                    root_html,
                    ready_sender,
                )
            })
            .map_err(|error| format!("실행기 스레드 생성 실패: {error}"))?;

        let owner_thread_id = match ready_receiver.recv() {
            Ok(Ok(thread_id)) => thread_id,
            Ok(Err(error)) => {
                let _ = worker.join();
                return Err(error);
            }
            Err(_) => {
                let _ = worker.join();
                return Err("V8 실행기 초기화 응답을 받지 못했습니다".to_owned());
            }
        };
        let report = format!(
            "session=ready owner_tid={owner_thread_id} isolate_per_session=1 queue_capacity={QUEUE_CAPACITY} queue_policy=strict-priority-fifo css_worker_per_session=1 css_worker_ready_us={css_worker_ready_us} session_startup_us={}",
            ua_cascade::elapsed_microseconds(startup_started)
        );
        Ok((
            Self {
                scheduler,
                worker: Mutex::new(Some(worker)),
                control,
                submission: Mutex::new(()),
                shutdown_gate: Mutex::new(()),
                next_sequence: AtomicU64::new(0),
                ua_cascade,
                root_html_environment,
            },
            report,
        ))
    }

    /// JavaScript 평가를 지정한 논리 우선순위로 제출하고 완료 응답을 돌려줍니다.
    pub fn eval(&self, source: &str, priority: TaskPriority) -> OperationResponse {
        let source = match CString::new(source) {
            Ok(source) => source,
            Err(_) => {
                return OperationResponse {
                    status: ERR_ARGUMENT,
                    report: "JavaScript 원본에 NUL 문자가 있습니다".to_owned(),
                };
            }
        };
        submit(
            self,
            priority,
            |sequence, caller_thread_id, trace_cookie, reply| Command::Eval {
                sequence,
                trace_cookie,
                source,
                caller_thread_id,
                reply,
            },
        )
    }

    /// 등록된 JavaScript 이벤트 함수를 지정한 우선순위로 호출합니다.
    pub fn dispatch(&self, node_id: i32, priority: TaskPriority) -> OperationResponse {
        submit(
            self,
            priority,
            |sequence, caller_thread_id, trace_cookie, reply| Command::Dispatch {
                sequence,
                trace_cookie,
                node_id,
                caller_thread_id,
                reply,
            },
        )
    }

    /// C14.1 opt-in fixture에서 owner actor가 보유한 최신 문서 snapshot을 복사합니다.
    ///
    /// 이 진단 경로는 공개 제품 API가 아닙니다. V8 actor가 처리한 시점의 snapshot을
    /// 반환하며, actor가 5초 안에 응답하지 않으면 오류를 반환합니다.
    #[cfg(feature = "c14-1-live-v8-fixture-test")]
    pub fn snapshot_document_for_c14_1_fixture(&self) -> Result<HostDocumentSnapshot, String> {
        let receiver = {
            let _submission = lock(&self.submission);
            if lock(&self.control).closing {
                return Err("V8 문서 snapshot 요청 중 세션이 종료되었습니다".to_owned());
            }
            let (reply, receiver) = mpsc::sync_channel(1);
            match self.scheduler.try_enqueue(
                TaskPriority::UserVisible,
                Command::SnapshotDocumentForC14Fixture { reply },
                0,
                0,
            ) {
                Ok(()) => receiver,
                Err(EnqueueError::Full) => {
                    return Err(format!(
                        "V8 문서 snapshot actor queue가 가득 찼습니다 capacity={QUEUE_CAPACITY}"
                    ));
                }
                Err(EnqueueError::Stopped) => {
                    return Err("V8 문서 snapshot actor queue가 종료되었습니다".to_owned());
                }
            }
        };
        match receiver.recv_timeout(Duration::from_secs(5)) {
            Ok(result) => result,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                Err("V8 문서 snapshot actor 응답이 5초를 넘었습니다".to_owned())
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                Err("V8 문서 snapshot actor 응답 채널이 닫혔습니다".to_owned())
            }
        }
    }

    /// 호스트가 정한 압박 단계를 V8에 전달합니다. OS 신호나 판단 시점은 정하지 않습니다.
    ///
    /// 이 호출은 JavaScript 취소나 이벤트 dispatch를 요청하지 않습니다. V8이 안전 지점에서
    /// 회수를 수행할 수 있지만 완료 시점·회수량은 보장하지 않습니다. 종료 중이면
    /// `ERR_CLOSED`를 돌려주며, Isolate 해제 전에 통지가 끝나도록 런타임 상태 잠금을 유지합니다.
    pub fn notify_memory_pressure(&self, level: MemoryPressureLevel) -> i32 {
        let control = lock(&self.control);
        if control.closing {
            return ERR_CLOSED;
        }
        let Some(runtime) = control.runtime else {
            return ERR_CLOSED;
        };
        let status = unsafe {
            spinon_v8_runtime_notify_memory_pressure(runtime as *mut SpinonV8Runtime, level as i32)
        };
        if status == 0 { OK } else { ERR_ARGUMENT }
    }

    /// 실행 중인 JavaScript를 취소합니다. 실행 중인 작업이 없으면 1을 돌려줍니다.
    pub fn cancel(&self) -> i32 {
        cancel_control(&self.control)
    }

    /// Runtime이 사용할 CSS 환경을 명시하고 UA cascade 재계산을 요청합니다.
    pub fn set_ua_cascade_environment(
        &self,
        width_css_px: f32,
        height_css_px: f32,
        device_scale_factor: f32,
        media_environment: spinon_style::CssMediaEnvironment,
    ) -> Result<spinon_core::EnvironmentRevision, RuntimeUaCascadeError> {
        let _submission = lock(&self.submission);
        if lock(&self.control).closing {
            return Err(RuntimeUaCascadeError::Closed);
        }
        self.ua_cascade.handle().set_environment(
            width_css_px,
            height_css_px,
            device_scale_factor,
            media_environment,
        )
    }

    /// Root HTML metadata를 surface 입력에 적용하고 CSS cascade 환경을 갱신합니다.
    ///
    /// Surface 크기는 physical pixel, `device_scale_factor`는 physical pixel per CSS px입니다.
    /// 반환된 `page_scale`은 layout viewport와 분리되어 있으며 현재 renderer transform에 적용되지 않습니다.
    pub fn set_root_html_environment(
        &self,
        surface_width_px: f32,
        surface_height_px: f32,
        device_scale_factor: f32,
        media_environment: spinon_style::CssMediaEnvironment,
    ) -> Result<RootHtmlEnvironmentSnapshot, RuntimeUaCascadeError> {
        let metadata = self
            .root_html_environment
            .ok_or(RuntimeUaCascadeError::InvalidEnvironment)?;
        let mut resolved = resolve_root_html_environment(
            metadata,
            surface_width_px,
            surface_height_px,
            device_scale_factor,
            media_environment,
            spinon_core::EnvironmentRevision::INITIAL,
        )
        .map_err(|_| RuntimeUaCascadeError::InvalidEnvironment)?;
        let revision = self.set_ua_cascade_environment(
            resolved.viewport_width_css_px,
            resolved.viewport_height_css_px,
            resolved.device_scale_factor,
            resolved.media_environment,
        )?;
        resolved.environment_revision = revision;
        Ok(resolved)
    }

    /// 지정한 루트 HTML 환경의 최신 root computed style을 기다려 Canvas 색을 선택합니다.
    pub fn resolve_root_html_canvas(
        &self,
        surface_width_px: f32,
        surface_height_px: f32,
        device_scale_factor: f32,
        media_environment: spinon_style::CssMediaEnvironment,
        timeout: Duration,
    ) -> Result<RootHtmlCanvasSnapshot, RootHtmlCanvasError> {
        let environment = self
            .set_root_html_environment(
                surface_width_px,
                surface_height_px,
                device_scale_factor,
                media_environment,
            )
            .map_err(RootHtmlCanvasError::Environment)?;
        self.resolve_root_html_canvas_for_environment(environment, timeout)
    }

    /// 이미 적용한 루트 HTML 환경의 최신 root computed style에서 Canvas 색을 선택합니다.
    pub fn resolve_root_html_canvas_for_environment(
        &self,
        environment: RootHtmlEnvironmentSnapshot,
        timeout: Duration,
    ) -> Result<RootHtmlCanvasSnapshot, RootHtmlCanvasError> {
        let metadata = self
            .root_html_environment
            .ok_or(RootHtmlCanvasError::Environment(
                RuntimeUaCascadeError::InvalidEnvironment,
            ))?;
        if metadata.has_duplicate_color_scheme_metadata() {
            return Err(RootHtmlCanvasError::DuplicateColorSchemeMetadata);
        }

        let snapshot = self.ua_cascade.handle().wait_for_snapshot(timeout);
        if snapshot.state == RuntimeUaCascadeState::Pending {
            return Err(RootHtmlCanvasError::CascadeTimeout);
        }
        if snapshot.state == RuntimeUaCascadeState::Failed {
            return Err(RootHtmlCanvasError::CascadeFailed);
        }
        let completed = snapshot.completed.ok_or(RootHtmlCanvasError::MissingRoot)?;
        if snapshot.requested != Some(completed.key)
            || completed.key.environment_revision != environment.environment_revision.get()
        {
            return Err(RootHtmlCanvasError::StaleSnapshot);
        }
        if completed.roots.len() != 1 {
            return Err(RootHtmlCanvasError::MissingRoot);
        }
        let root = &completed.roots[0];
        let viewport = root.styles.viewport;
        if root.styles.generation.get() != completed.key.generation
            || root.styles.document_revision.get() != completed.key.document_revision
            || root.styles.render_tree_revision.get() != completed.key.render_tree_revision
            || root.styles.style_revision != completed.key.style_revision
            || viewport.environment_revision.get() != completed.key.environment_revision
            || viewport.width_css_px.to_bits() != environment.viewport_width_css_px.to_bits()
            || viewport.height_css_px.to_bits() != environment.viewport_height_css_px.to_bits()
            || viewport.device_scale_factor.to_bits() != environment.device_scale_factor.to_bits()
            || viewport.media_environment != environment.media_environment
        {
            return Err(RootHtmlCanvasError::StaleSnapshot);
        }
        let root_style = root
            .styles
            .elements
            .iter()
            .find(|element| element.node_id.get() == root.root_node_id)
            .ok_or(RootHtmlCanvasError::MissingRootStyle)?;
        let computed_root_value = root_style
            .properties
            .get("color-scheme")
            .ok_or(RootHtmlCanvasError::MissingRootStyle)?;
        let used_color_scheme = metadata
            .resolve_canvas_color_scheme(computed_root_value, environment.prefers_color_scheme)
            .ok_or(RootHtmlCanvasError::UnsupportedComputedColorScheme)?;
        let html_background = root_style
            .background_paint
            .ok_or(RootHtmlCanvasError::MissingRootStyle)?;
        let body_background = root
            .body_node_id
            .map(|body_node_id| {
                root.styles
                    .elements
                    .iter()
                    .find(|element| element.node_id.get() == body_node_id)
                    .and_then(|element| element.background_paint)
                    .ok_or(RootHtmlCanvasError::MissingRootStyle)
            })
            .transpose()?;
        let color_rgba = resolve_root_html_canvas_background(
            html_background,
            body_background,
            used_color_scheme,
        )
        .ok_or(RootHtmlCanvasError::UnsupportedComputedBackgroundColor)?;

        Ok(RootHtmlCanvasSnapshot {
            environment,
            used_color_scheme,
            color_rgba,
            cascade_key: completed.key,
        })
    }

    /// 최신 UA cascade 상태를 한 번에 복사합니다. Stylo 계산 완료는 기다리지 않습니다.
    pub fn ua_cascade_snapshot(&self) -> RuntimeUaCascadeSnapshot {
        self.ua_cascade.handle().snapshot()
    }

    #[cfg(feature = "c12-4-worker-stack-residency-test")]
    pub fn css_worker_stack_residency_sample_json_for_test(&self) -> Result<String, String> {
        self.ua_cascade
            .handle()
            .worker_stack_residency_sample_json()
    }

    /// 최신 runtime layout 결과를 한 번에 복사합니다. 계산 완료는 기다리지 않습니다.
    pub fn layout_snapshot(&self) -> RuntimeLayoutSnapshot {
        self.ua_cascade.handle().layout_snapshot()
    }

    /// Runtime GPU host가 background executor에서 최신 layout 결과를 제한 시간 동안 기다립니다.
    pub fn wait_for_layout_snapshot(&self, timeout: Duration) -> RuntimeLayoutSnapshot {
        self.ua_cascade.handle().wait_for_layout_snapshot(timeout)
    }
}

impl Drop for RuntimeSession {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}

#[cfg(test)]
mod tests;

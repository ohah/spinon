#[cfg(feature = "c13-3-gesture-fixture-test")]
use fixtures::c13_3_scroll_gesture_host::C13_3GestureState;
use spinon_render::RuntimeRenderSnapshot;
use spinon_render_wgpu::{PendingRuntimeSurface, RuntimeRenderer, RuntimeRendererError};
#[cfg(feature = "c13-1-overflow-clip-fixture-test")]
use spinon_runtime::RuntimeLayoutFrame;
use spinon_runtime::{RuntimeLayoutState, RuntimeSession, TaskPriority};
use spinon_style::{CssColorScheme, CssMediaEnvironment};
#[cfg(feature = "c13-3-gesture-fixture-test")]
use std::sync::Arc;
#[cfg(any(
    feature = "c13-2-scroll-fixture-test",
    feature = "c18-1-image-paint-fixture-test",
    feature = "c18-2-alpha-image-compositing-fixture-test",
    feature = "c18-3-object-fit-position-fixture-test",
    feature = "c18-4-image-fixture-test"
))]
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::Duration;

mod presentation;
mod renderer;
use presentation::PresentationScenes;

#[cfg(feature = "c04-runtime-gpu-fixture-test")]
mod fixtures;
#[cfg(feature = "c04-runtime-gpu-fixture-test")]
pub(crate) use fixtures::c11::{c11_2_1_grid_placement_runtime_source, c11_grid_runtime_scene};
#[cfg(feature = "c04-runtime-gpu-fixture-test")]
pub(crate) use fixtures::c11_2_2::c11_2_2_grid_auto_placement_runtime_source;
#[cfg(feature = "c11-2-3-grid-fixture-test")]
pub(crate) use fixtures::c11_2_3::c11_2_3_grid_auto_placement_runtime_source;
#[cfg(feature = "c04-runtime-gpu-fixture-test")]
pub(crate) use fixtures::c11_3_1::c11_3_1_grid_repeat_runtime_source;
#[cfg(feature = "c04-runtime-gpu-fixture-test")]
pub(crate) use fixtures::c11_3_2::c11_3_2_grid_named_line_runtime_source;
#[cfg(feature = "c04-runtime-gpu-fixture-test")]
pub(crate) use fixtures::c11_3_3::c11_3_3_grid_template_areas_runtime_source;
#[cfg(feature = "c04-runtime-gpu-fixture-test")]
pub(crate) use fixtures::data::*;

const ERR_ARGUMENT: i32 = -1;
const ERR_TIMEOUT: i32 = -10;
const ERR_LAYOUT: i32 = -11;
const ERR_STALE: i32 = -12;
#[cfg(feature = "c13-1-overflow-clip-fixture-test")]
const C13_CHROME_HIT_TEST_ORACLE: &str = include_str!(
    "../../../spec/internal/evidence/c13-1-overflow-clip-android-avd-2026-10-14/chrome155-runtime-observation.json"
);
#[cfg(feature = "c13-1-overflow-clip-fixture-test")]
const C13_BORDER_HIT_TEST_ORACLE: &str = include_str!(
    "../../../spec/internal/evidence/c13-1-overflow-clip-paint-hit-border-2026-10-14/oracle-fixture-observation.json"
);
#[repr(C)]
pub struct SpinonRuntimeGpuHost {
    _private: [u8; 0],
}

struct RuntimeGpuHost {
    session: RuntimeSession,
    include_node_frame_report: bool,
    include_paint_order_report: bool,
    #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
    include_c13_1_hit_test_report: bool,
    #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
    include_c13_1_border_hit_test_report: bool,
    #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
    include_c13_1_compact_surface_matrix_hit_test_report: bool,
    #[cfg(any(
        feature = "c12-4-metal-surface-readback-test",
        feature = "c11-grid-metal-surface-pixels-test",
        feature = "c13-1-overflow-clip-fixture-test",
        feature = "c13-2-scroll-fixture-test",
        feature = "c18-1-image-paint-fixture-test",
        feature = "c18-2-alpha-image-compositing-fixture-test",
        feature = "c18-3-object-fit-position-fixture-test",
        feature = "c18-4-image-fixture-test",
        feature = "c19-2-box-paint-opacity-fixture-test",
        feature = "c15-3-wgpu-glyph-paint-fixture-test"
    ))]
    surface_pixel_readback_test: bool,
    #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
    c19_2_android_surface_readback_test: bool,
    #[cfg(feature = "c18-1-image-paint-fixture-test")]
    c18_1_image_paint_fixture: bool,
    #[cfg(feature = "c18-1-image-paint-fixture-test")]
    c18_1_image_document_ready: AtomicBool,
    #[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
    c18_2_alpha_image_fixture: bool,
    #[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
    c18_2_image_document_ready: AtomicBool,
    #[cfg(feature = "c18-3-object-fit-position-fixture-test")]
    c18_3_object_fit_position_fixture: bool,
    #[cfg(feature = "c18-3-object-fit-position-fixture-test")]
    c18_3_image_document_ready: AtomicBool,
    #[cfg(feature = "c18-4-image-fixture-test")]
    c18_4_async_image_fixture: bool,
    #[cfg(feature = "c18-4-image-fixture-test")]
    c18_4_image_document_ready: AtomicBool,
    #[cfg(feature = "c13-2-scroll-fixture-test")]
    c13_2_scroll_state: Mutex<Option<RuntimeRenderSnapshot>>,
    #[cfg(feature = "c13-2-scroll-fixture-test")]
    c13_2_fail_next_scroll_reserve: AtomicBool,
    #[cfg(feature = "c13-2-scroll-fixture-test")]
    c13_2_reserve_failure_observed: AtomicBool,
    #[cfg(feature = "c13-3-gesture-fixture-test")]
    c13_3_gesture_state: Option<Arc<C13_3GestureState>>,
    #[cfg(feature = "c15-3-wgpu-glyph-paint-fixture-test")]
    c15_3_glyph_paint_fixture: bool,
    operation: Mutex<()>,
    presentation: PresentationScenes,
    renderer: Mutex<Option<RuntimeRenderer>>,
    pending_uikit_surface: Mutex<Option<PendingRuntimeSurface>>,
}

struct WaitAndPublishOptions {
    include_node_frame_report: bool,
    #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
    include_c13_1_hit_test_report: bool,
    #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
    include_c13_1_border_hit_test_report: bool,
    #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
    include_c13_1_compact_surface_matrix_hit_test_report: bool,
    #[cfg(feature = "c18-1-image-paint-fixture-test")]
    attach_c18_1_images: bool,
    #[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
    attach_c18_2_image: bool,
    #[cfg(feature = "c18-3-object-fit-position-fixture-test")]
    attach_c18_3_images: bool,
    #[cfg(feature = "c18-4-image-fixture-test")]
    attach_c18_4_images: bool,
}

impl RuntimeGpuHost {
    fn new() -> Result<(Self, String), String> {
        let (session, report) = RuntimeSession::new_runtime_gpu()?;
        Ok((Self::from_session(session), report))
    }

    fn from_session(session: RuntimeSession) -> Self {
        Self::from_session_with_frame_report_mode(session, false)
    }

    #[cfg(feature = "c04-runtime-gpu-fixture-test")]
    fn from_session_with_frame_report(session: RuntimeSession) -> Self {
        Self::from_session_with_frame_report_mode(session, true)
    }

    fn from_session_with_frame_report_mode(
        session: RuntimeSession,
        include_node_frame_report: bool,
    ) -> Self {
        Self {
            session,
            include_node_frame_report,
            include_paint_order_report: false,
            #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
            include_c13_1_hit_test_report: false,
            #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
            include_c13_1_border_hit_test_report: false,
            #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
            include_c13_1_compact_surface_matrix_hit_test_report: false,
            #[cfg(any(
                feature = "c12-4-metal-surface-readback-test",
                feature = "c11-grid-metal-surface-pixels-test",
                feature = "c13-1-overflow-clip-fixture-test",
                feature = "c13-2-scroll-fixture-test",
                feature = "c18-1-image-paint-fixture-test",
                feature = "c18-2-alpha-image-compositing-fixture-test",
                feature = "c18-3-object-fit-position-fixture-test",
                feature = "c18-4-image-fixture-test",
                feature = "c19-2-box-paint-opacity-fixture-test",
                feature = "c15-3-wgpu-glyph-paint-fixture-test"
            ))]
            surface_pixel_readback_test: false,
            #[cfg(feature = "c19-2-box-paint-opacity-fixture-test")]
            c19_2_android_surface_readback_test: false,
            #[cfg(feature = "c18-1-image-paint-fixture-test")]
            c18_1_image_paint_fixture: false,
            #[cfg(feature = "c18-1-image-paint-fixture-test")]
            c18_1_image_document_ready: AtomicBool::new(false),
            #[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
            c18_2_alpha_image_fixture: false,
            #[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
            c18_2_image_document_ready: AtomicBool::new(false),
            #[cfg(feature = "c18-3-object-fit-position-fixture-test")]
            c18_3_object_fit_position_fixture: false,
            #[cfg(feature = "c18-3-object-fit-position-fixture-test")]
            c18_3_image_document_ready: AtomicBool::new(false),
            #[cfg(feature = "c18-4-image-fixture-test")]
            c18_4_async_image_fixture: false,
            #[cfg(feature = "c18-4-image-fixture-test")]
            c18_4_image_document_ready: AtomicBool::new(false),
            #[cfg(feature = "c13-2-scroll-fixture-test")]
            c13_2_scroll_state: Mutex::new(None),
            #[cfg(feature = "c13-2-scroll-fixture-test")]
            c13_2_fail_next_scroll_reserve: AtomicBool::new(false),
            #[cfg(feature = "c13-2-scroll-fixture-test")]
            c13_2_reserve_failure_observed: AtomicBool::new(false),
            #[cfg(feature = "c13-3-gesture-fixture-test")]
            c13_3_gesture_state: None,
            #[cfg(feature = "c15-3-wgpu-glyph-paint-fixture-test")]
            c15_3_glyph_paint_fixture: false,
            operation: Mutex::new(()),
            presentation: PresentationScenes::new(),
            renderer: Mutex::new(None),
            pending_uikit_surface: Mutex::new(None),
        }
    }

    fn invalidate_scene(&self) -> Result<u64, String> {
        self.presentation.invalidate()
    }

    fn begin_presentation_update(&self) -> Result<u64, String> {
        let sequence = self.invalidate_scene()?;
        #[cfg(feature = "c13-3-gesture-fixture-test")]
        if let Some(state) = &self.c13_3_gesture_state
            && let Ok(snapshot) = state.current_snapshot()
            && !self.presentation.publish(sequence, snapshot)
        {
            return Err(
                "C13.3 presentation 갱신 뒤 gesture snapshot 복구가 stale입니다".to_owned(),
            );
        }
        Ok(sequence)
    }

    fn presentation_sequence(&self) -> u64 {
        self.presentation.sequence()
    }

    /// 지정 시간은 CSS worker 대기에만 적용하며 호출한 V8 작업을 취소하지 않습니다.
    fn wait_and_publish(
        &self,
        sequence: u64,
        layout_timeout_millis: u64,
        options: WaitAndPublishOptions,
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
            let cascade_error = self.session.ua_cascade_snapshot().error;
            let detail = cascade_error
                .map(|message| format!(" cascade_error={message}"))
                .unwrap_or_default();
            return Err((
                ERR_LAYOUT,
                format!("CSS runtime 장면 생성 실패: {error}{detail}"),
            ));
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
        let cascade_snapshot = self.session.ua_cascade_snapshot();
        let Some(cascade_completed) = cascade_snapshot
            .completed
            .filter(|cascade| cascade.key == completed.key)
        else {
            return Err((
                ERR_STALE,
                "layout과 같은 revision의 cascade 결과가 없습니다".to_owned(),
            ));
        };
        let Some(scene) = completed.render_snapshot.clone() else {
            return Err((
                ERR_LAYOUT,
                "runtime GPU profile의 장면이 없습니다".to_owned(),
            ));
        };
        #[cfg(feature = "c18-1-image-paint-fixture-test")]
        let (scene, c18_1_image_report) = if options.attach_c18_1_images {
            let document = self
                .session
                .snapshot_document_for_c14_1_fixture()
                .map_err(|error| {
                    (
                        ERR_LAYOUT,
                        format!("C18.1 V8 document snapshot 실패: {error}"),
                    )
                })?;
            fixtures::c18_1::attach_images(&document, scene).map_err(|error| (ERR_LAYOUT, error))?
        } else {
            (scene, String::new())
        };
        #[cfg(not(feature = "c18-1-image-paint-fixture-test"))]
        let (scene, c18_1_image_report) = (scene, String::new());
        #[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
        let (scene, c18_2_image_report) = if options.attach_c18_2_image {
            let document = self
                .session
                .snapshot_document_for_c14_1_fixture()
                .map_err(|error| {
                    (
                        ERR_LAYOUT,
                        format!("C18.2 V8 document snapshot 실패: {error}"),
                    )
                })?;
            fixtures::c18_2::attach_image(&document, scene).map_err(|error| (ERR_LAYOUT, error))?
        } else {
            (scene, String::new())
        };
        #[cfg(not(feature = "c18-2-alpha-image-compositing-fixture-test"))]
        let (scene, c18_2_image_report) = (scene, String::new());
        #[cfg(feature = "c18-3-object-fit-position-fixture-test")]
        let (scene, c18_3_image_report) = if options.attach_c18_3_images {
            let document = self
                .session
                .snapshot_document_for_c14_1_fixture()
                .map_err(|error| {
                    (
                        ERR_LAYOUT,
                        format!("C18.3 V8 document snapshot 실패: {error}"),
                    )
                })?;
            fixtures::c18_3::attach_images(&document, scene, &cascade_completed.roots)
                .map_err(|error| (ERR_LAYOUT, error))?
        } else {
            (scene, String::new())
        };
        #[cfg(not(feature = "c18-3-object-fit-position-fixture-test"))]
        let (scene, c18_3_image_report) = (scene, String::new());
        #[cfg(feature = "c18-4-image-fixture-test")]
        let (scene, c18_4_image_report) = if options.attach_c18_4_images {
            let document = self
                .session
                .snapshot_document_for_c14_1_fixture()
                .map_err(|error| {
                    (
                        ERR_LAYOUT,
                        format!("C18.4 V8 document snapshot 실패: {error}"),
                    )
                })?;
            fixtures::c18_4::attach_images(&document, scene, &cascade_completed.roots)
                .map_err(|error| (ERR_LAYOUT, error))?
        } else {
            (scene, String::new())
        };
        #[cfg(not(feature = "c18-4-image-fixture-test"))]
        let (scene, c18_4_image_report) = (scene, String::new());
        #[cfg(feature = "c13-2-scroll-fixture-test")]
        let scene = self.reapply_c13_2_scroll_state(scene)?;
        #[cfg(feature = "c13-3-gesture-fixture-test")]
        let scene = self.reapply_c13_3_scroll_state(scene)?;
        #[cfg(feature = "c15-3-wgpu-glyph-paint-fixture-test")]
        let (scene, c15_3_glyph_paint_report) =
            if self.c15_3_glyph_paint_fixture && !scene.boxes().is_empty() {
                fixtures::c15_3::attach_glyph_paint(scene).map_err(|error| (ERR_LAYOUT, error))?
            } else {
                (scene, String::new())
            };
        #[cfg(not(feature = "c15-3-wgpu-glyph-paint-fixture-test"))]
        let (scene, c15_3_glyph_paint_report) = (scene, String::new());
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
        let paint_order_report = if self.include_paint_order_report {
            let nodes = scene
                .boxes()
                .iter()
                .map(|render_box| format!("{}:{}", render_box.paint_order(), render_box.node_id()))
                .collect::<Vec<_>>()
                .join(",");
            format!(" paint_node_order=[{nodes}]")
        } else {
            String::new()
        };
        #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
        let hit_test_report = if options.include_c13_1_compact_surface_matrix_hit_test_report {
            let samples = fixtures::c13_1_compact_surface_matrix::hit_test_samples_json(
                &scene,
                &completed.frames,
            )
            .map_err(|error| (ERR_LAYOUT, error))?;
            let hit_identity_grid =
                fixtures::c13_1_compact_surface_matrix::hit_identity_grid_summary_json(
                    &scene,
                    &completed.frames,
                )
                .map_err(|error| (ERR_LAYOUT, error))?;
            format!(
                " hit_test_samples_json={samples} hit_identity_grid_summary_json={hit_identity_grid}"
            )
        } else if options.include_c13_1_hit_test_report {
            let samples = c13_1_hit_test_samples_json(&scene, &completed.frames)
                .map_err(|error| (ERR_LAYOUT, error))?;
            format!(" hit_test_samples_json={samples}")
        } else if options.include_c13_1_border_hit_test_report {
            let samples = c13_1_border_hit_test_samples_json(&scene, &completed.frames)
                .map_err(|error| (ERR_LAYOUT, error))?;
            format!(" hit_test_samples_json={samples}")
        } else {
            String::new()
        };
        #[cfg(not(feature = "c13-1-overflow-clip-fixture-test"))]
        let hit_test_report = String::new();
        if !self.presentation.publish(sequence, scene.clone()) {
            return Err((
                ERR_STALE,
                "새로운 화면 요청이 도착해 오래된 장면을 거부했습니다".to_owned(),
            ));
        }
        #[cfg(feature = "c18-1-image-paint-fixture-test")]
        if options.attach_c18_1_images {
            self.c18_1_image_document_ready
                .store(true, std::sync::atomic::Ordering::Release);
        }
        #[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
        if options.attach_c18_2_image {
            self.c18_2_image_document_ready
                .store(true, std::sync::atomic::Ordering::Release);
        }
        #[cfg(feature = "c18-3-object-fit-position-fixture-test")]
        if options.attach_c18_3_images {
            self.c18_3_image_document_ready
                .store(true, std::sync::atomic::Ordering::Release);
        }
        #[cfg(feature = "c18-4-image-fixture-test")]
        if options.attach_c18_4_images {
            self.c18_4_image_document_ready
                .store(true, std::sync::atomic::Ordering::Release);
        }
        #[cfg(feature = "c13-3-gesture-fixture-test")]
        if let Some(state) = &self.c13_3_gesture_state
            && state.ready_snapshot().is_some()
        {
            state.publish_snapshot(scene.clone());
        }
        #[cfg(feature = "c13-2-scroll-fixture-test")]
        {
            let mut scroll_state = lock(&self.c13_2_scroll_state);
            if scroll_state.is_some() {
                *scroll_state = Some(scene);
            }
        }
        let root_frame = completed
            .frames
            .first()
            .map(|frame| format!(" root_frame_css_px={}x{}", frame.width, frame.height))
            .unwrap_or_default();
        let node_frame_report = if options.include_node_frame_report {
            let frames = completed
                .frames
                .iter()
                .enumerate()
                .map(|(preorder, frame)| {
                    format!(
                        "{preorder}:node={},x={},y={},width={},height={}",
                        frame.node_id, frame.x, frame.y, frame.width, frame.height
                    )
                })
                .collect::<Vec<_>>()
                .join(";");
            format!(" node_frames_css_px=[{frames}]")
        } else {
            String::new()
        };
        Ok(format!(
            "layout={} boxes={} generation={} document_revision={} render_tree_revision={} style_revision={} environment_revision={} cacheHit={} cascadeRecomputedStyleElements={} cascadeReusedStyleElements={} cascadeContextStyleElements={}{}{}{}{}{}{}{}{}{}{}",
            snapshot.state.as_str(),
            box_count,
            completed.key.generation,
            completed.key.document_revision,
            completed.key.render_tree_revision,
            completed.key.style_revision.get(),
            completed.key.environment_revision,
            completed.cache_hit,
            cascade_completed.cascade_recomputed_style_elements,
            cascade_completed.cascade_reused_style_elements,
            cascade_completed.cascade_context_style_elements,
            root_frame,
            render_root,
            node_frame_report,
            paint_order_report,
            hit_test_report,
            c18_1_image_report,
            c18_2_image_report,
            c18_3_image_report,
            c18_4_image_report,
            c15_3_glyph_paint_report,
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
        #[cfg(feature = "c18-1-image-paint-fixture-test")]
        let attach_c18_1_images = self.c18_1_image_paint_fixture
            && self
                .c18_1_image_document_ready
                .load(std::sync::atomic::Ordering::Acquire);
        #[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
        let attach_c18_2_image = self.c18_2_alpha_image_fixture
            && self
                .c18_2_image_document_ready
                .load(std::sync::atomic::Ordering::Acquire);
        #[cfg(feature = "c18-3-object-fit-position-fixture-test")]
        let attach_c18_3_images = self.c18_3_object_fit_position_fixture
            && self
                .c18_3_image_document_ready
                .load(std::sync::atomic::Ordering::Acquire);
        #[cfg(feature = "c18-4-image-fixture-test")]
        let attach_c18_4_images = self.c18_4_async_image_fixture
            && self
                .c18_4_image_document_ready
                .load(std::sync::atomic::Ordering::Acquire);
        let ready = self.wait_and_publish(
            sequence,
            layout_timeout_millis,
            WaitAndPublishOptions {
                include_node_frame_report: self.include_node_frame_report,
                #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
                include_c13_1_hit_test_report: false,
                #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
                include_c13_1_border_hit_test_report: false,
                #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
                include_c13_1_compact_surface_matrix_hit_test_report: false,
                #[cfg(feature = "c18-1-image-paint-fixture-test")]
                attach_c18_1_images,
                #[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
                attach_c18_2_image,
                #[cfg(feature = "c18-3-object-fit-position-fixture-test")]
                attach_c18_3_images,
                #[cfg(feature = "c18-4-image-fixture-test")]
                attach_c18_4_images,
            },
        )?;
        Ok(format!("environment_revision={} {ready}", revision.get()))
    }

    fn eval(&self, source: &str, layout_timeout_millis: u64) -> Result<String, (i32, String)> {
        self.eval_with_node_frame_report(
            source,
            layout_timeout_millis,
            self.include_node_frame_report,
        )
    }

    fn eval_with_node_frame_report(
        &self,
        source: &str,
        layout_timeout_millis: u64,
        include_node_frame_report: bool,
    ) -> Result<String, (i32, String)> {
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
        let ready = self.wait_and_publish(
            sequence,
            layout_timeout_millis,
            WaitAndPublishOptions {
                include_node_frame_report,
                #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
                include_c13_1_hit_test_report: self.include_c13_1_hit_test_report,
                #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
                include_c13_1_border_hit_test_report: self.include_c13_1_border_hit_test_report,
                #[cfg(feature = "c13-1-overflow-clip-fixture-test")]
                include_c13_1_compact_surface_matrix_hit_test_report: self
                    .include_c13_1_compact_surface_matrix_hit_test_report,
                #[cfg(feature = "c18-1-image-paint-fixture-test")]
                attach_c18_1_images: self.c18_1_image_paint_fixture,
                #[cfg(feature = "c18-2-alpha-image-compositing-fixture-test")]
                attach_c18_2_image: self.c18_2_alpha_image_fixture,
                #[cfg(feature = "c18-3-object-fit-position-fixture-test")]
                attach_c18_3_images: self.c18_3_object_fit_position_fixture,
                #[cfg(feature = "c18-4-image-fixture-test")]
                attach_c18_4_images: self.c18_4_async_image_fixture,
            },
        );
        match (response.status, ready) {
            (0, Ok(ready)) => Ok(format!("{} {ready}", response.report)),
            (0, Err((status, error))) => Err((status, format!("{} {error}", response.report))),
            (status, Ok(ready)) => Err((status, format!("{} {ready}", response.report))),
            (status, Err((_, error))) => Err((status, format!("{} {error}", response.report))),
        }
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
        && scene_key.style_revision() == key.style_revision
        && scene_key.environment_revision().get() == key.environment_revision
}

#[cfg(feature = "c13-1-overflow-clip-fixture-test")]
fn c13_1_hit_test_samples_json(
    scene: &RuntimeRenderSnapshot,
    frames: &[RuntimeLayoutFrame],
) -> Result<String, String> {
    let oracle: serde_json::Value = serde_json::from_str(C13_CHROME_HIT_TEST_ORACLE)
        .map_err(|error| format!("pinned C13.1 Chrome oracle JSON 읽기 실패: {error}"))?;
    if oracle["browser"]["version"].as_str() != Some("155.0.8059.40") {
        return Err("C13.1 hit-test oracle Chrome version이 고정값과 다릅니다".to_owned());
    }
    let oracle_frames = oracle["frames"]
        .as_array()
        .ok_or_else(|| "C13.1 hit-test oracle frame inventory가 없습니다".to_owned())?;
    let expected_samples = oracle["hitTestSamples"]
        .as_array()
        .filter(|samples| samples.len() == 15)
        .ok_or_else(|| "C13.1 hit-test oracle은 고정 표본 15개여야 합니다".to_owned())?;
    let mut observations = Vec::with_capacity(expected_samples.len());
    for sample in expected_samples {
        let sample_id = sample["id"]
            .as_str()
            .ok_or_else(|| "C13.1 hit-test oracle sample ID가 없습니다".to_owned())?;
        let x = sample["xCssPx"]
            .as_f64()
            .ok_or_else(|| format!("C13.1 hit-test sample {sample_id} x가 없습니다"))?
            as f32;
        let y = sample["yCssPx"]
            .as_f64()
            .ok_or_else(|| format!("C13.1 hit-test sample {sample_id} y가 없습니다"))?
            as f32;
        let expected_element_id = sample["elementId"].as_str();
        let expected_node_index = match expected_element_id {
            Some(element_id) => Some(
                oracle_frames
                    .iter()
                    .position(|frame| frame["id"].as_str() == Some(element_id))
                    .ok_or_else(|| {
                        format!(
                            "C13.1 hit-test sample {sample_id} target {element_id} has no oracle frame"
                        )
                    })?,
            ),
            None if sample["elementId"].is_null() => None,
            None => {
                return Err(format!(
                    "C13.1 hit-test sample {sample_id} target must be a string or null"
                ));
            }
        };
        let hit = scene.hit_test_css_point(x, y);
        let actual_node_index = match hit {
            Some(hit) => Some(
                frames
                    .iter()
                    .position(|frame| frame.node_id == hit.node_id().get())
                    .ok_or_else(|| {
                        format!("C13.1 hit-test sample {sample_id} returned a node without frame")
                    })?,
            ),
            None => None,
        };
        if actual_node_index != expected_node_index {
            return Err(format!(
                "C13.1 hit-test sample {sample_id} returned node index {actual_node_index:?}, expected {expected_node_index:?}"
            ));
        }
        observations.push(serde_json::json!({
            "id": sample_id,
            "xCssPx": x,
            "yCssPx": y,
            "elementId": expected_element_id,
            "nodeIndex": actual_node_index,
            "status": "matched",
        }));
    }
    serde_json::to_string(&observations)
        .map_err(|error| format!("C13.1 hit-test sample JSON 변환 실패: {error}"))
}

#[cfg(feature = "c13-1-overflow-clip-fixture-test")]
fn c13_1_border_hit_test_samples_json(
    scene: &RuntimeRenderSnapshot,
    frames: &[RuntimeLayoutFrame],
) -> Result<String, String> {
    const EXPECTED_FRAME_IDS: [&str; 5] = [
        "root",
        "static-clip",
        "static-target",
        "positioned-clip",
        "positioned-target",
    ];

    let oracle: serde_json::Value = serde_json::from_str(C13_BORDER_HIT_TEST_ORACLE)
        .map_err(|error| format!("C13.1 border Chrome oracle JSON 읽기 실패: {error}"))?;
    if !oracle["browser"]["userAgent"]
        .as_str()
        .is_some_and(|user_agent| user_agent.contains("HeadlessChrome/155.0.0.0"))
    {
        return Err("C13.1 border oracle Chrome version이 고정값과 다릅니다".to_owned());
    }
    if frames.len() != EXPECTED_FRAME_IDS.len() || scene.boxes().len() != EXPECTED_FRAME_IDS.len() {
        return Err(format!(
            "C13.1 border fixture frame 수가 다릅니다: frames={} boxes={}",
            frames.len(),
            scene.boxes().len()
        ));
    }
    let expected_samples = oracle["samples"]
        .as_array()
        .filter(|samples| samples.len() == 6)
        .ok_or_else(|| "C13.1 border oracle는 고정 표본 6개여야 합니다".to_owned())?;
    let mut observations = Vec::with_capacity(expected_samples.len());
    for sample in expected_samples {
        let sample_id = sample["id"]
            .as_str()
            .ok_or_else(|| "C13.1 border sample ID가 없습니다".to_owned())?;
        let x = sample["xCssPx"]
            .as_f64()
            .ok_or_else(|| format!("C13.1 border sample {sample_id} x가 없습니다"))?
            as f32;
        let y = sample["yCssPx"]
            .as_f64()
            .ok_or_else(|| format!("C13.1 border sample {sample_id} y가 없습니다"))?
            as f32;
        let expected_element_id = sample["expectedElementId"]
            .as_str()
            .ok_or_else(|| format!("C13.1 border sample {sample_id} target가 없습니다"))?;
        if sample["hitMatches"] != true || sample["actualElementId"] != expected_element_id {
            return Err(format!(
                "C13.1 border Chrome oracle sample {sample_id} 자체가 일치하지 않습니다"
            ));
        }
        let expected_node_index = EXPECTED_FRAME_IDS
            .iter()
            .position(|id| *id == expected_element_id)
            .ok_or_else(|| format!("C13.1 border sample {sample_id} target ID가 알 수 없습니다"))?;
        let actual_node_index = scene.hit_test_css_point(x, y).and_then(|hit| {
            frames
                .iter()
                .position(|frame| frame.node_id == hit.node_id().get())
        });
        if actual_node_index != Some(expected_node_index) {
            return Err(format!(
                "C13.1 border sample {sample_id} returned node index {actual_node_index:?}, expected {expected_node_index}"
            ));
        }
        observations.push(serde_json::json!({
            "id": sample_id,
            "xCssPx": x,
            "yCssPx": y,
            "elementId": expected_element_id,
            "nodeIndex": actual_node_index,
            "status": "matched",
        }));
    }
    serde_json::to_string(&observations)
        .map_err(|error| format!("C13.1 border sample JSON 변환 실패: {error}"))
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

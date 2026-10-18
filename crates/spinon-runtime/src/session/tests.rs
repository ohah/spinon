use super::{
    CallbackState, Command, ERR_CANCELLED, ERR_CLOSED, ERR_QUEUE_FULL, EnqueueError,
    MemoryPressureLevel, OK, OperationReport, QUEUE_CAPACITY, RuntimeSession,
    RuntimeUaCascadeState, SpinonDocumentCollectionStats, TaskPriority, TaskScheduler,
    operation_report,
};
use crate::host::{DocumentCommitCallback, HostDocumentBridge};
use std::ffi::{CStr, c_char, c_void};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct FakeV8Runtime {
    node_callback: super::NodeCallback,
    text_callback: super::TextCallback,
    user_data: usize,
    terminated: AtomicBool,
    was_terminated: AtomicBool,
    operation_order: Mutex<Vec<String>>,
}

unsafe impl Sync for FakeV8Runtime {}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_new(
    node_callback: super::NodeCallback,
    text_callback: super::TextCallback,
    _document_commit_callback: DocumentCommitCallback,
    _document_query_callback: crate::host::DocumentQueryCallback,
    _document_collect_callback: crate::host::DocumentCollectCallback,
    user_data: *mut c_void,
    _document_user_data: *mut c_void,
) -> *mut super::SpinonV8Runtime {
    Box::into_raw(Box::new(FakeV8Runtime {
        node_callback,
        text_callback,
        user_data: user_data as usize,
        terminated: AtomicBool::new(false),
        was_terminated: AtomicBool::new(false),
        operation_order: Mutex::new(Vec::new()),
    }))
    .cast::<super::SpinonV8Runtime>()
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_eval(
    runtime: *mut super::SpinonV8Runtime,
    source: *const c_char,
) -> i32 {
    let runtime = unsafe { &*runtime.cast::<FakeV8Runtime>() };
    let source = unsafe { CStr::from_ptr(source) }.to_bytes();
    runtime.was_terminated.store(false, Ordering::Release);
    if source == b"hang" {
        while !runtime.terminated.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(1));
        }
        runtime.was_terminated.store(true, Ordering::Release);
        return -1;
    }
    runtime
        .operation_order
        .lock()
        .unwrap()
        .push(format!("eval:{}", String::from_utf8_lossy(source)));
    (runtime.node_callback)(runtime.user_data as *mut c_void, 7, c"view".as_ptr());
    (runtime.text_callback)(runtime.user_data as *mut c_void, c"ready".as_ptr());
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_dispatch(
    runtime: *mut super::SpinonV8Runtime,
    node_id: i32,
) -> i32 {
    let runtime = unsafe { &*runtime.cast::<FakeV8Runtime>() };
    runtime.was_terminated.store(false, Ordering::Release);
    runtime
        .operation_order
        .lock()
        .unwrap()
        .push(format!("dispatch:{node_id}"));
    (runtime.node_callback)(
        runtime.user_data as *mut c_void,
        node_id + 1,
        c"text".as_ptr(),
    );
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_notify_memory_pressure(
    _runtime: *mut super::SpinonV8Runtime,
    level: i32,
) -> i32 {
    if (0..=2).contains(&level) { 0 } else { -1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_was_terminated(runtime: *mut super::SpinonV8Runtime) -> i32 {
    let runtime = unsafe { &*runtime.cast::<FakeV8Runtime>() };
    i32::from(runtime.was_terminated.load(Ordering::Acquire))
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_last_error(
    _runtime: *mut super::SpinonV8Runtime,
) -> *const c_char {
    c"fake JavaScript error".as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_last_collection_error(
    _runtime: *mut super::SpinonV8Runtime,
) -> *const c_char {
    c"".as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_document_collection_stats(
    _runtime: *mut super::SpinonV8Runtime,
    stats: *mut super::SpinonDocumentCollectionStats,
) {
    if !stats.is_null() {
        unsafe { *stats = super::SpinonDocumentCollectionStats::default() };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_terminate(runtime: *mut super::SpinonV8Runtime) {
    let runtime = unsafe { &*runtime.cast::<FakeV8Runtime>() };
    runtime.terminated.store(true, Ordering::Release);
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_cancel_termination(runtime: *mut super::SpinonV8Runtime) {
    let runtime = unsafe { &*runtime.cast::<FakeV8Runtime>() };
    runtime.terminated.store(false, Ordering::Release);
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_runtime_free(runtime: *mut super::SpinonV8Runtime) {
    drop(unsafe { Box::from_raw(runtime.cast::<FakeV8Runtime>()) });
}

#[unsafe(no_mangle)]
pub extern "C" fn spinon_v8_current_thread_id() -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    thread::current().id().hash(&mut hasher);
    hasher.finish()
}

fn new_session() -> RuntimeSession {
    RuntimeSession::new()
        .expect("가짜 V8 런타임 세션을 만들어야 합니다")
        .0
}

#[test]
fn root_html_session_imports_document_before_v8_worker_is_ready() {
    let (session, report) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='viewport' content='width=device-width'></head><body><main>초기 화면</main></body></html>",
    )
    .expect("no-quirks HTML document should initialize the runtime");
    assert!(report.contains("session=ready"));

    let error = RuntimeSession::new_with_root_html("<html><body>quirks</body></html>")
        .err()
        .expect("quirks mode must fail before exposing a session");
    assert!(error.contains("quirks mode"));
    drop(session);
}

#[test]
fn root_html_environment_projects_viewport_and_os_scheme_into_css_cascade() {
    let (session, _) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='viewport' content='width=device-width, initial-scale=1'><meta name='color-scheme' content='dark'><style>:root { color-scheme: light dark; } main { aspect-ratio: 2; } @media (max-width: 390px) { main { width: 123px; } } @media (prefers-color-scheme: dark) { main { height: 456px; } }</style></head><body><main>초기 화면</main></body></html>",
    )
    .expect("root HTML session should import the initial document");
    let mut media_environment = spinon_style::CssMediaEnvironment::MOBILE;
    media_environment.color_scheme = spinon_style::CssColorScheme::Light;

    let initial = session
        .set_root_html_environment(1170.0, 2532.0, 3.0, media_environment)
        .expect("root metadata should configure the CSS environment");
    assert_eq!(initial.viewport_width_css_px, 390.0);
    assert_eq!(initial.viewport_height_css_px, 844.0);
    assert_eq!(initial.device_scale_factor, 3.0);
    assert_eq!(initial.page_scale, 1.0);
    assert_eq!(
        initial.prefers_color_scheme,
        spinon_style::CssColorScheme::Light
    );
    assert_eq!(
        initial.document_color_scheme,
        spinon_style::CssColorScheme::Dark
    );
    assert_eq!(initial.environment_revision.get(), 0);

    let _initial_layout = session.wait_for_layout_snapshot(Duration::from_secs(3));
    let initial_cascade = session.ua_cascade_snapshot();
    assert_eq!(initial_cascade.state, RuntimeUaCascadeState::Ready);
    let initial_styles = &initial_cascade.completed.unwrap().roots[0].styles;
    assert!(initial_styles.elements.iter().any(|element| {
        element
            .properties
            .get("color-scheme")
            .is_some_and(|value| value == "light dark")
    }));
    assert!(initial_styles.elements.iter().any(|element| {
        element
            .properties
            .get("width")
            .is_some_and(|value| value == "123px")
    }));
    assert!(initial_styles.elements.iter().any(|element| {
        element
            .properties
            .get("aspect-ratio")
            .is_some_and(|value| value == "2 / 1" && element.layout_aspect_ratio == Some(2.0))
    }));
    assert!(!initial_styles.elements.iter().any(|element| {
        element
            .properties
            .get("height")
            .is_some_and(|value| value == "456px")
    }));
    assert_eq!(initial_cascade.requested.unwrap().environment_revision, 0);

    let unchanged = session
        .set_root_html_environment(1170.0, 2532.0, 3.0, media_environment)
        .expect("identical host inputs should keep the environment");
    assert_eq!(unchanged.environment_revision.get(), 0);

    media_environment.color_scheme = spinon_style::CssColorScheme::Dark;
    let dark = session
        .set_root_html_environment(1170.0, 2532.0, 3.0, media_environment)
        .expect("OS preference changes should create a new environment");
    assert_eq!(
        dark.prefers_color_scheme,
        spinon_style::CssColorScheme::Dark
    );
    assert_eq!(
        dark.document_color_scheme,
        spinon_style::CssColorScheme::Dark
    );
    assert_eq!(dark.environment_revision.get(), 1);
    let _dark_layout = session.wait_for_layout_snapshot(Duration::from_secs(3));
    let dark_cascade = session.ua_cascade_snapshot();
    assert_eq!(dark_cascade.state, RuntimeUaCascadeState::Ready);
    let dark_styles = &dark_cascade.completed.unwrap().roots[0].styles;
    assert!(dark_styles.elements.iter().any(|element| {
        element
            .properties
            .get("height")
            .is_some_and(|value| value == "456px")
    }));
}

#[test]
fn root_html_layout_media_queries_use_layout_height_not_visual_height() {
    let (session, _) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='viewport' content='width=500, initial-scale=1'><style>@media (min-height: 1000px) { main { height: 123px; } } @media (max-height: 900px) { main { height: 456px; } }</style></head><body><main>viewport</main></body></html>",
    )
    .expect("root HTML session should import the viewport media fixture");

    let environment = session
        .set_root_html_environment(
            1170.0,
            2532.0,
            3.0,
            spinon_style::CssMediaEnvironment::MOBILE,
        )
        .expect("fixed width metadata should create layout and visual viewports");
    assert_eq!(environment.viewport_width_css_px, 500.0);
    assert!((environment.viewport_height_css_px - 1082.0513).abs() <= 0.001);
    assert_eq!(environment.visual_viewport_width_css_px, 390.0);
    assert_eq!(environment.visual_viewport_height_css_px, 844.0);

    let _layout = session.wait_for_layout_snapshot(Duration::from_secs(3));
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let styles = &cascade.completed.unwrap().roots[0].styles;
    assert!(styles.elements.iter().any(|element| {
        element
            .properties
            .get("height")
            .is_some_and(|value| value == "123px")
    }));
    assert!(!styles.elements.iter().any(|element| {
        element
            .properties
            .get("height")
            .is_some_and(|value| value == "456px")
    }));
}

#[test]
fn root_html_canvas_uses_computed_root_scheme_and_exact_chrome_srgb_bytes() {
    let (session, _) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='viewport' content='width=device-width'><meta name='color-scheme' content='light'><style>:root { color-scheme: dark; }</style></head><body><main>canvas</main></body></html>",
    )
    .expect("root HTML session should initialize");
    let mut media_environment = spinon_style::CssMediaEnvironment::MOBILE;
    media_environment.color_scheme = spinon_style::CssColorScheme::Light;

    let canvas = session
        .resolve_root_html_canvas(
            1170.0,
            2532.0,
            3.0,
            media_environment,
            Duration::from_secs(3),
        )
        .expect("completed root style should select the Canvas color");
    assert_eq!(canvas.used_color_scheme, spinon_style::CssColorScheme::Dark);
    assert_eq!(canvas.color_rgba, [18, 18, 18, 255]);
    assert_eq!(
        canvas.environment.prefers_color_scheme,
        spinon_style::CssColorScheme::Light
    );
    assert_eq!(
        canvas.cascade_key.environment_revision,
        canvas.environment.environment_revision.get()
    );
    assert_eq!(
        canvas.cascade_key,
        session.ua_cascade_snapshot().completed.unwrap().key
    );
}

#[test]
fn root_html_canvas_follows_os_only_when_root_css_and_single_meta_allow_it() {
    let (session, _) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='viewport' content='width=device-width'><meta name='color-scheme' content='light dark'><style>:root { color-scheme: normal; }</style></head><body><main>canvas</main></body></html>",
    )
    .expect("root HTML session should initialize");
    let mut media_environment = spinon_style::CssMediaEnvironment::MOBILE;
    media_environment.color_scheme = spinon_style::CssColorScheme::Dark;

    let canvas = session
        .resolve_root_html_canvas(
            1170.0,
            2532.0,
            3.0,
            media_environment,
            Duration::from_secs(3),
        )
        .expect("supported metadata scheme should use the OS preference");
    assert_eq!(canvas.used_color_scheme, spinon_style::CssColorScheme::Dark);
    assert_eq!(canvas.color_rgba, [18, 18, 18, 255]);
    assert_eq!(
        canvas.environment.prefers_color_scheme,
        spinon_style::CssColorScheme::Dark
    );
}

#[test]
fn root_html_canvas_rejects_duplicate_scheme_metadata_before_publishing_a_color() {
    let (session, _) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='color-scheme' content='unknown'><meta name='color-scheme' content='dark'><style>:root { color-scheme: dark; }</style></head><body>canvas</body></html>",
    )
    .expect("duplicate metadata is parsed as HTML but rejected by the Canvas profile");

    let error = session
        .resolve_root_html_canvas(
            1170.0,
            2532.0,
            3.0,
            spinon_style::CssMediaEnvironment::MOBILE,
            Duration::from_secs(3),
        )
        .expect_err("duplicate metadata must not publish a Canvas color");
    assert_eq!(
        error,
        super::RootHtmlCanvasError::DuplicateColorSchemeMetadata
    );
}

#[test]
fn root_html_canvas_prefers_opaque_html_background_then_propagates_body_background() {
    let (session, _) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='viewport' content='width=device-width'><meta name='color-scheme' content='light dark'><style>html { color-scheme: dark; background-color: rgb(18 52 86); } body { background-color: rgb(171 205 239); }</style></head><body><main>canvas</main></body></html>",
    )
    .expect("root HTML session should initialize");

    let canvas = session
        .resolve_root_html_canvas(
            1170.0,
            2532.0,
            3.0,
            spinon_style::CssMediaEnvironment::MOBILE,
            Duration::from_secs(3),
        )
        .expect("supported root and body backgrounds should resolve");
    assert_eq!(canvas.used_color_scheme, spinon_style::CssColorScheme::Dark);
    assert_eq!(canvas.color_rgba, [18, 52, 86, 255]);

    let completed = session.ua_cascade_snapshot().completed.unwrap();
    let root = &completed.roots[0];
    let body_id = root.body_node_id.expect("HTML document should have a body");
    let body = root
        .styles
        .elements
        .iter()
        .find(|element| element.node_id.get() == body_id)
        .expect("computed styles should include the body");
    assert_eq!(
        body.background_paint,
        Some(spinon_style::ComputedBackgroundPaint::Opaque(
            spinon_style::OpaqueCssSrgb {
                red: 171,
                green: 205,
                blue: 239,
            }
        ))
    );
}

#[test]
fn root_html_canvas_ignores_unsupported_body_alpha_when_opaque_html_wins() {
    let (session, _) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='viewport' content='width=device-width'><meta name='color-scheme' content='light dark'><style>html { background-color: rgb(18 52 86); } body { background-color: rgba(200 100 50 / 0.5); }</style></head><body>canvas</body></html>",
    )
    .expect("root HTML session should initialize");

    let canvas = session
        .resolve_root_html_canvas(
            1170.0,
            2532.0,
            3.0,
            spinon_style::CssMediaEnvironment::MOBILE,
            Duration::from_secs(3),
        )
        .expect("opaque html background takes precedence over body alpha");
    assert_eq!(canvas.color_rgba, [18, 52, 86, 255]);
}

#[test]
fn root_html_canvas_treats_zero_alpha_body_background_as_transparent() {
    let (session, _) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='viewport' content='width=device-width'><meta name='color-scheme' content='light dark'><style>:root { color-scheme: dark; } body { background-color: rgb(30 140 70 / 0); }</style></head><body>canvas</body></html>",
    )
    .expect("root HTML session should initialize");
    let mut media_environment = spinon_style::CssMediaEnvironment::MOBILE;
    media_environment.color_scheme = spinon_style::CssColorScheme::Dark;

    let canvas = session
        .resolve_root_html_canvas(
            1170.0,
            2532.0,
            3.0,
            media_environment,
            Duration::from_secs(3),
        )
        .expect("zero-alpha body background should use the dark Canvas fallback");
    assert_eq!(canvas.color_rgba, [18, 18, 18, 255]);
}

#[test]
fn root_html_canvas_propagates_body_color_from_stylesheet_and_custom_property() {
    let (session, _) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='viewport' content='width=device-width'><meta name='color-scheme' content='light dark'><style>:root { --canvas-test: rgb(30 140 70); } body { background-color: var(--canvas-test); }</style></head><body><main>canvas</main></body></html>",
    )
    .expect("root HTML session should initialize");
    let mut media_environment = spinon_style::CssMediaEnvironment::MOBILE;
    media_environment.color_scheme = spinon_style::CssColorScheme::Dark;

    let canvas = session
        .resolve_root_html_canvas(
            1170.0,
            2532.0,
            3.0,
            media_environment,
            Duration::from_secs(3),
        )
        .expect("computed var() body background should resolve");
    assert_eq!(canvas.used_color_scheme, spinon_style::CssColorScheme::Dark);
    assert_eq!(canvas.color_rgba, [30, 140, 70, 255]);
}

#[test]
fn root_html_canvas_recomputes_body_background_for_prefers_color_scheme() {
    let (session, _) = RuntimeSession::new_with_root_html(
        "<!doctype html><html><head><meta name='viewport' content='width=device-width'><meta name='color-scheme' content='light dark'><style>:root { color-scheme: normal; } @media (prefers-color-scheme: light) { body { background-color: rgb(210 220 230); } } @media (prefers-color-scheme: dark) { body { background-color: rgb(12 34 56); } }</style></head><body>canvas</body></html>",
    )
    .expect("root HTML session should initialize");
    let mut media_environment = spinon_style::CssMediaEnvironment::MOBILE;
    media_environment.color_scheme = spinon_style::CssColorScheme::Light;

    let light = session
        .resolve_root_html_canvas(
            1170.0,
            2532.0,
            3.0,
            media_environment,
            Duration::from_secs(3),
        )
        .expect("light media body background should resolve");
    assert_eq!(light.color_rgba, [210, 220, 230, 255]);
    assert_eq!(light.used_color_scheme, spinon_style::CssColorScheme::Light);

    media_environment.color_scheme = spinon_style::CssColorScheme::Dark;
    let dark = session
        .resolve_root_html_canvas(
            1170.0,
            2532.0,
            3.0,
            media_environment,
            Duration::from_secs(3),
        )
        .expect("dark media body background should resolve");
    assert_eq!(dark.color_rgba, [12, 34, 56, 255]);
    assert_eq!(dark.used_color_scheme, spinon_style::CssColorScheme::Dark);
    assert!(dark.cascade_key.environment_revision > light.cascade_key.environment_revision);
}

#[test]
fn root_html_canvas_fails_closed_for_partial_alpha_and_background_images() {
    for style in [
        "body { background-color: rgba(200, 100, 50, 0.5); }",
        "body { background-color: color(srgb 0.2 0.2 0.9); }",
        "body { background-color: hsl(120 100% 25%); }",
        "body { background-color: hwb(240 20% 10%); }",
        "body { background-color: Canvas; }",
        "body { background-image: linear-gradient(red, blue); }",
    ] {
        let source = format!(
            "<!doctype html><html><head><meta name='viewport' content='width=device-width'><meta name='color-scheme' content='light dark'><style>{style}</style></head><body>canvas</body></html>"
        );
        let (session, _) = RuntimeSession::new_with_root_html(&source)
            .expect("unsupported background style still parses as HTML");
        let error = session
            .resolve_root_html_canvas(
                1170.0,
                2532.0,
                3.0,
                spinon_style::CssMediaEnvironment::MOBILE,
                Duration::from_secs(3),
            )
            .expect_err("unsupported paint input must not publish a Canvas color");
        let expected = if style.contains("rgba") {
            super::RootHtmlCanvasError::UnsupportedComputedBackgroundColor
        } else {
            super::RootHtmlCanvasError::CascadeFailed
        };
        assert_eq!(error, expected);
    }
}

#[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
#[test]
fn c04_12_root_html_block_paint_fixture_matches_light_and_dark_geometry() {
    const SOURCE: &str =
        include_str!("../../../../tests/fixtures/css/c04/root-html-block-paint.html");
    fn near(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() <= 0.5,
            "actual={actual} expected={expected}"
        );
    }

    let (session, _) = RuntimeSession::new_runtime_gpu_root_html_block_paint_fixture(SOURCE)
        .expect("C04.12 parsed root HTML fixture should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    let light_environment = session
        .set_root_html_environment(320.0, 240.0, 1.0, media)
        .expect("meta viewport should resolve to the 320 by 240 CSS viewport");
    assert_eq!(light_environment.viewport_width_css_px, 320.0);
    assert_eq!(light_environment.viewport_height_css_px, 240.0);

    let light = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(light.state, super::RuntimeLayoutState::Ready, "{light:?}");
    let light_completed = light.completed.expect("light layout should be complete");
    assert_eq!(light.requested, Some(light_completed.key));
    let expected = [
        [0.0, 0.0, 320.0, 240.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 320.0, 200.0],
        [0.0, 0.0, 160.0, 32.0],
        [0.0, 32.0, 320.0, 24.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 32.0, 120.0, 24.0],
    ];
    assert_eq!(light_completed.frames.len(), expected.len());
    for (frame, expected) in light_completed.frames.iter().zip(expected) {
        for (actual, expected) in [frame.x, frame.y, frame.width, frame.height]
            .into_iter()
            .zip(expected)
        {
            near(actual, expected);
        }
    }
    let light_scene = light_completed
        .render_snapshot
        .as_ref()
        .expect("root HTML fixture should produce a render scene");
    assert_eq!(light_scene.boxes().len(), 5);
    assert!(matches!(
        light_scene.boxes()[4].paint(),
        spinon_render::RuntimePaint::Opaque(color)
            if color == spinon_render::OpaqueCssSrgb::new(64, 160, 96)
    ));

    media.color_scheme = spinon_style::CssColorScheme::Dark;
    let dark_environment = session
        .set_root_html_environment(320.0, 240.0, 1.0, media)
        .expect("dark scheme should update the parsed document environment");
    let dark = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(dark.state, super::RuntimeLayoutState::Ready, "{dark:?}");
    let dark_completed = dark.completed.expect("dark layout should be complete");
    assert!(dark_completed.key.environment_revision > light_completed.key.environment_revision);
    assert_eq!(
        dark_completed.key.environment_revision,
        dark_environment.environment_revision.get()
    );
    let dark_scene = dark_completed
        .render_snapshot
        .as_ref()
        .expect("dark root HTML fixture should produce a render scene");
    assert_eq!(dark_scene.boxes().len(), 5);
    assert!(matches!(
        dark_scene.boxes()[4].paint(),
        spinon_render::RuntimePaint::Opaque(color)
            if color == spinon_render::OpaqueCssSrgb::new(160, 92, 44)
    ));
}

#[cfg(feature = "c04-12-root-html-block-paint-fixture-test")]
#[test]
fn c04_12_root_html_block_paint_rejects_visible_text_and_svg_but_ignores_hidden_text() {
    fn run(source: &str) -> super::RuntimeLayoutSnapshot {
        let (session, _) = RuntimeSession::new_runtime_gpu_root_html_block_paint_fixture(source)
            .expect("valid HTML source should initialize the fixture session");
        let mut media = spinon_style::CssMediaEnvironment::MOBILE;
        media.color_scheme = spinon_style::CssColorScheme::Light;
        session
            .set_root_html_environment(320.0, 240.0, 1.0, media)
            .expect("root HTML environment should resolve");
        session.wait_for_layout_snapshot(Duration::from_secs(5))
    }

    let visible_text = run(
        "<!doctype html><html><head><style>main{display:block;width:10px;height:10px}</style></head><body><main>visible text</main></body></html>",
    );
    assert_eq!(visible_text.state, super::RuntimeLayoutState::Failed);
    assert_eq!(
        visible_text.error.as_ref().map(|error| error.code),
        Some("visible_root_html_text")
    );

    let visible_svg = run(
        "<!doctype html><html><head><style>main{display:block;width:10px;height:10px}</style></head><body><svg></svg><main></main></body></html>",
    );
    assert_eq!(visible_svg.state, super::RuntimeLayoutState::Failed);
    assert_eq!(
        visible_svg.error.as_ref().map(|error| error.code),
        Some("unsupported_visible_root_html_element")
    );

    let hidden_text = run(
        "<!doctype html><html><head><style>#hidden{display:none}main{display:block;width:10px;height:10px;background-color:#3366ff}</style></head><body><div id='hidden'>hidden text</div><main></main></body></html>",
    );
    assert_eq!(
        hidden_text.state,
        super::RuntimeLayoutState::Ready,
        "{hidden_text:?}"
    );
    assert_eq!(
        hidden_text
            .completed
            .as_ref()
            .and_then(|completed| completed.render_snapshot.as_ref())
            .map(|scene| scene.boxes().len()),
        Some(3)
    );
}

fn eval(session: &RuntimeSession, source: &str) -> super::OperationResponse {
    session.eval(source, TaskPriority::UserVisible)
}

#[cfg(feature = "c14-1-live-v8-fixture-test")]
#[test]
fn c14_fixture_snapshot_is_actor_owned_and_closed_sessions_fail() {
    let session = new_session();
    let snapshot = session
        .snapshot_document_for_c14_1_fixture()
        .expect("actor should return the initial HostDocument snapshot");
    assert_eq!(snapshot.document_revision().get(), 0);
    assert_eq!(snapshot.root_children().count(), 0);

    session
        .shutdown()
        .expect("test session should close cleanly");
    assert!(
        session
            .snapshot_document_for_c14_1_fixture()
            .expect_err("closed session must reject fixture snapshots")
            .contains("종료")
    );
}

#[test]
fn report_exposes_caller_owner_callback_threads_and_timings() {
    let callbacks = CallbackState {
        callback_count: 2,
        created_nodes: 1,
        last_node_id: 7,
        callback_thread_id: 42,
        document: HostDocumentBridge::new().unwrap(),
    };
    let report = operation_report(OperationReport {
        sequence: 3,
        trace_cookie: 30,
        operation: "dispatch",
        status: 0,
        caller_thread_id: 10,
        owner_thread_id: 42,
        callback_thread_id: 42,
        submission_lock_wait_us: 1,
        control_lock_wait_us: 2,
        scheduler_lock_wait_us: 3,
        queue_residence_us: 11,
        v8_call_us: 29,
        cancel_requested: false,
        callbacks: &callbacks,
        collection_error: "",
        collection_stats: SpinonDocumentCollectionStats::default(),
        error: "",
    });
    assert!(report.contains("caller_tid=10"));
    assert!(report.contains("trace_cookie=30"));
    assert!(report.contains("owner_tid=42"));
    assert!(report.contains("callback_tid=42"));
    assert!(report.contains("submission_lock_wait_us=1"));
    assert!(report.contains("control_lock_wait_us=2"));
    assert!(report.contains("scheduler_lock_wait_us=3"));
    assert!(report.contains("queue_residence_us=11"));
    assert!(report.contains("v8_call_us=29"));
    assert!(report.contains("cancel_requested=false"));
    assert!(report.contains("document_collection_scans=0"));
    assert!(report.contains("document_collection_deferred=0"));
    assert!(report.contains("document_collection_error=none"));
    assert!(report.contains("document_revision=0"));
    assert!(report.contains("render_tree_revision=0"));
    assert!(report.contains("document_nodes=0"));
    assert!(report.contains("document_string_units=0"));
    assert!(report.contains("document_collection_error=none"));
    assert!(report.contains("callback_count=2"));
}

#[test]
fn bounded_scheduler_selects_priority_then_fifo_and_drains_on_stop() {
    let scheduler = TaskScheduler::new();
    for (priority, node_id) in [
        (TaskPriority::Background, 1),
        (TaskPriority::UserVisible, 2),
        (TaskPriority::Background, 3),
        (TaskPriority::UserBlocking, 4),
        (TaskPriority::UserBlocking, 5),
        (TaskPriority::UserVisible, 6),
    ] {
        let (reply, _response) = std::sync::mpsc::sync_channel(1);
        let command = Command::Dispatch {
            sequence: node_id as u64,
            trace_cookie: node_id as u32,
            node_id,
            caller_thread_id: 0,
            reply,
        };
        assert!(scheduler.try_enqueue(priority, command, 0, 0).is_ok());
    }

    let mut order = Vec::new();
    for _ in 0..6 {
        let command = scheduler.receive().expect("대기 작업을 받아야 합니다");
        if let Command::Dispatch { node_id, .. } = command.command {
            order.push(node_id);
        }
    }
    assert_eq!(order, [4, 5, 2, 6, 1, 3]);

    scheduler.stop();
    assert!(scheduler.receive().is_none());
    let (reply, _response) = std::sync::mpsc::sync_channel(1);
    assert!(matches!(
        scheduler.try_enqueue(
            TaskPriority::UserBlocking,
            Command::Dispatch {
                sequence: 7,
                trace_cookie: 7,
                node_id: 7,
                caller_thread_id: 0,
                reply,
            },
            0,
            0,
        ),
        Err(EnqueueError::Stopped)
    ));
}

#[test]
fn session_executes_pending_commands_by_priority_and_fifo() {
    let session = Arc::new(new_session());
    let running_session = Arc::clone(&session);
    let running = thread::spawn(move || eval(&running_session, "hang"));
    wait_until_active(&session);

    let background_session = Arc::clone(&session);
    let background =
        thread::spawn(move || background_session.eval("background", TaskPriority::Background));
    wait_for_queue_len(&session, 1);
    let visible_session = Arc::clone(&session);
    let visible = thread::spawn(move || visible_session.dispatch(22, TaskPriority::UserVisible));
    wait_for_queue_len(&session, 2);
    let blocking_first_session = Arc::clone(&session);
    let blocking_first =
        thread::spawn(move || blocking_first_session.dispatch(31, TaskPriority::UserBlocking));
    wait_for_queue_len(&session, 3);
    let blocking_second_session = Arc::clone(&session);
    let blocking_second =
        thread::spawn(move || blocking_second_session.dispatch(32, TaskPriority::UserBlocking));
    wait_for_queue_len(&session, 4);

    assert_eq!(session.cancel(), OK);
    assert_eq!(running.join().unwrap().status, ERR_CANCELLED);
    assert_eq!(background.join().unwrap().status, OK);
    assert_eq!(visible.join().unwrap().status, OK);
    assert_eq!(blocking_first.join().unwrap().status, OK);
    assert_eq!(blocking_second.join().unwrap().status, OK);

    let runtime = super::lock(&session.control)
        .runtime
        .expect("세션 V8 실행기가 남아 있어야 합니다");
    let operation_order = unsafe { &*(runtime as *const FakeV8Runtime) }
        .operation_order
        .lock()
        .unwrap()
        .clone();
    assert_eq!(
        operation_order,
        [
            "dispatch:31",
            "dispatch:32",
            "dispatch:22",
            "eval:background"
        ]
    );
}

fn wait_until_active(session: &RuntimeSession) {
    let deadline = Instant::now() + Duration::from_secs(2);
    while !super::lock(&session.control).active && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(1));
    }
    assert!(super::lock(&session.control).active);
}

fn wait_for_queue_len(session: &RuntimeSession, expected: usize) {
    let deadline = Instant::now() + Duration::from_secs(2);
    while super::lock(&session.scheduler.state).queue.len() != expected && Instant::now() < deadline
    {
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(super::lock(&session.scheduler.state).queue.len(), expected);
}

#[test]
fn persistent_session_keeps_owner_and_callbacks_on_one_worker() {
    let session = new_session();
    let first = eval(&session, "callback");
    assert_eq!(first.status, OK);
    let caller = field(&first.report, "caller_tid");
    let owner = field(&first.report, "owner_tid");
    assert_ne!(caller, owner);
    assert_eq!(owner, field(&first.report, "callback_tid"));

    let second = session.dispatch(7, TaskPriority::UserBlocking);
    assert_eq!(second.status, OK);
    assert_eq!(field(&second.report, "owner_tid"), owner);
    assert_eq!(field(&second.report, "callback_tid"), owner);
}

#[test]
fn cancel_interrupts_active_call_and_session_accepts_next_call() {
    let session = Arc::new(new_session());
    let running_session = Arc::clone(&session);
    let running = thread::spawn(move || eval(&running_session, "hang"));
    wait_until_active(&session);
    assert_eq!(session.cancel(), OK);

    let cancelled = running.join().unwrap();
    assert_eq!(cancelled.status, ERR_CANCELLED);
    assert!(cancelled.report.contains("cancel_requested=true"));
    assert!(cancelled.report.contains("owner_tid="));
    let resumed = eval(&session, "again");
    assert_eq!(resumed.status, OK);
    assert!(resumed.report.contains("error=none"));
}

#[test]
fn memory_pressure_levels_reach_the_runtime_and_closed_sessions_reject_them() {
    let session = new_session();
    assert_eq!(MemoryPressureLevel::None as i32, 0);
    assert_eq!(MemoryPressureLevel::Moderate as i32, 1);
    assert_eq!(MemoryPressureLevel::Critical as i32, 2);
    for level in [
        MemoryPressureLevel::None,
        MemoryPressureLevel::Moderate,
        MemoryPressureLevel::Critical,
    ] {
        assert_eq!(session.notify_memory_pressure(level), OK);
    }

    session.shutdown().expect("유휴 세션은 종료되어야 합니다");
    assert_eq!(
        session.notify_memory_pressure(MemoryPressureLevel::None),
        ERR_CLOSED
    );
}

#[test]
fn bounded_queue_rejects_overflow_and_drains_after_cancellation() {
    assert_eq!(QUEUE_CAPACITY, 64);
    let session = Arc::new(new_session());
    let running_session = Arc::clone(&session);
    let long_eval = thread::spawn(move || eval(&running_session, "hang"));
    wait_until_active(&session);

    let start = Arc::new(Barrier::new(66));
    let completed = Arc::new(AtomicUsize::new(0));
    let outcomes = Arc::new(Mutex::new(Vec::with_capacity(65)));
    let mut callers = Vec::with_capacity(65);
    for node_id in 0..65 {
        let start = Arc::clone(&start);
        let completed = Arc::clone(&completed);
        let outcomes = Arc::clone(&outcomes);
        let session = Arc::clone(&session);
        callers.push(thread::spawn(move || {
            start.wait();
            let response = session.dispatch(node_id, TaskPriority::UserVisible);
            let sequence = (response.status == OK)
                .then(|| field(&response.report, "seq").parse::<u64>().unwrap());
            outcomes
                .lock()
                .unwrap()
                .push((node_id, response.status, sequence));
            completed.fetch_add(1, Ordering::Release);
        }));
    }
    start.wait();
    let deadline = Instant::now() + Duration::from_secs(2);
    while (session.next_sequence.load(Ordering::Acquire) != 66
        || super::lock(&session.scheduler.state).queue.len() != QUEUE_CAPACITY
        || completed.load(Ordering::Acquire) == 0)
        && Instant::now() < deadline
    {
        thread::sleep(Duration::from_millis(1));
    }
    let next_sequence = session.next_sequence.load(Ordering::Acquire);
    let queued_before_cancel = super::lock(&session.scheduler.state).queue.len();
    let completed_before_cancel = completed.load(Ordering::Acquire);
    let cancel_status = session.cancel();
    let blocker_status = long_eval.join().unwrap().status;
    for caller in callers {
        caller.join().unwrap();
    }
    assert_eq!(next_sequence, 66);
    assert_eq!(queued_before_cancel, QUEUE_CAPACITY);
    assert_eq!(
        completed_before_cancel, 1,
        "65개 제출 중 대기 큐 포화 거부가 정확히 한 건이어야 합니다"
    );
    assert_eq!(cancel_status, OK);
    assert_eq!(blocker_status, ERR_CANCELLED);
    let outcomes = outcomes.lock().unwrap();
    assert_eq!(
        outcomes
            .iter()
            .filter(|(_, status, _)| *status == OK)
            .count(),
        QUEUE_CAPACITY
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|(_, status, _)| *status == ERR_QUEUE_FULL)
            .count(),
        1
    );
    assert!(
        outcomes
            .iter()
            .all(|(_, status, _)| { *status == OK || *status == ERR_QUEUE_FULL })
    );

    let rejected_node = outcomes
        .iter()
        .find_map(|(node_id, status, _)| (*status == ERR_QUEUE_FULL).then_some(*node_id))
        .expect("정확히 한 번의 포화 거부가 있어야 합니다");
    let mut accepted_by_sequence = outcomes
        .iter()
        .filter_map(|(node_id, status, sequence)| {
            if *status == OK {
                Some((
                    sequence.expect("수락된 작업은 sequence를 가져야 합니다"),
                    *node_id,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    accepted_by_sequence.sort_unstable_by_key(|(sequence, _)| *sequence);
    let expected_order = accepted_by_sequence
        .iter()
        .map(|(_, node_id)| format!("dispatch:{node_id}"))
        .collect::<Vec<_>>();
    let runtime = super::lock(&session.control)
        .runtime
        .expect("세션 V8 실행기가 취소 뒤에도 유지되어야 합니다");
    let operation_order = unsafe { &*(runtime as *const FakeV8Runtime) }
        .operation_order
        .lock()
        .unwrap()
        .clone();
    assert_eq!(operation_order, expected_order);
    assert!(!operation_order.contains(&format!("dispatch:{rejected_node}")));
    assert_eq!(eval(&session, "after-overflow-drain").status, OK);
    let operation_order = unsafe { &*(runtime as *const FakeV8Runtime) }
        .operation_order
        .lock()
        .unwrap()
        .clone();
    assert_eq!(operation_order.last().unwrap(), "eval:after-overflow-drain");
}

fn field(report: &str, key: &str) -> String {
    report
        .split_whitespace()
        .find_map(|field| field.strip_prefix(&format!("{key}=")))
        .unwrap_or_else(|| panic!("보고서에 {key} 필드가 없습니다: {report}"))
        .to_owned()
}

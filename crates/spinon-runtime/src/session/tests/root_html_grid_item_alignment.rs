use super::*;

const SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c11/root-html-grid-item-alignment.html");
const EXPECTED: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-root-html-grid-item-alignment-precomparison-2026-11-03/expected-values.json"
);

#[test]
fn c11_1_root_html_fixed_grid_item_alignment_matches_all_expected_frames() {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(SOURCE)
        .expect("parsed Root HTML fixture should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("Root HTML metadata should configure the CSS environment");

    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        cascade.state,
        RuntimeUaCascadeState::Ready,
        "cascade error={:?}",
        cascade.error
    );
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; cascade={cascade:?}",
        layout.error
    );
    let scene = layout
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("default Root HTML route should publish the Grid paint scene");
    let expected: serde_json::Value =
        serde_json::from_str(EXPECTED).expect("independent expected values should parse");
    let cases = expected["cases"].as_array().expect("cases array");
    let mut checked_items = 0usize;

    for (case_index, case) in cases.iter().enumerate() {
        let name = case["case"].as_str().expect("case name");
        let origin_y = case_index as f32 * 64.0;
        if let Some(item) = case.get("item") {
            assert_frame_for_color(
                &scene,
                case["color"].as_str().unwrap(),
                item,
                name,
                origin_y,
            );
            checked_items += 1;
        } else {
            let colors = case["colors"].as_array().expect("combined colors");
            let items = case["items"].as_array().expect("combined frames");
            assert_eq!(colors.len(), items.len(), "combined item/color count");
            for (index, (color, item)) in colors.iter().zip(items).enumerate() {
                assert_frame_for_color(
                    &scene,
                    color.as_str().expect("combined color"),
                    item,
                    &format!("{name}[{index}]"),
                    origin_y,
                );
                checked_items += 1;
            }
        }
    }

    assert_eq!(cases.len(), 39, "fixed alignment scene count");
    assert_eq!(checked_items, 42, "fixed alignment item count");
}

#[test]
fn c11_1_root_html_rejected_item_alignment_does_not_publish_a_scene() {
    let source = SOURCE.replace(
        ".case-align-normal { align-items: normal; }",
        ".case-align-normal { align-items: baseline; }",
    );
    assert_ne!(
        source, SOURCE,
        "the baseline control must change the fixture"
    );
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(&source)
        .expect("parsed Root HTML rejection fixture should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("Root HTML metadata should configure the CSS environment");

    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "baseline alignment must be rejected by projection"
    );
    assert!(
        layout.error.is_some(),
        "rejection should retain its diagnostic"
    );
    assert!(
        layout.completed.is_none(),
        "failed alignment projection must not publish a partial render scene"
    );
}

fn assert_frame_for_color(
    scene: &spinon_render::RuntimeRenderSnapshot,
    expected_color: &str,
    expected_frame: &serde_json::Value,
    label: &str,
    origin_y: f32,
) {
    let color = parse_color(expected_color);
    let render_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(actual) if actual == color
            )
        })
        .unwrap_or_else(|| panic!("expected item color {expected_color} for {label}"));
    let frame = render_box.frame_css_px();
    let actual = [frame.x(), frame.y(), frame.width(), frame.height()];
    for (component, value) in actual.into_iter().enumerate() {
        let expected_local = expected_frame[component]
            .as_f64()
            .unwrap_or_else(|| panic!("missing expected frame component {component} for {label}"))
            as f32;
        let expected = if component == 1 {
            expected_local + origin_y
        } else {
            expected_local
        };
        assert!(
            (value - expected).abs() <= 1.0 / 64.0,
            "{label} frame component {component}: actual={value}, expected={expected} ± 1/64 CSS px"
        );
    }
}

fn parse_color(value: &str) -> spinon_render::OpaqueCssSrgb {
    let digits = value.strip_prefix('#').expect("hex CSS color");
    assert_eq!(digits.len(), 6);
    let channel = |start| u8::from_str_radix(&digits[start..start + 2], 16).unwrap();
    spinon_render::OpaqueCssSrgb::new(channel(0), channel(2), channel(4))
}

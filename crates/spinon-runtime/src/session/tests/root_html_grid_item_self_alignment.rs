use super::*;

const SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c11/root-html-grid-item-self-alignment.html");
const EXPECTED: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-root-html-grid-item-self-alignment-precomparison-2026-11-04/expected.json"
);

#[test]
fn c11_5_root_html_grid_item_self_alignment_matches_all_expected_frames() {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(SOURCE)
        .expect("parsed Root HTML fixture should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 1100.0, 1.0, media)
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
        "layout error={:?}; cascade state={:?}; cascade error={:?}",
        layout.error,
        cascade.state,
        cascade.error
    );
    let scene = layout
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("default Root HTML route should publish the Grid paint scene");
    let expected: serde_json::Value =
        serde_json::from_str(EXPECTED).expect("independent expected values should parse");
    let expected_cases = expected["scenes"].as_array().expect("scene array");
    let grid_color = parse_color("#dbeafe");
    let item_color = parse_color("#f97316");
    let mut grid_boxes = scene
        .boxes()
        .iter()
        .filter(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(color) if color == grid_color
            )
        })
        .copied()
        .collect::<Vec<_>>();
    let mut item_boxes = scene
        .boxes()
        .iter()
        .filter(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(color) if color == item_color
            )
        })
        .copied()
        .collect::<Vec<_>>();
    grid_boxes.sort_by_key(|render_box| render_box.node_id());
    item_boxes.sort_by_key(|render_box| render_box.node_id());
    assert_eq!(grid_boxes.len(), expected_cases.len(), "Grid area count");
    assert_eq!(item_boxes.len(), expected_cases.len(), "Grid item count");

    for (index, ((grid, item), case)) in grid_boxes
        .iter()
        .zip(&item_boxes)
        .zip(expected_cases)
        .enumerate()
    {
        let grid_frame = grid.frame_css_px();
        let item_frame = item.frame_css_px();
        let actual = [
            item_frame.x() - grid_frame.x(),
            item_frame.y() - grid_frame.y(),
            item_frame.width(),
            item_frame.height(),
        ];
        let expected_frame = &case["expectedItemFrame"];
        for (component, value) in actual.into_iter().enumerate() {
            let expected = expected_frame[match component {
                0 => "x",
                1 => "y",
                2 => "width",
                3 => "height",
                _ => unreachable!("the item frame has four components"),
            }]
            .as_f64()
            .unwrap_or_else(|| panic!("missing expected frame component {component}"))
                as f32;
            assert!(
                (value - expected).abs() <= 1.0 / 64.0,
                "scene {} frame component {component}: actual={value}, expected={expected} ± 1/64 CSS px",
                case["id"].as_str().unwrap_or("<unknown>")
            );
        }
        assert_eq!(grid_frame.width(), 40.0, "scene {index} Grid width");
        assert_eq!(grid_frame.height(), 30.0, "scene {index} Grid height");
    }
}

#[test]
fn c11_5_root_html_grid_item_self_alignment_rejects_unsupported_values_atomically() {
    for (needle, replacement, property, value) in [
        (
            "align-self:auto;justify-self:auto",
            "align-self:baseline;justify-self:auto",
            "align-self",
            "baseline",
        ),
        (
            "align-self:auto;justify-self:auto",
            "align-self:auto;justify-self:safe start",
            "justify-self",
            "safe start",
        ),
    ] {
        let source = SOURCE.replacen(needle, replacement, 1);
        assert_ne!(source, SOURCE, "the unsupported-value control must change");
        let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(&source)
            .expect("parsed Root HTML rejection fixture should initialize");
        let mut media = spinon_style::CssMediaEnvironment::MOBILE;
        media.color_scheme = spinon_style::CssColorScheme::Light;
        session
            .set_root_html_environment(1280.0, 1100.0, 1.0, media)
            .expect("Root HTML metadata should configure the CSS environment");

        let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{property}:{value} must fail closed"
        );
        let error = layout
            .error
            .as_ref()
            .expect("projection failure diagnostic");
        assert_eq!(error.property.as_deref(), Some(property));
        let node_id = error.node_id.expect("failure should identify the item");
        let computed_value = cascade
            .completed
            .as_ref()
            .expect("cascade completion")
            .roots
            .iter()
            .flat_map(|root| root.styles.elements.iter())
            .find(|element| element.node_id.get() == node_id)
            .and_then(|element| element.properties.get(property))
            .map(String::as_str);
        assert_eq!(computed_value, Some(value), "computed {property} value");
        assert!(
            layout.completed.is_none(),
            "unsupported self-alignment must not publish a partial scene"
        );
    }
}

#[test]
fn c11_5_root_html_grid_item_self_alignment_stays_rejected_outside_fixed_track_scope() {
    let source = SOURCE
        .replacen(
            "style=\"align-items:center;justify-items:center\"",
            "style=\"grid-template-columns:1fr;align-items:normal;justify-items:normal\"",
            1,
        )
        .replacen(
            "align-self:auto;justify-self:auto",
            "align-self:center;justify-self:auto",
            1,
        );
    assert_ne!(source, SOURCE, "the noneligible-scope control must change");
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(&source)
        .expect("parsed Root HTML rejection fixture should initialize");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 1100.0, 1.0, media)
        .expect("Root HTML metadata should configure the CSS environment");

    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "self-alignment must remain rejected for a fractional-track Grid"
    );
    let error = layout
        .error
        .as_ref()
        .expect("projection failure diagnostic");
    assert_eq!(error.property.as_deref(), Some("align-self"));
    assert!(layout.completed.is_none(), "failure must be atomic");
}

fn parse_color(value: &str) -> spinon_render::OpaqueCssSrgb {
    let digits = value.strip_prefix('#').expect("hex CSS color");
    assert_eq!(digits.len(), 6);
    let channel = |start| u8::from_str_radix(&digits[start..start + 2], 16).unwrap();
    spinon_render::OpaqueCssSrgb::new(channel(0), channel(2), channel(4))
}

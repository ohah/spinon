use super::*;

const SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c11/root-html-grid-item-self-alignment.html");
const EXPECTED: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-root-html-grid-item-self-alignment-precomparison-2026-11-04/expected.json"
);
const AUTO_STRETCH_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-item-auto-stretch-runtime.html"
);
const AUTO_STRETCH_EXPECTED: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-5-grid-item-auto-stretch-precomparison-2026-11-04/expected.json"
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
fn c11_5_root_html_grid_item_auto_stretch_matches_all_expected_frames() {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(AUTO_STRETCH_SOURCE)
        .expect("parsed Root HTML auto-stretch fixture should initialize");
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
    let completed = layout.completed.as_ref().expect("completed layout");
    let cascade_completed = cascade.completed.as_ref().expect("completed cascade");
    assert_eq!(
        completed.key, cascade_completed.key,
        "layout/cascade revision"
    );
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML route should publish the Grid paint scene");
    let expected: serde_json::Value =
        serde_json::from_str(AUTO_STRETCH_EXPECTED).expect("independent expected values parse");
    let expected_cases = expected["scenes"].as_array().expect("scene array");
    let styles = cascade_completed
        .roots
        .iter()
        .flat_map(|root| root.styles.elements.iter())
        .collect::<Vec<_>>();
    let mut grid_styles = styles
        .iter()
        .copied()
        .filter(|element| {
            element
                .properties
                .get("display")
                .is_some_and(|value| value == "grid")
                && element
                    .properties
                    .get("background-color")
                    .is_some_and(|value| value == "rgb(219, 234, 254)")
        })
        .collect::<Vec<_>>();
    let mut item_styles = styles
        .iter()
        .copied()
        .filter(|element| {
            element
                .properties
                .get("background-color")
                .is_some_and(|value| value == "rgb(249, 115, 22)")
        })
        .collect::<Vec<_>>();
    grid_styles.sort_by_key(|element| element.node_id);
    item_styles.sort_by_key(|element| element.node_id);
    assert_eq!(grid_styles.len(), expected_cases.len(), "Grid area count");
    assert_eq!(
        item_styles.len(),
        expected_cases.len(),
        "Grid item style count"
    );
    let painted_grid_count = scene
        .boxes()
        .iter()
        .filter(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(color)
                    if color == parse_color("#dbeafe")
            )
        })
        .count();
    let painted_item_count = scene
        .boxes()
        .iter()
        .filter(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(color)
                    if color == parse_color("#f97316")
            )
        })
        .count();
    assert_eq!(
        painted_grid_count,
        expected_cases.len(),
        "painted Grid area count"
    );
    assert_eq!(painted_item_count, 6, "zero-area items are not painted");

    let frame_for = |node_id: spinon_core::NodeId| {
        completed
            .frames
            .iter()
            .find(|frame| frame.node_id == node_id.get())
            .copied()
            .unwrap_or_else(|| panic!("missing layout frame for node {node_id}"))
    };
    let expected_parent_alignment = [
        ("normal", "normal"),
        ("start", "start"),
        ("start", "start"),
        ("normal", "normal"),
        ("start", "start"),
        ("start", "start"),
        ("start", "start"),
        ("start", "start"),
        ("start", "start"),
        ("start", "start"),
    ];
    for (index, ((grid, item), case)) in grid_styles
        .iter()
        .zip(&item_styles)
        .zip(expected_cases)
        .enumerate()
    {
        assert_eq!(
            grid.properties.get("align-items").map(String::as_str),
            Some(expected_parent_alignment[index].0),
            "{} parent align-items",
            case["id"]
        );
        assert_eq!(
            grid.properties.get("justify-items").map(String::as_str),
            Some(expected_parent_alignment[index].1),
            "{} parent justify-items",
            case["id"]
        );
        assert_eq!(
            item.properties.get("align-self").map(String::as_str),
            case["expectedComputedAlignSelf"].as_str(),
            "{} computed align-self",
            case["id"]
        );
        assert_eq!(
            item.properties.get("justify-self").map(String::as_str),
            case["expectedComputedJustifySelf"].as_str(),
            "{} computed justify-self",
            case["id"]
        );

        let grid_frame = frame_for(grid.node_id);
        let item_frame = frame_for(item.node_id);
        let actual = [
            item_frame.x - grid_frame.x,
            item_frame.y - grid_frame.y,
            item_frame.width,
            item_frame.height,
        ];
        let expected_frame = &case["expectedItemFrame"];
        for (component, value) in actual.into_iter().enumerate() {
            let expected = expected_frame[["x", "y", "width", "height"][component]]
                .as_f64()
                .unwrap_or_else(|| panic!("missing expected frame component {component}"))
                as f32;
            assert!(
                (value - expected).abs() <= 1.0 / 64.0,
                "scene {} frame component {component}: actual={value}, expected={expected} ± 1/64 CSS px",
                case["id"].as_str().unwrap_or("<unknown>")
            );
        }
        assert_eq!(grid_frame.width, 40.0, "scene {index} Grid width");
        assert_eq!(grid_frame.height, 30.0, "scene {index} Grid height");
        assert!(item_frame.x + 1.0 / 64.0 >= grid_frame.x);
        assert!(item_frame.y + 1.0 / 64.0 >= grid_frame.y);
        assert!(item_frame.x + item_frame.width <= grid_frame.x + grid_frame.width + 1.0 / 64.0);
        assert!(item_frame.y + item_frame.height <= grid_frame.y + grid_frame.height + 1.0 / 64.0);
    }
}

#[test]
fn c11_5_root_html_grid_item_auto_stretch_rejects_out_of_scope_inputs_atomically() {
    let item = "<div class=\"item\" style=\"align-self:auto;justify-self:auto\"></div>";
    let cases = [
        (
            "aspect ratio",
            item,
            "<div class=\"item\" style=\"aspect-ratio:1;align-self:auto;justify-self:auto\"></div>",
        ),
        (
            "content-box sizing",
            item,
            "<div class=\"item\" style=\"box-sizing:content-box;align-self:auto;justify-self:auto\"></div>",
        ),
        (
            "nonzero margin",
            item,
            "<div class=\"item\" style=\"margin-left:1px;align-self:auto;justify-self:auto\"></div>",
        ),
        (
            "nonzero padding",
            item,
            "<div class=\"item\" style=\"padding-top:1px;align-self:auto;justify-self:auto\"></div>",
        ),
        (
            "nonzero border",
            item,
            "<div class=\"item\" style=\"border-top:1px solid #000;align-self:auto;justify-self:auto\"></div>",
        ),
        (
            "min-size constraint",
            item,
            "<div class=\"item\" style=\"min-width:1px;align-self:auto;justify-self:auto\"></div>",
        ),
        (
            "max-size constraint",
            item,
            "<div class=\"item\" style=\"max-height:29px;align-self:auto;justify-self:auto\"></div>",
        ),
        (
            "percentage size",
            item,
            "<div class=\"item\" style=\"width:50%;align-self:auto;justify-self:auto\"></div>",
        ),
        (
            "unsupported self alignment",
            item,
            "<div class=\"item\" style=\"align-self:baseline;justify-self:auto\"></div>",
        ),
        (
            "content-bearing item",
            item,
            "<div class=\"item\" style=\"align-self:auto;justify-self:auto\">x</div>",
        ),
        (
            "replaced item",
            item,
            "<img class=\"item\" style=\"align-self:auto;justify-self:auto\" />",
        ),
        (
            "spanning item",
            item,
            "<div class=\"item\" style=\"grid-column:1 / span 2;align-self:auto;justify-self:auto\"></div>",
        ),
        (
            "auto row track",
            "grid-template-rows:30px",
            "grid-template-rows:auto",
        ),
        (
            "nonzero Grid gap",
            "column-gap:0px; row-gap:0px",
            "column-gap:0px; row-gap:1px",
        ),
        (
            "content-box Grid container",
            ".grid { display:grid;",
            ".grid { box-sizing:content-box; display:grid;",
        ),
        (
            "implicit auto-placement row",
            "<div class=\"grid\" style=\"align-items:normal;justify-items:normal\"><div class=\"item\" style=\"align-self:auto;justify-self:auto\"></div></div>",
            "<div class=\"grid\" style=\"align-items:normal;justify-items:normal\"><div class=\"item\" style=\"align-self:auto;justify-self:auto\"></div><div class=\"item\" style=\"align-self:auto;justify-self:auto\"></div></div>",
        ),
    ];
    for (label, needle, replacement) in cases {
        let source = AUTO_STRETCH_SOURCE.replacen(needle, replacement, 1);
        assert_ne!(
            source, AUTO_STRETCH_SOURCE,
            "{label} control must modify input"
        );
        let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(&source)
            .unwrap_or_else(|error| panic!("{label} fixture should initialize: {error}"));
        let mut media = spinon_style::CssMediaEnvironment::MOBILE;
        media.color_scheme = spinon_style::CssColorScheme::Light;
        session
            .set_root_html_environment(1280.0, 1100.0, 1.0, media)
            .expect("Root HTML metadata should configure the CSS environment");
        let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
        assert_ne!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{label} must be rejected before scene publication"
        );
        assert!(
            layout
                .completed
                .as_ref()
                .is_none_or(|completed| completed.render_snapshot.is_none()),
            "{label} must not publish a partial scene"
        );
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

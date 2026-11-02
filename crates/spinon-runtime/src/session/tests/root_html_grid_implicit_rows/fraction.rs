use super::*;

const FRACTION_SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-auto-row-fr-runtime.html"
);

const GRID_BACKGROUND: spinon_render::OpaqueCssSrgb =
    spinon_render::OpaqueCssSrgb::new(221, 221, 221);
const FRACTION_RED: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(229, 72, 77);
const FRACTION_BLUE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(59, 130, 246);
const FRACTION_GREEN: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(58, 143, 91);

fn fraction_auto_rows() -> spinon_style::ComputedCssGridAutoRows {
    spinon_style::ComputedCssGridAutoRows::TrackList(vec![
        spinon_style::ComputedCssGridTrackSize::Fraction(1.0),
        spinon_style::ComputedCssGridTrackSize::Fraction(2.0),
    ])
}

#[test]
fn c11_1_root_html_default_runtime_cycles_fractional_implicit_rows() {
    let (session, layout) = run(FRACTION_SOURCE);
    let scene = ready_scene_with_auto_rows(&session, &layout, fraction_auto_rows());

    assert_eq!(scene.boxes().len(), 13);
    assert_eq!(
        frame_for_paint(&scene, GRID_BACKGROUND),
        [0.0, 0.0, 100.0, 100.0]
    );
    assert_eq!(
        frame_for_paint(&scene, FRACTION_RED),
        [0.0, 0.0, 40.0, 20.0]
    );
    assert_eq!(
        frame_for_paint(&scene, FRACTION_BLUE),
        [45.0, 0.0, 55.0, 20.0]
    );
    assert_eq!(
        frames_for_paint(&scene, FRACTION_GREEN),
        vec![
            [0.0, 25.0, 40.0, 10.0],
            [45.0, 25.0, 55.0, 10.0],
            [0.0, 40.0, 40.0, 10.0],
            [45.0, 40.0, 55.0, 10.0],
            [0.0, 65.0, 40.0, 10.0],
            [45.0, 65.0, 55.0, 10.0],
            [0.0, 80.0, 40.0, 10.0],
            [45.0, 80.0, 55.0, 10.0],
        ]
    );
}

#[test]
fn c11_1_root_html_fractional_implicit_rows_ignore_class_names_and_follow_source_order() {
    let renamed = FRACTION_SOURCE
        .replace(".item", ".tile")
        .replace("class=\"item", "class=\"tile")
        .replace("#item1", "#first")
        .replace("#item2", "#second")
        .replace("id=\"item1\"", "id=\"first\"")
        .replace("id=\"item2\"", "id=\"second\"");
    let (session, layout) = run(&renamed);
    let scene = ready_scene_with_auto_rows(&session, &layout, fraction_auto_rows());
    assert_eq!(
        frame_for_paint(&scene, FRACTION_RED),
        [0.0, 0.0, 40.0, 20.0]
    );
    assert_eq!(
        frame_for_paint(&scene, FRACTION_BLUE),
        [45.0, 0.0, 55.0, 20.0]
    );

    let reordered = FRACTION_SOURCE.replace(
        "<div id=\"item1\" class=\"item\"></div>\n      <div id=\"item2\" class=\"item\"></div>",
        "<div id=\"item2\" class=\"item\"></div>\n      <div id=\"item1\" class=\"item\"></div>",
    );
    let (session, layout) = run(&reordered);
    let scene = ready_scene_with_auto_rows(&session, &layout, fraction_auto_rows());
    assert_eq!(
        frame_for_paint(&scene, FRACTION_RED),
        [45.0, 0.0, 55.0, 20.0]
    );
    assert_eq!(
        frame_for_paint(&scene, FRACTION_BLUE),
        [0.0, 0.0, 40.0, 20.0]
    );
}

#[test]
fn c11_1_root_html_fractional_implicit_rows_preserve_visible_percentage_overflow() {
    let source = FRACTION_SOURCE.replace("grid-auto-rows: 1fr 2fr;", "grid-auto-rows: 120% 1fr;");
    let (session, layout) = run(&source);
    let expected = spinon_style::ComputedCssGridAutoRows::TrackList(vec![
        spinon_style::ComputedCssGridTrackSize::Percentage(1.2),
        spinon_style::ComputedCssGridTrackSize::Fraction(1.0),
    ]);
    let scene = ready_scene_with_auto_rows(&session, &layout, expected);
    assert_eq!(
        frame_for_paint(&scene, GRID_BACKGROUND),
        [0.0, 0.0, 100.0, 100.0]
    );
    assert_eq!(
        frame_for_paint(&scene, FRACTION_RED),
        [0.0, 0.0, 40.0, 20.0]
    );
    assert_eq!(
        frame_for_paint(&scene, FRACTION_BLUE),
        [45.0, 0.0, 55.0, 20.0]
    );
    assert_eq!(
        frames_for_paint(&scene, FRACTION_GREEN),
        vec![
            [0.0, 25.0, 40.0, 10.0],
            [45.0, 25.0, 55.0, 10.0],
            [0.0, 150.0, 40.0, 10.0],
            [45.0, 150.0, 55.0, 10.0],
            [0.0, 165.0, 40.0, 10.0],
            [45.0, 165.0, 55.0, 10.0],
            [0.0, 290.0, 40.0, 10.0],
            [45.0, 290.0, 55.0, 10.0],
        ]
    );
}

#[test]
fn c11_1_root_html_fractional_implicit_rows_reject_indefinite_or_constrained_height() {
    let cases = [
        (
            "indefinite height",
            FRACTION_SOURCE.replace("height: 100px;", "height: auto;"),
        ),
        (
            "min-height override",
            FRACTION_SOURCE.replace("height: 100px;", "height: 100px; min-height: 10px;"),
        ),
        (
            "max-height override",
            FRACTION_SOURCE.replace("height: 100px;", "height: 100px; max-height: 100px;"),
        ),
        (
            "aspect-ratio override",
            FRACTION_SOURCE.replace("height: 100px;", "height: 100px; aspect-ratio: 1;"),
        ),
    ];

    for (name, source) in cases {
        let (_session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "case={name}; expected a fail-closed layout result"
        );
        assert!(layout.error.is_some(), "case={name}; missing error");
        assert!(
            layout.completed.is_none(),
            "case={name}; failed layout must not publish a scene"
        );
    }
}

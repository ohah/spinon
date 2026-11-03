use super::*;
use spinon_style::{ComputedCssGridAutoRows, ComputedCssGridTrackSize, ComputedCssMath};

const CALC_SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-auto-row-calc-runtime.html"
);
const CALC_GRID: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(16, 24, 32);
const CALC_ITEM: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(229, 72, 77);

fn calc_auto_rows() -> ComputedCssGridAutoRows {
    ComputedCssGridAutoRows::TrackList(vec![
        ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
            ComputedCssMath::Percentage(0.1),
            ComputedCssMath::LengthPx(2.0),
        ])),
        ComputedCssGridTrackSize::Percentage(0.25),
        ComputedCssGridTrackSize::LengthPx(15.0),
    ])
}

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert_eq!(source.matches(from).count(), 1, "fixture token {from:?}");
    source.replacen(from, to, 1)
}

fn with_item_count(source: &str, count: usize) -> String {
    assert!(count <= 8);
    let mut source = source.to_owned();
    for index in (count + 1..=8).rev() {
        let item = format!("      <div class=\"item\" id=\"item{index}\"></div>\n");
        assert_eq!(source.matches(&item).count(), 1, "fixture item {index}");
        source = source.replacen(&item, "", 1);
    }
    source
}

fn assert_frames(
    scene: &spinon_render::RuntimeRenderSnapshot,
    color: spinon_render::OpaqueCssSrgb,
    expected: &[[f32; 4]],
) {
    let mut actual = scene
        .boxes()
        .iter()
        .filter(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(paint) if paint == color
            )
        })
        .map(|render_box| {
            let frame = render_box.frame_css_px();
            [frame.x(), frame.y(), frame.width(), frame.height()]
        })
        .collect::<Vec<_>>();
    actual.sort_by(|left, right| {
        left[1]
            .total_cmp(&right[1])
            .then(left[0].total_cmp(&right[0]))
    });
    assert_eq!(actual.len(), expected.len(), "paint color {color:?}");
    for (actual, expected) in actual.iter().zip(expected) {
        for (axis, (actual, expected)) in actual.iter().zip(expected).enumerate() {
            assert!(
                (actual - expected).abs() <= 0.02,
                "paint={color:?}, axis={axis}, actual={actual:?}, expected={expected:?} ± 0.02"
            );
        }
    }
}

fn assert_ready_scene(
    session: &RuntimeSession,
    layout: &crate::session::RuntimeLayoutSnapshot,
    expected_rows: ComputedCssGridAutoRows,
) -> spinon_render::RuntimeRenderSnapshot {
    let cascade = session.ua_cascade_snapshot();
    let computed_rows = cascade
        .completed
        .as_ref()
        .into_iter()
        .flat_map(|completed| completed.roots.iter())
        .flat_map(|root| root.styles.elements.iter())
        .filter_map(|style| style.grid_layout.as_ref())
        .map(|grid| grid.auto_rows.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; cascade state={:?}, error={:?}; computed auto rows={computed_rows:?}",
        layout.error,
        cascade.state,
        cascade.error
    );
    let completed = layout.completed.as_ref().expect("layout should complete");
    let scene = completed
        .render_snapshot
        .clone()
        .expect("default Root HTML runtime should publish a render scene");
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let cascade_completed = cascade.completed.expect("computed styles should complete");
    assert_eq!(
        completed.key, cascade_completed.key,
        "layout and computed styles must describe the same document revisions"
    );
    let grid = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find_map(|style| style.grid_layout.as_ref())
        .expect("parsed root Grid styles should remain observable");
    assert_eq!(grid.auto_rows, expected_rows);
    scene
}

fn frames_at_y(positions: &[f32]) -> Vec<[f32; 4]> {
    positions.iter().map(|y| [0.0, *y, 10.0, 10.0]).collect()
}

#[test]
fn c11_1_root_html_default_runtime_projects_calc_rows_and_places_the_cycled_pattern() {
    let (session, layout) = run(CALC_SOURCE);
    let scene = assert_ready_scene(&session, &layout, calc_auto_rows());
    assert_frames(&scene, CALC_GRID, &[[0.0, 0.0, 50.0, 100.0]]);
    assert_frames(
        &scene,
        CALC_ITEM,
        &frames_at_y(&[0.0, 13.0, 36.0, 51.0, 79.0, 97.0, 112.0, 140.0]),
    );
}

#[test]
fn c11_1_root_html_default_runtime_clamps_negative_rows_and_keeps_overflow() {
    let source = CALC_SOURCE
        .replace("grid-template-rows: 10px 20px;", "grid-template-rows: 0px;")
        .replace(
            "grid-auto-rows: calc(10% + 2px) 25% 15px;",
            "grid-auto-rows: calc(1px - 10%);",
        );
    let (session, layout) = run(&source);
    let negative_rows = ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Calc(
        ComputedCssMath::Sum(vec![
            ComputedCssMath::Percentage(-0.1),
            ComputedCssMath::LengthPx(1.0),
        ]),
    )]);
    let scene = assert_ready_scene(&session, &layout, negative_rows);
    assert_frames(&scene, CALC_GRID, &[[0.0, 0.0, 50.0, 100.0]]);
    assert_frames(
        &scene,
        CALC_ITEM,
        &frames_at_y(&[0.0, 3.0, 6.0, 9.0, 12.0, 15.0, 18.0, 21.0]),
    );

    let source = CALC_SOURCE
        .replace("grid-template-rows: 10px 20px;", "grid-template-rows: 0px;")
        .replace(
            "grid-auto-rows: calc(10% + 2px) 25% 15px;",
            "grid-auto-rows: calc(120% + 1px);",
        );
    let source = with_item_count(&source, 3);
    let (session, layout) = run(&source);
    let overflowing_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Calc(
            ComputedCssMath::Sum(vec![
                ComputedCssMath::Percentage(1.2),
                ComputedCssMath::LengthPx(1.0),
            ]),
        )]);
    let scene = assert_ready_scene(&session, &layout, overflowing_rows);
    assert_frames(&scene, CALC_GRID, &[[0.0, 0.0, 50.0, 100.0]]);
    assert_frames(&scene, CALC_ITEM, &frames_at_y(&[0.0, 3.0, 127.0]));
}

#[test]
fn c11_1_root_html_default_runtime_uses_zero_basis_and_keeps_gap_separate() {
    let source = CALC_SOURCE
        .replace("grid-template-rows: 10px 20px;", "grid-template-rows: 0px;")
        .replace("height: 100px;", "height: 0px;")
        .replace(
            "grid-auto-rows: calc(10% + 2px) 25% 15px;",
            "grid-auto-rows: calc(10% + 5px);",
        )
        .replace("row-gap: 3px;", "row-gap: 0px;");
    let source = with_item_count(&source, 3);
    let (session, layout) = run(&source);
    let zero_rows = ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::Calc(
        ComputedCssMath::Sum(vec![
            ComputedCssMath::Percentage(0.1),
            ComputedCssMath::LengthPx(5.0),
        ]),
    )]);
    let scene = assert_ready_scene(&session, &layout, zero_rows.clone());
    assert_frames(&scene, CALC_ITEM, &frames_at_y(&[0.0, 0.0, 5.0]));

    let source = CALC_SOURCE
        .replace("grid-template-rows: 10px 20px;", "grid-template-rows: 0px;")
        .replace(
            "grid-auto-rows: calc(10% + 2px) 25% 15px;",
            "grid-auto-rows: calc(10% + 5px);",
        )
        .replace("row-gap: 3px;", "row-gap: 10px;");
    let source = with_item_count(&source, 3);
    let (session, layout) = run(&source);
    let scene = assert_ready_scene(&session, &layout, zero_rows);
    assert_frames(&scene, CALC_ITEM, &frames_at_y(&[0.0, 10.0, 35.0]));
}

#[test]
fn c11_1_root_html_default_runtime_distributes_calc_neighbor_fr_remaining_space() {
    let source = CALC_SOURCE
        .replace(
            "grid-auto-rows: calc(10% + 2px) 25% 15px;",
            "grid-auto-rows: calc(10% + 5px) 1fr;",
        )
        .replace("row-gap: 3px;", "row-gap: 5px;");
    let source = with_item_count(&source, 5);
    let (session, layout) = run(&source);
    let expected_rows = ComputedCssGridAutoRows::TrackList(vec![
        ComputedCssGridTrackSize::Calc(ComputedCssMath::Sum(vec![
            ComputedCssMath::Percentage(0.1),
            ComputedCssMath::LengthPx(5.0),
        ])),
        ComputedCssGridTrackSize::Fraction(1.0),
    ]);
    let scene = assert_ready_scene(&session, &layout, expected_rows);
    assert_frames(
        &scene,
        CALC_ITEM,
        &frames_at_y(&[0.0, 15.0, 40.0, 60.0, 85.0]),
    );
}

#[test]
fn c11_1_root_html_default_runtime_ignores_syntax_invalid_calc_and_rejects_valid_unsupported_calc()
{
    let syntax_invalid = replace_once(
        CALC_SOURCE,
        "grid-auto-rows: calc(10% + 2px) 25% 15px;",
        "grid-auto-rows: calc(10% + 2px) 25% 15px; grid-auto-rows: calc(10% +);",
    );
    let (session, layout) = run(&syntax_invalid);
    let scene = assert_ready_scene(&session, &layout, calc_auto_rows());
    assert_frames(
        &scene,
        CALC_ITEM,
        &frames_at_y(&[0.0, 13.0, 36.0, 51.0, 79.0, 97.0, 112.0, 140.0]),
    );

    let unsupported = replace_once(
        CALC_SOURCE,
        "grid-auto-rows: calc(10% + 2px) 25% 15px;",
        "grid-auto-rows: calc(min(10%, 2px) + 3px);",
    );
    let (session, layout) = run(&unsupported);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "valid but unsupported Root HTML math must fail closed"
    );
    assert!(layout.error.is_some());
    assert!(
        layout.completed.is_none(),
        "failure must not publish a partial scene"
    );
}

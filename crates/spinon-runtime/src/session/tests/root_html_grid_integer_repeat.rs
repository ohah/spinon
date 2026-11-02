use super::*;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-integer-repeat-precomparison.html"
);

#[derive(Clone, Copy)]
enum ExpectedTrack {
    Px(f32),
    Percentage(f32),
    Fraction(f32),
}

impl ExpectedTrack {
    fn computed(self) -> spinon_style::ComputedCssGridTrackSize {
        match self {
            Self::Px(value) => spinon_style::ComputedCssGridTrackSize::LengthPx(value),
            Self::Percentage(value) => spinon_style::ComputedCssGridTrackSize::Percentage(value),
            Self::Fraction(value) => spinon_style::ComputedCssGridTrackSize::Fraction(value),
        }
    }
}

struct ExpectedCase {
    root_color: spinon_render::OpaqueCssSrgb,
    item_colors: &'static [spinon_render::OpaqueCssSrgb],
    root_frame: [f32; 4],
    columns: &'static [ExpectedTrack],
    rows: &'static [ExpectedTrack],
    column_frames: &'static [f32],
    row_frames: &'static [f32],
}

const RED: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(229, 72, 77);
const BLUE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(59, 130, 246);
const GREEN: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(22, 163, 74);
const ORANGE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(245, 158, 11);
const PURPLE: spinon_render::OpaqueCssSrgb = spinon_render::OpaqueCssSrgb::new(168, 85, 247);

const CASES: &[ExpectedCase] = &[
    ExpectedCase {
        root_color: spinon_render::OpaqueCssSrgb::new(16, 24, 32),
        item_colors: &[RED, BLUE, GREEN, ORANGE],
        root_frame: [0.0, 0.0, 160.0, 20.0],
        columns: &[
            ExpectedTrack::Px(30.0),
            ExpectedTrack::Px(50.0),
            ExpectedTrack::Px(30.0),
            ExpectedTrack::Px(50.0),
        ],
        rows: &[ExpectedTrack::Px(20.0)],
        column_frames: &[30.0, 50.0, 30.0, 50.0],
        row_frames: &[20.0],
    },
    ExpectedCase {
        root_color: spinon_render::OpaqueCssSrgb::new(17, 24, 39),
        item_colors: &[RED, BLUE, GREEN, ORANGE],
        root_frame: [0.0, 20.0, 40.0, 100.0],
        columns: &[ExpectedTrack::Px(40.0)],
        rows: &[
            ExpectedTrack::Px(20.0),
            ExpectedTrack::Px(30.0),
            ExpectedTrack::Px(20.0),
            ExpectedTrack::Px(30.0),
        ],
        column_frames: &[40.0],
        row_frames: &[20.0, 30.0, 20.0, 30.0],
    },
    ExpectedCase {
        root_color: spinon_render::OpaqueCssSrgb::new(31, 41, 55),
        item_colors: &[RED, BLUE, GREEN, ORANGE],
        root_frame: [0.0, 120.0, 200.0, 20.0],
        columns: &[
            ExpectedTrack::Percentage(0.25),
            ExpectedTrack::Percentage(0.25),
            ExpectedTrack::Percentage(0.25),
            ExpectedTrack::Percentage(0.25),
        ],
        rows: &[ExpectedTrack::Px(20.0)],
        column_frames: &[50.0, 50.0, 50.0, 50.0],
        row_frames: &[20.0],
    },
    ExpectedCase {
        root_color: spinon_render::OpaqueCssSrgb::new(15, 61, 46),
        item_colors: &[RED, BLUE, GREEN, ORANGE],
        root_frame: [0.0, 140.0, 150.0, 20.0],
        columns: &[
            ExpectedTrack::Fraction(1.0),
            ExpectedTrack::Fraction(2.0),
            ExpectedTrack::Fraction(1.0),
            ExpectedTrack::Fraction(2.0),
        ],
        rows: &[ExpectedTrack::Px(20.0)],
        column_frames: &[25.0, 50.0, 25.0, 50.0],
        row_frames: &[20.0],
    },
    ExpectedCase {
        root_color: spinon_render::OpaqueCssSrgb::new(76, 29, 149),
        item_colors: &[RED, BLUE, GREEN, ORANGE, PURPLE],
        root_frame: [0.0, 160.0, 200.0, 20.0],
        columns: &[
            ExpectedTrack::Px(20.0),
            ExpectedTrack::Percentage(0.2),
            ExpectedTrack::Fraction(1.0),
            ExpectedTrack::Percentage(0.2),
            ExpectedTrack::Fraction(1.0),
        ],
        rows: &[ExpectedTrack::Px(20.0)],
        column_frames: &[20.0, 40.0, 50.0, 40.0, 50.0],
        row_frames: &[20.0],
    },
    ExpectedCase {
        root_color: spinon_render::OpaqueCssSrgb::new(22, 78, 99),
        item_colors: &[RED, BLUE, GREEN, ORANGE, PURPLE],
        root_frame: [0.0, 180.0, 110.0, 20.0],
        columns: &[
            ExpectedTrack::Px(20.0),
            ExpectedTrack::Px(20.0),
            ExpectedTrack::Px(10.0),
            ExpectedTrack::Px(30.0),
            ExpectedTrack::Px(30.0),
        ],
        rows: &[ExpectedTrack::Px(20.0)],
        column_frames: &[20.0, 20.0, 10.0, 30.0, 30.0],
        row_frames: &[20.0],
    },
    ExpectedCase {
        root_color: spinon_render::OpaqueCssSrgb::new(113, 63, 18),
        item_colors: &[RED, BLUE],
        root_frame: [0.0, 200.0, 80.0, 20.0],
        columns: &[ExpectedTrack::Px(30.0), ExpectedTrack::Px(50.0)],
        rows: &[ExpectedTrack::Px(20.0)],
        column_frames: &[30.0, 50.0],
        row_frames: &[20.0],
    },
];

fn run(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("parsed Root HTML should initialize the default runtime");
    session
        .set_root_html_environment(
            1280.0,
            633.0,
            1.0,
            spinon_style::CssMediaEnvironment::MOBILE,
        )
        .expect("root metadata should configure the CSS environment");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    (session, layout)
}

fn assert_frame(
    scene: &spinon_render::RuntimeRenderSnapshot,
    paint: spinon_render::OpaqueCssSrgb,
    expected: [f32; 4],
) {
    let found = scene.boxes().iter().any(|render_box| {
        if !matches!(
            render_box.paint(),
            spinon_render::RuntimePaint::Opaque(color) if color == paint
        ) {
            return false;
        }
        let frame = render_box.frame_css_px();
        [frame.x(), frame.y(), frame.width(), frame.height()]
            .into_iter()
            .zip(expected)
            .all(|(actual, expected)| actual.is_finite() && (actual - expected).abs() <= 0.02)
    });
    assert!(
        found,
        "expected painted frame {expected:?} for {paint:?}; observed {:?}",
        scene
            .boxes()
            .iter()
            .filter_map(|render_box| {
                let spinon_render::RuntimePaint::Opaque(color) = render_box.paint() else {
                    return None;
                };
                let frame = render_box.frame_css_px();
                Some((color, [frame.x(), frame.y(), frame.width(), frame.height()]))
            })
            .collect::<Vec<_>>()
    );
}

fn assert_expected_source(source: &str) {
    let (session, layout) = run(source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "error={:?}; cascade state={:?}; cascade error={:?}",
        layout.error,
        session.ua_cascade_snapshot().state,
        session.ua_cascade_snapshot().error
    );
    let completed = layout.completed.expect("integer repeat should lay out");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML runtime should publish the scene");
    let cascade = session.ua_cascade_snapshot();
    let cascade_completed = cascade
        .completed
        .expect("computed Grid styles should be available");
    assert_eq!(completed.key, cascade_completed.key);

    let grids = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .filter_map(|element| element.grid_layout.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(grids.len(), CASES.len());

    let mut expected_box_count = 0;
    for (index, (grid, case)) in grids.iter().zip(CASES).enumerate() {
        let expected_columns = case
            .columns
            .iter()
            .copied()
            .map(ExpectedTrack::computed)
            .collect::<Vec<_>>();
        let expected_rows = case
            .rows
            .iter()
            .copied()
            .map(ExpectedTrack::computed)
            .collect::<Vec<_>>();
        assert_eq!(grid.columns, expected_columns, "grid case {index} columns");
        assert_eq!(grid.rows, expected_rows, "grid case {index} rows");
        assert!(case.item_colors.len() <= case.column_frames.len() * case.row_frames.len());
        assert_frame(scene, case.root_color, case.root_frame);
        expected_box_count += 1;

        for (item_index, color) in case.item_colors.iter().copied().enumerate() {
            let column = item_index % case.column_frames.len();
            let row = item_index / case.column_frames.len();
            let x = case.root_frame[0] + case.column_frames[..column].iter().sum::<f32>();
            let y = case.root_frame[1] + case.row_frames[..row].iter().sum::<f32>();
            assert_frame(
                scene,
                color,
                [x, y, case.column_frames[column], case.row_frames[row]],
            );
            expected_box_count += 1;
        }
    }
    assert_eq!(
        scene
            .boxes()
            .iter()
            .filter(|render_box| {
                matches!(render_box.paint(), spinon_render::RuntimePaint::None)
            })
            .count(),
        2,
        "only the transparent html/body wrappers should lack paint"
    );
    assert_eq!(scene.boxes().len(), expected_box_count + 2);
}

#[test]
fn c11_1_root_html_default_runtime_expands_integer_repeat_for_bounded_direct_tracks() {
    assert_expected_source(SOURCE);
}

#[test]
fn c11_1_root_html_integer_repeat_matches_equivalent_direct_track_control() {
    let direct = SOURCE
        .replace("repeat(2, 30px 50px)", "30px 50px 30px 50px")
        .replace("repeat(2, 20px 30px)", "20px 30px 20px 30px")
        .replace("repeat(2, 25% 25%)", "25% 25% 25% 25%")
        .replace("repeat(2, 1fr 2fr)", "1fr 2fr 1fr 2fr")
        .replace("repeat(2, 20% 1fr)", "20% 1fr 20% 1fr")
        .replace("repeat(2, 20px)", "20px 20px")
        .replace("repeat(2, 30px)", "30px 30px")
        .replace("repeat(1, 30px 50px)", "30px 50px");
    assert_expected_source(&direct);
}

#[test]
fn c11_1_root_html_integer_repeat_preserves_named_tracks_and_dom_source_order() {
    let renamed = SOURCE
        .replace("<section", "<div")
        .replace("</section>", "</div>")
        .replace("class=\"grid\"", "class=\"tiles\"")
        .replace(".grid {", ".tiles {")
        .replace(".grid > div", ".tiles > div")
        .replace("#px-columns", "#renamed-columns")
        .replace("id=\"px-columns\"", "id=\"renamed-columns\"");
    assert_expected_source(&renamed);

    let reversed = SOURCE.replace(
        "<div class=\"a\" style=\"width:30px;height:20px\"></div>\n    <div class=\"b\" style=\"width:50px;height:20px\"></div>\n    <div class=\"c\" style=\"width:30px;height:20px\"></div>\n    <div class=\"d\" style=\"width:50px;height:20px\"></div>",
        "<div class=\"d\" style=\"width:50px;height:20px\"></div>\n    <div class=\"c\" style=\"width:30px;height:20px\"></div>\n    <div class=\"b\" style=\"width:50px;height:20px\"></div>\n    <div class=\"a\" style=\"width:30px;height:20px\"></div>",
    );
    assert_ne!(
        reversed, SOURCE,
        "the reverse-order control must alter source"
    );
    let (session, layout) = run(&reversed);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "reordered items should still lay out; error={:?}; cascade={:?}",
        layout.error,
        session.ua_cascade_snapshot()
    );
    let scene = layout
        .completed
        .as_ref()
        .and_then(|completed| completed.render_snapshot.as_ref())
        .expect("reordered items should publish a scene");
    assert_frame(scene, RED, [110.0, 0.0, 30.0, 20.0]);
    assert_frame(scene, BLUE, [80.0, 0.0, 50.0, 20.0]);
    assert_frame(scene, GREEN, [30.0, 0.0, 30.0, 20.0]);
    assert_frame(scene, ORANGE, [0.0, 0.0, 50.0, 20.0]);
}

#[test]
fn c11_1_root_html_default_runtime_ignores_invalid_repeat_declaration_after_valid_tracks() {
    let source = SOURCE.replace(
        "grid-template-columns: repeat(2, 30px 50px);",
        "grid-template-columns: 30px 50px 30px 50px;\n      grid-template-columns: repeat(0, 30px);",
    );
    assert_expected_source(&source);
}

#[test]
fn c11_1_root_html_integer_repeat_does_not_publish_area_inferred_empty_tracks() {
    let source = SOURCE.replace(
        "grid-template-columns: repeat(2, 30px 50px);\n      grid-template-rows: 20px;",
        "grid-template-columns: none;\n      grid-template-rows: none;\n      grid-template-areas: \"header header\" \"side content\";",
    );
    assert_ne!(
        source, SOURCE,
        "the empty-track area control must change source"
    );

    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Failed,
        "integer-repeat admission must not widen the Root HTML named-area track profile"
    );
    assert!(layout.error.is_some());
    assert!(layout.completed.is_none());
    let cascade = session.ua_cascade_snapshot();
    let cascade_completed = cascade
        .completed
        .expect("the complete typed cascade may exist before Root HTML projection rejects it");
    let grid = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find(|element| {
            element
                .properties
                .get("display")
                .is_some_and(|value| value == "grid")
        })
        .and_then(|element| element.grid_layout.as_ref())
        .expect("the complete cascade should retain the authored empty track lists");
    assert!(grid.columns.is_empty());
    assert!(grid.rows.is_empty());
}

#[test]
fn c11_1_root_html_integer_repeat_rejects_out_of_profile_and_over_budget_inputs() {
    let oversized_name = "x".repeat(128);
    let over_budget_names = SOURCE.replace(
        "repeat(2, 30px 50px)",
        &format!("repeat(10000, [{oversized_name}] 1px)"),
    );
    let cases = [
        (
            "auto-repeat",
            SOURCE.replace("repeat(2, 30px 50px)", "repeat(auto-fill, 30px 50px)"),
        ),
        (
            "named lines",
            SOURCE.replace("repeat(2, 30px 50px)", "repeat(2, [edge] 30px 50px)"),
        ),
        (
            "intrinsic track",
            SOURCE.replace("repeat(2, 30px 50px)", "repeat(2, min-content 50px)"),
        ),
        (
            "explicit track cap",
            SOURCE.replace("repeat(2, 30px 50px)", "repeat(10001, 1px)"),
        ),
        ("line metadata quota", over_budget_names),
    ];

    for (name, source) in cases {
        let (session, layout) = run(&source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{name} must fail closed; layout={layout:?}; cascade={:?}",
            session.ua_cascade_snapshot()
        );
        assert!(layout.error.is_some(), "{name} should keep its diagnostic");
        assert!(
            layout.completed.is_none(),
            "{name} must not publish a partial layout or scene"
        );
    }
}

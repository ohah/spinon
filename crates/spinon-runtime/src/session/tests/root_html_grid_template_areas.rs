use super::*;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-template-areas-precomparison.html"
);
const FRACTION_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-fr-named-areas-runtime.html"
);
const PERCENTAGE_CASES: [(&str, &str); 4] = [
    (
        "both-axes",
        include_str!(
            "../../../../../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/both-axes.html"
        ),
    ),
    (
        "mixed-px-percent",
        include_str!(
            "../../../../../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/mixed-px-percent.html"
        ),
    ),
    (
        "overflowing-columns",
        include_str!(
            "../../../../../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/overflowing-columns.html"
        ),
    ),
    (
        "zero-track",
        include_str!(
            "../../../../../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/zero-track.html"
        ),
    ),
];

fn run(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("parsed root HTML should initialize the default runtime");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(1280.0, 633.0, 1.0, media)
        .expect("root metadata should configure the CSS environment");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    (session, layout)
}

fn frame_for_paint(
    scene: &spinon_render::RuntimeRenderSnapshot,
    expected: spinon_render::OpaqueCssSrgb,
) -> [f32; 4] {
    let render_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(
                render_box.paint(),
                spinon_render::RuntimePaint::Opaque(color) if color == expected
            )
        })
        .expect("semantic area item should have its computed paint");
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

fn area_frames(scene: &spinon_render::RuntimeRenderSnapshot) -> [[f32; 4]; 3] {
    [
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(229, 72, 77)),
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(58, 143, 91)),
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(10, 132, 255)),
    ]
}

fn assert_frame_close(actual: [f32; 4], expected: [f32; 4], name: &str) {
    for (index, (actual_component, expected_component)) in actual.iter().zip(expected).enumerate() {
        assert!(
            actual_component.is_finite() && (*actual_component - expected_component).abs() <= 0.02,
            "{name} frame component {index}: actual={actual_component}, expected={expected_component}, full actual={actual:?}"
        );
    }
}

#[test]
fn c11_1_default_root_html_runtime_places_semantic_named_areas_independent_of_source_order() {
    let (session, baseline) = run(SOURCE);
    assert_eq!(
        baseline.state,
        crate::session::RuntimeLayoutState::Ready,
        "{baseline:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = baseline
        .completed
        .expect("named-area Grid layout should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML runtime should publish the named-area scene");
    assert_eq!(
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(16, 24, 32)),
        [0.0, 0.0, 120.0, 70.0]
    );
    let expected = [
        [0.0, 0.0, 120.0, 20.0],
        [0.0, 20.0, 40.0, 50.0],
        [40.0, 20.0, 80.0, 50.0],
    ];
    assert_eq!(area_frames(scene), expected);

    let cascade = session.ua_cascade_snapshot();
    assert_eq!(cascade.state, RuntimeUaCascadeState::Ready);
    let cascade_completed = cascade
        .completed
        .expect("named-area typed styles should complete");
    assert_eq!(
        completed.key, cascade_completed.key,
        "layout and typed areas must use one document/style/environment revision"
    );
    let styles = &cascade_completed.roots[0].styles;
    let grid = styles
        .elements
        .iter()
        .find_map(|style| style.grid_layout.as_ref())
        .expect("the typed root Grid should reach the layout projector");
    assert_eq!(
        grid.template_areas
            .as_ref()
            .map(|areas| (areas.rows, areas.columns)),
        Some((2, 2))
    );

    let reordered = SOURCE
        .replace("#layout", "#renamed-grid")
        .replace("id=\"layout\"", "id=\"renamed-grid\"")
        .replace("data-area", "data-zone")
        .replace(
            "<div data-zone=\"content\"></div>\n      <div data-zone=\"header\"></div>\n      <div data-zone=\"side\"></div>",
            "<div data-zone=\"side\"></div>\n      <div data-zone=\"content\"></div>\n      <div data-zone=\"header\"></div>",
        );
    let (_, reordered) = run(&reordered);
    assert_eq!(
        reordered.state,
        crate::session::RuntimeLayoutState::Ready,
        "renamed selectors and reordered items should preserve named placement: {reordered:?}"
    );
    let reordered_scene = reordered
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("reordered named-area Grid should publish a scene");
    assert_eq!(area_frames(&reordered_scene), expected);
}

#[test]
fn c11_1_default_root_html_runtime_combines_named_areas_with_definite_fraction_tracks() {
    let (session, layout) = run(FRACTION_SOURCE);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "fractional named-area Grid should be admitted: {layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .expect("fractional named-area Grid layout should complete");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("fractional named-area Grid should publish the default Root HTML scene");
    assert_frame_close(
        frame_for_paint(scene, spinon_render::OpaqueCssSrgb::new(16, 24, 32)),
        [0.0, 0.0, 120.0, 70.0],
        "fractional named-area root",
    );
    let expected = [
        [0.0, 0.0, 120.0, 70.0 / 3.0],
        [0.0, 70.0 / 3.0, 40.0, 140.0 / 3.0],
        [40.0, 70.0 / 3.0, 80.0, 140.0 / 3.0],
    ];
    for (index, (actual, expected)) in area_frames(scene).into_iter().zip(expected).enumerate() {
        assert_frame_close(actual, expected, ["header", "side", "content"][index]);
    }

    let cascade = session.ua_cascade_snapshot();
    let cascade_completed = cascade
        .completed
        .expect("fractional area typed style should complete");
    assert_eq!(completed.key, cascade_completed.key);
    let root_grid = cascade_completed.roots[0]
        .styles
        .elements
        .iter()
        .find(|style| style.grid_layout.is_some())
        .and_then(|style| style.grid_layout.as_ref())
        .expect("fractional named-area root should retain typed tracks");
    assert_eq!(
        root_grid.columns,
        [
            spinon_style::ComputedCssGridTrackSize::Fraction(1.0),
            spinon_style::ComputedCssGridTrackSize::Fraction(2.0),
        ]
    );
    assert_eq!(root_grid.rows, root_grid.columns);
    assert_eq!(
        root_grid
            .template_areas
            .as_ref()
            .map(|areas| (areas.rows, areas.columns)),
        Some((2, 2))
    );

    let reordered_source = FRACTION_SOURCE
        .replace("#layout", "#renamed-grid")
        .replace("id=\"layout\"", "id=\"renamed-grid\"")
        .replace("data-area", "data-zone")
        .replace(
            "<div data-zone=\"content\"></div>\n      <div data-zone=\"header\"></div>\n      <div data-zone=\"side\"></div>",
            "<div data-zone=\"side\"></div>\n      <div data-zone=\"content\"></div>\n      <div data-zone=\"header\"></div>",
        );
    let (_, reordered) = run(&reordered_source);
    assert_eq!(
        reordered.state,
        crate::session::RuntimeLayoutState::Ready,
        "renamed selector and source order must not block fractional named areas: {reordered:?}"
    );
    let reordered_scene = reordered
        .completed
        .and_then(|completed| completed.render_snapshot.clone())
        .expect("reordered fractional named areas should publish a scene");
    for (index, (actual, expected)) in area_frames(&reordered_scene)
        .into_iter()
        .zip(expected)
        .enumerate()
    {
        assert_frame_close(
            actual,
            expected,
            ["reordered header", "reordered side", "reordered content"][index],
        );
    }
}

#[test]
fn c11_1_default_root_html_runtime_combines_named_areas_with_percentage_tracks() {
    use spinon_render::OpaqueCssSrgb;
    use spinon_style::ComputedCssGridTrackSize::{LengthPx, Percentage};

    let cases = [
        (
            "both-axes",
            [Percentage(0.25), Percentage(0.75)],
            [Percentage(0.2), Percentage(0.8)],
            [
                [0.0, 0.0, 120.0, 14.0],
                [0.0, 14.0, 30.0, 56.0],
                [30.0, 14.0, 90.0, 56.0],
            ],
        ),
        (
            "mixed-px-percent",
            [LengthPx(40.0), Percentage(0.5)],
            [Percentage(0.2), LengthPx(50.0)],
            [
                [0.0, 0.0, 100.0, 14.0],
                [0.0, 14.0, 40.0, 50.0],
                [40.0, 14.0, 60.0, 50.0],
            ],
        ),
        (
            "overflowing-columns",
            [Percentage(1.2), Percentage(0.1)],
            [Percentage(0.2), Percentage(0.8)],
            [
                [0.0, 0.0, 156.0, 14.0],
                [0.0, 14.0, 144.0, 56.0],
                [144.0, 14.0, 12.0, 56.0],
            ],
        ),
        (
            "zero-track",
            [Percentage(0.0), Percentage(1.0)],
            [Percentage(0.2), Percentage(0.8)],
            [
                [0.0, 0.0, 120.0, 14.0],
                [0.0, 14.0, 0.0, 56.0],
                [0.0, 14.0, 120.0, 56.0],
            ],
        ),
    ];
    let colors = [
        OpaqueCssSrgb::new(238, 51, 51),
        OpaqueCssSrgb::new(51, 170, 51),
        OpaqueCssSrgb::new(51, 102, 204),
    ];

    for ((name, source), (expected_name, columns, rows, expected_frames)) in
        PERCENTAGE_CASES.iter().zip(cases)
    {
        assert_eq!(
            *name, expected_name,
            "fixed source order is part of the matrix"
        );
        let (session, layout) = run(source);
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{name}: percentage named-area scene should be admitted: {layout:?}; cascade={:?}",
            session.ua_cascade_snapshot()
        );
        let completed = layout
            .completed
            .expect("percentage named-area Grid layout should complete");
        let scene = completed
            .render_snapshot
            .as_ref()
            .expect("percentage named-area Grid should publish a Root HTML scene");
        let cascade = session.ua_cascade_snapshot();
        let cascade_completed = cascade
            .completed
            .expect("percentage Grid typed styles should complete");
        assert_eq!(completed.key, cascade_completed.key, "{name} revision");
        let root_style = cascade_completed.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("percentage named-area root should retain its computed style");
        assert_eq!(
            root_style.layout_dimensions.width,
            spinon_style::ComputedCssDimension::LengthPx(120.0),
            "{name} definite width basis"
        );
        assert_eq!(
            root_style.layout_dimensions.height,
            spinon_style::ComputedCssDimension::LengthPx(70.0),
            "{name} definite height basis"
        );
        let root_grid = root_style
            .grid_layout
            .as_ref()
            .expect("percentage named-area tracks should remain typed");
        assert_eq!(root_grid.columns, columns, "{name} columns");
        assert_eq!(root_grid.rows, rows, "{name} rows");
        assert_eq!(
            root_grid
                .template_areas
                .as_ref()
                .map(|areas| (areas.rows, areas.columns)),
            Some((2, 2)),
            "{name} area map"
        );
        let root_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == root_style.node_id.get())
            .expect("percentage named-area root should retain its layout frame");
        assert_frame_close(
            [
                root_frame.x,
                root_frame.y,
                root_frame.width,
                root_frame.height,
            ],
            [0.0, 0.0, 120.0, 70.0],
            &format!("{name} root"),
        );

        for (index, (color, expected)) in colors.into_iter().zip(expected_frames).enumerate() {
            let frame = scene
                .boxes()
                .iter()
                .find_map(|render_box| {
                    matches!(
                        render_box.paint(),
                        spinon_render::RuntimePaint::Opaque(actual) if actual == color
                    )
                    .then(|| {
                        let frame = render_box.frame_css_px();
                        [frame.x(), frame.y(), frame.width(), frame.height()]
                    })
                })
                .or_else(|| {
                    let style =
                        cascade_completed.roots[0]
                            .styles
                            .elements
                            .iter()
                            .find(|style| {
                                style.background_color.is_some_and(|background| {
                                    background.red == color.red()
                                        && background.green == color.green()
                                        && background.blue == color.blue()
                                })
                            })?;
                    let frame = completed
                        .frames
                        .iter()
                        .find(|frame| frame.node_id == style.node_id.get())?;
                    Some([frame.x, frame.y, frame.width, frame.height])
                })
                .unwrap_or_else(|| {
                    panic!(
                        "{name} semantic item {} should retain its layout frame",
                        index
                    )
                });
            assert_frame_close(
                frame,
                expected,
                &format!("{name} {}", ["header", "side", "content"][index]),
            );
        }
        if *name == "overflowing-columns" {
            assert_eq!(expected_frames[2][0], 144.0);
            assert!(expected_frames[0][2] > 120.0);
            assert!(expected_frames[2][0] >= 120.0);
        }
    }
}

#[test]
fn c11_1_default_root_html_runtime_rejects_out_of_scope_percentage_named_area_inputs() {
    let source = PERCENTAGE_CASES[0].1;
    let cases = [
        (
            "indefinite column percentage basis",
            source.replace("width: 120px;", "width: auto;"),
            Some("width"),
        ),
        (
            "indefinite row percentage basis",
            source.replace("height: 70px;", "height: auto;"),
            Some("height"),
        ),
        (
            "minimum width override",
            source.replace("width: 120px;", "width: 120px; min-width: 0;"),
            Some("min-width"),
        ),
        (
            "maximum width override",
            source.replace("width: 120px;", "width: 120px; max-width: 121px;"),
            Some("max-width"),
        ),
        (
            "minimum height override",
            source.replace("height: 70px;", "height: 70px; min-height: 0;"),
            Some("min-height"),
        ),
        (
            "maximum height override",
            source.replace("height: 70px;", "height: 70px; max-height: 71px;"),
            Some("max-height"),
        ),
        (
            "aspect ratio override",
            source.replace("height: 70px;", "height: 70px; aspect-ratio: 2 / 1;"),
            Some("aspect-ratio"),
        ),
        (
            "percentage gap",
            source.replace("gap: 0;", "gap: 0 1%;"),
            Some("column-gap"),
        ),
        (
            "calc track",
            source.replace(
                "grid-template-columns: 25% 75%;",
                "grid-template-columns: calc(25% + 10px) 75%;",
            ),
            Some("grid-template-columns"),
        ),
        (
            "minmax track",
            source.replace(
                "grid-template-columns: 25% 75%;",
                "grid-template-columns: minmax(0px, 25%) 75%;",
            ),
            Some("grid-template-columns"),
        ),
        (
            "negative percentage track",
            source.replace(
                "grid-template-columns: 25% 75%;",
                "grid-template-columns: -25% 75%;",
            ),
            Some("grid-template-columns"),
        ),
    ];

    for (name, source, expected_property) in cases {
        let (session, result) = run(&source);
        assert_eq!(
            result.state,
            crate::session::RuntimeLayoutState::Failed,
            "out-of-scope percentage named-area case={name} must fail closed: {result:?}; cascade={:?}",
            session.ua_cascade_snapshot()
        );
        assert!(
            result.completed.is_none(),
            "{name} must not publish a partial scene"
        );
        let error = result
            .error
            .as_ref()
            .expect("failed case should report an error");
        if let Some(property) = expected_property
            && error.property.as_deref() != Some(property)
        {
            let cascade_error = session
                .ua_cascade_snapshot()
                .error
                .expect("cascade failure should identify the rejected property");
            assert!(
                cascade_error.contains(property),
                "{name}; expected {property}; layout={error:?}; cascade={cascade_error}"
            );
        }
    }
}

#[test]
fn c11_1_default_root_html_runtime_rejects_out_of_scope_fractional_named_area_tracks() {
    let cases = [
        (
            "wrong fraction ratio",
            FRACTION_SOURCE.replace(
                "grid-template-columns: 1fr 2fr;",
                "grid-template-columns: 1fr 1fr;",
            ),
        ),
        (
            "wrong explicit track count",
            FRACTION_SOURCE.replace(
                "grid-template-columns: 1fr 2fr;",
                "grid-template-columns: 1fr 2fr 3fr;",
            ),
        ),
        (
            "mixed px and fr tracks",
            FRACTION_SOURCE.replace(
                "grid-template-columns: 1fr 2fr;",
                "grid-template-columns: 40px 2fr;",
            ),
        ),
        (
            "percentage and fraction tracks",
            FRACTION_SOURCE.replace(
                "grid-template-columns: 1fr 2fr;",
                "grid-template-columns: 25% 1fr;",
            ),
        ),
        (
            "indefinite fraction row axis",
            FRACTION_SOURCE.replace("height: 70px;", "height: auto;"),
        ),
        (
            "minimum size override",
            FRACTION_SOURCE.replace("width: 120px;", "width: 120px; min-width: 0;"),
        ),
        (
            "maximum width override",
            FRACTION_SOURCE.replace("width: 120px;", "width: 120px; max-width: 121px;"),
        ),
        (
            "minimum height override",
            FRACTION_SOURCE.replace("height: 70px;", "height: 70px; min-height: 0;"),
        ),
        (
            "maximum height override",
            FRACTION_SOURCE.replace("height: 70px;", "height: 70px; max-height: 71px;"),
        ),
        (
            "aspect ratio override",
            FRACTION_SOURCE.replace("height: 70px;", "height: 70px; aspect-ratio: 2 / 1;"),
        ),
        (
            "nonzero row gap",
            FRACTION_SOURCE.replace(
                "grid-template-rows: 1fr 2fr;",
                "grid-template-rows: 1fr 2fr; row-gap: 1px;",
            ),
        ),
        (
            "nonzero column gap",
            FRACTION_SOURCE.replace(
                "grid-template-columns: 1fr 2fr;",
                "grid-template-columns: 1fr 2fr; column-gap: 1px;",
            ),
        ),
        (
            "intrinsic minmax track",
            FRACTION_SOURCE.replace(
                "grid-template-columns: 1fr 2fr;",
                "grid-template-columns: minmax(0px, 1fr) 2fr;",
            ),
        ),
    ];

    for (name, source) in cases {
        let (_, result) = run(&source);
        assert_eq!(
            result.state,
            crate::session::RuntimeLayoutState::Failed,
            "out-of-scope fractional named-area case={name} must fail closed: {result:?}"
        );
        assert!(
            result.error.is_some(),
            "{name} should report its unsupported input"
        );
        let failure = result.error.as_ref().expect("failure was checked above");
        let expected_property = match name {
            "wrong fraction ratio"
            | "wrong explicit track count"
            | "mixed px and fr tracks"
            | "percentage and fraction tracks" => Some("grid-template-areas"),
            "indefinite fraction row axis" => None,
            "minimum size override" => None,
            "maximum width override" => None,
            "minimum height override" => None,
            "maximum height override" => None,
            "aspect ratio override" => None,
            "nonzero row gap" => Some("row-gap"),
            "nonzero column gap" => Some("column-gap"),
            // The intrinsic form is rejected by the cascade before typed
            // projection, so only the fail-closed state is stable here.
            "intrinsic minmax track" => None,
            _ => unreachable!("unexpected rejection matrix case {name}"),
        };
        if let Some(expected_property) = expected_property {
            assert_eq!(failure.code, "unsupported_computed_value", "{name}");
            assert_eq!(
                failure.property.as_deref(),
                Some(expected_property),
                "{name} should identify the unsupported computed property"
            );
        }
        assert!(
            result.completed.is_none(),
            "{name} must not publish a partial or stale scene"
        );
    }
}

#[test]
fn c11_1_default_root_html_runtime_rejects_out_of_scope_named_area_candidates() {
    let cases = [
        (
            "wrong root width",
            SOURCE.replace("width: 120px;", "width: 121px;"),
        ),
        (
            "root max-width constraint",
            SOURCE.replace("width: 120px;", "width: 120px; max-width: 121px;"),
        ),
        (
            "wrong column tracks",
            SOURCE.replace(
                "grid-template-columns: 40px 80px;",
                "grid-template-columns: 41px 79px;",
            ),
        ),
        (
            "wrong row tracks",
            SOURCE.replace(
                "grid-template-rows: 20px 50px;",
                "grid-template-rows: 21px 49px;",
            ),
        ),
        (
            "nonzero gap",
            SOURCE.replace(
                "grid-template-rows: 20px 50px;",
                "grid-template-rows: 20px 50px; row-gap: 1px;",
            ),
        ),
        (
            "nonzero item edge",
            SOURCE.replace("grid-area: header;", "grid-area: header; margin-left: 1px;"),
        ),
        (
            "nonrectangular template",
            SOURCE.replace(
                "\"header header\" \"side content\"",
                "\"header side\" \"side content\"",
            ),
        ),
        (
            "template area and track dimension mismatch",
            SOURCE.replace(
                "grid-template-areas: \"header header\" \"side content\";",
                "grid-template-areas: \"header header\" \"side content\" \"extra extra\";",
            ),
        ),
        (
            "missing semantic item",
            SOURCE.replace("<div data-area=\"header\"></div>\n", ""),
        ),
        (
            "duplicate semantic area",
            SOURCE.replace("data-area=\"side\"", "data-area=\"header\""),
        ),
        (
            "unknown semantic area",
            SOURCE.replace("grid-area: side;", "grid-area: sidebar;"),
        ),
        (
            "extra visible item",
            SOURCE.replace("</main>", "<div data-area=\"extra\"></div>\n    </main>"),
        ),
        (
            "hidden extra item",
            SOURCE.replace("</main>", "<div style=\"display:none\"></div>\n    </main>"),
        ),
        (
            "image item",
            SOURCE.replace(
                "<div data-area=\"side\"></div>",
                "<img data-area=\"side\" />",
            ),
        ),
        (
            "anonymous text item",
            SOURCE.replace(
                "<main id=\"layout\">",
                "<main id=\"layout\">unexpected text",
            ),
        ),
        (
            "nonempty div item",
            SOURCE.replace(
                "<div data-area=\"side\"></div>",
                "<div data-area=\"side\"><span></span></div>",
            ),
        ),
        (
            "numeric item placement",
            SOURCE.replace(
                "grid-area: header;",
                "grid-area: header; grid-column-start: 1;",
            ),
        ),
        (
            "nondefault auto flow",
            SOURCE.replace(
                "grid-template-rows: 20px 50px;",
                "grid-template-rows: 20px 50px; grid-auto-flow: column;",
            ),
        ),
        (
            "implicit row sizing",
            SOURCE.replace(
                "grid-template-rows: 20px 50px;",
                "grid-template-rows: 20px 50px; grid-auto-rows: 10px;",
            ),
        ),
        (
            "implicit column sizing",
            SOURCE.replace(
                "grid-template-rows: 20px 50px;",
                "grid-template-rows: 20px 50px; grid-auto-columns: 10px;",
            ),
        ),
        (
            "nested grid item",
            SOURCE.replace(
                "[data-area=\"header\"] {",
                "[data-area=\"header\"] { display: grid;",
            ),
        ),
    ];

    for (name, source) in cases {
        let (_, result) = run(&source);
        assert_eq!(
            result.state,
            crate::session::RuntimeLayoutState::Failed,
            "out-of-scope named-area case={name} should fail closed: {result:?}"
        );
        assert!(
            result.error.is_some(),
            "out-of-scope named-area case={name} should preserve a failure diagnostic"
        );
    }
}

#[test]
fn c11_1_failed_new_environment_does_not_retain_a_stale_named_area_scene() {
    let responsive = SOURCE.replace(
        "  </style>",
        "  @media (max-width: 500px) { [data-area=\"header\"] { grid-area: sidebar; } }\n  </style>",
    );
    let (session, initial) = run(&responsive);
    assert_eq!(
        initial.state,
        crate::session::RuntimeLayoutState::Ready,
        "wide environment should produce the supported named-area scene: {initial:?}"
    );
    let initial_completed = initial.completed.expect("wide scene should complete");
    let initial_key = initial_completed.key;
    assert!(initial_completed.render_snapshot.is_some());

    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    let narrow_environment = session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("narrow viewport should update the environment revision");
    let narrow = session.wait_for_layout_snapshot(Duration::from_secs(5));
    assert_eq!(
        narrow.state,
        crate::session::RuntimeLayoutState::Failed,
        "the media-query placement is outside the named-area subset: {narrow:?}"
    );
    assert!(
        narrow.requested.is_some_and(|key| {
            key.environment_revision == narrow_environment.environment_revision.get()
                && key.environment_revision > initial_key.environment_revision
        }),
        "the failed result must identify the newer environment revision: {narrow:?}"
    );
    assert!(
        narrow.completed.is_none(),
        "a failed newer request must not expose the prior named-area scene: {narrow:?}"
    );
    assert!(narrow.error.is_some());
}

#[test]
fn c11_1_invalid_area_declaration_uses_css_cascade_fallback() {
    let invalid_without_items = SOURCE
        .replace("\"header header\" \"side content\"", "1px")
        .replace("grid-area: header;", "")
        .replace("grid-area: side;", "")
        .replace("grid-area: content;", "")
        .replace(
            "background-color: #e5484d;",
            "width: 120px; height: 20px; background-color: #e5484d;",
        )
        .replace(
            "background-color: #3a8f5b;",
            "width: 40px; height: 50px; background-color: #3a8f5b;",
        )
        .replace(
            "background-color: #0a84ff;",
            "width: 80px; height: 50px; background-color: #0a84ff;",
        );
    let (session, result) = run(&invalid_without_items);
    assert_eq!(
        result.state,
        crate::session::RuntimeLayoutState::Ready,
        "CSS-invalid grid-template-areas should drop to computed none when item placement also falls back: {result:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
}

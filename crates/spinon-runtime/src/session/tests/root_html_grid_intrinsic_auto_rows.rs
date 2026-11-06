use super::*;
use serde_json::Value;
use spinon_render::{OpaqueCssSrgb, RuntimePaint};

mod combined_multiple_text;
mod multiple_text;
mod normal_whitespace;

const SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-precomparison.html"
);
const EXPECTED: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-precomparison-2026-11-06/expected.json"
);
const TEXT_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-text-precomparison.html"
);
const TEXT_EXPECTED: &str = include_str!(
    "../../../../../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-text-precomparison-2026-11-06/expected.json"
);
const COMBINED_SOURCE: &str = include_str!(
    "../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-combined-tracks-runtime.html"
);
const TOLERANCE: f32 = 0.5;

fn run(source: &str) -> (RuntimeSession, crate::session::RuntimeLayoutSnapshot) {
    let (session, _) = RuntimeSession::new_runtime_gpu_with_root_html(source)
        .expect("parsed Root HTML should initialize the default runtime");
    let mut media = spinon_style::CssMediaEnvironment::MOBILE;
    media.color_scheme = spinon_style::CssColorScheme::Light;
    session
        .set_root_html_environment(390.0, 844.0, 1.0, media)
        .expect("root metadata should configure the CSS environment");
    let layout = session.wait_for_layout_snapshot(Duration::from_secs(5));
    (session, layout)
}

fn painted_frame(
    scene: &spinon_render::RuntimeRenderSnapshot,
    color: OpaqueCssSrgb,
    label: &str,
) -> [f32; 4] {
    let render_box = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(observed) if observed == color)
        })
        .unwrap_or_else(|| panic!("{label}: expected painted box is missing"));
    let frame = render_box.frame_css_px();
    [frame.x(), frame.y(), frame.width(), frame.height()]
}

#[test]
fn parsed_root_html_intrinsic_auto_rows_match_one_and_two_implicit_row_geometry() {
    let expected: Value = serde_json::from_str(EXPECTED)
        .expect("intrinsic auto-row expected values should be valid JSON");
    let expected_items = &expected["items"];
    let one_row_source = SOURCE.replace("        <div id=\"third\" class=\"item\"></div>\n", "");
    assert_ne!(
        one_row_source, SOURCE,
        "one-row mutation should match the fixture"
    );

    for (label, source, item_ids) in [
        (
            "one-implicit-row",
            one_row_source.as_str(),
            ["first", "second", ""],
        ),
        ("two-implicit-rows", SOURCE, ["first", "second", "third"]),
    ] {
        let (session, layout) = run(source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{label}: layout error={:?}; cascade state={:?}; cascade error={:?}",
            layout.error,
            session.ua_cascade_snapshot().state,
            session.ua_cascade_snapshot().error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported implicit rows should publish a complete layout");
        assert_eq!(
            completed.key,
            cascade
                .completed
                .as_ref()
                .expect("cascade should complete for the same source revision")
                .key,
            "{label}: style and layout revisions should match"
        );
        let render_scene = completed
            .render_snapshot
            .as_ref()
            .expect("default Root HTML route should publish paint boxes");
        let grid_frame = painted_frame(
            render_scene,
            OpaqueCssSrgb::new(57, 67, 79),
            &format!("{label} Grid"),
        );
        assert_frame_values(grid_frame, [0.0, 0.0, 40.0, 40.0], &format!("{label} Grid"));
        let mut render_positions = Vec::new();
        for id in item_ids.into_iter().filter(|id| !id.is_empty()) {
            let expected_item = &expected_items[id];
            let expected_frame = [
                expected_item["x"].as_f64().unwrap() as f32,
                expected_item["y"].as_f64().unwrap() as f32,
                expected_item["width"].as_f64().unwrap() as f32,
                expected_item["height"].as_f64().unwrap() as f32,
            ];
            let color = match id {
                "first" => OpaqueCssSrgb::new(242, 102, 94),
                "second" => OpaqueCssSrgb::new(91, 168, 245),
                "third" => OpaqueCssSrgb::new(85, 199, 145),
                _ => unreachable!("fixed Grid item id"),
            };
            let item_frame = painted_frame(render_scene, color, &format!("{label} {id}"));
            assert_frame_values(item_frame, expected_frame, &format!("{label} {id}"));
            let render_position = render_scene
                .boxes()
                .iter()
                .position(|render_box| {
                    matches!(render_box.paint(), RuntimePaint::Opaque(observed) if observed == color)
                });
            let render_position = render_position.unwrap_or_else(|| {
                panic!("{label}: {id} should remain visible in the paint scene")
            });
            render_positions.push(render_position);
        }
        assert!(
            render_positions.windows(2).all(|pair| pair[0] < pair[1]),
            "{label}: paint boxes should preserve DOM source order"
        );

        if label == "two-implicit-rows" {
            let third = painted_frame(
                render_scene,
                OpaqueCssSrgb::new(85, 199, 145),
                "two implicit rows third item",
            );
            assert_eq!(third[1], 32.0);
            assert_eq!(third[1] + third[3], 66.0);
            assert!(
                third[1] + third[3] > 40.0,
                "visible overflow should be preserved"
            );
        }
    }
}

#[test]
fn parsed_root_html_intrinsic_auto_rows_use_text_measure_for_min_content_contribution() {
    let expected: Value = serde_json::from_str(TEXT_EXPECTED)
        .expect("text intrinsic auto-row expected values should be valid JSON");
    let expected_items = &expected["independentExpected"]["items"];
    let one_row_source =
        TEXT_SOURCE.replace("        <div id=\"last\" class=\"item\"></div>\n", "");
    assert_ne!(
        one_row_source, TEXT_SOURCE,
        "one-row mutation should match the text fixture"
    );

    for (label, source, item_ids) in [
        (
            "one-implicit-row",
            one_row_source.as_str(),
            ["first", "wrapped-text", ""],
        ),
        (
            "two-implicit-rows",
            TEXT_SOURCE,
            ["first", "wrapped-text", "last"],
        ),
    ] {
        let (session, layout) = run(source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "{label}: layout error={:?}; cascade state={:?}; cascade error={:?}",
            layout.error,
            cascade.state,
            cascade.error
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("supported text contribution should publish a complete layout");
        assert_eq!(
            completed.key,
            cascade
                .completed
                .as_ref()
                .expect("cascade should complete for the same source revision")
                .key,
            "{label}: style and layout revisions should match"
        );
        let scene = completed
            .render_snapshot
            .as_ref()
            .expect("default Root HTML route should publish text and paint boxes");
        let grid_box = scene
            .boxes()
            .iter()
            .find(|render_box| {
                matches!(render_box.paint(), RuntimePaint::Opaque(color)
                    if color == OpaqueCssSrgb::new(57, 67, 79))
            })
            .expect("text Grid should remain visible");
        let grid_frame = grid_box.frame_css_px();
        assert_frame_values(
            [
                grid_frame.x(),
                grid_frame.y(),
                grid_frame.width(),
                grid_frame.height(),
            ],
            [0.0, 0.0, 40.0, 40.0],
            &format!("{label} Grid"),
        );

        let mut render_positions = Vec::new();
        let mut text_item_node = None;
        let mut last_item_node = None;
        for id in item_ids.into_iter().filter(|id| !id.is_empty()) {
            let expected_frame = std::array::from_fn(|component| {
                expected_items[id][component]
                    .as_f64()
                    .expect("expected frame component should be numeric") as f32
            });
            let color = match id {
                "first" => OpaqueCssSrgb::new(242, 102, 94),
                "wrapped-text" => OpaqueCssSrgb::new(91, 168, 245),
                "last" => OpaqueCssSrgb::new(85, 199, 145),
                _ => unreachable!("fixed text Grid item id"),
            };
            let render_box = scene
                .boxes()
                .iter()
                .find(|render_box| {
                    matches!(render_box.paint(), RuntimePaint::Opaque(observed) if observed == color)
                })
                .unwrap_or_else(|| panic!("{label}: {id} should remain visible in paint scene"));
            let frame = render_box.frame_css_px();
            assert_frame_values(
                [frame.x(), frame.y(), frame.width(), frame.height()],
                expected_frame,
                &format!("{label} {id}"),
            );
            if id == "wrapped-text" {
                text_item_node = Some(render_box.node_id());
            } else if id == "last" {
                last_item_node = Some(render_box.node_id());
            }
            render_positions.push(
                scene
                    .boxes()
                    .iter()
                    .position(|candidate| candidate.node_id() == render_box.node_id())
                    .expect("render box should retain its source order"),
            );
        }
        assert!(
            render_positions.windows(2).all(|pair| pair[0] < pair[1]),
            "{label}: paint boxes should preserve DOM source order"
        );
        let text_item_node = text_item_node.expect("text item should have a paint box");
        let text = scene
            .texts()
            .iter()
            .find(|text| text.node_id() == text_item_node)
            .expect("the wrapped Grid item should publish its text paint");
        assert_eq!(
            scene.texts().len(),
            1,
            "{label}: one text owner is supported"
        );
        assert_eq!(text.origin_x_css_px(), 0.0, "{label}: text x origin");
        assert_eq!(text.origin_y_css_px(), 10.0, "{label}: text y origin");
        assert_eq!(text.glyphs().len(), 6, "{label}: six visible A glyphs");
        let baselines = text
            .glyphs()
            .iter()
            .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            baselines.len(),
            2,
            "{label}: text paint should contain two lines"
        );
        assert_eq!(text.source().generation().get(), completed.key.generation);
        assert_eq!(
            text.source().document_revision().get(),
            completed.key.document_revision
        );
        assert_eq!(
            text.source().render_tree_revision().get(),
            completed.key.render_tree_revision
        );
        assert_eq!(text.source().style_revision(), completed.key.style_revision);

        let box_index = scene
            .boxes()
            .iter()
            .position(|render_box| render_box.node_id() == text_item_node)
            .expect("text owner box should remain in the paint scene");
        let text_index = scene
            .texts()
            .iter()
            .position(|candidate| candidate.node_id() == text_item_node)
            .expect("text paint index should be present");
        let text_command_index = scene
            .paint_commands()
            .iter()
            .position(|command| {
                matches!(command, spinon_render::RuntimePaintCommand::DrawText { index } if *index == text_index)
            })
            .expect("the text paint command should be present");
        let text_box_command_index = scene
            .paint_commands()
            .iter()
            .position(|command| {
                matches!(command, spinon_render::RuntimePaintCommand::DrawBox { index } if *index == box_index)
            })
            .expect("the text owner's background command should be present");
        assert!(text_box_command_index < text_command_index);
        if let Some(last_item_node) = last_item_node {
            let last_box_index = scene
                .boxes()
                .iter()
                .position(|render_box| render_box.node_id() == last_item_node)
                .expect("last Grid item box should retain paint order");
            let last_box_command_index = scene
                .paint_commands()
                .iter()
                .position(|command| {
                    matches!(command, spinon_render::RuntimePaintCommand::DrawBox { index } if *index == last_box_index)
                })
                .expect("the later Grid item's background command should be present");
            assert!(
                text_command_index < last_box_command_index,
                "{label}: text paint must precede the next item"
            );
        }
        assert!(
            completed
                .frames
                .iter()
                .any(|frame| frame.node_id == text_item_node.get()),
            "{label}: layout result should retain the text owner's frame identity"
        );
    }
}

#[test]
fn parsed_root_html_intrinsic_columns_and_text_rows_run_in_default_session() {
    let (session, layout) = run(COMBINED_SOURCE);
    let cascade = session.ua_cascade_snapshot();
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "layout error={:?}; cascade state={:?}; cascade error={:?}",
        layout.error,
        cascade.state,
        cascade.error
    );
    let completed = layout
        .completed
        .as_ref()
        .expect("combined intrinsic tracks should publish a complete layout");
    assert_eq!(
        completed.key,
        cascade
            .completed
            .as_ref()
            .expect("cascade should complete for the same source revision")
            .key
    );
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("default Root HTML session should publish paint");
    let grid = painted_frame(scene, OpaqueCssSrgb::new(57, 67, 79), "combined Grid");
    assert_frame_values(grid, [0.0, 0.0, 40.0, 40.0], "combined Grid");
    let first = painted_frame(scene, OpaqueCssSrgb::new(242, 102, 94), "first item");
    assert_frame_values(first, [0.0, 0.0, 1.0, 10.0], "first item");
    let text_frame = painted_frame(scene, OpaqueCssSrgb::new(91, 168, 245), "text item");
    assert!(
        (6.0..=7.0).contains(&text_frame[2]),
        "runtime font min-content width should be retained: {text_frame:?}"
    );
    assert_frame_values(
        [text_frame[0], text_frame[1], text_frame[3], 0.0],
        [0.0, 10.0, 60.0, 0.0],
        "text item frame",
    );
    let last = painted_frame(scene, OpaqueCssSrgb::new(85, 199, 145), "last item");
    assert_frame_values(last, [0.0, 70.0, 1.0, 34.0], "last item");
    assert!(
        last[1] + last[3] > grid[3],
        "visible overflow should be preserved"
    );

    assert_eq!(scene.texts().len(), 1, "one direct text owner is supported");
    let text = &scene.texts()[0];
    assert_eq!(text.origin_x_css_px(), 0.0);
    assert_eq!(text.origin_y_css_px(), 10.0);
    assert_eq!(text.glyphs().len(), 6);
    let baselines = text
        .glyphs()
        .iter()
        .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        baselines.len(),
        6,
        "intrinsic column width should wrap each word"
    );
    assert_eq!(text.source().generation().get(), completed.key.generation);
    assert_eq!(
        text.source().document_revision().get(),
        completed.key.document_revision
    );
    assert_eq!(
        text.source().render_tree_revision().get(),
        completed.key.render_tree_revision
    );
    assert_eq!(text.source().style_revision(), completed.key.style_revision);
}

#[test]
fn parsed_root_html_intrinsic_auto_rows_ignore_hidden_text_items() {
    let source = TEXT_SOURCE.replace(
        "#wrapped-text {\n        width: auto;",
        "#wrapped-text {\n        display: none;\n        width: auto;",
    );
    assert_ne!(
        source, TEXT_SOURCE,
        "hidden-text mutation should match the fixture"
    );
    let (session, layout) = run(&source);
    assert_eq!(
        layout.state,
        crate::session::RuntimeLayoutState::Ready,
        "hidden text should not break the default Root HTML route: {layout:?}; cascade={:?}",
        session.ua_cascade_snapshot()
    );
    let completed = layout
        .completed
        .expect("hidden text should still produce a complete layout");
    let scene = completed
        .render_snapshot
        .as_ref()
        .expect("the visible Grid items should retain their paint scene");
    assert!(scene.texts().is_empty(), "hidden text must not be painted");
    let last = scene
        .boxes()
        .iter()
        .find(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(color)
                if color == OpaqueCssSrgb::new(85, 199, 145))
        })
        .expect("the visible last item should remain painted");
    let frame = last.frame_css_px();
    assert_frame_values(
        [frame.x(), frame.y(), frame.width(), frame.height()],
        [0.0, 10.0, 40.0, 34.0],
        "last item after hidden text is removed from auto-placement",
    );
    assert!(
        !scene.boxes().iter().any(|render_box| {
            matches!(render_box.paint(), RuntimePaint::Opaque(color)
                if color == OpaqueCssSrgb::new(91, 168, 245))
        }),
        "hidden text item's box must not be published"
    );
}

fn assert_frame_values(actual: [f32; 4], expected: [f32; 4], label: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= TOLERANCE,
            "{label} component {component}: actual={actual}, expected={expected} ±{TOLERANCE} CSS px"
        );
    }
}

#[test]
fn parsed_root_html_intrinsic_auto_rows_reject_unsupported_values_atomically() {
    for (label, source, expected_code) in [
        (
            "max-content",
            SOURCE.replace(
                "grid-auto-rows: min-content;",
                "grid-auto-rows: max-content;",
            ),
            "unsupported_computed_value",
        ),
        (
            "multi-track-list",
            SOURCE.replace(
                "grid-auto-rows: min-content;",
                "grid-auto-rows: min-content 12px;",
            ),
            "unsupported_computed_value",
        ),
        (
            "nonzero-row-gap",
            SOURCE.replace("gap: 0;", "row-gap: 1px; column-gap: 0;"),
            "unsupported_grid_auto_rows",
        ),
        (
            "nonzero-column-gap",
            SOURCE.replace("gap: 0;", "row-gap: 0; column-gap: 1px;"),
            "unsupported_grid_auto_rows",
        ),
        (
            "auto-sized-item",
            SOURCE.replace(
                ".item { box-sizing: border-box; width: 40px; margin: 0; padding: 0; }",
                ".item { box-sizing: border-box; width: auto; margin: 0; padding: 0; }",
            ),
            "unsupported_computed_value",
        ),
        (
            "item-min-size",
            SOURCE.replace(
                ".item { box-sizing: border-box; width: 40px; margin: 0; padding: 0; }",
                ".item { box-sizing: border-box; width: 40px; min-height: 0; margin: 0; padding: 0; }",
            ),
            "unsupported_computed_value",
        ),
        (
            "explicit-item-placement",
            SOURCE.replace(
                ".item { box-sizing: border-box; width: 40px; margin: 0; padding: 0; }",
                ".item { box-sizing: border-box; width: 40px; grid-row-start: 1; margin: 0; padding: 0; }",
            ),
            "unsupported_computed_value",
        ),
        (
            "implicit-column-with-intrinsic-track",
            SOURCE.replace(
                "grid-template-columns: 40px;",
                "grid-template-columns: min-content; grid-auto-columns: 10px;",
            ),
            "unsupported_computed_value",
        ),
        (
            "non-div-item",
            SOURCE.replace(
                "<div id=\"third\" class=\"item\"></div>",
                "<span id=\"third\" class=\"item\"></span>",
            ),
            "unsupported_computed_value",
        ),
        (
            "anonymous-text-item",
            SOURCE.replace("<div id=\"third\" class=\"item\"></div>", "text"),
            "unsupported_root_html_grid_text_item",
        ),
        (
            "text-descendant",
            SOURCE.replace(
                "<div id=\"third\" class=\"item\"></div>",
                "<div id=\"third\" class=\"item\">text</div>",
            ),
            "unsupported_computed_value",
        ),
        (
            "nested-grid-item",
            SOURCE.replace(
                ".item { box-sizing: border-box; width: 40px; margin: 0; padding: 0; }",
                ".item { display: grid; grid-template-columns: 40px; grid-template-rows: 10px; box-sizing: border-box; width: 40px; margin: 0; padding: 0; }",
            ),
            "unsupported_computed_value",
        ),
        (
            "multiple-visible-grid-owners",
            SOURCE.replace(
                "    </main>",
                "      <div class=\"grid\"></div>\n    </main>",
            ),
            "unsupported_computed_value",
        ),
    ] {
        assert_ne!(source, SOURCE, "{label}: source mutation should match");
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            cascade.state,
            crate::session::RuntimeUaCascadeState::Ready,
            "{label}: CSS-valid input should finish cascade before the bounded adapter rejects it; error={:?}",
            cascade.error
        );
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{label}"
        );
        let failure = layout
            .error
            .as_ref()
            .unwrap_or_else(|| panic!("{label}: expected a contextual rejection"));
        assert_eq!(failure.code, expected_code, "{label}");
        if label != "anonymous-text-item" {
            assert_eq!(
                failure.property.as_deref(),
                Some("grid-auto-rows"),
                "{label}: rejection should identify the unsupported Grid property"
            );
        }
        assert!(
            layout.completed.is_none(),
            "{label}: rejection must not publish partial frames or paint"
        );
    }
}

#[test]
fn parsed_root_html_combined_intrinsic_tracks_reject_out_of_slice_combinations_atomically() {
    for (label, source, expected_property) in [
        (
            "fit-content-track",
            COMBINED_SOURCE.replace(
                "grid-template-columns: min-content;",
                "grid-template-columns: min-content fit-content(20px);",
            ),
            "grid-auto-rows",
        ),
        (
            "named-area",
            COMBINED_SOURCE.replace(
                "grid-template-columns: min-content;",
                "grid-template-columns: min-content; grid-template-areas: \"a\";",
            ),
            "grid-template-areas",
        ),
        (
            "dense-flow",
            COMBINED_SOURCE.replace("grid-auto-flow: row;", "grid-auto-flow: row dense;"),
            "grid-auto-rows",
        ),
        (
            "multiple-explicit-rows",
            COMBINED_SOURCE.replace(
                "grid-template-rows: 10px;",
                "grid-template-rows: 10px 10px;",
            ),
            "grid-auto-rows",
        ),
    ] {
        assert_ne!(
            source, COMBINED_SOURCE,
            "{label}: source mutation should match"
        );
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            cascade.state,
            crate::session::RuntimeUaCascadeState::Ready,
            "{label}: CSS-valid input should finish cascade before the bounded adapter rejects it; error={:?}",
            cascade.error
        );
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{label}"
        );
        let failure = layout
            .error
            .as_ref()
            .unwrap_or_else(|| panic!("{label}: expected a contextual rejection"));
        assert_eq!(failure.code, "unsupported_computed_value", "{label}");
        assert_eq!(
            failure.property.as_deref(),
            Some(expected_property),
            "{label}: reject the unsupported combined track composition"
        );
        assert!(
            layout.completed.is_none(),
            "{label}: rejection must not publish partial frames or paint"
        );
    }
}

#[test]
fn parsed_root_html_intrinsic_auto_rows_reject_unsupported_text_items_atomically() {
    for (label, source) in [
        (
            "different-text-source",
            TEXT_SOURCE.replace("A A A A A A", "B B B B B B"),
        ),
        (
            "nested-inline-markup",
            TEXT_SOURCE.replace("A A A A A A", "<span>A A A A A A</span>"),
        ),
        (
            "multiple-direct-children",
            TEXT_SOURCE.replace(
                "<div id=\"wrapped-text\" class=\"item\">A A A A A A</div>",
                "<div id=\"wrapped-text\" class=\"item\">A</div><span></span>",
            ),
        ),
        (
            "wrong-font-family",
            TEXT_SOURCE.replace("font-family: monospace;", "font-family: sans-serif;"),
        ),
        (
            "wrong-line-height",
            TEXT_SOURCE.replace("line-height: 10px;", "line-height: 12px;"),
        ),
        (
            "nowrap-text",
            TEXT_SOURCE.replace("white-space: normal;", "white-space: nowrap;"),
        ),
        (
            "fixed-text-height",
            TEXT_SOURCE.replace("height: auto;", "height: 20px;"),
        ),
        (
            "fixed-text-width",
            TEXT_SOURCE.replace(
                "#wrapped-text {\n        width: auto;",
                "#wrapped-text {\n        width: 40px;",
            ),
        ),
    ] {
        assert_ne!(source, TEXT_SOURCE, "{label}: source mutation should match");
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            cascade.state,
            crate::session::RuntimeUaCascadeState::Ready,
            "{label}: CSS-valid text style should finish cascade before bounded rejection; error={:?}",
            cascade.error
        );
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{label}: unsupported text input must fail layout"
        );
        let failure = layout
            .error
            .as_ref()
            .unwrap_or_else(|| panic!("{label}: expected a contextual rejection"));
        assert_eq!(failure.code, "unsupported_computed_value", "{label}");
        assert_eq!(
            failure.property.as_deref(),
            Some("grid-auto-rows"),
            "{label}: rejection should identify the bounded Grid property"
        );
        assert!(
            layout.completed.is_none(),
            "{label}: rejection must not publish partial frames or paint"
        );
    }
}

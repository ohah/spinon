use super::*;
use serde_json::Value;

const COMBINED_MULTIPLE_TEXT_SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-combined-multiple-text-precomparison.html"
);
const COMBINED_MULTIPLE_TEXT_EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-intrinsic-combined-multiple-text-precomparison-2026-11-06/expected.json"
);

#[test]
fn parsed_root_html_combines_intrinsic_columns_with_multiple_text_rows() {
    let expected: Value = serde_json::from_str(COMBINED_MULTIPLE_TEXT_EXPECTED)
        .expect("combined multiple-text expected JSON should be valid");
    for (columns, scene_id) in [
        ("min-content", "min-content"),
        ("max-content", "max-content"),
        ("auto", "auto"),
        ("minmax(min-content, 1fr)", "minmax-min-fr"),
    ] {
        for reverse in [false, true] {
            let mut source = COMBINED_MULTIPLE_TEXT_SOURCE.replace(
                "grid-template-columns: min-content;",
                &format!("grid-template-columns: {columns};"),
            );
            if columns != "min-content" {
                assert_ne!(source, COMBINED_MULTIPLE_TEXT_SOURCE, "{columns}");
            }
            if reverse {
                let forward = "        <div id=\"long-text\" class=\"item wrapped-text\">A A A A A A</div>\n        <div id=\"short-text\" class=\"item wrapped-text\">A</div>";
                let reversed = "        <div id=\"short-text\" class=\"item wrapped-text\">A</div>\n        <div id=\"long-text\" class=\"item wrapped-text\">A A A A A A</div>";
                let reordered = source.replace(forward, reversed);
                assert_ne!(reordered, source, "source-order mutation should match");
                source = reordered;
            }

            let (session, layout) = run(&source);
            let cascade = session.ua_cascade_snapshot();
            assert_eq!(
                layout.state,
                crate::session::RuntimeLayoutState::Ready,
                "columns={columns}, reverse={reverse}: layout={:?}; cascade={:?}",
                layout.error,
                cascade.error
            );
            let completed = layout.completed.as_ref().expect("layout should complete");
            assert_eq!(
                completed.key,
                cascade
                    .completed
                    .as_ref()
                    .expect("cascade should complete")
                    .key,
                "style and layout should use the same revision"
            );
            let scene = completed
                .render_snapshot
                .as_ref()
                .expect("default Root HTML route should publish its render snapshot");
            assert_eq!(
                painted_frame(scene, OpaqueCssSrgb::new(57, 67, 79), "Grid"),
                [0.0, 0.0, 40.0, 40.0]
            );
            let expected_scene = &expected["scenes"][scene_id];
            let col_range = match columns {
                "min-content" => (5.0, 9.0),
                "max-content" => (58.0, 72.0),
                _ => (39.5, 40.5),
            };
            let id_color = |id: &str| match id {
                "first" => OpaqueCssSrgb::new(242, 102, 94),
                "long-text" => OpaqueCssSrgb::new(91, 168, 245),
                "short-text" => OpaqueCssSrgb::new(255, 209, 102),
                "last" => OpaqueCssSrgb::new(85, 199, 145),
                _ => unreachable!("fixed combined multiple-text item id"),
            };
            let item_order = if reverse {
                ["first", "short-text", "long-text", "last"]
            } else {
                ["first", "long-text", "short-text", "last"]
            };
            let mut item_nodes = Vec::new();
            for id in item_order {
                let frame = painted_frame(scene, id_color(id), id);
                if id == "first" {
                    assert_eq!(frame, [0.0, 0.0, 1.0, 10.0]);
                } else if id == "last" {
                    let long_lines = expected_scene["textLineCounts"]["long-text"]
                        .as_u64()
                        .unwrap() as f32;
                    assert_eq!(frame[0], 0.0);
                    assert_eq!(frame[1], 20.0 + long_lines * 10.0);
                    assert_eq!(frame[2], 1.0);
                    assert_eq!(frame[3], 34.0);
                } else {
                    assert!(
                        (col_range.0..=col_range.1).contains(&frame[2]),
                        "columns={columns}, {id}: unexpected width {}",
                        frame[2]
                    );
                }
                let node = scene
                    .boxes()
                    .iter()
                    .find(|render_box| {
                        matches!(render_box.paint(), RuntimePaint::Opaque(color) if color == id_color(id))
                    })
                    .expect("painted item box should be present")
                    .node_id();
                item_nodes.push(node);
            }
            let positions = item_nodes
                .iter()
                .map(|node| {
                    scene
                        .boxes()
                        .iter()
                        .position(|render_box| render_box.node_id() == *node)
                        .expect("Grid item should retain its box")
                })
                .collect::<Vec<_>>();
            assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));

            for (item_index, id) in item_order.iter().enumerate() {
                if *id != "long-text" && *id != "short-text" {
                    continue;
                }
                let node = item_nodes[item_index];
                let text_index = scene
                    .texts()
                    .iter()
                    .position(|text| text.node_id() == node)
                    .expect("text paint should preserve its item node");
                let box_command = scene
                    .paint_commands()
                    .iter()
                    .position(|command| {
                        matches!(command, spinon_render::RuntimePaintCommand::DrawBox { index } if *index == positions[item_index])
                    })
                    .expect("text item background draw command should be present");
                let text_command = scene
                    .paint_commands()
                    .iter()
                    .position(|command| {
                        matches!(command, spinon_render::RuntimePaintCommand::DrawText { index } if *index == text_index)
                    })
                    .expect("text draw command should be present");
                assert!(box_command < text_command);
                if item_index + 1 < item_order.len() {
                    let next_box_command = scene
                        .paint_commands()
                        .iter()
                        .position(|command| {
                            matches!(command, spinon_render::RuntimePaintCommand::DrawBox { index } if *index == positions[item_index + 1])
                        })
                        .expect("next Grid item background draw command should be present");
                    assert!(text_command < next_box_command);
                }
            }

            assert_eq!(scene.texts().len(), 2);
            for id in ["long-text", "short-text"] {
                let item_index = item_order
                    .iter()
                    .position(|item_id| *item_id == id)
                    .expect("text item should occur in DOM order");
                let node = item_nodes[item_index];
                let text = scene
                    .texts()
                    .iter()
                    .find(|text| text.node_id() == node)
                    .expect("each text paint should retain its Grid item identity");
                let expected_lines = expected_scene["textLineCounts"][id]
                    .as_u64()
                    .expect("expected line count should be an integer")
                    as usize;
                let expected_y = match (id, reverse) {
                    ("long-text", false) => 10.0,
                    ("long-text", true) => 20.0,
                    ("short-text", false) => {
                        let long_lines = expected_scene["textLineCounts"]["long-text"]
                            .as_u64()
                            .unwrap() as f32;
                        10.0 + long_lines * 10.0
                    }
                    ("short-text", true) => 10.0,
                    _ => unreachable!("fixed combined text item id"),
                };
                let baselines = text
                    .glyphs()
                    .iter()
                    .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                    .collect::<std::collections::BTreeSet<_>>();
                assert_eq!(baselines.len(), expected_lines, "columns={columns}, {id}");
                assert_eq!(text.origin_y_css_px(), expected_y);
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
            let last = painted_frame(scene, id_color("last"), "last item");
            let grid = painted_frame(scene, OpaqueCssSrgb::new(57, 67, 79), "Grid");
            assert!(last[1] + last[3] > grid[3]);
        }
    }
}

#[test]
fn parsed_root_html_combined_intrinsic_rows_reject_invalid_second_text_atomically() {
    let invalid_cases = [
        (
            "invalid-second-source",
            COMBINED_MULTIPLE_TEXT_SOURCE.replace(
                "<div id=\"short-text\" class=\"item wrapped-text\">A</div>",
                "<div id=\"short-text\" class=\"item wrapped-text\">B1</div>",
            ),
            crate::session::RuntimeUaCascadeState::Ready,
        ),
        (
            "invalid-second-style",
            COMBINED_MULTIPLE_TEXT_SOURCE.replace(
                "#short-text { background-color: #ffd166; }",
                "#short-text { white-space: nowrap; background-color: #ffd166; }",
            ),
            crate::session::RuntimeUaCascadeState::Ready,
        ),
        (
            "intrinsic-explicit-row",
            COMBINED_MULTIPLE_TEXT_SOURCE.replace(
                "grid-template-rows: 10px;",
                "grid-template-rows: min-content;",
            ),
            crate::session::RuntimeUaCascadeState::Ready,
        ),
        (
            "fit-content-column",
            COMBINED_MULTIPLE_TEXT_SOURCE.replace(
                "grid-template-columns: min-content;",
                "grid-template-columns: fit-content(20px);",
            ),
            crate::session::RuntimeUaCascadeState::Ready,
        ),
        (
            "column-auto-repeat",
            COMBINED_MULTIPLE_TEXT_SOURCE.replace(
                "grid-template-columns: min-content;",
                "grid-template-columns: min-content repeat(auto-fill, minmax(10px, 1fr));",
            ),
            crate::session::RuntimeUaCascadeState::Failed,
        ),
        (
            "row-auto-repeat",
            COMBINED_MULTIPLE_TEXT_SOURCE.replace(
                "grid-template-rows: 10px;",
                "grid-template-rows: repeat(auto-fill, 10px);",
            ),
            crate::session::RuntimeUaCascadeState::Failed,
        ),
        (
            "named-area",
            COMBINED_MULTIPLE_TEXT_SOURCE.replace(
                "grid-template-rows: 10px;",
                "grid-template-rows: 10px; grid-template-areas: \"content\";",
            ),
            crate::session::RuntimeUaCascadeState::Ready,
        ),
        (
            "implicit-columns",
            COMBINED_MULTIPLE_TEXT_SOURCE.replace(
                "grid-auto-flow: row;",
                "grid-auto-flow: row; grid-auto-columns: 8px;",
            ),
            crate::session::RuntimeUaCascadeState::Ready,
        ),
    ];
    for (label, source, expected_cascade_state) in invalid_cases {
        assert_ne!(source, COMBINED_MULTIPLE_TEXT_SOURCE, "{label}");
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            cascade.state, expected_cascade_state,
            "{label}: unexpected cascade result; error={:?}",
            cascade.error
        );
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{label}"
        );
        assert!(
            layout.completed.is_none(),
            "{label}: failed layout must not publish partial frames or paint"
        );
    }
}

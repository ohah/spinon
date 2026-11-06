use super::*;
use serde_json::Value;
use spinon_render::{OpaqueCssSrgb, RuntimePaint};

const MULTIPLE_TEXT_SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-multiple-text-precomparison.html"
);
const MULTIPLE_TEXT_EXPECTED: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-multiple-text-precomparison-2026-11-06/expected.json"
);
fn assert_multiple_text_auto_row_scene(
    source: &str,
    expected_items: &Value,
    text_line_counts: &Value,
    item_order: &[&str],
    label: &str,
) {
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
        .expect("multiple text rows should publish a complete layout");
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
        .expect("default Root HTML should publish the multi-text paint scene");
    assert_frame_values(
        painted_frame(scene, OpaqueCssSrgb::new(57, 67, 79), "multi-text Grid"),
        [0.0, 0.0, 40.0, 40.0],
        &format!("{label} Grid"),
    );

    let color_for = |id| match id {
        "first" => OpaqueCssSrgb::new(242, 102, 94),
        "long-text" => OpaqueCssSrgb::new(91, 168, 245),
        "short-text" => OpaqueCssSrgb::new(255, 209, 102),
        "last" => OpaqueCssSrgb::new(85, 199, 145),
        _ => unreachable!("fixed multi-text Grid item id"),
    };
    let mut ordered_nodes = Vec::new();
    for id in item_order {
        let values = expected_items[*id]
            .as_array()
            .unwrap_or_else(|| panic!("{label}: expected frame for {id}"));
        let expected_frame = std::array::from_fn(|component| {
            values[component]
                .as_f64()
                .expect("expected frame component should be numeric") as f32
        });
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| {
                matches!(render_box.paint(), RuntimePaint::Opaque(observed) if observed == color_for(id))
            })
            .unwrap_or_else(|| panic!("{label}: expected {id} box should be painted"));
        let frame = render_box.frame_css_px();
        assert_frame_values(
            [frame.x(), frame.y(), frame.width(), frame.height()],
            expected_frame,
            &format!("{label} {id}"),
        );
        assert!(
            completed
                .frames
                .iter()
                .any(|frame| frame.node_id == render_box.node_id().get()),
            "{label}: {id} frame identity should be retained"
        );
        ordered_nodes.push(render_box.node_id());
    }
    let box_positions = ordered_nodes
        .iter()
        .map(|node| {
            scene
                .boxes()
                .iter()
                .position(|render_box| render_box.node_id() == *node)
                .expect("visible Grid item should have a paint box")
        })
        .collect::<Vec<_>>();
    assert!(
        box_positions.windows(2).all(|pair| pair[0] < pair[1]),
        "{label}: Grid item boxes should preserve DOM source order"
    );
    assert_eq!(
        scene.texts().len(),
        2,
        "{label}: both text paints are required"
    );
    for (index, id) in item_order.iter().enumerate() {
        if !id.ends_with("-text") {
            continue;
        }
        let node = ordered_nodes[index];
        let text_index = scene
            .texts()
            .iter()
            .position(|text| text.node_id() == node)
            .unwrap_or_else(|| panic!("{label}: {id} text paint should retain item identity"));
        let text = &scene.texts()[text_index];
        let expected_lines = text_line_counts[*id]
            .as_u64()
            .expect("expected line count should be an integer")
            as usize;
        let expected_glyphs = if *id == "long-text" { 6 } else { 1 };
        assert_eq!(text.glyphs().len(), expected_glyphs, "{label}: {id} glyphs");
        let baselines = text
            .glyphs()
            .iter()
            .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(baselines.len(), expected_lines, "{label}: {id} lines");
        assert_eq!(
            text.origin_y_css_px(),
            expected_items[*id][1].as_f64().unwrap() as f32
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

        let box_command_index = scene
            .paint_commands()
            .iter()
            .position(|command| {
                matches!(command, spinon_render::RuntimePaintCommand::DrawBox { index: box_index } if *box_index == box_positions[index])
            })
            .expect("text item box command should be present");
        let text_command_index = scene
            .paint_commands()
            .iter()
            .position(|command| {
                matches!(command, spinon_render::RuntimePaintCommand::DrawText { index: paint_index } if *paint_index == text_index)
            })
            .expect("text item draw command should be present");
        assert!(box_command_index < text_command_index);
        if index + 1 < item_order.len() {
            let next_box_position = box_positions[index + 1];
            let next_box_command_index = scene
                .paint_commands()
                .iter()
                .position(|command| {
                    matches!(command, spinon_render::RuntimePaintCommand::DrawBox { index: box_index } if *box_index == next_box_position)
                })
                .expect("following Grid item box command should be present");
            assert!(text_command_index < next_box_command_index);
        }
    }
}

#[test]
fn parsed_root_html_intrinsic_auto_rows_measure_multiple_direct_text_items() {
    let expected: Value = serde_json::from_str(MULTIPLE_TEXT_EXPECTED)
        .expect("multiple text expected values should be valid JSON");
    let independent = &expected["independentExpected"];
    let item_order = ["first", "long-text", "short-text", "last"];
    assert_multiple_text_auto_row_scene(
        MULTIPLE_TEXT_SOURCE,
        &independent["items"],
        &independent["textLineCounts"],
        &item_order,
        "forward-source-order",
    );

    let reversed = MULTIPLE_TEXT_SOURCE.replace(
        "        <div id=\"long-text\" class=\"item wrapped-text\">A A A A A A</div>\n        <div id=\"short-text\" class=\"item wrapped-text\">A</div>",
        "        <div id=\"short-text\" class=\"item wrapped-text\">A</div>\n        <div id=\"long-text\" class=\"item wrapped-text\">A A A A A A</div>",
    );
    assert_ne!(
        reversed, MULTIPLE_TEXT_SOURCE,
        "source-order mutation should match"
    );
    assert_multiple_text_auto_row_scene(
        &reversed,
        &expected["reversedTextSourceOrder"]["items"],
        &independent["textLineCounts"],
        &["first", "short-text", "long-text", "last"],
        "reversed-source-order",
    );
}

#[test]
fn parsed_root_html_intrinsic_auto_rows_reject_an_invalid_second_text_item_atomically() {
    let invalid_sources = [
        (
            "invalid-second-source",
            MULTIPLE_TEXT_SOURCE.replace(
                "<div id=\"short-text\" class=\"item wrapped-text\">A</div>",
                "<div id=\"short-text\" class=\"item wrapped-text\">B1</div>",
            ),
        ),
        (
            "invalid-second-style",
            MULTIPLE_TEXT_SOURCE.replace(
                "#short-text { background-color: #ffd166; }",
                "#short-text { white-space: nowrap; background-color: #ffd166; }",
            ),
        ),
        (
            "nested-second-text",
            MULTIPLE_TEXT_SOURCE.replace(
                "<div id=\"short-text\" class=\"item wrapped-text\">A</div>",
                "<div id=\"short-text\" class=\"item wrapped-text\"><span>A</span></div>",
            ),
        ),
        (
            "multiple-second-children",
            MULTIPLE_TEXT_SOURCE.replace(
                "<div id=\"short-text\" class=\"item wrapped-text\">A</div>",
                "<div id=\"short-text\" class=\"item wrapped-text\">A<span></span></div>",
            ),
        ),
    ];
    for (label, source) in invalid_sources {
        assert_ne!(
            source, MULTIPLE_TEXT_SOURCE,
            "{label}: mutation should match"
        );
        let (session, layout) = run(&source);
        let cascade = session.ua_cascade_snapshot();
        assert_eq!(
            cascade.state,
            crate::session::RuntimeUaCascadeState::Ready,
            "{label}: CSS-valid document should complete cascade before route rejection"
        );
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Failed,
            "{label}"
        );
        let failure = layout
            .error
            .as_ref()
            .unwrap_or_else(|| panic!("{label}: expected contextual rejection"));
        assert_eq!(failure.code, "unsupported_computed_value", "{label}");
        assert_eq!(
            failure.property.as_deref(),
            Some("grid-auto-rows"),
            "{label}: unsupported second text should identify Grid sizing"
        );
        assert!(
            layout.completed.is_none(),
            "{label}: rejected scene must not publish partial frames or paint"
        );
    }
}

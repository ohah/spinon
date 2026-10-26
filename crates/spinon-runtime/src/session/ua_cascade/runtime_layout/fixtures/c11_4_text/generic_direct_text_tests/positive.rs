use spinon_core::StyleRevision;
use spinon_layout::{
    FontStyleMode, LayoutDimension, LayoutGridItemIntrinsicSource, LayoutIntrinsicMeasureSpace,
    LayoutIntrinsicMeasurementSource, LayoutLengthPercentage, LigatureMode, LineBreakMode,
    TextParagraphDirection,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoFlow, ComputedCssGridAutoRows,
    ComputedCssGridTrackSize, ComputedCssSpacingValue, ComputedStyleProfile,
};

use super::super::generic_direct_text;
use super::cases::*;
use super::test_fixture::*;

#[test]
fn generic_collector_matches_six_scene_computed_style_intrinsic_and_c15_geometry() {
    for scene in SCENES {
        let fixture = positive_fixture(scene);
        assert_eq!(
            fixture.view.mount_root_element().handle(),
            Some(fixture.root)
        );
        let computed = cascade(&fixture.view, POSITIVE_STYLESHEET);
        assert_eq!(
            computed.profile,
            ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
            "{}",
            scene.label
        );
        assert_eq!(computed.style_revision, StyleRevision::INITIAL);

        let expected_root_style = computed
            .elements
            .iter()
            .find(|style| style.node_id == fixture.root.id())
            .unwrap();
        assert_eq!(expected_root_style.properties["display"], "grid");
        assert_eq!(expected_root_style.properties["align-items"], "start");
        assert_eq!(
            expected_root_style.layout_dimensions.width,
            ComputedCssDimension::LengthPx(100.0)
        );
        assert_eq!(
            expected_root_style.layout_dimensions.height,
            ComputedCssDimension::Auto
        );
        assert_eq!(
            expected_root_style.layout_spacing.column_gap,
            ComputedCssSpacingValue::LengthPx(10.0)
        );
        let grid = expected_root_style.grid_layout.as_ref().unwrap();
        assert_eq!(
            grid.columns,
            [
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::MaxContent
            ]
        );
        assert_eq!(grid.rows, [ComputedCssGridTrackSize::Auto]);
        assert_eq!(grid.auto_flow, ComputedCssGridAutoFlow::Row);
        assert_eq!(grid.auto_rows, ComputedCssGridAutoRows::Auto);

        let input = generic_direct_text::prepare(&fixture.snapshot, fixture.root, &computed)
            .unwrap_or_else(|error| panic!("{}: {error:?}", scene.label))
            .unwrap_or_else(|| panic!("{}: generic candidate fell back", scene.label));
        assert!(!input.two_axis_nested_grid);
        assert_eq!(input.paragraph_styles.len(), 2, "{}", scene.label);
        for (index, expected) in scene.items.iter().enumerate() {
            let item = fixture.items[index];
            let style = &input.paragraph_styles[&item.id()];
            assert_eq!(style.font_family, "Ahem", "{}", scene.label);
            assert_eq!(style.font_size, 10.0, "{}", scene.label);
            assert_eq!(style.line_height, 10.0, "{}", scene.label);
            assert!(!style.line_height_is_normal, "{}", scene.label);
            assert_eq!(style.font_weight, 400.0, "{}", scene.label);
            assert_eq!(style.font_style, FontStyleMode::Normal, "{}", scene.label);
            assert_eq!(style.white_space, expected_white_space(scene));
            assert_eq!(style.word_break, expected_word_break(expected.word_break));
            assert_eq!(style.direction, TextParagraphDirection::Ltr);
            assert_eq!(
                style.writing_mode,
                spinon_core::WritingMode::HorizontalTb,
                "{}",
                scene.label
            );
            assert_eq!(
                style.text_orientation,
                spinon_core::TextOrientation::Mixed,
                "{}",
                scene.label
            );
            assert_eq!(style.line_break, LineBreakMode::Strict, "{}", scene.label);
            assert_eq!(style.ligatures, LigatureMode::Normal, "{}", scene.label);
        }

        let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
            &fixture.snapshot,
            fixture.root,
            computed.clone(),
            VIEWPORT,
            &input.paragraph_styles,
            &input.engine,
        )
        .unwrap_or_else(|error| panic!("{}: {error:?}", scene.label));
        assert_eq!(
            output.computed_styles.style_revision,
            computed.style_revision
        );
        assert_eq!(output.layout.frames[&fixture.root.id()].x, 0.0);
        assert_eq!(output.layout.frames[&fixture.root.id()].y, 0.0);
        close(
            output.layout.frames[&fixture.root.id()].width,
            100.0,
            scene.label,
        );
        close(
            output.layout.frames[&fixture.root.id()].height,
            scene.root_height,
            scene.label,
        );
        assert_eq!(output.layout.grid_item_intrinsic_inputs.len(), 2);

        for (index, expected) in scene.items.iter().enumerate() {
            let item = fixture.items[index];
            let text_node = direct_text(&fixture.snapshot, item);
            let item_input = output
                .layout
                .grid_item_intrinsic_inputs
                .iter()
                .find(|candidate| candidate.node == item.id())
                .unwrap_or_else(|| panic!("{}: missing intrinsic input", scene.label));
            assert_eq!(item_input.grid_container, fixture.root.id());
            assert_eq!(item_input.source_order, index, "{}", scene.label);
            assert_eq!(item_input.revision, output.layout.revision);
            assert!(item_input.text_sources.is_empty());
            let raw_sizes = match &item_input.source {
                LayoutGridItemIntrinsicSource::Text { sizes } => sizes,
                source => panic!("{}: unexpected intrinsic source {source:?}", scene.label),
            };
            assert_eq!(raw_sizes.paragraph(), item.id());
            close(raw_sizes.min_content(), expected.min_width, scene.label);
            close(raw_sizes.max_content(), expected.max_width, scene.label);
            close(
                raw_sizes.min_content_height(),
                expected.min_height,
                scene.label,
            );
            close(
                raw_sizes.max_content_height(),
                expected.max_height,
                scene.label,
            );

            let contribution = item_input.contribution_input().unwrap();
            assert_eq!(contribution.revision, item_input.revision);
            close_option(
                contribution.width.min_content,
                expected.min_width,
                scene.label,
            );
            close_option(
                contribution.width.max_content,
                expected.max_width,
                scene.label,
            );
            close_option(
                contribution.height.min_content,
                expected.min_height,
                scene.label,
            );
            close_option(
                contribution.height.max_content,
                expected.max_height,
                scene.label,
            );
            assert_eq!(contribution.width.preferred_size, LayoutDimension::Auto);
            assert_eq!(contribution.height.preferred_size, LayoutDimension::Auto);
            assert_eq!(contribution.width.min_size, LayoutDimension::Auto);
            assert_eq!(contribution.width.max_size, LayoutDimension::Auto);
            assert_eq!(contribution.height.min_size, LayoutDimension::Auto);
            assert_eq!(contribution.height.max_size, LayoutDimension::Auto);
            assert_eq!(
                contribution.width.margin_start,
                LayoutLengthPercentage::LengthPx(0.0)
            );
            assert_eq!(
                contribution.width.margin_end,
                LayoutLengthPercentage::LengthPx(0.0)
            );
            assert_eq!(
                contribution.height.margin_start,
                LayoutLengthPercentage::LengthPx(0.0)
            );
            assert_eq!(
                contribution.height.margin_end,
                LayoutLengthPercentage::LengthPx(0.0)
            );
            assert_eq!(
                contribution.width.padding_start,
                LayoutLengthPercentage::LengthPx(0.0)
            );
            assert_eq!(
                contribution.width.padding_end,
                LayoutLengthPercentage::LengthPx(0.0)
            );
            assert_eq!(
                contribution.height.padding_start,
                LayoutLengthPercentage::LengthPx(0.0)
            );
            assert_eq!(
                contribution.height.padding_end,
                LayoutLengthPercentage::LengthPx(0.0)
            );
            assert_eq!(contribution.width.border_start, 0.0);
            assert_eq!(contribution.width.border_end, 0.0);
            assert_eq!(contribution.height.border_start, 0.0);
            assert_eq!(contribution.height.border_end, 0.0);
            assert_eq!(contribution.text_sources.len(), 0);
            match &contribution.source {
                LayoutGridItemIntrinsicSource::Text { sizes } => {
                    assert!(raw_sizes.uses_same_font_registry_revision(sizes));
                }
                source => panic!("{}: contribution source changed to {source:?}", scene.label),
            }

            let measurements = output
                .layout
                .intrinsic_measurements
                .iter()
                .filter(|measurement| measurement.node == item.id())
                .collect::<Vec<_>>();
            assert!(
                !measurements.is_empty(),
                "{}: missing measure callback",
                scene.label
            );
            assert!(measurements.iter().all(|measurement| {
                measurement.revision == output.layout.revision
                    && matches!(
                        &measurement.source,
                        LayoutIntrinsicMeasurementSource::Text { font_registry_revision }
                            if *font_registry_revision == raw_sizes.font_registry_revision()
                    )
            }));
            let selected_probe = if index == 0 {
                LayoutIntrinsicMeasureSpace::MinContent
            } else {
                LayoutIntrinsicMeasureSpace::MaxContent
            };
            assert!(
                measurements
                    .iter()
                    .any(|measurement| { measurement.available_space.width == selected_probe }),
                "{}: missing selected {selected_probe:?} text probe for source order {index}",
                scene.label
            );

            let frame = output.layout.frames[&item.id()];
            close(frame.x, expected.x, scene.label);
            close(frame.y, 0.0, scene.label);
            close(frame.width, expected.width, scene.label);
            close(frame.height, expected.height, scene.label);
            let text_layout = &output.layout.text_layouts[&item.id()];
            assert_eq!(text_layout.revision, output.layout.revision);
            assert_eq!(text_layout.input.node, item.id());
            assert_eq!(text_layout.input.sources.len(), 1);
            assert_eq!(text_layout.input.sources[0].node, text_node.id());
            assert_eq!(text_layout.input.sources[0].text, expected.text);
            assert!(text_layout.input.sources[0].inline_ancestors.is_empty());
            close(text_layout.layout.available_width, frame.width, scene.label);
            close(text_layout.layout.width, frame.width, scene.label);
            close(text_layout.layout.height, frame.height, scene.label);
            assert_eq!(
                text_layout.layout.font_registry_revision,
                raw_sizes.font_registry_revision()
            );
            assert_eq!(text_layout.layout.lines.len(), expected.line_advances.len());
            for (line_index, (line, expected_advance)) in text_layout
                .layout
                .lines
                .iter()
                .zip(expected.line_advances)
                .enumerate()
            {
                close(line.advance, *expected_advance, scene.label);
                close(line.line_top, line_index as f32 * 10.0, scene.label);
                close(line.line_height, 10.0, scene.label);
            }
        }
    }
}

use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent,
    IntrinsicSizeRevision, OwnerId, StyleRevision, TextOrientation, WritingMode,
};
use spinon_image::PngDecoder;
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutIntrinsicMeasurementSource,
    LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot, LayoutSourceRevision, ParagraphStyle,
    TextLayoutEngine,
};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};

use super::{append_element, append_element_with_tag};
use crate::{
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes,
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs,
};

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");
const PNG: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c14-1-png-basic.png");

#[test]
fn style_only_track_sizing_rejects_vertical_writing_without_text_input() {
    let fixture = super::Fixture::new(
        "display:grid;box-sizing:border-box;width:50px;height:auto;grid-template-columns:50px;grid-template-rows:min-content;writing-mode:horizontal-tb;margin:0;padding:0;border:0",
        &[(
            "item",
            "display:block;box-sizing:border-box;width:auto;height:auto;writing-mode:vertical-rl;margin:0;padding:0;border:0",
        )],
    );

    assert!(matches!(
        fixture.try_compute(),
        Err(crate::StyleLayoutError::UnsupportedComputedValue {
            node,
            property: "writing-mode",
            value,
        }) if node == fixture.nodes["item"].id() && value == "vertical-rl"
    ));
}

#[test]
fn orthogonal_row_intrinsics_reach_taffy_from_computed_style_and_paragraph_input() {
    for (row_track, expected_height, reversed) in [
        ("min-content", 50.0, false),
        ("min-content", 50.0, true),
        ("max-content", 80.0, false),
        ("max-content", 80.0, true),
    ] {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let item = document.reserve_node_handle().unwrap();
        let image = document.reserve_node_handle().unwrap();
        let text = document.reserve_node_handle().unwrap();
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_241).unwrap(), document.document_revision());
        append_element(
            &mut batch,
            root,
            HostParent::Root,
            "grid",
            &format!(
                "display:grid;box-sizing:border-box;width:74px;height:auto;grid-template-columns:50px 24px;grid-template-rows:{row_track};align-items:normal;align-content:start;justify-content:start;column-gap:0;row-gap:0;direction:ltr;writing-mode:horizontal-tb;margin:0;padding:0;border:0"
            ),
        );
        if reversed {
            append_element_with_tag(
                &mut batch,
                image,
                HostParent::Node(root),
                "img",
                "image",
                "display:block;box-sizing:border-box;width:auto;height:auto;grid-column:2;grid-row:1;margin:0;padding:0;border:0",
            );
        }
        append_element(
            &mut batch,
            item,
            HostParent::Node(root),
            "text",
            "display:block;box-sizing:border-box;width:auto;height:auto;grid-column:1;grid-row:1;direction:ltr;writing-mode:vertical-rl;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;margin:0;padding:0;border:0",
        );
        if !reversed {
            append_element_with_tag(
                &mut batch,
                image,
                HostParent::Node(root),
                "img",
                "image",
                "display:block;box-sizing:border-box;width:auto;height:auto;grid-column:2;grid-row:1;margin:0;padding:0;border:0",
            );
        }
        batch.push(DocumentOperation::SetAttribute {
            node: image,
            name: AttributeName::new(None, "src").unwrap(),
            value: "/fixtures/c14-1-png-basic.png".into(),
        });
        batch
            .push(DocumentOperation::CreateText {
                node: text,
                data: "XX XXXXX".to_owned().into(),
            })
            .push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(item),
                node: text,
                before: None,
            });
        document.commit(batch).unwrap();

        let snapshot = document.snapshot();
        let view =
            StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root)
                .unwrap();
        let viewport = CssViewport {
            width_css_px: 240.0,
            height_css_px: 80.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let computed_styles =
            compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
                &view,
                &[],
                viewport,
                StyleRevision::INITIAL,
            )
            .unwrap();
        assert_eq!(
            computed_styles.writing_style_for(root.id()),
            Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
        );
        assert_eq!(
            computed_styles.writing_style_for(item.id()),
            Some((WritingMode::VerticalRl, TextOrientation::Mixed))
        );
        assert_eq!(
            computed_styles.writing_style_for(image.id()),
            Some((WritingMode::HorizontalTb, TextOrientation::Mixed))
        );
        for (node, property, expected) in [
            (root.id(), "align-items", "normal"),
            (root.id(), "align-content", "start"),
            (root.id(), "justify-items", "normal"),
            (root.id(), "justify-content", "start"),
            (item.id(), "align-self", "auto"),
            (item.id(), "justify-self", "auto"),
            (image.id(), "align-self", "auto"),
            (image.id(), "justify-self", "auto"),
            (root.id(), "direction", "ltr"),
            (root.id(), "writing-mode", "horizontal-tb"),
            (root.id(), "text-orientation", "mixed"),
            (item.id(), "direction", "ltr"),
            (item.id(), "writing-mode", "vertical-rl"),
            (item.id(), "text-orientation", "mixed"),
            (item.id(), "font-family", "Ahem"),
            (item.id(), "font-size", "10px"),
            (item.id(), "font-weight", "400"),
            (item.id(), "font-style", "normal"),
            (item.id(), "line-height", "10px"),
            (item.id(), "white-space-collapse", "collapse"),
            (item.id(), "text-wrap-mode", "wrap"),
            (item.id(), "word-break", "normal"),
            (item.id(), "overflow-wrap", "normal"),
            (item.id(), "line-break", "auto"),
            (item.id(), "text-align", "start"),
            (item.id(), "text-transform", "none"),
            (item.id(), "letter-spacing", "normal"),
            (item.id(), "word-spacing", "0px"),
            (item.id(), "text-indent", "0px"),
            (image.id(), "direction", "ltr"),
            (image.id(), "writing-mode", "horizontal-tb"),
            (image.id(), "text-orientation", "mixed"),
        ] {
            let element = computed_styles
                .elements
                .iter()
                .find(|element| element.node_id == node)
                .unwrap();
            assert_eq!(
                element.properties.get(property).map(String::as_str),
                Some(expected),
                "computed {property} for node {node:?}"
            );
        }

        let paragraphs = BTreeMap::from([(
            item.id(),
            ParagraphStyle {
                writing_mode: WritingMode::VerticalRl,
                text_orientation: TextOrientation::Mixed,
                font_family: "Ahem".to_owned(),
                font_size: 10.0,
                line_height: 10.0,
                direction: spinon_layout::TextParagraphDirection::Ltr,
                line_break: spinon_layout::LineBreakMode::Normal,
                ..ParagraphStyle::default()
            },
        )]);
        let mut text_engine = TextLayoutEngine::new_without_system_fonts();
        text_engine.register_font("Ahem", AHEM).unwrap();
        let decoded = PngDecoder.decode(PNG).unwrap();
        let dimensions = decoded.dimensions();
        assert_eq!((dimensions.width(), dimensions.height()), (24, 16));
        let intrinsic_sizes = LayoutIntrinsicSizeSnapshot::new(
            LayoutSourceRevision::HostDocument {
                generation: snapshot.generation(),
                document: snapshot.document_revision(),
                render_tree: snapshot.render_tree_revision(),
            },
            IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
            [(
                image.id(),
                LayoutIntrinsicSize::new(dimensions.width() as f32, dimensions.height() as f32),
            )],
        )
        .unwrap();
        let output = compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
            &snapshot,
            root,
            computed_styles,
            viewport,
            &paragraphs,
            &text_engine,
            intrinsic_sizes,
        )
        .unwrap();

        let root_frame = output.layout.frames[&root.id()];
        let item_frame = output.layout.frames[&item.id()];
        let image_frame = output.layout.frames[&image.id()];
        assert_eq!(
            (root_frame.width, root_frame.height),
            (74.0, expected_height)
        );
        assert_eq!(
            (
                item_frame.x,
                item_frame.y,
                item_frame.width,
                item_frame.height
            ),
            (0.0, 0.0, 50.0, expected_height)
        );
        assert_eq!(
            (
                image_frame.x,
                image_frame.y,
                image_frame.width,
                image_frame.height
            ),
            (50.0, 0.0, 24.0, 16.0)
        );
        assert!(
            output
                .layout
                .intrinsic_measurements
                .iter()
                .any(|measurement| {
                    measurement.node == item.id()
                        && measurement.content_size.height == expected_height
                        && matches!(
                            measurement.source,
                            LayoutIntrinsicMeasurementSource::OrthogonalText { .. }
                        )
                })
        );
        let item_input = output
            .layout
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == item.id())
            .unwrap();
        assert!(matches!(
            &item_input.source,
            LayoutGridItemIntrinsicSource::OrthogonalText { .. }
        ));
        let physical = item_input.contribution_input().unwrap();
        assert_eq!(physical.width.min_content, Some(20.0));
        assert_eq!(physical.width.max_content, Some(10.0));
        assert_eq!(physical.height.min_content, Some(50.0));
        assert_eq!(physical.height.max_content, Some(80.0));
        let image_input = output
            .layout
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == image.id())
            .unwrap();
        assert_eq!(image_input.source_order, if reversed { 0 } else { 1 });
        let LayoutGridItemIntrinsicSource::Replaced { natural } = &image_input.source else {
            panic!("natural image must retain replaced intrinsic metadata");
        };
        assert_eq!(natural.width, Some(24.0));
        assert_eq!(natural.height, Some(16.0));
        assert_eq!(item_input.source_order, if reversed { 1 } else { 0 });
        assert_axis_tracks(
            &output.layout,
            root.id(),
            LayoutGridTrackSizingAxis::Columns,
            &[50.0, 24.0],
        );
        assert_axis_tracks(
            &output.layout,
            root.id(),
            LayoutGridTrackSizingAxis::Rows,
            &[expected_height],
        );
        let row_contributions = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .filter(|trace| trace.node == root.id())
            .flat_map(|trace| trace.passes.iter())
            .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .flat_map(|pass| pass.contributions.iter())
            .collect::<Vec<_>>();
        assert!(row_contributions.iter().any(|contribution| {
            contribution.item == item.id() && (contribution.value - expected_height).abs() <= 0.05
        }));
        assert!(row_contributions.iter().any(|contribution| {
            contribution.item == image.id() && (contribution.value - 16.0).abs() <= 0.05
        }));
    }
}

fn assert_axis_tracks(
    output: &spinon_layout::LayoutOutput,
    grid: spinon_core::NodeId,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
) {
    let trace = output
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid)
        .unwrap();
    let pass = trace.passes.iter().rfind(|pass| pass.axis == axis).unwrap();
    let tracks = pass
        .stages
        .last()
        .unwrap()
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(tracks.len(), expected.len());
    for (actual, expected) in tracks.iter().zip(expected) {
        assert!((actual - expected).abs() <= 0.05);
    }
}

#[test]
fn orthogonal_paragraph_must_match_its_computed_writing_style() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_242).unwrap(), document.document_revision());
    append_element(
        &mut batch,
        root,
        HostParent::Root,
        "grid",
        "display:grid;box-sizing:border-box;width:74px;height:auto;grid-template-columns:50px 24px;grid-template-rows:min-content;align-items:normal;align-content:start;justify-content:start;column-gap:0;row-gap:0;direction:ltr;writing-mode:horizontal-tb;margin:0;padding:0;border:0",
    );
    append_element(
        &mut batch,
        item,
        HostParent::Node(root),
        "text",
        "display:block;box-sizing:border-box;width:auto;height:auto;grid-column:1;grid-row:1;direction:ltr;writing-mode:vertical-rl;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;margin:0;padding:0;border:0",
    );
    batch
        .push(DocumentOperation::CreateText {
            node: text,
            data: "XX XXXXX".to_owned().into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(item),
            node: text,
            before: None,
        });
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root).unwrap();
    let viewport = CssViewport {
        width_css_px: 240.0,
        height_css_px: 80.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let computed_styles = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let paragraphs = BTreeMap::from([(
        item.id(),
        ParagraphStyle {
            writing_mode: WritingMode::HorizontalTb,
            text_orientation: TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size: 10.0,
            line_height: 10.0,
            ..ParagraphStyle::default()
        },
    )]);
    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();

    let error = compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
        &snapshot,
        root,
        computed_styles,
        viewport,
        &paragraphs,
        &text_engine,
    )
    .unwrap_err();
    assert!(
        matches!(
            &error,
            crate::StyleLayoutError::UnsupportedOrthogonalTextProfile { node, .. }
                if *node == item.id()
        ),
        "unexpected error: {error:?}"
    );
}

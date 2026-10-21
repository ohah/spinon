use super::*;

use std::collections::BTreeSet;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind,
};

const ROOT_STYLE: &str = "box-sizing:border-box;display:grid;width:220px;height:180px;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,min-content);grid-template-rows:repeat(3,60px);font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820";
const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const HTML: &str = "http://www.w3.org/1999/xhtml";

#[path = "anonymous_text_multi_image/scenes.rs"]
mod scenes;
use scenes::{Child, SCENES};

#[test]
fn c114_anonymous_text_and_two_or_three_images_share_one_revision_and_track_pass() {
    for (scene_index, scene) in SCENES.iter().enumerate() {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_700 + scene_index as u64).unwrap(),
            document.document_revision(),
        );
        batch.push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        set_attribute(&mut batch, root, "id", &format!("c114-{}-grid", scene.id));
        set_attribute(&mut batch, root, "style", ROOT_STYLE);
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });

        let mut images = Vec::new();
        let mut text = None;
        for child in scene.children {
            match child {
                Child::Text => {
                    let node = document.reserve_node_handle().unwrap();
                    batch.push(DocumentOperation::CreateText {
                        node,
                        data: "XX".into(),
                    });
                    batch.push(DocumentOperation::InsertBefore {
                        parent: HostParent::Node(root),
                        node,
                        before: None,
                    });
                    text = Some(node);
                }
                Child::Image {
                    id,
                    height,
                    column,
                    span,
                    row,
                } => {
                    let node = document.reserve_node_handle().unwrap();
                    batch.push(DocumentOperation::CreateElement {
                        node,
                        namespace: HTML.to_owned(),
                        local_name: "img".to_owned(),
                    });
                    set_attribute(&mut batch, node, "id", id);
                    set_attribute(&mut batch, node, "src", IMAGE_URL);
                    set_attribute(
                        &mut batch,
                        node,
                        "style",
                        &format!(
                            "box-sizing:border-box;width:auto;height:{height}px;margin:0;padding:0;border:0;grid-column:{column} / span {span};grid-row:{row}"
                        ),
                    );
                    batch.push(DocumentOperation::InsertBefore {
                        parent: HostParent::Node(root),
                        node,
                        before: None,
                    });
                    images.push(node);
                }
            }
        }
        assert_eq!(images.len(), scene.children.len() - 1, "{}", scene.id);
        assert!(text.is_some(), "{} must have one direct text run", scene.id);
        document.commit(batch).unwrap();
        let snapshot = Arc::new(document.snapshot());
        let request = WorkRequest {
            key: key_for(&snapshot, EnvironmentRevision::INITIAL),
            snapshot,
            viewport: CssViewport {
                width_css_px: 220.0,
                height_css_px: 180.0,
                device_scale_factor: 1.0,
                environment_revision: EnvironmentRevision::INITIAL,
                media_environment: CssMediaEnvironment::MOBILE,
            },
            previous_snapshot: None,
            force_full: true,
            invalidation: None,
            previous_styles: None,
        };

        let calculation = compute_request_for_block_flex_grid_track_sizing(&request)
            .unwrap_or_else(|error| panic!("{}: {error:?}", scene.id));
        let layout = calculation.layout.unwrap();
        let frame = |handle: HostNodeHandle| {
            layout
                .frames
                .iter()
                .find(|frame| frame.node_id == handle.id().get())
                .unwrap()
        };
        assert_eq!(
            (
                frame(root).x,
                frame(root).y,
                frame(root).width,
                frame(root).height
            ),
            (0.0, 0.0, 220.0, 180.0),
            "{} root frame",
            scene.id
        );

        let diagnostics = layout.grid_track_sizing_diagnostics.unwrap();
        assert_eq!(
            diagnostics.revision.intrinsic_size().get(),
            1,
            "{}",
            scene.id
        );
        assert_eq!(
            diagnostics.item_inputs.len(),
            scene.input_nodes.len(),
            "{}",
            scene.id
        );
        assert!(
            diagnostics
                .item_inputs
                .iter()
                .all(|input| input.revision == diagnostics.revision)
        );
        assert_eq!(
            diagnostics
                .item_inputs
                .iter()
                .map(|input| input.node.get())
                .collect::<Vec<_>>(),
            scene.input_nodes,
            "{} source order",
            scene.id
        );
        for (source_order, input) in diagnostics.item_inputs.iter().enumerate() {
            assert_eq!(input.source_order, source_order, "{}", scene.id);
            assert_eq!(input.grid_container, root.id(), "{}", scene.id);
        }
        let text_input = diagnostics
            .item_inputs
            .iter()
            .find(|input| matches!(input.source, LayoutGridItemIntrinsicSource::Text { .. }))
            .unwrap();
        assert_eq!(
            text_input.node,
            text.unwrap().id(),
            "{} anonymous anchor",
            scene.id
        );
        assert_eq!(text_input.text_sources.len(), 1, "{}", scene.id);
        assert_eq!(
            text_input.text_sources[0].node, text_input.node,
            "{}",
            scene.id
        );
        assert_eq!(text_input.text_sources[0].text, "XX", "{}", scene.id);
        let LayoutGridItemIntrinsicSource::Text { sizes } = &text_input.source else {
            unreachable!();
        };
        assert_eq!(
            (sizes.min_content(), sizes.max_content()),
            (20.0, 20.0),
            "{}",
            scene.id
        );
        assert_eq!(
            (sizes.min_content_height(), sizes.max_content_height()),
            (10.0, 10.0),
            "{}",
            scene.id
        );

        let replaced = diagnostics
            .item_inputs
            .iter()
            .filter(|input| matches!(input.source, LayoutGridItemIntrinsicSource::Replaced { .. }))
            .collect::<Vec<_>>();
        assert_eq!(
            replaced.len(),
            images.len(),
            "{} all images present",
            scene.id
        );
        let expected_image_ids = images
            .iter()
            .map(|image| image.id())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            replaced
                .iter()
                .map(|input| input.node)
                .collect::<BTreeSet<_>>(),
            expected_image_ids,
            "{} image IDs match decoded image IDs",
            scene.id
        );
        for image_input in replaced {
            assert!(matches!(
                &image_input.source,
                LayoutGridItemIntrinsicSource::Replaced { natural }
                    if (natural.width, natural.height) == (Some(24.0), Some(16.0))
            ));
        }

        let actual_item_frames = diagnostics
            .item_frames
            .iter()
            .map(|(node, frame)| (node.get(), frame.x, frame.y, frame.width, frame.height))
            .collect::<Vec<_>>();
        assert_eq!(
            actual_item_frames.len(),
            scene.item_frames.len(),
            "{}",
            scene.id
        );
        for (actual, expected) in actual_item_frames.iter().zip(scene.item_frames) {
            assert_eq!(actual.0, expected.0, "{} frame node", scene.id);
            assert!(
                (actual.1 - expected.1).abs() <= 1.0 / 64.0
                    && (actual.2 - expected.2).abs() <= 1.0 / 64.0
                    && (actual.3 - expected.3).abs() <= 1.0 / 64.0
                    && (actual.4 - expected.4).abs() <= 1.0 / 64.0,
                "{} frame {:?} != {:?}",
                scene.id,
                actual,
                expected
            );
        }

        let trace = diagnostics
            .traces
            .iter()
            .find(|trace| trace.node == root.id())
            .unwrap();
        let columns = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .unwrap();
        let final_columns = columns
            .stages
            .last()
            .unwrap()
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(final_columns.len(), scene.columns.len(), "{}", scene.id);
        for (actual, expected) in final_columns.iter().zip(scene.columns) {
            assert!(
                (actual - expected).abs() <= 1.0 / 64.0,
                "{}: {actual} != {expected}",
                scene.id
            );
        }
        let rows = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .unwrap();
        let final_rows = rows
            .stages
            .last()
            .unwrap()
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(final_rows, [60.0, 60.0, 60.0], "{} row sizing", scene.id);
        let spans = columns
            .stages
            .iter()
            .filter_map(|stage| match stage.stage {
                LayoutGridTrackSizingStageKind::IntrinsicItemBatch { span, .. } => Some(span),
                _ => None,
            })
            .collect::<Vec<_>>();
        let expected_spans = if images.len() == 2 {
            vec![1, 2]
        } else {
            vec![1, 2, 3]
        };
        assert_eq!(spans, expected_spans, "{} intrinsic batch spans", scene.id);

        let mut expected_contributions = Vec::new();
        for (item_index, child) in scene.children.iter().enumerate() {
            let node = scene.input_nodes[item_index];
            match child {
                Child::Text => expected_contributions
                    .extend([(node, "MinContent", 1, 20.0), (node, "MinContent", 1, 20.0)]),
                Child::Image { height, span, .. } => {
                    let span = u16::from(*span);
                    let min_content = f32::from(*height) * 1.5;
                    expected_contributions.push((node, "Minimum", span, 0.0));
                    expected_contributions.extend([
                        (node, "MinContent", span, min_content),
                        (node, "MinContent", span, min_content),
                    ]);
                }
            }
        }
        assert_eq!(
            columns.contributions.len(),
            expected_contributions.len(),
            "{} contribution trace length",
            scene.id
        );
        let mut actual_contributions = columns
            .contributions
            .iter()
            .map(|contribution| {
                (
                    contribution.item.get(),
                    format!("{:?}", contribution.kind),
                    contribution.span,
                    contribution.value,
                )
            })
            .collect::<Vec<_>>();
        let sort_contributions = |items: &mut Vec<(u64, String, u16, f32)>| {
            items.sort_by(|left, right| {
                left.0
                    .cmp(&right.0)
                    .then_with(|| left.1.cmp(&right.1))
                    .then_with(|| left.2.cmp(&right.2))
                    .then_with(|| left.3.total_cmp(&right.3))
            });
        };
        let mut expected_contributions = expected_contributions
            .into_iter()
            .map(|(node, kind, span, value)| (node, kind.to_owned(), span, value))
            .collect::<Vec<_>>();
        sort_contributions(&mut actual_contributions);
        sort_contributions(&mut expected_contributions);
        for (actual, expected) in actual_contributions.iter().zip(expected_contributions) {
            assert_eq!(actual.0, expected.0, "{} contribution item", scene.id);
            assert_eq!(actual.1, expected.1, "{} contribution kind", scene.id);
            assert_eq!(actual.2, expected.2, "{} contribution span", scene.id);
            assert!(
                (actual.3 - expected.3).abs() <= 1.0 / 64.0,
                "{} contribution value {} != {}",
                scene.id,
                actual.3,
                expected.3
            );
        }
    }
}

fn set_attribute(batch: &mut DocumentChangeBatch, node: HostNodeHandle, name: &str, value: &str) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, name).unwrap(),
        value: value.to_owned().into(),
    });
}

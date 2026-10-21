use super::*;

const BORDERED_ROOT_STYLE: &str = "box-sizing:border-box;display:grid;width:40px;height:184px;margin:0;padding:0;border:2px solid #f0c674;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,min-content);grid-template-rows:repeat(3,60px);font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820";
const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";

#[derive(Clone, Copy)]
enum Child {
    Text,
    Image {
        id: &'static str,
        height: u8,
        column: u8,
        row: u8,
    },
}

struct Scene {
    id: &'static str,
    children: &'static [Child],
    input_nodes: &'static [u64],
    expected_item_frames: &'static [(u64, f32, f32, f32, f32)],
}

const FORWARD: [Child; 4] = [
    Child::Text,
    Child::Image {
        id: "c114-overflow-image-a-forward",
        height: 16,
        column: 1,
        row: 1,
    },
    Child::Text,
    Child::Image {
        id: "c114-overflow-image-b-forward",
        height: 32,
        column: 2,
        row: 2,
    },
];
const REVERSED: [Child; 4] = [
    Child::Image {
        id: "c114-overflow-image-b-reversed",
        height: 32,
        column: 2,
        row: 2,
    },
    Child::Text,
    Child::Image {
        id: "c114-overflow-image-a-reversed",
        height: 16,
        column: 1,
        row: 1,
    },
    Child::Text,
];
const SCENES: [Scene; 2] = [
    Scene {
        id: "anonymous-runs-two-images-forward",
        children: &FORWARD,
        input_nodes: &[2, 3, 4, 5],
        expected_item_frames: &[
            (2, 42.0, 2.0, 20.0, 10.0),
            (3, 2.0, 2.0, 24.0, 16.0),
            (4, 62.0, 2.0, 20.0, 10.0),
            (5, 14.0, 62.0, 48.0, 32.0),
        ],
    },
    Scene {
        id: "two-images-anonymous-runs-reversed",
        children: &REVERSED,
        input_nodes: &[2, 3, 4, 5],
        expected_item_frames: &[
            (2, 14.0, 62.0, 48.0, 32.0),
            (3, 42.0, 2.0, 20.0, 10.0),
            (4, 2.0, 2.0, 24.0, 16.0),
            (5, 62.0, 2.0, 20.0, 10.0),
        ],
    },
];

#[test]
fn two_anonymous_runs_and_images_keep_border_box_and_visible_overflow_in_both_orders() {
    for (scene_index, scene) in SCENES.iter().enumerate() {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_800 + scene_index as u64).unwrap(),
            document.document_revision(),
        );
        batch.push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        set_attribute(&mut batch, root, "id", &format!("c114-{}-grid", scene.id));
        set_attribute(&mut batch, root, "style", BORDERED_ROOT_STYLE);
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });

        let mut text_nodes = Vec::new();
        let mut image_nodes = Vec::new();
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
                    text_nodes.push(node);
                }
                Child::Image {
                    id,
                    height,
                    column,
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
                            "box-sizing:border-box;width:auto;height:{height}px;margin:0;padding:0;border:0;grid-column:{column} / span 2;grid-row:{row}"
                        ),
                    );
                    batch.push(DocumentOperation::InsertBefore {
                        parent: HostParent::Node(root),
                        node,
                        before: None,
                    });
                    image_nodes.push(node);
                }
            }
        }
        document.commit(batch).unwrap();
        let snapshot = Arc::new(document.snapshot());
        let request = WorkRequest {
            key: key_for(&snapshot, EnvironmentRevision::INITIAL),
            snapshot,
            viewport: CssViewport {
                width_css_px: 80.0,
                height_css_px: 200.0,
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
            (0.0, 0.0, 40.0, 184.0),
            "{} border-box frame",
            scene.id
        );
        let diagnostics = layout.grid_track_sizing_diagnostics.unwrap();
        assert_eq!(
            diagnostics.revision.intrinsic_size().get(),
            1,
            "{}",
            scene.id
        );
        assert_eq!(diagnostics.item_inputs.len(), 4, "{}", scene.id);
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
            "{} direct source order",
            scene.id
        );
        for (source_order, input) in diagnostics.item_inputs.iter().enumerate() {
            assert_eq!(input.source_order, source_order, "{}", scene.id);
            assert_eq!(input.grid_container, root.id(), "{}", scene.id);
        }

        let text_inputs = diagnostics
            .item_inputs
            .iter()
            .filter(|input| matches!(input.source, LayoutGridItemIntrinsicSource::Text { .. }))
            .collect::<Vec<_>>();
        assert_eq!(text_inputs.len(), 2, "{} anonymous text items", scene.id);
        assert_eq!(
            text_inputs
                .iter()
                .map(|input| input.node)
                .collect::<Vec<_>>(),
            text_nodes.iter().map(|node| node.id()).collect::<Vec<_>>(),
            "{} text anchors follow source order",
            scene.id
        );
        for input in text_inputs {
            assert_eq!(input.text_sources.len(), 1, "{}", scene.id);
            assert_eq!(input.text_sources[0].node, input.node, "{}", scene.id);
            assert_eq!(input.text_sources[0].text, "XX", "{}", scene.id);
            let LayoutGridItemIntrinsicSource::Text { sizes } = &input.source else {
                unreachable!();
            };
            assert_eq!(
                (sizes.min_content(), sizes.max_content()),
                (20.0, 20.0),
                "{}",
                scene.id
            );
        }

        let replaced = diagnostics
            .item_inputs
            .iter()
            .filter(|input| matches!(input.source, LayoutGridItemIntrinsicSource::Replaced { .. }))
            .collect::<Vec<_>>();
        assert_eq!(replaced.len(), 2, "{} image inputs", scene.id);
        assert_eq!(
            replaced
                .iter()
                .map(|input| input.node)
                .collect::<BTreeSet<_>>(),
            image_nodes
                .iter()
                .map(|node| node.id())
                .collect::<BTreeSet<_>>(),
            "{} image identities",
            scene.id
        );
        assert!(replaced.iter().all(|input| matches!(
            &input.source,
            LayoutGridItemIntrinsicSource::Replaced { natural }
                if (natural.width, natural.height) == (Some(24.0), Some(16.0))
        )));

        let actual_item_frames = diagnostics
            .item_frames
            .iter()
            .map(|(node, item)| (node.get(), item.x, item.y, item.width, item.height))
            .collect::<Vec<_>>();
        assert_eq!(
            actual_item_frames.len(),
            scene.expected_item_frames.len(),
            "{}",
            scene.id
        );
        for (actual, expected) in actual_item_frames.iter().zip(scene.expected_item_frames) {
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
        let second_image = actual_item_frames
            .iter()
            .find(|frame| frame.3 == 48.0 && frame.4 == 32.0)
            .unwrap();
        assert_eq!(
            second_image.1 + second_image.3 - frame(root).width,
            22.0,
            "{} overflow",
            scene.id
        );

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
        let column_tracks = columns
            .stages
            .last()
            .unwrap()
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(
            column_tracks,
            [12.0, 28.0, 20.0, 20.0],
            "{} column sizing",
            scene.id
        );
        let rows = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .unwrap();
        let row_tracks = rows
            .stages
            .last()
            .unwrap()
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(row_tracks, [60.0, 60.0, 60.0], "{} row sizing", scene.id);
        let spans = columns
            .stages
            .iter()
            .filter_map(|stage| match stage.stage {
                LayoutGridTrackSizingStageKind::IntrinsicItemBatch { span, .. } => Some(span),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(spans, [1, 2], "{} intrinsic span stages", scene.id);
    }
}

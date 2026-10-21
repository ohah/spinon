use super::*;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind,
};

const ROOT_STYLE: &str = "box-sizing:border-box;display:grid;width:220px;height:60px;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,min-content);grid-template-rows:60px;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#101820";
const IMAGE_STYLE: &str =
    "box-sizing:border-box;width:auto;height:48px;margin:0;padding:0;border:0;grid-column:span 2";
const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const HTML: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone, Copy)]
enum Child {
    Text(&'static str),
    Image(&'static str),
    Hidden {
        id: &'static str,
        text: &'static str,
    },
}

struct Scene {
    id: &'static str,
    children: &'static [Child],
    tracks: &'static [f32],
    image_x: f32,
    input_nodes: &'static [u64],
    text_nodes: &'static [(&'static str, &'static [&'static str], f32, f32)],
}

const SCENES: [Scene; 5] = [
    Scene {
        id: "anonymous-text-image-text",
        children: &[
            Child::Text("XX"),
            Child::Image("c114-anonymous-text-image-middle-image"),
            Child::Text("XX"),
        ],
        tracks: &[20.0, 36.0, 36.0, 20.0],
        image_x: 20.0,
        input_nodes: &[2, 3, 4],
        text_nodes: &[("XX", &["XX"], 0.0, 20.0), ("XX", &["XX"], 92.0, 20.0)],
    },
    Scene {
        id: "image-before-comment-group",
        children: &[
            Child::Image("c114-image-before-comment-group-image"),
            Child::Text("XX"),
            Child::Text("XX"),
        ],
        tracks: &[36.0, 36.0, 40.0, 0.0],
        image_x: 0.0,
        input_nodes: &[2, 3],
        text_nodes: &[("XXXX", &["XX", "XX"], 72.0, 40.0)],
    },
    Scene {
        id: "comment-group-before-image",
        children: &[
            Child::Text("XX"),
            Child::Text("XX"),
            Child::Image("c114-comment-group-before-image-image"),
        ],
        tracks: &[40.0, 36.0, 36.0, 0.0],
        image_x: 40.0,
        input_nodes: &[2, 4],
        text_nodes: &[("XXXX", &["XX", "XX"], 0.0, 40.0)],
    },
    Scene {
        id: "whitespace-around-image",
        children: &[
            Child::Text(" \t\n"),
            Child::Image("c114-whitespace-around-image-image"),
            Child::Text("\n\tXX"),
        ],
        tracks: &[36.0, 36.0, 20.0, 0.0],
        image_x: 0.0,
        input_nodes: &[3, 4],
        text_nodes: &[("\n\tXX", &["\n\tXX"], 72.0, 20.0)],
    },
    Scene {
        id: "hidden-child-around-image",
        children: &[
            Child::Text("XX"),
            Child::Hidden {
                id: "c114-hidden-child-around-image-hidden",
                text: "ignored",
            },
            Child::Image("c114-hidden-child-around-image-image"),
            Child::Text("XX"),
        ],
        tracks: &[20.0, 36.0, 36.0, 20.0],
        image_x: 20.0,
        input_nodes: &[2, 5, 6],
        text_nodes: &[("XX", &["XX"], 0.0, 20.0), ("XX", &["XX"], 92.0, 20.0)],
    },
];

#[test]
fn c114_anonymous_text_and_default_inline_image_share_one_revision_and_track_pass() {
    for (scene_index, scene) in SCENES.iter().enumerate() {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_600 + scene_index as u64).unwrap(),
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

        let mut image = None;
        for child in scene.children {
            match child {
                Child::Text(value) => {
                    let text = document.reserve_node_handle().unwrap();
                    batch.push(DocumentOperation::CreateText {
                        node: text,
                        data: (*value).into(),
                    });
                    batch.push(DocumentOperation::InsertBefore {
                        parent: HostParent::Node(root),
                        node: text,
                        before: None,
                    });
                }
                Child::Image(id) => {
                    let node = document.reserve_node_handle().unwrap();
                    batch.push(DocumentOperation::CreateElement {
                        node,
                        namespace: HTML.to_owned(),
                        local_name: "img".to_owned(),
                    });
                    set_attribute(&mut batch, node, "id", id);
                    set_attribute(&mut batch, node, "src", IMAGE_URL);
                    set_attribute(&mut batch, node, "style", IMAGE_STYLE);
                    batch.push(DocumentOperation::InsertBefore {
                        parent: HostParent::Node(root),
                        node,
                        before: None,
                    });
                    image = Some(node);
                }
                Child::Hidden { id, text: value } => {
                    let node = document.reserve_node_handle().unwrap();
                    batch.push(DocumentOperation::CreateElement {
                        node,
                        namespace: HTML.to_owned(),
                        local_name: "span".to_owned(),
                    });
                    set_attribute(&mut batch, node, "id", id);
                    set_attribute(&mut batch, node, "style", "display:none");
                    batch.push(DocumentOperation::InsertBefore {
                        parent: HostParent::Node(root),
                        node,
                        before: None,
                    });
                    let child_text = document.reserve_node_handle().unwrap();
                    batch.push(DocumentOperation::CreateText {
                        node: child_text,
                        data: (*value).into(),
                    });
                    batch.push(DocumentOperation::InsertBefore {
                        parent: HostParent::Node(node),
                        node: child_text,
                        before: None,
                    });
                }
            }
        }
        document.commit(batch).unwrap();
        let snapshot = Arc::new(document.snapshot());
        let request = WorkRequest {
            key: key_for(&snapshot, EnvironmentRevision::INITIAL),
            snapshot,
            viewport: CssViewport {
                width_css_px: 220.0,
                height_css_px: 60.0,
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
        let frame = |handle: spinon_core::HostNodeHandle| {
            layout
                .frames
                .iter()
                .find(|frame| frame.node_id == handle.id().get())
                .unwrap()
        };
        assert_eq!(
            (frame(root).width, frame(root).height),
            (220.0, 60.0),
            "{}",
            scene.id
        );
        let image = image.expect("one image per scene");
        assert_eq!(
            (
                frame(image).x,
                frame(image).y,
                frame(image).width,
                frame(image).height
            ),
            (scene.image_x, 0.0, 72.0, 48.0),
            "{} image frame",
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
            scene.text_nodes.len() + 1,
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
            "{} item source order",
            scene.id
        );
        for (source_order, input) in diagnostics.item_inputs.iter().enumerate() {
            assert_eq!(input.source_order, source_order, "{}", scene.id);
        }
        let image_input = diagnostics
            .item_inputs
            .iter()
            .find(|input| input.node == image.id())
            .unwrap();
        assert_eq!(image_input.grid_container, root.id());
        assert!(matches!(
            &image_input.source,
            LayoutGridItemIntrinsicSource::Replaced { natural }
                if (natural.width, natural.height) == (Some(24.0), Some(16.0))
        ));

        let mut text_inputs = diagnostics
            .item_inputs
            .iter()
            .filter(|input| matches!(input.source, LayoutGridItemIntrinsicSource::Text { .. }))
            .collect::<Vec<_>>();
        text_inputs.sort_by_key(|input| input.source_order);
        assert_eq!(text_inputs.len(), scene.text_nodes.len(), "{}", scene.id);
        for (input, (expected_joined_text, expected_sources, expected_x, expected_width)) in
            text_inputs.iter().zip(scene.text_nodes)
        {
            let LayoutGridItemIntrinsicSource::Text { sizes } = &input.source else {
                unreachable!();
            };
            assert_eq!(input.grid_container, root.id(), "{}", scene.id);
            assert_eq!(sizes.min_content(), *expected_width, "{}", scene.id);
            assert_eq!(sizes.max_content(), *expected_width, "{}", scene.id);
            assert_eq!(sizes.min_content_height(), 10.0, "{}", scene.id);
            assert_eq!(sizes.max_content_height(), 10.0, "{}", scene.id);
            assert_eq!(
                input.text_sources.len(),
                expected_sources.len(),
                "{}",
                scene.id
            );
            let actual_sources = input
                .text_sources
                .iter()
                .map(|source| source.text.as_str())
                .collect::<Vec<_>>();
            assert_eq!(
                actual_sources, *expected_sources,
                "{} source grouping",
                scene.id
            );
            assert_eq!(
                actual_sources.concat(),
                *expected_joined_text,
                "{} joined anonymous text",
                scene.id
            );
            assert_eq!(
                input.node, input.text_sources[0].node,
                "{} anchor node",
                scene.id
            );
            assert!(
                input
                    .text_sources
                    .iter()
                    .any(|source| !source.text.trim().is_empty())
            );
            let (_, anonymous_frame) = diagnostics
                .item_frames
                .iter()
                .find(|(node, _)| *node == input.node)
                .unwrap();
            assert_eq!(anonymous_frame.x, *expected_x, "{}", scene.id);
            assert_eq!(anonymous_frame.width, *expected_width, "{}", scene.id);
            assert_eq!(anonymous_frame.height, 10.0, "{}", scene.id);
        }

        let trace = diagnostics
            .traces
            .iter()
            .find(|trace| trace.node == root.id())
            .unwrap();
        let column_pass = trace
            .passes
            .iter()
            .rfind(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .unwrap();
        let final_tracks = column_pass
            .stages
            .last()
            .unwrap()
            .tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(final_tracks.len(), scene.tracks.len(), "{}", scene.id);
        for (actual, expected) in final_tracks.iter().zip(scene.tracks) {
            assert!(
                (actual - expected).abs() <= 1.0 / 64.0,
                "{}: {actual} != {expected}",
                scene.id
            );
        }
        let spans = column_pass
            .stages
            .iter()
            .filter_map(|stage| match stage.stage {
                LayoutGridTrackSizingStageKind::IntrinsicItemBatch { span, .. } => Some(span),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(spans, vec![1, 2], "{} intrinsic span trace", scene.id);
    }
}

fn set_attribute(
    batch: &mut DocumentChangeBatch,
    node: spinon_core::HostNodeHandle,
    name: &str,
    value: &str,
) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, name).unwrap(),
        value: value.to_owned().into(),
    });
}

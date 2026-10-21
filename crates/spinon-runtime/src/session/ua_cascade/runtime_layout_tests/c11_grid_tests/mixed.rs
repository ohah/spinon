use super::*;

use std::collections::BTreeSet;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingMinimumContributionSource,
    LayoutGridTrackSizingStageKind,
};

#[derive(Clone, Copy)]
enum Kind {
    Text {
        value: &'static str,
        min: f32,
        max: f32,
        min_height: f32,
    },
    Image {
        height: f32,
    },
}

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    kind: Kind,
    column: usize,
    span: usize,
    row: usize,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    items: &'static [Item],
    tracks: &'static [f32],
}

const A_FORWARD: Item = Item {
    id: "mixed-text-a-forward",
    kind: Kind::Text {
        value: "XX XXXXX",
        min: 50.0,
        max: 80.0,
        min_height: 20.0,
    },
    column: 1,
    span: 2,
    row: 1,
};
const B_FORWARD: Item = Item {
    id: "mixed-image-b-forward",
    kind: Kind::Image { height: 48.0 },
    column: 2,
    span: 2,
    row: 2,
};
const B_REVERSED: Item = Item {
    id: "mixed-image-b-reversed",
    ..B_FORWARD
};
const A_REVERSED: Item = Item {
    id: "mixed-text-a-reversed",
    ..A_FORWARD
};
const A_THREE: Item = Item {
    id: "mixed-text-c-a",
    ..A_FORWARD
};
const B_THREE: Item = Item {
    id: "mixed-image-c-b",
    ..B_FORWARD
};
const C_THREE: Item = Item {
    id: "mixed-text-c-c",
    kind: Kind::Text {
        value: "XXXXXXXXXXXXXXXXXX",
        min: 180.0,
        max: 180.0,
        min_height: 10.0,
    },
    column: 1,
    span: 3,
    row: 3,
};
const C_THREE_REVERSED: Item = Item {
    id: "mixed-text-r-c",
    ..C_THREE
};
const B_THREE_REVERSED: Item = Item {
    id: "mixed-image-r-b",
    ..B_THREE
};
const A_THREE_REVERSED: Item = Item {
    id: "mixed-text-r-a",
    ..A_THREE
};

const SPAN2_FORWARD: [Item; 2] = [A_FORWARD, B_FORWARD];
const SPAN2_REVERSED: [Item; 2] = [B_REVERSED, A_REVERSED];
const SPAN2_THEN_SPAN3: [Item; 3] = [A_THREE, B_THREE, C_THREE];
const SPAN3_THEN_SPAN2: [Item; 3] = [C_THREE_REVERSED, B_THREE_REVERSED, A_THREE_REVERSED];

const SCENES: [Scene; 4] = [
    Scene {
        id: "mixed-text-image-span2-forward",
        items: &SPAN2_FORWARD,
        tracks: &[25.0, 36.0, 36.0, 0.0],
    },
    Scene {
        id: "mixed-text-image-span2-reversed",
        items: &SPAN2_REVERSED,
        tracks: &[25.0, 36.0, 36.0, 0.0],
    },
    Scene {
        id: "mixed-text-image-span2-before-span3",
        items: &SPAN2_THEN_SPAN3,
        tracks: &[158.0 / 3.0, 191.0 / 3.0, 191.0 / 3.0, 0.0],
    },
    Scene {
        id: "mixed-text-image-span3-before-span2",
        items: &SPAN3_THEN_SPAN2,
        tracks: &[158.0 / 3.0, 191.0 / 3.0, 191.0 / 3.0, 0.0],
    },
];

#[test]
fn c114_mixed_text_image_inputs_share_one_revision_and_span_sizing_pass() {
    const ROOT_STYLE: &str = "box-sizing:border-box;display:grid;width:220px;height:180px;margin:0;padding:0;border:0;gap:0;align-items:start;align-content:start;justify-content:start;grid-template-columns:repeat(4,min-content);grid-template-rows:repeat(3,60px);background-color:#101820";
    const TEXT_STYLE: &str = "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d";
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";

    for (scene_index, scene) in SCENES.iter().enumerate() {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_417 + scene_index as u64).unwrap(),
            document.document_revision(),
        );
        batch.push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        for (name, value) in [
            ("id", format!("c114-{}-grid", scene.id)),
            ("style", ROOT_STYLE.to_owned()),
        ] {
            batch.push(DocumentOperation::SetAttribute {
                node: root,
                name: AttributeName::new(None, name).unwrap(),
                value: value.into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });

        let mut item_handles = Vec::with_capacity(scene.items.len());
        for item in scene.items {
            let handle = document.reserve_node_handle().unwrap();
            let tag = match item.kind {
                Kind::Text { .. } => "div",
                Kind::Image { .. } => "img",
            };
            let style = match item.kind {
                Kind::Text { .. } => format!(
                    "{TEXT_STYLE};grid-column:{} / span {};grid-row:{}",
                    item.column, item.span, item.row
                ),
                Kind::Image { height } => format!(
                    "box-sizing:border-box;display:block;width:auto;height:{height}px;margin:0;padding:0;border:0;grid-column:{} / span {};grid-row:{};justify-self:auto;align-self:auto;background-color:#e5484d",
                    item.column, item.span, item.row
                ),
            };
            batch.push(DocumentOperation::CreateElement {
                node: handle,
                namespace: HTML.to_owned(),
                local_name: tag.to_owned(),
            });
            for (name, value) in [("id", item.id.to_owned()), ("style", style)] {
                batch.push(DocumentOperation::SetAttribute {
                    node: handle,
                    name: AttributeName::new(None, name).unwrap(),
                    value: value.into(),
                });
            }
            if matches!(item.kind, Kind::Image { .. }) {
                batch.push(DocumentOperation::SetAttribute {
                    node: handle,
                    name: AttributeName::new(None, "src").unwrap(),
                    value: IMAGE_URL.to_owned().into(),
                });
            }
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(root),
                node: handle,
                before: None,
            });
            if let Kind::Text { value, .. } = item.kind {
                let text = document.reserve_node_handle().unwrap();
                batch.push(DocumentOperation::CreateText {
                    node: text,
                    data: value.into(),
                });
                batch.push(DocumentOperation::InsertBefore {
                    parent: HostParent::Node(handle),
                    node: text,
                    before: None,
                });
            }
            item_handles.push(handle);
        }
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

        let calculation = compute_request_for_block_flex_grid_track_sizing(&request).unwrap();
        let layout = calculation.layout.unwrap();
        let frame = |handle: HostNodeHandle| {
            layout
                .frames
                .iter()
                .find(|frame| frame.node_id == handle.id().get())
                .unwrap()
        };
        assert_eq!(
            (frame(root).width, frame(root).height),
            (220.0, 180.0),
            "{}",
            scene.id
        );

        let diagnostics = layout.grid_track_sizing_diagnostics.unwrap();
        assert_eq!(
            diagnostics.item_inputs.len(),
            scene.items.len(),
            "{}",
            scene.id
        );
        assert_eq!(
            diagnostics.revision.intrinsic_size().get(),
            1,
            "{}",
            scene.id
        );
        let mut seen_text_nodes = BTreeSet::new();
        for (index, (input, expected)) in
            diagnostics.item_inputs.iter().zip(scene.items).enumerate()
        {
            assert_eq!(input.grid_container, root.id(), "{} item {index}", scene.id);
            assert_eq!(
                input.node,
                item_handles[index].id(),
                "{} item {index}",
                scene.id
            );
            assert_eq!(input.source_order, index, "{} item {index}", scene.id);
            assert_eq!(
                input.revision, diagnostics.revision,
                "{} item {index}",
                scene.id
            );
            match (&input.source, expected.kind) {
                (
                    LayoutGridItemIntrinsicSource::Text { sizes },
                    Kind::Text {
                        min,
                        max,
                        min_height,
                        ..
                    },
                ) => {
                    assert_eq!(sizes.min_content(), min, "{} item {index}", scene.id);
                    assert_eq!(sizes.max_content(), max, "{} item {index}", scene.id);
                    assert_eq!(
                        sizes.min_content_height(),
                        min_height,
                        "{} item {index}",
                        scene.id
                    );
                    assert_eq!(
                        sizes.max_content_height(),
                        10.0,
                        "{} item {index}",
                        scene.id
                    );
                    seen_text_nodes.insert(input.node);
                }
                (LayoutGridItemIntrinsicSource::Replaced { natural }, Kind::Image { .. }) => {
                    assert_eq!(
                        (natural.width, natural.height),
                        (Some(24.0), Some(16.0)),
                        "{} item {index}",
                        scene.id
                    );
                }
                _ => panic!("{} item {index} intrinsic source kind changed", scene.id),
            }
        }
        assert_eq!(
            seen_text_nodes.len(),
            if scene.items.len() == 2 { 1 } else { 2 },
            "{}",
            scene.id
        );

        let trace = diagnostics
            .traces
            .iter()
            .find(|trace| trace.node == root.id())
            .unwrap();
        let column_pass = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .last()
            .unwrap();
        let expected_spans = if scene.items.len() == 2 {
            vec![2]
        } else {
            vec![2, 3]
        };
        let observed_spans = column_pass
            .stages
            .iter()
            .filter_map(|stage| match stage.stage {
                LayoutGridTrackSizingStageKind::IntrinsicItemBatch { span, .. } => Some(span),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(observed_spans, expected_spans, "{} span groups", scene.id);

        let final_tracks = column_pass
            .stages
            .last()
            .unwrap()
            .tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(
            final_tracks.len(),
            scene.tracks.len(),
            "{} track count",
            scene.id
        );
        for (actual, expected) in final_tracks.iter().zip(scene.tracks) {
            assert!(
                (actual - expected).abs() <= 1.0 / 64.0,
                "{} tracks: expected {expected}, got {actual}",
                scene.id
            );
        }

        for span in &expected_spans {
            let span_items = scene
                .items
                .iter()
                .enumerate()
                .filter(|(_, item)| item.span == *span as usize)
                .collect::<Vec<_>>();
            for kind in [
                LayoutGridTrackSizingContributionKind::Minimum,
                LayoutGridTrackSizingContributionKind::MinContent,
            ] {
                for (index, expected) in &span_items {
                    let contribution = column_pass
                        .contributions
                        .iter()
                        .find(|contribution| {
                            contribution.item == item_handles[*index].id()
                                && contribution.kind == kind
                                && contribution.span == *span
                        })
                        .unwrap_or_else(|| panic!("{} missing {kind:?} item {index}", scene.id));
                    let expected_value = match (kind, expected.kind) {
                        (LayoutGridTrackSizingContributionKind::Minimum, _) => 0.0,
                        (
                            LayoutGridTrackSizingContributionKind::MinContent,
                            Kind::Text { min, .. },
                        ) => min,
                        (LayoutGridTrackSizingContributionKind::MinContent, Kind::Image { .. }) => {
                            72.0
                        }
                        _ => unreachable!(),
                    };
                    assert!(
                        (contribution.value - expected_value).abs() <= 0.0001,
                        "{} item {index} {kind:?}: expected {expected_value}, got {}",
                        scene.id,
                        contribution.value
                    );
                    if kind == LayoutGridTrackSizingContributionKind::Minimum {
                        assert!(
                            matches!(
                            contribution.minimum_source,
                            Some(LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                                spans_auto_min_track: false,
                                spans_multiple_tracks: true,
                                content_based: false,
                                ..
                            })
                        ),
                            "{} item {index} automatic minimum trace",
                            scene.id
                        );
                    }
                }
            }
        }

        let row_pass = trace
            .passes
            .iter()
            .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .unwrap();
        let rows = row_pass
            .stages
            .last()
            .unwrap()
            .tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(rows, [60.0, 60.0, 60.0], "{} fixed rows", scene.id);

        for (handle, expected) in item_handles.iter().zip(scene.items) {
            let start = expected.column - 1;
            let x = scene.tracks[..start].iter().sum::<f32>();
            let grid_area_width = scene.tracks[start..start + expected.span]
                .iter()
                .sum::<f32>();
            let width = match expected.kind {
                Kind::Image { .. } => 72.0,
                Kind::Text { .. } => grid_area_width,
            };
            let y = (expected.row as f32 - 1.0) * 60.0;
            let height = match expected.kind {
                Kind::Image { height } => height,
                Kind::Text { max, .. } if width < max => 20.0,
                Kind::Text { .. } => 10.0,
            };
            let actual = frame(*handle);
            for (value, target) in [
                (actual.x, x),
                (actual.y, y),
                (actual.width, width),
                (actual.height, height),
            ] {
                assert!(
                    (value - target).abs() <= 1.0 / 64.0,
                    "{} node {} frame {actual:?}, expected x={x} y={y} width={width} height={height}",
                    scene.id,
                    handle.id().get()
                );
            }
        }
    }
}

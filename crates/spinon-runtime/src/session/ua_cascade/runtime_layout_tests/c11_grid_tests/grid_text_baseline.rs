use super::*;

use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument};

#[derive(Clone, Copy)]
enum Scene {
    LastAlignItems,
    LastAlignSelf,
    LastReversed,
    FirstControl,
    StartControl,
}

impl Scene {
    fn root_id(self) -> &'static str {
        match self {
            Self::LastAlignItems => "c114-grid-text-last-baseline-align-items-multiline",
            Self::LastAlignSelf => "c114-grid-text-last-baseline-align-self-multiline",
            Self::LastReversed => "c114-grid-text-last-baseline-reversed-multiline",
            Self::FirstControl => "c114-grid-text-last-baseline-first-control",
            Self::StartControl => "c114-grid-text-last-baseline-start-control",
        }
    }

    fn align_items(self) -> &'static str {
        match self {
            Self::LastAlignItems | Self::LastReversed => "last baseline",
            Self::FirstControl => "first baseline",
            Self::LastAlignSelf | Self::StartControl => "start",
        }
    }

    fn align_self(self) -> &'static str {
        match self {
            Self::LastAlignSelf => "last baseline",
            Self::FirstControl => "auto",
            Self::LastAlignItems | Self::LastReversed | Self::StartControl => "auto",
        }
    }

    fn small_text(self) -> &'static str {
        match self {
            Self::FirstControl => "XX",
            Self::LastAlignItems
            | Self::LastAlignSelf
            | Self::LastReversed
            | Self::StartControl => "XX XX",
        }
    }

    fn reversed(self) -> bool {
        matches!(self, Self::LastReversed)
    }

    fn expected_root_height(self) -> f32 {
        match self {
            Self::LastAlignItems | Self::LastAlignSelf | Self::LastReversed => 22.0,
            Self::FirstControl | Self::StartControl => 20.0,
        }
    }

    fn expected_small_y(self) -> f32 {
        match self {
            Self::FirstControl => 8.0,
            Self::LastAlignItems
            | Self::LastAlignSelf
            | Self::LastReversed
            | Self::StartControl => 0.0,
        }
    }

    fn expected_small_height(self) -> f32 {
        match self {
            Self::FirstControl => 10.0,
            Self::LastAlignItems
            | Self::LastAlignSelf
            | Self::LastReversed
            | Self::StartControl => 20.0,
        }
    }

    fn expected_large_y(self) -> f32 {
        match self {
            Self::LastAlignItems | Self::LastAlignSelf | Self::LastReversed => 2.0,
            Self::FirstControl | Self::StartControl => 0.0,
        }
    }
}

#[test]
fn last_baseline_text_scenes_match_the_css_grid_geometry_and_line_metrics() {
    for (index, scene) in [
        Scene::LastAlignItems,
        Scene::LastAlignSelf,
        Scene::LastReversed,
        Scene::FirstControl,
        Scene::StartControl,
    ]
    .into_iter()
    .enumerate()
    {
        let (snapshot, root, small, large) = fixture(scene, 11_470 + index as u64);
        let request = WorkRequest {
            key: key_for(&snapshot, EnvironmentRevision::INITIAL),
            snapshot,
            viewport: CssViewport {
                width_css_px: 73.0,
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
            .unwrap_or_else(|error| panic!("{}: {error:?}", scene.root_id()));
        let layout = calculation
            .layout
            .unwrap_or_else(|error| panic!("{}: {error:?}", scene.root_id()));
        let frame = |node: HostNodeHandle| {
            layout
                .frames
                .iter()
                .find(|frame| frame.node_id == node.id().get())
                .unwrap_or_else(|| panic!("missing frame for {}", node.id().get()))
        };

        assert_eq!(
            (
                frame(root).x,
                frame(root).y,
                frame(root).width,
                frame(root).height
            ),
            (0.0, 0.0, 73.0, scene.expected_root_height()),
            "{} root",
            scene.root_id()
        );
        assert_eq!(
            (
                frame(small).x,
                frame(small).y,
                frame(small).width,
                frame(small).height
            ),
            (
                0.0,
                scene.expected_small_y(),
                25.0,
                scene.expected_small_height()
            ),
            "{} small item",
            scene.root_id()
        );
        assert_eq!(
            (
                frame(large).x,
                frame(large).y,
                frame(large).width,
                frame(large).height
            ),
            (33.0, scene.expected_large_y(), 40.0, 20.0),
            "{} large item",
            scene.root_id()
        );

        let diagnostics = layout
            .grid_track_sizing_diagnostics
            .as_ref()
            .expect("Grid text fixture diagnostics are attached");
        assert_eq!(diagnostics.item_text_layouts.len(), 2);
        let text_layout = |node: HostNodeHandle| {
            diagnostics
                .item_text_layouts
                .iter()
                .find(|text| text.input.node == node.id())
                .unwrap_or_else(|| panic!("missing text layout for {}", node.id().get()))
        };
        let small_text = text_layout(small);
        let large_text = text_layout(large);
        assert_eq!(small_text.layout.text, scene.small_text());
        assert_eq!(large_text.layout.text, "XX");
        if matches!(scene, Scene::FirstControl) {
            assert_eq!(small_text.layout.lines.len(), 1);
            assert_eq!(small_text.layout.lines[0].baseline, 8.0);
            assert_eq!(frame(small).y + small_text.layout.lines[0].baseline, 16.0);
        } else {
            assert_eq!(small_text.layout.lines.len(), 2, "{}", scene.root_id());
            assert_eq!(small_text.layout.lines[0].baseline, 8.0);
            assert_eq!(small_text.layout.lines[1].line_top, 10.0);
            assert_eq!(small_text.layout.lines[1].baseline, 18.0);
        }
        assert_eq!(large_text.layout.lines.len(), 1);
        assert_eq!(large_text.layout.lines[0].baseline, 16.0);
        if matches!(
            scene,
            Scene::LastAlignItems | Scene::LastAlignSelf | Scene::LastReversed
        ) {
            assert_eq!(frame(small).y + small_text.layout.lines[1].baseline, 18.0);
            assert_eq!(frame(large).y + large_text.layout.lines[0].baseline, 18.0);
        }
    }
}

#[test]
fn multiline_last_baseline_scene_mutation_fails_closed() {
    let (snapshot, _, _, _) = fixture_with_root_style(Scene::LastAlignItems, 11_480, "width:74px");
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 74.0,
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
        .expect("mutated scene reaches admission");
    assert!(calculation.layout.is_err());
}

fn fixture(
    scene: Scene,
    owner: u64,
) -> (
    Arc<HostDocumentSnapshot>,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
) {
    fixture_with_root_style(scene, owner, "width:73px")
}

fn fixture_with_root_style(
    scene: Scene,
    owner: u64,
    width: &str,
) -> (
    Arc<HostDocumentSnapshot>,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
) {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let root_style = format!(
        "box-sizing:border-box;display:grid;{width};height:auto;margin:0;padding:0;border:0;grid-template-columns:25px 40px;grid-template-rows:auto;grid-auto-flow:row;column-gap:8px;row-gap:0;align-items:{};align-content:normal;justify-content:normal;justify-items:normal;direction:ltr;writing-mode:horizontal-tb;background-color:#eef3f8",
        scene.align_items()
    );
    let small_column = "1 / 2";
    let large_column = "2 / 3";
    let child_style = |font_size: u8, align_self: &str, grid_column: &str| {
        format!(
            "box-sizing:border-box;display:block;position:static;width:auto;height:auto;min-width:auto;max-width:none;min-height:auto;max-height:none;margin:0;padding:0;border:0;grid-column:{grid_column};grid-row:1 / 2;align-self:{align_self};justify-self:auto;font-family:Ahem;font-size:{font_size}px;font-weight:400;font-style:normal;line-height:{font_size}px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;writing-mode:horizontal-tb;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d;color:#e5484d"
        )
    };
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let small = document.reserve_node_handle().unwrap();
    let small_text = document.reserve_node_handle().unwrap();
    let large = document.reserve_node_handle().unwrap();
    let large_text = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    let mut elements = vec![
        (root, scene.root_id(), HostParent::Root, root_style),
        (
            small,
            "c114-baseline-small",
            HostParent::Node(root),
            child_style(10, scene.align_self(), small_column),
        ),
        (
            large,
            "c114-baseline-large",
            HostParent::Node(root),
            child_style(20, scene.align_self(), large_column),
        ),
    ];
    if scene.reversed() {
        elements.swap(1, 2);
    }
    for (node, id, parent, style) in elements {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    batch.push(DocumentOperation::CreateText {
        node: small_text,
        data: scene.small_text().into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(small),
        node: small_text,
        before: None,
    });
    batch.push(DocumentOperation::CreateText {
        node: large_text,
        data: "XX".into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(large),
        node: large_text,
        before: None,
    });
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), root, small, large)
}

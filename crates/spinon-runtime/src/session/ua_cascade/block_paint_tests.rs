use super::*;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_render::{OpaqueCssSrgb, RuntimePaint};

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn block_request(root_style: &str, child_styles: &[&str]) -> (WorkRequest, Vec<HostNodeHandle>) {
    let child_specs = child_styles
        .iter()
        .map(|style| (*style, 0))
        .collect::<Vec<_>>();
    block_request_with_parents(root_style, &child_specs)
}

fn block_request_with_parents(
    root_style: &str,
    child_styles: &[(&str, usize)],
) -> (WorkRequest, Vec<HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let owner = OwnerId::new(41).unwrap();
    let nodes = (0..=child_styles.len())
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    for (index, node) in nodes.iter().enumerate() {
        let style = if index == 0 {
            root_style
        } else {
            child_styles[index - 1].0
        };
        batch.push(DocumentOperation::CreateElement {
            node: *node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: *node,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: if index == 0 {
                HostParent::Root
            } else {
                HostParent::Node(nodes[child_styles[index - 1].1])
            },
            node: *node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let snapshot = Arc::new(document.snapshot());
    let key = key_for(&snapshot, EnvironmentRevision::INITIAL);
    (
        WorkRequest {
            key,
            snapshot,
            viewport: CssViewport {
                width_css_px: 320.0,
                height_css_px: 240.0,
                device_scale_factor: 1.0,
                environment_revision: EnvironmentRevision::INITIAL,
                media_environment: CssMediaEnvironment::MOBILE,
            },
            previous_snapshot: None,
            force_full: true,
            invalidation: None,
            previous_styles: None,
        },
        nodes,
    )
}

pub(super) fn block_request_with_author_stylesheet(css: &str) -> (WorkRequest, HostNodeHandle) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let style = document.reserve_node_handle().unwrap();
    let stylesheet_text = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let owner = OwnerId::new(42).unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "style").unwrap(),
        value: "width:280px;background-color:#101827".to_owned().into(),
    });
    batch.push(DocumentOperation::CreateElement {
        node: style,
        namespace: HTML.to_owned(),
        local_name: "style".to_owned(),
    });
    batch.push(DocumentOperation::CreateText {
        node: stylesheet_text,
        data: css.to_owned().into(),
    });
    batch.push(DocumentOperation::CreateElement {
        node: child,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node: child,
        name: AttributeName::new(None, "class").unwrap(),
        value: "painted".to_owned().into(),
    });
    for (parent, node) in [
        (HostParent::Root, root),
        (HostParent::Node(root), style),
        (HostParent::Node(style), stylesheet_text),
        (HostParent::Node(root), child),
    ] {
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let snapshot = Arc::new(document.snapshot());
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 240.0,
        device_scale_factor: 1.0,
        environment_revision: EnvironmentRevision::INITIAL,
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let key = key_for(&snapshot, EnvironmentRevision::INITIAL);
    (
        WorkRequest {
            key,
            snapshot,
            viewport,
            previous_snapshot: None,
            force_full: true,
            invalidation: None,
            previous_styles: None,
        },
        child,
    )
}

fn block_request_with_text(hidden: bool) -> WorkRequest {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let owner = OwnerId::new(43).unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: root,
        namespace: HTML.to_owned(),
        local_name: "div".to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node: root,
        name: AttributeName::new(None, "style").unwrap(),
        value: if hidden {
            "display:none;width:280px;height:86px;background-color:#101827"
        } else {
            "width:280px;height:86px;background-color:#101827"
        }
        .to_owned()
        .into(),
    });
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: "visible text".into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Root,
        node: root,
        before: None,
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(root),
        node: text,
        before: None,
    });
    document.commit(batch).unwrap();
    let snapshot = Arc::new(document.snapshot());
    let key = key_for(&snapshot, EnvironmentRevision::INITIAL);
    WorkRequest {
        key,
        snapshot,
        viewport: CssViewport {
            width_css_px: 320.0,
            height_css_px: 240.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    }
}

#[test]
fn block_profile_matches_chromium_flow_frames_and_paint_order() {
    let (request, nodes) = block_request_with_parents(
        "box-sizing:border-box;width:280px;background-color:#101827",
        &[
            (
                "display:block;height:32px;background-color:#3366ff;color:#f9fafb;font-size:20px;font-family:Arial,sans-serif",
                0,
            ),
            ("display:block;height:10px;background-color:#60a5fa", 1),
            ("display:block;background-color:transparent", 0),
            ("display:block;height:20px;background-color:#e11d48", 3),
            (
                "display:block;width:120px;height:18px;background-color:#f97316",
                3,
            ),
            ("display:none;height:40px;background-color:#ff0000", 0),
            ("display:block;height:14px;background-color:#00ff00", 6),
            ("display:block;height:16px;background-color:#22c55e", 0),
        ],
    );
    let calculation = compute_request_for_block_paint(&request).unwrap();
    let layout = calculation.layout.unwrap();
    let frames = layout
        .frames
        .iter()
        .map(|frame| (frame.node_id, (frame.x, frame.y, frame.width, frame.height)))
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(frames[&nodes[0].id().get()], (0.0, 0.0, 280.0, 86.0));
    assert_eq!(frames[&nodes[1].id().get()], (0.0, 0.0, 280.0, 32.0));
    assert_eq!(frames[&nodes[2].id().get()], (0.0, 0.0, 280.0, 10.0));
    assert_eq!(frames[&nodes[3].id().get()], (0.0, 32.0, 280.0, 38.0));
    assert_eq!(frames[&nodes[4].id().get()], (0.0, 32.0, 280.0, 20.0));
    assert_eq!(frames[&nodes[5].id().get()], (0.0, 52.0, 120.0, 18.0));
    assert_eq!(frames[&nodes[6].id().get()], (0.0, 0.0, 0.0, 0.0));
    assert_eq!(frames[&nodes[7].id().get()], (0.0, 0.0, 0.0, 0.0));
    assert_eq!(frames[&nodes[8].id().get()], (0.0, 70.0, 280.0, 16.0));

    let scene = layout.render_snapshot.unwrap();
    assert_eq!(scene.boxes().len(), 7);
    assert_eq!(
        scene
            .boxes()
            .iter()
            .map(|item| item.node_id())
            .collect::<Vec<_>>(),
        nodes[..6]
            .iter()
            .map(|node| node.id())
            .chain(std::iter::once(nodes[8].id()))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        scene.boxes()[0].paint(),
        RuntimePaint::Opaque(OpaqueCssSrgb::new(16, 24, 39))
    );
    assert_eq!(
        scene.boxes()[6].paint(),
        RuntimePaint::Opaque(OpaqueCssSrgb::new(34, 197, 94))
    );
    assert_eq!(scene.boxes()[3].paint(), RuntimePaint::None);
    assert_eq!(scene.boxes()[6].paint_order(), 6);

    let reference: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/css/references/c08-block-flow-v1.json"
    )))
    .unwrap();
    let expected = reference["observations"][0]["nodes"].as_array().unwrap();
    let actual_styles = &calculation.roots[0].styles.elements;
    let expected_ids = [
        "root",
        "hero",
        "hero-child",
        "group",
        "first",
        "second",
        "hidden",
        "hidden-child",
        "last",
    ];
    assert_eq!(expected.len(), expected_ids.len());
    for (index, (id, reference_node)) in expected_ids.iter().zip(expected).enumerate() {
        assert_eq!(reference_node["id"], *id);
        let actual = actual_styles
            .iter()
            .find(|style| style.node_id == nodes[index].id())
            .unwrap();
        for property in ["display", "background-color", "color", "font-size"] {
            assert_eq!(
                actual.properties[property],
                reference_node["properties"][property].as_str().unwrap(),
                "{id}.{property}"
            );
        }
        let expected_family = reference_node["properties"]["font-family"]
            .as_str()
            .unwrap();
        if index == 1 || index == 2 {
            assert_eq!(
                actual.properties["font-family"], expected_family,
                "{id}.font-family"
            );
        } else {
            assert!(
                !actual.properties["font-family"].is_empty(),
                "{id}.font-family"
            );
        }
        let rect = &reference_node["rect"];
        let frame = frames[&nodes[index].id().get()];
        for (field, actual) in [
            ("x", frame.0),
            ("y", frame.1),
            ("width", frame.2),
            ("height", frame.3),
        ] {
            let expected = rect[field].as_f64().unwrap() as f32;
            assert!(
                (actual - expected).abs() <= 0.5,
                "{id}.{field}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn block_profile_preserves_inherited_foreground_and_font_values() {
    let (request, nodes) = block_request(
        "width:280px;color:#f9fafb;font-size:20px;font-family:Arial,sans-serif",
        &["height:32px", "height:10px"],
    );
    let calculation = compute_request_for_block_paint(&request).unwrap();
    let styles = &calculation.roots[0].styles.elements;
    let hero = styles
        .iter()
        .find(|style| style.node_id == nodes[1].id())
        .unwrap();
    let child = styles
        .iter()
        .find(|style| style.node_id == nodes[2].id())
        .unwrap();
    assert_eq!(hero.properties["color"], "rgb(249, 250, 251)");
    assert_eq!(child.properties["color"], hero.properties["color"]);
    assert_eq!(child.properties["font-size"], "20px");
    assert_eq!(
        child.properties["font-family"],
        hero.properties["font-family"]
    );
}

#[test]
fn block_profile_accepts_hidden_root_and_omits_its_scene() {
    let (request, _) = block_request(
        "display:none;width:280px;height:86px;background-color:#101827",
        &["display:block;height:20px;background-color:#3366ff"],
    );
    let calculation = compute_request_for_block_paint(&request).unwrap();
    let layout = calculation.layout.unwrap();
    assert!(
        layout
            .frames
            .iter()
            .all(|frame| frame.width == 0.0 && frame.height == 0.0)
    );
    assert!(layout.render_snapshot.unwrap().boxes().is_empty());
}

#[test]
fn block_profile_rejects_flex_display_and_unsupported_inline_properties() {
    for display in ["flex", "flow-root"] {
        let (request, _) = block_request(&format!("display:{display};width:280px"), &[]);
        let failure = compute_request_for_block_paint(&request)
            .unwrap()
            .layout
            .unwrap_err();
        assert_eq!(failure.code, "unsupported_block_display", "{display}");
    }

    let (request, _) = block_request("width:280px;border:1px solid red", &[]);
    let failure = compute_request_for_block_paint(&request)
        .unwrap()
        .layout
        .unwrap_err();
    assert_eq!(failure.code, "unsupported_inline_property");
    assert!(failure.property.unwrap().starts_with("border"));
}

#[test]
fn block_profile_rejects_partial_alpha_without_publishing_a_scene() {
    let (request, _) = block_request("width:280px;background-color:rgba(1,2,3,.5)", &[]);
    let error = compute_request_for_block_paint(&request).unwrap_err();
    assert!(error.contains("alpha가 0 또는 1이 아닙니다"), "{error}");
}

#[test]
fn block_profile_cascades_supported_author_stylesheet_rules_into_the_scene() {
    let (request, child) =
        block_request_with_author_stylesheet(".painted { height:16px; background-color:#22c55e; }");
    let calculation = compute_request_for_block_paint(&request).unwrap();
    let layout = calculation.layout.unwrap();
    let child_frame = layout
        .frames
        .iter()
        .find(|frame| frame.node_id == child.id().get())
        .unwrap();
    assert_eq!((child_frame.width, child_frame.height), (280.0, 16.0));
    let scene = layout.render_snapshot.unwrap();
    let child_box = scene
        .boxes()
        .iter()
        .find(|item| item.node_id() == child.id())
        .unwrap();
    assert_eq!(
        child_box.paint(),
        RuntimePaint::Opaque(OpaqueCssSrgb::new(34, 197, 94))
    );
}

#[test]
fn block_profile_rejects_unsupported_author_stylesheet_properties() {
    let (request, _) = block_request_with_author_stylesheet(".painted { border:1px solid red; }");
    let error = compute_request_for_block_paint(&request).unwrap_err();
    assert!(error.contains("지원하지 않는 CSS 선언"), "{error}");
}

#[test]
fn block_profile_does_not_publish_a_scene_after_stylesheet_parse_diagnostics() {
    let (request, _) = block_request_with_author_stylesheet(
        ".painted { height:16px; background-color:not-a-color; }",
    );
    let calculation = compute_request_for_block_paint(&request).unwrap();
    let failure = calculation.layout.unwrap_err();
    assert_eq!(failure.code, "cascade_diagnostics");
}

#[test]
fn block_profile_rejects_visible_text_and_omits_hidden_text() {
    let visible = compute_request_for_block_paint(&block_request_with_text(false)).unwrap();
    assert!(visible.layout.is_err());

    let hidden = compute_request_for_block_paint(&block_request_with_text(true)).unwrap();
    let layout = hidden.layout.unwrap();
    assert!(layout.frames.iter().all(|frame| frame.width == 0.0));
    assert!(layout.render_snapshot.unwrap().boxes().is_empty());
}

#[test]
fn block_profile_omits_zero_area_boxes_without_turning_layout_into_an_error() {
    let (request, _) = block_request("width:0;height:16px;background-color:#22c55e", &[]);
    let calculation = compute_request_for_block_paint(&request).unwrap();
    let layout = calculation.layout.unwrap();
    assert_eq!(layout.frames[0].width, 0.0);
    assert_eq!(layout.frames[0].height, 16.0);
    assert!(layout.render_snapshot.unwrap().boxes().is_empty());
}

#[test]
fn block_profile_rejects_a_stale_requested_style_revision() {
    let (mut request, _) = block_request("width:280px;height:16px", &[]);
    request.key.style_revision = StyleRevision::INITIAL.get() + 1;
    let error = compute_request_for_block_paint(&request).unwrap_err();
    assert!(error.contains("revision"), "{error}");
}

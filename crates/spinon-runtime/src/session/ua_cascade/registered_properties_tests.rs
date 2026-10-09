use super::author_stylesheets::collect_runtime_author_stylesheets;
use super::*;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_render::{OpaqueCssSrgb, RuntimePaint};
use spinon_style::{ComputedBackgroundPaint, ComputedStyleProfile};
use std::collections::HashMap;

const HTML: &str = "http://www.w3.org/1999/xhtml";
const FIRST_SHEET: &str = r#"
@property --tile-width { syntax:"<length>"; inherits:false; initial-value:23px; }
@property --tone { syntax:"<length>"; inherits:true; initial-value:7px; }
@property --tile-color { syntax:"<color>"; inherits:false; initial-value:rgb(51,102,255); }
@property --unknown-descriptor { syntax:"<length>"; inherits:false; initial-value:13px; vendor-extension:ignored; }
#app { display:flex; box-sizing:border-box; width:301px; height:100px; align-items:flex-start; gap:5px; background-color:#123456; }
.tile { display:block; box-sizing:border-box; height:14px; flex-shrink:0; background-color:var(--tile-color); }
#declared { --tile-width:41px; width:var(--tile-width); }
#default { width:var(--tile-width); }
#invalid { --tile-width:red; width:var(--tile-width); }
#inherit-parent { display:flex; flex-direction:column; gap:2px; width:60px; height:30px; flex-shrink:0; --tile-width:71px; --tone:19px; }
#not-inherited { width:var(--tile-width); }
#inherited { width:var(--tone); }
#late { width:var(--late-width); }
@property --late-width { syntax:"<length>"; inherits:false; initial-value:31px; }
#unknown { width:var(--unknown-descriptor); }
#inline-priority { --tile-width:39px; width:var(--tile-width); }
#color-override { --tile-color:#ff6600; width:11px; }
@property --duplicate { syntax:"<length>"; inherits:false; initial-value:17px; }
"#;
const SECOND_SHEET: &str = r#"
@property --duplicate { syntax:"<number>"; inherits:false; initial-value:3; }
#duplicate { --duplicate:29px; width:var(--duplicate); }
"#;

fn append_element(
    document: &mut HostDocument,
    batch: &mut DocumentChangeBatch,
    parent: HostParent,
    id: &str,
    class: Option<&str>,
    inline_style: Option<&str>,
    tag: &str,
) -> HostNodeHandle {
    let node = document.reserve_node_handle().unwrap();
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: tag.to_owned(),
    });
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, "id").unwrap(),
        value: id.to_owned().into(),
    });
    if let Some(class) = class {
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "class").unwrap(),
            value: class.to_owned().into(),
        });
    }
    if let Some(inline_style) = inline_style {
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: inline_style.to_owned().into(),
        });
    }
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node,
        before: None,
    });
    node
}

fn append_style(
    document: &mut HostDocument,
    batch: &mut DocumentChangeBatch,
    parent: HostNodeHandle,
    id: &str,
    css: &str,
) -> (HostNodeHandle, HostNodeHandle) {
    let style = append_element(
        document,
        batch,
        HostParent::Node(parent),
        id,
        None,
        None,
        "style",
    );
    let text = document.reserve_node_handle().unwrap();
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: css.to_owned().into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(style),
        node: text,
        before: None,
    });
    (style, text)
}

fn make_document() -> (HostDocument, HashMap<&'static str, HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(9052).unwrap(), document.document_revision());
    let app = append_element(
        &mut document,
        &mut batch,
        HostParent::Root,
        "app",
        None,
        None,
        "div",
    );
    let mut nodes = HashMap::from([("app", app)]);
    let (first_style, first_text) =
        append_style(&mut document, &mut batch, app, "sheet-one", FIRST_SHEET);
    nodes.insert("sheet-one", first_style);
    nodes.insert("sheet-one-text", first_text);

    for id in ["declared", "default", "invalid"] {
        let node = append_element(
            &mut document,
            &mut batch,
            HostParent::Node(app),
            id,
            Some("tile"),
            None,
            "div",
        );
        nodes.insert(id, node);
    }
    let parent = append_element(
        &mut document,
        &mut batch,
        HostParent::Node(app),
        "inherit-parent",
        None,
        None,
        "div",
    );
    nodes.insert("inherit-parent", parent);
    for id in ["not-inherited", "inherited"] {
        let node = append_element(
            &mut document,
            &mut batch,
            HostParent::Node(parent),
            id,
            Some("tile"),
            None,
            "div",
        );
        nodes.insert(id, node);
    }
    for id in ["late", "unknown"] {
        let node = append_element(
            &mut document,
            &mut batch,
            HostParent::Node(app),
            id,
            Some("tile"),
            None,
            "div",
        );
        nodes.insert(id, node);
    }
    let inline = append_element(
        &mut document,
        &mut batch,
        HostParent::Node(app),
        "inline-priority",
        Some("tile"),
        Some("--tile-width:47px !important"),
        "div",
    );
    nodes.insert("inline-priority", inline);
    for id in ["color-override", "duplicate"] {
        let node = append_element(
            &mut document,
            &mut batch,
            HostParent::Node(app),
            id,
            Some("tile"),
            None,
            "div",
        );
        nodes.insert(id, node);
    }
    let (second_style, second_text) =
        append_style(&mut document, &mut batch, app, "sheet-two", SECOND_SHEET);
    nodes.insert("sheet-two", second_style);
    nodes.insert("sheet-two-text", second_text);
    document.commit(batch).unwrap();
    (document, nodes)
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 301.0,
        height_css_px: 100.0,
        device_scale_factor: 1.0,
        environment_revision: EnvironmentRevision::INITIAL,
        media_environment: CssMediaEnvironment::MOBILE,
    }
}

fn request(document: &HostDocument) -> WorkRequest {
    let viewport = viewport();
    let snapshot = Arc::new(document.snapshot());
    WorkRequest {
        key: key_for(&snapshot, viewport.environment_revision),
        snapshot,
        viewport,
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    }
}

#[test]
fn c0410_runtime_gpu_profile_rejects_registered_properties() {
    let (document, _) = make_document();
    assert!(compute_request_for_runtime_gpu(&request(&document)).is_err());
}

#[test]
fn registered_property_in_one_host_root_applies_to_every_root_cascade() {
    let mut document = HostDocument::new().unwrap();
    let first_root = document.reserve_node_handle().unwrap();
    let style = document.reserve_node_handle().unwrap();
    let style_text = document.reserve_node_handle().unwrap();
    let second_root = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(9053).unwrap(), document.document_revision());
    for (node, name, id) in [
        (first_root, "main", "first-root"),
        (style, "style", ""),
        (second_root, "div", "shared-target"),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: "http://www.w3.org/1999/xhtml".to_owned(),
            local_name: name.to_owned(),
        });
        if !id.is_empty() {
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "id").unwrap(),
                value: id.to_owned().into(),
            });
        }
    }
    batch.push(DocumentOperation::CreateText {
        node: style_text,
        data: r#"@property --cross-root-size { syntax: "<length>"; inherits: false; initial-value: 19px; } #shared-target { display:block; width:var(--cross-root-size); height:14px; }"#.to_owned().into(),
    });
    for (parent, node) in [
        (HostParent::Root, first_root),
        (HostParent::Node(first_root), style),
        (HostParent::Node(style), style_text),
        (HostParent::Root, second_root),
    ] {
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();

    let calculation = compute_request_for_registered_properties_gpu(&request(&document)).unwrap();
    assert_eq!(calculation.roots.len(), 2);
    let second_root_styles = &calculation.roots[1].styles;
    let target = second_root_styles
        .elements
        .iter()
        .find(|style| style.node_id == second_root.id())
        .unwrap();
    assert_eq!(target.properties["width"], "19px");
    assert_eq!(calculation.layout.unwrap_err().code, "multiple_host_roots");
}

#[test]
fn host_document_styles_feed_registered_properties_into_taffy_and_wgpu() {
    let (document, nodes) = make_document();
    let sources = collect_runtime_author_stylesheets(&document.snapshot()).unwrap();
    assert_eq!(sources.len(), 2);
    assert!(sources[0].css.contains("@property --tile-width"));
    assert!(sources[1].css.contains("@property --duplicate"));

    let calculation = compute_request_for_registered_properties_gpu(&request(&document)).unwrap();
    let styles = &calculation.roots[0].styles;
    assert_eq!(
        styles.profile,
        ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
    );
    let style_for = |name: &str| {
        styles
            .elements
            .iter()
            .find(|style| style.node_id == nodes[name].id())
            .unwrap()
    };
    assert_eq!(style_for("declared").properties["width"], "41px");
    assert_eq!(style_for("declared").properties["flex-shrink"], "0");
    assert_eq!(style_for("default").properties["width"], "23px");
    assert_eq!(style_for("invalid").properties["width"], "23px");
    assert_eq!(style_for("not-inherited").properties["width"], "23px");
    assert_eq!(style_for("inherit-parent").properties["flex-shrink"], "0");
    assert_eq!(style_for("inherited").properties["width"], "19px");
    assert_eq!(style_for("late").properties["width"], "31px");
    assert_eq!(style_for("unknown").properties["width"], "13px");
    assert_eq!(style_for("inline-priority").properties["width"], "47px");
    assert_eq!(style_for("color-override").properties["width"], "11px");
    assert_eq!(
        style_for("color-override").background_paint,
        Some(ComputedBackgroundPaint::Opaque(
            spinon_style::OpaqueCssSrgb {
                red: 255,
                green: 102,
                blue: 0,
            }
        ))
    );
    assert_eq!(style_for("duplicate").properties["width"], "auto");
    assert!(styles.diagnostics.is_empty());

    let layout = calculation.layout.unwrap();
    let frame_for = |name: &str| {
        layout
            .frames
            .iter()
            .find(|frame| frame.node_id == nodes[name].id().get())
            .unwrap()
    };
    assert_eq!(frame_for("declared").width, 41.0);
    assert_eq!(frame_for("default").width, 23.0);
    assert_eq!(frame_for("invalid").width, 23.0);
    assert_eq!(frame_for("not-inherited").width, 23.0);
    assert_eq!(frame_for("inherited").width, 19.0);
    assert_eq!(frame_for("late").width, 31.0);
    assert_eq!(frame_for("unknown").width, 13.0);
    assert_eq!(frame_for("inline-priority").width, 47.0);
    assert_eq!(frame_for("sheet-one").width, 0.0);
    assert_eq!(frame_for("sheet-two").height, 0.0);
    for (name, expected) in [
        ("app", (0.0, 0.0, 301.0, 100.0)),
        ("sheet-one", (0.0, 0.0, 0.0, 0.0)),
        ("declared", (0.0, 0.0, 41.0, 14.0)),
        ("default", (46.0, 0.0, 23.0, 14.0)),
        ("invalid", (74.0, 0.0, 23.0, 14.0)),
        ("inherit-parent", (102.0, 0.0, 60.0, 30.0)),
        ("not-inherited", (102.0, 0.0, 23.0, 14.0)),
        ("inherited", (102.0, 16.0, 19.0, 14.0)),
        ("late", (167.0, 0.0, 31.0, 14.0)),
        ("unknown", (203.0, 0.0, 13.0, 14.0)),
        ("inline-priority", (221.0, 0.0, 47.0, 14.0)),
        ("color-override", (273.0, 0.0, 11.0, 14.0)),
        ("duplicate", (289.0, 0.0, 0.0, 14.0)),
        ("sheet-two", (0.0, 0.0, 0.0, 0.0)),
    ] {
        let frame = frame_for(name);
        for (actual, expected) in [
            (frame.x, expected.0),
            (frame.y, expected.1),
            (frame.width, expected.2),
            (frame.height, expected.3),
        ] {
            assert!(
                (actual - expected).abs() <= 0.5,
                "{name} CSS px frame differs from Chromium: actual={frame:?}, expected={expected:?}"
            );
        }
    }

    let scene = layout.render_snapshot.unwrap();
    let color_node = scene
        .boxes()
        .iter()
        .find(|item| item.node_id() == nodes["color-override"].id())
        .unwrap();
    assert_eq!(
        color_node.paint(),
        RuntimePaint::Opaque(OpaqueCssSrgb::new(255, 102, 0))
    );
}

#[test]
fn registration_rebuilds_after_style_text_update_move_and_detach() {
    let (mut document, nodes) = make_document();
    let property_width = |document: &HostDocument, node: HostNodeHandle| {
        compute_request_for_registered_properties_gpu(&request(document))
            .unwrap()
            .roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.node_id == node.id())
            .unwrap()
            .properties["width"]
            .clone()
    };
    let duplicate_width = |document: &HostDocument| property_width(document, nodes["duplicate"]);
    let duplicate_frame = |document: &HostDocument| {
        let calculation =
            compute_request_for_registered_properties_gpu(&request(document)).unwrap();
        let layout = calculation.layout.unwrap();
        let frame = layout
            .frames
            .iter()
            .find(|frame| frame.node_id == nodes["duplicate"].id().get())
            .unwrap();
        [frame.x, frame.y, frame.width, frame.height]
    };
    assert_eq!(property_width(&document, nodes["default"]), "23px");
    assert_eq!(duplicate_width(&document), "auto");

    let owner = OwnerId::new(9052).unwrap();
    let updated_css = FIRST_SHEET.replace("initial-value:23px", "initial-value:37px");
    let mut update = DocumentChangeBatch::new(owner, document.document_revision());
    update.push(DocumentOperation::SetTextData {
        node: nodes["sheet-one-text"],
        data: updated_css.into(),
    });
    document.commit(update).unwrap();
    assert_eq!(property_width(&document, nodes["default"]), "37px");

    let mut move_second = DocumentChangeBatch::new(owner, document.document_revision());
    move_second.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(nodes["app"]),
        node: nodes["sheet-two"],
        before: Some(nodes["sheet-one"]),
    });
    document.commit(move_second).unwrap();
    let moved_sources = collect_runtime_author_stylesheets(&document.snapshot()).unwrap();
    assert_eq!(moved_sources.len(), 2);
    assert!(moved_sources[0].css.contains("syntax:\"<number>\""));
    assert!(moved_sources[1].css.contains("syntax:\"<length>\""));
    assert_eq!(duplicate_width(&document), "29px");
    for (actual, expected) in duplicate_frame(&document)
        .into_iter()
        .zip([317.0, 0.0, 29.0, 14.0])
    {
        assert!(
            (actual - expected).abs() <= 0.5,
            "이동 뒤 Chrome 프레임과 다릅니다: actual={actual}, expected={expected}"
        );
    }

    let mut detach = DocumentChangeBatch::new(owner, document.document_revision());
    detach.push(DocumentOperation::RemoveChild {
        parent: HostParent::Node(nodes["app"]),
        node: nodes["sheet-one"],
    });
    document.commit(detach).unwrap();
    let detached_sources = collect_runtime_author_stylesheets(&document.snapshot()).unwrap();
    assert_eq!(detached_sources.len(), 1);
    assert!(detached_sources[0].css.contains("syntax:\"<number>\""));
    assert_eq!(property_width(&document, nodes["default"]), "auto");
    assert_eq!(duplicate_width(&document), "auto");
    for (actual, expected) in duplicate_frame(&document)
        .into_iter()
        .zip([0.0, 0.0, 301.0, 0.0])
    {
        assert!(
            (actual - expected).abs() <= 0.5,
            "분리 뒤 Chrome 프레임과 다릅니다: actual={actual}, expected={expected}"
        );
    }

    let mut reinsert = DocumentChangeBatch::new(owner, document.document_revision());
    reinsert.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(nodes["app"]),
        node: nodes["sheet-one"],
        before: None,
    });
    document.commit(reinsert).unwrap();
    let reinserted_sources = collect_runtime_author_stylesheets(&document.snapshot()).unwrap();
    assert_eq!(reinserted_sources.len(), 2);
    assert!(reinserted_sources[0].css.contains("syntax:\"<number>\""));
    assert!(reinserted_sources[1].css.contains("syntax:\"<length>\""));
    assert_eq!(property_width(&document, nodes["default"]), "37px");
    assert_eq!(duplicate_width(&document), "29px");
    for (actual, expected) in duplicate_frame(&document)
        .into_iter()
        .zip([317.0, 0.0, 29.0, 14.0])
    {
        assert!(
            (actual - expected).abs() <= 0.5,
            "재삽입 뒤 Chrome 프레임과 다릅니다: actual={actual}, expected={expected}"
        );
    }
}

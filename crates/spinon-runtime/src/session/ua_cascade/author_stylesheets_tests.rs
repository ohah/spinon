use super::author_stylesheets::collect_runtime_author_stylesheets;
use super::*;
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_render::RuntimePaint;
use spinon_style::{ComputedBackgroundPaint, CssOrigin, OpaqueCssSrgb};

const HTML: &str = "http://www.w3.org/1999/xhtml";
const SVG: &str = "http://www.w3.org/2000/svg";
type StylesheetFixture<'a> = (&'a str, &'a [(&'a str, &'a str)], &'a str);

fn create_element(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    namespace: &str,
    name: &str,
    attributes: &[(&str, &str)],
) {
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: namespace.to_owned(),
        local_name: name.to_owned(),
    });
    for (name, value) in attributes {
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, *name).unwrap(),
            value: (*value).into(),
        });
    }
}

fn append_text(
    document: &mut HostDocument,
    batch: &mut DocumentChangeBatch,
    parent: HostNodeHandle,
    text: impl Into<spinon_core::DomString>,
) -> HostNodeHandle {
    let text_node = document.reserve_node_handle().unwrap();
    batch.push(DocumentOperation::CreateText {
        node: text_node,
        data: text.into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(parent),
        node: text_node,
        before: None,
    });
    text_node
}

fn append_element(batch: &mut DocumentChangeBatch, parent: HostParent, node: HostNodeHandle) {
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node,
        before: None,
    });
}

fn make_document_with_stylesheets(
    stylesheets: &[StylesheetFixture<'_>],
) -> (HostDocument, HostNodeHandle, Vec<HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut sheet_handles = Vec::new();
    for _ in stylesheets {
        sheet_handles.push(document.reserve_node_handle().unwrap());
    }
    let owner = OwnerId::new(9041).unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    create_element(&mut batch, root, HTML, "main", &[]);
    append_element(&mut batch, HostParent::Root, root);
    for ((tag, attributes, css), sheet) in stylesheets.iter().zip(&sheet_handles) {
        create_element(&mut batch, *sheet, HTML, tag, attributes);
        append_element(&mut batch, HostParent::Node(root), *sheet);
        append_text(&mut document, &mut batch, *sheet, *css);
    }
    document.commit(batch).unwrap();
    (document, root, sheet_handles)
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

fn request(document: &HostDocument, viewport: CssViewport) -> WorkRequest {
    let snapshot = Arc::new(document.snapshot());
    WorkRequest {
        key: key_for(&snapshot, viewport.environment_revision),
        snapshot,
        viewport,
    }
}

#[test]
fn collector_keeps_connected_dom_order_and_ignores_non_css_style_data_blocks() {
    let (document, _, sheets) = make_document_with_stylesheets(&[
        (
            "style",
            &[("type", "text/css; charset=utf-8"), ("media", "SCREEN")],
            ".a { width: 1px; }",
        ),
        (
            "style",
            &[("type", "application/json")],
            "{\"not\":\"css\"}",
        ),
        ("style", &[("media", "all")], ".b { height: 2px; }"),
    ]);
    let snapshot = document.snapshot();
    let sources = collect_runtime_author_stylesheets(&snapshot).unwrap();

    assert_eq!(sources.len(), 2);
    assert!(sources[0].css.contains(".a"));
    assert!(sources[1].css.contains(".b"));
    assert_eq!(sources[0].origin, CssOrigin::Author);
    assert_eq!(
        sources[0].id,
        format!(
            "host-style:{}:{}",
            snapshot.generation().get(),
            sheets[0].id().get()
        )
    );
}

#[test]
fn collector_rejects_unsupported_media_svg_style_and_external_stylesheet_links() {
    let (media_document, _, _) = make_document_with_stylesheets(&[(
        "style",
        &[("media", "print")],
        "div { display:block; }",
    )]);
    assert!(
        collect_runtime_author_stylesheets(&media_document.snapshot())
            .unwrap_err()
            .to_string()
            .contains("media")
    );

    let mut svg_document = HostDocument::new().unwrap();
    let root = svg_document.reserve_node_handle().unwrap();
    let style = svg_document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(9042).unwrap(),
        svg_document.document_revision(),
    );
    create_element(&mut batch, root, HTML, "main", &[]);
    create_element(&mut batch, style, SVG, "style", &[]);
    append_element(&mut batch, HostParent::Root, root);
    append_element(&mut batch, HostParent::Node(root), style);
    append_text(&mut svg_document, &mut batch, style, "rect { width: 1px; }");
    svg_document.commit(batch).unwrap();
    assert!(
        collect_runtime_author_stylesheets(&svg_document.snapshot())
            .unwrap_err()
            .to_string()
            .contains("namespace")
    );

    let mut link_document = HostDocument::new().unwrap();
    let root = link_document.reserve_node_handle().unwrap();
    let link = link_document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(9043).unwrap(),
        link_document.document_revision(),
    );
    create_element(&mut batch, root, HTML, "main", &[]);
    create_element(
        &mut batch,
        link,
        HTML,
        "link",
        &[
            ("rel", "preload stylesheet"),
            ("href", "https://example.invalid/app.css"),
        ],
    );
    append_element(&mut batch, HostParent::Root, root);
    append_element(&mut batch, HostParent::Node(root), link);
    link_document.commit(batch).unwrap();
    assert!(
        collect_runtime_author_stylesheets(&link_document.snapshot())
            .unwrap_err()
            .to_string()
            .contains("외부 stylesheet")
    );
}

#[test]
fn collector_rejects_unpaired_utf16_in_css_text() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let style = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(9044).unwrap(), document.document_revision());
    create_element(&mut batch, root, HTML, "main", &[]);
    create_element(&mut batch, style, HTML, "style", &[]);
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: spinon_core::DomString::from_utf16(vec![0xD800]),
    });
    append_element(&mut batch, HostParent::Root, root);
    append_element(&mut batch, HostParent::Node(root), style);
    append_element(&mut batch, HostParent::Node(style), text);
    document.commit(batch).unwrap();

    assert!(
        collect_runtime_author_stylesheets(&document.snapshot())
            .unwrap_err()
            .to_string()
            .contains("UTF-16")
    );
}

fn make_cascade_fixture(
    invalid_css: Option<&str>,
    visible_text: bool,
) -> (
    HostDocument,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
    HostNodeHandle,
) {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let first_sheet = document.reserve_node_handle().unwrap();
    let first = document.reserve_node_handle().unwrap();
    let second_sheet = document.reserve_node_handle().unwrap();
    let second = document.reserve_node_handle().unwrap();
    let first_text = document.reserve_node_handle().unwrap();
    let second_text = document.reserve_node_handle().unwrap();
    let visible = visible_text.then(|| document.reserve_node_handle().unwrap());
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(9045).unwrap(), document.document_revision());
    create_element(&mut batch, root, HTML, "div", &[("id", "app")]);
    create_element(
        &mut batch,
        first_sheet,
        HTML,
        "style",
        &[("id", "sheet-one"), ("media", "screen")],
    );
    create_element(
        &mut batch,
        second_sheet,
        HTML,
        "style",
        &[("id", "sheet-two")],
    );
    create_element(
        &mut batch,
        first,
        HTML,
        "div",
        &[
            ("id", "first"),
            ("class", "tile first"),
            ("style", "width:47px !important"),
        ],
    );
    create_element(
        &mut batch,
        second,
        HTML,
        "div",
        &[("id", "second"), ("class", "tile"), ("style", "width:47px")],
    );
    let first_css = invalid_css.unwrap_or(
        "#app { display:flex; box-sizing:border-box; width:301px; height:100px; flex-direction:row; align-items:flex-start; justify-content:flex-start; gap:var(--space); --space:11px; --surface:#123456; background-color:var(--surface); }\ndiv.tile { display:block; box-sizing:border-box; width:24px; height:14px; --tile:#3366ff; background-color:var(--tile); }\n.tile { width:31px; }\n#second { width:39px !important; }\n.first { width:47px !important; }",
    );
    let second_css = ".tile { width:41px; }\n#second { width:43px !important; }";
    batch.push(DocumentOperation::CreateText {
        node: first_text,
        data: first_css.into(),
    });
    batch.push(DocumentOperation::CreateText {
        node: second_text,
        data: second_css.into(),
    });
    append_element(&mut batch, HostParent::Root, root);
    append_element(&mut batch, HostParent::Node(root), first_sheet);
    append_element(&mut batch, HostParent::Node(first_sheet), first_text);
    append_element(&mut batch, HostParent::Node(root), first);
    append_element(&mut batch, HostParent::Node(root), second_sheet);
    append_element(&mut batch, HostParent::Node(second_sheet), second_text);
    append_element(&mut batch, HostParent::Node(root), second);
    if let Some(visible) = visible {
        batch.push(DocumentOperation::CreateText {
            node: visible,
            data: "visible text".into(),
        });
        append_element(&mut batch, HostParent::Node(root), visible);
    }
    document.commit(batch).unwrap();
    (
        document,
        root,
        first_sheet,
        first,
        second_sheet,
        second_text,
        second,
    )
}

#[test]
fn runtime_author_css_matches_pinned_chromium_values_geometry_and_paint() {
    let (document, root, first_sheet, first, second_sheet, _second_text, second) =
        make_cascade_fixture(None, false);
    let request = request(&document, viewport());
    assert!(
        compute_request(&request)
            .unwrap_err()
            .contains("background-color")
    );
    let calculation = compute_request_for_runtime_gpu(&request).unwrap();
    let styles = &calculation.roots[0].styles;
    let style_for = |node: HostNodeHandle| {
        styles
            .elements
            .iter()
            .find(|style| style.node_id == node.id())
            .unwrap()
    };
    assert_eq!(style_for(root).properties["display"], "flex");
    assert_eq!(style_for(root).properties["width"], "301px");
    assert_eq!(style_for(root).properties["height"], "100px");
    assert_eq!(style_for(root).properties["row-gap"], "11px");
    assert_eq!(style_for(root).properties["column-gap"], "11px");
    assert_eq!(
        style_for(root).background_color,
        Some(OpaqueCssSrgb {
            red: 18,
            green: 52,
            blue: 86,
        })
    );
    assert_eq!(style_for(first_sheet).properties["display"], "none");
    assert_eq!(style_for(first_sheet).properties["width"], "auto");
    assert_eq!(style_for(first_sheet).properties["height"], "auto");
    assert_eq!(style_for(second_sheet).properties["display"], "none");
    assert_eq!(style_for(second_sheet).properties["width"], "auto");
    assert_eq!(style_for(second_sheet).properties["height"], "auto");
    assert_eq!(style_for(first).properties["display"], "block");
    assert_eq!(style_for(first).properties["width"], "47px");
    assert_eq!(style_for(first).properties["height"], "14px");
    assert_eq!(style_for(second).properties["width"], "43px");
    assert_eq!(
        style_for(first).background_paint,
        Some(ComputedBackgroundPaint::Opaque(OpaqueCssSrgb {
            red: 51,
            green: 102,
            blue: 255,
        }))
    );

    let layout = calculation.layout.unwrap();
    let frame_for = |node: HostNodeHandle| {
        layout
            .frames
            .iter()
            .find(|frame| frame.node_id == node.id().get())
            .unwrap()
    };
    assert_eq!(frame_for(first).width, 47.0);
    assert_eq!(frame_for(first).height, 14.0);
    assert_eq!(frame_for(first_sheet).width, 0.0);
    assert_eq!(frame_for(first_sheet).height, 0.0);
    assert_eq!(frame_for(second_sheet).width, 0.0);
    assert_eq!(frame_for(second_sheet).height, 0.0);
    assert_eq!(frame_for(second).x, 58.0);
    assert_eq!(frame_for(second).width, 43.0);
    assert_eq!(frame_for(second).height, 14.0);
    let scene = layout.render_snapshot.unwrap();
    assert_eq!(scene.boxes().len(), 3);
    assert_eq!(
        scene.boxes()[1].paint(),
        RuntimePaint::Opaque(spinon_render::OpaqueCssSrgb::new(51, 102, 255))
    );
    assert_eq!(scene.boxes()[2].frame_css_px().x(), 58.0);
}

#[test]
fn runtime_rejects_author_stylesheet_parse_diagnostics_before_layout_or_paint() {
    let invalid_css = "#app { display:flex; width:301px; height:100px; }\n.first { width:???; }";
    let (document, _, _, _, _, _, _) = make_cascade_fixture(Some(invalid_css), false);
    let request = request(&document, viewport());

    let layout_only = compute_request(&request).unwrap_err();
    let gpu = compute_request_for_runtime_gpu(&request).unwrap_err();
    assert!(layout_only.contains("host-style:"));
    assert!(layout_only.contains(":2:"));
    assert_eq!(layout_only, gpu);
}

#[test]
fn runtime_omits_style_text_but_still_rejects_visible_text() {
    let (hidden_document, _, _, _, _, _, _) = make_cascade_fixture(None, false);
    let hidden_request = request(&hidden_document, viewport());
    assert!(
        compute_request_for_runtime_gpu(&hidden_request)
            .unwrap()
            .layout
            .unwrap()
            .render_snapshot
            .is_some()
    );

    let (visible_document, _, _, _, _, _, _) = make_cascade_fixture(None, true);
    let visible_request = request(&visible_document, viewport());
    let layout = compute_request_for_runtime_gpu(&visible_request)
        .unwrap()
        .layout
        .unwrap_err();
    assert_eq!(layout.code, "unsupported_text_node");
}

#[test]
fn stylesheet_text_move_detach_and_reinsert_use_the_current_document_order() {
    let (mut document, root, first_sheet, first, second_sheet, second_text, second) =
        make_cascade_fixture(None, false);
    let owner = OwnerId::new(9045).unwrap();
    let width = |document: &HostDocument| {
        let calculation = compute_request_for_runtime_gpu(&request(document, viewport())).unwrap();
        calculation.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.node_id == second.id())
            .unwrap()
            .properties["width"]
            .clone()
    };
    assert_eq!(width(&document), "43px");

    let mut update = DocumentChangeBatch::new(owner, document.document_revision());
    update.push(DocumentOperation::SetTextData {
        node: second_text,
        data: "#second { width:45px !important; }".into(),
    });
    document.commit(update).unwrap();
    assert_eq!(width(&document), "45px");

    let mut move_sheet = DocumentChangeBatch::new(owner, document.document_revision());
    move_sheet.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(root),
        node: second_sheet,
        before: Some(first_sheet),
    });
    document.commit(move_sheet).unwrap();
    assert_eq!(width(&document), "39px");

    let mut detach = DocumentChangeBatch::new(owner, document.document_revision());
    detach.push(DocumentOperation::RemoveChild {
        parent: HostParent::Node(root),
        node: second_sheet,
    });
    document.commit(detach).unwrap();
    let sources = collect_runtime_author_stylesheets(&document.snapshot()).unwrap();
    assert_eq!(sources.len(), 1);
    assert_eq!(width(&document), "39px");

    let mut reinsert = DocumentChangeBatch::new(owner, document.document_revision());
    reinsert.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(root),
        node: second_sheet,
        before: Some(first),
    });
    document.commit(reinsert).unwrap();
    assert_eq!(width(&document), "45px");
}

use std::sync::Arc;

use sha2::{Digest, Sha256};
use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, IntrinsicSizeRevision, StyleRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_runtime::RuntimeSession;
use spinon_style::{
    ComputedStyleSnapshot, CssOrigin, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};
use spinon_style_to_layout::compute_runtime_style_layout_with_intrinsic_size_snapshot;

use super::{HTML_NAMESPACE, IMAGE_URL, PNG, element_attribute, json_number, viewport};

const ROOT_HTML_DOCUMENT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/html/c14-1-root-html-image.html"
));
const ROOT_HTML_SHA256: &str = "56fc0d9c5a444550866eba7ee65fbe3960b27631830294093107c8c20d77b386";

pub(super) fn run_fixture(
    reference: &serde_json::Value,
    png_sha256: &str,
) -> Result<(String, bool), String> {
    let (runtime, document, document_element, root, image, head) = create_document()?;
    let root_html_sha256 = Sha256::digest(ROOT_HTML_DOCUMENT.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let root_html_source_matches = root_html_sha256 == ROOT_HTML_SHA256;
    let stylesheet = stylesheet_from_document(&document, head)?;
    let request = ImageRequest::from_document(&document, image)
        .map_err(|error| format!("parsed root HTML image request: {error}"))?;
    if request.src().to_string_lossy() != IMAGE_URL {
        return Err("파싱된 루트 HTML img src가 고정 C14.1 fixture URL과 다릅니다".to_owned());
    }
    let request_source = request.source();
    let completion = request
        .decode(PNG)
        .map_err(|error| format!("parsed root HTML PNG decode: {error}"))?;
    let resources = ImageResourceSnapshot::new(
        &document,
        IntrinsicSizeRevision::INITIAL
            .checked_next()
            .ok_or_else(|| "intrinsic revision exhausted".to_owned())?,
        [completion],
    )
    .map_err(|error| format!("parsed root HTML resource snapshot: {error}"))?;
    let natural_size = resources
        .images()
        .get(&image.id())
        .ok_or_else(|| "parsed root HTML image missing from resource snapshot".to_owned())?
        .dimensions();
    let intrinsic = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| format!("parsed root HTML intrinsic layout snapshot: {error}"))?;
    let request_source_matches = request_source == resources.source();
    let intrinsic_source_matches = intrinsic.source() == resources.source();
    let source_matches = request_source_matches && intrinsic_source_matches;
    let revision_matches = intrinsic.revision() == resources.revision();

    let view = StyloDocumentView::new_html_shared(Arc::new(document.clone()), document_element)
        .map_err(|error| format!("parsed root HTML Stylo document: {error}"))?;
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        viewport(),
        StyleRevision::INITIAL,
    )
    .map_err(|error| format!("parsed root HTML CSS cascade: {error}"))?;
    let document_element_display = computed_property(&computed, document_element, "display");
    let head_display = computed_property(&computed, head, "display");
    let root_display = computed_property(&computed, root, "display");
    let image_display = computed_property(&computed, image, "display");
    let computed_css_matches = document_element_display.as_deref() == Some("block")
        && head_display.as_deref() == Some("none")
        && root_display.as_deref() == Some("block")
        && image_display.as_deref() == Some("block");
    let output = compute_runtime_style_layout_with_intrinsic_size_snapshot(
        &document,
        document_element,
        computed,
        viewport(),
        intrinsic,
    )
    .map_err(|error| {
        format!(
            "parsed root HTML CSS layout: {error}; html.display={document_element_display:?}; head.display={head_display:?}; main.display={root_display:?}; img.display={image_display:?}"
        )
    })?;
    let frame = *output
        .layout
        .frames
        .get(&image.id())
        .ok_or_else(|| "parsed root HTML image frame missing from layout output".to_owned())?;

    let natural_width = natural_size.width() as f32;
    let natural_height = natural_size.height() as f32;
    let observation = reference
        .get("observation")
        .ok_or_else(|| "reference observation missing".to_owned())?;
    let expected_width = json_number(observation, "natural_width")?;
    let expected_height = json_number(observation, "natural_height")?;
    let expected_rect = observation
        .get("rect")
        .ok_or_else(|| "reference rect missing".to_owned())?;
    let expected_frame = [
        json_number(expected_rect, "x")?,
        json_number(expected_rect, "y")?,
        json_number(expected_rect, "width")?,
        json_number(expected_rect, "height")?,
    ];
    let actual_frame = [frame.x, frame.y, frame.width, frame.height];
    let frame_matches = actual_frame
        .iter()
        .zip(expected_frame)
        .all(|(actual, expected)| actual.is_finite() && (*actual - expected).abs() <= 0.5);
    let natural_size_matches = natural_width == expected_width && natural_height == expected_height;
    let matches = source_matches
        && revision_matches
        && frame_matches
        && natural_size_matches
        && computed_css_matches
        && root_html_source_matches;
    let report = serde_json::json!({
        "status": if matches { "ok" } else { "mismatch" },
        "document_origin": "parsed-root-html-v8",
        "root_html_sha256": root_html_sha256,
        "root_html_source_matches": root_html_source_matches,
        "png_sha256": png_sha256,
        "html_document_element_count": 1,
        "html_head_count": 1,
        "html_body_count": 1,
        "root_html_style_count": 1,
        "html_image_count": 1,
        "script_execution": "not-run",
        "document_generation": format!("{:?}", document.generation()),
        "document_revision": document.document_revision().get(),
        "render_tree_revision": document.render_tree_revision().get(),
        "document_element_node_id": format!("{:?}", document_element.id()),
        "html_computed_display": document_element_display,
        "head_computed_display": head_display,
        "root_node_id": format!("{:?}", root.id()),
        "node_id": format!("{:?}", image.id()),
        "request_source_matches": request_source_matches,
        "intrinsic_source_matches": intrinsic_source_matches,
        "intrinsic_revision_matches": revision_matches,
        "main_computed_display": root_display,
        "image_computed_display": image_display,
        "source_revision": format!("{:?}", resources.source()),
        "intrinsic_revision": format!("{:?}", resources.revision()),
        "natural_size_css_px": [natural_width, natural_height],
        "expected_natural_size_css_px": [expected_width, expected_height],
        "frame_css_px": [frame.x, frame.y, frame.width, frame.height],
        "expected_frame_css_px": expected_frame,
        "tolerance_css_px": 0.5,
    });
    let serialized = serde_json::to_string(&report)
        .map_err(|error| format!("parsed root HTML report serialization: {error}"))?;
    drop(runtime);
    Ok((serialized, matches))
}

fn create_document() -> Result<
    (
        RuntimeSession,
        HostDocumentSnapshot,
        HostNodeHandle,
        HostNodeHandle,
        HostNodeHandle,
        HostNodeHandle,
    ),
    String,
> {
    let (runtime, _ready_report) = RuntimeSession::new_with_root_html(ROOT_HTML_DOCUMENT)
        .map_err(|error| format!("parsed root HTML V8 runtime 생성 실패: {error}"))?;
    let document = runtime
        .snapshot_document_for_c14_1_fixture()
        .map_err(|error| format!("parsed root HTML HostDocument snapshot 실패: {error}"))?;
    let roots = document.root_children().collect::<Vec<_>>();
    let [html] = roots.as_slice() else {
        return Err(format!(
            "parsed root HTML documentElement 개수가 1이 아닙니다: {}",
            roots.len()
        ));
    };
    if !is_html_element(&document, *html, "html") {
        return Err("parsed root HTML documentElement가 HTML html이 아닙니다".to_owned());
    }
    let head = unique_direct_child_element(&document, *html, "head")?;
    let body = unique_direct_child_element(&document, *html, "body")?;
    let root = unique_html_element_by_id(&document, "main", "root")?;
    let image = unique_html_element_by_id(&document, "img", "image")?;
    if !document
        .children(body)
        .is_some_and(|mut children| children.any(|child| child == root))
    {
        return Err("parsed root HTML main#root가 body 직계 자식이 아닙니다".to_owned());
    }
    if !document
        .children(root)
        .is_some_and(|mut children| children.any(|child| child == image))
    {
        return Err("parsed root HTML img#image가 main#root 직계 자식이 아닙니다".to_owned());
    }
    if !document.is_connected(root) || !document.is_connected(image) {
        return Err("parsed root HTML fixture node가 document에 연결되지 않았습니다".to_owned());
    }
    Ok((runtime, document, *html, root, image, head))
}

fn unique_direct_child_element(
    document: &HostDocumentSnapshot,
    parent: HostNodeHandle,
    local_name: &str,
) -> Result<HostNodeHandle, String> {
    let children = document
        .children(parent)
        .ok_or_else(|| format!("parsed root HTML {local_name} parent child 목록이 없습니다"))?;
    let matches = children
        .filter(|child| is_html_element(document, *child, local_name))
        .collect::<Vec<_>>();
    let [child] = matches.as_slice() else {
        return Err(format!(
            "parsed root HTML direct {local_name} 개수가 1이 아닙니다: {}",
            matches.len()
        ));
    };
    Ok(*child)
}

fn unique_html_element_by_id(
    document: &HostDocumentSnapshot,
    local_name: &str,
    id: &str,
) -> Result<HostNodeHandle, String> {
    let mut pending = document.root_children().collect::<Vec<_>>();
    let mut matches = Vec::new();
    while let Some(handle) = pending.pop() {
        if is_html_element(document, handle, local_name)
            && document
                .node(handle)
                .and_then(|node| match node.kind() {
                    HostNodeKind::Element(element) => element_attribute(element, "id"),
                    _ => None,
                })
                .as_deref()
                == Some(id)
        {
            matches.push(handle);
        }
        if let Some(children) = document.children(handle) {
            pending.extend(children);
        }
    }
    let [element] = matches.as_slice() else {
        return Err(format!(
            "parsed root HTML unique {local_name}#{id} 개수가 1이 아닙니다: {}",
            matches.len()
        ));
    };
    Ok(*element)
}

fn is_html_element(
    document: &HostDocumentSnapshot,
    handle: HostNodeHandle,
    local_name: &str,
) -> bool {
    document.node(handle).is_some_and(|node| match node.kind() {
        HostNodeKind::Element(element) => {
            element.namespace() == HTML_NAMESPACE && element.local_name() == local_name
        }
        _ => false,
    })
}

fn stylesheet_from_document(
    document: &HostDocumentSnapshot,
    head: HostNodeHandle,
) -> Result<StylesheetSource, String> {
    let head_children = document
        .children(head)
        .ok_or_else(|| "parsed root HTML head child 목록이 없습니다".to_owned())?;
    let styles = head_children
        .filter(|child| is_html_element(document, *child, "style"))
        .collect::<Vec<_>>();
    let [style] = styles.as_slice() else {
        return Err(format!(
            "parsed root HTML style element 개수가 1이 아닙니다: {}",
            styles.len()
        ));
    };
    let style_children = document
        .children(*style)
        .ok_or_else(|| "parsed root HTML style child 목록이 없습니다".to_owned())?;
    let mut css = String::new();
    for child in style_children {
        let node = document
            .node(child)
            .ok_or_else(|| "parsed root HTML style text node가 없습니다".to_owned())?;
        match node.kind() {
            HostNodeKind::Text(text) => css.push_str(&text.to_string_lossy()),
            HostNodeKind::Element(_) => {
                return Err("parsed root HTML style 안에 element child가 있습니다".to_owned());
            }
        }
    }
    if css.trim().is_empty() {
        return Err("parsed root HTML style text가 비어 있습니다".to_owned());
    }
    Ok(StylesheetSource {
        id: "c14-1-parsed-root-html".to_owned(),
        base_url: "https://spinon.invalid/c14-1-root-html-image.css".to_owned(),
        origin: CssOrigin::Author,
        css,
    })
}

fn computed_property(
    snapshot: &ComputedStyleSnapshot,
    node: HostNodeHandle,
    property: &str,
) -> Option<String> {
    snapshot
        .elements
        .iter()
        .find(|style| style.node_id == node.id())
        .and_then(|style| style.properties.get(property))
        .cloned()
}

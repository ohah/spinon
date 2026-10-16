use std::sync::Arc;

use sha2::{Digest, Sha256};
use spinon_core::{
    AttributeName, EnvironmentRevision, HostDocumentSnapshot, HostElement, HostNodeHandle,
    HostNodeKind, IntrinsicSizeRevision, StyleRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::LayoutSourceRevision;
use spinon_runtime::{RuntimeSession, TaskPriority};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_block_flex_grid_template_cascade_with_stylesheets,
};
use spinon_style_to_layout::compute_runtime_style_layout_with_intrinsic_size_snapshot;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const CASE_IDS: [&str; 14] = [
    "min-content",
    "border-box-min-content",
    "max-content",
    "fit-content-small-basis",
    "percent-width",
    "percent-height",
    "fit-content-max-clamped",
    "percent-cyclic-width",
    "flex-min-content",
    "flex-percent-width",
    "flex-percent-height",
    "grid-min-content",
    "grid-percent-width",
    "grid-percent-height",
];
const PNG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/resources/c14-1-png-basic.png"
));
const V8_DOCUMENT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/c14-3-live-v8-document.js"
));
const STYLESHEET: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/c14-3-live-v8-stylesheet.css"
));
const REFERENCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/c14-3-image-intrinsic-contributions-v1.json"
));

fn run_fixture() -> Result<(String, bool), String> {
    let reference: serde_json::Value =
        serde_json::from_str(REFERENCE).map_err(|error| format!("reference JSON: {error}"))?;
    let png_sha256 = verify_png(&reference)?;
    let (document, root, cases) = create_live_document()?;
    let mut completions = Vec::with_capacity(cases.len());
    for (id, _, image) in &cases {
        let request = ImageRequest::from_document(&document, *image)
            .map_err(|error| format!("{id} image request: {error}"))?;
        if request.src().to_string_lossy() != IMAGE_URL {
            return Err(format!("{id} V8 img src가 고정 C14.3 PNG URL과 다릅니다"));
        }
        completions.push(
            request
                .decode(PNG)
                .map_err(|error| format!("{id} PNG decode: {error}"))?,
        );
    }
    let resources = ImageResourceSnapshot::new(
        &document,
        IntrinsicSizeRevision::INITIAL
            .checked_next()
            .ok_or_else(|| "intrinsic revision exhausted".to_owned())?,
        completions,
    )
    .map_err(|error| format!("resource snapshot: {error}"))?;
    let intrinsic = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| format!("intrinsic layout snapshot: {error}"))?;
    let view = StyloDocumentView::new_html_runtime_mount_shared(Arc::new(document.clone()), root)
        .map_err(|error| format!("Stylo document: {error}"))?;
    let stylesheet = StylesheetSource {
        id: "c14-3-live-v8-intrinsic-contributions".to_owned(),
        base_url: "https://spinon.invalid/c14-3-live-v8.css".to_owned(),
        origin: CssOrigin::Author,
        css: STYLESHEET.to_owned(),
    };
    let viewport = viewport();
    let computed = compute_runtime_block_flex_grid_template_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        viewport,
        StyleRevision::INITIAL,
    )
    .map_err(|error| format!("CSS cascade: {error}"))?;
    let output = compute_runtime_style_layout_with_intrinsic_size_snapshot(
        &document,
        root,
        computed,
        viewport,
        intrinsic.clone(),
    )
    .map_err(|error| format!("CSS layout: {error}"))?;

    let tolerance = reference["toleranceCssPx"]
        .as_f64()
        .ok_or_else(|| "oracle tolerance missing".to_owned())? as f32;
    let mut matched_count = 0usize;
    let mut report_cases = Vec::with_capacity(cases.len());
    for (id, basis, image) in &cases {
        let case = reference_case(&reference, id)?;
        let basis_frame = output
            .layout
            .frames
            .get(&basis.id())
            .ok_or_else(|| format!("{id} containing block frame missing"))?;
        let image_frame = output
            .layout
            .frames
            .get(&image.id())
            .ok_or_else(|| format!("{id} image frame missing"))?;
        let edge = if *id == "fit-content-small-basis" {
            10.0
        } else {
            0.0
        };
        let actual_basis = [basis_frame.width - edge, basis_frame.height - edge];
        let expected_basis = [
            json_number(&case["containingBlockContentSize"], "width")?,
            json_number(&case["containingBlockContentSize"], "height")?,
        ];
        let actual_size = [image_frame.width, image_frame.height];
        let expected_size = [
            json_number(&case["imageUsedSize"], "width")?,
            json_number(&case["imageUsedSize"], "height")?,
        ];
        let basis_matches = within_tolerance(actual_basis, expected_basis, tolerance);
        let size_matches = within_tolerance(actual_size, expected_size, tolerance);
        let right_overrun = if *id == "fit-content-small-basis" {
            image_frame.x + image_frame.width - (basis_frame.x + basis_frame.width)
        } else {
            0.0
        };
        let overflow_matches =
            *id != "fit-content-small-basis" || (right_overrun - 7.0).abs() <= tolerance;
        let natural_size = resources
            .images()
            .get(&image.id())
            .ok_or_else(|| format!("{id} decoded image missing from snapshot"))?
            .dimensions();
        let natural_matches = natural_size.width() == 24 && natural_size.height() == 16;
        let matches = basis_matches && size_matches && overflow_matches && natural_matches;
        if matches {
            matched_count += 1;
        }
        report_cases.push(serde_json::json!({
            "id": id,
            "status": if matches { "ok" } else { "mismatch" },
            "context": case["context"],
            "containing_block_content_size_css_px": actual_basis,
            "expected_containing_block_content_size_css_px": expected_basis,
            "image_frame_css_px": [image_frame.x, image_frame.y, image_frame.width, image_frame.height],
            "expected_image_size_css_px": expected_size,
            "natural_size_css_px": [natural_size.width(), natural_size.height()],
            "right_overrun_beyond_border_css_px": right_overrun,
        }));
    }
    let source_matches = intrinsic.source()
        == LayoutSourceRevision::HostDocument {
            generation: document.generation(),
            document: document.document_revision(),
            render_tree: document.render_tree_revision(),
        }
        && resources.source() == intrinsic.source();
    let revision_matches = intrinsic.revision() == resources.revision();
    let matches = matched_count == CASE_IDS.len() && source_matches && revision_matches;
    let report = serde_json::json!({
        "status": if matches { "ok" } else { "mismatch" },
        "fixture": "c14-3-live-v8-image-intrinsic-contributions",
        "document_origin": "live-v8",
        "oracle": "HeadlessChrome/155",
        "png_sha256": png_sha256,
        "document_generation": format!("{:?}", document.generation()),
        "document_revision": document.document_revision().get(),
        "render_tree_revision": document.render_tree_revision().get(),
        "image_count": cases.len(),
        "matched_count": matched_count,
        "intrinsic_source_matches": source_matches,
        "intrinsic_revision_matches": revision_matches,
        "tolerance_css_px": tolerance,
        "cases": report_cases,
    });
    let serialized =
        serde_json::to_string(&report).map_err(|error| format!("report serialization: {error}"))?;
    Ok((serialized, matches))
}

fn create_live_document() -> Result<
    (
        HostDocumentSnapshot,
        HostNodeHandle,
        Vec<(String, HostNodeHandle, HostNodeHandle)>,
    ),
    String,
> {
    let (runtime, _) = RuntimeSession::new().map_err(|error| format!("live V8 생성: {error}"))?;
    let eval = runtime.eval(V8_DOCUMENT, TaskPriority::UserVisible);
    if eval.status != 0 {
        return Err(format!(
            "live V8 C14.3 문서 script 실패 status={} report={}",
            eval.status, eval.report
        ));
    }
    let document = runtime
        .snapshot_document_for_c14_1_fixture()
        .map_err(|error| format!("live V8 HostDocument snapshot: {error}"))?;
    let root_children = document.root_children().collect::<Vec<_>>();
    let [root] = root_children.as_slice() else {
        return Err(format!(
            "V8 문서 root child 수가 1이 아닙니다: {}",
            root_children.len()
        ));
    };
    let root = *root;
    ensure_element(&document, root, "div", "root")?;
    let basis_nodes = document
        .children(root)
        .ok_or_else(|| "V8 root child 목록이 없습니다".to_owned())?
        .collect::<Vec<_>>();
    if basis_nodes.len() != CASE_IDS.len() {
        return Err(format!(
            "V8 C14.3 basis 수가 다릅니다: expected={} actual={}",
            CASE_IDS.len(),
            basis_nodes.len()
        ));
    }
    let mut cases = Vec::with_capacity(CASE_IDS.len());
    for (id, basis) in CASE_IDS.iter().zip(basis_nodes) {
        ensure_element(&document, basis, "div", &format!("basis-{id}"))?;
        let children = document
            .children(basis)
            .ok_or_else(|| format!("{id} image child 목록이 없습니다"))?
            .collect::<Vec<_>>();
        let [image] = children.as_slice() else {
            return Err(format!(
                "{id} basis child 수가 1이 아닙니다: {}",
                children.len()
            ));
        };
        let image = *image;
        ensure_element(&document, image, "img", id)?;
        if !document.is_connected(image) {
            return Err(format!("{id} HTML img가 V8 문서에 연결되지 않았습니다"));
        }
        cases.push(((*id).to_owned(), basis, image));
    }
    Ok((document, root, cases))
}

fn ensure_element(
    document: &HostDocumentSnapshot,
    node: HostNodeHandle,
    local_name: &str,
    id: &str,
) -> Result<(), String> {
    let snapshot_node = document
        .node(node)
        .ok_or_else(|| format!("{id} HostDocument node가 없습니다"))?;
    let element = match snapshot_node.kind() {
        HostNodeKind::Element(element)
            if element.namespace() == HTML_NAMESPACE && element.local_name() == local_name =>
        {
            element
        }
        _ => return Err(format!("{id} node가 HTML {local_name}이 아닙니다")),
    };
    if element_attribute(element, "id").as_deref() != Some(id) {
        return Err(format!("{id} node id가 기대값과 다릅니다"));
    }
    Ok(())
}

fn element_attribute(element: &HostElement, name: &str) -> Option<String> {
    let name = AttributeName::new(None, name.to_owned())?;
    element
        .attribute(&name)
        .map(|value| value.to_string_lossy())
}

fn reference_case<'a>(
    reference: &'a serde_json::Value,
    id: &str,
) -> Result<&'a serde_json::Value, String> {
    reference["cases"]
        .as_array()
        .and_then(|cases| cases.iter().find(|case| case["id"] == id))
        .ok_or_else(|| format!("oracle case missing: {id}"))
}

fn json_number(value: &serde_json::Value, key: &str) -> Result<f32, String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_f64)
        .map(|number| number as f32)
        .ok_or_else(|| format!("oracle number missing: {key}"))
}

fn within_tolerance(actual: [f32; 2], expected: [f32; 2], tolerance: f32) -> bool {
    actual
        .iter()
        .zip(expected)
        .all(|(actual, expected)| actual.is_finite() && (*actual - expected).abs() <= tolerance)
}

fn verify_png(reference: &serde_json::Value) -> Result<String, String> {
    let expected = reference["naturalImage"]["sha256"]
        .as_str()
        .ok_or_else(|| "oracle PNG SHA-256 missing".to_owned())?;
    let actual = Sha256::digest(PNG)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    if actual != expected {
        return Err(format!(
            "C14.3 PNG hash mismatch: expected={expected} actual={actual}"
        ));
    }
    Ok(actual)
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 1280.0,
        height_css_px: 633.0,
        device_scale_factor: 1.0,
        environment_revision: EnvironmentRevision::default(),
        media_environment: CssMediaEnvironment::DESKTOP,
    }
}

/// 고정 Chrome 155 C14.3 oracle을 live V8 문서의 14개 이미지에서 실행합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트만큼 쓸 수 있는 메모리여야 합니다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spinon_c14_3_app_intrinsic_contributions_fixture(
    output: *mut std::ffi::c_char,
    output_capacity: usize,
) -> i32 {
    if output.is_null() || output_capacity == 0 {
        return -1;
    }
    match run_fixture() {
        Ok((report, matches)) => {
            if !crate::write_report(output, output_capacity, &report) {
                return -3;
            }
            if matches { 0 } else { -4 }
        }
        Err(detail) => {
            let report = serde_json::json!({"status": "error", "detail": detail}).to_string();
            if !crate::write_report(output, output_capacity, &report) {
                return -3;
            }
            -2
        }
    }
}

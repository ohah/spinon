use std::sync::Arc;

use sha2::{Digest, Sha256};
use spinon_core::{
    AttributeName, EnvironmentRevision, HostDocumentSnapshot, HostElement, HostNodeHandle,
    HostNodeKind, IntrinsicSizeRevision, StyleRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_runtime::{RuntimeSession, TaskPriority};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};
use spinon_style_to_layout::compute_runtime_style_layout_with_intrinsic_size_snapshot;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const PNG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/resources/c14-1-png-basic.png"
));
const HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/replaced-auto-size.html"
));
const V8_DOCUMENT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/c14-1-live-v8-document.js"
));
const REFERENCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/resources/c14-1-png-basic-reference.json"
));

fn run_fixture() -> Result<(String, bool), String> {
    let reference: serde_json::Value =
        serde_json::from_str(REFERENCE).map_err(|error| format!("reference JSON: {error}"))?;
    let png_sha256 = verify_png_reference(&reference)?;
    let (runtime, document_snapshot, root, image) = create_live_document()?;
    let image_request = ImageRequest::from_document(&document_snapshot, image)
        .map_err(|error| format!("image request: {error}"))?;
    if image_request.src().to_string_lossy() != IMAGE_URL {
        return Err("V8 문서 img src가 고정 C14.1 fixture URL과 다릅니다".to_owned());
    }
    let request_source = image_request.source();
    let stale_completion = image_request
        .clone()
        .decode(PNG)
        .map_err(|error| format!("stale-control PNG decode: {error}"))?;
    let completion = image_request
        .decode(PNG)
        .map_err(|error| format!("PNG decode: {error}"))?;
    let resources = ImageResourceSnapshot::new(
        &document_snapshot,
        IntrinsicSizeRevision::INITIAL
            .checked_next()
            .ok_or_else(|| "intrinsic revision exhausted".to_owned())?,
        [completion],
    )
    .map_err(|error| format!("resource snapshot: {error}"))?;
    let natural_size = resources
        .images()
        .get(&image.id())
        .ok_or_else(|| "decoded image missing from resource snapshot".to_owned())?
        .dimensions();
    let intrinsic = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| format!("intrinsic layout snapshot: {error}"))?;
    let request_source_matches = request_source == resources.source();
    let source_matches = request_source_matches && intrinsic.source() == resources.source();
    let revision_matches = intrinsic.revision() == resources.revision();

    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(document_snapshot.clone()), root)
            .map_err(|error| format!("Stylo document: {error}"))?;
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet()?],
        viewport(),
        StyleRevision::INITIAL,
    )
    .map_err(|error| format!("CSS cascade: {error}"))?;
    let output = compute_runtime_style_layout_with_intrinsic_size_snapshot(
        &document_snapshot,
        root,
        computed,
        viewport(),
        intrinsic,
    )
    .map_err(|error| format!("CSS layout: {error}"))?;
    let frame = *output
        .layout
        .frames
        .get(&image.id())
        .ok_or_else(|| "image frame missing from layout output".to_owned())?;
    let natural_width = natural_size.width() as f32;
    let natural_height = natural_size.height() as f32;
    let reference_observation = reference
        .get("observation")
        .ok_or_else(|| "reference observation missing".to_owned())?;
    let expected_width = json_number(reference_observation, "natural_width")?;
    let expected_height = json_number(reference_observation, "natural_height")?;
    let expected_rect = reference_observation
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
    let (stale_source_rejected, stale_source_document_revision) = reject_live_v8_stale_source(
        &runtime,
        image,
        stale_completion,
        document_snapshot.document_revision().get(),
        resources
            .revision()
            .checked_next()
            .ok_or_else(|| "intrinsic revision exhausted during stale control".to_owned())?,
    )?;
    let matches = source_matches
        && revision_matches
        && frame_matches
        && natural_size_matches
        && stale_source_rejected;
    let report = serde_json::json!({
        "status": if matches { "ok" } else { "mismatch" },
        "fixture": "c14-1-live-v8-png-basic",
        "png_sha256": png_sha256,
        "document_origin": "live-v8",
        "document_generation": format!("{:?}", document_snapshot.generation()),
        "document_revision": document_snapshot.document_revision().get(),
        "render_tree_revision": document_snapshot.render_tree_revision().get(),
        "root_node_id": format!("{:?}", root.id()),
        "document_revision_after_stale_control": stale_source_document_revision,
        "request_source_matches": request_source_matches,
        "stale_source_rejected": stale_source_rejected,
        "stale_source_rejection": "SourceChanged",
        "oracle": "HeadlessChrome/155",
        "viewport_css_px": [120, 80],
        "node_id": format!("{:?}", image.id()),
        "source_revision": format!("{:?}", resources.source()),
        "intrinsic_revision": format!("{:?}", resources.revision()),
        "intrinsic_source_matches": source_matches,
        "intrinsic_revision_matches": revision_matches,
        "natural_size_css_px": [natural_width, natural_height],
        "expected_natural_size_css_px": [expected_width, expected_height],
        "frame_css_px": [frame.x, frame.y, frame.width, frame.height],
        "expected_frame_css_px": expected_frame,
        "tolerance_css_px": 0.5,
    });
    let serialized =
        serde_json::to_string(&report).map_err(|error| format!("report serialization: {error}"))?;
    Ok((serialized, matches))
}

fn verify_png_reference(reference: &serde_json::Value) -> Result<String, String> {
    let fixture = reference
        .get("fixture")
        .ok_or_else(|| "reference fixture metadata missing".to_owned())?;
    if fixture.get("id").and_then(serde_json::Value::as_str) != Some("U05.1-png-natural-size-v1")
        || fixture.get("png").and_then(serde_json::Value::as_str) != Some("c14-1-png-basic.png")
    {
        return Err("C14.1 reference가 고정 PNG fixture를 가리키지 않습니다".to_owned());
    }
    let expected_bytes = fixture
        .get("png_bytes")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "reference PNG byte length missing".to_owned())?;
    if expected_bytes != PNG.len() as u64 {
        return Err(format!(
            "C14.1 PNG byte length mismatch: expected={expected_bytes} actual={}",
            PNG.len()
        ));
    }
    let expected_sha256 = fixture
        .get("png_sha256")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "reference PNG SHA-256 missing".to_owned())?;
    let actual_sha256 = Sha256::digest(PNG)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    if actual_sha256 != expected_sha256 {
        return Err(format!(
            "C14.1 PNG SHA-256 mismatch: expected={expected_sha256} actual={actual_sha256}"
        ));
    }
    Ok(actual_sha256)
}

fn reject_live_v8_stale_source(
    runtime: &RuntimeSession,
    image: HostNodeHandle,
    completion: spinon_image::ImageResourceCompletion,
    previous_document_revision: u64,
    revision: IntrinsicSizeRevision,
) -> Result<(bool, u64), String> {
    let mutation = runtime.eval(
        "globalThis.__spinonC141Image.setAttribute('src', 'https://spinon.invalid/replaced.png');",
        TaskPriority::UserVisible,
    );
    if mutation.status != 0 {
        return Err(format!(
            "live V8 stale-source mutation failed status={} report={}",
            mutation.status, mutation.report
        ));
    }
    let current = runtime
        .snapshot_document_for_c14_1_fixture()
        .map_err(|error| format!("stale-source HostDocument snapshot 실패: {error}"))?;
    if current.document_revision().get() <= previous_document_revision {
        return Err(format!(
            "src 변경 뒤 document revision이 증가하지 않았습니다: before={previous_document_revision} after={}",
            current.document_revision().get()
        ));
    }
    match ImageResourceSnapshot::new(&current, revision, [completion]) {
        Err(spinon_image::ImageSnapshotError::SourceChanged(node)) if node == image.id() => {
            Ok((true, current.document_revision().get()))
        }
        Err(error) => Err(format!(
            "stale-src completion이 다른 이유로 거부됐습니다: {error}"
        )),
        Ok(_) => {
            Err("stale-src completion이 현재 문서 snapshot에 잘못 admission됐습니다".to_owned())
        }
    }
}

fn create_live_document() -> Result<
    (
        RuntimeSession,
        HostDocumentSnapshot,
        HostNodeHandle,
        HostNodeHandle,
    ),
    String,
> {
    let (runtime, _ready_report) =
        RuntimeSession::new().map_err(|error| format!("live V8 runtime 생성 실패: {error}"))?;
    let eval = runtime.eval(V8_DOCUMENT, TaskPriority::UserVisible);
    if eval.status != 0 {
        return Err(format!(
            "live V8 C14.1 document script 실패 status={} report={}",
            eval.status, eval.report
        ));
    }
    let document = runtime
        .snapshot_document_for_c14_1_fixture()
        .map_err(|error| format!("live V8 HostDocument snapshot 실패: {error}"))?;
    let roots = document.root_children().collect::<Vec<_>>();
    let [root] = roots.as_slice() else {
        return Err(format!(
            "live V8 문서 root 개수가 1이 아닙니다: {}",
            roots.len()
        ));
    };
    let root = *root;
    let root_node = document
        .node(root)
        .ok_or_else(|| "live V8 root snapshot node가 없습니다".to_owned())?;
    let root_element = match root_node.kind() {
        HostNodeKind::Element(element)
            if element.namespace() == HTML_NAMESPACE && element.local_name() == "div" =>
        {
            element
        }
        _ => return Err("live V8 root는 HTML div가 아닙니다".to_owned()),
    };
    if element_attribute(root_element, "id").as_deref() != Some("root") {
        return Err("live V8 root id가 root가 아닙니다".to_owned());
    }
    let children = document
        .children(root)
        .ok_or_else(|| "live V8 root child 목록이 없습니다".to_owned())?
        .collect::<Vec<_>>();
    let [image] = children.as_slice() else {
        return Err(format!(
            "live V8 root의 child 개수가 1이 아닙니다: {}",
            children.len()
        ));
    };
    let image = *image;
    let image_node = document
        .node(image)
        .ok_or_else(|| "live V8 image snapshot node가 없습니다".to_owned())?;
    let image_element = match image_node.kind() {
        HostNodeKind::Element(element)
            if element.namespace() == HTML_NAMESPACE && element.local_name() == "img" =>
        {
            element
        }
        _ => return Err("live V8 child는 HTML img가 아닙니다".to_owned()),
    };
    if element_attribute(image_element, "id").as_deref() != Some("image") {
        return Err("live V8 image id가 image가 아닙니다".to_owned());
    }
    if !document.is_connected(image) {
        return Err("live V8 HTML img가 document에 연결되지 않았습니다".to_owned());
    }
    Ok((runtime, document, root, image))
}

fn element_attribute(element: &HostElement, name: &str) -> Option<String> {
    let name = AttributeName::new(None, name.to_owned())?;
    element
        .attribute(&name)
        .map(|value| value.to_string_lossy())
}

fn stylesheet() -> Result<StylesheetSource, String> {
    let (_, rest) = HTML
        .split_once("<style>")
        .ok_or_else(|| "fixture style start missing".to_owned())?;
    let (css, _) = rest
        .split_once("</style>")
        .ok_or_else(|| "fixture style end missing".to_owned())?;
    Ok(StylesheetSource {
        id: "c14-1-png-resource".to_owned(),
        base_url: "https://spinon.invalid/c14-1-png-resource.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    })
}

fn viewport() -> CssViewport {
    CssViewport {
        width_css_px: 120.0,
        height_css_px: 80.0,
        device_scale_factor: 1.0,
        environment_revision: EnvironmentRevision::default(),
        media_environment: CssMediaEnvironment::DESKTOP,
    }
}

fn json_number(value: &serde_json::Value, key: &str) -> Result<f32, String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_f64)
        .map(|number| number as f32)
        .ok_or_else(|| format!("reference number missing: {key}"))
}

/// 고정 C14.1 PNG를 CSS px intrinsic snapshot 경로에서 실행합니다.
///
/// # Safety
/// `output`은 `output_capacity` 바이트만큼 쓸 수 있는 메모리여야 합니다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spinon_c14_1_app_intrinsic_size_fixture(
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
            let report = serde_json::json!({ "status": "error", "detail": detail });
            let report = report.to_string();
            if !crate::write_report(output, output_capacity, &report) {
                return -3;
            }
            -2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{REFERENCE, verify_png_reference};

    #[test]
    fn png_reference_binds_the_embedded_fixture_bytes() {
        let reference: serde_json::Value =
            serde_json::from_str(REFERENCE).expect("C14.1 reference JSON should parse");
        assert_eq!(
            verify_png_reference(&reference).expect("fixture PNG should match its reference"),
            "5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2"
        );
    }

    #[test]
    fn png_reference_rejects_substituted_same_size_payload_metadata() {
        let mut reference: serde_json::Value =
            serde_json::from_str(REFERENCE).expect("C14.1 reference JSON should parse");
        reference["fixture"]["png_sha256"] = serde_json::json!("00".repeat(32));
        assert!(
            verify_png_reference(&reference)
                .expect_err("a different PNG digest must fail closed")
                .contains("SHA-256 mismatch")
        );
    }
}

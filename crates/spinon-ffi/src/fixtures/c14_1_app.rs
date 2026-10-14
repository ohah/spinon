use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostNodeHandle, HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};
use spinon_style_to_layout::compute_runtime_style_layout_with_intrinsic_size_snapshot;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const PNG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/resources/c14-1-png-basic.png"
));
const HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/c14/replaced-auto-size.html"
));
const REFERENCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/resources/c14-1-png-basic-reference.json"
));

fn run_fixture() -> Result<(String, bool), String> {
    let reference: serde_json::Value =
        serde_json::from_str(REFERENCE).map_err(|error| format!("reference JSON: {error}"))?;
    let (document, root, image) = create_document()?;
    let document_snapshot = document.snapshot();
    let image_request = ImageRequest::from_document(&document_snapshot, image)
        .map_err(|error| format!("image request: {error}"))?;
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
    let source_matches = intrinsic.source() == resources.source();
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
    let matches = source_matches && revision_matches && frame_matches && natural_size_matches;
    let report = serde_json::json!({
        "status": if matches { "ok" } else { "mismatch" },
        "fixture": "c14-1-png-basic",
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

fn create_document() -> Result<(HostDocument, HostNodeHandle, HostNodeHandle), String> {
    let mut document = HostDocument::new().map_err(|error| format!("HostDocument: {error}"))?;
    let root = document
        .reserve_node_handle()
        .map_err(|error| format!("root handle: {error}"))?;
    let image = document
        .reserve_node_handle()
        .map_err(|error| format!("image handle: {error}"))?;
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(1415).ok_or_else(|| "invalid owner id".to_owned())?,
        document.document_revision(),
    );
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML_NAMESPACE.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "id")
                .ok_or_else(|| "invalid id attribute".to_owned())?,
            value: "root".to_owned().into(),
        })
        .push(DocumentOperation::CreateElement {
            node: image,
            namespace: HTML_NAMESPACE.to_owned(),
            local_name: "img".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: image,
            name: AttributeName::new(None, "id")
                .ok_or_else(|| "invalid id attribute".to_owned())?,
            value: "image".to_owned().into(),
        })
        .push(DocumentOperation::SetAttribute {
            node: image,
            name: AttributeName::new(None, "src")
                .ok_or_else(|| "invalid src attribute".to_owned())?,
            value: "https://spinon.invalid/c14-1-png-basic.png"
                .to_owned()
                .into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: image,
            before: None,
        });
    document
        .commit(batch)
        .map_err(|error| format!("document commit: {error}"))?;
    Ok((document, root, image))
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
    #[test]
    fn app_fixture_report_matches_the_chrome_reference() {
        let (report, matches) = super::run_fixture().expect("C14.1 fixture should run");
        assert!(matches, "unexpected C14.1 report: {report}");
        let report: serde_json::Value =
            serde_json::from_str(&report).expect("fixture report should be JSON");
        assert_eq!(report["status"], "ok");
        assert_eq!(
            report["natural_size_css_px"],
            serde_json::json!([24.0, 16.0])
        );
        assert_eq!(
            report["frame_css_px"],
            serde_json::json!([0.0, 0.0, 24.0, 16.0])
        );
        assert_eq!(report["intrinsic_revision_matches"], true);
        assert_eq!(report["intrinsic_source_matches"], true);
    }

    #[test]
    fn app_fixture_c_abi_rejects_a_short_output_buffer() {
        let mut output = [b'x' as std::ffi::c_char; 8];
        let status = unsafe {
            super::spinon_c14_1_app_intrinsic_size_fixture(output.as_mut_ptr(), output.len())
        };
        assert_eq!(status, -3);
        assert_eq!(output[0], 0);
    }
}

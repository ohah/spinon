use std::collections::BTreeMap;

use serde_json::Value;
use sha2::{Digest, Sha256};
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_render::StaticRenderSnapshot;
use spinon_style::{CssOrigin, CssViewport, StylesheetSource, StyloDocumentView};
use spinon_style_to_layout::compute_s04_style_layout;
use spinon_style_to_render::{
    build_s04_static_render_snapshot, FixtureNodeMapping, RenderFixtureProvenance,
};
use style::context::QuirksMode;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const ASYMMETRIC_Y_FIXTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/s04/asymmetric-y.v1.json"
));
const ASYMMETRIC_Y_FIXTURE_CSS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/s04/asymmetric-y.v1.css"
));
const ASYMMETRIC_Y_CHROMIUM_REFERENCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/s04-asymmetric-y-v1-chromium-154.0.8037.98-f7ffacb8763c-06ff4aab2ac9-ccffd5c5fe77.json"
));
const ASYMMETRIC_Y_CHROMIUM_REFERENCE_SHA256: &str =
    "8cd484cf7022d3da12eeab317b9dfb10f05f010e6ddd0d9e9ee4273c295f8a3b";

#[cfg(test)]
const FLEX_PAINT_FIXTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/s04/flex-paint.v1.json"
));
#[cfg(test)]
const FLEX_PAINT_FIXTURE_CSS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/s04/flex-paint.v1.css"
));
#[cfg(test)]
const FLEX_PAINT_CHROMIUM_REFERENCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/css/references/s04-flex-paint-v1-chromium-154.0.8037.95-a4abee019ac5-827b7e12ddf3-affc6715a14a.json"
));
#[cfg(test)]
const FLEX_PAINT_CHROMIUM_REFERENCE_SHA256: &str =
    "20a55f01c35bd0b6546026bb7d6a68d0a2bfc0f4010984573ca0ac791cc85b05";

pub(crate) fn build_asymmetric_y_snapshot() -> Result<StaticRenderSnapshot, String> {
    build_snapshot_from(
        ASYMMETRIC_Y_FIXTURE_JSON,
        ASYMMETRIC_Y_FIXTURE_CSS,
        ASYMMETRIC_Y_CHROMIUM_REFERENCE,
        ASYMMETRIC_Y_CHROMIUM_REFERENCE_SHA256,
    )
}

#[cfg(test)]
fn build_flex_paint_snapshot() -> Result<StaticRenderSnapshot, String> {
    build_snapshot_from(
        FLEX_PAINT_FIXTURE_JSON,
        FLEX_PAINT_FIXTURE_CSS,
        FLEX_PAINT_CHROMIUM_REFERENCE,
        FLEX_PAINT_CHROMIUM_REFERENCE_SHA256,
    )
}

fn build_snapshot_from(
    fixture_json: &str,
    fixture_css: &str,
    chromium_reference: &str,
    chromium_reference_sha256: &str,
) -> Result<StaticRenderSnapshot, String> {
    let fixture: Value = serde_json::from_str(fixture_json)
        .map_err(|error| format!("S04 fixture JSON 파싱 실패: {error}"))?;
    let reference: Value = serde_json::from_str(chromium_reference)
        .map_err(|error| format!("S04 Chromium 기준 JSON 파싱 실패: {error}"))?;
    let fixture_sha256 = sha256(fixture_json.as_bytes());
    let stylesheet_sha256 = sha256(fixture_css.as_bytes());
    let reference_sha256 = sha256(chromium_reference.as_bytes());
    validate_digest(&fixture_sha256, &reference["fixture"]["sha256"], "fixture")?;
    validate_digest(
        &stylesheet_sha256,
        &reference["stylesheet"]["sha256"],
        "stylesheet",
    )?;
    validate_digest(
        &reference_sha256,
        &Value::String(chromium_reference_sha256.to_owned()),
        "Chromium reference",
    )?;

    let preorder = string_array(&fixture["tree"]["preorder"], "tree.preorder")?;
    let root_id = string_value(&fixture["tree"]["root"], "tree.root")?;
    let viewport = CssViewport {
        width_css_px: number_value(&fixture["viewport"]["widthCssPx"], "viewport.widthCssPx")?,
        height_css_px: number_value(&fixture["viewport"]["heightCssPx"], "viewport.heightCssPx")?,
        device_scale_factor: number_value(
            &fixture["viewport"]["deviceScaleFactor"],
            "viewport.deviceScaleFactor",
        )?,
        environment_revision: Default::default(),
    };
    let nodes = create_fixture_document(&preorder, &root_id)?;
    let root = *nodes
        .handles
        .get(&root_id)
        .ok_or_else(|| format!("S04 root 노드가 없습니다: {root_id}"))?;
    let document = nodes.document;
    let document_snapshot = document.snapshot();
    let document_base_url = string_value(&fixture["documentBaseUrl"], "documentBaseUrl")?;
    let view = StyloDocumentView::new_with_base_url(
        document_snapshot.clone(),
        root,
        true,
        QuirksMode::NoQuirks,
        &document_base_url,
    )
    .map_err(|error| format!("S04 Stylo 문서 구성 실패: {error}"))?;
    let stylesheet = StylesheetSource {
        id: string_value(&fixture["stylesheet"]["id"], "stylesheet.id")?,
        base_url: string_value(&fixture["stylesheet"]["baseUrl"], "stylesheet.baseUrl")?,
        origin: CssOrigin::Author,
        css: fixture_css.to_owned(),
    };
    let output = compute_s04_style_layout(
        &document_snapshot,
        &view,
        root,
        &[stylesheet],
        viewport,
        Default::default(),
    )
    .map_err(|error| format!("S04 CSS·레이아웃 fixture 계산 실패: {error}"))?;
    let mappings = preorder
        .iter()
        .map(|fixture_id| {
            let node = nodes
                .handles
                .get(fixture_id)
                .ok_or_else(|| format!("S04 fixture 노드가 없습니다: {fixture_id}"))?;
            Ok(FixtureNodeMapping {
                fixture_id,
                node_id: node.id(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    validate_chromium_reference(&output, &reference, &preorder, &nodes.handles)?;
    let provenance = RenderFixtureProvenance {
        fixture_id: string_value(&fixture["fixtureId"], "fixtureId")?,
        fixture_sha256,
        stylesheet_sha256,
        chromium_reference_id: string_value(&reference["referenceId"], "referenceId")?,
        chromium_reference_sha256: reference_sha256,
    };
    let current_layout_inputs = spinon_style_to_render::CurrentLayoutInputs::for_host_document(
        &document.snapshot(),
        Default::default(),
        viewport,
    );
    build_s04_static_render_snapshot(
        &document.snapshot(),
        root,
        &output,
        current_layout_inputs,
        &mappings,
        provenance,
    )
    .map_err(|error| format!("S04 정적 RenderSnapshot 생성 실패: {error}"))
}

fn validate_chromium_reference(
    output: &spinon_style_to_layout::StyleLayoutOutput,
    reference: &Value,
    preorder: &[String],
    handles: &BTreeMap<String, HostNodeHandle>,
) -> Result<(), String> {
    let observations = reference["observations"]
        .as_array()
        .ok_or_else(|| "S04 Chromium observations 값은 배열이어야 합니다".to_owned())?;
    let properties = reference["computedProperties"]
        .as_array()
        .ok_or_else(|| "S04 Chromium computedProperties 값은 배열이어야 합니다".to_owned())?;
    if observations.len() != preorder.len() || handles.len() != preorder.len() {
        return Err("S04 Chromium·HostDocument node 수가 fixture와 다릅니다".to_owned());
    }

    for (index, fixture_id) in preorder.iter().enumerate() {
        let observation = &observations[index];
        if string_value(&observation["fixtureId"], "observation.fixtureId")?.as_str() != fixture_id
        {
            return Err(format!(
                "S04 Chromium node 순서가 fixture와 다릅니다: {fixture_id}"
            ));
        }
        let handle = handles
            .get(fixture_id)
            .ok_or_else(|| format!("S04 HostDocument node mapping이 없습니다: {fixture_id}"))?;
        let computed = output
            .computed_styles
            .elements
            .iter()
            .find(|element| element.node_id == handle.id())
            .ok_or_else(|| format!("S04 computed style이 없습니다: {fixture_id}"))?;
        for property in properties {
            let property = property
                .as_str()
                .ok_or_else(|| "S04 computed property 이름은 문자열이어야 합니다".to_owned())?;
            let expected = string_value(
                &observation["computedValues"][property],
                "observation.computedValues",
            )?;
            let actual = computed.properties.get(property).ok_or_else(|| {
                format!("S04 computed property가 빠졌습니다: {fixture_id}.{property}")
            })?;
            if actual != &expected {
                return Err(format!(
                    "S04 Chromium computed style 불일치 {fixture_id}.{property}: {actual} != {expected}"
                ));
            }
        }

        let frame = output
            .layout
            .frames
            .get(&handle.id())
            .ok_or_else(|| format!("S04 layout frame이 없습니다: {fixture_id}"))?;
        let expected_frame = &observation["frameRelativeToRoot"];
        for (axis, actual) in [
            ("x", frame.x),
            ("y", frame.y),
            ("width", frame.width),
            ("height", frame.height),
        ] {
            let expected = number_value(
                &expected_frame[axis],
                &format!("observation.frameRelativeToRoot.{axis}"),
            )?;
            let error = (actual - expected).abs();
            if error > 0.5 {
                return Err(format!(
                    "S04 Chromium geometry 불일치 {fixture_id}.{axis}: {actual} != {expected} (오차 {error})"
                ));
            }
        }
    }
    Ok(())
}

struct FixtureDocument {
    document: HostDocument,
    handles: BTreeMap<String, HostNodeHandle>,
}

fn create_fixture_document(preorder: &[String], root_id: &str) -> Result<FixtureDocument, String> {
    let mut document = HostDocument::new().map_err(|error| error.to_string())?;
    let handles = preorder
        .iter()
        .map(|fixture_id| {
            document
                .reserve_node_handle()
                .map(|handle| (fixture_id.clone(), handle))
                .map_err(|error| error.to_string())
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let root = *handles
        .get(root_id)
        .ok_or_else(|| format!("S04 root ID가 preorder에 없습니다: {root_id}"))?;
    let owner =
        OwnerId::new(1804).ok_or_else(|| "S04 fixture owner ID가 잘못됐습니다".to_owned())?;
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    for fixture_id in preorder {
        let handle = *handles
            .get(fixture_id)
            .ok_or_else(|| format!("S04 노드 핸들이 없습니다: {fixture_id}"))?;
        batch
            .push(DocumentOperation::CreateElement {
                node: handle,
                namespace: HTML_NAMESPACE.to_owned(),
                local_name: "div".to_owned(),
            })
            .push(DocumentOperation::SetAttribute {
                node: handle,
                name: AttributeName::new(None, "id")
                    .ok_or_else(|| "fixture id 속성 이름이 잘못됐습니다".to_owned())?,
                value: fixture_id.clone().into(),
            });
        let parent = if fixture_id == root_id {
            HostParent::Root
        } else {
            HostParent::Node(root)
        };
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node: handle,
            before: None,
        });
    }
    document.commit(batch).map_err(|error| error.to_string())?;
    Ok(FixtureDocument { document, handles })
}

fn validate_digest(actual: &[u8; 32], expected: &Value, name: &str) -> Result<(), String> {
    let expected = string_value(expected, name)?;
    if encode_hex(actual) != expected {
        return Err(format!("S04 {name} SHA-256가 고정 기준과 다릅니다"));
    }
    Ok(())
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[usize::from(byte >> 4)] as char);
        output.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    output
}

fn string_array(value: &Value, name: &str) -> Result<Vec<String>, String> {
    value
        .as_array()
        .ok_or_else(|| format!("S04 {name} 값은 배열이어야 합니다"))?
        .iter()
        .map(|entry| string_value(entry, name))
        .collect()
}

fn string_value(value: &Value, name: &str) -> Result<String, String> {
    value
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("S04 {name} 값은 문자열이어야 합니다"))
}

fn number_value(value: &Value, name: &str) -> Result<f32, String> {
    value
        .as_f64()
        .map(|number| number as f32)
        .filter(|number| number.is_finite())
        .ok_or_else(|| format!("S04 {name} 값은 유한한 숫자여야 합니다"))
}

#[cfg(test)]
mod tests {
    use super::{build_asymmetric_y_snapshot, build_flex_paint_snapshot};
    use spinon_render::ComputedStyleProfileId;

    #[test]
    fn asymmetric_y_fixture_matches_chromium_styles_and_geometry() {
        let snapshot = build_asymmetric_y_snapshot().expect("고정 S04 snapshot 생성 성공");
        assert_eq!(
            snapshot.source().computed_style_profile,
            ComputedStyleProfileId::S04FlexPaintV1
        );
        assert_eq!(snapshot.boxes().len(), 4);
        assert_eq!(snapshot.viewport_css_px().width(), 301.0);
        assert_eq!(snapshot.viewport_css_px().height(), 65.0);
        assert_eq!(snapshot.boxes()[0].frame_css_px().y(), 0.0);
        assert_eq!(snapshot.boxes()[1].frame_css_px().y(), 0.0);
        assert_eq!(snapshot.boxes()[1].frame_css_px().height(), 12.0);
        assert_eq!(snapshot.boxes()[2].frame_css_px().y(), 15.0);
        assert_eq!(snapshot.boxes()[2].frame_css_px().height(), 18.0);
        assert_eq!(snapshot.boxes()[3].frame_css_px().y(), 36.0);
        assert_eq!(snapshot.boxes()[3].frame_css_px().height(), 24.0);
    }

    #[test]
    fn original_flex_paint_fixture_still_matches_its_chromium_reference() {
        let snapshot = build_flex_paint_snapshot().expect("기존 S04 가로 snapshot 생성 성공");
        assert_eq!(snapshot.boxes().len(), 4);
        assert_eq!(snapshot.viewport_css_px().width(), 301.0);
        assert_eq!(snapshot.viewport_css_px().height(), 40.0);
        assert_eq!(snapshot.boxes()[2].frame_css_px().x(), 53.5);
        assert_eq!(snapshot.boxes()[3].frame_css_px().x(), 155.5);
    }
}

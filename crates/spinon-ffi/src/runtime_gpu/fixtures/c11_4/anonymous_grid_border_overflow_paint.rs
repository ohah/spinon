use std::sync::Arc;

use spinon_core::{AttributeName, HostDocumentSnapshot, HostNodeKind};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_render::{
    RuntimeImageObjectStyle, RuntimeImageResourceKey, RuntimeRenderImage, RuntimeRenderSnapshot,
};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const PNG: &[u8] = include_bytes!("../../../../../../tests/fixtures/resources/c14-1-png-basic.png");
const FORWARD_TWO_IMAGE_IDS: &[&str] = &[
    "c114-overflow-image-a-forward",
    "c114-overflow-image-b-forward",
];
const REVERSED_TWO_IMAGE_IDS: &[&str] = &[
    "c114-overflow-image-b-reversed",
    "c114-overflow-image-a-reversed",
];
const FORWARD_THREE_IMAGE_IDS: &[&str] = &[
    "c114-overflow-three-image-a-forward",
    "c114-overflow-three-image-b-forward",
    "c114-overflow-three-image-c-forward",
];
const REVERSED_THREE_IMAGE_IDS: &[&str] = &[
    "c114-overflow-three-image-c-reversed",
    "c114-overflow-three-image-b-reversed",
    "c114-overflow-three-image-a-reversed",
];
const CYCLIC_INTRINSIC_FORWARD_IMAGE_IDS: &[&str] = &[
    "c114-cyclic-minmax-replaced-image-span-intrinsic-forward-image-a",
    "c114-cyclic-minmax-replaced-image-span-intrinsic-forward-image-b",
    "c114-cyclic-minmax-replaced-image-span-intrinsic-forward-image-c",
];
const CYCLIC_INTRINSIC_REVERSE_IMAGE_IDS: &[&str] = &[
    "c114-cyclic-minmax-replaced-image-span-intrinsic-reverse-image-c",
    "c114-cyclic-minmax-replaced-image-span-intrinsic-reverse-image-b",
    "c114-cyclic-minmax-replaced-image-span-intrinsic-reverse-image-a",
];
const CYCLIC_DEFINITE_FORWARD_IMAGE_IDS: &[&str] = &[
    "c114-cyclic-minmax-replaced-image-span-definite-forward-image-a",
    "c114-cyclic-minmax-replaced-image-span-definite-forward-image-b",
    "c114-cyclic-minmax-replaced-image-span-definite-forward-image-c",
];
const CYCLIC_DEFINITE_REVERSE_IMAGE_IDS: &[&str] = &[
    "c114-cyclic-minmax-replaced-image-span-definite-reverse-image-c",
    "c114-cyclic-minmax-replaced-image-span-definite-reverse-image-b",
    "c114-cyclic-minmax-replaced-image-span-definite-reverse-image-a",
];
const CYCLIC_MIXED_INTRINSIC_FORWARD_IMAGE_IDS: &[&str] =
    &["c114-cyclic-minmax-calc-mixed-text-image-span-intrinsic-forward-item-b"];
const CYCLIC_MIXED_INTRINSIC_REVERSE_IMAGE_IDS: &[&str] =
    &["c114-cyclic-minmax-calc-mixed-text-image-span-intrinsic-reverse-item-b"];
const CYCLIC_MIXED_DEFINITE_FORWARD_IMAGE_IDS: &[&str] =
    &["c114-definite-minmax-calc-mixed-text-image-span-forward-item-b"];
const CYCLIC_MIXED_DEFINITE_REVERSE_IMAGE_IDS: &[&str] =
    &["c114-definite-minmax-calc-mixed-text-image-span-reverse-item-b"];
const MAX_CONTENT_FORWARD_IMAGE_IDS: &[&str] = &["c114-max-content-forward-item-b"];
const MAX_CONTENT_REVERSE_IMAGE_IDS: &[&str] = &["c114-max-content-reverse-item-b"];
const DEFINITE_FORWARD_IMAGE_IDS: &[&str] = &["c114-definite-forward-item-b"];
const DEFINITE_REVERSE_IMAGE_IDS: &[&str] = &["c114-definite-reverse-item-b"];

pub(super) fn attach_images(
    document: &HostDocumentSnapshot,
    scene: RuntimeRenderSnapshot,
) -> Result<(RuntimeRenderSnapshot, String), String> {
    let id_attribute = AttributeName::new(None, "id")
        .ok_or_else(|| "C11.4 border overflow id 이름을 만들 수 없습니다".to_owned())?;
    let expected_roots = [
        "c114-anonymous-runs-two-images-forward-grid",
        "c114-two-images-anonymous-runs-reversed-grid",
        "c114-two-anonymous-runs-three-images-forward-grid",
        "c114-three-images-two-anonymous-runs-reversed-grid",
        "c114-cyclic-minmax-replaced-image-span-intrinsic-forward-grid",
        "c114-cyclic-minmax-replaced-image-span-intrinsic-reverse-grid",
        "c114-cyclic-minmax-replaced-image-span-definite-forward-grid",
        "c114-cyclic-minmax-replaced-image-span-definite-reverse-grid",
        "c114-cyclic-minmax-calc-mixed-text-image-span-intrinsic-forward-grid",
        "c114-cyclic-minmax-calc-mixed-text-image-span-intrinsic-reverse-grid",
        "c114-definite-minmax-calc-mixed-text-image-span-forward-grid",
        "c114-definite-minmax-calc-mixed-text-image-span-reverse-grid",
        "c114-max-content-forward-grid",
        "c114-max-content-reverse-grid",
        "c114-definite-forward-grid",
        "c114-definite-reverse-grid",
    ];
    let mut stack = document.root_children().collect::<Vec<_>>();
    let mut roots = Vec::new();
    while let Some(handle) = stack.pop() {
        let node = document
            .node(handle)
            .ok_or_else(|| "C11.4 document snapshot에서 node가 사라졌습니다".to_owned())?;
        if let HostNodeKind::Element(element) = node.kind()
            && element.namespace() == HTML_NAMESPACE
            && element.local_name() == "div"
        {
            let root_id = element
                .attribute(&id_attribute)
                .map(|value| String::from_utf16_lossy(value.code_units()))
                .unwrap_or_default();
            if expected_roots.contains(&root_id.as_str()) {
                roots.push((handle, root_id));
            }
        }
        if let Some(children) = document.children(handle) {
            stack.extend(children);
        }
    }
    let [(root, root_id)] = roots.as_slice() else {
        if roots.is_empty() {
            return Ok((scene, String::new()));
        }
        return Err("C11.4 bordered Grid root가 유일하지 않습니다".to_owned());
    };
    let scene_id = root_id
        .strip_prefix("c114-")
        .and_then(|value| value.strip_suffix("-grid"))
        .ok_or_else(|| "C11.4 bordered Grid root ID가 올바르지 않습니다".to_owned())?;
    let image_ids = match scene_id {
        "anonymous-runs-two-images-forward" => FORWARD_TWO_IMAGE_IDS,
        "two-images-anonymous-runs-reversed" => REVERSED_TWO_IMAGE_IDS,
        "two-anonymous-runs-three-images-forward" => FORWARD_THREE_IMAGE_IDS,
        "three-images-two-anonymous-runs-reversed" => REVERSED_THREE_IMAGE_IDS,
        "cyclic-minmax-replaced-image-span-intrinsic-forward" => CYCLIC_INTRINSIC_FORWARD_IMAGE_IDS,
        "cyclic-minmax-replaced-image-span-intrinsic-reverse" => CYCLIC_INTRINSIC_REVERSE_IMAGE_IDS,
        "cyclic-minmax-replaced-image-span-definite-forward" => CYCLIC_DEFINITE_FORWARD_IMAGE_IDS,
        "cyclic-minmax-replaced-image-span-definite-reverse" => CYCLIC_DEFINITE_REVERSE_IMAGE_IDS,
        "cyclic-minmax-calc-mixed-text-image-span-intrinsic-forward" => {
            CYCLIC_MIXED_INTRINSIC_FORWARD_IMAGE_IDS
        }
        "cyclic-minmax-calc-mixed-text-image-span-intrinsic-reverse" => {
            CYCLIC_MIXED_INTRINSIC_REVERSE_IMAGE_IDS
        }
        "definite-minmax-calc-mixed-text-image-span-forward" => {
            CYCLIC_MIXED_DEFINITE_FORWARD_IMAGE_IDS
        }
        "definite-minmax-calc-mixed-text-image-span-reverse" => {
            CYCLIC_MIXED_DEFINITE_REVERSE_IMAGE_IDS
        }
        "max-content-forward" => MAX_CONTENT_FORWARD_IMAGE_IDS,
        "max-content-reverse" => MAX_CONTENT_REVERSE_IMAGE_IDS,
        "definite-forward" => DEFINITE_FORWARD_IMAGE_IDS,
        "definite-reverse" => DEFINITE_REVERSE_IMAGE_IDS,
        _ => return Ok((scene, String::new())),
    };
    let expected_non_image_children =
        usize::from(!scene_id.starts_with("cyclic-minmax-replaced-image-span-")) * 2;
    let children = document
        .children(*root)
        .ok_or_else(|| "C11.4 bordered Grid child 목록이 없습니다".to_owned())?
        .collect::<Vec<_>>();
    if children.len() != image_ids.len() + expected_non_image_children {
        return Err("C11.4 bordered Grid direct child 구조가 다릅니다".to_owned());
    }

    let mut completions = Vec::new();
    let mut image_nodes = Vec::new();
    let mut child_images = children.iter().filter_map(|handle| {
        let node = document.node(*handle)?;
        let HostNodeKind::Element(element) = node.kind() else {
            return None;
        };
        (element.namespace() == HTML_NAMESPACE && element.local_name() == "img").then_some(*handle)
    });
    for expected_id in image_ids {
        let handle = child_images
            .next()
            .ok_or_else(|| format!("C11.4 image node가 없습니다: {expected_id}"))?;
        let node = document
            .node(handle)
            .ok_or_else(|| "C11.4 image node가 사라졌습니다".to_owned())?;
        let HostNodeKind::Element(element) = node.kind() else {
            return Err("C11.4 replaced item이 element가 아닙니다".to_owned());
        };
        let actual_id = element
            .attribute(&id_attribute)
            .map(|value| String::from_utf16_lossy(value.code_units()))
            .unwrap_or_default();
        if element.local_name() != "img" || actual_id != *expected_id {
            return Err(format!("C11.4 image order 또는 id가 다릅니다: {actual_id}"));
        }
        let request = ImageRequest::from_document(document, handle)
            .map_err(|error| format!("C11.4 image request 실패: {error}"))?;
        if request.src().code_units() != IMAGE_URL.encode_utf16().collect::<Vec<_>>().as_slice() {
            return Err(format!(
                "C11.4 image src가 고정 입력과 다릅니다: {actual_id}"
            ));
        }
        completions.push(
            request
                .decode(PNG)
                .map_err(|error| format!("C11.4 PNG decode 실패: {error}"))?,
        );
        image_nodes.push(handle.id());
    }
    if child_images.next().is_some() {
        return Err("C11.4 bordered Grid에 예상하지 않은 image가 있습니다".to_owned());
    }

    let resources =
        ImageResourceSnapshot::new(document, scene.key().intrinsic_size_revision(), completions)
            .map_err(|error| format!("C11.4 image resource snapshot 실패: {error}"))?;
    let source = RuntimeImageResourceKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        scene.key().intrinsic_size_revision(),
    );
    let mut images = Vec::new();
    let mut rows = Vec::new();
    for image_id in image_nodes {
        let decoded = resources
            .images()
            .get(&image_id)
            .ok_or_else(|| format!("C11.4 decoded image가 없습니다: {}", image_id.get()))?;
        if decoded.dimensions().width() != 24 || decoded.dimensions().height() != 16 {
            return Err("C11.4 PNG intrinsic size가 고정값 24x16과 다릅니다".to_owned());
        }
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == image_id)
            .ok_or_else(|| format!("C11.4 image frame이 없습니다: {}", image_id.get()))?;
        let frame = render_box.frame_css_px();
        let object_style = RuntimeImageObjectStyle::parse("fill", "50% 50%", "auto")
            .map_err(|error| format!("C11.4 default object style을 읽을 수 없습니다: {error}"))?;
        images.push(
            RuntimeRenderImage::new_with_object_style(
                image_id,
                source,
                decoded.dimensions().width(),
                decoded.dimensions().height(),
                Arc::from(decoded.rgba8()),
                object_style,
                scene.key().style_revision(),
            )
            .map_err(|error| {
                format!("C11.4 RuntimeRenderImage 입력이 유효하지 않습니다: {error}")
            })?,
        );
        rows.push(serde_json::json!({
            "nodeId": image_id.get(),
            "frameCssPx": {
                "x": frame.x(), "y": frame.y(),
                "width": frame.width(), "height": frame.height(),
            },
            "width": decoded.dimensions().width(),
            "height": decoded.dimensions().height(),
        }));
    }
    let scene = scene
        .with_images(images)
        .map_err(|error| format!("C11.4 bordered Grid image scene 결합 실패: {error}"))?;
    let report = serde_json::json!({
        "sceneId": scene_id,
        "intrinsicSizeRevision": scene.key().intrinsic_size_revision().get(),
        "images": rows,
    });
    let report = serde_json::to_string(&report)
        .map_err(|error| format!("C11.4 image report JSON 변환 실패: {error}"))?;
    Ok((
        scene,
        format!(" c11_4_bordered_grid_image_report_json={report}"),
    ))
}

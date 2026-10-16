use std::{collections::BTreeMap, sync::Arc};

use sha2::{Digest, Sha256};
use spinon_core::{AttributeName, HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_render::{
    RuntimeImageObjectStyle, RuntimeImageResourceKey, RuntimeRenderImage, RuntimeRenderSnapshot,
};
use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot};

pub(crate) const RUNTIME_CSS_C18_3_OBJECT_FIT_POSITION_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c18/runtime-image-object-fit-c18-3.js");

const PNG_BYTES: &[u8] =
    include_bytes!("../../../../../tests/fixtures/resources/c18-3-object-fit-pattern.png");
const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const IMAGE_SRC: &str = "spinon-fixture:c18-3/object-fit-pattern";
const EXPECTED_IMAGES: [ExpectedImage; 12] = [
    ExpectedImage::new("fill-center", 8.0, 8.0, 24.0, 24.0, "fill", "50% 50%"),
    ExpectedImage::new(
        "contain-center",
        40.0,
        8.0,
        24.0,
        24.0,
        "contain",
        "50% 50%",
    ),
    ExpectedImage::new(
        "contain-quarter",
        72.0,
        8.0,
        24.0,
        24.0,
        "contain",
        "25% 75%",
    ),
    ExpectedImage::new("cover-center", 104.0, 8.0, 24.0, 24.0, "cover", "50% 50%"),
    ExpectedImage::new("cover-start", 136.0, 8.0, 24.0, 24.0, "cover", "0% 0%"),
    ExpectedImage::new("cover-end", 168.0, 8.0, 24.0, 24.0, "cover", "100% 100%"),
    ExpectedImage::new("none-clip", 8.0, 40.0, 16.0, 16.0, "none", "100% 100%"),
    ExpectedImage::new(
        "scale-down-shrink",
        40.0,
        40.0,
        16.0,
        16.0,
        "scale-down",
        "50% 50%",
    ),
    ExpectedImage::new(
        "scale-down-no-upscale",
        72.0,
        40.0,
        32.0,
        20.0,
        "scale-down",
        "50% 50%",
    ),
    ExpectedImage::new("initial-values", 112.0, 40.0, 24.0, 12.0, "fill", "50% 50%"),
    ExpectedImage::new(
        "contain-extended-position",
        144.0,
        40.0,
        24.0,
        24.0,
        "contain",
        "-25% 150%",
    ),
    ExpectedImage::new(
        "cover-extended-position",
        176.0,
        40.0,
        24.0,
        24.0,
        "cover",
        "150% -25%",
    ),
];

const EXPECTED_SAMPLES: [ExpectedSample; 19] = [
    ExpectedSample::new("root-background-anchor", 2, 2, [17, 34, 51, 255]),
    ExpectedSample::new("fill-top-left", 9, 9, [24, 32, 232, 255]),
    ExpectedSample::new("fill-bottom-right", 30, 30, [209, 214, 51, 255]),
    ExpectedSample::new("contain-background-top", 42, 9, [210, 10, 180, 255]),
    ExpectedSample::new("contain-image-top", 42, 15, [24, 32, 232, 255]),
    ExpectedSample::new("contain-image-bottom", 42, 25, [24, 214, 206, 255]),
    ExpectedSample::new("contain-quarter-image", 75, 18, [24, 32, 232, 255]),
    ExpectedSample::new("cover-center-left", 105, 12, [61, 32, 201, 255]),
    ExpectedSample::new("cover-center-right", 126, 12, [172, 32, 108, 255]),
    ExpectedSample::new("cover-start-left", 137, 12, [24, 32, 232, 255]),
    ExpectedSample::new("cover-end-left", 169, 12, [135, 32, 139, 255]),
    ExpectedSample::new("none-clipped-object", 10, 45, [98, 32, 170, 255]),
    ExpectedSample::new("none-outside-content-box", 25, 45, [17, 34, 51, 255]),
    ExpectedSample::new("scale-down-shrink-background", 42, 41, [210, 10, 180, 255]),
    ExpectedSample::new("scale-down-shrink-image", 45, 46, [88, 56, 175, 255]),
    ExpectedSample::new("scale-down-no-upscale-image", 82, 47, [61, 32, 201, 255]),
    ExpectedSample::new("initial-values-natural", 113, 41, [24, 32, 232, 255]),
    ExpectedSample::new(
        "contain-extended-position-image",
        145,
        60,
        [24, 32, 232, 255],
    ),
    ExpectedSample::new(
        "cover-extended-position-image",
        177,
        44,
        [172, 32, 108, 255],
    ),
];

#[derive(Clone, Copy)]
struct ExpectedImage {
    id: &'static str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    fit: &'static str,
    position: &'static str,
}

impl ExpectedImage {
    const fn new(
        id: &'static str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        fit: &'static str,
        position: &'static str,
    ) -> Self {
        Self {
            id,
            x,
            y,
            width,
            height,
            fit,
            position,
        }
    }
}

#[derive(Clone, Copy)]
struct ExpectedSample {
    id: &'static str,
    x: u32,
    y: u32,
    rgba: [u8; 4],
}

impl ExpectedSample {
    const fn new(id: &'static str, x: u32, y: u32, rgba: [u8; 4]) -> Self {
        Self { id, x, y, rgba }
    }
}

pub(crate) fn attach_images(
    document: &HostDocumentSnapshot,
    scene: RuntimeRenderSnapshot,
    roots: &[spinon_runtime::RuntimeUaCascadeRoot],
) -> Result<(RuntimeRenderSnapshot, String), String> {
    let styles = current_styles(document, &scene, roots)?;
    let nodes = image_nodes(document)?;
    let mut completions = Vec::new();
    completions
        .try_reserve_exact(EXPECTED_IMAGES.len())
        .map_err(|_| "C18.3 image decode 목록을 할당하지 못했습니다".to_owned())?;
    for expected in EXPECTED_IMAGES {
        let handle = *nodes
            .get(expected.id)
            .ok_or_else(|| format!("C18.3 img 요소가 없습니다: {}", expected.id))?;
        let request = ImageRequest::from_document(document, handle)
            .map_err(|error| format!("C18.3 image request 실패: {error}"))?;
        if request.src().code_units() != IMAGE_SRC.encode_utf16().collect::<Vec<_>>().as_slice() {
            return Err(format!(
                "C18.3 img src가 고정 입력과 다릅니다: {}",
                expected.id
            ));
        }
        let completion = request
            .decode(PNG_BYTES)
            .map_err(|error| format!("C18.3 PNG decode 실패 {}: {error}", expected.id))?;
        completions.push(completion);
    }
    let resources =
        ImageResourceSnapshot::new(document, scene.key().intrinsic_size_revision(), completions)
            .map_err(|error| format!("C18.3 image resource snapshot 실패: {error}"))?;
    if resources.images().len() != EXPECTED_IMAGES.len() {
        return Err(format!(
            "C18.3 decoded image 수가 다릅니다: expected={} actual={}",
            EXPECTED_IMAGES.len(),
            resources.images().len()
        ));
    }
    let source = RuntimeImageResourceKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        scene.key().intrinsic_size_revision(),
    );
    let mut images = Vec::new();
    let mut rows = Vec::new();
    images
        .try_reserve_exact(EXPECTED_IMAGES.len())
        .map_err(|_| "C18.3 runtime image 목록을 할당하지 못했습니다".to_owned())?;
    rows.try_reserve_exact(EXPECTED_IMAGES.len())
        .map_err(|_| "C18.3 image report 목록을 할당하지 못했습니다".to_owned())?;
    let mut seen = BTreeMap::new();
    for (node_id, decoded) in resources.images() {
        let expected = EXPECTED_IMAGES
            .iter()
            .find(|expected| {
                nodes
                    .get(expected.id)
                    .is_some_and(|handle| handle.id() == *node_id)
            })
            .ok_or_else(|| format!("C18.3 decoded NodeId가 fixture와 다릅니다: {node_id}"))?;
        if seen.insert(expected.id, ()).is_some() {
            return Err(format!(
                "C18.3 decoded NodeId가 중복되었습니다: {}",
                expected.id
            ));
        }
        let computed = styles
            .elements
            .iter()
            .find(|style| style.node_id == *node_id)
            .ok_or_else(|| format!("C18.3 computed style이 없습니다: {}", expected.id))?;
        let fit = computed.properties.get("object-fit").ok_or_else(|| {
            format!(
                "C18.3 object-fit computed value가 없습니다: {}",
                expected.id
            )
        })?;
        let position = computed.properties.get("object-position").ok_or_else(|| {
            format!(
                "C18.3 object-position computed value가 없습니다: {}",
                expected.id
            )
        })?;
        let rendering = computed.properties.get("image-rendering").ok_or_else(|| {
            format!(
                "C18.3 image-rendering computed value가 없습니다: {}",
                expected.id
            )
        })?;
        if fit != expected.fit || position != expected.position || rendering != "auto" {
            return Err(format!(
                "C18.3 computed image style이 oracle과 다릅니다: id={} fit={} position={} rendering={}",
                expected.id, fit, position, rendering
            ));
        }
        if decoded.dimensions().width() != 24 || decoded.dimensions().height() != 12 {
            return Err(format!(
                "C18.3 PNG natural size가 다릅니다: id={} width={} height={}",
                expected.id,
                decoded.dimensions().width(),
                decoded.dimensions().height()
            ));
        }
        let object_style = RuntimeImageObjectStyle::parse(fit, position, rendering)
            .map_err(|error| format!("C18.3 object style 거부 {}: {error}", expected.id))?;
        let image = RuntimeRenderImage::new_with_object_style(
            *node_id,
            source,
            decoded.dimensions().width(),
            decoded.dimensions().height(),
            Arc::from(decoded.rgba8()),
            object_style,
            styles.style_revision,
        )
        .map_err(|error| format!("C18.3 runtime image 입력이 유효하지 않습니다: {error}"))?;
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == *node_id)
            .ok_or_else(|| format!("C18.3 image frame이 장면에 없습니다: {node_id}"))?;
        let frame = render_box.frame_css_px();
        if !same_geometry(frame.x(), expected.x)
            || !same_geometry(frame.y(), expected.y)
            || !same_geometry(frame.width(), expected.width)
            || !same_geometry(frame.height(), expected.height)
        {
            return Err(format!(
                "C18.3 image element frame이 oracle과 다릅니다: id={} actual=({},{},{},{}) expected=({},{},{},{})",
                expected.id,
                frame.x(),
                frame.y(),
                frame.width(),
                frame.height(),
                expected.x,
                expected.y,
                expected.width,
                expected.height
            ));
        }
        let object = image.object_rect_css_px(frame).map_err(|error| {
            format!("C18.3 concrete object geometry가 유효하지 않습니다: {error}")
        })?;
        let paint_clip = render_box.clip_css_px();
        rows.push(serde_json::json!({
            "id": expected.id,
            "nodeId": node_id.get(),
            "src": IMAGE_SRC,
            "width": decoded.dimensions().width(),
            "height": decoded.dimensions().height(),
            "encodedPngSha256": sha256_hex(PNG_BYTES),
            "decodedRgbaSha256": sha256_hex(decoded.rgba8()),
            "computed": {
                "objectFit": fit,
                "objectPosition": position,
                "imageRendering": rendering,
            },
            "frameCssPx": {
                "x": frame.x(), "y": frame.y(),
                "width": frame.width(), "height": frame.height(),
            },
            "objectCssPx": {
                "x": object.x(), "y": object.y(),
                "width": object.width(), "height": object.height(),
            },
            "paintClipCssPx": {
                "x": paint_clip.x().map(|axis| [axis.min(), axis.max()]),
                "y": paint_clip.y().map(|axis| [axis.min(), axis.max()]),
            },
        }));
        images.push(image);
    }
    if seen.len() != EXPECTED_IMAGES.len() {
        return Err(format!(
            "C18.3 decoded ID 집합이 다릅니다: expected={} actual={}",
            EXPECTED_IMAGES.len(),
            seen.len()
        ));
    }
    let scene = scene
        .with_images(images)
        .map_err(|error| format!("C18.3 image scene 결합 실패: {error}"))?;
    let report = serde_json::json!({
        "schema": "spinon-c18-3-object-fit-position/v1",
        "source": {
            "generation": document.generation().get(),
            "documentRevision": document.document_revision().get(),
            "renderTreeRevision": document.render_tree_revision().get(),
            "styleRevision": styles.style_revision.get(),
            "intrinsicSizeRevision": scene.key().intrinsic_size_revision().get(),
        },
        "fixture": {
            "imageCount": images_len(&scene),
            "encodedPngSha256": sha256_hex(PNG_BYTES),
            "oracleScreenshotSha256": "a7eb0ad22f995a3e44ec29ba644ce408c06346c2da1c95230c718f306baba5d4",
        },
        "images": rows,
        "oracleSamples": EXPECTED_SAMPLES.map(|sample| serde_json::json!({
            "id": sample.id,
            "x": sample.x,
            "y": sample.y,
            "rgba": sample.rgba,
        })),
    });
    let report = serde_json::to_string(&report)
        .map_err(|error| format!("C18.3 image report JSON 변환 실패: {error}"))?;
    Ok((scene, format!(" c18_3_image_report_json={report}")))
}

fn images_len(scene: &RuntimeRenderSnapshot) -> usize {
    scene.images().len()
}

fn current_styles<'a>(
    document: &HostDocumentSnapshot,
    scene: &RuntimeRenderSnapshot,
    roots: &'a [spinon_runtime::RuntimeUaCascadeRoot],
) -> Result<&'a ComputedStyleSnapshot, String> {
    if roots.len() != 1 {
        return Err(format!(
            "C18.3 computed style root 수가 다릅니다: {}",
            roots.len()
        ));
    }
    let styles = &roots[0].styles;
    if styles.profile != ComputedStyleProfile::RuntimeBlockImageV1
        || styles.generation != document.generation()
        || styles.document_revision != document.document_revision()
        || styles.render_tree_revision != document.render_tree_revision()
        || styles.generation != scene.key().generation()
        || styles.document_revision != scene.key().document_revision()
        || styles.render_tree_revision != scene.key().render_tree_revision()
        || styles.style_revision != scene.key().style_revision()
    {
        return Err("C18.3 style/document/scene revision tuple이 일치하지 않습니다".to_owned());
    }
    Ok(styles)
}

fn image_nodes(
    document: &HostDocumentSnapshot,
) -> Result<BTreeMap<&'static str, HostNodeHandle>, String> {
    let id_attribute = AttributeName::new(None, "id")
        .ok_or_else(|| "C18.3 id attribute 이름을 만들 수 없습니다".to_owned())?;
    let mut found = BTreeMap::new();
    let mut image_count = 0;
    let mut pending = document.root_children().collect::<Vec<_>>();
    while let Some(handle) = pending.pop() {
        let node = document
            .node(handle)
            .ok_or_else(|| "C18.3 V8 document traversal node가 사라졌습니다".to_owned())?;
        if let HostNodeKind::Element(element) = node.kind()
            && element.namespace() == HTML_NAMESPACE
            && element.local_name() == "img"
        {
            image_count += 1;
            let id = EXPECTED_IMAGES.iter().find(|expected| {
                element.attribute(&id_attribute).is_some_and(|value| {
                    value.code_units() == expected.id.encode_utf16().collect::<Vec<_>>().as_slice()
                })
            });
            let Some(expected) = id else {
                return Err("C18.3 fixture에 예상하지 않은 img ID가 있습니다".to_owned());
            };
            if found.insert(expected.id, handle).is_some() {
                return Err(format!("C18.3 img id가 중복되었습니다: {}", expected.id));
            }
        }
        if let Some(children) = document.children(handle) {
            pending.extend(children);
        }
    }
    if image_count != EXPECTED_IMAGES.len() || found.len() != EXPECTED_IMAGES.len() {
        return Err(format!(
            "C18.3 img 수가 다릅니다: expected={} actual={image_count}",
            EXPECTED_IMAGES.len()
        ));
    }
    Ok(found)
}

fn same_geometry(actual: f32, expected: f32) -> bool {
    actual.is_finite() && (actual - expected).abs() <= 0.001
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

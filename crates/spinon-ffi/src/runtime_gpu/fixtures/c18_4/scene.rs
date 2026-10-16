use std::{collections::BTreeMap, sync::Arc, thread};

use sha2::{Digest, Sha256};
use spinon_core::{AttributeName, HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_render::{
    RuntimeImageObjectStyle, RuntimeImageResourceKey, RuntimeRenderImage, RuntimeRenderSnapshot,
};
use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot};

use super::{fixture::*, run_transition_sequence};

pub(crate) fn attach_images(
    document: &HostDocumentSnapshot,
    scene: RuntimeRenderSnapshot,
    roots: &[spinon_runtime::RuntimeUaCascadeRoot],
) -> Result<(RuntimeRenderSnapshot, String), String> {
    let styles = current_styles(document, &scene, roots)?;
    let nodes = image_nodes(document)?;
    let mut pending = None;
    let mut workers = Vec::new();
    workers
        .try_reserve_exact(EXPECTED_IMAGES.len() - 1)
        .map_err(|_| "C18.4 decode task 목록을 할당하지 못했습니다".to_owned())?;
    for expected in EXPECTED_IMAGES {
        let handle = *nodes
            .get(expected.id)
            .ok_or_else(|| format!("C18.4 img 요소가 없습니다: {}", expected.id))?;
        let request = ImageRequest::from_document(document, handle)
            .map_err(|error| format!("C18.4 image request 실패: {error}"))?;
        if request.src().to_string_lossy() != expected.src {
            return Err(format!(
                "C18.4 img src가 고정 입력과 다릅니다: id={} src={}",
                expected.id,
                request.src().to_string_lossy()
            ));
        }
        let Some(bytes) = expected.bytes else {
            pending = Some(handle);
            continue;
        };
        workers.push((
            expected.id,
            handle.id(),
            thread::spawn(move || request.decode_outcome(bytes)),
        ));
    }
    let mut outcomes = Vec::new();
    outcomes
        .try_reserve_exact(workers.len())
        .map_err(|_| "C18.4 decode outcome 목록을 할당하지 못했습니다".to_owned())?;
    for (id, _, worker) in workers {
        outcomes.push(
            worker
                .join()
                .map_err(|_| format!("C18.4 실제 PNG decoder task가 panic했습니다: {id}"))?,
        );
    }
    let (resources, rejected) = ImageResourceSnapshot::new_with_outcomes(
        document,
        scene.key().intrinsic_size_revision(),
        outcomes,
    )
    .map_err(|error| format!("C18.4 resource snapshot 실패: {error}"))?;
    if !rejected.is_empty() || resources.images().len() != 3 || resources.failures().len() != 1 {
        return Err(format!(
            "C18.4 결과 집합이 fixture와 다릅니다: images={} failures={} stale={}",
            resources.images().len(),
            resources.failures().len(),
            rejected.len()
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
        .try_reserve_exact(resources.images().len())
        .map_err(|_| "C18.4 runtime image 목록을 할당하지 못했습니다".to_owned())?;
    for (node_id, decoded) in resources.images() {
        let expected = EXPECTED_IMAGES
            .iter()
            .find(|entry| {
                nodes
                    .get(entry.id)
                    .is_some_and(|node| node.id() == *node_id)
            })
            .ok_or_else(|| format!("C18.4 decoded NodeId가 fixture와 다릅니다: {node_id}"))?;
        if expected.bytes.is_none() {
            return Err(format!(
                "C18.4 pending node가 decode됐습니다: {}",
                expected.id
            ));
        }
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == *node_id)
            .ok_or_else(|| format!("C18.4 image frame이 장면에 없습니다: {node_id}"))?;
        let frame = render_box.frame_css_px();
        if !same_geometry(frame.x(), expected.x)
            || !same_geometry(frame.y(), expected.y)
            || !same_geometry(frame.width(), 48.0)
            || !same_geometry(frame.height(), 32.0)
        {
            return Err(format!(
                "C18.4 fixed frame이 다릅니다: id={} actual=({},{},{},{})",
                expected.id,
                frame.x(),
                frame.y(),
                frame.width(),
                frame.height()
            ));
        }
        let computed = styles
            .elements
            .iter()
            .find(|style| style.node_id == *node_id)
            .ok_or_else(|| format!("C18.4 computed style이 없습니다: {}", expected.id))?;
        let fit = computed.properties.get("object-fit").ok_or_else(|| {
            format!(
                "C18.4 object-fit computed value가 없습니다: {}",
                expected.id
            )
        })?;
        let position = computed.properties.get("object-position").ok_or_else(|| {
            format!(
                "C18.4 object-position computed value가 없습니다: {}",
                expected.id
            )
        })?;
        let rendering = computed.properties.get("image-rendering").ok_or_else(|| {
            format!(
                "C18.4 image-rendering computed value가 없습니다: {}",
                expected.id
            )
        })?;
        if fit != "fill" || position != "50% 50%" || rendering != "auto" {
            return Err(format!(
                "C18.4 computed image style이 fixture와 다릅니다: id={} fit={} position={} rendering={}",
                expected.id, fit, position, rendering
            ));
        }
        let object_style = RuntimeImageObjectStyle::parse(fit, position, rendering)
            .map_err(|error| format!("C18.4 object style 거부 {}: {error}", expected.id))?;
        images.push(
            RuntimeRenderImage::new_with_object_style(
                *node_id,
                source,
                decoded.dimensions().width(),
                decoded.dimensions().height(),
                Arc::from(decoded.rgba8()),
                object_style,
                styles.style_revision,
            )
            .map_err(|error| format!("C18.4 runtime image 입력이 유효하지 않습니다: {error}"))?,
        );
        rows.push(serde_json::json!({
            "id": expected.id,
            "nodeId": node_id.get(),
            "src": expected.src,
            "naturalSize": [decoded.dimensions().width(), decoded.dimensions().height()],
            "computed": {
                "objectFit": fit,
                "objectPosition": position,
                "imageRendering": rendering,
            },
            "encodedPngSha256": expected.bytes.map(sha256_hex),
            "decodedRgbaSha256": sha256_hex(decoded.rgba8()),
            "frameCssPx": {
                "x": frame.x(), "y": frame.y(),
                "width": frame.width(), "height": frame.height(),
            },
        }));
    }
    let pending = pending.ok_or_else(|| "C18.4 pending img 요소가 없습니다".to_owned())?;
    let scene = scene
        .with_images(images)
        .map_err(|error| format!("C18.4 image scene 결합 실패: {error}"))?;
    let transition = run_transition_sequence()?;
    let report = serde_json::json!({
        "schema": "spinon-c18-4-async-image-replacement/v1",
        "source": {
            "generation": document.generation().get(),
            "documentRevision": document.document_revision().get(),
            "renderTreeRevision": document.render_tree_revision().get(),
            "styleRevision": styles.style_revision.get(),
            "intrinsicSizeRevision": scene.key().intrinsic_size_revision().get(),
        },
        "fixture": {
            "imageCount": scene.images().len(),
            "failureCount": resources.failures().len(),
            "pendingNodeId": pending.id().get(),
            "encodedPngSha256": EXPECTED_IMAGES.iter().filter_map(|entry| {
                entry.bytes.map(|bytes| serde_json::json!({
                    "src": entry.src,
                    "sha256": sha256_hex(bytes),
                }))
            }).collect::<Vec<_>>(),
        },
        "transition": serde_json::from_str::<serde_json::Value>(&transition)
            .map_err(|error| format!("C18.4 transition JSON 검증 실패: {error}"))?,
        "images": rows,
        "oracleSamples": EXPECTED_SAMPLES.map(|sample| serde_json::json!({
            "id": sample.id,
            "x": sample.x,
            "y": sample.y,
            "rgba": sample.rgba,
        })),
    });
    let report = serde_json::to_string(&report)
        .map_err(|error| format!("C18.4 image report JSON 변환 실패: {error}"))?;
    Ok((scene, format!(" c18_4_image_report_json={report}")))
}

fn current_styles<'a>(
    document: &HostDocumentSnapshot,
    scene: &RuntimeRenderSnapshot,
    roots: &'a [spinon_runtime::RuntimeUaCascadeRoot],
) -> Result<&'a ComputedStyleSnapshot, String> {
    if roots.len() != 1 {
        return Err(format!(
            "C18.4 computed style root 수가 다릅니다: {}",
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
        return Err("C18.4 style/document/scene revision tuple이 일치하지 않습니다".to_owned());
    }
    Ok(styles)
}

fn image_nodes(
    document: &HostDocumentSnapshot,
) -> Result<BTreeMap<&'static str, HostNodeHandle>, String> {
    let id_attribute = AttributeName::new(None, "id")
        .ok_or_else(|| "C18.4 id attribute 이름을 만들 수 없습니다".to_owned())?;
    let mut found = BTreeMap::new();
    let mut image_count = 0;
    let mut pending = document.root_children().collect::<Vec<_>>();
    while let Some(handle) = pending.pop() {
        let node = document
            .node(handle)
            .ok_or_else(|| "C18.4 V8 document traversal node가 사라졌습니다".to_owned())?;
        if let HostNodeKind::Element(element) = node.kind()
            && element.namespace() == HTML_NAMESPACE
            && element.local_name() == "img"
        {
            image_count += 1;
            let Some(expected) = element.attribute(&id_attribute).and_then(|value| {
                EXPECTED_IMAGES.iter().find(|expected| {
                    value.code_units() == expected.id.encode_utf16().collect::<Vec<_>>()
                })
            }) else {
                return Err("C18.4 fixture에 예상하지 않은 img ID가 있습니다".to_owned());
            };
            if found.insert(expected.id, handle).is_some() {
                return Err(format!("C18.4 img id가 중복되었습니다: {}", expected.id));
            }
        }
        if let Some(children) = document.children(handle) {
            pending.extend(children);
        }
    }
    if image_count != EXPECTED_IMAGES.len() || found.len() != EXPECTED_IMAGES.len() {
        return Err(format!(
            "C18.4 img 수가 fixture와 다릅니다: expected={} actual={image_count}",
            EXPECTED_IMAGES.len()
        ));
    }
    Ok(found)
}

fn same_geometry(actual: f32, expected: f32) -> bool {
    (actual - expected).abs() <= 0.001
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

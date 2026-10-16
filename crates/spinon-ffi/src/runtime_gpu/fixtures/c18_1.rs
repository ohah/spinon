use std::{collections::BTreeMap, sync::Arc};

use sha2::{Digest, Sha256};
use spinon_core::{AttributeName, HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_render::{RuntimeImageResourceKey, RuntimeRenderImage, RuntimeRenderSnapshot};

pub(crate) const RUNTIME_CSS_C18_1_IMAGE_PAINT_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c18/runtime-image-overflow-c18-1.js");

const PNG_BYTES: &[u8] =
    include_bytes!("../../../../../tests/fixtures/resources/c14-1-png-basic.png");
const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const IMAGE_IDS: [(&str, &str); 2] = [
    (
        "c18-visible-image",
        "spinon-fixture:c18-1/static-png-visible",
    ),
    ("c18-hidden-image", "spinon-fixture:c18-1/static-png-hidden"),
];

pub(crate) fn attach_images(
    document: &HostDocumentSnapshot,
    scene: RuntimeRenderSnapshot,
) -> Result<(RuntimeRenderSnapshot, String), String> {
    let nodes = image_nodes(document)?;
    if nodes.len() != IMAGE_IDS.len() {
        return Err("C18.1 fixture는 정확히 두 개의 img 요소를 요구합니다".to_owned());
    }
    let mut completions = Vec::new();
    completions
        .try_reserve_exact(IMAGE_IDS.len())
        .map_err(|_| "C18.1 이미지 decode 목록을 할당하지 못했습니다".to_owned())?;
    for (id, src) in IMAGE_IDS {
        let handle = *nodes
            .get(id)
            .ok_or_else(|| format!("C18.1 img 요소가 없습니다: {id}"))?;
        let request = ImageRequest::from_document(document, handle)
            .map_err(|error| format!("C18.1 image request 실패: {error}"))?;
        if request.src().code_units() != src.encode_utf16().collect::<Vec<_>>().as_slice() {
            return Err(format!("C18.1 img src가 고정 입력과 다릅니다: {id}"));
        }
        completions.push(
            request
                .decode(PNG_BYTES)
                .map_err(|error| format!("C18.1 PNG decode 실패: {error}"))?,
        );
    }
    let resources =
        ImageResourceSnapshot::new(document, scene.key().intrinsic_size_revision(), completions)
            .map_err(|error| format!("C18.1 image resource snapshot 실패: {error}"))?;
    let source = RuntimeImageResourceKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        scene.key().intrinsic_size_revision(),
    );
    let encoded_sha256 = sha256_hex(PNG_BYTES);
    let mut images = Vec::new();
    let mut rows = Vec::new();
    images
        .try_reserve_exact(resources.images().len())
        .map_err(|_| "C18.1 runtime image 목록을 할당하지 못했습니다".to_owned())?;
    rows.try_reserve_exact(resources.images().len())
        .map_err(|_| "C18.1 image report 목록을 할당하지 못했습니다".to_owned())?;
    for (node_id, decoded) in resources.images() {
        let image = RuntimeRenderImage::new(
            *node_id,
            source,
            decoded.dimensions().width(),
            decoded.dimensions().height(),
            Arc::from(decoded.rgba8()),
        )
        .map_err(|error| format!("C18.1 runtime image 입력이 유효하지 않습니다: {error}"))?;
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == *node_id)
            .ok_or_else(|| format!("C18.1 image frame이 장면에 없습니다: {node_id}"))?;
        let frame = render_box.frame_css_px();
        let paint_clip = render_box.clip_css_px();
        let decoded_sha256 = sha256_hex(decoded.rgba8());
        let src = IMAGE_IDS
            .iter()
            .find(|(id, _)| nodes.get(*id).is_some_and(|handle| handle.id() == *node_id))
            .map(|(_, src)| *src)
            .ok_or_else(|| format!("C18.1 예상하지 않은 image node입니다: {node_id}"))?;
        rows.push(serde_json::json!({
            "nodeId": node_id.get(),
            "src": src,
            "width": decoded.dimensions().width(),
            "height": decoded.dimensions().height(),
            "decodedRgbaSha256": decoded_sha256,
            "paintClipCssPx": {
                "x": paint_clip.x().map(|axis| [axis.min(), axis.max()]),
                "y": paint_clip.y().map(|axis| [axis.min(), axis.max()]),
            },
            "frameCssPx": {
                "x": frame.x(),
                "y": frame.y(),
                "width": frame.width(),
                "height": frame.height(),
            },
        }));
        images.push(image);
    }
    let scene = scene
        .with_images(images)
        .map_err(|error| format!("C18.1 image scene 결합 실패: {error}"))?;
    let report = serde_json::json!({
        "schema": "spinon-c18-1-image-paint/v1",
        "source": {
            "generation": document.generation().get(),
            "documentRevision": document.document_revision().get(),
            "renderTreeRevision": document.render_tree_revision().get(),
            "intrinsicSizeRevision": scene.key().intrinsic_size_revision().get(),
        },
        "encodedPngSha256": encoded_sha256,
        "images": rows,
    });
    let report = serde_json::to_string(&report)
        .map_err(|error| format!("C18.1 image report JSON 변환 실패: {error}"))?;
    Ok((scene, format!(" c18_1_image_report_json={report}")))
}

fn image_nodes(
    document: &HostDocumentSnapshot,
) -> Result<BTreeMap<&'static str, HostNodeHandle>, String> {
    let id_attribute = AttributeName::new(None, "id")
        .ok_or_else(|| "C18.1 id attribute 이름을 만들 수 없습니다".to_owned())?;
    let expected = IMAGE_IDS
        .iter()
        .map(|(id, _)| (*id, id.encode_utf16().collect::<Vec<_>>()))
        .collect::<BTreeMap<_, _>>();
    let mut found = BTreeMap::new();
    let mut image_count = 0;
    let mut pending = document.root_children().collect::<Vec<_>>();
    while let Some(handle) = pending.pop() {
        let node = document
            .node(handle)
            .ok_or_else(|| "C18.1 V8 document에서 traversal node가 사라졌습니다".to_owned())?;
        if let HostNodeKind::Element(element) = node.kind()
            && element.namespace() == HTML_NAMESPACE
            && element.local_name() == "img"
        {
            image_count += 1;
            let value = element.attribute(&id_attribute);
            let matching_id = expected.iter().find_map(|(id, units)| {
                value
                    .is_some_and(|value| value.code_units() == units.as_slice())
                    .then_some(*id)
            });
            let Some(id) = matching_id else {
                return Err("C18.1 fixture에 예상하지 않은 img 요소가 있습니다".to_owned());
            };
            if found.insert(id, handle).is_some() {
                return Err(format!("C18.1 img id가 중복되었습니다: {id}"));
            }
        }
        if let Some(children) = document.children(handle) {
            pending.extend(children);
        }
    }
    if image_count != IMAGE_IDS.len() {
        return Err(format!(
            "C18.1 img 수가 다릅니다: expected={} actual={image_count}",
            IMAGE_IDS.len()
        ));
    }
    Ok(found)
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{HTML_NAMESPACE, IMAGE_IDS, attach_images};
    use spinon_core::{
        AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
        HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
    };
    use spinon_render::{
        CssRect, CssSize, RuntimePaint, RuntimeRenderBox, RuntimeRenderKey, RuntimeRenderSnapshot,
    };

    fn document_with_images(images: &[(&str, &str)]) -> (HostDocument, Vec<spinon_core::NodeId>) {
        let mut document = HostDocument::new().unwrap();
        let owner = OwnerId::new(1).unwrap();
        let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
        let mut node_ids = Vec::new();
        for (id, src) in images {
            let node = document.reserve_node_handle().unwrap();
            node_ids.push(node.id());
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML_NAMESPACE.to_owned(),
                local_name: "img".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "id").unwrap(),
                value: (*id).to_owned().into(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "src").unwrap(),
                value: (*src).to_owned().into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Root,
                node,
                before: None,
            });
        }
        document.commit(batch).unwrap();
        (document, node_ids)
    }

    fn scene_for(
        document: &HostDocument,
        node_ids: &[spinon_core::NodeId],
    ) -> RuntimeRenderSnapshot {
        let source = document.snapshot();
        let intrinsic_revision = IntrinsicSizeRevision::INITIAL.checked_next().unwrap();
        let key = RuntimeRenderKey::new(
            source.generation(),
            source.document_revision(),
            source.render_tree_revision(),
            StyleRevision::INITIAL,
            EnvironmentRevision::INITIAL,
        )
        .with_intrinsic_size_revision(intrinsic_revision);
        let boxes = node_ids
            .iter()
            .enumerate()
            .map(|(index, node_id)| {
                RuntimeRenderBox::new(
                    *node_id,
                    CssRect::new(5.0 + index as f32 * 30.0, 5.0, 24.0, 16.0).unwrap(),
                    RuntimePaint::None,
                    index as u32,
                )
            })
            .collect();
        RuntimeRenderSnapshot::new(key, CssSize::new(80.0, 40.0).unwrap(), boxes).unwrap()
    }

    fn valid_sources() -> [(&'static str, &'static str); 2] {
        IMAGE_IDS
    }

    #[test]
    fn fixture_attachment_accepts_only_the_two_pinned_image_ids_and_sources() {
        let sources = valid_sources();
        let (document, node_ids) = document_with_images(&sources);
        let (scene, report) = attach_images(&document.snapshot(), scene_for(&document, &node_ids))
            .expect("fixed live-document inputs");
        assert_eq!(scene.images().len(), 2);
        assert!(report.contains("spinon-c18-1-image-paint/v1"));

        let (missing_document, missing_nodes) = document_with_images(&sources[..1]);
        assert!(
            attach_images(
                &missing_document.snapshot(),
                scene_for(&missing_document, &missing_nodes),
            )
            .is_err()
        );

        let unexpected = [sources[0], ("unexpected-image", sources[1].1)];
        let (unexpected_document, unexpected_nodes) = document_with_images(&unexpected);
        assert!(
            attach_images(
                &unexpected_document.snapshot(),
                scene_for(&unexpected_document, &unexpected_nodes),
            )
            .is_err()
        );

        let duplicate = [sources[0], sources[0]];
        let (duplicate_document, duplicate_nodes) = document_with_images(&duplicate);
        assert!(
            attach_images(
                &duplicate_document.snapshot(),
                scene_for(&duplicate_document, &duplicate_nodes),
            )
            .is_err()
        );

        let changed_source = [sources[0], (sources[1].0, "spinon-fixture:c18-1/changed")];
        let (changed_document, changed_nodes) = document_with_images(&changed_source);
        assert!(
            attach_images(
                &changed_document.snapshot(),
                scene_for(&changed_document, &changed_nodes),
            )
            .is_err()
        );
    }
}

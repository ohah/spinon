use std::{collections::BTreeMap, sync::Arc};

use sha2::{Digest, Sha256};
use spinon_core::{AttributeName, HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_render::{RuntimeImageResourceKey, RuntimeRenderImage, RuntimeRenderSnapshot};

pub(crate) const RUNTIME_CSS_C18_2_ALPHA_IMAGE_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c18/runtime-image-alpha-c18-2.js");

const PNG_BYTES: &[u8] =
    include_bytes!("../../../../../tests/fixtures/resources/c18-2-alpha-source-over.png");
const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const IMAGE_ID: &str = "c18-alpha-image";
const IMAGE_SRC: &str = "spinon-fixture:c18-2/alpha-source-over";

pub(crate) fn attach_image(
    document: &HostDocumentSnapshot,
    scene: RuntimeRenderSnapshot,
) -> Result<(RuntimeRenderSnapshot, String), String> {
    let nodes = image_nodes(document)?;
    let handle = *nodes
        .get(IMAGE_ID)
        .ok_or_else(|| "C18.2 img 요소가 없습니다".to_owned())?;
    let request = ImageRequest::from_document(document, handle)
        .map_err(|error| format!("C18.2 image request 실패: {error}"))?;
    if request.src().code_units() != IMAGE_SRC.encode_utf16().collect::<Vec<_>>().as_slice() {
        return Err("C18.2 img src가 고정 입력과 다릅니다".to_owned());
    }
    let completion = request
        .decode(PNG_BYTES)
        .map_err(|error| format!("C18.2 PNG decode 실패: {error}"))?;
    let resources = ImageResourceSnapshot::new(
        document,
        scene.key().intrinsic_size_revision(),
        vec![completion],
    )
    .map_err(|error| format!("C18.2 image resource snapshot 실패: {error}"))?;
    let source = RuntimeImageResourceKey::new(
        document.generation(),
        document.document_revision(),
        document.render_tree_revision(),
        scene.key().intrinsic_size_revision(),
    );
    let mut images = Vec::new();
    let mut rows = Vec::new();
    images
        .try_reserve_exact(1)
        .map_err(|_| "C18.2 runtime image 목록을 할당하지 못했습니다".to_owned())?;
    rows.try_reserve_exact(1)
        .map_err(|_| "C18.2 image report 목록을 할당하지 못했습니다".to_owned())?;
    for (node_id, decoded) in resources.images() {
        let image = RuntimeRenderImage::new(
            *node_id,
            source,
            decoded.dimensions().width(),
            decoded.dimensions().height(),
            Arc::from(decoded.rgba8()),
        )
        .map_err(|error| format!("C18.2 runtime image 입력이 유효하지 않습니다: {error}"))?;
        let render_box = scene
            .boxes()
            .iter()
            .find(|render_box| render_box.node_id() == *node_id)
            .ok_or_else(|| format!("C18.2 image frame이 장면에 없습니다: {node_id}"))?;
        let frame = render_box.frame_css_px();
        let paint_clip = render_box.clip_css_px();
        let src_sha256 = sha256_hex(PNG_BYTES);
        let decoded_sha256 = sha256_hex(decoded.rgba8());
        rows.push(serde_json::json!({
            "nodeId": node_id.get(),
            "src": IMAGE_SRC,
            "width": decoded.dimensions().width(),
            "height": decoded.dimensions().height(),
            "encodedPngSha256": src_sha256,
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
        .map_err(|error| format!("C18.2 image scene 결합 실패: {error}"))?;
    let report = serde_json::json!({
        "schema": "spinon-c18-2-alpha-image-compositing/v1",
        "source": {
            "generation": document.generation().get(),
            "documentRevision": document.document_revision().get(),
            "renderTreeRevision": document.render_tree_revision().get(),
            "intrinsicSizeRevision": scene.key().intrinsic_size_revision().get(),
        },
        "images": rows,
    });
    let report = serde_json::to_string(&report)
        .map_err(|error| format!("C18.2 image report JSON 변환 실패: {error}"))?;
    Ok((scene, format!(" c18_2_image_report_json={report}")))
}

fn image_nodes(
    document: &HostDocumentSnapshot,
) -> Result<BTreeMap<&'static str, HostNodeHandle>, String> {
    let id_attribute = AttributeName::new(None, "id")
        .ok_or_else(|| "C18.2 id attribute 이름을 만들 수 없습니다".to_owned())?;
    let expected_id = IMAGE_ID.encode_utf16().collect::<Vec<_>>();
    let mut found = BTreeMap::new();
    let mut image_count = 0;
    let mut pending = document.root_children().collect::<Vec<_>>();
    while let Some(handle) = pending.pop() {
        let node = document
            .node(handle)
            .ok_or_else(|| "C18.2 V8 document traversal node가 사라졌습니다".to_owned())?;
        if let HostNodeKind::Element(element) = node.kind()
            && element.namespace() == HTML_NAMESPACE
            && element.local_name() == "img"
        {
            image_count += 1;
            let matches = element
                .attribute(&id_attribute)
                .is_some_and(|value| value.code_units() == expected_id.as_slice());
            if !matches {
                return Err("C18.2 fixture에 예상하지 않은 img 요소가 있습니다".to_owned());
            }
            if found.insert(IMAGE_ID, handle).is_some() {
                return Err("C18.2 img id가 중복되었습니다".to_owned());
            }
        }
        if let Some(children) = document.children(handle) {
            pending.extend(children);
        }
    }
    if image_count != 1 {
        return Err(format!(
            "C18.2 img 수가 다릅니다: expected=1 actual={image_count}"
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
    use super::{HTML_NAMESPACE, IMAGE_ID, IMAGE_SRC, attach_image};
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
                    CssRect::new(8.0, 12.0, 24.0, 16.0).unwrap(),
                    RuntimePaint::None,
                    index as u32,
                )
            })
            .collect();
        RuntimeRenderSnapshot::new(key, CssSize::new(80.0, 40.0).unwrap(), boxes).unwrap()
    }

    #[test]
    fn fixture_attachment_accepts_only_the_single_pinned_image_id_and_source() {
        let source = [(IMAGE_ID, IMAGE_SRC)];
        let (document, node_ids) = document_with_images(&source);
        let (scene, report) = attach_image(&document.snapshot(), scene_for(&document, &node_ids))
            .expect("fixed live-document input");
        assert_eq!(scene.images().len(), 1);
        assert!(report.contains("spinon-c18-2-alpha-image-compositing/v1"));
        let rgba = scene.images()[0].rgba8();
        assert_eq!(&rgba[0..4], &[250, 20, 210, 0]);
        assert_eq!(&rgba[8 * 4..9 * 4], &[220, 40, 80, 128]);
        assert_eq!(&rgba[16 * 4..17 * 4], &[32, 200, 64, 255]);

        let (missing_document, missing_nodes) = document_with_images(&[]);
        assert!(
            attach_image(
                &missing_document.snapshot(),
                scene_for(&missing_document, &missing_nodes),
            )
            .is_err()
        );

        let (unexpected_document, unexpected_nodes) =
            document_with_images(&[("unexpected-image", IMAGE_SRC)]);
        assert!(
            attach_image(
                &unexpected_document.snapshot(),
                scene_for(&unexpected_document, &unexpected_nodes),
            )
            .is_err()
        );

        let (duplicate_document, duplicate_nodes) =
            document_with_images(&[(IMAGE_ID, IMAGE_SRC), (IMAGE_ID, IMAGE_SRC)]);
        assert!(
            attach_image(
                &duplicate_document.snapshot(),
                scene_for(&duplicate_document, &duplicate_nodes),
            )
            .is_err()
        );

        let (changed_document, changed_nodes) =
            document_with_images(&[(IMAGE_ID, "spinon-fixture:c18-2/changed")]);
        assert!(
            attach_image(
                &changed_document.snapshot(),
                scene_for(&changed_document, &changed_nodes),
            )
            .is_err()
        );
    }
}

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostDocumentSnapshot,
    HostNodeHandle, HostParent, IntrinsicSizeRevision, OwnerId, StyleRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_style::{
    CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade_with_stylesheets,
};
use spinon_style_to_layout::compute_runtime_style_layout_with_intrinsic_size_snapshot;
use std::{
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

mod fixture;
mod scene;
#[cfg(test)]
mod tests;

use fixture::*;

pub(crate) const RUNTIME_CSS_C18_4_ASYNC_IMAGE_FIXTURE_SOURCE: &str =
    include_str!("../../../../../tests/fixtures/css/c18/runtime-image-async-replacement-c18-4.js");

pub(crate) use scene::attach_images;

/// 실제 decoder와 gated worker를 사용해 교체·실패·복구 순서를 재생합니다.
pub(crate) fn run_transition_sequence() -> Result<String, String> {
    let (mut document, image) = transition_document(A_SRC)?;
    let a_document = document.snapshot();
    let a_request = ImageRequest::from_document(&a_document, image)
        .map_err(|error| format!("C18.4 느린 A request 생성 실패: {error}"))?;

    let (release_a, wait_a) = mpsc::sync_channel(0);
    let (send_a, receive_a) = mpsc::channel();
    let worker_a = thread::spawn(move || {
        if wait_a.recv().is_ok() {
            let _ = send_a.send(a_request.decode_outcome(A_PNG));
        }
    });
    let current_b_document = set_source(&mut document, image, B_SRC)?;
    let b_request = ImageRequest::from_document(&current_b_document, image)
        .map_err(|error| format!("C18.4 빠른 B request 생성 실패: {error}"))?;
    let (send_b, receive_b) = mpsc::channel();
    let worker_b = thread::spawn(move || {
        let _ = send_b.send(b_request.decode_outcome(B_PNG));
    });
    let b_outcome = receive_b
        .recv_timeout(Duration::from_secs(2))
        .map_err(|error| format!("C18.4 빠른 B decode task가 완료되지 않았습니다: {error}"))?;
    worker_b
        .join()
        .map_err(|_| "C18.4 빠른 B decode task가 panic했습니다".to_owned())?;
    release_a
        .send(())
        .map_err(|_| "C18.4 느린 A decode worker가 취소됐습니다".to_owned())?;
    let a_outcome = receive_a
        .recv_timeout(Duration::from_secs(2))
        .map_err(|error| format!("C18.4 늦은 A decode task가 완료되지 않았습니다: {error}"))?;
    worker_a
        .join()
        .map_err(|_| "C18.4 느린 A decode task가 panic했습니다".to_owned())?;
    let (b_snapshot, rejected_a) = ImageResourceSnapshot::new_with_outcomes(
        &current_b_document,
        IntrinsicSizeRevision::INITIAL,
        [b_outcome, a_outcome],
    )
    .map_err(|error| format!("C18.4 stale A 거부 뒤 B snapshot 실패: {error}"))?;
    if b_snapshot.images().len() != 1
        || !b_snapshot.failures().is_empty()
        || rejected_a.len() != 1
        || rejected_a[0].request().src().to_string_lossy() != A_SRC
    {
        return Err("C18.4 늦은 A 결과가 빠른 B resource를 덮었습니다".to_owned());
    }

    let current_c_document = set_source(&mut document, image, C_SRC)?;
    let c_request = ImageRequest::from_document(&current_c_document, image)
        .map_err(|error| format!("C18.4 C pending request 생성 실패: {error}"))?;
    let (c_pending, stale_c) = b_snapshot
        .replace_with_outcomes(&current_c_document, [])
        .map_err(|error| format!("C18.4 C pending snapshot 실패: {error}"))?;
    let c_pending_layout = b_snapshot
        .to_pending_layout_intrinsic_size_snapshot(
            &current_c_document,
            c_pending.revision(),
            [image],
        )
        .map_err(|error| format!("C18.4 C pending intrinsic size 보존 실패: {error}"))?;
    if !stale_c.is_empty()
        || !c_pending.images().is_empty()
        || c_pending_layout
            .sizes()
            .get(&image.id())
            .is_none_or(|size| size.width != Some(1.0) || size.height != Some(4.0))
    {
        return Err(
            "C18.4 C pending이 이전 RGBA를 남기거나 자연 크기를 보존하지 않았습니다".to_owned(),
        );
    }
    let c_outcome = decode_on_worker(c_request, C_PNG, "C18.4 C")?;
    let (c_success, stale_c) = c_pending
        .replace_with_outcomes(&current_c_document, [c_outcome])
        .map_err(|error| format!("C18.4 C success snapshot 실패: {error}"))?;
    if !stale_c.is_empty()
        || c_success.images().get(&image.id()).is_none_or(|decoded| {
            decoded.dimensions().width() != 4 || decoded.dimensions().height() != 2
        })
    {
        return Err(
            "C18.4 C 성공 결과가 자연 크기 또는 resource snapshot에 반영되지 않았습니다".to_owned(),
        );
    }

    let broken_document = set_source(&mut document, image, BROKEN_SRC)?;
    let broken_request = ImageRequest::from_document(&broken_document, image)
        .map_err(|error| format!("C18.4 손상 PNG request 생성 실패: {error}"))?;
    let broken_outcome = decode_on_worker(broken_request, BROKEN_PNG, "C18.4 broken PNG")?;
    let (failed, stale_broken) = c_success
        .replace_with_outcomes(&broken_document, [broken_outcome])
        .map_err(|error| format!("C18.4 실패 resource snapshot 생성 실패: {error}"))?;
    if !stale_broken.is_empty()
        || !failed.images().is_empty()
        || !failed.failures().contains_key(&image.id())
    {
        return Err("C18.4 현재 decode 실패가 이전 image resource를 비우지 않았습니다".to_owned());
    }

    let d_document = set_source(&mut document, image, D_SRC)?;
    let d_request = ImageRequest::from_document(&d_document, image)
        .map_err(|error| format!("C18.4 복구 D request 생성 실패: {error}"))?;
    let d_outcome = decode_on_worker(d_request, D_PNG, "C18.4 recovery D")?;
    let (recovered, stale_d) = failed
        .replace_with_outcomes(&d_document, [d_outcome])
        .map_err(|error| format!("C18.4 D 복구 snapshot 실패: {error}"))?;
    if !stale_d.is_empty()
        || !recovered.failures().is_empty()
        || recovered.images().get(&image.id()).is_none_or(|decoded| {
            decoded.dimensions().width() != 2 || decoded.dimensions().height() != 1
        })
    {
        return Err("C18.4 정상 D source가 decode 실패 뒤 paint를 복구하지 못했습니다".to_owned());
    }

    let auto_report = run_auto_size_sequence()?;
    serde_json::to_string(&serde_json::json!({
        "schema": "spinon-c18-4-async-image-transition/v1",
        "race": {
            "completionOrder": ["B", "A"],
            "activeSrc": B_SRC,
            "activeImageCount": b_snapshot.images().len(),
            "staleARejected": rejected_a.len(),
            "naturalSize": [1, 4],
        },
        "replacement": {
            "pendingSrc": C_SRC,
            "pendingImageCount": c_pending.images().len(),
            "retainedIntrinsicSize": [1, 4],
            "successImageCount": c_success.images().len(),
            "successNaturalSize": [4, 2],
        },
        "failure": {
            "src": BROKEN_SRC,
            "imageCount": failed.images().len(),
            "recordedErrors": failed.failures().len(),
        },
        "recovery": {
            "src": D_SRC,
            "imageCount": recovered.images().len(),
            "naturalSize": [2, 1],
        },
        "autoSize": auto_report,
    }))
    .map_err(|error| format!("C18.4 transition report JSON 변환 실패: {error}"))
}

fn run_auto_size_sequence() -> Result<serde_json::Value, String> {
    let (mut document, image) = transition_document(A_SRC)?;
    let first_pending_document = document.snapshot();
    let first_pending = ImageResourceSnapshot::new_with_outcomes(
        &first_pending_document,
        IntrinsicSizeRevision::INITIAL,
        [],
    )
    .map_err(|error| format!("C18.4 auto-size 첫 pending 생성 실패: {error}"))?
    .0;
    let first_pending_layout = first_pending
        .to_pending_layout_intrinsic_size_snapshot(
            &first_pending_document,
            IntrinsicSizeRevision::INITIAL,
            [image],
        )
        .map_err(|error| format!("C18.4 auto-size 첫 pending measurement 실패: {error}"))?;
    let first_pending_frame = auto_size_layout_frame(
        &first_pending_document,
        document
            .root_children()
            .next()
            .ok_or_else(|| "C18.4 root가 없습니다".to_owned())?,
        image,
        first_pending_layout.clone(),
    )?;
    if first_pending_layout
        .sizes()
        .get(&image.id())
        .is_none_or(|size| size.width != Some(0.0) || size.height != Some(0.0))
        || first_pending_frame != [0.0, 0.0]
    {
        return Err("C18.4 auto-size 첫 pending이 natural size/frame 0x0이 아닙니다".to_owned());
    }

    let a_request = ImageRequest::from_document(&first_pending_document, image)
        .map_err(|error| format!("C18.4 auto-size A request 생성 실패: {error}"))?;
    let a_outcome = decode_on_worker(a_request, A_PNG, "C18.4 auto-size A")?;
    let (a_success, stale_a) = first_pending
        .replace_with_outcomes(&first_pending_document, [a_outcome])
        .map_err(|error| format!("C18.4 auto-size A success 생성 실패: {error}"))?;
    if !stale_a.is_empty() || a_success.images().len() != 1 {
        return Err("C18.4 auto-size A success가 반영되지 않았습니다".to_owned());
    }
    let a_layout = a_success
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| format!("C18.4 auto-size A layout snapshot 실패: {error}"))?;
    let a_frame = auto_size_layout_frame(
        &first_pending_document,
        document
            .root_children()
            .next()
            .ok_or_else(|| "C18.4 root가 없습니다".to_owned())?,
        image,
        a_layout,
    )?;
    if a_frame != [2.0, 2.0] {
        return Err(format!(
            "C18.4 auto-size A Taffy frame이 2x2가 아닙니다: {a_frame:?}"
        ));
    }
    let c_document = set_source(&mut document, image, C_SRC)?;
    let c_pending = a_success
        .replace_with_outcomes(&c_document, [])
        .map_err(|error| format!("C18.4 auto-size C pending 생성 실패: {error}"))?
        .0;
    let c_pending_layout = a_success
        .to_pending_layout_intrinsic_size_snapshot(&c_document, c_pending.revision(), [image])
        .map_err(|error| format!("C18.4 auto-size C pending measurement 실패: {error}"))?;
    let c_pending_frame = auto_size_layout_frame(
        &c_document,
        document
            .root_children()
            .next()
            .ok_or_else(|| "C18.4 root가 없습니다".to_owned())?,
        image,
        c_pending_layout.clone(),
    )?;
    if c_pending_layout.source()
        != (spinon_layout::LayoutSourceRevision::HostDocument {
            generation: c_document.generation(),
            document: c_document.document_revision(),
            render_tree: c_document.render_tree_revision(),
        })
        || c_pending_layout
            .sizes()
            .get(&image.id())
            .is_none_or(|size| size.width != Some(2.0) || size.height != Some(2.0))
        || c_pending_frame != [2.0, 2.0]
        || !c_pending.images().is_empty()
    {
        return Err(
            "C18.4 auto-size pending이 새 source stamp의 2x2 size-only 값이 아닙니다".to_owned(),
        );
    }
    let c_request = ImageRequest::from_document(&c_document, image)
        .map_err(|error| format!("C18.4 auto-size C request 생성 실패: {error}"))?;
    let c_outcome = decode_on_worker(c_request, C_PNG, "C18.4 auto-size C")?;
    let (c_success, stale_c) = c_pending
        .replace_with_outcomes(&c_document, [c_outcome])
        .map_err(|error| format!("C18.4 auto-size C success 생성 실패: {error}"))?;
    let size = c_success
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| format!("C18.4 auto-size C layout snapshot 실패: {error}"))?;
    let c_success_frame = auto_size_layout_frame(
        &c_document,
        document
            .root_children()
            .next()
            .ok_or_else(|| "C18.4 root가 없습니다".to_owned())?,
        image,
        size.clone(),
    )?;
    if !stale_c.is_empty()
        || size
            .sizes()
            .get(&image.id())
            .is_none_or(|size| size.width != Some(4.0) || size.height != Some(2.0))
        || c_success_frame != [4.0, 2.0]
    {
        return Err("C18.4 auto-size C natural dimensions가 4x2로 갱신되지 않았습니다".to_owned());
    }
    Ok(serde_json::json!({
        "firstPendingIntrinsicEntries": first_pending_layout.sizes().len(),
        "firstPendingNaturalSize": [0, 0],
        "firstPendingFrame": first_pending_frame,
        "firstSuccessNaturalSize": [2, 2],
        "firstSuccessFrame": a_frame,
        "replacementPendingSource": {
            "generation": c_document.generation().get(),
            "document": c_document.document_revision().get(),
            "renderTree": c_document.render_tree_revision().get(),
        },
        "replacementPendingNaturalSize": [2, 2],
        "replacementPendingFrame": c_pending_frame,
        "replacementPendingImageCount": c_pending.images().len(),
        "replacementSuccessNaturalSize": [4, 2],
        "replacementSuccessFrame": c_success_frame,
        "replacementSuccessIntrinsicSize": {
            "width": size.sizes()[&image.id()].width,
            "height": size.sizes()[&image.id()].height,
            "aspectRatio": size.sizes()[&image.id()].aspect_ratio,
        },
    }))
}

fn auto_size_layout_frame(
    document: &HostDocumentSnapshot,
    root: HostNodeHandle,
    image: HostNodeHandle,
    intrinsic_sizes: spinon_layout::LayoutIntrinsicSizeSnapshot,
) -> Result<[f32; 2], String> {
    let view = StyloDocumentView::new_html_runtime_mount_shared(Arc::new(document.clone()), root)
        .map_err(|error| format!("C18.4 auto-size Stylo document 생성 실패: {error}"))?;
    let stylesheet = StylesheetSource {
        id: "c18-4-async-image-auto-size".to_owned(),
        base_url: "https://spinon.invalid/c18-4-auto-size.css".to_owned(),
        origin: CssOrigin::Author,
        css: "#root { width: 120px; height: 80px; position: relative; } #image { display: block; width: auto; height: auto; }".to_owned(),
    };
    let computed = compute_runtime_flex_custom_properties_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        CssViewport::C04_FIXTURE,
        StyleRevision::INITIAL,
    )
    .map_err(|error| format!("C18.4 auto-size Stylo cascade 실패: {error}"))?;
    let output = compute_runtime_style_layout_with_intrinsic_size_snapshot(
        document,
        root,
        computed,
        CssViewport::C04_FIXTURE,
        intrinsic_sizes,
    )
    .map_err(|error| format!("C18.4 auto-size Taffy layout 실패: {error}"))?;
    let frame = output
        .layout
        .frames
        .get(&image.id())
        .ok_or_else(|| "C18.4 auto-size Taffy frame이 없습니다".to_owned())?;
    Ok([frame.width, frame.height])
}

fn decode_on_worker(
    request: ImageRequest,
    bytes: &'static [u8],
    label: &'static str,
) -> Result<spinon_image::ImageResourceOutcome, String> {
    thread::spawn(move || request.decode_outcome(bytes))
        .join()
        .map_err(|_| format!("{label} decode task가 panic했습니다"))
}

fn transition_document(src: &str) -> Result<(HostDocument, HostNodeHandle), String> {
    let mut document = HostDocument::new().map_err(|error| error.to_string())?;
    let root = document
        .reserve_node_handle()
        .map_err(|error| error.to_string())?;
    let image = document
        .reserve_node_handle()
        .map_err(|error| error.to_string())?;
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(184).ok_or_else(|| "C18.4 owner ID가 유효하지 않습니다".to_owned())?,
        document.document_revision(),
    );
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML_NAMESPACE.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: image,
            namespace: HTML_NAMESPACE.to_owned(),
            local_name: "img".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "id")
                .ok_or_else(|| "C18.4 root id attribute 이름이 유효하지 않습니다".to_owned())?,
            value: "root".to_owned().into(),
        })
        .push(DocumentOperation::SetAttribute {
            node: image,
            name: AttributeName::new(None, "id")
                .ok_or_else(|| "C18.4 image id attribute 이름이 유효하지 않습니다".to_owned())?,
            value: "image".to_owned().into(),
        })
        .push(DocumentOperation::SetAttribute {
            node: image,
            name: AttributeName::new(None, "src")
                .ok_or_else(|| "C18.4 src attribute 이름이 유효하지 않습니다".to_owned())?,
            value: src.to_owned().into(),
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
    document.commit(batch).map_err(|error| error.to_string())?;
    Ok((document, image))
}

fn set_source(
    document: &mut HostDocument,
    image: HostNodeHandle,
    src: &str,
) -> Result<HostDocumentSnapshot, String> {
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(184).ok_or_else(|| "C18.4 owner ID가 유효하지 않습니다".to_owned())?,
        document.document_revision(),
    );
    batch.push(DocumentOperation::SetAttribute {
        node: image,
        name: AttributeName::new(None, "src")
            .ok_or_else(|| "C18.4 src attribute 이름이 유효하지 않습니다".to_owned())?,
        value: src.to_owned().into(),
    });
    document.commit(batch).map_err(|error| error.to_string())?;
    Ok(document.snapshot())
}

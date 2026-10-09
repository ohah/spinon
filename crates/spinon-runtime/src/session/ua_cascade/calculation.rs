use super::runtime_layout::{RuntimeLayoutContext, RuntimeLayoutFailure, compute_runtime_layout};
use super::{
    CascadeDiagnostic, RuntimeLayoutCompleted, RuntimeUaCascadeRoot, WorkRequest,
    elapsed_microseconds, key_for,
};
use spinon_core::{HostNodeHandle, HostNodeKind, HostParent, StyleRevision};
use spinon_style::{CssCascadeError, StyloDocumentView, compute_runtime_flex_layout_cascade};
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug)]
pub(super) struct RuntimeCalculation {
    pub(super) roots: Vec<RuntimeUaCascadeRoot>,
    pub(super) cascade_duration_us: u128,
    pub(super) layout: Result<RuntimeLayoutCompleted, RuntimeLayoutFailure>,
    pub(super) layout_diagnostics: Vec<CascadeDiagnostic>,
}

pub(super) fn compute_request(request: &WorkRequest) -> Result<RuntimeCalculation, String> {
    let mut roots = Vec::new();
    let mut layout_context: Option<RuntimeLayoutContext> = None;
    let mut cascade_duration_us = 0_u128;
    let mut layout_diagnostics = Vec::new();
    for root in request.snapshot.root_children() {
        let Some(node) = request.snapshot.node(root) else {
            return Err("HostRoot가 snapshot에 없는 노드를 가리킵니다".to_owned());
        };
        match node.kind() {
            HostNodeKind::Text(_) => {
                return Err(format!(
                    "HostRoot 직속 텍스트 노드 {}는 지원하지 않습니다",
                    root.id()
                ));
            }
            HostNodeKind::Element(_) => {
                if request.snapshot.parent(root) != Some(HostParent::Root) {
                    return Err(format!(
                        "cascade root {}의 부모가 HostRoot가 아닙니다",
                        root.id()
                    ));
                }
                let cascade_started = Instant::now();
                let (computed_root, view) = compute_root(request, root)?;
                cascade_duration_us =
                    cascade_duration_us.saturating_add(elapsed_microseconds(cascade_started));
                layout_diagnostics.extend(computed_root.styles.diagnostics.iter().cloned());
                if roots.is_empty() {
                    layout_context = Some((root, view, computed_root.styles.clone()));
                } else {
                    layout_context = None;
                }
                roots.push(computed_root);
            }
        }
    }

    let layout = compute_runtime_layout(request, roots.len(), layout_context);

    Ok(RuntimeCalculation {
        roots,
        cascade_duration_us,
        layout,
        layout_diagnostics,
    })
}

fn compute_root(
    request: &WorkRequest,
    root: HostNodeHandle,
) -> Result<(RuntimeUaCascadeRoot, StyloDocumentView), String> {
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::clone(&request.snapshot), root)
            .map_err(|error| error.to_string())?;
    let styles =
        compute_runtime_flex_layout_cascade(&view, request.viewport, StyleRevision::INITIAL)
            .map_err(|error: CssCascadeError| error.to_string())?;
    if key_for(&request.snapshot, request.viewport.environment_revision) != request.key
        || styles.generation.get() != request.key.generation
        || styles.document_revision.get() != request.key.document_revision
        || styles.render_tree_revision.get() != request.key.render_tree_revision
        || styles.style_revision.get() != request.key.style_revision
    {
        return Err("Stylo 결과 revision이 요청 key와 다릅니다".to_owned());
    }
    Ok((
        RuntimeUaCascadeRoot {
            root_node_id: root.id().get(),
            styles,
        },
        view,
    ))
}

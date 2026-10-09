use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot, StyloDocumentView};

use crate::StyleLayoutError;

pub(super) fn assert_no_inline_style(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> Result<(), StyleLayoutError> {
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        let Some(node) = snapshot.node(handle) else {
            continue;
        };
        if let HostNodeKind::Element(element) = node.kind()
            && element.attributes().keys().any(|attribute| {
                attribute.namespace().is_none()
                    && attribute.local_name().eq_ignore_ascii_case("style")
            })
        {
            return Err(StyleLayoutError::UnsupportedInlineStyle(node.id()));
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children);
        }
    }
    Ok(())
}

pub(super) fn assert_view_matches_snapshot(
    snapshot: &HostDocumentSnapshot,
    view: &StyloDocumentView,
) -> Result<(), StyleLayoutError> {
    if snapshot.generation() != view.generation() {
        return Err(StyleLayoutError::SnapshotMismatch {
            field: "DocumentGeneration",
        });
    }
    if snapshot.document_revision() != view.document_revision() {
        return Err(StyleLayoutError::SnapshotMismatch {
            field: "DocumentRevision",
        });
    }
    if snapshot.render_tree_revision() != view.render_tree_revision() {
        return Err(StyleLayoutError::SnapshotMismatch {
            field: "RenderTreeRevision",
        });
    }
    Ok(())
}

pub(super) fn assert_matching_revision(
    snapshot: &HostDocumentSnapshot,
    styles: &ComputedStyleSnapshot,
    expected_profile: ComputedStyleProfile,
) -> Result<(), StyleLayoutError> {
    if styles.profile != expected_profile {
        return Err(StyleLayoutError::UnsupportedProfile {
            profile: format!("{:?}", styles.profile),
        });
    }
    if snapshot.generation() != styles.generation {
        return Err(StyleLayoutError::SnapshotMismatch {
            field: "DocumentGeneration",
        });
    }
    if snapshot.document_revision() != styles.document_revision {
        return Err(StyleLayoutError::SnapshotMismatch {
            field: "DocumentRevision",
        });
    }
    if snapshot.render_tree_revision() != styles.render_tree_revision {
        return Err(StyleLayoutError::SnapshotMismatch {
            field: "RenderTreeRevision",
        });
    }
    Ok(())
}

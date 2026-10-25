use super::{IMAGE_CASES, prepare};
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostParent, OwnerId,
    StyleRevision,
};
use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot, CssViewport};
use std::sync::Arc;

#[test]
fn image_matrix_has_unique_ids_and_bounded_geometry() {
    assert_eq!(IMAGE_CASES.len(), 20);
    for (index, case) in IMAGE_CASES.iter().enumerate() {
        assert!(
            IMAGE_CASES[..index]
                .iter()
                .all(|earlier| earlier.scene_id != case.scene_id)
        );
        assert!(case.grid_width.is_finite() && case.grid_width > 0.0);
        assert!(case.grid_height.is_finite() && case.grid_height > 0.0);
        assert!(matches!(case.columns.len(), 1 | 2));
        assert!(matches!(case.rows.len(), 1 | 2));
        assert!(case.column_gap.is_finite() && case.column_gap >= 0.0);
        assert!(case.row_gap.is_finite() && case.row_gap >= 0.0);
        assert!(case.grid_border.is_finite() && case.grid_border >= 0.0);
    }
}

#[test]
fn rejects_non_html_namespace_for_allowlisted_grid_root() {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    const SVG: &str = "http://www.w3.org/2000/svg";

    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_414).unwrap(), document.document_revision());
    for (node, namespace, tag, id, parent) in [
        (root, SVG, "div", "c114-auto-image-grid", HostParent::Root),
        (
            image,
            HTML,
            "img",
            "c114-auto-image-item",
            HostParent::Node(root),
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: namespace.to_owned(),
            local_name: tag.to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    batch.push(DocumentOperation::SetAttribute {
        node: image,
        name: AttributeName::new(None, "src").unwrap(),
        value: super::IMAGE_URL.to_owned().into(),
    });
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let styles = ComputedStyleSnapshot {
        profile: ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
        viewport: CssViewport::C04_FIXTURE,
        style_revision: StyleRevision::INITIAL,
        generation: snapshot.generation(),
        document_revision: snapshot.document_revision(),
        render_tree_revision: snapshot.render_tree_revision(),
        document_root_font_size_css_px: 16.0,
        elements: Arc::from([]),
        writing_styles: Arc::from([]),
        diagnostics: Vec::new(),
    };

    let error = match prepare(&snapshot, root, &styles) {
        Err(error) => error,
        Ok(_) => panic!("non-HTML root must not enter the image fixture"),
    };
    assert_eq!(error.code, "unsupported_c114_image_case");
}

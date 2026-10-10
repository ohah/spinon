use super::*;

#[test]
fn changing_only_ratio_uses_the_new_style_revision_and_frame() {
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(7734).unwrap(), document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML_NS.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::SetAttribute {
            node: root,
            name: AttributeName::new(None, "id").unwrap(),
            value: "ratio-revision".into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        });
    document.commit(batch).unwrap();

    let before = compute_stylesheet_layout(
        &document,
        root,
        "#ratio-revision { width:100px; aspect-ratio:2; }",
        StyleRevision::INITIAL,
    );
    let after_revision = StyleRevision::INITIAL.checked_next().unwrap();
    let after = compute_stylesheet_layout(
        &document,
        root,
        "#ratio-revision { width:100px; aspect-ratio:1; }",
        after_revision,
    );

    assert_eq!(
        before.computed_styles.style_revision,
        StyleRevision::INITIAL
    );
    assert_eq!(after.computed_styles.style_revision, after_revision);
    assert_eq!(before.layout.frames[&root.id()].height, 50.0);
    assert_eq!(after.layout.frames[&root.id()].height, 100.0);
}

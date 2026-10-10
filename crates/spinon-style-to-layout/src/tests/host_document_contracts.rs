use super::*;

#[test]
fn inline_style_attributes_are_rejected_before_layout_projection() {
    let fixture = DocumentFixture::with_inline_style("padding: 8px");
    assert!(matches!(
        fixture.compute(None),
        Err(StyleLayoutError::UnsupportedInlineStyle(node))
            if node == fixture.root.id()
    ));
}

#[test]
fn layout_rejects_text_in_a_host_document_subtree() {
    let fixture = DocumentFixture::new(true);
    assert!(matches!(
        fixture.compute(None),
        Err(StyleLayoutError::Layout(
            spinon_layout::LayoutError::UnsupportedTextNode(_)
        ))
    ));
}

#[test]
fn a_snapshot_from_another_document_generation_is_rejected_before_cascade() {
    let first = DocumentFixture::new(false);
    let second = DocumentFixture::new(false);
    let view = first.view();
    let stylesheets = vec![first.stylesheet()];
    assert_ne!(first.document.generation(), second.document.generation());
    assert!(matches!(
        compute_style_layout(
            &second.document.snapshot(),
            &view,
            second.root,
            &stylesheets,
            first.viewport(),
            StyleRevision::default()
        ),
        Err(StyleLayoutError::SnapshotMismatch {
            field: "DocumentGeneration"
        })
    ));
}

#[test]
fn stale_document_revision_is_rejected_before_cascade_or_layout() {
    let mut fixture = DocumentFixture::new(false);
    let view = fixture.view();
    let owner = OwnerId::new(1702).unwrap();
    let detached = fixture.document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, fixture.document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node: detached,
        namespace: HTML.to_owned(),
        local_name: "aside".to_owned(),
    });
    fixture.document.commit(batch).unwrap();
    assert!(matches!(
        compute_style_layout(
            &fixture.document.snapshot(),
            &view,
            fixture.root,
            &[fixture.stylesheet()],
            fixture.viewport(),
            StyleRevision::default()
        ),
        Err(StyleLayoutError::SnapshotMismatch {
            field: "DocumentRevision"
        })
    ));
}

#[test]
fn projection_errors_keep_the_css_node_and_property_context() {
    let fixture = DocumentFixture::new(false);
    let error = fixture
        .compute(Some("#flex-parent { flex-direction: row-reverse; }"))
        .unwrap_err();
    assert!(
        matches!(
            &error,
            StyleLayoutError::UnsupportedComputedValue {
                node,
                property: "flex-direction",
                ..
            } if *node == fixture.nodes["flex-parent"].id()
        ),
        "unexpected error: {error:?}"
    );
}

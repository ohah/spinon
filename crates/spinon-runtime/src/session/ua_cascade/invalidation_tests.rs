use super::invalidation::{RuntimeStyleInvalidation, classify_style_invalidation};
use super::invalidation_test_support::{TestTree, set_attribute_in_batch};
use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, HostParent};

#[test]
fn a_style_change_dirties_only_the_changed_element_subtree() {
    let mut tree = TestTree::new();
    let previous = tree.snapshot();
    tree.set_attribute(tree.left, "style", "--size:46px");
    let current = tree.snapshot();

    assert_eq!(
        classify_style_invalidation(Some(&previous), &current),
        RuntimeStyleInvalidation::Subtrees {
            generation: current.generation().get(),
            from_document_revision: previous.document_revision(),
            dirty_roots: vec![tree.left],
        }
    );
}

#[test]
fn detached_mutation_does_not_dirty_the_connected_style_tree() {
    let mut tree = TestTree::new();
    let previous = tree.snapshot();
    tree.set_attribute(tree.detached, "style", "width:72px");
    let current = tree.snapshot();

    assert!(matches!(
        classify_style_invalidation(Some(&previous), &current),
        RuntimeStyleInvalidation::Unchanged { .. }
    ));
}

#[test]
fn overlapping_style_changes_keep_only_the_outer_dirty_root() {
    let mut tree = TestTree::new();
    let previous = tree.snapshot();
    let mut batch = DocumentChangeBatch::new(tree.owner, tree.document.document_revision());
    set_attribute_in_batch(&mut batch, tree.left, "style", "--size:46px");
    set_attribute_in_batch(&mut batch, tree.left_child, "style", "width:51px");
    tree.document.commit(batch).unwrap();

    let invalidation = classify_style_invalidation(Some(&previous), &tree.snapshot());
    assert_eq!(
        invalidation.dirty_roots(),
        &[tree.left],
        "조상 dirty subtree가 자손 변경을 포함해야 합니다"
    );
}

#[test]
fn separate_changed_branches_keep_separate_dirty_roots() {
    let mut tree = TestTree::new();
    let previous = tree.snapshot();
    let mut batch = DocumentChangeBatch::new(tree.owner, tree.document.document_revision());
    set_attribute_in_batch(&mut batch, tree.left, "style", "--size:46px");
    set_attribute_in_batch(&mut batch, tree.right, "style", "--size:51px");
    tree.document.commit(batch).unwrap();

    assert_eq!(
        classify_style_invalidation(Some(&previous), &tree.snapshot()).dirty_roots(),
        &[tree.left, tree.right]
    );
}

#[test]
fn non_style_attribute_change_forces_a_full_recalculation() {
    let mut tree = TestTree::new();
    let previous = tree.snapshot();
    tree.set_attribute(tree.left, "class", "selected");

    assert_eq!(
        classify_style_invalidation(Some(&previous), &tree.snapshot()),
        RuntimeStyleInvalidation::Full
    );
}

#[test]
fn changing_tree_order_forces_a_full_recalculation() {
    let mut tree = TestTree::new();
    let previous = tree.snapshot();
    let mut batch = DocumentChangeBatch::new(tree.owner, tree.document.document_revision());
    batch.push(DocumentOperation::RemoveChild {
        parent: HostParent::Node(tree.root),
        node: tree.left,
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(tree.root),
        node: tree.left,
        before: Some(tree.right),
    });
    tree.document.commit(batch).unwrap();

    assert_eq!(
        classify_style_invalidation(Some(&previous), &tree.snapshot()),
        RuntimeStyleInvalidation::Full
    );
}

#[test]
fn html_style_name_case_change_with_same_value_is_unstyled_equivalent() {
    let mut tree = TestTree::new();
    let previous = tree.snapshot();
    let mut batch = DocumentChangeBatch::new(tree.owner, tree.document.document_revision());
    batch.push(DocumentOperation::RemoveAttribute {
        node: tree.left,
        name: AttributeName::new(None, "style").unwrap(),
    });
    set_attribute_in_batch(&mut batch, tree.left, "STYLE", "--size:32px");
    tree.document.commit(batch).unwrap();

    assert!(matches!(
        classify_style_invalidation(Some(&previous), &tree.snapshot()),
        RuntimeStyleInvalidation::Unchanged { .. }
    ));
}

#[test]
fn ambiguous_html_style_attributes_force_a_full_recalculation() {
    let mut tree = TestTree::new();
    let mut add_duplicate = DocumentChangeBatch::new(tree.owner, tree.document.document_revision());
    set_attribute_in_batch(&mut add_duplicate, tree.left, "STYLE", "--size:33px");
    tree.document.commit(add_duplicate).unwrap();
    let previous = tree.snapshot();
    tree.set_attribute(tree.left, "style", "--size:47px");

    assert_eq!(
        classify_style_invalidation(Some(&previous), &tree.snapshot()),
        RuntimeStyleInvalidation::Full
    );
}

#[test]
fn a_text_mutation_and_an_unavailable_baseline_force_full_recalculation() {
    let mut tree = TestTree::new();
    let previous = tree.snapshot();
    let text = tree.document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(tree.owner, tree.document.document_revision());
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: "old".into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(tree.left),
        node: text,
        before: None,
    });
    tree.document.commit(batch).unwrap();
    let current = tree.snapshot();

    assert_eq!(
        classify_style_invalidation(Some(&previous), &current),
        RuntimeStyleInvalidation::Full
    );
    assert_eq!(
        classify_style_invalidation(None, &current),
        RuntimeStyleInvalidation::Full
    );
}

#[test]
fn detached_revision_changes_can_be_composed_without_retaining_the_first_snapshot() {
    let mut tree = TestTree::new();
    let previous = tree.snapshot();
    tree.set_attribute(tree.detached, "style", "width:72px");
    let intermediate = tree.snapshot();
    tree.set_attribute(tree.left, "style", "--size:46px");
    let current = tree.snapshot();

    let invalidation = classify_style_invalidation(Some(&previous), &current);
    assert_eq!(
        invalidation.dirty_roots(),
        &[tree.left],
        "연속된 detached 변경은 현재 연결 dirty root를 누락하면 안 됩니다"
    );
    assert!(current.document_revision().get() > intermediate.document_revision().get());
}

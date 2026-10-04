use super::*;

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn owner(value: u64) -> OwnerId {
    OwnerId::new(value).unwrap()
}

fn create_element(document: &mut HostDocument, owner: OwnerId, name: &str) -> HostNodeHandle {
    let node = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch.push(DocumentOperation::CreateElement {
        node,
        namespace: HTML.to_owned(),
        local_name: name.to_owned(),
    });
    document.commit(batch).unwrap();
    node
}

fn create_text(document: &mut HostDocument, owner: OwnerId, text: &str) -> HostNodeHandle {
    let node = document.reserve_node_handle().unwrap();
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch.push(DocumentOperation::CreateText {
        node,
        data: DomString::from(text),
    });
    document.commit(batch).unwrap();
    node
}

fn insert(document: &mut HostDocument, owner: OwnerId, parent: HostParent, node: HostNodeHandle) {
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node,
        before: None,
    });
    document.commit(batch).unwrap();
}

#[test]
fn plan_is_read_only_and_commit_reclaims_only_unrooted_components() {
    let mut document = HostDocument::new().unwrap();
    let owner = owner(1);
    let connected = create_element(&mut document, owner, "main");
    let connected_text = create_text(&mut document, owner, "visible");
    insert(&mut document, owner, HostParent::Root, connected);
    insert(
        &mut document,
        owner,
        HostParent::Node(connected),
        connected_text,
    );
    let detached = create_element(&mut document, owner, "section");
    let detached_child = create_element(&mut document, owner, "article");
    let detached_grandchild = create_text(&mut document, owner, "retained");
    insert(
        &mut document,
        owner,
        HostParent::Node(detached),
        detached_child,
    );
    insert(
        &mut document,
        owner,
        HostParent::Node(detached_child),
        detached_grandchild,
    );
    let isolated = create_text(&mut document, owner, "reclaim");
    let before = document.snapshot();

    let plan = document.plan_collection(&[detached_grandchild]).unwrap();

    assert_eq!(plan.reclaimed_handles(), &[isolated]);
    assert_eq!(document.snapshot(), before);
    document.commit_collection(&plan).unwrap();
    assert_eq!(document.node_count(), 5);
    assert_eq!(document.node(isolated), None);
    assert!(document.node(detached).is_some());
    assert!(document.node(detached_child).is_some());
    assert!(document.node(detached_grandchild).is_some());
    assert_eq!(document.document_revision(), before.document_revision());
    assert_eq!(
        document.render_tree_revision(),
        before.render_tree_revision()
    );
    assert_eq!(
        document.commit_collection(&plan).unwrap_err().kind(),
        &DocumentErrorKind::StaleCollectionPlan
    );
}

#[test]
fn host_root_keeps_its_tree_and_empty_root_set_reclaims_detached_subtrees() {
    let mut document = HostDocument::new().unwrap();
    let owner = owner(2);
    let connected = create_element(&mut document, owner, "main");
    let connected_child = create_text(&mut document, owner, "live");
    insert(&mut document, owner, HostParent::Root, connected);
    insert(
        &mut document,
        owner,
        HostParent::Node(connected),
        connected_child,
    );
    let detached = create_element(&mut document, owner, "aside");
    let detached_child = create_text(&mut document, owner, "dead");
    insert(
        &mut document,
        owner,
        HostParent::Node(detached),
        detached_child,
    );

    let plan = document.plan_collection(&[]).unwrap();
    assert_eq!(plan.reclaimed_handles(), &[detached, detached_child]);
    document.commit_collection(&plan).unwrap();
    assert_eq!(document.node_count(), 2);
    assert!(document.node(connected).is_some());
    assert!(document.node(connected_child).is_some());
}

#[test]
fn invalid_roots_reject_the_whole_plan_without_mutating_the_document() {
    let mut document = HostDocument::new().unwrap();
    let owner = owner(3);
    let live = create_element(&mut document, owner, "main");
    let detached = create_text(&mut document, owner, "detached");
    let before = document.snapshot();
    let mut other_document = HostDocument::new().unwrap();
    let foreign = other_document
        .reserve_node_handle()
        .expect("유효한 다른 문서 handle이어야 합니다");

    let stale = document.plan_collection(&[foreign]).unwrap_err();
    assert!(matches!(
        stale.kind(),
        DocumentErrorKind::StaleGeneration { .. }
    ));
    let duplicate = document.plan_collection(&[detached, detached]).unwrap_err();
    assert_eq!(
        duplicate.kind(),
        &DocumentErrorKind::DuplicateCollectionRoot(detached.id())
    );
    assert_eq!(document.snapshot(), before);
    assert!(document.node(live).is_some());
}

#[test]
fn malformed_parent_child_root_and_cycle_graphs_fail_closed() {
    let mut mismatched = HostDocument::new().unwrap();
    let owner = owner(4);
    let parent = create_element(&mut mismatched, owner, "div");
    let child = create_text(&mut mismatched, owner, "child");
    insert(&mut mismatched, owner, HostParent::Node(parent), child);
    mismatched
        .nodes
        .get_mut(&parent.id)
        .unwrap()
        .children
        .clear();
    let before = mismatched.snapshot();
    assert_eq!(
        mismatched.plan_collection(&[]).unwrap_err().kind(),
        &DocumentErrorKind::InvalidCollectionGraph
    );
    assert_eq!(mismatched.snapshot(), before);

    let mut broken_root = HostDocument::new().unwrap();
    let root_node = create_element(&mut broken_root, owner, "root");
    insert(&mut broken_root, owner, HostParent::Root, root_node);
    broken_root.root_children.clear();
    let before = broken_root.snapshot();
    assert_eq!(
        broken_root.plan_collection(&[]).unwrap_err().kind(),
        &DocumentErrorKind::InvalidCollectionGraph
    );
    assert_eq!(broken_root.snapshot(), before);

    let mut dangling_child = HostDocument::new().unwrap();
    let parent = create_element(&mut dangling_child, owner, "parent");
    dangling_child
        .nodes
        .get_mut(&parent.id)
        .unwrap()
        .children
        .push(NodeId::new(99).unwrap());
    assert_eq!(
        dangling_child.plan_collection(&[]).unwrap_err().kind(),
        &DocumentErrorKind::InvalidCollectionGraph
    );

    let mut duplicate_child = HostDocument::new().unwrap();
    let parent = create_element(&mut duplicate_child, owner, "parent");
    let child = create_text(&mut duplicate_child, owner, "child");
    insert(&mut duplicate_child, owner, HostParent::Node(parent), child);
    duplicate_child
        .nodes
        .get_mut(&parent.id)
        .unwrap()
        .children
        .push(child.id);
    assert_eq!(
        duplicate_child.plan_collection(&[]).unwrap_err().kind(),
        &DocumentErrorKind::InvalidCollectionGraph
    );

    let mut cyclic = HostDocument::new().unwrap();
    let first = create_element(&mut cyclic, owner, "first");
    let second = create_element(&mut cyclic, owner, "second");
    cyclic.nodes.get_mut(&first.id).unwrap().parent = Some(ParentRef::Node(second.id));
    cyclic
        .nodes
        .get_mut(&first.id)
        .unwrap()
        .children
        .push(second.id);
    cyclic.nodes.get_mut(&second.id).unwrap().parent = Some(ParentRef::Node(first.id));
    cyclic
        .nodes
        .get_mut(&second.id)
        .unwrap()
        .children
        .push(first.id);
    let before = cyclic.snapshot();
    assert_eq!(
        cyclic.plan_collection(&[]).unwrap_err().kind(),
        &DocumentErrorKind::InvalidCollectionGraph
    );
    assert_eq!(cyclic.snapshot(), before);
}

#[test]
fn cross_owner_edges_and_non_element_parents_fail_closed() {
    let first_owner = owner(6);
    let second_owner = owner(7);

    let mut cross_owner = HostDocument::new().unwrap();
    let parent = create_element(&mut cross_owner, first_owner, "parent");
    let child = create_text(&mut cross_owner, second_owner, "child");
    cross_owner
        .nodes
        .get_mut(&parent.id)
        .unwrap()
        .children
        .push(child.id);
    cross_owner.nodes.get_mut(&child.id).unwrap().parent = Some(ParentRef::Node(parent.id));
    let before = cross_owner.snapshot();
    assert_eq!(
        cross_owner.plan_collection(&[]).unwrap_err().kind(),
        &DocumentErrorKind::InvalidCollectionGraph
    );
    assert_eq!(cross_owner.snapshot(), before);

    let mut text_parent = HostDocument::new().unwrap();
    let parent = create_text(&mut text_parent, first_owner, "parent");
    let child = create_text(&mut text_parent, first_owner, "child");
    text_parent
        .nodes
        .get_mut(&parent.id)
        .unwrap()
        .children
        .push(child.id);
    text_parent.nodes.get_mut(&child.id).unwrap().parent = Some(ParentRef::Node(parent.id));
    let before = text_parent.snapshot();
    assert_eq!(
        text_parent.plan_collection(&[]).unwrap_err().kind(),
        &DocumentErrorKind::InvalidCollectionGraph
    );
    assert_eq!(text_parent.snapshot(), before);
}

#[test]
fn stale_plan_after_document_change_cannot_remove_any_node() {
    let mut document = HostDocument::new().unwrap();
    let owner = owner(5);
    let dead = create_text(&mut document, owner, "dead");
    let plan = document.plan_collection(&[]).unwrap();
    create_text(&mut document, owner, "new");
    let before = document.snapshot();

    assert_eq!(
        document.commit_collection(&plan).unwrap_err().kind(),
        &DocumentErrorKind::StaleCollectionPlan
    );
    assert_eq!(document.snapshot(), before);
    assert!(document.node(dead).is_some());
}

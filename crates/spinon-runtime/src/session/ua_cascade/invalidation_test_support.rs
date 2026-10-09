use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};

const HTML: &str = "http://www.w3.org/1999/xhtml";

pub(super) struct TestTree {
    pub(super) document: HostDocument,
    pub(super) owner: OwnerId,
    pub(super) root: HostNodeHandle,
    pub(super) left: HostNodeHandle,
    pub(super) left_child: HostNodeHandle,
    pub(super) right: HostNodeHandle,
    pub(super) right_child: HostNodeHandle,
    pub(super) detached: HostNodeHandle,
}

impl TestTree {
    pub(super) fn new() -> Self {
        let mut document = HostDocument::new().unwrap();
        let owner = OwnerId::new(5401).unwrap();
        let root = document.reserve_node_handle().unwrap();
        let left = document.reserve_node_handle().unwrap();
        let left_child = document.reserve_node_handle().unwrap();
        let right = document.reserve_node_handle().unwrap();
        let right_child = document.reserve_node_handle().unwrap();
        let detached = document.reserve_node_handle().unwrap();
        let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
        for (node, name) in [
            (root, "main"),
            (left, "section"),
            (left_child, "div"),
            (right, "section"),
            (right_child, "div"),
            (detached, "div"),
        ] {
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML.to_owned(),
                local_name: name.to_owned(),
            });
        }
        for (node, value) in [
            (root, "display:flex"),
            (left, "--size:32px"),
            (left_child, "width:var(--size)"),
            (right, "--size:41px"),
            (right_child, "width:var(--size)"),
            (detached, "width:11px"),
        ] {
            set_attribute_in_batch(&mut batch, node, "style", value);
        }
        for (parent, node) in [
            (HostParent::Root, root),
            (HostParent::Node(root), left),
            (HostParent::Node(left), left_child),
            (HostParent::Node(root), right),
            (HostParent::Node(right), right_child),
        ] {
            batch.push(DocumentOperation::InsertBefore {
                parent,
                node,
                before: None,
            });
        }
        document.commit(batch).unwrap();
        Self {
            document,
            owner,
            root,
            left,
            left_child,
            right,
            right_child,
            detached,
        }
    }

    pub(super) fn set_attribute(&mut self, node: HostNodeHandle, name: &str, value: &str) {
        let mut batch = DocumentChangeBatch::new(self.owner, self.document.document_revision());
        set_attribute_in_batch(&mut batch, node, name, value);
        self.document.commit(batch).unwrap();
    }

    pub(super) fn remove_attribute(&mut self, node: HostNodeHandle, name: &str) {
        let mut batch = DocumentChangeBatch::new(self.owner, self.document.document_revision());
        batch.push(DocumentOperation::RemoveAttribute {
            node,
            name: AttributeName::new(None, name).unwrap(),
        });
        self.document.commit(batch).unwrap();
    }

    pub(super) fn add_author_stylesheet(&mut self, css: &str) {
        let stylesheet = self.document.reserve_node_handle().unwrap();
        let text = self.document.reserve_node_handle().unwrap();
        let mut batch = DocumentChangeBatch::new(self.owner, self.document.document_revision());
        batch.push(DocumentOperation::CreateElement {
            node: stylesheet,
            namespace: HTML.to_owned(),
            local_name: "style".to_owned(),
        });
        batch.push(DocumentOperation::CreateText {
            node: text,
            data: css.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(stylesheet),
            node: text,
            before: None,
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(self.root),
            node: stylesheet,
            before: None,
        });
        self.document.commit(batch).unwrap();
    }

    pub(super) fn snapshot(&self) -> spinon_core::HostDocumentSnapshot {
        self.document.snapshot()
    }
}

pub(super) fn set_attribute_in_batch(
    batch: &mut DocumentChangeBatch,
    node: HostNodeHandle,
    name: &str,
    value: &str,
) {
    batch.push(DocumentOperation::SetAttribute {
        node,
        name: AttributeName::new(None, name).unwrap(),
        value: value.into(),
    });
}

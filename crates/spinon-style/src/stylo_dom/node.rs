use std::{fmt, marker::PhantomData};

use spinon_core::{HostNodeHandle, HostNodeKind, HostParent};
use style::context::QuirksMode;
use style::dom::{NodeInfo, OpaqueNode, TDocument, TNode, TShadowRoot};
use style::shared_lock::SharedRwLock;
use style::stylist::CascadeData;

use super::{StyloDocument, StyloDocumentView, StyloElement, element::StyloElementRef};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StyloNodeRef {
    Document,
    SyntheticHtml,
    SyntheticBody,
    Host(HostNodeHandle),
}

/// HostDocument의 node 또는 view가 제공하는 가상 Document/HTML wrapper node입니다.
#[derive(Clone, Copy)]
pub struct StyloNode<'a> {
    view: &'a StyloDocumentView,
    node_ref: StyloNodeRef,
}

impl<'a> StyloNode<'a> {
    pub(super) const fn new(view: &'a StyloDocumentView, handle: Option<HostNodeHandle>) -> Self {
        match handle {
            Some(handle) => Self::host(view, handle),
            None => Self::document(view),
        }
    }

    pub(super) const fn document(view: &'a StyloDocumentView) -> Self {
        Self {
            view,
            node_ref: StyloNodeRef::Document,
        }
    }

    pub(super) const fn synthetic_html(view: &'a StyloDocumentView) -> Self {
        Self {
            view,
            node_ref: StyloNodeRef::SyntheticHtml,
        }
    }

    pub(super) const fn synthetic_body(view: &'a StyloDocumentView) -> Self {
        Self {
            view,
            node_ref: StyloNodeRef::SyntheticBody,
        }
    }

    pub(super) const fn host(view: &'a StyloDocumentView, handle: HostNodeHandle) -> Self {
        Self {
            view,
            node_ref: StyloNodeRef::Host(handle),
        }
    }

    pub(super) const fn for_element(element: StyloElement<'a>) -> Self {
        match element.element_ref {
            StyloElementRef::Host(handle) => Self::host(element.view, handle),
            StyloElementRef::SyntheticHtml => Self::synthetic_html(element.view),
            StyloElementRef::SyntheticBody => Self::synthetic_body(element.view),
        }
    }

    pub const fn handle(self) -> Option<HostNodeHandle> {
        match self.node_ref {
            StyloNodeRef::Host(handle) => Some(handle),
            StyloNodeRef::Document | StyloNodeRef::SyntheticHtml | StyloNodeRef::SyntheticBody => {
                None
            }
        }
    }

    pub(super) fn node_children(self) -> Vec<Self> {
        match self.node_ref {
            StyloNodeRef::Document if self.view.has_synthetic_html_document() => {
                vec![Self::synthetic_html(self.view)]
            }
            StyloNodeRef::Document => vec![Self::host(self.view, self.view.root_handle())],
            StyloNodeRef::SyntheticHtml => vec![Self::synthetic_body(self.view)],
            StyloNodeRef::SyntheticBody => vec![Self::host(self.view, self.view.root_handle())],
            StyloNodeRef::Host(handle) => self
                .view
                .snapshot()
                .children(handle)
                .into_iter()
                .flatten()
                .filter(|child| self.view.is_member(*child))
                .map(|child| Self::host(self.view, child))
                .collect(),
        }
    }

    fn sibling(self, next: bool) -> Option<Self> {
        let StyloNodeRef::Host(handle) = self.node_ref else {
            return None;
        };
        if handle == self.view.root_handle() {
            return None;
        }
        let sibling = if next {
            self.view.snapshot().next_sibling(handle)
        } else {
            self.view.snapshot().previous_sibling(handle)
        }?;
        self.view
            .is_member(sibling)
            .then(|| Self::host(self.view, sibling))
    }

    fn parent(self) -> Option<Self> {
        match self.node_ref {
            StyloNodeRef::Document => None,
            StyloNodeRef::SyntheticHtml => Some(Self::document(self.view)),
            StyloNodeRef::SyntheticBody => Some(Self::synthetic_html(self.view)),
            StyloNodeRef::Host(handle) => match self.view.snapshot().parent(handle)? {
                HostParent::Root if handle == self.view.root_handle() => {
                    if self.view.has_synthetic_html_document() {
                        Some(Self::synthetic_body(self.view))
                    } else {
                        Some(Self::document(self.view))
                    }
                }
                HostParent::Root => None,
                HostParent::Node(parent) if self.view.is_member(parent) => {
                    Some(Self::host(self.view, parent))
                }
                HostParent::Node(_) => None,
            },
        }
    }
}

impl fmt::Debug for StyloNode<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StyloNode")
            .field("node_ref", &self.node_ref)
            .finish()
    }
}

impl PartialEq for StyloNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.view, other.view) && self.node_ref == other.node_ref
    }
}

impl NodeInfo for StyloNode<'_> {
    fn is_element(&self) -> bool {
        self.as_element().is_some()
    }

    fn is_text_node(&self) -> bool {
        self.handle()
            .and_then(|handle| self.view.snapshot().node(handle))
            .is_some_and(|node| matches!(node.kind(), HostNodeKind::Text(_)))
    }
}

impl<'a> TNode for StyloNode<'a> {
    type ConcreteElement = StyloElement<'a>;
    type ConcreteDocument = StyloDocument<'a>;
    type ConcreteShadowRoot = StyloShadowRoot<'a>;

    fn parent_node(&self) -> Option<Self> {
        self.parent()
    }

    fn first_child(&self) -> Option<Self> {
        self.node_children().into_iter().next()
    }

    fn last_child(&self) -> Option<Self> {
        self.node_children().into_iter().last()
    }

    fn prev_sibling(&self) -> Option<Self> {
        self.sibling(false)
    }

    fn next_sibling(&self) -> Option<Self> {
        self.sibling(true)
    }

    fn owner_doc(&self) -> StyloDocument<'a> {
        StyloDocument { view: self.view }
    }

    fn is_in_document(&self) -> bool {
        match self.node_ref {
            StyloNodeRef::Document | StyloNodeRef::SyntheticHtml | StyloNodeRef::SyntheticBody => {
                true
            }
            StyloNodeRef::Host(handle) => self.view.is_member(handle),
        }
    }

    fn traversal_parent(&self) -> Option<StyloElement<'a>> {
        self.parent_node().and_then(|parent| parent.as_element())
    }

    fn opaque(&self) -> OpaqueNode {
        let address = match self.node_ref {
            StyloNodeRef::Host(handle) => self
                .view
                .snapshot()
                .node(handle)
                .map_or(0, |node| std::ptr::from_ref(node) as usize),
            StyloNodeRef::Document => std::ptr::from_ref(self.view) as usize,
            StyloNodeRef::SyntheticHtml => {
                std::ptr::from_ref(self.view.element_data(StyloElementRef::SyntheticHtml)) as usize
            }
            StyloNodeRef::SyntheticBody => {
                std::ptr::from_ref(self.view.element_data(StyloElementRef::SyntheticBody)) as usize
            }
        };
        OpaqueNode(address)
    }

    fn debug_id(self) -> usize {
        self.opaque().id()
    }

    fn as_element(&self) -> Option<StyloElement<'a>> {
        match self.node_ref {
            StyloNodeRef::Document => None,
            StyloNodeRef::SyntheticHtml => {
                Some(StyloElement::new(self.view, StyloElementRef::SyntheticHtml))
            }
            StyloNodeRef::SyntheticBody => {
                Some(StyloElement::new(self.view, StyloElementRef::SyntheticBody))
            }
            StyloNodeRef::Host(handle) => self.view.element(handle),
        }
    }

    fn as_document(&self) -> Option<StyloDocument<'a>> {
        (self.node_ref == StyloNodeRef::Document).then_some(StyloDocument { view: self.view })
    }

    fn as_shadow_root(&self) -> Option<StyloShadowRoot<'a>> {
        None
    }
}

impl<'a> TDocument for StyloDocument<'a> {
    type ConcreteNode = StyloNode<'a>;

    fn as_node(&self) -> StyloNode<'a> {
        StyloNode::document(self.view)
    }

    fn is_html_document(&self) -> bool {
        self.view.is_html_document()
    }

    fn quirks_mode(&self) -> QuirksMode {
        self.view.quirks_mode()
    }

    fn shared_lock(&self) -> &SharedRwLock {
        self.view.shared_lock()
    }
}

/// C03에서는 shadow tree를 만들지 않으므로 외부에서 생성할 수 없는 marker입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StyloShadowRoot<'a>(PhantomData<&'a StyloDocumentView>);

impl<'a> TShadowRoot for StyloShadowRoot<'a> {
    type ConcreteNode = StyloNode<'a>;

    fn as_node(&self) -> StyloNode<'a> {
        unreachable!("C03 어댑터는 ShadowRoot를 생성하지 않습니다")
    }

    fn host(&self) -> StyloElement<'a> {
        unreachable!("C03 어댑터는 ShadowRoot를 생성하지 않습니다")
    }

    fn style_data<'b>(&self) -> Option<&'b CascadeData>
    where
        Self: 'b,
    {
        None
    }
}

use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt,
    sync::Arc,
};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use style::context::QuirksMode;
use style::shared_lock::SharedRwLock;
use url::Url;

use crate::stylo_dom::{element::StyloElementData, node::StyloNode};

use super::{StyloElement, element::StyloElementRef};

/// Stylo view를 만들 수 없는 내부 문서 입력입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StyloDomError {
    /// root가 현재 snapshot의 HostRoot 직속 연결 요소가 아닙니다.
    InvalidRoot,
    /// HTML 요소에 ASCII 대소문자만 다른 no-namespace 속성이 중복됩니다.
    AmbiguousHtmlAttributeNames { node: NodeId },
    /// 문서 base URL을 절대 URL로 파싱할 수 없습니다.
    InvalidDocumentBaseUrl,
}

impl fmt::Display for StyloDomError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRoot => formatter.write_str("Stylo root는 연결된 요소여야 합니다"),
            Self::AmbiguousHtmlAttributeNames { node } => write!(
                formatter,
                "HTML 요소 {node}에 대소문자만 다른 속성 이름이 중복됩니다"
            ),
            Self::InvalidDocumentBaseUrl => {
                formatter.write_str("문서 base URL은 올바른 절대 URL이어야 합니다")
            }
        }
    }
}

impl Error for StyloDomError {}

/// 불변 HostDocument snapshot을 Stylo 문서·요소 trait에 연결하는 view입니다.
///
/// 생성한 뒤에는 snapshot과 root가 고정됩니다. 문서 변경은 새 view로 전달합니다.
pub struct StyloDocumentView {
    snapshot: Arc<HostDocumentSnapshot>,
    root: HostNodeHandle,
    root_matches_root_pseudo: bool,
    is_html_document: bool,
    quirks_mode: QuirksMode,
    members: BTreeSet<NodeId>,
    elements: BTreeMap<NodeId, StyloElementData>,
    synthetic_html: Option<StyloElementData>,
    synthetic_body: Option<StyloElementData>,
    shared_lock: SharedRwLock,
    document_base_url: Url,
}

impl StyloDocumentView {
    /// 명시한 HostRoot 직속 요소를 스타일 문서의 유일한 루트로 보입니다.
    pub fn new(
        snapshot: HostDocumentSnapshot,
        root: HostNodeHandle,
        is_html_document: bool,
        quirks_mode: QuirksMode,
    ) -> Result<Self, StyloDomError> {
        Self::new_shared_with_base_url(
            Arc::new(snapshot),
            root,
            is_html_document,
            quirks_mode,
            "https://spinon.invalid/document.html",
        )
    }

    /// 하나의 immutable snapshot을 여러 cascade root에서 공유하는 Stylo view를 만듭니다.
    pub fn new_shared(
        snapshot: Arc<HostDocumentSnapshot>,
        root: HostNodeHandle,
        is_html_document: bool,
        quirks_mode: QuirksMode,
    ) -> Result<Self, StyloDomError> {
        Self::new_shared_with_base_url(
            snapshot,
            root,
            is_html_document,
            quirks_mode,
            "https://spinon.invalid/document.html",
        )
    }

    /// HTML UA cascade가 사용할 no-quirks Stylo view를 공유 snapshot으로 만듭니다.
    pub fn new_html_shared(
        snapshot: Arc<HostDocumentSnapshot>,
        root: HostNodeHandle,
    ) -> Result<Self, StyloDomError> {
        Self::new_shared(snapshot, root, true, QuirksMode::NoQuirks)
    }

    /// 문서 URL을 기준으로 inline style 속성을 파싱하는 Stylo view를 만듭니다.
    pub fn new_with_base_url(
        snapshot: HostDocumentSnapshot,
        root: HostNodeHandle,
        is_html_document: bool,
        quirks_mode: QuirksMode,
        document_base_url: &str,
    ) -> Result<Self, StyloDomError> {
        Self::new_shared_with_base_url(
            Arc::new(snapshot),
            root,
            is_html_document,
            quirks_mode,
            document_base_url,
        )
    }

    /// immutable snapshot과 base URL을 공유하는 Stylo view를 만듭니다.
    pub fn new_shared_with_base_url(
        snapshot: Arc<HostDocumentSnapshot>,
        root: HostNodeHandle,
        is_html_document: bool,
        quirks_mode: QuirksMode,
        document_base_url: &str,
    ) -> Result<Self, StyloDomError> {
        Self::build_shared_with_base_url(
            snapshot,
            root,
            is_html_document,
            quirks_mode,
            document_base_url,
            true,
            false,
        )
    }

    /// HTML UA cascade에서 앱 mount root를 fragment 자식으로 연결합니다.
    ///
    /// 선택한 HostNode는 CSS `:root`가 아니므로 document-root display blockification을 받지
    /// 않습니다. 기존 `new` 계열 생성자는 문서 root semantics를 계속 사용합니다.
    pub fn new_html_fragment_child_shared(
        snapshot: Arc<HostDocumentSnapshot>,
        root: HostNodeHandle,
    ) -> Result<Self, StyloDomError> {
        Self::build_shared_with_base_url(
            snapshot,
            root,
            true,
            QuirksMode::NoQuirks,
            "https://spinon.invalid/document.html",
            false,
            false,
        )
    }

    /// runtime 앱 mount를 합성 HTML 문서 루트와 body 아래에 연결하는 view를 만듭니다.
    ///
    /// 합성 요소는 Stylo selector/cascade에만 참여하고 HostDocument 노드나 layout 결과가 되지
    /// 않습니다. 공개 DOM `documentElement`를 대신하지 않습니다.
    pub fn new_html_runtime_mount_shared(
        snapshot: Arc<HostDocumentSnapshot>,
        root: HostNodeHandle,
    ) -> Result<Self, StyloDomError> {
        Self::build_shared_with_base_url(
            snapshot,
            root,
            true,
            QuirksMode::NoQuirks,
            "https://spinon.invalid/document.html",
            false,
            true,
        )
    }

    fn build_shared_with_base_url(
        snapshot: Arc<HostDocumentSnapshot>,
        root: HostNodeHandle,
        is_html_document: bool,
        quirks_mode: QuirksMode,
        document_base_url: &str,
        root_matches_root_pseudo: bool,
        synthetic_html_document: bool,
    ) -> Result<Self, StyloDomError> {
        let document_base_url =
            Url::parse(document_base_url).map_err(|_| StyloDomError::InvalidDocumentBaseUrl)?;
        if snapshot.parent(root) != Some(HostParent::Root)
            || !matches!(
                snapshot.node(root).map(|node| node.kind()),
                Some(HostNodeKind::Element(_))
            )
        {
            return Err(StyloDomError::InvalidRoot);
        }

        let mut members = BTreeSet::new();
        let mut elements = BTreeMap::new();
        let shared_lock = SharedRwLock::new();
        let mut pending = vec![root];

        while let Some(handle) = pending.pop() {
            if !members.insert(handle.id()) {
                continue;
            }

            let Some(node) = snapshot.node(handle) else {
                return Err(StyloDomError::InvalidRoot);
            };
            if let HostNodeKind::Element(element) = node.kind() {
                if is_html_document
                    && element.namespace() == "http://www.w3.org/1999/xhtml"
                    && has_ambiguous_html_attribute_names(element)
                {
                    return Err(StyloDomError::AmbiguousHtmlAttributeNames { node: handle.id() });
                }
                elements.insert(
                    handle.id(),
                    StyloElementData::from_host_element(
                        element,
                        is_html_document,
                        &document_base_url,
                        &shared_lock,
                        quirks_mode,
                    ),
                );
            }
            if let Some(children) = snapshot.children(handle) {
                pending.extend(children);
            }
        }

        Ok(Self {
            snapshot,
            root,
            root_matches_root_pseudo,
            is_html_document,
            quirks_mode,
            members,
            elements,
            synthetic_html: synthetic_html_document
                .then(|| StyloElementData::synthetic_html("html")),
            synthetic_body: synthetic_html_document
                .then(|| StyloElementData::synthetic_html("body")),
            shared_lock,
            document_base_url,
        })
    }

    /// 이 view의 가상 Document node를 반환합니다.
    pub fn document(&self) -> StyloDocument<'_> {
        StyloDocument { view: self }
    }

    /// 선택한 유일한 문서 루트를 반환합니다.
    pub fn root_element(&self) -> StyloElement<'_> {
        if self.synthetic_html.is_some() {
            StyloElement::new(self, StyloElementRef::SyntheticHtml)
        } else {
            self.mount_root_element()
        }
    }

    /// layout에서 선택한 실제 HostDocument mount 요소를 반환합니다.
    pub fn mount_root_element(&self) -> StyloElement<'_> {
        StyloElement::new(self, StyloElementRef::Host(self.root))
    }

    /// 선택한 루트 하위 트리의 노드만 반환합니다.
    pub fn node(&self, handle: HostNodeHandle) -> Option<StyloNode<'_>> {
        (self.members.contains(&handle.id()) && self.snapshot.node(handle).is_some())
            .then_some(StyloNode::new(self, Some(handle)))
    }

    /// 선택한 루트 하위 트리의 요소만 반환합니다.
    pub fn element(&self, handle: HostNodeHandle) -> Option<StyloElement<'_>> {
        (self.members.contains(&handle.id())
            && self.snapshot.node(handle).is_some()
            && self.elements.contains_key(&handle.id()))
        .then(|| StyloElement::new(self, StyloElementRef::Host(handle)))
    }

    /// view가 고정한 문서 revision입니다.
    pub fn document_revision(&self) -> spinon_core::DocumentRevision {
        self.snapshot.document_revision()
    }

    /// 이 view가 고정한 문서 generation입니다.
    pub fn generation(&self) -> spinon_core::DocumentGeneration {
        self.snapshot.generation()
    }

    /// view가 고정한 연결 표시 트리 revision입니다.
    pub fn render_tree_revision(&self) -> spinon_core::RenderTreeRevision {
        self.snapshot.render_tree_revision()
    }

    /// 이 view가 사용하는 문서 모드입니다.
    pub const fn is_html_document(&self) -> bool {
        self.is_html_document
    }

    pub fn document_base_url(&self) -> &Url {
        &self.document_base_url
    }

    pub(super) fn snapshot(&self) -> &HostDocumentSnapshot {
        &self.snapshot
    }

    pub(super) const fn root_handle(&self) -> HostNodeHandle {
        self.root
    }

    pub(super) const fn root_matches_root_pseudo(&self) -> bool {
        self.root_matches_root_pseudo
    }

    pub(super) const fn quirks_mode(&self) -> QuirksMode {
        self.quirks_mode
    }

    pub(super) fn element_data(&self, element_ref: StyloElementRef) -> &StyloElementData {
        match element_ref {
            StyloElementRef::Host(handle) => self
                .elements
                .get(&handle.id())
                .expect("Host StyloElement은 view에 속한 요소여야 합니다"),
            StyloElementRef::SyntheticHtml => self
                .synthetic_html
                .as_ref()
                .expect("합성 HTML 루트가 있는 view여야 합니다"),
            StyloElementRef::SyntheticBody => self
                .synthetic_body
                .as_ref()
                .expect("합성 body가 있는 view여야 합니다"),
        }
    }

    pub(super) const fn has_synthetic_html_document(&self) -> bool {
        self.synthetic_html.is_some()
    }

    pub(super) fn synthetic_body_element(&self) -> Option<StyloElement<'_>> {
        self.synthetic_body
            .as_ref()
            .map(|_| StyloElement::new(self, StyloElementRef::SyntheticBody))
    }

    pub(super) fn inline_style_sources(&self) -> impl Iterator<Item = (NodeId, &str)> {
        self.elements.iter().filter_map(|(node_id, element)| {
            element
                .inline_style_text
                .as_deref()
                .map(|css| (*node_id, css))
        })
    }

    pub(super) fn is_member(&self, handle: HostNodeHandle) -> bool {
        self.members.contains(&handle.id())
    }

    pub(super) fn shared_lock(&self) -> &SharedRwLock {
        &self.shared_lock
    }
}

fn has_ambiguous_html_attribute_names(element: &spinon_core::HostElement) -> bool {
    let mut names = BTreeSet::new();
    element
        .attributes()
        .keys()
        .filter(|name| name.namespace().unwrap_or("").is_empty())
        .any(|name| !names.insert(name.local_name().to_ascii_lowercase()))
}

/// Stylo의 `TDocument`에 대응하는 가상 문서 wrapper입니다.
#[derive(Clone, Copy)]
pub struct StyloDocument<'a> {
    pub(super) view: &'a StyloDocumentView,
}

impl<'a> StyloDocument<'a> {
    /// 이 문서의 유일한 요소 루트를 반환합니다.
    pub fn document_element(self) -> StyloElement<'a> {
        self.view.root_element()
    }

    /// 이 문서의 가상 node를 반환합니다.
    pub fn as_node(self) -> StyloNode<'a> {
        StyloNode::new(self.view, None)
    }
}

impl fmt::Debug for StyloDocument<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StyloDocument")
            .field("generation", &self.view.snapshot.generation())
            .field("revision", &self.view.snapshot.document_revision())
            .finish()
    }
}

impl PartialEq for StyloDocument<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.view, other.view)
    }
}

impl Eq for StyloDocument<'_> {}

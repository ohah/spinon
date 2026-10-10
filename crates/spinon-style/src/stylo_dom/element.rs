use std::{cell::Cell, fmt, hash::Hash, ops::Deref};

use selectors::matching::ElementSelectorFlags;
use spinon_core::{ElementState as HostElementState, HostElement, HostNodeHandle};
use style::{
    Atom, LocalName, Namespace, data::ElementDataWrapper, properties::PropertyDeclarationBlock,
    servo_arc::Arc, shared_lock::Locked, values::AtomIdent,
};
use stylo_dom::ElementState;
use url::Url;

use super::StyloDocumentView;
use crate::stylesheet_registry::{CssParseDiagnostic, parse_inline_style_attribute};

pub(super) fn atom_text<T>(atom: &T) -> &str
where
    T: Deref<Target = str>,
{
    atom.deref()
}

/// 요소 속성의 Stylo용 UTF-8 사본입니다.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct StyloAttribute {
    pub namespace: Namespace,
    pub local_name: LocalName,
    pub value: String,
}

/// 하나의 고정 snapshot 요소에 대한 이름과 별도 스타일 계산 상태입니다.
pub(super) struct StyloElementData {
    pub local_name: LocalName,
    pub namespace: Namespace,
    pub id: Option<Atom>,
    pub classes: Vec<AtomIdent>,
    pub attributes: Vec<StyloAttribute>,
    pub state: ElementState,
    pub selector_flags: Cell<ElementSelectorFlags>,
    pub dirty_descendants: Cell<bool>,
    pub has_snapshot: Cell<bool>,
    pub handled_snapshot: Cell<bool>,
    pub children_to_process: Cell<isize>,
    pub data_present: Cell<bool>,
    pub style_data: ElementDataWrapper,
    pub inline_style: Option<Arc<Locked<PropertyDeclarationBlock>>>,
    pub inline_style_text: Option<String>,
    pub inline_style_diagnostics: Vec<CssParseDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) enum StyloElementRef {
    Host(HostNodeHandle),
    SyntheticHtml,
    SyntheticBody,
}

impl StyloElementData {
    pub fn synthetic_html(local_name: &str) -> Self {
        Self {
            local_name: LocalName::from(local_name),
            namespace: Namespace::from("http://www.w3.org/1999/xhtml"),
            id: None,
            classes: Vec::new(),
            attributes: Vec::new(),
            state: ElementState::empty(),
            selector_flags: Cell::new(ElementSelectorFlags::empty()),
            dirty_descendants: Cell::new(false),
            has_snapshot: Cell::new(false),
            handled_snapshot: Cell::new(false),
            children_to_process: Cell::new(0),
            data_present: Cell::new(false),
            style_data: ElementDataWrapper::default(),
            inline_style: None,
            inline_style_text: None,
            inline_style_diagnostics: Vec::new(),
        }
    }

    pub fn from_host_element(
        element: &HostElement,
        is_html_document: bool,
        document_base_url: &Url,
        shared_lock: &style::shared_lock::SharedRwLock,
        quirks_mode: style::context::QuirksMode,
    ) -> Self {
        let attributes = element
            .attributes()
            .iter()
            .map(|(name, value)| StyloAttribute {
                namespace: Namespace::from(name.namespace().unwrap_or("")),
                local_name: LocalName::from(name.local_name()),
                value: value.to_string_lossy(),
            })
            .collect::<Vec<_>>();

        let html_name_matching =
            is_html_document && element.namespace() == "http://www.w3.org/1999/xhtml";
        let style_attribute = attributes.iter().find(|attribute| {
            atom_text(&attribute.namespace.0).is_empty()
                && attribute_name_matches(&attribute.local_name, "style", html_name_matching)
        });
        let inline_style_text = style_attribute.map(|attribute| attribute.value.clone());
        let (inline_style, inline_style_diagnostics) = if html_name_matching {
            style_attribute.map_or((None, Vec::new()), |attribute| {
                let (declarations, diagnostics) = parse_inline_style_attribute(
                    &attribute.value,
                    document_base_url,
                    shared_lock,
                    quirks_mode,
                );
                (Some(declarations), diagnostics)
            })
        } else {
            (None, Vec::new())
        };
        let id = attributes
            .iter()
            .find(|attribute| {
                atom_text(&attribute.namespace.0).is_empty()
                    && attribute_name_matches(&attribute.local_name, "id", html_name_matching)
            })
            .map(|attribute| Atom::from(attribute.value.as_str()));
        let classes = attributes
            .iter()
            .find(|attribute| {
                atom_text(&attribute.namespace.0).is_empty()
                    && attribute_name_matches(&attribute.local_name, "class", html_name_matching)
            })
            .map(|attribute| {
                attribute
                    .value
                    .split([' ', '\t', '\n', '\r', '\u{000C}'])
                    .filter(|part| !part.is_empty())
                    .map(AtomIdent::from)
                    .collect()
            })
            .unwrap_or_default();

        let mut state = ElementState::empty();
        for host_state in element.states() {
            state |= match host_state {
                HostElementState::Active => ElementState::ACTIVE,
                HostElementState::Checked => ElementState::CHECKED,
                HostElementState::Disabled => ElementState::DISABLED,
                HostElementState::Focus => ElementState::FOCUS,
                HostElementState::FocusVisible => ElementState::FOCUSRING,
                HostElementState::Hover => ElementState::HOVER,
            };
        }

        Self {
            local_name: LocalName::from(element.local_name()),
            namespace: Namespace::from(element.namespace()),
            id,
            classes,
            attributes,
            state,
            selector_flags: Cell::new(ElementSelectorFlags::empty()),
            dirty_descendants: Cell::new(false),
            has_snapshot: Cell::new(false),
            handled_snapshot: Cell::new(false),
            children_to_process: Cell::new(0),
            data_present: Cell::new(false),
            style_data: ElementDataWrapper::default(),
            inline_style,
            inline_style_text,
            inline_style_diagnostics,
        }
    }
}

pub(super) fn attribute_name_matches(
    actual: &LocalName,
    expected: &str,
    ascii_case_insensitive: bool,
) -> bool {
    let actual = atom_text(&actual.0);
    if ascii_case_insensitive {
        actual.eq_ignore_ascii_case(expected)
    } else {
        actual == expected
    }
}

/// 불변 HostDocument 요소를 가리키는 Stylo 요소 wrapper입니다.
#[derive(Clone, Copy)]
pub struct StyloElement<'a> {
    pub(super) view: &'a StyloDocumentView,
    pub(super) element_ref: StyloElementRef,
}

impl<'a> StyloElement<'a> {
    pub(super) const fn new(view: &'a StyloDocumentView, element_ref: StyloElementRef) -> Self {
        Self { view, element_ref }
    }

    pub const fn handle(self) -> Option<HostNodeHandle> {
        match self.element_ref {
            StyloElementRef::Host(handle) => Some(handle),
            StyloElementRef::SyntheticHtml | StyloElementRef::SyntheticBody => None,
        }
    }

    pub fn local_name(self) -> &'a LocalName {
        &self.data().local_name
    }

    pub fn namespace(self) -> &'a Namespace {
        &self.data().namespace
    }

    pub(super) fn data(self) -> &'a StyloElementData {
        self.view.element_data(self.element_ref)
    }

    pub(super) fn host_element(self) -> Option<&'a HostElement> {
        let StyloElementRef::Host(handle) = self.element_ref else {
            return None;
        };
        let node = self.view.snapshot().node(handle)?;
        let spinon_core::HostNodeKind::Element(element) = node.kind() else {
            return None;
        };
        Some(element)
    }

    pub(super) fn attribute(
        self,
        namespace: &Namespace,
        local_name: &LocalName,
    ) -> Option<&'a str> {
        let html_name_matching = self.view.is_html_document()
            && atom_text(&self.data().namespace.0) == "http://www.w3.org/1999/xhtml"
            && atom_text(&namespace.0).is_empty();
        self.data()
            .attributes
            .iter()
            .find(|attribute| {
                attribute.namespace == *namespace
                    && (attribute.local_name == *local_name
                        || html_name_matching
                            && attribute_name_matches(
                                &attribute.local_name,
                                atom_text(&local_name.0),
                                true,
                            ))
            })
            .map(|attribute| attribute.value.as_str())
    }

    pub(super) fn has_state(self, state: ElementState) -> bool {
        self.data().state.contains(state)
    }
}

impl fmt::Debug for StyloElement<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StyloElement")
            .field("element_ref", &self.element_ref)
            .field("local_name", &self.data().local_name)
            .finish()
    }
}

impl PartialEq for StyloElement<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.view, other.view) && self.element_ref == other.element_ref
    }
}

impl Eq for StyloElement<'_> {}

impl Hash for StyloElement<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::from_ref(self.view).hash(state);
        self.element_ref.hash(state);
    }
}

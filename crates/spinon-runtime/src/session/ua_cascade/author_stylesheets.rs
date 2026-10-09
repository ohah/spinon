use spinon_core::{HostDocumentSnapshot, HostElement, HostNodeHandle, HostNodeKind};
use spinon_style::{CssOrigin, StylesheetSource};
use std::fmt;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const DOCUMENT_BASE_URL: &str = "https://spinon.invalid/document.html";

#[derive(Debug, Eq, PartialEq)]
pub(super) enum RuntimeAuthorStylesheetError {
    InvalidText { node_id: u64 },
    UnsupportedNamespace { node_id: u64, namespace: String },
    UnsupportedMedia { node_id: u64, media: String },
    ExternalStylesheetLink { node_id: u64 },
}

impl fmt::Display for RuntimeAuthorStylesheetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidText { node_id } => write!(
                formatter,
                "HTML style 요소 {node_id}의 CSS 본문이 유효한 UTF-16 문자열이 아닙니다"
            ),
            Self::UnsupportedNamespace { node_id, namespace } => write!(
                formatter,
                "style 요소 {node_id}의 namespace는 현재 runtime 입력 범위가 아닙니다: {namespace}"
            ),
            Self::UnsupportedMedia { node_id, media } => write!(
                formatter,
                "style 요소 {node_id}의 media 조건은 지원하지 않습니다: {media}"
            ),
            Self::ExternalStylesheetLink { node_id } => write!(
                formatter,
                "link 요소 {node_id}의 외부 stylesheet 자원은 아직 지원하지 않습니다"
            ),
        }
    }
}

pub(super) fn collect_runtime_author_stylesheets(
    snapshot: &HostDocumentSnapshot,
) -> Result<Vec<StylesheetSource>, RuntimeAuthorStylesheetError> {
    let mut sources = Vec::new();
    let mut pending = snapshot.root_children().collect::<Vec<_>>();
    pending.reverse();

    while let Some(handle) = pending.pop() {
        let Some(node) = snapshot.node(handle) else {
            continue;
        };
        if let HostNodeKind::Element(element) = node.kind() {
            collect_element_stylesheet(snapshot, handle, element, &mut sources)?;
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children.collect::<Vec<_>>().into_iter().rev());
        }
    }

    Ok(sources)
}

fn collect_element_stylesheet(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
    element: &HostElement,
    sources: &mut Vec<StylesheetSource>,
) -> Result<(), RuntimeAuthorStylesheetError> {
    let local_name = element.local_name();
    if local_name.eq_ignore_ascii_case("style") {
        if element.namespace() != HTML_NAMESPACE {
            return Err(RuntimeAuthorStylesheetError::UnsupportedNamespace {
                node_id: handle.id().get(),
                namespace: element.namespace().to_owned(),
            });
        }
        if !style_type_is_css(element) || !style_media_is_supported(element, handle)? {
            return Ok(());
        }
        let content =
            snapshot
                .text_content(handle)
                .ok_or(RuntimeAuthorStylesheetError::InvalidText {
                    node_id: handle.id().get(),
                })?;
        let css = String::from_utf16(content.code_units()).map_err(|_| {
            RuntimeAuthorStylesheetError::InvalidText {
                node_id: handle.id().get(),
            }
        })?;
        sources.push(StylesheetSource {
            id: format!(
                "host-style:{}:{}",
                snapshot.generation().get(),
                handle.id().get()
            ),
            base_url: DOCUMENT_BASE_URL.to_owned(),
            origin: CssOrigin::Author,
            css,
        });
    } else if local_name.eq_ignore_ascii_case("link")
        && element.namespace() == HTML_NAMESPACE
        && attribute(element, "rel").is_some_and(|value| {
            value
                .to_string_lossy()
                .split_ascii_whitespace()
                .any(|token| token.eq_ignore_ascii_case("stylesheet"))
        })
    {
        return Err(RuntimeAuthorStylesheetError::ExternalStylesheetLink {
            node_id: handle.id().get(),
        });
    }

    Ok(())
}

fn style_type_is_css(element: &HostElement) -> bool {
    let Some(type_value) = attribute(element, "type") else {
        return true;
    };
    let value = type_value.to_string_lossy();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return true;
    }
    let essence = trimmed
        .split_once(';')
        .map_or(trimmed, |(essence, _)| essence.trim());
    essence.eq_ignore_ascii_case("text/css")
}

fn style_media_is_supported(
    element: &HostElement,
    handle: HostNodeHandle,
) -> Result<bool, RuntimeAuthorStylesheetError> {
    let Some(media) = attribute(element, "media") else {
        return Ok(true);
    };
    let value = media.to_string_lossy();
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.eq_ignore_ascii_case("all")
        || trimmed.eq_ignore_ascii_case("screen")
    {
        return Ok(true);
    }
    Err(RuntimeAuthorStylesheetError::UnsupportedMedia {
        node_id: handle.id().get(),
        media: trimmed.to_owned(),
    })
}

fn attribute<'a>(element: &'a HostElement, name: &str) -> Option<&'a spinon_core::DomString> {
    element.attributes().iter().find_map(|(candidate, value)| {
        (candidate.namespace().is_none() && candidate.local_name().eq_ignore_ascii_case(name))
            .then_some(value)
    })
}

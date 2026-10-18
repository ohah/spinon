use std::collections::BTreeMap;

use spinon_core::{
    EnvironmentRevision, HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId,
    StyleRevision,
};
use spinon_text_layout::{InlineRunStyle, ParagraphStyle};

use crate::{
    LayoutError, LayoutInput, LayoutInputRevision, LayoutNode, LayoutSourceRevision, LayoutStyle,
    RootSizingPolicy, Viewport,
};

#[path = "host_document/text_paragraphs.rs"]
mod text_paragraphs;

struct HostDocumentLayoutProfile<'a> {
    root_sizing: RootSizingPolicy,
    paragraph_styles: Option<&'a BTreeMap<NodeId, ParagraphStyle>>,
    inline_run_styles: Option<&'a BTreeMap<NodeId, InlineRunStyle>>,
    ignore_collapsible_whitespace_text: bool,
}

/// HostDocument layout에서 paragraph와 inline run 스타일 입력을 함께 전달합니다.
pub struct HostTextStyleProfile<'a> {
    /// 각 paragraph element의 computed text style입니다.
    pub paragraphs: &'a BTreeMap<NodeId, ParagraphStyle>,
    /// 각 inline run element의 computed font metric style입니다.
    pub inline_runs: &'a BTreeMap<NodeId, InlineRunStyle>,
}

impl LayoutInput {
    /// HostDocument의 요소 하위 트리를 순서 보존 레이아웃 snapshot으로 만듭니다.
    ///
    /// root는 HostRoot 직속 요소여야 합니다. 텍스트 intrinsic measurement가 없으므로
    /// 보이는 텍스트는 거부하고 `display:none` 아래 텍스트는 입력에서 제외합니다.
    pub fn from_host_document(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_root_sizing(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            RootSizingPolicy::Match,
        )
    }

    /// HostRoot 직속 요소를 viewport containing block 안에서 CSS 크기로 계산합니다.
    pub fn from_host_document_with_viewport_containing_block(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_root_sizing(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            RootSizingPolicy::ResolveWithin,
        )
    }

    /// Block formatting을 위해 가상 viewport containing block을 둡니다.
    /// HostRoot 직속 요소도 일반 자식 Block처럼 margin과 auto width를 계산합니다.
    pub fn from_host_document_with_block_formatting_viewport(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_root_sizing(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            RootSizingPolicy::BlockFormatting,
        )
    }

    /// 고정 text profile이 제공한 Block paragraph를 source order와 제한 inline span에 연결합니다.
    ///
    /// paragraph 안에는 direct text, 두 단계 HTML span, 직접 Block child를 허용합니다.
    pub fn from_host_document_with_text_paragraphs(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        let inline_run_styles = BTreeMap::new();
        Self::from_host_document_with_text_paragraphs_and_run_styles(
            snapshot,
            root,
            viewport,
            styles,
            HostTextStyleProfile {
                paragraphs: paragraph_styles,
                inline_runs: &inline_run_styles,
            },
            style_revision,
            environment_revision,
        )
    }

    /// Root HTML text profile에서 block 사이 collapsible whitespace를 제외해 문단을 계산합니다.
    pub fn from_host_document_with_viewport_containing_block_and_text_paragraphs_ignoring_collapsible_whitespace(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        paragraph_styles: &BTreeMap<NodeId, ParagraphStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        let inline_run_styles = BTreeMap::new();
        Self::from_host_document_with_text_profile(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            HostDocumentLayoutProfile {
                root_sizing: RootSizingPolicy::ResolveWithin,
                paragraph_styles: Some(paragraph_styles),
                inline_run_styles: Some(&inline_run_styles),
                ignore_collapsible_whitespace_text: true,
            },
        )
    }

    /// Root HTML에서 block·inline run의 computed 글꼴을 연결해 문단을 계산합니다.
    pub fn from_host_document_with_viewport_containing_block_and_text_paragraphs_and_run_styles_ignoring_collapsible_whitespace(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        text_styles: HostTextStyleProfile<'_>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_text_profile(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            HostDocumentLayoutProfile {
                root_sizing: RootSizingPolicy::ResolveWithin,
                paragraph_styles: Some(text_styles.paragraphs),
                inline_run_styles: Some(text_styles.inline_runs),
                ignore_collapsible_whitespace_text: true,
            },
        )
    }

    /// 제한된 C15 text profile에서 inline 글꼴·크기·line-height를 명시합니다.
    pub(crate) fn from_host_document_with_text_paragraphs_and_run_styles(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        text_styles: HostTextStyleProfile<'_>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_text_profile(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            HostDocumentLayoutProfile {
                root_sizing: RootSizingPolicy::Match,
                paragraph_styles: Some(text_styles.paragraphs),
                inline_run_styles: Some(text_styles.inline_runs),
                ignore_collapsible_whitespace_text: false,
            },
        )
    }

    fn from_host_document_with_root_sizing(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
        root_sizing: RootSizingPolicy,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_text_profile(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            HostDocumentLayoutProfile {
                root_sizing,
                paragraph_styles: None,
                inline_run_styles: None,
                ignore_collapsible_whitespace_text: false,
            },
        )
    }

    /// HTML root scene에서 접을 수 있는 공백 text node를 layout box에서 제외합니다.
    pub fn from_host_document_with_viewport_containing_block_ignoring_collapsible_whitespace(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
    ) -> Result<Self, LayoutError> {
        Self::from_host_document_with_text_profile(
            snapshot,
            root,
            viewport,
            styles,
            style_revision,
            environment_revision,
            HostDocumentLayoutProfile {
                root_sizing: RootSizingPolicy::ResolveWithin,
                paragraph_styles: None,
                inline_run_styles: None,
                ignore_collapsible_whitespace_text: true,
            },
        )
    }

    fn from_host_document_with_text_profile(
        snapshot: &HostDocumentSnapshot,
        root: HostNodeHandle,
        viewport: Viewport,
        styles: &BTreeMap<NodeId, LayoutStyle>,
        style_revision: StyleRevision,
        environment_revision: EnvironmentRevision,
        profile: HostDocumentLayoutProfile<'_>,
    ) -> Result<Self, LayoutError> {
        let HostDocumentLayoutProfile {
            root_sizing,
            paragraph_styles,
            inline_run_styles,
            ignore_collapsible_whitespace_text,
        } = profile;
        let root_node = snapshot
            .node(root)
            .ok_or(LayoutError::InvalidHostDocumentRoot(root.id()))?;
        if snapshot.parent(root) != Some(HostParent::Root)
            || !matches!(root_node.kind(), HostNodeKind::Element(_))
        {
            return Err(LayoutError::InvalidHostDocumentRoot(root.id()));
        }

        let empty_inline_run_styles = BTreeMap::new();
        let inline_run_styles = inline_run_styles.unwrap_or(&empty_inline_run_styles);
        let collected_text = paragraph_styles
            .map(|paragraph_styles| {
                text_paragraphs::collect(
                    snapshot,
                    root,
                    styles,
                    paragraph_styles,
                    inline_run_styles,
                )
            })
            .transpose()?
            .unwrap_or_default();
        let inline_nodes = collected_text.inline_nodes;
        let text_source_nodes = collected_text.text_source_nodes;
        let excluded_layout_nodes = collected_text.excluded_layout_nodes;
        let mut text_paragraphs = collected_text
            .paragraphs
            .into_iter()
            .map(|(node, paragraph)| (node, crate::TextParagraphInput::Paragraph(paragraph)))
            .collect::<BTreeMap<_, _>>();
        for (node, flow) in collected_text.flows {
            if text_paragraphs
                .insert(node, crate::TextParagraphInput::Flow(flow))
                .is_some()
            {
                return Err(LayoutError::DuplicateTextParagraph(node));
            }
        }

        let mut nodes = Vec::new();
        let mut included = excluded_layout_nodes.clone();
        let mut pending = vec![(root, false)];
        while let Some((handle, ancestor_hidden)) = pending.pop() {
            if !included.insert(handle.id()) {
                return Err(LayoutError::DuplicateNode(handle.id()));
            }

            let node = snapshot
                .node(handle)
                .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?;
            if let HostNodeKind::Text(text) = node.kind() {
                if ancestor_hidden {
                    continue;
                }
                if ignore_collapsible_whitespace_text && text.is_only_collapsible_html_whitespace()
                {
                    continue;
                }
                return Err(LayoutError::UnsupportedTextNode(handle.id()));
            }

            let style = styles
                .get(&handle.id())
                .copied()
                .ok_or(LayoutError::MissingStyle(handle.id()))?;
            let hidden = ancestor_hidden || style.display == crate::LayoutDisplay::None;

            let child_handles = snapshot
                .children(handle)
                .ok_or(LayoutError::InvalidHostDocumentRoot(handle.id()))?
                .collect::<Vec<_>>();
            let mut children = Vec::with_capacity(child_handles.len());
            for child in &child_handles {
                let Some(child_node) = snapshot.node(*child) else {
                    return Err(LayoutError::MissingChild {
                        parent: handle.id(),
                        child: child.id(),
                    });
                };
                if let HostNodeKind::Text(text) = child_node.kind() {
                    if !hidden && !text_source_nodes.contains(&child.id()) {
                        if ignore_collapsible_whitespace_text
                            && text.is_only_collapsible_html_whitespace()
                        {
                            continue;
                        }
                        return Err(LayoutError::UnsupportedTextNode(child.id()));
                    }
                    continue;
                }
                if inline_nodes.contains(&child.id()) {
                    continue;
                }
                if excluded_layout_nodes.contains(&child.id()) {
                    continue;
                }
                children.push(child.id());
            }

            if inline_nodes.contains(&handle.id()) {
                pending.extend(
                    child_handles
                        .into_iter()
                        .rev()
                        .filter(|child| {
                            !excluded_layout_nodes.contains(&child.id())
                                && snapshot.node(*child).is_some_and(|node| {
                                    matches!(node.kind(), HostNodeKind::Element(_))
                                })
                        })
                        .map(|child| (child, hidden)),
                );
                continue;
            }

            nodes.push(LayoutNode {
                id: handle.id(),
                children,
                style,
            });
            pending.extend(
                child_handles
                    .into_iter()
                    .rev()
                    .filter(|child| {
                        !excluded_layout_nodes.contains(&child.id())
                            && snapshot
                                .node(*child)
                                .is_some_and(|node| matches!(node.kind(), HostNodeKind::Element(_)))
                    })
                    .map(|child| (child, hidden)),
            );
        }

        if let Some(id) = styles.keys().find(|id| !included.contains(*id)) {
            return Err(LayoutError::UnknownStyleNode(*id));
        }
        nodes.sort_by_key(|node| node.id);

        Ok(Self {
            root: root.id(),
            revision: LayoutInputRevision::new(
                LayoutSourceRevision::HostDocument {
                    generation: snapshot.generation(),
                    document: snapshot.document_revision(),
                    render_tree: snapshot.render_tree_revision(),
                },
                style_revision,
                environment_revision,
            ),
            viewport,
            root_sizing,
            nodes,
            css_math: Vec::new(),
            positioning: Default::default(),
            grid_containers: Default::default(),
            grid_items: Default::default(),
            text_paragraphs,
        })
    }
}

#[cfg(test)]
#[path = "host_document_tests.rs"]
mod tests;

use spinon_core::NodeId;
use spinon_text_layout::ParagraphInput;

use crate::LayoutGridItemTextSource;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TextParagraphInput {
    Paragraph(ParagraphInput),
    AnonymousGridText {
        grid_container: NodeId,
        paragraph: ParagraphInput,
        root_html_default: bool,
        source_nodes: Vec<LayoutGridItemTextSource>,
    },
    Flow(TextFlowInput),
}

impl TextParagraphInput {
    #[cfg(test)]
    pub(crate) fn paragraph(&self) -> Option<&ParagraphInput> {
        match self {
            Self::Paragraph(paragraph) | Self::AnonymousGridText { paragraph, .. } => {
                Some(paragraph)
            }
            Self::Flow(_) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TextFlowInput {
    pub owner: NodeId,
    pub items: Vec<TextFlowItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TextFlowItem {
    Paragraph(ParagraphInput),
    Block(NodeId),
}

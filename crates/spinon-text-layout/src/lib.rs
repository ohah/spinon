//! 고정 글꼴·제한된 inline profile을 위한 Parley adapter입니다.

mod engine;
mod error;
mod model;
mod normalize;
mod snapshot;

pub use engine::TextLayoutEngine;
pub use error::TextLayoutError;
pub use model::{
    InlineBoxStyle, InlineEdges, InlineRunStyle, ParagraphInput, ParagraphStyle, TextSource,
    WhiteSpaceMode, WordBreakMode,
};
pub use snapshot::{
    FontMetricsSnapshot, FontRegistryRevision, InlineFragment, TextGlyph, TextLayoutSnapshot,
    TextLine, TextRect, TextRun, TextSourceRange,
};

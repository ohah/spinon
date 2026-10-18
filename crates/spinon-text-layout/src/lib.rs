//! 고정 글꼴·제한된 inline profile을 위한 Parley adapter입니다.

mod engine;
mod error;
mod model;
mod normalize;
mod raster;
mod snapshot;

pub use engine::TextLayoutEngine;
pub use error::TextLayoutError;
pub use model::{
    FontStyleMode, InlineBoxStyle, InlineEdges, InlineRunStyle, LigatureMode, LineBreakMode,
    ParagraphInput, ParagraphStyle, TextDirection, TextSource, WhiteSpaceMode, WordBreakMode,
};
pub use raster::{GlyphBitmap, GlyphBitmapContent, TextGlyphRaster, TextGlyphRasterSnapshot};
pub use snapshot::{
    FontMetricsSnapshot, FontRegistryRevision, InlineFragment, TextGlyph, TextLayoutSnapshot,
    TextLine, TextRect, TextRun, TextSourceRange,
};

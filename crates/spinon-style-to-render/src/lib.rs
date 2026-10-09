//! S04 computed style과 layout 결과를 플랫폼 중립 RenderSnapshot으로 변환합니다.

mod adapter;
mod error;
mod runtime;

#[cfg(test)]
mod tests;

pub use adapter::{
    CurrentLayoutInputs, FixtureNodeMapping, RenderFixtureProvenance,
    build_s04_static_render_snapshot,
};
pub use error::StyleRenderError;
pub use runtime::build_runtime_render_snapshot;

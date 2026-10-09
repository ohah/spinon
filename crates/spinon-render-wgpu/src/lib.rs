//! `RuntimeRenderSnapshot`를 모바일 WGPU 표면에 그리는 제품 렌더 경계입니다.

mod geometry;
mod readback;
mod renderer;

pub use readback::readback_runtime_scene_samples;
pub use renderer::{PendingRuntimeSurface, RuntimeRenderer, RuntimeRendererError};

mod bootstrap;
mod host;
mod root_html;
mod session;
mod v8;

pub use root_html::RootHtmlEnvironmentSnapshot;

/// 최대 UTF-8 루트 HTML source 크기입니다.
pub const ROOT_HTML_SOURCE_MAX_BYTES: usize = 1_048_576;

pub use bootstrap::{BootstrapSmokeError, run_bootstrap_smoke};
#[cfg(any(target_os = "android", target_os = "ios", test))]
pub use session::run_priority_fairness_probe;
pub use session::{
    MemoryPressureLevel, OperationResponse, RuntimeLayoutCompleted, RuntimeLayoutFailure,
    RuntimeLayoutFrame, RuntimeLayoutSnapshot, RuntimeLayoutState, RuntimeSession,
    RuntimeUaCascadeCompleted, RuntimeUaCascadeError, RuntimeUaCascadeKey, RuntimeUaCascadeRoot,
    RuntimeUaCascadeSnapshot, RuntimeUaCascadeState, TaskPriority, run_priority_probe,
    run_shutdown_probe,
};

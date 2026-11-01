use std::collections::BTreeMap;

use spinon_core::{
    EnvironmentRevision, HostDocumentSnapshot, HostNodeHandle, NodeId, StyleRevision,
};

use crate::{LayoutError, LayoutInput, LayoutStyle, RootSizingPolicy, Viewport};

use super::{HostDocumentLayoutProfile, HostTextStyleProfile};

impl LayoutInput {
    /// C11.4 fixture에서만 Grid direct text를 제한 anonymous item으로 계산합니다.
    pub fn from_host_document_with_viewport_containing_block_and_c11_4_anonymous_grid_text_items(
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
                allow_c11_4_anonymous_grid_text_items: true,
                allow_root_html_default_grid_text_items: false,
            },
        )
    }
}

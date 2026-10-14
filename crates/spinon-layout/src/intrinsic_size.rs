use std::collections::BTreeMap;

use spinon_core::{IntrinsicSizeRevision, NodeId};

use crate::{LayoutError, LayoutSourceRevision};

/// HTML replaced leaf에 decoder가 제공한 자연 콘텐츠 크기(CSS px)입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutIntrinsicSize {
    pub width: f32,
    pub height: f32,
}

impl LayoutIntrinsicSize {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

/// 같은 layout source의 HTML image 자연 크기와 revision을 묶은 immutable 입력입니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutIntrinsicSizeSnapshot {
    source: LayoutSourceRevision,
    revision: IntrinsicSizeRevision,
    sizes: BTreeMap<NodeId, LayoutIntrinsicSize>,
}

impl LayoutIntrinsicSizeSnapshot {
    pub fn new(
        source: LayoutSourceRevision,
        revision: IntrinsicSizeRevision,
        entries: impl IntoIterator<Item = (NodeId, LayoutIntrinsicSize)>,
    ) -> Result<Self, LayoutError> {
        let mut sizes = BTreeMap::new();
        for (node, size) in entries {
            if sizes.insert(node, size).is_some() {
                return Err(LayoutError::DuplicateIntrinsicSizeNode(node));
            }
        }
        Ok(Self {
            source,
            revision,
            sizes,
        })
    }

    pub const fn source(&self) -> LayoutSourceRevision {
        self.source
    }

    pub const fn revision(&self) -> IntrinsicSizeRevision {
        self.revision
    }

    pub fn sizes(&self) -> &BTreeMap<NodeId, LayoutIntrinsicSize> {
        &self.sizes
    }
}

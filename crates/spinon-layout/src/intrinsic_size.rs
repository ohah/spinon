use std::collections::BTreeMap;

use spinon_core::{IntrinsicSizeRevision, NodeId};

use crate::{LayoutError, LayoutSourceRevision};

/// HTML replaced leaf에 완료된 리소스가 제공한 자연 크기 metadata입니다.
/// 자연 너비·높이·비율은 각각 없을 수 있습니다. 세 값이 모두 `None`인 entry는
/// 성공적으로 조사했지만 자연 sizing 정보가 없는 리소스입니다. `Some(0), Some(0)`은
/// 요청 완료 전의 크기 없는 pending image를 나타내며, snapshot entry 자체가 없는 상태와 다릅니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutIntrinsicSize {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub aspect_ratio: Option<f32>,
}

impl LayoutIntrinsicSize {
    pub const fn new(width: f32, height: f32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
            aspect_ratio: None,
        }
    }

    pub const fn from_natural_parts(
        width: Option<f32>,
        height: Option<f32>,
        aspect_ratio: Option<f32>,
    ) -> Self {
        Self {
            width,
            height,
            aspect_ratio,
        }
    }

    pub const fn no_natural_dimensions() -> Self {
        Self::from_natural_parts(None, None, None)
    }

    pub fn natural_aspect_ratio(self) -> Option<f32> {
        match (self.width, self.height) {
            (Some(width), Some(height)) if width > 0.0 && height > 0.0 => Some(width / height),
            _ => self.aspect_ratio,
        }
    }
}

/// 같은 layout source의 replaced-resource natural metadata와 revision을 묶은 immutable 입력입니다.
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

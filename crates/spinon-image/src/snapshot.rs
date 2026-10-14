use std::{collections::BTreeMap, fmt, sync::Arc};

use spinon_core::{HostDocumentSnapshot, IntrinsicSizeRevision, NodeId};
use spinon_layout::{LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot, LayoutSourceRevision};

use crate::{DecodedImage, ImageResourceCompletion};

const MAX_SNAPSHOT_IMAGES: usize = 4096;
const MAX_SNAPSHOT_IMAGE_BYTES: usize = 64 * 1024 * 1024;

/// 한 HostDocument source와 resource revision에 묶인 디코드 이미지 snapshot입니다.
#[derive(Clone, Debug)]
pub struct ImageResourceSnapshot {
    source: LayoutSourceRevision,
    revision: IntrinsicSizeRevision,
    images: BTreeMap<NodeId, Arc<DecodedImage>>,
}

impl ImageResourceSnapshot {
    /// 현재 문서에서 나온 요청별 decode 완료를 불변 snapshot으로 묶습니다.
    pub fn new(
        document: &HostDocumentSnapshot,
        revision: IntrinsicSizeRevision,
        entries: impl IntoIterator<Item = ImageResourceCompletion>,
    ) -> Result<Self, ImageSnapshotError> {
        let source = LayoutSourceRevision::HostDocument {
            generation: document.generation(),
            document: document.document_revision(),
            render_tree: document.render_tree_revision(),
        };
        let mut images = BTreeMap::new();
        let mut total_image_bytes = 0;
        for completion in entries {
            let request = completion.request;
            let image = completion.image;
            if request.element().generation() != document.generation() {
                return Err(ImageSnapshotError::StaleRequest(request.element().id()));
            }
            let current_matches = request
                .matches_current_src(document)
                .map_err(|_| ImageSnapshotError::InvalidRequest(request.element().id()))?;
            if !current_matches {
                return Err(ImageSnapshotError::SourceChanged(request.element().id()));
            }
            if request.source() != source {
                return Err(ImageSnapshotError::StaleRequest(request.element().id()));
            }
            let node = request.element().id();
            if images.contains_key(&node) {
                return Err(ImageSnapshotError::DuplicateNode(node));
            }
            let (next_image_count, next_image_bytes) =
                next_snapshot_budget(images.len(), total_image_bytes, image.byte_len())?;
            images.insert(node, image);
            total_image_bytes = next_image_bytes;
            debug_assert_eq!(images.len(), next_image_count);
        }
        Ok(Self {
            source,
            revision,
            images,
        })
    }

    /// 이미지가 요청된 HostDocument source stamp를 반환합니다.
    pub const fn source(&self) -> LayoutSourceRevision {
        self.source
    }

    /// 자원 입력의 단조 증가 revision을 반환합니다.
    pub const fn revision(&self) -> IntrinsicSizeRevision {
        self.revision
    }

    /// 요청 source에 연결된 디코드 이미지를 NodeId 순서로 반환합니다.
    pub fn images(&self) -> &BTreeMap<NodeId, Arc<DecodedImage>> {
        &self.images
    }

    /// PNG pixel 크기를 CSS px 측정값으로 바꾸고 C14.1 snapshot을 만듭니다.
    pub fn to_layout_intrinsic_size_snapshot(
        &self,
    ) -> Result<LayoutIntrinsicSizeSnapshot, spinon_layout::LayoutError> {
        LayoutIntrinsicSizeSnapshot::new(
            self.source,
            self.revision,
            self.images.iter().map(|(node, image)| {
                let dimensions = image.dimensions();
                (
                    *node,
                    LayoutIntrinsicSize::new(dimensions.width() as f32, dimensions.height() as f32),
                )
            }),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// resource snapshot 입력을 거부한 이유입니다.
pub enum ImageSnapshotError {
    StaleRequest(NodeId),
    InvalidRequest(NodeId),
    SourceChanged(NodeId),
    DuplicateNode(NodeId),
    TooManyImages { maximum: usize },
    SnapshotImageBytesExceeded { maximum: usize },
}

fn next_snapshot_budget(
    current_images: usize,
    current_image_bytes: usize,
    next_image_bytes: usize,
) -> Result<(usize, usize), ImageSnapshotError> {
    let image_count = current_images
        .checked_add(1)
        .filter(|count| *count <= MAX_SNAPSHOT_IMAGES)
        .ok_or(ImageSnapshotError::TooManyImages {
            maximum: MAX_SNAPSHOT_IMAGES,
        })?;
    let image_bytes = current_image_bytes
        .checked_add(next_image_bytes)
        .filter(|bytes| *bytes <= MAX_SNAPSHOT_IMAGE_BYTES)
        .ok_or(ImageSnapshotError::SnapshotImageBytesExceeded {
            maximum: MAX_SNAPSHOT_IMAGE_BYTES,
        })?;
    Ok((image_count, image_bytes))
}

impl fmt::Display for ImageSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaleRequest(node) => write!(f, "오래된 이미지 요청입니다: node {node}"),
            Self::InvalidRequest(node) => write!(
                f,
                "현재 문서에서 유효하지 않은 이미지 요청입니다: node {node}"
            ),
            Self::SourceChanged(node) => {
                write!(f, "이미지 요청 이후 src가 바뀌었습니다: node {node}")
            }
            Self::DuplicateNode(node) => {
                write!(f, "이미지 snapshot에 node가 중복되었습니다: node {node}")
            }
            Self::TooManyImages { maximum } => {
                write!(f, "이미지 snapshot 항목 수가 {maximum}개 제한을 넘었습니다")
            }
            Self::SnapshotImageBytesExceeded { maximum } => {
                write!(
                    f,
                    "이미지 snapshot pixel 합계가 {maximum} bytes 제한을 넘었습니다"
                )
            }
        }
    }
}

impl std::error::Error for ImageSnapshotError {}

#[cfg(test)]
mod snapshot_tests;

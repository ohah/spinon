use std::{collections::BTreeMap, fmt, sync::Arc};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, IntrinsicSizeRevision, NodeId};
use spinon_layout::{LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot, LayoutSourceRevision};

use crate::{DecodedImage, ImageDecodeError, ImageResourceCompletion, ImageResourceOutcome};

const MAX_SNAPSHOT_IMAGES: usize = 4096;
const MAX_SNAPSHOT_IMAGE_BYTES: usize = 64 * 1024 * 1024;

/// 한 HostDocument source와 resource revision에 묶인 디코드 이미지 snapshot입니다.
#[derive(Clone, Debug)]
pub struct ImageResourceSnapshot {
    source: LayoutSourceRevision,
    revision: IntrinsicSizeRevision,
    images: BTreeMap<NodeId, Arc<DecodedImage>>,
    failures: BTreeMap<NodeId, Arc<ImageDecodeError>>,
}

impl ImageResourceSnapshot {
    /// 현재 문서에서 나온 요청별 decode 완료를 불변 snapshot으로 묶습니다.
    pub fn new(
        document: &HostDocumentSnapshot,
        revision: IntrinsicSizeRevision,
        entries: impl IntoIterator<Item = ImageResourceCompletion>,
    ) -> Result<Self, ImageSnapshotError> {
        let (snapshot, rejected) = Self::new_with_outcomes(
            document,
            revision,
            entries
                .into_iter()
                .map(ImageResourceCompletion::into_outcome),
        )?;
        if let Some(rejection) = rejected.first() {
            return Err(match rejection.reason {
                ImageResourceRejectionReason::StaleRequest => {
                    ImageSnapshotError::StaleRequest(rejection.request.element().id())
                }
                ImageResourceRejectionReason::InvalidRequest => {
                    ImageSnapshotError::InvalidRequest(rejection.request.element().id())
                }
                ImageResourceRejectionReason::SourceChanged => {
                    ImageSnapshotError::SourceChanged(rejection.request.element().id())
                }
            });
        }
        Ok(snapshot)
    }

    /// 현재 문서에 속한 decode 결과만 적용하고 오래된 완료를 별도로 반환합니다.
    pub fn new_with_outcomes(
        document: &HostDocumentSnapshot,
        revision: IntrinsicSizeRevision,
        entries: impl IntoIterator<Item = ImageResourceOutcome>,
    ) -> Result<(Self, Vec<ImageResourceRejection>), ImageSnapshotError> {
        let source = LayoutSourceRevision::HostDocument {
            generation: document.generation(),
            document: document.document_revision(),
            render_tree: document.render_tree_revision(),
        };
        let mut images = BTreeMap::new();
        let mut failures = BTreeMap::new();
        let mut rejected = Vec::new();
        let mut total_image_bytes = 0;
        let mut accepted_results = 0_usize;
        for completion in entries {
            let (request, result) = completion.into_parts();
            if request.element().generation() != document.generation() {
                rejected.push(ImageResourceRejection {
                    request,
                    reason: ImageResourceRejectionReason::StaleRequest,
                });
                continue;
            }
            let reason = match request.matches_current_src(document) {
                Ok(true) => None,
                Ok(false) => Some(ImageResourceRejectionReason::SourceChanged),
                Err(_) => Some(ImageResourceRejectionReason::InvalidRequest),
            };
            if let Some(reason) = reason {
                rejected.push(ImageResourceRejection { request, reason });
                continue;
            }
            if request.source() != source {
                rejected.push(ImageResourceRejection {
                    request,
                    reason: ImageResourceRejectionReason::StaleRequest,
                });
                continue;
            }
            let node = request.element().id();
            if images.contains_key(&node) || failures.contains_key(&node) {
                return Err(ImageSnapshotError::DuplicateNode(node));
            }
            let next_result_count = accepted_results
                .checked_add(1)
                .filter(|count| *count <= MAX_SNAPSHOT_IMAGES)
                .ok_or(ImageSnapshotError::TooManyImages {
                    maximum: MAX_SNAPSHOT_IMAGES,
                })?;
            match result {
                Ok(image) => {
                    let (_, next_image_bytes) =
                        next_snapshot_budget(images.len(), total_image_bytes, image.byte_len())?;
                    images.insert(node, image);
                    total_image_bytes = next_image_bytes;
                }
                Err(error) => {
                    failures.insert(node, error);
                }
            }
            accepted_results = next_result_count;
        }
        Ok((
            Self {
                source,
                revision,
                images,
                failures,
            },
            rejected,
        ))
    }

    /// 현재 snapshot 다음 resource revision으로 새 결과 집합을 만듭니다.
    pub fn replace_with_outcomes(
        &self,
        document: &HostDocumentSnapshot,
        entries: impl IntoIterator<Item = ImageResourceOutcome>,
    ) -> Result<(Self, Vec<ImageResourceRejection>), ImageSnapshotError> {
        let revision = self
            .revision
            .checked_next()
            .ok_or(ImageSnapshotError::RevisionExhausted)?;
        Self::new_with_outcomes(document, revision, entries)
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

    /// 현재 revision에서 decode에 실패한 node와 원인을 반환합니다.
    pub fn failures(&self) -> &BTreeMap<NodeId, Arc<ImageDecodeError>> {
        &self.failures
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

    /// 현재 document stamp에 pending node의 이전 자연 크기만 다시 연결합니다.
    /// 이전 성공 크기가 없으면 이 source의 초기 pending 자연 크기인 0×0을 기록합니다.
    /// 이미지 bytes와 resource key는 결과 snapshot에 포함되지 않습니다.
    pub fn to_pending_layout_intrinsic_size_snapshot(
        &self,
        document: &HostDocumentSnapshot,
        revision: IntrinsicSizeRevision,
        pending_elements: impl IntoIterator<Item = HostNodeHandle>,
    ) -> Result<LayoutIntrinsicSizeSnapshot, ImageSnapshotError> {
        let source = LayoutSourceRevision::HostDocument {
            generation: document.generation(),
            document: document.document_revision(),
            render_tree: document.render_tree_revision(),
        };
        let same_generation = matches!(
            self.source,
            LayoutSourceRevision::HostDocument { generation, .. }
                if generation == document.generation()
        );
        let mut sizes = Vec::new();
        for element in pending_elements {
            if element.generation() != document.generation()
                || document.node(element).is_none()
                || !document.is_connected(element)
            {
                return Err(ImageSnapshotError::InvalidRequest(element.id()));
            }
            let size = same_generation
                .then(|| self.images.get(&element.id()))
                .flatten()
                .map(|image| {
                    let dimensions = image.dimensions();
                    LayoutIntrinsicSize::new(dimensions.width() as f32, dimensions.height() as f32)
                })
                .unwrap_or_else(|| LayoutIntrinsicSize::new(0.0, 0.0));
            sizes.push((element.id(), size));
        }
        LayoutIntrinsicSizeSnapshot::new(source, revision, sizes).map_err(|error| match error {
            spinon_layout::LayoutError::DuplicateIntrinsicSizeNode(node) => {
                ImageSnapshotError::DuplicatePendingNode(node)
            }
            _ => ImageSnapshotError::InvalidIntrinsicSize,
        })
    }
}

/// 현재 snapshot에 포함되지 않은 오래된 decode 완료입니다.
#[derive(Debug)]
pub struct ImageResourceRejection {
    request: crate::ImageRequest,
    reason: ImageResourceRejectionReason,
}

impl ImageResourceRejection {
    pub fn request(&self) -> &crate::ImageRequest {
        &self.request
    }

    pub const fn reason(&self) -> ImageResourceRejectionReason {
        self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// 현재 문서와 맞지 않는 decode 결과의 분류입니다.
pub enum ImageResourceRejectionReason {
    StaleRequest,
    InvalidRequest,
    SourceChanged,
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
    InvalidIntrinsicSize,
    DuplicatePendingNode(NodeId),
    RevisionExhausted,
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
            Self::InvalidIntrinsicSize => {
                f.write_str("pending image intrinsic size snapshot을 만들 수 없습니다")
            }
            Self::DuplicatePendingNode(node) => {
                write!(f, "pending image node가 중복되었습니다: node {node}")
            }
            Self::RevisionExhausted => f.write_str("image resource revision 번호를 소진했습니다"),
        }
    }
}

impl std::error::Error for ImageSnapshotError {}

#[cfg(test)]
mod snapshot_tests;

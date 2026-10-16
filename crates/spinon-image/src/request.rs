use std::{fmt, sync::Arc};

use crate::decode::{DecodedImage, ImageDecodeError, PngDecoder};
use spinon_core::{AttributeName, DomString, HostDocumentSnapshot, HostNodeHandle, HostNodeKind};
use spinon_layout::LayoutSourceRevision;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const MAX_SRC_CODE_UNITS: usize = 1_048_576;

/// 특정 HostDocument snapshot에서 읽은 이미지 요청 identity입니다.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageRequest {
    source: LayoutSourceRevision,
    element: HostNodeHandle,
    src: DomString,
}

impl ImageRequest {
    /// 연결된 HTML `img` 요소의 source와 revision을 캡처합니다.
    pub fn from_document(
        document: &HostDocumentSnapshot,
        element: HostNodeHandle,
    ) -> Result<Self, ImageRequestError> {
        let src = document_src(document, element)?;

        Ok(Self {
            source: LayoutSourceRevision::HostDocument {
                generation: document.generation(),
                document: document.document_revision(),
                render_tree: document.render_tree_revision(),
            },
            element,
            src: src.clone(),
        })
    }

    pub(crate) fn matches_current_src(
        &self,
        document: &HostDocumentSnapshot,
    ) -> Result<bool, ImageRequestError> {
        Ok(document_src(document, self.element)? == &self.src)
    }

    /// 이 요청에 전달된 PNG bytes를 디코드한 완료 결과를 만듭니다.
    ///
    /// `encoded`는 호출자가 이 요청의 src에 대해 읽은 bytes여야 합니다.
    pub fn decode(self, encoded: &[u8]) -> Result<ImageResourceCompletion, ImageDecodeError> {
        let image = PngDecoder.decode(encoded)?;
        Ok(ImageResourceCompletion::from_parts(self, Arc::new(image)))
    }

    /// 이 요청의 PNG decode 성공 또는 실패를 요청 identity와 함께 보존합니다.
    pub fn decode_outcome(self, encoded: &[u8]) -> ImageResourceOutcome {
        let result = PngDecoder.decode(encoded).map(Arc::new).map_err(Arc::new);
        ImageResourceOutcome {
            request: self,
            result,
        }
    }

    pub const fn element(&self) -> HostNodeHandle {
        self.element
    }

    pub const fn source(&self) -> LayoutSourceRevision {
        self.source
    }

    pub fn src(&self) -> &DomString {
        &self.src
    }
}

fn document_src(
    document: &HostDocumentSnapshot,
    element: HostNodeHandle,
) -> Result<&DomString, ImageRequestError> {
    let node = document
        .node(element)
        .ok_or(ImageRequestError::UnknownElement)?;
    if !document.is_connected(element) {
        return Err(ImageRequestError::DetachedElement);
    }
    let HostNodeKind::Element(element_data) = node.kind() else {
        return Err(ImageRequestError::NotHtmlImage);
    };
    if element_data.namespace() != HTML_NAMESPACE || element_data.local_name() != "img" {
        return Err(ImageRequestError::NotHtmlImage);
    }
    let src_name = AttributeName::new(None, "src").ok_or(ImageRequestError::MissingSource)?;
    let src = element_data
        .attribute(&src_name)
        .ok_or(ImageRequestError::MissingSource)?;
    if src.code_units().is_empty() {
        return Err(ImageRequestError::EmptySource);
    }
    if src.code_units().len() > MAX_SRC_CODE_UNITS {
        return Err(ImageRequestError::SourceTooLong {
            actual: src.code_units().len(),
            maximum: MAX_SRC_CODE_UNITS,
        });
    }
    Ok(src)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// HTML 이미지 요청을 만들 수 없는 이유입니다.
pub enum ImageRequestError {
    UnknownElement,
    DetachedElement,
    NotHtmlImage,
    MissingSource,
    EmptySource,
    SourceTooLong { actual: usize, maximum: usize },
}

impl fmt::Display for ImageRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownElement => f.write_str("이미지 요청의 요소가 현재 문서에 없습니다"),
            Self::DetachedElement => f.write_str("분리된 이미지 요소는 요청할 수 없습니다"),
            Self::NotHtmlImage => f.write_str("요청 대상은 HTML namespace의 img여야 합니다"),
            Self::MissingSource => f.write_str("이미지 요소에 unqualified src 속성이 없습니다"),
            Self::EmptySource => f.write_str("빈 src 값은 이미지 요청에 사용할 수 없습니다"),
            Self::SourceTooLong { actual, maximum } => write!(
                f,
                "이미지 src 길이가 {maximum} UTF-16 code unit 제한을 넘었습니다: {actual}"
            ),
        }
    }
}

impl std::error::Error for ImageRequestError {}

/// 한 이미지 요청과 그 요청에 전달한 bytes의 decode 결과를 함께 소유합니다.
#[derive(Debug)]
pub struct ImageResourceCompletion {
    pub(crate) request: ImageRequest,
    pub(crate) image: Arc<DecodedImage>,
}

/// 한 요청에 귀속된 PNG decode 성공 또는 실패입니다.
#[derive(Debug)]
pub struct ImageResourceOutcome {
    request: ImageRequest,
    result: Result<Arc<DecodedImage>, Arc<ImageDecodeError>>,
}

impl ImageResourceOutcome {
    pub fn request(&self) -> &ImageRequest {
        &self.request
    }

    pub fn image(&self) -> Option<&DecodedImage> {
        self.result.as_ref().ok().map(Arc::as_ref)
    }

    pub fn error(&self) -> Option<&ImageDecodeError> {
        self.result.as_ref().err().map(Arc::as_ref)
    }

    pub fn is_success(&self) -> bool {
        self.result.is_ok()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ImageRequest,
        Result<Arc<DecodedImage>, Arc<ImageDecodeError>>,
    ) {
        (self.request, self.result)
    }
}

impl ImageResourceCompletion {
    pub(crate) fn from_parts(request: ImageRequest, image: Arc<DecodedImage>) -> Self {
        Self { request, image }
    }

    pub fn request(&self) -> &ImageRequest {
        &self.request
    }

    pub fn image(&self) -> &DecodedImage {
        &self.image
    }

    pub(crate) fn into_outcome(self) -> ImageResourceOutcome {
        ImageResourceOutcome {
            request: self.request,
            result: Ok(self.image),
        }
    }
}

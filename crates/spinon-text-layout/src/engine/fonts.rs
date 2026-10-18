use std::sync::Arc;

use linebender_resource_handle::Blob;
use parley::{
    FontContext,
    fontique::{Collection, CollectionOptions, SourceCache},
};

use crate::{FontRegistryRevision, TextLayoutError, snapshot::FontRegistryToken};

use super::TextLayoutEngine;

impl TextLayoutEngine {
    /// 시스템 폰트 검색을 끈 font registry로 시작합니다.
    pub fn new_without_system_fonts() -> Self {
        Self::new_with_system_font_discovery(false)
    }

    /// Android·Apple runtime에서는 OS 글꼴을, 그 밖의 host에서는 고정 registry를 사용합니다.
    pub fn new_for_runtime() -> Self {
        #[cfg(any(target_os = "android", target_vendor = "apple"))]
        {
            Self::new_with_system_font_discovery(true)
        }
        #[cfg(not(any(target_os = "android", target_vendor = "apple")))]
        {
            Self::new_without_system_fonts()
        }
    }

    fn new_with_system_font_discovery(system_fonts: bool) -> Self {
        let collection = Collection::new(CollectionOptions {
            shared: false,
            system_fonts,
        });
        Self {
            font_context: FontContext {
                collection,
                source_cache: SourceCache::default(),
            },
            layout_context: parley::LayoutContext::new(),
            font_registry_revision: FontRegistryRevision::default(),
            font_registry_token: FontRegistryToken::new(),
        }
    }

    /// 명시적으로 제공한 글꼴 데이터만 registry에 등록합니다.
    pub fn register_font(
        &mut self,
        expected_family: &str,
        bytes: impl AsRef<[u8]>,
    ) -> Result<FontRegistryRevision, TextLayoutError> {
        let bytes = bytes.as_ref();
        let next_revision = FontRegistryRevision(
            self.font_registry_revision
                .0
                .checked_add(1)
                .ok_or(TextLayoutError::FontRevisionOverflow)?,
        );
        // 실패한 font 등록이 live registry에 남지 않도록 복제본에서 검증합니다.
        let font_data = Arc::new(bytes.to_vec());
        let mut candidate = self.font_context.collection.clone();
        let registered = candidate.register_fonts(Blob::new(font_data.clone()), None);
        let expected_family_id = candidate.family_id(expected_family);
        if !registered
            .iter()
            .any(|(family_id, _)| Some(*family_id) == expected_family_id)
        {
            return Err(TextLayoutError::InvalidFontData(expected_family.to_owned()));
        }
        self.font_context.collection = candidate;
        self.font_registry_token = FontRegistryToken::new();
        self.font_registry_revision = next_revision;
        Ok(next_revision)
    }

    pub const fn font_registry_revision(&self) -> FontRegistryRevision {
        self.font_registry_revision
    }
}

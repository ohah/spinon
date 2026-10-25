use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use style::{
    context::QuirksMode,
    device::{Device, servo::FontMetricsProvider},
    font_metrics::FontMetrics,
    media_queries::MediaType,
    properties::{ComputedValues, style_structs::Font},
    queries::values::PrefersColorScheme,
    servo::media_features::PointerCapabilities,
    thread_state::{self, ThreadState},
    values::computed::{CSSPixelLength, Length, font::QueryFontMetricsFlags},
};

use super::{CssColorScheme, CssPrimaryPointer, CssViewport};

static STYLO_PREFERENCE_LOCK: RwLock<()> = RwLock::new(());

pub(crate) enum StyloPreferenceGuard {
    Standard {
        _guard: RwLockReadGuard<'static, ()>,
    },
    OrthogonalWritingMode {
        _guard: OrthogonalWritingModePreferenceGuard,
    },
    ScrollGesture {
        _guard: ScrollGesturePreferenceGuard,
    },
}

pub(crate) struct ScrollGesturePreferenceGuard {
    _guard: RwLockWriteGuard<'static, ()>,
}

impl ScrollGesturePreferenceGuard {
    fn enter() -> Self {
        let guard = STYLO_PREFERENCE_LOCK
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        stylo_static_prefs::set_pref!("layout.unimplemented", true);
        Self { _guard: guard }
    }
}

impl Drop for ScrollGesturePreferenceGuard {
    fn drop(&mut self) {
        stylo_static_prefs::set_pref!("layout.unimplemented", false);
    }
}

pub(crate) struct OrthogonalWritingModePreferenceGuard {
    _guard: RwLockWriteGuard<'static, ()>,
    previous_enabled: bool,
}

impl OrthogonalWritingModePreferenceGuard {
    fn enter() -> Self {
        let guard = STYLO_PREFERENCE_LOCK
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let previous_enabled = stylo_static_prefs::pref!("layout.writing-mode.enabled");
        stylo_static_prefs::set_pref!("layout.writing-mode.enabled", true);
        Self {
            _guard: guard,
            previous_enabled,
        }
    }
}

impl Drop for OrthogonalWritingModePreferenceGuard {
    fn drop(&mut self) {
        stylo_static_prefs::set_pref!("layout.writing-mode.enabled", self.previous_enabled);
    }
}

pub(super) fn acquire_stylo_preferences(
    profile: super::ComputedStyleProfile,
) -> StyloPreferenceGuard {
    match profile {
        super::ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1 => {
            StyloPreferenceGuard::ScrollGesture {
                _guard: ScrollGesturePreferenceGuard::enter(),
            }
        }
        super::ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 => {
            StyloPreferenceGuard::OrthogonalWritingMode {
                _guard: OrthogonalWritingModePreferenceGuard::enter(),
            }
        }
        _ => acquire_standard_stylo_preferences(),
    }
}

pub(crate) fn acquire_standard_stylo_preferences() -> StyloPreferenceGuard {
    StyloPreferenceGuard::Standard {
        _guard: STYLO_PREFERENCE_LOCK
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner),
    }
}

pub(super) fn enable_scroll_gesture_preferences() -> ScrollGesturePreferenceGuard {
    ScrollGesturePreferenceGuard::enter()
}

pub(super) fn enable_orthogonal_writing_mode_preferences() -> OrthogonalWritingModePreferenceGuard {
    OrthogonalWritingModePreferenceGuard::enter()
}

/// Stylo preference는 프로세스 전역이므로 파서·cascade 진입마다 사용값을 다시 고정합니다.
pub(crate) fn initialize_stylo_preferences() {
    stylo_static_prefs::set_pref!("layout.grid.enabled", true);
}

pub(super) fn make_device(quirks_mode: QuirksMode, viewport: CssViewport) -> Device {
    let font_metrics = FixedFontMetricsProvider;
    let viewport_size = euclid::Size2D::new(viewport.width_css_px, viewport.height_css_px);
    let device_size = euclid::Size2D::new(
        viewport.width_css_px * viewport.device_scale_factor,
        viewport.height_css_px * viewport.device_scale_factor,
    );
    let media_environment = viewport.media_environment;
    let color_scheme = match media_environment.color_scheme {
        CssColorScheme::Light => PrefersColorScheme::Light,
        CssColorScheme::Dark => PrefersColorScheme::Dark,
    };
    let primary_pointer = match media_environment.primary_pointer {
        CssPrimaryPointer::None => PointerCapabilities::empty(),
        CssPrimaryPointer::Coarse => PointerCapabilities::COARSE,
        CssPrimaryPointer::Fine => PointerCapabilities::FINE,
    } | if media_environment.primary_hover {
        PointerCapabilities::HOVER
    } else {
        PointerCapabilities::empty()
    };
    let all_pointers = {
        let capabilities = media_environment.all_pointers;
        (if capabilities.coarse {
            PointerCapabilities::COARSE
        } else {
            PointerCapabilities::empty()
        }) | (if capabilities.fine {
            PointerCapabilities::FINE
        } else {
            PointerCapabilities::empty()
        }) | (if capabilities.hover {
            PointerCapabilities::HOVER
        } else {
            PointerCapabilities::empty()
        })
    };
    Device::new(
        MediaType::screen(),
        quirks_mode,
        viewport_size,
        device_size,
        euclid::Scale::new(viewport.device_scale_factor),
        Box::new(font_metrics),
        ComputedValues::initial_values_with_font_override(Font::initial_values()),
        color_scheme,
        primary_pointer,
        all_pointers,
    )
}

#[derive(Debug)]
struct FixedFontMetricsProvider;

impl FontMetricsProvider for FixedFontMetricsProvider {
    fn query_font_metrics(
        &self,
        _vertical: bool,
        _font: &Font,
        _base_size: CSSPixelLength,
        _flags: QueryFontMetricsFlags,
    ) -> FontMetrics {
        FontMetrics::default()
    }

    fn base_size_for_generic(
        &self,
        _generic: style::values::computed::font::GenericFontFamily,
    ) -> Length {
        Length::new(16.0)
    }
}

pub(super) struct LayoutThreadState {
    entered: bool,
}

impl LayoutThreadState {
    pub(super) fn enter() -> Self {
        let entered = !thread_state::get().contains(ThreadState::LAYOUT);
        if entered {
            thread_state::enter(ThreadState::LAYOUT);
        }
        Self { entered }
    }
}

impl Drop for LayoutThreadState {
    fn drop(&mut self) {
        if self.entered {
            thread_state::exit(ThreadState::LAYOUT);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orthogonal_writing_mode_guard_restores_its_pref_without_enabling_unimplemented() {
        {
            let _preferences = OrthogonalWritingModePreferenceGuard::enter();
            assert!(stylo_static_prefs::pref!("layout.writing-mode.enabled"));
            assert!(!stylo_static_prefs::pref!("layout.unimplemented"));
        }

        let _preferences = acquire_standard_stylo_preferences();
        assert!(!stylo_static_prefs::pref!("layout.writing-mode.enabled"));
        assert!(!stylo_static_prefs::pref!("layout.unimplemented"));
    }
}

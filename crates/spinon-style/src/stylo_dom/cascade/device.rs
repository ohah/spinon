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

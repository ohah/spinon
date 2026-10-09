use spinon_core::EnvironmentRevision;
use spinon_style::{CssMediaEnvironment, CssViewport};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct RuntimeCssEnvironment {
    pub(super) width_css_px: f32,
    pub(super) height_css_px: f32,
    pub(super) device_scale_factor: f32,
    pub(super) media_environment: CssMediaEnvironment,
    pub(super) revision: EnvironmentRevision,
}

impl RuntimeCssEnvironment {
    pub(super) fn same_input(self, other: Self) -> bool {
        self.width_css_px == other.width_css_px
            && self.height_css_px == other.height_css_px
            && self.device_scale_factor == other.device_scale_factor
            && self.media_environment == other.media_environment
    }

    pub(super) fn viewport(self) -> CssViewport {
        CssViewport {
            width_css_px: self.width_css_px,
            height_css_px: self.height_css_px,
            device_scale_factor: self.device_scale_factor,
            environment_revision: self.revision,
            media_environment: self.media_environment,
        }
    }
}

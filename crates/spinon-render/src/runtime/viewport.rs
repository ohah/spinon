use super::RuntimeRenderError;

/// Root HTML의 초기 CSS px → physical surface px 배율 입력입니다.
///
/// `device_scale_factor`와 `page_scale`은 분리 보존하며 WGPU 투영 때만 곱합니다.
/// Layout viewport 및 hit-test CSS 좌표를 변경하지 않습니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimePageScaleTransform {
    device_scale_factor: f32,
    page_scale: f32,
    physical_scale: f32,
}

impl RuntimePageScaleTransform {
    pub fn new(device_scale_factor: f32, page_scale: f32) -> Result<Self, RuntimeRenderError> {
        let physical_scale = device_scale_factor * page_scale;
        if !device_scale_factor.is_finite()
            || device_scale_factor <= 0.0
            || !page_scale.is_finite()
            || page_scale <= 0.0
            || !physical_scale.is_finite()
            || physical_scale <= 0.0
        {
            return Err(RuntimeRenderError::InvalidPageScaleTransform);
        }
        Ok(Self {
            device_scale_factor,
            page_scale,
            physical_scale,
        })
    }

    pub const fn device_scale_factor(self) -> f32 {
        self.device_scale_factor
    }

    pub const fn page_scale(self) -> f32 {
        self.page_scale
    }

    /// 물리 표면에서 CSS px 하나가 차지하는 pixel 수입니다.
    pub const fn physical_scale(self) -> f32 {
        self.physical_scale
    }
}

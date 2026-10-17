use crate::OpaqueCssSrgb;

use super::RuntimeRenderError;

/// encoded sRGB 색과 CSS alpha를 보존하는 내부 paint 색입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeSrgbColor {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl RuntimeSrgbColor {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Result<Self, RuntimeRenderError> {
        if [red, green, blue, alpha]
            .iter()
            .any(|component| !component.is_finite() || !(0.0..=1.0).contains(component))
        {
            return Err(RuntimeRenderError::InvalidPaintColor);
        }
        Ok(Self {
            red,
            green,
            blue,
            alpha,
        })
    }

    pub const fn red(self) -> f32 {
        self.red
    }

    pub const fn green(self) -> f32 {
        self.green
    }

    pub const fn blue(self) -> f32 {
        self.blue
    }

    pub const fn alpha(self) -> f32 {
        self.alpha
    }
}

/// 투명, 불투명 sRGB 또는 alpha가 있는 sRGB 단색 배경 paint입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RuntimePaint {
    None,
    Opaque(OpaqueCssSrgb),
    Color(RuntimeSrgbColor),
}

/// 단색으로 그릴 한 물리 방향 border 변입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeBorderSide {
    width_css_px: f32,
    paint: RuntimePaint,
}

impl RuntimeBorderSide {
    pub fn new(width_css_px: f32, paint: RuntimePaint) -> Result<Self, RuntimeRenderError> {
        if !width_css_px.is_finite() || width_css_px < 0.0 {
            return Err(RuntimeRenderError::InvalidBorderWidth);
        }
        Ok(Self {
            width_css_px,
            paint,
        })
    }

    pub const fn none() -> Self {
        Self {
            width_css_px: 0.0,
            paint: RuntimePaint::None,
        }
    }

    pub const fn width_css_px(self) -> f32 {
        self.width_css_px
    }

    pub const fn paint(self) -> RuntimePaint {
        self.paint
    }
}

/// 네 면의 단색 CSS border geometry와 paint입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeBorder {
    top: RuntimeBorderSide,
    right: RuntimeBorderSide,
    bottom: RuntimeBorderSide,
    left: RuntimeBorderSide,
}

impl RuntimeBorder {
    pub const fn none() -> Self {
        Self {
            top: RuntimeBorderSide::none(),
            right: RuntimeBorderSide::none(),
            bottom: RuntimeBorderSide::none(),
            left: RuntimeBorderSide::none(),
        }
    }

    pub const fn new(
        top: RuntimeBorderSide,
        right: RuntimeBorderSide,
        bottom: RuntimeBorderSide,
        left: RuntimeBorderSide,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub const fn top(self) -> RuntimeBorderSide {
        self.top
    }

    pub const fn right(self) -> RuntimeBorderSide {
        self.right
    }

    pub const fn bottom(self) -> RuntimeBorderSide {
        self.bottom
    }

    pub const fn left(self) -> RuntimeBorderSide {
        self.left
    }
}

use style::properties::ComputedValues;

use crate::stylo_dom::cascade::ComputedLayoutBorder;

pub(crate) fn computed_layout_border(computed: &ComputedValues) -> ComputedLayoutBorder {
    ComputedLayoutBorder {
        top: used_width(
            computed.clone_border_top_width().0.to_f32_px(),
            computed.clone_border_top_style().none_or_hidden(),
        ),
        right: used_width(
            computed.clone_border_right_width().0.to_f32_px(),
            computed.clone_border_right_style().none_or_hidden(),
        ),
        bottom: used_width(
            computed.clone_border_bottom_width().0.to_f32_px(),
            computed.clone_border_bottom_style().none_or_hidden(),
        ),
        left: used_width(
            computed.clone_border_left_width().0.to_f32_px(),
            computed.clone_border_left_style().none_or_hidden(),
        ),
    }
}

fn used_width(computed_width_css_px: f32, style_is_none_or_hidden: bool) -> f32 {
    if !computed_width_css_px.is_finite() || computed_width_css_px < 0.0 {
        // 손상된 typed value를 style gate나 floor가 0/1px으로 숨기지 않게 둡니다.
        // Layout DTO 검증이 node·side 오류로 전체 계산을 거부합니다.
        computed_width_css_px
    } else if style_is_none_or_hidden || computed_width_css_px == 0.0 {
        0.0
    } else {
        // Chromium 기준은 0보다 큰 폭을 CSS px 정수로 내림하고 최소 1px을 둡니다.
        // Stylo는 DPR의 device pixel 단위로 snap하므로 layout 경계에서 기준 동작을 맞춥니다.
        computed_width_css_px.floor().max(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::used_width;

    #[test]
    fn none_and_hidden_suppress_only_their_used_border_width() {
        assert_eq!(used_width(6.0, true), 0.0);
        assert_eq!(used_width(0.0, false), 0.0);
        assert_eq!(used_width(0.25, false), 1.0);
        assert_eq!(used_width(1.75, false), 1.0);
        assert_eq!(used_width(2.5, false), 2.0);
        assert_eq!(used_width(3.5, false), 3.0);
        assert_eq!(used_width(-1.0, false), -1.0);
        assert_eq!(used_width(-1.0, true), -1.0);
        assert!(used_width(f32::NAN, false).is_nan());
        assert_eq!(used_width(f32::INFINITY, false), f32::INFINITY);
    }
}

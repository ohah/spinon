/// Alpha를 합성한 뒤의 불투명 encoded sRGB 색입니다.
///
/// 각 채널은 선형 RGB가 아니라 `[0, 1]` 범위의 sRGB encoded component입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FinalOpaqueSrgb {
    red: f64,
    green: f64,
    blue: f64,
}

/// 최종 sRGB 색에 유효하지 않은 채널이 있을 때 반환합니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FinalOpaqueSrgbError {
    /// 0=red, 1=green, 2=blue 채널이 유한하지 않습니다.
    NonFiniteChannel { channel: usize },
    /// 0=red, 1=green, 2=blue 채널이 `[0, 1]` 밖입니다.
    ChannelOutOfRange { channel: usize },
}

impl FinalOpaqueSrgb {
    /// alpha 합성이 끝난 encoded sRGB 색을 검증해 만듭니다.
    pub fn new(red: f64, green: f64, blue: f64) -> Result<Self, FinalOpaqueSrgbError> {
        let channels = [red, green, blue];
        for (channel, value) in channels.iter().copied().enumerate() {
            if !value.is_finite() {
                return Err(FinalOpaqueSrgbError::NonFiniteChannel { channel });
            }
            if !(0.0..=1.0).contains(&value) {
                return Err(FinalOpaqueSrgbError::ChannelOutOfRange { channel });
            }
        }
        Ok(Self { red, green, blue })
    }

    fn relative_luminance(self) -> f64 {
        0.2126 * linearize_srgb(self.red)
            + 0.7152 * linearize_srgb(self.green)
            + 0.0722 * linearize_srgb(self.blue)
    }
}

/// alpha 합성이 끝난 두 불투명 sRGB 색의 WCAG 2.2 대비 비율을 계산합니다.
///
/// 이 함수는 CSS alpha compositing을 수행하지 않습니다. 전경과 배경 모두
/// 실제 사용 배경에 합성된 불투명 sRGB 값으로 전달해야 합니다.
pub fn wcag_22_contrast_ratio(foreground: FinalOpaqueSrgb, background: FinalOpaqueSrgb) -> f64 {
    let foreground_luminance = foreground.relative_luminance();
    let background_luminance = background.relative_luminance();
    let lighter = foreground_luminance.max(background_luminance);
    let darker = foreground_luminance.min(background_luminance);
    (lighter + 0.05) / (darker + 0.05)
}

fn linearize_srgb(channel: f64) -> f64 {
    if channel <= 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use super::{FinalOpaqueSrgb, FinalOpaqueSrgbError, wcag_22_contrast_ratio};

    fn gray_8bit(value: u8) -> FinalOpaqueSrgb {
        let channel = f64::from(value) / 255.0;
        FinalOpaqueSrgb::new(channel, channel, channel).unwrap()
    }

    #[test]
    fn black_and_white_have_the_maximum_wcag_ratio() {
        assert_eq!(wcag_22_contrast_ratio(gray_8bit(0), gray_8bit(255)), 21.0);
    }

    #[test]
    fn neighboring_srgb_grays_straddle_the_normal_text_threshold() {
        let white = gray_8bit(255);
        let darker = wcag_22_contrast_ratio(gray_8bit(118), white);
        let lighter = wcag_22_contrast_ratio(gray_8bit(119), white);
        assert!((darker - 4.542_224_959_605_253).abs() < 1e-12);
        assert!((lighter - 4.478_089_453_577_214).abs() < 1e-12);
        assert!(darker > 4.5);
        assert!(lighter < 4.5);
    }

    #[test]
    fn primary_channel_luminance_weights_match_wcag_srgb_values() {
        let white = gray_8bit(255);
        let red = FinalOpaqueSrgb::new(1.0, 0.0, 0.0).unwrap();
        let green = FinalOpaqueSrgb::new(0.0, 1.0, 0.0).unwrap();
        let blue = FinalOpaqueSrgb::new(0.0, 0.0, 1.0).unwrap();

        assert!((wcag_22_contrast_ratio(red, white) - 3.998_476_770_753_998_5).abs() < 1e-12);
        assert!((wcag_22_contrast_ratio(green, white) - 1.372_190_277_051_751_3).abs() < 1e-12);
        assert!((wcag_22_contrast_ratio(blue, white) - 8.592_471_358_428_805).abs() < 1e-12);
    }

    #[test]
    fn srgb_linearization_covers_both_sides_of_the_piecewise_threshold() {
        let below = FinalOpaqueSrgb::new(0.040_45, 0.040_45, 0.040_45).unwrap();
        let above = FinalOpaqueSrgb::new(0.040_451, 0.040_451, 0.040_451).unwrap();
        let expected_below = 0.040_45 / 12.92;
        let expected_above = ((0.040_451 + 0.055) / 1.055_f64).powf(2.4);

        assert!((below.relative_luminance() - expected_below).abs() < 1e-15);
        assert!((above.relative_luminance() - expected_above).abs() < 1e-15);
    }

    #[test]
    fn fractional_srgb_inputs_cover_the_exact_boundary_and_both_sides() {
        let boundary_luminance: f64 = 1.05 / 4.5 - 0.05;
        let boundary_channel = 1.055 * boundary_luminance.powf(1.0 / 2.4) - 0.055;
        let gray = |channel| FinalOpaqueSrgb::new(channel, channel, channel).unwrap();
        let white = gray(1.0);
        let exact = wcag_22_contrast_ratio(gray(boundary_channel), white);
        let above = wcag_22_contrast_ratio(gray(boundary_channel - 1e-7), white);
        let below = wcag_22_contrast_ratio(gray(boundary_channel + 1e-7), white);

        assert!((exact - 4.5).abs() < 1e-12);
        assert!(above > 4.5);
        assert!(below < 4.5);
    }

    #[test]
    fn contrast_ratio_is_symmetric_and_identical_colors_are_one_to_one() {
        let first = FinalOpaqueSrgb::new(0.1, 0.4, 0.8).unwrap();
        let second = FinalOpaqueSrgb::new(0.9, 0.2, 0.05).unwrap();
        assert_eq!(
            wcag_22_contrast_ratio(first, second),
            wcag_22_contrast_ratio(second, first)
        );
        assert_eq!(wcag_22_contrast_ratio(first, first), 1.0);
    }

    #[test]
    fn final_srgb_rejects_non_finite_and_out_of_range_channels() {
        assert_eq!(
            FinalOpaqueSrgb::new(f64::NAN, 0.0, 0.0),
            Err(FinalOpaqueSrgbError::NonFiniteChannel { channel: 0 })
        );
        assert_eq!(
            FinalOpaqueSrgb::new(0.0, f64::INFINITY, 0.0),
            Err(FinalOpaqueSrgbError::NonFiniteChannel { channel: 1 })
        );
        assert_eq!(
            FinalOpaqueSrgb::new(0.0, -0.001, 0.0),
            Err(FinalOpaqueSrgbError::ChannelOutOfRange { channel: 1 })
        );
        assert_eq!(
            FinalOpaqueSrgb::new(0.0, 0.0, 1.001),
            Err(FinalOpaqueSrgbError::ChannelOutOfRange { channel: 2 })
        );
    }
}

use style::color::{AbsoluteColor, ColorFlags, ColorSpace};

/// Stylo가 계산한 CSS 색상의 원래 색 공간입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComputedCssColorSpace {
    Srgb,
    Hsl,
    Hwb,
    Lab,
    Lch,
    Oklab,
    Oklch,
    SrgbLinear,
    DisplayP3,
    DisplayP3Linear,
    A98Rgb,
    ProphotoRgb,
    Rec2020,
    XyzD50,
    XyzD65,
}

/// `<color>` computed value와 고정 sRGB 출력용 변환 결과입니다.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputedCssColor {
    pub color_space: ComputedCssColorSpace,
    pub components: [f32; 3],
    pub alpha: f32,
    pub missing_components: [bool; 3],
    pub missing_alpha: bool,
    pub legacy_syntax: bool,
    /// gamut mapping 전 encoded sRGB 변환값입니다. 채널은 0..1 범위 밖일 수 있습니다.
    pub converted_srgb: [f32; 4],
    /// CSS Color 4 gamut mapping을 적용한 encoded sRGB 채널과 alpha입니다.
    pub gamut_mapped_srgb: [f32; 4],
}

impl ComputedCssColor {
    pub(crate) fn from_absolute_color(color: AbsoluteColor) -> Result<Self, &'static str> {
        let [red, green, blue, alpha] = *color.raw_components();
        if [red, green, blue, alpha]
            .iter()
            .any(|component| !component.is_finite())
        {
            return Err("Stylo 계산 색상에 유한하지 않은 component가 있습니다");
        }

        // 이미 sRGB 범위 안인 색은 gamut mapping이 필요하지 않습니다. OkLCh를
        // 거쳐 돌아오면 sRGB 채널이 8-bit 반올림 경계를 넘을 수 있습니다
        // (예: 무채색 HSL 50%가 128이 아니라 127이 됩니다).
        let direct_srgb = color.to_color_space(ColorSpace::Srgb);
        let direct_components = *direct_srgb.raw_components();
        if direct_components
            .iter()
            .any(|component| !component.is_finite())
        {
            return Err("sRGB 변환 결과에 유한하지 않은 component가 있습니다");
        }
        let direct_is_in_srgb_gamut = direct_components[..3]
            .iter()
            .all(|component| (0.0..=1.0).contains(component));
        let mapped = if direct_is_in_srgb_gamut {
            direct_srgb.into_srgb_legacy()
        } else {
            color
                .gamut_map_binary_search(ColorSpace::Srgb)
                .into_srgb_legacy()
        };
        let gamut_mapped_srgb = *mapped.raw_components();
        if gamut_mapped_srgb[..3]
            .iter()
            .any(|component| !(0.0..=1.0).contains(component))
        {
            return Err("CSS Color 4 gamut mapping 결과가 sRGB 범위 밖입니다");
        }
        if !(0.0..=1.0).contains(&gamut_mapped_srgb[3]) {
            return Err("gamut mapping 뒤 alpha가 0..1 범위 밖입니다");
        }

        Ok(Self {
            color_space: color_space(color.color_space),
            components: [red, green, blue],
            alpha,
            missing_components: [
                color.c0().is_none(),
                color.c1().is_none(),
                color.c2().is_none(),
            ],
            missing_alpha: color.flags.contains(ColorFlags::ALPHA_IS_NONE),
            legacy_syntax: color.is_legacy_syntax(),
            converted_srgb: direct_components,
            gamut_mapped_srgb,
        })
    }

    /// Encoded sRGB를 unorm8로 양자화할 때 채널을 범위에 맞춥니다.
    /// 이것은 gamut mapping 결과가 아니라 sRGB surface 변환 경로입니다.
    pub fn converted_srgb_clipped_rgba8(self) -> Result<[u8; 4], &'static str> {
        Ok([
            to_srgb_byte_clipped(self.converted_srgb[0])?,
            to_srgb_byte_clipped(self.converted_srgb[1])?,
            to_srgb_byte_clipped(self.converted_srgb[2])?,
            to_srgb_byte_clipped(self.converted_srgb[3])?,
        ])
    }

    pub fn gamut_mapped_srgb_rgba8(self) -> Result<[u8; 4], &'static str> {
        Ok([
            to_srgb_byte(self.gamut_mapped_srgb[0])?,
            to_srgb_byte(self.gamut_mapped_srgb[1])?,
            to_srgb_byte(self.gamut_mapped_srgb[2])?,
            to_srgb_byte(self.gamut_mapped_srgb[3])?,
        ])
    }
}

fn color_space(color_space: ColorSpace) -> ComputedCssColorSpace {
    match color_space {
        ColorSpace::Srgb => ComputedCssColorSpace::Srgb,
        ColorSpace::Hsl => ComputedCssColorSpace::Hsl,
        ColorSpace::Hwb => ComputedCssColorSpace::Hwb,
        ColorSpace::Lab => ComputedCssColorSpace::Lab,
        ColorSpace::Lch => ComputedCssColorSpace::Lch,
        ColorSpace::Oklab => ComputedCssColorSpace::Oklab,
        ColorSpace::Oklch => ComputedCssColorSpace::Oklch,
        ColorSpace::SrgbLinear => ComputedCssColorSpace::SrgbLinear,
        ColorSpace::DisplayP3 => ComputedCssColorSpace::DisplayP3,
        ColorSpace::DisplayP3Linear => ComputedCssColorSpace::DisplayP3Linear,
        ColorSpace::A98Rgb => ComputedCssColorSpace::A98Rgb,
        ColorSpace::ProphotoRgb => ComputedCssColorSpace::ProphotoRgb,
        ColorSpace::Rec2020 => ComputedCssColorSpace::Rec2020,
        ColorSpace::XyzD50 => ComputedCssColorSpace::XyzD50,
        ColorSpace::XyzD65 => ComputedCssColorSpace::XyzD65,
    }
}

fn to_srgb_byte(component: f32) -> Result<u8, &'static str> {
    if !component.is_finite() || !(0.0..=1.0).contains(&component) {
        return Err("gamut-mapped sRGB component가 유한한 0..1 값이 아닙니다");
    }
    Ok((component * 255.0).round() as u8)
}

fn to_srgb_byte_clipped(component: f32) -> Result<u8, &'static str> {
    if !component.is_finite() {
        return Err("변환된 sRGB component가 유한한 값이 아닙니다");
    }
    Ok((component.clamp(0.0, 1.0) * 255.0).round() as u8)
}

#[cfg(test)]
mod tests {
    use style::color::{AbsoluteColor, ColorSpace};

    use super::{ComputedCssColor, ComputedCssColorSpace};

    #[test]
    fn preserves_wide_gamut_source_and_both_srgb_output_paths() {
        let source = AbsoluteColor::new(ColorSpace::DisplayP3, 1.0, 0.4, 0.0, 1.0);
        let unbounded = source.to_color_space(ColorSpace::Srgb);
        assert!(
            unbounded.raw_components()[..3]
                .iter()
                .any(|component| !(0.0..=1.0).contains(component))
        );

        let computed = ComputedCssColor::from_absolute_color(source).unwrap();
        assert_eq!(computed.color_space, ComputedCssColorSpace::DisplayP3);
        assert_eq!(computed.converted_srgb, *unbounded.raw_components());
        assert_eq!(computed.gamut_mapped_srgb_rgba8(), Ok([255, 100, 0, 255]));
        assert_eq!(
            computed.converted_srgb_clipped_rgba8(),
            Ok([255, 87, 0, 255])
        );
    }

    #[test]
    fn preserves_missing_components_and_partial_alpha_in_source_value() {
        let source = AbsoluteColor::new(ColorSpace::Oklch, 0.6, None, 230.0, Some(0.5));
        let computed = ComputedCssColor::from_absolute_color(source).unwrap();

        assert_eq!(computed.color_space, ComputedCssColorSpace::Oklch);
        assert_eq!(computed.missing_components, [false, true, false]);
        assert!(!computed.missing_alpha);
        assert_eq!(computed.alpha, 0.5);
        assert_eq!(computed.gamut_mapped_srgb_rgba8().unwrap()[3], 128);
    }

    #[test]
    fn reports_missing_alpha_separately_from_zero_alpha() {
        let source = AbsoluteColor::new(ColorSpace::Srgb, 0.2, 0.4, 0.8, None);
        let computed = ComputedCssColor::from_absolute_color(source).unwrap();

        assert!(computed.missing_alpha);
        assert_eq!(computed.alpha, 0.0);
        assert_eq!(computed.gamut_mapped_srgb_rgba8().unwrap()[3], 0);
    }

    #[test]
    fn rejects_non_finite_source_components() {
        let source = AbsoluteColor::new(ColorSpace::Srgb, f32::NAN, 0.2, 0.8, 1.0);
        assert!(ComputedCssColor::from_absolute_color(source).is_err());
    }

    #[test]
    fn mapped_byte_path_rejects_non_finite_and_out_of_range_mutations() {
        let source = AbsoluteColor::new(ColorSpace::Srgb, 0.2, 0.4, 0.8, 1.0);
        let mut computed = ComputedCssColor::from_absolute_color(source).unwrap();

        computed.gamut_mapped_srgb[0] = f32::NAN;
        assert!(computed.gamut_mapped_srgb_rgba8().is_err());
        computed.gamut_mapped_srgb[0] = 1.01;
        assert!(computed.gamut_mapped_srgb_rgba8().is_err());
    }
}

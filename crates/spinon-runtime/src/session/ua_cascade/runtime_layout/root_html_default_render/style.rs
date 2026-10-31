use spinon_core::NodeId;
use spinon_layout::{
    FontStyleMode, InlineRunStyle, LayoutFrame, LigatureMode, LineBreakMode, ParagraphStyle,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_render::RuntimeSrgbColor;
use spinon_style::ComputedElementStyle;

use super::super::{RuntimeLayoutFailure, layout_failure};

pub(super) fn paragraph_style(
    style: &ComputedElementStyle,
) -> Result<ParagraphStyle, RuntimeLayoutFailure> {
    let (white_space, word_break, direction, line_break) = text_flow_style(style)?;
    let (line_height, line_height_is_normal) = line_height(style)?;
    Ok(ParagraphStyle {
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: property(style, "font-family")?.to_owned(),
        font_size: style.font_size_css_px,
        line_height,
        line_height_is_normal,
        font_weight: font_weight(style)?,
        font_style: font_style(style)?,
        white_space,
        word_break,
        direction,
        line_break,
        ligatures: LigatureMode::Normal,
        letter_spacing: 0.0,
    })
}

pub(super) fn inline_run_style(
    style: &ComputedElementStyle,
) -> Result<InlineRunStyle, RuntimeLayoutFailure> {
    let (line_height, line_height_is_normal) = line_height(style)?;
    Ok(InlineRunStyle {
        font_family: property(style, "font-family")?.to_owned(),
        font_size: style.font_size_css_px,
        line_height,
        line_height_is_normal,
        font_weight: font_weight(style)?,
        font_style: font_style(style)?,
    })
}

fn text_flow_style(
    style: &ComputedElementStyle,
) -> Result<
    (
        WhiteSpaceMode,
        WordBreakMode,
        TextParagraphDirection,
        LineBreakMode,
    ),
    RuntimeLayoutFailure,
> {
    let collapse = property(style, "white-space-collapse")?;
    let wrap = property(style, "text-wrap-mode")?;
    let white_space = white_space_mode(collapse, wrap)
        .ok_or_else(|| unsupported_style(style.node_id, "white-space", "unsupported"))?;
    let word_break = match property(style, "word-break")? {
        "normal" => WordBreakMode::Normal,
        "break-all" => WordBreakMode::BreakAll,
        value => return Err(unsupported_style(style.node_id, "word-break", value)),
    };
    let direction = match property(style, "direction")? {
        "ltr" => TextParagraphDirection::Ltr,
        "rtl" => TextParagraphDirection::Rtl,
        value => return Err(unsupported_style(style.node_id, "direction", value)),
    };
    let line_break = match property(style, "line-break")? {
        // CSS `auto` is a UA-selected policy; this runtime slice uses the fixed C15
        // Parley default because the supported engine does not expose an auto mode.
        "auto" => LineBreakMode::Strict,
        "normal" => LineBreakMode::Normal,
        "strict" => LineBreakMode::Strict,
        value => return Err(unsupported_style(style.node_id, "line-break", value)),
    };
    require_property(style, "text-align", &["start"])?;
    require_property(style, "overflow-wrap", &["normal"])?;
    require_property(style, "text-transform", &["none"])?;
    require_property(style, "writing-mode", &["horizontal-tb"])?;
    require_zero_length_property(style, "letter-spacing")?;
    require_zero_length_property(style, "word-spacing")?;
    require_zero_length_property(style, "text-indent")?;
    Ok((white_space, word_break, direction, line_break))
}

fn white_space_mode(collapse: &str, wrap: &str) -> Option<WhiteSpaceMode> {
    match (collapse, wrap) {
        ("collapse", "wrap") => Some(WhiteSpaceMode::Normal),
        ("collapse", "nowrap") => Some(WhiteSpaceMode::NoWrap),
        ("preserve", "nowrap") => Some(WhiteSpaceMode::Pre),
        ("preserve-breaks", "wrap") => Some(WhiteSpaceMode::PreLine),
        _ => None,
    }
}

fn font_weight(style: &ComputedElementStyle) -> Result<f32, RuntimeLayoutFailure> {
    let value = property(style, "font-weight")?;
    let weight = value
        .parse::<f32>()
        .ok()
        .filter(|weight| weight.is_finite() && (1.0..=1000.0).contains(weight))
        .ok_or_else(|| unsupported_style(style.node_id, "font-weight", value))?;
    Ok(weight)
}

fn font_style(style: &ComputedElementStyle) -> Result<FontStyleMode, RuntimeLayoutFailure> {
    let value = property(style, "font-style")?;
    match value {
        "normal" => Ok(FontStyleMode::Normal),
        "italic" => Ok(FontStyleMode::Italic),
        "oblique" => Ok(FontStyleMode::Oblique(14.0)),
        value if value.starts_with("oblique ") => {
            let angle = value
                .trim_start_matches("oblique ")
                .strip_suffix("deg")
                .and_then(|degrees| degrees.trim().parse::<f32>().ok())
                .filter(|angle| angle.is_finite() && (-90.0..=90.0).contains(angle))
                .ok_or_else(|| unsupported_style(style.node_id, "font-style", value))?;
            Ok(FontStyleMode::Oblique(angle))
        }
        value => Err(unsupported_style(style.node_id, "font-style", value)),
    }
}

fn line_height(style: &ComputedElementStyle) -> Result<(f32, bool), RuntimeLayoutFailure> {
    let value = property(style, "line-height")?;
    if value == "normal" {
        return Ok((0.0, true));
    }
    let line_height = parse_css_px(value)
        .filter(|line_height| line_height.is_finite() && *line_height > 0.0)
        .ok_or_else(|| unsupported_style(style.node_id, "line-height", value))?;
    Ok((line_height, false))
}

pub(super) fn runtime_text_color(
    style: &ComputedElementStyle,
) -> Result<RuntimeSrgbColor, RuntimeLayoutFailure> {
    let color = style.computed_color.ok_or_else(|| {
        layout_failure(
            "missing_root_html_default_render_color",
            Some(style.node_id.get()),
            Some("color".to_owned()),
        )
    })?;
    let channels = color.gamut_mapped_srgb.map(|value| {
        let byte = (value * 255.0).round();
        if value.is_finite()
            && (0.0..=1.0).contains(&value)
            && (value * 255.0 - byte).abs() <= 0.000_01
        {
            Some(byte / 255.0)
        } else {
            None
        }
    });
    let [Some(red), Some(green), Some(blue), Some(alpha)] = channels else {
        return Err(unsupported_style(
            style.node_id,
            "color",
            "not representable as 8-bit sRGB",
        ));
    };
    RuntimeSrgbColor::new(red, green, blue, alpha).map_err(|error| {
        layout_failure(
            "root_html_default_render_color_invalid",
            Some(style.node_id.get()),
            Some(error.to_string()),
        )
    })
}

pub(super) fn property<'a>(
    style: &'a ComputedElementStyle,
    name: &'static str,
) -> Result<&'a str, RuntimeLayoutFailure> {
    style
        .properties
        .get(name)
        .map(String::as_str)
        .ok_or_else(|| {
            layout_failure(
                "missing_root_html_default_render_property",
                Some(style.node_id.get()),
                Some(name.to_owned()),
            )
        })
}

fn require_property(
    style: &ComputedElementStyle,
    name: &'static str,
    allowed: &[&str],
) -> Result<(), RuntimeLayoutFailure> {
    let value = property(style, name)?;
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(unsupported_style(style.node_id, name, value))
    }
}

fn require_zero_length_property(
    style: &ComputedElementStyle,
    name: &'static str,
) -> Result<(), RuntimeLayoutFailure> {
    let value = property(style, name)?;
    if value == "normal" || parse_css_px(value).is_some_and(|value| value == 0.0) {
        Ok(())
    } else {
        Err(unsupported_style(style.node_id, name, value))
    }
}

fn parse_css_px(value: &str) -> Option<f32> {
    value
        .strip_suffix("px")
        .and_then(|value| value.trim().parse::<f32>().ok())
        .or_else(|| (value == "0").then_some(0.0))
}

pub(super) fn unsupported_style(
    node: NodeId,
    property: &'static str,
    value: &str,
) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_root_html_default_render_style",
        Some(node.get()),
        Some(format!("{property}: {value}")),
    )
}

pub(super) fn validate_frame(frame: LayoutFrame, node: NodeId) -> Result<(), RuntimeLayoutFailure> {
    if [frame.x, frame.y, frame.width, frame.height]
        .into_iter()
        .all(f32::is_finite)
        && frame.width >= 0.0
        && frame.height >= 0.0
    {
        Ok(())
    } else {
        Err(layout_failure(
            "root_html_default_render_frame_invalid",
            Some(node.get()),
            None,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::white_space_mode;
    use spinon_layout::WhiteSpaceMode;

    #[test]
    fn root_html_maps_pre_to_preserved_no_wrap_without_aliasing_neighbor_modes() {
        assert_eq!(
            white_space_mode("preserve", "nowrap"),
            Some(WhiteSpaceMode::Pre)
        );
        assert_eq!(white_space_mode("preserve", "wrap"), None);
        assert_eq!(
            white_space_mode("collapse", "nowrap"),
            Some(WhiteSpaceMode::NoWrap)
        );
    }
}

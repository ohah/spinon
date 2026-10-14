use spinon_core::NodeId;
use taffy::geometry::Size;

use crate::{
    LayoutBoxSizing, LayoutDimension, LayoutError, LayoutIntrinsicSize, LayoutLengthPercentage,
    LayoutStyle,
};

const DEFAULT_OBJECT_WIDTH: f32 = 300.0;
const DEFAULT_OBJECT_HEIGHT: f32 = 150.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct ReplacedSizingInput {
    pub(super) node: NodeId,
    pub(super) style: LayoutStyle,
    pub(super) natural: LayoutIntrinsicSize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct UsedReplacedSize {
    pub(super) content_width: f32,
    pub(super) content_height: f32,
    /// Taffy leaf size is the outer border box size.
    pub(super) width: f32,
    pub(super) height: f32,
}

#[derive(Clone, Copy)]
struct EffectiveRatio {
    value: f32,
    basis: RatioBasis,
}

#[derive(Clone, Copy)]
enum RatioBasis {
    ContentBox,
    BorderBox,
}

pub(super) fn resolve_used_size(
    node: NodeId,
    input: ReplacedSizingInput,
    known_dimensions: Size<Option<f32>>,
    inherent_size: bool,
) -> Result<UsedReplacedSize, LayoutError> {
    let style = input.style;
    let edges = edge_sums(node, style)?;
    let ratio = style
        .aspect_ratio
        .map(|value| EffectiveRatio {
            value,
            basis: match style.box_sizing {
                LayoutBoxSizing::ContentBox => RatioBasis::ContentBox,
                LayoutBoxSizing::BorderBox => RatioBasis::BorderBox,
            },
        })
        .or_else(|| {
            input
                .natural
                .natural_aspect_ratio()
                .map(|value| EffectiveRatio {
                    value,
                    basis: RatioBasis::ContentBox,
                })
        });
    let natural_width = input.natural.width;
    let natural_height = input.natural.height;

    let known_width = known_dimensions
        .width
        .map(|value| outer_to_content(node, "width", value, edges.width))
        .transpose()?;
    let known_height = known_dimensions
        .height
        .map(|value| outer_to_content(node, "height", value, edges.height))
        .transpose()?;

    let width_edge = if style.box_sizing == LayoutBoxSizing::BorderBox {
        edges.width
    } else {
        0.0
    };
    let height_edge = if style.box_sizing == LayoutBoxSizing::BorderBox {
        edges.height
    } else {
        0.0
    };
    let css_width = if inherent_size {
        dimension_to_content(node, "width", style.width, width_edge)?
    } else {
        None
    };
    let css_height = if inherent_size {
        dimension_to_content(node, "height", style.height, height_edge)?
    } else {
        None
    };
    let width = known_width.or(css_width);
    let height = known_height.or(css_height);
    let constraints = Constraints::from_style(node, style, edges)?;

    let (width, height) = match (width, height) {
        (Some(width), Some(height)) => (
            clamp(width, constraints.min_width, constraints.max_width),
            clamp(height, constraints.min_height, constraints.max_height),
        ),
        (Some(width), None) => {
            let width = clamp(width, constraints.min_width, constraints.max_width);
            let height = ratio
                .map(|ratio| height_from_width(width, ratio, edges))
                .or(natural_height)
                .unwrap_or(DEFAULT_OBJECT_HEIGHT);
            (
                width,
                clamp(height, constraints.min_height, constraints.max_height),
            )
        }
        (None, Some(height)) => {
            let height = clamp(height, constraints.min_height, constraints.max_height);
            let width = ratio
                .map(|ratio| width_from_height(height, ratio, edges))
                .or(natural_width)
                .unwrap_or(DEFAULT_OBJECT_WIDTH);
            (
                clamp(width, constraints.min_width, constraints.max_width),
                height,
            )
        }
        (None, None) => {
            let natural = unconstrained_size(node, natural_width, natural_height, ratio, edges)?;
            if let Some(ratio) = ratio {
                clamp_replaced_ratio_size(natural, ratio.value, constraints)
            } else {
                (
                    clamp(
                        natural.content_width,
                        constraints.min_width,
                        constraints.max_width,
                    ),
                    clamp(
                        natural.content_height,
                        constraints.min_height,
                        constraints.max_height,
                    ),
                )
            }
        }
    };

    validate_used_size(node, width, height)?;
    let outer_width = width + edges.width;
    let outer_height = height + edges.height;
    validate_used_size(node, outer_width, outer_height)?;
    Ok(UsedReplacedSize {
        content_width: width,
        content_height: height,
        width: outer_width,
        height: outer_height,
    })
}

#[derive(Clone, Copy, Debug, Default)]
struct Edges {
    width: f32,
    height: f32,
}

fn edge_sums(node: NodeId, style: LayoutStyle) -> Result<Edges, LayoutError> {
    let lengths = [
        ("padding-top", style.padding.top),
        ("padding-right", style.padding.right),
        ("padding-bottom", style.padding.bottom),
        ("padding-left", style.padding.left),
    ];
    let mut padding = [0.0; 4];
    for (position, (field, value)) in lengths.into_iter().enumerate() {
        padding[position] = match value {
            LayoutLengthPercentage::LengthPx(value) if value.is_finite() && value >= 0.0 => value,
            LayoutLengthPercentage::LengthPx(_) => {
                return Err(LayoutError::InvalidStyle { node, field });
            }
            LayoutLengthPercentage::Auto
            | LayoutLengthPercentage::Percentage(_)
            | LayoutLengthPercentage::Calc(_) => {
                return Err(unsupported(node, "padding은 절대 CSS px만 지원합니다"));
            }
        };
    }
    let border = [
        ("border-top", style.border.top),
        ("border-right", style.border.right),
        ("border-bottom", style.border.bottom),
        ("border-left", style.border.left),
    ];
    let mut border_px = [0.0; 4];
    for (position, (field, value)) in border.into_iter().enumerate() {
        if !value.is_finite() || value < 0.0 {
            return Err(LayoutError::InvalidStyle { node, field });
        }
        border_px[position] = value;
    }
    let edges = Edges {
        width: padding[1] + padding[3] + border_px[1] + border_px[3],
        height: padding[0] + padding[2] + border_px[0] + border_px[2],
    };
    if !edges.width.is_finite() || !edges.height.is_finite() {
        return Err(unsupported(
            node,
            "padding과 border 합계가 유한 CSS px가 아닙니다",
        ));
    }
    Ok(edges)
}

#[derive(Clone, Copy, Debug, Default)]
struct Constraints {
    min_width: Option<f32>,
    max_width: Option<f32>,
    min_height: Option<f32>,
    max_height: Option<f32>,
}

impl Constraints {
    fn from_style(node: NodeId, style: LayoutStyle, edges: Edges) -> Result<Self, LayoutError> {
        let width_edge = if style.box_sizing == LayoutBoxSizing::BorderBox {
            edges.width
        } else {
            0.0
        };
        let height_edge = if style.box_sizing == LayoutBoxSizing::BorderBox {
            edges.height
        } else {
            0.0
        };
        let min_width = dimension_to_content(node, "min-width", style.min_width, width_edge)?;
        let max_width = dimension_to_content(node, "max-width", style.max_width, width_edge)?;
        let min_height = dimension_to_content(node, "min-height", style.min_height, height_edge)?;
        let max_height = dimension_to_content(node, "max-height", style.max_height, height_edge)?;
        Ok(Self {
            min_width,
            max_width: normalize_max(min_width, max_width),
            min_height,
            max_height: normalize_max(min_height, max_height),
        })
    }
}

fn normalize_max(minimum: Option<f32>, maximum: Option<f32>) -> Option<f32> {
    maximum.map(|maximum| minimum.map_or(maximum, |minimum| maximum.max(minimum)))
}

fn dimension_to_content(
    node: NodeId,
    property: &'static str,
    dimension: LayoutDimension,
    edge: f32,
) -> Result<Option<f32>, LayoutError> {
    match dimension {
        LayoutDimension::Auto => Ok(None),
        LayoutDimension::Fixed(value) if value.is_finite() && value >= 0.0 => {
            Ok(Some((value - edge).max(0.0)))
        }
        LayoutDimension::Fixed(_) => Err(LayoutError::InvalidStyle {
            node,
            field: dimension_field(property),
        }),
        LayoutDimension::Percent(_) | LayoutDimension::Calc(_) => Err(unsupported(
            node,
            "width/height/min/max는 절대 CSS px만 지원합니다",
        )),
    }
}

fn dimension_field(property: &'static str) -> &'static str {
    match property {
        "width" => "width",
        "height" => "height",
        "min-width" => "min_width",
        "max-width" => "max_width",
        "min-height" => "min_height",
        "max-height" => "max_height",
        _ => "width",
    }
}

fn outer_to_content(
    node: NodeId,
    axis: &'static str,
    value: f32,
    edge: f32,
) -> Result<f32, LayoutError> {
    if !value.is_finite() || value < 0.0 {
        return Err(LayoutError::InvalidIntrinsicSize { node, axis });
    }
    Ok((value - edge).max(0.0))
}

fn unconstrained_size(
    node: NodeId,
    natural_width: Option<f32>,
    natural_height: Option<f32>,
    ratio: Option<EffectiveRatio>,
    edges: Edges,
) -> Result<UsedReplacedSize, LayoutError> {
    let size = match (natural_width, natural_height, ratio) {
        (Some(width), Some(_height), Some(ratio)) => UsedReplacedSize {
            content_width: width,
            content_height: height_from_width(width, ratio, edges),
            width: 0.0,
            height: 0.0,
        },
        (Some(width), Some(height), None) => UsedReplacedSize {
            content_width: width,
            content_height: height,
            width: 0.0,
            height: 0.0,
        },
        (Some(width), None, Some(ratio)) => UsedReplacedSize {
            content_width: width,
            content_height: height_from_width(width, ratio, edges),
            width: 0.0,
            height: 0.0,
        },
        (None, Some(height), Some(ratio)) => UsedReplacedSize {
            content_width: width_from_height(height, ratio, edges),
            content_height: height,
            width: 0.0,
            height: 0.0,
        },
        (Some(width), None, None) => UsedReplacedSize {
            content_width: width,
            content_height: DEFAULT_OBJECT_HEIGHT,
            width: 0.0,
            height: 0.0,
        },
        (None, Some(height), None) => UsedReplacedSize {
            content_width: DEFAULT_OBJECT_WIDTH,
            content_height: height,
            width: 0.0,
            height: 0.0,
        },
        (None, None, Some(_)) => {
            return Err(unsupported(
                node,
                "ratio만 있는 auto/auto intrinsic contribution은 지원하지 않습니다",
            ));
        }
        (None, None, None) => UsedReplacedSize {
            content_width: DEFAULT_OBJECT_WIDTH,
            content_height: DEFAULT_OBJECT_HEIGHT,
            width: 0.0,
            height: 0.0,
        },
    };
    validate_used_size(node, size.content_width, size.content_height)?;
    Ok(size)
}

fn height_from_width(width: f32, ratio: EffectiveRatio, edges: Edges) -> f32 {
    match ratio.basis {
        RatioBasis::ContentBox => width / ratio.value,
        RatioBasis::BorderBox => ((width + edges.width) / ratio.value - edges.height).max(0.0),
    }
}

fn width_from_height(height: f32, ratio: EffectiveRatio, edges: Edges) -> f32 {
    match ratio.basis {
        RatioBasis::ContentBox => height * ratio.value,
        RatioBasis::BorderBox => ((height + edges.height) * ratio.value - edges.width).max(0.0),
    }
}

fn clamp_replaced_ratio_size(
    natural: UsedReplacedSize,
    ratio: f32,
    constraints: Constraints,
) -> (f32, f32) {
    let width = natural.content_width;
    let height = natural.content_height;
    let min_width = constraints.min_width.unwrap_or(0.0);
    let min_height = constraints.min_height.unwrap_or(0.0);
    let max_width = constraints.max_width.unwrap_or(f32::INFINITY);
    let max_height = constraints.max_height.unwrap_or(f32::INFINITY);
    let width_too_large = width > max_width;
    let width_too_small = width < min_width;
    let height_too_large = height > max_height;
    let height_too_small = height < min_height;

    match (
        width_too_large || width_too_small,
        height_too_large || height_too_small,
    ) {
        (false, false) => (width, height),
        (true, false) if width_too_large => (max_width, (max_width / ratio).max(min_height)),
        (true, false) => (min_width, (min_width / ratio).min(max_height)),
        (false, true) if height_too_large => ((max_height * ratio).max(min_width), max_height),
        (false, true) => ((min_height * ratio).min(max_width), min_height),
        (true, true) if width_too_large && height_too_large => {
            if max_width / width <= max_height / height {
                (max_width, min_height.max(max_width / ratio))
            } else {
                (min_width.max(max_height * ratio), max_height)
            }
        }
        (true, true) if width_too_small && height_too_small => {
            if min_width / width <= min_height / height {
                (max_width.min(min_height * ratio), min_height)
            } else {
                (min_width, max_height.min(min_width / ratio))
            }
        }
        (true, true) if width_too_small && height_too_large => (min_width, max_height),
        (true, true) => (max_width, min_height),
    }
}

fn clamp(value: f32, minimum: Option<f32>, maximum: Option<f32>) -> f32 {
    let value = maximum.map_or(value, |maximum| value.min(maximum));
    minimum.map_or(value, |minimum| value.max(minimum))
}

fn validate_used_size(node: NodeId, width: f32, height: f32) -> Result<(), LayoutError> {
    for (axis, value) in [("width", width), ("height", height)] {
        if !value.is_finite() || value < 0.0 {
            return Err(LayoutError::InvalidIntrinsicSize { node, axis });
        }
    }
    Ok(())
}

fn unsupported(node: NodeId, reason: &'static str) -> LayoutError {
    LayoutError::UnsupportedIntrinsicSizing { node, reason }
}

#[cfg(test)]
mod tests {
    use super::{Constraints, UsedReplacedSize, clamp_replaced_ratio_size, normalize_max};

    #[test]
    fn css_22_replaced_ratio_min_max_table_covers_all_ten_violations() {
        let natural = UsedReplacedSize {
            content_width: 24.0,
            content_height: 16.0,
            width: 24.0,
            height: 16.0,
        };
        let cases = [
            (
                "width exceeds max",
                Constraints {
                    max_width: Some(20.0),
                    ..Constraints::default()
                },
                (20.0, 13.333333),
            ),
            (
                "width below min",
                Constraints {
                    min_width: Some(30.0),
                    ..Constraints::default()
                },
                (30.0, 20.0),
            ),
            (
                "height exceeds max",
                Constraints {
                    max_height: Some(10.0),
                    ..Constraints::default()
                },
                (15.0, 10.0),
            ),
            (
                "height below min",
                Constraints {
                    min_height: Some(24.0),
                    ..Constraints::default()
                },
                (36.0, 24.0),
            ),
            (
                "both above max, width ratio wins",
                Constraints {
                    max_width: Some(20.0),
                    max_height: Some(14.0),
                    ..Constraints::default()
                },
                (20.0, 13.333333),
            ),
            (
                "both above max, height ratio wins",
                Constraints {
                    max_width: Some(22.0),
                    max_height: Some(12.0),
                    ..Constraints::default()
                },
                (18.0, 12.0),
            ),
            (
                "both below min, height ratio wins",
                Constraints {
                    min_width: Some(30.0),
                    min_height: Some(22.0),
                    ..Constraints::default()
                },
                (33.0, 22.0),
            ),
            (
                "both below min, width ratio wins",
                Constraints {
                    min_width: Some(36.0),
                    min_height: Some(20.0),
                    ..Constraints::default()
                },
                (36.0, 24.0),
            ),
            (
                "width below min and height above max",
                Constraints {
                    min_width: Some(30.0),
                    max_height: Some(10.0),
                    ..Constraints::default()
                },
                (30.0, 10.0),
            ),
            (
                "width above max and height below min",
                Constraints {
                    max_width: Some(20.0),
                    min_height: Some(20.0),
                    ..Constraints::default()
                },
                (20.0, 20.0),
            ),
        ];

        for (name, constraints, expected) in cases {
            let actual = clamp_replaced_ratio_size(natural, 1.5, constraints);
            assert!(
                (actual.0 - expected.0).abs() < 0.0001 && (actual.1 - expected.1).abs() < 0.0001,
                "{name}: {actual:?} != {expected:?}"
            );
        }
    }

    #[test]
    fn minimum_wins_when_maximum_is_smaller() {
        assert_eq!(normalize_max(Some(48.0), Some(30.0)), Some(48.0));
        assert_eq!(normalize_max(None, Some(30.0)), Some(30.0));
    }
}

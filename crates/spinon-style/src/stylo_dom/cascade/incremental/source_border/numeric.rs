use style::typed_om::{MathValue, NumericValue};

#[derive(Clone, Copy)]
pub(in crate::stylo_dom::cascade::incremental) struct CssLengthContext {
    pub font_size_css_px: f64,
    pub root_font_size_css_px: f64,
    pub viewport_width_css_px: f64,
    pub viewport_height_css_px: f64,
}

#[derive(Clone, Copy)]
pub(super) enum CssNumeric {
    Number(f64),
    Length(f64),
}

pub(super) fn evaluate_css_length(
    value: &NumericValue,
    context: CssLengthContext,
    depth: usize,
    remaining_nodes: &mut usize,
) -> Option<CssNumeric> {
    if depth > 32 || *remaining_nodes == 0 {
        return None;
    }
    *remaining_nodes -= 1;
    match value {
        NumericValue::Unit(unit) => {
            let value = f64::from(unit.value);
            match unit.unit_str().to_ascii_lowercase().as_str() {
                "number" => Some(CssNumeric::Number(value)),
                "px" => Some(CssNumeric::Length(value)),
                "in" => Some(CssNumeric::Length(value * 96.0)),
                "cm" => Some(CssNumeric::Length(value * (96.0 / 2.54))),
                "mm" => Some(CssNumeric::Length(value * (96.0 / 25.4))),
                "q" => Some(CssNumeric::Length(value * (96.0 / 101.6))),
                "pt" => Some(CssNumeric::Length(value * (96.0 / 72.0))),
                "pc" => Some(CssNumeric::Length(value * 16.0)),
                "em" => Some(CssNumeric::Length(value * context.font_size_css_px)),
                "rem" => Some(CssNumeric::Length(value * context.root_font_size_css_px)),
                "vw" | "lvw" | "svw" | "dvw" | "vi" | "lvi" | "svi" | "dvi" => Some(
                    CssNumeric::Length(value * context.viewport_width_css_px / 100.0),
                ),
                "vh" | "lvh" | "svh" | "dvh" | "vb" | "lvb" | "svb" | "dvb" => Some(
                    CssNumeric::Length(value * context.viewport_height_css_px / 100.0),
                ),
                "vmin" | "lvmin" | "svmin" | "dvmin" => Some(CssNumeric::Length(
                    value
                        * context
                            .viewport_width_css_px
                            .min(context.viewport_height_css_px)
                        / 100.0,
                )),
                "vmax" | "lvmax" | "svmax" | "dvmax" => Some(CssNumeric::Length(
                    value
                        * context
                            .viewport_width_css_px
                            .max(context.viewport_height_css_px)
                        / 100.0,
                )),
                _ => None,
            }
        }
        NumericValue::Math(math) => match math {
            MathValue::Sum(value) => evaluate_sum(&value.values, context, depth, remaining_nodes),
            MathValue::Product(value) => {
                evaluate_product(&value.values, context, depth, remaining_nodes)
            }
            MathValue::Negate(value) => {
                evaluate_css_length(&value.value, context, depth + 1, remaining_nodes)
                    .map(|value| map_numeric(value, |n| -n))
            }
            MathValue::Invert(value) => {
                match evaluate_css_length(&value.value, context, depth + 1, remaining_nodes)? {
                    CssNumeric::Number(value) if value != 0.0 => {
                        Some(CssNumeric::Number(1.0 / value))
                    }
                    _ => None,
                }
            }
            MathValue::Min(value) => {
                evaluate_extreme(&value.values, context, depth, remaining_nodes, f64::min)
            }
            MathValue::Max(value) => {
                evaluate_extreme(&value.values, context, depth, remaining_nodes, f64::max)
            }
            MathValue::Clamp(value) => {
                let [min, center, max] = &*value.values;
                let min = evaluate_css_length(min, context, depth + 1, remaining_nodes)?;
                let center = evaluate_css_length(center, context, depth + 1, remaining_nodes)?;
                let max = evaluate_css_length(max, context, depth + 1, remaining_nodes)?;
                clamp_numeric(min, center, max)
            }
        },
    }
}

fn evaluate_sum(
    values: &[NumericValue],
    context: CssLengthContext,
    depth: usize,
    remaining_nodes: &mut usize,
) -> Option<CssNumeric> {
    let mut result = None;
    for value in values {
        let value = evaluate_css_length(value, context, depth + 1, remaining_nodes)?;
        result = Some(match (result, value) {
            (None, value) => value,
            (Some(CssNumeric::Number(a)), CssNumeric::Number(b)) => CssNumeric::Number(a + b),
            (Some(CssNumeric::Length(a)), CssNumeric::Length(b)) => CssNumeric::Length(a + b),
            _ => return None,
        });
    }
    result
}

fn evaluate_product(
    values: &[NumericValue],
    context: CssLengthContext,
    depth: usize,
    remaining_nodes: &mut usize,
) -> Option<CssNumeric> {
    let mut value = 1.0;
    let mut length = false;
    for item in values {
        match evaluate_css_length(item, context, depth + 1, remaining_nodes)? {
            CssNumeric::Number(number) => value *= number,
            CssNumeric::Length(px) if !length => {
                value *= px;
                length = true;
            }
            CssNumeric::Length(_) => return None,
        }
    }
    Some(if length {
        CssNumeric::Length(value)
    } else {
        CssNumeric::Number(value)
    })
}

fn evaluate_extreme(
    values: &[NumericValue],
    context: CssLengthContext,
    depth: usize,
    remaining_nodes: &mut usize,
    compare: fn(f64, f64) -> f64,
) -> Option<CssNumeric> {
    let mut result = None;
    for value in values {
        let value = evaluate_css_length(value, context, depth + 1, remaining_nodes)?;
        result = Some(match (result, value) {
            (None, value) => value,
            (Some(CssNumeric::Number(a)), CssNumeric::Number(b)) => {
                CssNumeric::Number(compare(a, b))
            }
            (Some(CssNumeric::Length(a)), CssNumeric::Length(b)) => {
                CssNumeric::Length(compare(a, b))
            }
            _ => return None,
        });
    }
    result
}

fn clamp_numeric(min: CssNumeric, center: CssNumeric, max: CssNumeric) -> Option<CssNumeric> {
    match (min, center, max) {
        (CssNumeric::Number(min), CssNumeric::Number(center), CssNumeric::Number(max)) => {
            Some(CssNumeric::Number(center.clamp(min, max)))
        }
        (CssNumeric::Length(min), CssNumeric::Length(center), CssNumeric::Length(max)) => {
            Some(CssNumeric::Length(center.clamp(min, max)))
        }
        _ => None,
    }
}

fn map_numeric(value: CssNumeric, map: impl FnOnce(f64) -> f64) -> CssNumeric {
    match value {
        CssNumeric::Number(value) => CssNumeric::Number(map(value)),
        CssNumeric::Length(value) => CssNumeric::Length(map(value)),
    }
}

use spinon_core::NodeId;

use crate::LayoutError;

mod property;
pub use property::{LayoutCalcId, LayoutCssMathProperty};

/// Stylo Typed OM에서 복사한 제한된 `<length-percentage>` 수학 트리입니다.
#[derive(Clone, Debug, PartialEq)]
pub enum LayoutCssMath {
    Number(f32),
    LengthPx(f32),
    Percentage(f32),
    Sum(Vec<Self>),
    Product(Vec<Self>),
    Negate(Box<Self>),
    Invert(Box<Self>),
    Min(Vec<Self>),
    Max(Vec<Self>),
    Clamp {
        min: Box<Self>,
        value: Box<Self>,
        max: Box<Self>,
    },
}

/// 계산식과 출처를 한 레이아웃 입력에 묶습니다.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutCssMathValue {
    pub id: LayoutCalcId,
    pub node_id: NodeId,
    pub property: LayoutCssMathProperty,
    pub expression: LayoutCssMath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MathDimension {
    Number,
    LengthPercentage,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct EvaluatedMath {
    value: f64,
    dimension: MathDimension,
}

const MAX_MATH_DEPTH: usize = 32;
const MAX_MATH_NODES: usize = 256;
/// 고정 Chromium의 CSS layout unit 상한(33,554,428 CSS px)입니다.
const MAX_CSS_LAYOUT_VALUE: f64 = 33_554_428.0;

impl LayoutCssMath {
    pub(crate) fn contains_percentage(&self) -> Result<bool, &'static str> {
        let mut remaining = MAX_MATH_NODES;
        self.contains_percentage_at(0, &mut remaining)
    }

    fn contains_percentage_at(
        &self,
        depth: usize,
        remaining: &mut usize,
    ) -> Result<bool, &'static str> {
        if depth > MAX_MATH_DEPTH || *remaining == 0 {
            return Err("CSS 계산식의 깊이 또는 노드 수 한도를 넘었습니다");
        }
        *remaining -= 1;
        match self {
            Self::Percentage(_) => Ok(true),
            Self::Number(_) | Self::LengthPx(_) => Ok(false),
            Self::Sum(values) | Self::Product(values) | Self::Min(values) | Self::Max(values) => {
                if values.is_empty() || values.len() > 64 {
                    return Err("CSS 계산식 연산자 인자 수가 유효하지 않습니다");
                }
                let mut contains = false;
                for value in values {
                    contains |= value.contains_percentage_at(depth + 1, remaining)?;
                }
                Ok(contains)
            }
            Self::Negate(value) | Self::Invert(value) => {
                value.contains_percentage_at(depth + 1, remaining)
            }
            Self::Clamp { min, value, max } => Ok(min
                .contains_percentage_at(depth + 1, remaining)?
                | value.contains_percentage_at(depth + 1, remaining)?
                | max.contains_percentage_at(depth + 1, remaining)?),
        }
    }

    pub(crate) fn resolve(
        &self,
        basis: f32,
        property: LayoutCssMathProperty,
    ) -> Result<f32, &'static str> {
        if !basis.is_finite() || basis < 0.0 {
            return Err("percentage 기준이 유한한 0 이상 값이 아닙니다");
        }
        let mut remaining = MAX_MATH_NODES;
        let value = self.evaluate_at(basis as f64, 0, &mut remaining)?;
        if value.dimension != MathDimension::LengthPercentage {
            return Err("최종 계산값이 length-percentage가 아닙니다");
        }
        Ok(censor(value.value, property))
    }

    fn evaluate_at(
        &self,
        basis: f64,
        depth: usize,
        remaining: &mut usize,
    ) -> Result<EvaluatedMath, &'static str> {
        if depth > MAX_MATH_DEPTH || *remaining == 0 {
            return Err("CSS 계산식의 깊이 또는 노드 수 한도를 넘었습니다");
        }
        *remaining -= 1;
        let number = |value: f32| {
            Some(EvaluatedMath {
                value: value as f64,
                dimension: MathDimension::Number,
            })
        };
        let length = |value: f32| {
            Some(EvaluatedMath {
                value: value as f64,
                dimension: MathDimension::LengthPercentage,
            })
        };
        match self {
            Self::Number(value) => number(*value).ok_or("CSS 계산식 number가 유한하지 않습니다"),
            Self::LengthPx(value) => length(*value).ok_or("CSS 계산식 px 길이가 유한하지 않습니다"),
            Self::Percentage(value) => {
                let mut output =
                    length(*value).ok_or("CSS 계산식 percentage가 유한하지 않습니다")?;
                output.value *= basis;
                Ok(output)
            }
            Self::Sum(values) => evaluate_sum(values, basis, depth, remaining),
            Self::Product(values) => evaluate_product(values, basis, depth, remaining),
            Self::Negate(value) => {
                let mut result = value.evaluate_at(basis, depth + 1, remaining)?;
                result.value = -result.value;
                Ok(result)
            }
            Self::Invert(value) => {
                let result = value.evaluate_at(basis, depth + 1, remaining)?;
                if result.dimension != MathDimension::Number {
                    return Err("CSS 계산식은 length-percentage를 역수로 만들 수 없습니다");
                }
                Ok(EvaluatedMath {
                    value: 1.0 / result.value,
                    dimension: MathDimension::Number,
                })
            }
            Self::Min(values) => evaluate_extreme(values, basis, depth, remaining, false),
            Self::Max(values) => evaluate_extreme(values, basis, depth, remaining, true),
            Self::Clamp { min, value, max } => {
                let minimum = min.evaluate_at(basis, depth + 1, remaining)?;
                let value = value.evaluate_at(basis, depth + 1, remaining)?;
                let maximum = max.evaluate_at(basis, depth + 1, remaining)?;
                if minimum.dimension != value.dimension || value.dimension != maximum.dimension {
                    return Err("clamp() 인자의 CSS 계산 차원이 서로 다릅니다");
                }
                let resolved =
                    if minimum.value.is_nan() || value.value.is_nan() || maximum.value.is_nan() {
                        f64::NAN
                    } else if minimum.value > maximum.value {
                        minimum.value
                    } else {
                        css_max(minimum.value, css_min(value.value, maximum.value))
                    };
                Ok(EvaluatedMath {
                    value: resolved,
                    dimension: value.dimension,
                })
            }
        }
    }
}

fn evaluate_sum(
    values: &[LayoutCssMath],
    basis: f64,
    depth: usize,
    remaining: &mut usize,
) -> Result<EvaluatedMath, &'static str> {
    if values.is_empty() || values.len() > 64 {
        return Err("sum 연산자 인자 수가 유효하지 않습니다");
    }
    let mut output = values[0].evaluate_at(basis, depth + 1, remaining)?;
    for item in &values[1..] {
        let item = item.evaluate_at(basis, depth + 1, remaining)?;
        if output.dimension != item.dimension {
            return Err("sum 연산자의 CSS 계산 차원이 서로 다릅니다");
        }
        output.value += item.value;
    }
    Ok(output)
}

fn evaluate_product(
    values: &[LayoutCssMath],
    basis: f64,
    depth: usize,
    remaining: &mut usize,
) -> Result<EvaluatedMath, &'static str> {
    if values.is_empty() || values.len() > 64 {
        return Err("product 연산자 인자 수가 유효하지 않습니다");
    }
    let mut output = EvaluatedMath {
        value: 1.0,
        dimension: MathDimension::Number,
    };
    for value in values {
        let value = value.evaluate_at(basis, depth + 1, remaining)?;
        if output.dimension == MathDimension::LengthPercentage
            && value.dimension == MathDimension::LengthPercentage
        {
            return Err("두 length-percentage 값을 곱할 수 없습니다");
        }
        output.value *= value.value;
        if value.dimension == MathDimension::LengthPercentage {
            output.dimension = MathDimension::LengthPercentage;
        }
    }
    Ok(output)
}

fn evaluate_extreme(
    values: &[LayoutCssMath],
    basis: f64,
    depth: usize,
    remaining: &mut usize,
    maximum: bool,
) -> Result<EvaluatedMath, &'static str> {
    if values.is_empty() || values.len() > 64 {
        return Err("min()/max() 인자 수가 유효하지 않습니다");
    }
    let mut output = values[0].evaluate_at(basis, depth + 1, remaining)?;
    for value in &values[1..] {
        let value = value.evaluate_at(basis, depth + 1, remaining)?;
        if output.dimension != value.dimension {
            return Err("min()/max() 인자의 CSS 계산 차원이 서로 다릅니다");
        }
        output.value = if maximum {
            css_max(output.value, value.value)
        } else {
            css_min(output.value, value.value)
        };
    }
    Ok(output)
}

fn css_min(left: f64, right: f64) -> f64 {
    if left == 0.0 && right == 0.0 {
        if left.is_sign_negative() || right.is_sign_negative() {
            -0.0
        } else {
            0.0
        }
    } else if left.is_nan() || right.is_nan() {
        f64::NAN
    } else if left < right {
        left
    } else {
        right
    }
}

fn css_max(left: f64, right: f64) -> f64 {
    if left == 0.0 && right == 0.0 {
        if left.is_sign_positive() || right.is_sign_positive() {
            0.0
        } else {
            -0.0
        }
    } else if left.is_nan() || right.is_nan() {
        f64::NAN
    } else if left > right {
        left
    } else {
        right
    }
}

fn censor(value: f64, property: LayoutCssMathProperty) -> f32 {
    let value = if value.is_nan() {
        0.0
    } else if value == f64::INFINITY {
        MAX_CSS_LAYOUT_VALUE
    } else if value == f64::NEG_INFINITY {
        if property.is_nonnegative() {
            0.0
        } else {
            -MAX_CSS_LAYOUT_VALUE
        }
    } else {
        value.clamp(-MAX_CSS_LAYOUT_VALUE, MAX_CSS_LAYOUT_VALUE)
    };
    if property.is_nonnegative() {
        value.max(0.0) as f32
    } else {
        value as f32
    }
}

impl LayoutCssMathValue {
    pub(crate) fn contains_percentage(&self) -> Result<bool, &'static str> {
        self.expression.contains_percentage()
    }

    pub(crate) fn resolve(&self, basis: f32) -> Result<f32, &'static str> {
        self.expression.resolve(basis, self.property)
    }

    pub(crate) fn error(&self, reason: &'static str) -> LayoutError {
        LayoutError::InvalidCssMath {
            node: self.node_id,
            property: self.property.name(),
            reason,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_non_finite_intermediates_are_censored_only_after_evaluation() {
        let clamp_nan = LayoutCssMath::Clamp {
            min: Box::new(LayoutCssMath::LengthPx(10.0)),
            value: Box::new(LayoutCssMath::LengthPx(f32::NAN)),
            max: Box::new(LayoutCssMath::LengthPx(20.0)),
        };
        let positive_infinity = LayoutCssMath::Product(vec![
            LayoutCssMath::LengthPx(10.0),
            LayoutCssMath::Invert(Box::new(LayoutCssMath::Number(0.0))),
        ]);
        let negative_infinity = LayoutCssMath::Product(vec![
            LayoutCssMath::LengthPx(-1.0),
            LayoutCssMath::Invert(Box::new(LayoutCssMath::Number(0.0))),
        ]);

        assert_eq!(
            clamp_nan.resolve(0.0, LayoutCssMathProperty::Width),
            Ok(0.0)
        );
        assert_eq!(
            positive_infinity.resolve(0.0, LayoutCssMathProperty::Width),
            Ok(MAX_CSS_LAYOUT_VALUE as f32)
        );
        assert_eq!(
            negative_infinity.resolve(0.0, LayoutCssMathProperty::MarginLeft),
            Ok(-(MAX_CSS_LAYOUT_VALUE as f32))
        );
        assert_eq!(
            negative_infinity.resolve(0.0, LayoutCssMathProperty::Width),
            Ok(0.0)
        );
    }

    #[test]
    fn non_finite_percentage_is_still_classified_as_percentage() {
        assert_eq!(
            LayoutCssMath::Percentage(f32::NAN).contains_percentage(),
            Ok(true)
        );
        assert_eq!(
            LayoutCssMath::Min(vec![
                LayoutCssMath::LengthPx(f32::NAN),
                LayoutCssMath::LengthPx(3.0),
            ])
            .resolve(0.0, LayoutCssMathProperty::Width),
            Ok(0.0)
        );
    }

    #[test]
    fn invalid_math_dimensions_and_resource_limits_still_fail() {
        assert!(matches!(
            LayoutCssMath::Sum(vec![
                LayoutCssMath::Number(1.0),
                LayoutCssMath::LengthPx(1.0),
            ])
            .resolve(0.0, LayoutCssMathProperty::Width),
            Err("sum 연산자의 CSS 계산 차원이 서로 다릅니다")
        ));
        assert!(matches!(
            LayoutCssMath::Min(Vec::new()).contains_percentage(),
            Err("CSS 계산식 연산자 인자 수가 유효하지 않습니다")
        ));
        assert!(matches!(
            LayoutCssMath::Sum(vec![LayoutCssMath::Number(1.0); 65]).contains_percentage(),
            Err("CSS 계산식 연산자 인자 수가 유효하지 않습니다")
        ));
        assert!(matches!(
            LayoutCssMath::Sum(
                (0..6)
                    .map(|_| LayoutCssMath::Sum(vec![LayoutCssMath::Number(1.0); 43]))
                    .collect()
            )
            .contains_percentage(),
            Err("CSS 계산식의 깊이 또는 노드 수 한도를 넘었습니다")
        ));
        let too_deep = (0..=MAX_MATH_DEPTH).fold(LayoutCssMath::LengthPx(1.0), |value, _| {
            LayoutCssMath::Negate(Box::new(value))
        });
        assert!(matches!(
            too_deep.contains_percentage(),
            Err("CSS 계산식의 깊이 또는 노드 수 한도를 넘었습니다")
        ));
        assert!(matches!(
            LayoutCssMath::Invert(Box::new(LayoutCssMath::LengthPx(1.0)))
                .resolve(0.0, LayoutCssMathProperty::Width),
            Err("CSS 계산식은 length-percentage를 역수로 만들 수 없습니다")
        ));
        assert!(matches!(
            LayoutCssMath::Product(vec![
                LayoutCssMath::LengthPx(1.0),
                LayoutCssMath::Percentage(0.5)
            ])
            .resolve(100.0, LayoutCssMathProperty::Width),
            Err("두 length-percentage 값을 곱할 수 없습니다")
        ));
        assert!(matches!(
            LayoutCssMath::Max(vec![
                LayoutCssMath::Number(1.0),
                LayoutCssMath::LengthPx(2.0)
            ])
            .resolve(0.0, LayoutCssMathProperty::Width),
            Err("min()/max() 인자의 CSS 계산 차원이 서로 다릅니다")
        ));
        assert!(matches!(
            LayoutCssMath::Clamp {
                min: Box::new(LayoutCssMath::LengthPx(1.0)),
                value: Box::new(LayoutCssMath::Number(2.0)),
                max: Box::new(LayoutCssMath::LengthPx(3.0)),
            }
            .resolve(0.0, LayoutCssMathProperty::Width),
            Err("clamp() 인자의 CSS 계산 차원이 서로 다릅니다")
        ));
        assert!(matches!(
            LayoutCssMath::LengthPx(10.0).resolve(-1.0, LayoutCssMathProperty::Width),
            Err("percentage 기준이 유한한 0 이상 값이 아닙니다")
        ));
    }
}

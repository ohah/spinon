use spinon_core::NodeId;
use spinon_layout::{LayoutCalcId, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue};
use spinon_style::ComputedCssMath;

use crate::StyleLayoutError;

#[derive(Default)]
pub(super) struct CssMathProjector {
    values: Vec<LayoutCssMathValue>,
}

impl CssMathProjector {
    pub(super) fn project(
        &mut self,
        node_id: NodeId,
        property: LayoutCssMathProperty,
        expression: &ComputedCssMath,
    ) -> Result<LayoutCalcId, StyleLayoutError> {
        let id = LayoutCalcId(u32::try_from(self.values.len()).map_err(|_| {
            StyleLayoutError::UnsupportedComputedValue {
                node: node_id,
                property: property.name(),
                value: "레이아웃 계산식 ID 범위를 초과했습니다".to_owned(),
            }
        })?);
        self.values.push(LayoutCssMathValue {
            id,
            node_id,
            property,
            expression: owned_math(expression),
        });
        Ok(id)
    }

    pub(super) fn into_values(self) -> Vec<LayoutCssMathValue> {
        self.values
    }
}

fn owned_math(value: &ComputedCssMath) -> LayoutCssMath {
    match value {
        ComputedCssMath::Number(value) => LayoutCssMath::Number(*value),
        ComputedCssMath::LengthPx(value) => LayoutCssMath::LengthPx(*value),
        ComputedCssMath::Percentage(value) => LayoutCssMath::Percentage(*value),
        ComputedCssMath::Sum(values) => LayoutCssMath::Sum(values.iter().map(owned_math).collect()),
        ComputedCssMath::Product(values) => {
            LayoutCssMath::Product(values.iter().map(owned_math).collect())
        }
        ComputedCssMath::Negate(value) => LayoutCssMath::Negate(Box::new(owned_math(value))),
        ComputedCssMath::Invert(value) => LayoutCssMath::Invert(Box::new(owned_math(value))),
        ComputedCssMath::Min(values) => LayoutCssMath::Min(values.iter().map(owned_math).collect()),
        ComputedCssMath::Max(values) => LayoutCssMath::Max(values.iter().map(owned_math).collect()),
        ComputedCssMath::Clamp { min, value, max } => LayoutCssMath::Clamp {
            min: Box::new(owned_math(min)),
            value: Box::new(owned_math(value)),
            max: Box::new(owned_math(max)),
        },
    }
}

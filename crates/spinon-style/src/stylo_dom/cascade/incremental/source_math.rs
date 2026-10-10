use std::collections::{BTreeMap, BTreeSet};

use style::{
    properties::{LonghandId, PropertyDeclaration, PropertyDeclarationId},
    rule_tree::StrongRuleNode,
    shared_lock::StylesheetGuards,
    typed_om::{ToTyped, TypedValue},
    values::specified::length::NonNegativeLengthPercentageOrNormal,
    values::specified::{FlexBasis, LengthPercentage, Margin, MaxSize, Size},
};

use super::{super::ComputedCssMath, dimensions::computed_math};

/// Stylo cascade 결과가 계산 과정에서 단순화한 calc AST를 winning rule에서 보완합니다.
pub(super) fn winning_layout_math_values(
    rules: &StrongRuleNode,
    guards: &StylesheetGuards<'_>,
) -> BTreeMap<String, ComputedCssMath> {
    let mut seen = BTreeSet::<&'static str>::new();
    let mut output = BTreeMap::new();

    for node in rules.self_and_ancestors() {
        let Some(source) = node.style_source() else {
            continue;
        };
        let guard = node.cascade_level().guard(guards);
        let block = source.read(guard);
        for (declaration, importance) in block
            .declaration_importance_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            if node.cascade_level().is_important() != importance.important() {
                continue;
            }
            let Some(property) = layout_property(declaration) else {
                continue;
            };
            if !seen.insert(property) {
                continue;
            }
            if let Some(value) = specified_math(declaration) {
                output.insert(property.to_owned(), value);
            }
        }
    }

    output
}

fn layout_property(declaration: &PropertyDeclaration) -> Option<&'static str> {
    let PropertyDeclarationId::Longhand(id) = declaration.id() else {
        return None;
    };
    let name = match id {
        LonghandId::Width => "width",
        LonghandId::Height => "height",
        LonghandId::MinWidth => "min-width",
        LonghandId::MaxWidth => "max-width",
        LonghandId::MinHeight => "min-height",
        LonghandId::MaxHeight => "max-height",
        LonghandId::FlexBasis => "flex-basis",
        LonghandId::MarginTop => "margin-top",
        LonghandId::MarginRight => "margin-right",
        LonghandId::MarginBottom => "margin-bottom",
        LonghandId::MarginLeft => "margin-left",
        LonghandId::PaddingTop => "padding-top",
        LonghandId::PaddingRight => "padding-right",
        LonghandId::PaddingBottom => "padding-bottom",
        LonghandId::PaddingLeft => "padding-left",
        LonghandId::RowGap => "row-gap",
        LonghandId::ColumnGap => "column-gap",
        _ => return None,
    };
    Some(name)
}

fn specified_math(declaration: &PropertyDeclaration) -> Option<ComputedCssMath> {
    let value = match declaration {
        PropertyDeclaration::Width(Size::LengthPercentage(value))
        | PropertyDeclaration::Height(Size::LengthPercentage(value))
        | PropertyDeclaration::MinWidth(Size::LengthPercentage(value))
        | PropertyDeclaration::MinHeight(Size::LengthPercentage(value)) => &value.0,
        PropertyDeclaration::MaxWidth(MaxSize::LengthPercentage(value))
        | PropertyDeclaration::MaxHeight(MaxSize::LengthPercentage(value)) => &value.0,
        PropertyDeclaration::FlexBasis(value) => match value.as_ref() {
            FlexBasis::Size(Size::LengthPercentage(value)) => &value.0,
            _ => return None,
        },
        PropertyDeclaration::MarginTop(Margin::LengthPercentage(value))
        | PropertyDeclaration::MarginRight(Margin::LengthPercentage(value))
        | PropertyDeclaration::MarginBottom(Margin::LengthPercentage(value))
        | PropertyDeclaration::MarginLeft(Margin::LengthPercentage(value)) => value,
        PropertyDeclaration::PaddingTop(value)
        | PropertyDeclaration::PaddingRight(value)
        | PropertyDeclaration::PaddingBottom(value)
        | PropertyDeclaration::PaddingLeft(value) => &value.0,
        PropertyDeclaration::RowGap(NonNegativeLengthPercentageOrNormal::LengthPercentage(
            value,
        ))
        | PropertyDeclaration::ColumnGap(NonNegativeLengthPercentageOrNormal::LengthPercentage(
            value,
        )) => &value.0,
        _ => return None,
    };
    if !matches!(value, LengthPercentage::Calc(_)) {
        return None;
    }
    let TypedValue::Numeric(value) = value.to_typed_value()? else {
        return None;
    };
    computed_math(&value, 0, &mut 256)
}

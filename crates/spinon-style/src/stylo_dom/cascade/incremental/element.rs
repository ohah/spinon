use selectors::matching::{
    MatchingContext, MatchingForInvalidation, MatchingMode, NeedsSelectorFlags, SelectorCaches,
};
use std::collections::BTreeMap;

use spinon_core::NodeId;
use style::{
    applicable_declarations::ApplicableDeclarationList,
    context::{CascadeInputs, TreeCountingCaches},
    dom::TElement,
    properties::{ComputedValues, FirstLineReparenting, LonghandId, PropertyDeclarationId},
    rule_cache::RuleCacheConditions,
    rule_tree::RuleCascadeFlags,
    selector_parser::SelectorImpl,
    shared_lock::StylesheetGuards,
    stylist::{RuleInclusion, Stylist},
};

use super::super::{
    ComputedCssMath, ComputedElementStyle, ComputedStyleProfile, CssCascadeError, runtime_paint,
};
use super::dimensions::{computed_layout_dimensions, computed_layout_math_values};
use super::source_math::winning_layout_math_values;
use super::spacing::computed_layout_spacing;
use crate::StyloElement;

pub(super) fn computed_element_output(
    properties: &[(&str, LonghandId)],
    profile: ComputedStyleProfile,
    computed: &ComputedValues,
    node_id: NodeId,
    source_math: BTreeMap<String, ComputedCssMath>,
) -> Result<ComputedElementStyle, CssCascadeError> {
    let (background_color, background_paint) =
        runtime_paint::computed_background_for_profile(profile, computed, node_id)?;
    let font_size_css_px = computed.get_font().clone_font_size().computed_size().px();
    Ok(ComputedElementStyle {
        node_id,
        font_size_css_px,
        layout_dimensions: computed_layout_dimensions(computed),
        layout_spacing: computed_layout_spacing(computed),
        layout_math_values: computed_layout_math_values(computed, source_math),
        properties: properties
            .iter()
            .map(|(name, id)| {
                (
                    (*name).to_owned(),
                    computed.computed_value_to_string(PropertyDeclarationId::Longhand(*id)),
                )
            })
            .collect(),
        background_color,
        background_paint,
    })
}

pub(super) fn compute_element_style(
    stylist: &Stylist,
    element: StyloElement<'_>,
    guards: &StylesheetGuards<'_>,
    parent_style: Option<&ComputedValues>,
) -> style::servo_arc::Arc<ComputedValues> {
    compute_element_style_with_math(stylist, element, guards, parent_style).0
}

pub(super) fn compute_element_style_with_math(
    stylist: &Stylist,
    element: StyloElement<'_>,
    guards: &StylesheetGuards<'_>,
    parent_style: Option<&ComputedValues>,
) -> (
    style::servo_arc::Arc<ComputedValues>,
    BTreeMap<String, ComputedCssMath>,
) {
    let mut selector_caches = SelectorCaches::default();
    let mut matching_context = MatchingContext::<SelectorImpl>::new(
        MatchingMode::Normal,
        None,
        &mut selector_caches,
        element.view.quirks_mode(),
        NeedsSelectorFlags::Yes,
        MatchingForInvalidation::No,
    );
    let mut declarations = ApplicableDeclarationList::new();
    stylist.push_applicable_declarations(
        element,
        None,
        element.style_attribute(),
        None,
        Default::default(),
        RuleInclusion::All,
        &mut declarations,
        &mut matching_context,
    );
    let rules = stylist
        .rule_tree()
        .compute_rule_node(&mut declarations, guards);
    let source_math = winning_layout_math_values(&rules, guards);
    let inputs = CascadeInputs {
        rules: Some(rules),
        visited_rules: None,
        flags: matching_context.extra_data.cascade_input_flags,
        included_cascade_flags: RuleCascadeFlags::empty(),
    };
    let computed = stylist.cascade_style_and_visited(
        Some(element),
        None,
        &inputs,
        guards,
        parent_style,
        parent_style,
        FirstLineReparenting::No,
        &Default::default(),
        None,
        &mut RuleCacheConditions::default(),
        &mut TreeCountingCaches::default(),
    );
    (computed, source_math)
}

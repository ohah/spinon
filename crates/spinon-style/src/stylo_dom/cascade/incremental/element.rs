use selectors::matching::{
    MatchingContext, MatchingForInvalidation, MatchingMode, NeedsSelectorFlags, SelectorCaches,
};
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

use super::super::{ComputedElementStyle, ComputedStyleProfile, CssCascadeError, runtime_paint};
use crate::StyloElement;

pub(super) fn computed_element_output(
    properties: &[(&str, LonghandId)],
    profile: ComputedStyleProfile,
    computed: &ComputedValues,
    node_id: NodeId,
) -> Result<ComputedElementStyle, CssCascadeError> {
    let (background_color, background_paint) =
        runtime_paint::computed_background_for_profile(profile, computed, node_id)?;
    Ok(ComputedElementStyle {
        node_id,
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
    let inputs = CascadeInputs {
        rules: Some(rules),
        visited_rules: None,
        flags: matching_context.extra_data.cascade_input_flags,
        included_cascade_flags: RuleCascadeFlags::empty(),
    };
    stylist.cascade_style_and_visited(
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
    )
}

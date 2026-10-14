use selectors::matching::{
    MatchingContext, MatchingForInvalidation, MatchingMode, NeedsSelectorFlags, SelectorCaches,
};
use std::collections::{BTreeMap, HashMap};

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
    ComputedCssMath, ComputedCssOverflow, ComputedCssOverflowClipMargin,
    ComputedCssOverflowVisualBox, ComputedCssPosition, ComputedCssVisibility, ComputedCssZIndex,
    ComputedElementStyle, ComputedStyleProfile, CssCascadeError, CssViewport,
    FixedContainingBlockEffect, runtime_paint,
};
use super::dimensions::{computed_layout_dimensions, computed_layout_math_values};
use super::source_border::{
    CssLengthContext, computed_border_width_values, winning_border_width_sources,
};
use super::source_math::winning_layout_math_values;
use super::spacing::{computed_layout_insets, computed_layout_spacing};
use super::{computed_layout_aspect_ratio, computed_layout_border};
use crate::StyloElement;

pub(super) fn computed_element_output(
    properties: &[(&str, LonghandId)],
    profile: ComputedStyleProfile,
    computed: &ComputedValues,
    node_id: NodeId,
    ancestor_hidden: bool,
    source_math: BTreeMap<String, ComputedCssMath>,
    source_border_widths: BTreeMap<&'static str, f32>,
) -> Result<ComputedElementStyle, CssCascadeError> {
    let (background_color, background_paint) =
        runtime_paint::computed_background_for_profile(profile, computed, node_id)?;
    let font_size_css_px = computed.get_font().clone_font_size().computed_size().px();
    let layout_border = computed_layout_border(computed, &source_border_widths);
    let layout_position = match computed.clone_position() {
        style::values::computed::PositionProperty::Static => ComputedCssPosition::Static,
        style::values::computed::PositionProperty::Relative => ComputedCssPosition::Relative,
        style::values::computed::PositionProperty::Absolute => ComputedCssPosition::Absolute,
        style::values::computed::PositionProperty::Fixed => ComputedCssPosition::Fixed,
        style::values::computed::PositionProperty::Sticky => ComputedCssPosition::Sticky,
    };
    let fixed_containing_block_effects = fixed_containing_block_effects(computed);
    let stacking_z_index = matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
    )
    .then(|| match computed.clone_z_index() {
        style::values::computed::ZIndex::Auto => ComputedCssZIndex::Auto,
        style::values::computed::ZIndex::Integer(value) => ComputedCssZIndex::Integer(value),
    });
    let stacking_visibility = matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
    )
    .then(|| match computed.clone_visibility() {
        style::properties::longhands::visibility::computed_value::T::Visible => {
            ComputedCssVisibility::Visible
        }
        style::properties::longhands::visibility::computed_value::T::Hidden => {
            ComputedCssVisibility::Hidden
        }
        style::properties::longhands::visibility::computed_value::T::Collapse => {
            ComputedCssVisibility::Collapse
        }
    });
    let has_overflow = profile == ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1;
    let layout_overflow_x = has_overflow.then(|| computed_overflow(computed.clone_overflow_x()));
    let layout_overflow_y = has_overflow.then(|| computed_overflow(computed.clone_overflow_y()));
    let overflow_clip_margin = has_overflow.then(|| {
        let value = computed.clone_overflow_clip_margin();
        ComputedCssOverflowClipMargin {
            visual_box: match value.visual_box {
                style::values::generics::box_::OverflowClipMarginBox::ContentBox => {
                    ComputedCssOverflowVisualBox::ContentBox
                }
                style::values::generics::box_::OverflowClipMarginBox::PaddingBox => {
                    ComputedCssOverflowVisualBox::PaddingBox
                }
                style::values::generics::box_::OverflowClipMarginBox::BorderBox => {
                    ComputedCssOverflowVisualBox::BorderBox
                }
            },
            offset_css_px: value.offset.px(),
        }
    });
    let layout_aspect_ratio = if matches!(
        profile,
        ComputedStyleProfile::RuntimeFlexLayoutV1
            | ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
    ) {
        computed_layout_aspect_ratio(computed, node_id)?
    } else {
        None
    };
    let mut computed_properties = properties
        .iter()
        .map(|(name, id)| {
            (
                (*name).to_owned(),
                computed.computed_value_to_string(PropertyDeclarationId::Longhand(*id)),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let layout_suppressed = ancestor_hidden
        || computed_properties
            .get("display")
            .is_some_and(|value| value == "none");
    let grid_layout = if !layout_suppressed
        && matches!(
            profile,
            ComputedStyleProfile::RuntimeBlockFlexGridV1
                | ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
                | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
                | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        ) {
        if computed_properties
            .get("display")
            .is_some_and(|value| value == "grid")
        {
            Some(super::grid::computed_grid_layout(
                computed,
                node_id,
                &computed_properties,
                profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
            )?)
        } else {
            None
        }
    } else {
        None
    };
    let grid_item_placement = (matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexGridPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementV1
            | ComputedStyleProfile::RuntimeBlockFlexGridAutoPlacementDenseV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
    ) && !layout_suppressed)
        .then(|| {
            super::grid::computed_grid_item_placement(
                computed,
                &computed_properties,
                profile == ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1,
            )
        });
    for (name, width) in [
        ("border-top-width", layout_border.top),
        ("border-right-width", layout_border.right),
        ("border-bottom-width", layout_border.bottom),
        ("border-left-width", layout_border.left),
    ] {
        if let Some(value) = computed_properties.get_mut(name) {
            *value = format!("{width:.0}px");
        }
    }
    Ok(ComputedElementStyle {
        node_id,
        layout_position,
        stacking_z_index,
        stacking_visibility,
        layout_overflow_x,
        layout_overflow_y,
        overflow_clip_margin,
        fixed_containing_block_effects,
        layout_insets: computed_layout_insets(computed),
        font_size_css_px,
        layout_dimensions: computed_layout_dimensions(computed),
        layout_aspect_ratio,
        layout_spacing: computed_layout_spacing(computed),
        grid_layout,
        grid_item_placement,
        layout_border,
        layout_math_values: computed_layout_math_values(computed, source_math),
        properties: computed_properties,
        background_color,
        background_paint,
    })
}

fn computed_overflow(value: style::values::computed::Overflow) -> ComputedCssOverflow {
    match value {
        style::values::computed::Overflow::Visible => ComputedCssOverflow::Visible,
        style::values::computed::Overflow::Clip => ComputedCssOverflow::Clip,
        style::values::computed::Overflow::Hidden => ComputedCssOverflow::Hidden,
        style::values::computed::Overflow::Scroll => ComputedCssOverflow::Scroll,
        style::values::computed::Overflow::Auto => ComputedCssOverflow::Auto,
    }
}

fn fixed_containing_block_effects(computed: &ComputedValues) -> Vec<FixedContainingBlockEffect> {
    let css_value =
        |property| computed.computed_value_to_string(PropertyDeclarationId::Longhand(property));
    let mut effects = Vec::new();
    for (property, effect) in [
        (LonghandId::Transform, FixedContainingBlockEffect::Transform),
        (LonghandId::Translate, FixedContainingBlockEffect::Translate),
        (LonghandId::Rotate, FixedContainingBlockEffect::Rotate),
        (LonghandId::Scale, FixedContainingBlockEffect::Scale),
        (
            LonghandId::OffsetPath,
            FixedContainingBlockEffect::MotionPath,
        ),
        (
            LonghandId::Perspective,
            FixedContainingBlockEffect::Perspective,
        ),
        (LonghandId::Filter, FixedContainingBlockEffect::Filter),
        (
            LonghandId::BackdropFilter,
            FixedContainingBlockEffect::BackdropFilter,
        ),
    ] {
        if css_value(property) != "none" {
            effects.push(effect);
        }
    }
    if css_value(LonghandId::TransformStyle) == "preserve-3d" {
        effects.push(FixedContainingBlockEffect::TransformStylePreserve3d);
    }

    let contain = computed.clone_contain();
    if contain.contains(style::values::computed::Contain::LAYOUT) {
        effects.push(FixedContainingBlockEffect::LayoutContainment);
    }
    if contain.contains(style::values::computed::Contain::PAINT) {
        effects.push(FixedContainingBlockEffect::PaintContainment);
    }

    let will_change = css_value(LonghandId::WillChange);
    let has_will_change =
        |property: &str| will_change.split(',').any(|value| value.trim() == property);
    if ["transform", "translate", "rotate", "scale", "offset-path"]
        .into_iter()
        .any(has_will_change)
    {
        effects.push(FixedContainingBlockEffect::WillChangeTransform);
    }
    for (property, effect) in [
        (
            "perspective",
            FixedContainingBlockEffect::WillChangePerspective,
        ),
        ("filter", FixedContainingBlockEffect::WillChangeFilter),
        (
            "backdrop-filter",
            FixedContainingBlockEffect::WillChangeBackdropFilter,
        ),
        ("contain", FixedContainingBlockEffect::WillChangeContain),
    ] {
        if has_will_change(property) {
            effects.push(effect);
        }
    }
    effects
}

pub(super) fn compute_element_style(
    stylist: &Stylist,
    element: StyloElement<'_>,
    guards: &StylesheetGuards<'_>,
    parent_style: Option<&ComputedValues>,
    viewport: CssViewport,
    stylesheet_sources: &HashMap<usize, String>,
) -> style::servo_arc::Arc<ComputedValues> {
    compute_element_style_with_math(
        stylist,
        element,
        guards,
        parent_style,
        viewport,
        stylesheet_sources,
    )
    .0
}

pub(super) fn compute_element_style_with_math(
    stylist: &Stylist,
    element: StyloElement<'_>,
    guards: &StylesheetGuards<'_>,
    parent_style: Option<&ComputedValues>,
    viewport: CssViewport,
    stylesheet_sources: &HashMap<usize, String>,
) -> (
    style::servo_arc::Arc<ComputedValues>,
    BTreeMap<String, ComputedCssMath>,
    BTreeMap<&'static str, f32>,
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
    let source_border_width_sources = winning_border_width_sources(
        &rules,
        guards,
        element.data().inline_style.as_ref(),
        stylesheet_sources,
    );
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
    let length_context = CssLengthContext {
        font_size_css_px: f64::from(computed.get_font().clone_font_size().computed_size().px()),
        root_font_size_css_px: f64::from(stylist.device().root_font_size().px()),
        viewport_width_css_px: f64::from(viewport.width_css_px),
        viewport_height_css_px: f64::from(viewport.height_css_px),
    };
    let source_border_widths = computed_border_width_values(
        &source_border_width_sources,
        &computed,
        element.view.quirks_mode(),
        element.data().inline_style_text.as_deref(),
        element.view.document_base_url(),
        element.view.shared_lock(),
        length_context,
    );
    (computed, source_math, source_border_widths)
}

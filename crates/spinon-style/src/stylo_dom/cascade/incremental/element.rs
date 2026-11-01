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

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
use super::super::ComputedCssZoom;
use super::super::{
    ComputedCssMath, ComputedCssOpacity, ComputedCssOverflow, ComputedCssOverflowClipMargin,
    ComputedCssOverflowVisualBox, ComputedCssOverscrollBehavior, ComputedCssPosition,
    ComputedCssTouchAction, ComputedCssVisibility, ComputedCssZIndex, ComputedElementStyle,
    ComputedStyleProfile, CssCascadeError, CssViewport, FixedContainingBlockEffect, runtime_paint,
};
use super::dimensions::{computed_layout_dimensions, computed_layout_math_values};
use super::source_border::{
    CssLengthContext, computed_border_width_values, winning_border_width_sources,
};
use super::source_math::winning_layout_math_values;
use super::source_word_break::{WordBreakSource, winning_word_break_source};
use super::spacing::{computed_layout_insets, computed_layout_spacing};
use super::{
    computed_aspect_ratio_serialization, computed_layout_aspect_ratio, computed_layout_border,
};
use crate::StyloElement;

pub(super) fn computed_element_output(
    properties: &[(&str, LonghandId)],
    profile: ComputedStyleProfile,
    computed: &ComputedValues,
    node_id: NodeId,
    ancestor_hidden: bool,
    source_overrides: ComputedElementSourceOverrides,
) -> Result<ComputedElementStyle, CssCascadeError> {
    let ComputedElementSourceOverrides {
        math_values: source_math,
        border_widths: source_border_widths,
        word_break_is_break_word,
    } = source_overrides;
    let (background_color, background_paint) =
        runtime_paint::computed_background_for_profile(profile, computed, node_id)?;
    let (computed_color, computed_background_color) =
        runtime_paint::computed_color_for_profile(profile, computed, node_id)?;
    let font_size_css_px = computed.get_font().clone_font_size().computed_size().px();
    let layout_border = computed_layout_border(computed, &source_border_widths);
    let has_overflow = matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::RuntimeRootHtmlTextPaintV1
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    );
    let layout_border_paint = has_overflow
        .then(|| {
            if profile == ComputedStyleProfile::C19BoxPaintOpacityV1 {
                runtime_paint::computed_c19_runtime_border_paint(computed, node_id, layout_border)
            } else {
                runtime_paint::computed_runtime_border_paint(computed, node_id, layout_border)
            }
        })
        .transpose()?;
    let layout_position = match computed.clone_position() {
        style::values::computed::PositionProperty::Static => ComputedCssPosition::Static,
        style::values::computed::PositionProperty::Relative => ComputedCssPosition::Relative,
        style::values::computed::PositionProperty::Absolute => ComputedCssPosition::Absolute,
        style::values::computed::PositionProperty::Fixed => ComputedCssPosition::Fixed,
        style::values::computed::PositionProperty::Sticky => ComputedCssPosition::Sticky,
    };
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    let c11_4_zoom =
        (profile == ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1).then(|| {
            ComputedCssZoom {
                specified: computed.clone_zoom().value(),
                effective: computed.effective_zoom.value(),
            }
        });
    let fixed_containing_block_effects = fixed_containing_block_effects(computed);
    let stacking_z_index = matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
    )
    .then(|| match computed.clone_z_index() {
        style::values::computed::ZIndex::Auto => ComputedCssZIndex::Auto,
        style::values::computed::ZIndex::Integer(value) => ComputedCssZIndex::Integer(value),
    });
    let stacking_opacity = matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
    )
    .then(|| {
        let opacity = computed.clone_opacity();
        ComputedCssOpacity::new(opacity).ok_or(CssCascadeError::UnsupportedComputedOpacity {
            node: node_id,
            value: opacity,
        })
    })
    .transpose()?;
    let stacking_visibility = matches!(
        profile,
        ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
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
    let has_scroll_gesture = profile == ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1;
    let input_overscroll_behavior_x = has_scroll_gesture
        .then(|| computed_overscroll_behavior(computed.clone_overscroll_behavior_x()));
    let input_overscroll_behavior_y = has_scroll_gesture
        .then(|| computed_overscroll_behavior(computed.clone_overscroll_behavior_y()));
    let input_touch_action =
        has_scroll_gesture.then(|| computed_touch_action(computed.clone_touch_action()));
    let layout_aspect_ratio = if matches!(
        profile,
        ComputedStyleProfile::RuntimeFlexLayoutV1
            | ComputedStyleProfile::RuntimeFlexPaintV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesMediaEnvironmentV1
            | ComputedStyleProfile::RuntimeFlexCustomPropertiesPaintV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesV1
            | ComputedStyleProfile::RuntimeFlexRegisteredPropertiesPaintV1
            | ComputedStyleProfile::RuntimeBlockPositioningV1
            | ComputedStyleProfile::RuntimeBlockFlexStackingV1
            | ComputedStyleProfile::RuntimeBlockFlexOverflowClipV1
            | ComputedStyleProfile::C19BoxPaintOpacityV1
            | ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    ) {
        computed_layout_aspect_ratio(computed, node_id, profile)?
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
    if word_break_is_break_word && computed_properties.contains_key("word-break") {
        computed_properties.insert("word-break".to_owned(), "break-word".to_owned());
    }
    if computed_properties.contains_key("aspect-ratio") {
        computed_properties.insert(
            "aspect-ratio".to_owned(),
            computed_aspect_ratio_serialization(computed),
        );
    }
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
                | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        ) {
        if computed_properties
            .get("display")
            .is_some_and(|value| value == "grid")
        {
            Some(super::grid::computed_grid_layout(
                computed,
                node_id,
                &computed_properties,
                matches!(
                    profile,
                    ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
                ),
                profile == ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
                profile == ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1,
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
            | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
    ) && !layout_suppressed)
        .then(|| {
            super::grid::computed_grid_item_placement(
                computed,
                &computed_properties,
                matches!(
                    profile,
                    ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
                        | ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
                ),
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
        #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
        c11_4_zoom,
        stacking_opacity,
        stacking_visibility,
        layout_overflow_x,
        layout_overflow_y,
        overflow_clip_margin,
        input_overscroll_behavior_x,
        input_overscroll_behavior_y,
        input_touch_action,
        fixed_containing_block_effects,
        layout_insets: computed_layout_insets(computed),
        font_size_css_px,
        layout_dimensions: computed_layout_dimensions(computed),
        layout_aspect_ratio,
        layout_spacing: computed_layout_spacing(computed),
        grid_layout,
        grid_item_placement,
        layout_border,
        layout_border_paint,
        layout_math_values: computed_layout_math_values(computed, source_math),
        properties: computed_properties,
        background_color,
        background_paint,
        computed_color,
        computed_background_color,
    })
}

pub(super) struct ComputedElementSourceOverrides {
    pub(super) math_values: BTreeMap<String, ComputedCssMath>,
    pub(super) border_widths: BTreeMap<&'static str, f32>,
    pub(super) word_break_is_break_word: bool,
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

fn computed_overscroll_behavior(
    value: style::values::computed::OverscrollBehavior,
) -> ComputedCssOverscrollBehavior {
    match value {
        style::values::computed::OverscrollBehavior::Auto => ComputedCssOverscrollBehavior::Auto,
        style::values::computed::OverscrollBehavior::Contain => {
            ComputedCssOverscrollBehavior::Contain
        }
        style::values::computed::OverscrollBehavior::None => ComputedCssOverscrollBehavior::None,
    }
}

fn computed_touch_action(value: style::values::computed::TouchAction) -> ComputedCssTouchAction {
    use style::values::computed::TouchAction;

    if value == TouchAction::AUTO {
        ComputedCssTouchAction::Auto
    } else if value == TouchAction::NONE {
        ComputedCssTouchAction::None
    } else if value == TouchAction::PAN_X {
        ComputedCssTouchAction::PanX
    } else if value == TouchAction::PAN_Y {
        ComputedCssTouchAction::PanY
    } else {
        ComputedCssTouchAction::Unsupported
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
    parent_word_break_is_break_word: bool,
    viewport: CssViewport,
    stylesheet_sources: &HashMap<usize, String>,
) -> (style::servo_arc::Arc<ComputedValues>, bool) {
    let (computed, _, _, word_break_is_break_word) = compute_element_style_with_math(
        stylist,
        element,
        guards,
        parent_style,
        parent_word_break_is_break_word,
        viewport,
        stylesheet_sources,
    );
    (computed, word_break_is_break_word)
}

pub(super) fn compute_element_style_with_math(
    stylist: &Stylist,
    element: StyloElement<'_>,
    guards: &StylesheetGuards<'_>,
    parent_style: Option<&ComputedValues>,
    parent_word_break_is_break_word: bool,
    viewport: CssViewport,
    stylesheet_sources: &HashMap<usize, String>,
) -> (
    style::servo_arc::Arc<ComputedValues>,
    BTreeMap<String, ComputedCssMath>,
    BTreeMap<&'static str, f32>,
    bool,
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
    let word_break_is_break_word = match winning_word_break_source(
        &rules,
        guards,
        element.data().inline_style.as_ref(),
        element.data().inline_style_text.as_deref(),
        stylesheet_sources,
    ) {
        Some(WordBreakSource::BreakWord) => true,
        Some(WordBreakSource::Inherit) | None => parent_word_break_is_break_word,
        Some(WordBreakSource::Revert | WordBreakSource::RevertLayer) => false,
        Some(WordBreakSource::Other) => false,
    };
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
    (
        computed,
        source_math,
        source_border_widths,
        word_break_is_break_word,
    )
}

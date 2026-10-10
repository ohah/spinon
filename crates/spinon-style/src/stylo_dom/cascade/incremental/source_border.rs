use std::collections::{BTreeMap, BTreeSet, HashMap};

use style::{
    properties::{
        LonghandId, PropertyDeclaration, PropertyDeclarationBlock, PropertyDeclarationId,
    },
    rule_tree::StrongRuleNode,
    servo_arc::Arc,
    shared_lock::{Locked, StylesheetGuards},
};

#[path = "source_border/numeric.rs"]
mod numeric;
#[path = "source_border/values.rs"]
mod values;

pub(super) use numeric::CssLengthContext;
pub(super) use values::computed_border_width_values;

type BorderWidthSources = BTreeMap<&'static str, BorderWidthSource>;

pub(super) struct BorderWidthSource {
    css: Option<String>,
    from_inline_style: bool,
    raw_declarations: Option<String>,
    important: bool,
}

/// Winning declarations retain their authored CSS so layout can apply its CSS-pixel width rule
/// before Stylo's device-pixel border snapping loses fractional precision.
pub(super) fn winning_border_width_sources(
    rules: &StrongRuleNode,
    guards: &StylesheetGuards<'_>,
    inline_style: Option<&Arc<Locked<PropertyDeclarationBlock>>>,
    stylesheet_sources: &HashMap<usize, String>,
) -> BorderWidthSources {
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
            let Some(property) = border_width_property(declaration) else {
                continue;
            };
            if !seen.insert(property) {
                continue;
            }

            let mut css = String::new();
            let css = declaration
                .to_css(&mut css)
                .is_ok()
                .then_some(css)
                .filter(|css| !css.is_empty());
            let from_inline_style =
                inline_style.is_some_and(|inline| Arc::ptr_eq(source.get(), inline));
            let raw_declarations = (!from_inline_style)
                .then(|| {
                    stylesheet_sources
                        .get(&(source.get().raw_ptr().as_ptr() as usize))
                        .cloned()
                })
                .flatten();
            output.insert(
                property,
                BorderWidthSource {
                    css,
                    from_inline_style,
                    raw_declarations,
                    important: importance.important(),
                },
            );
        }
    }

    output
}

fn border_width_property(declaration: &PropertyDeclaration) -> Option<&'static str> {
    let PropertyDeclarationId::Longhand(id) = declaration.id() else {
        return None;
    };
    match id {
        LonghandId::BorderTopWidth => Some("border-top-width"),
        LonghandId::BorderRightWidth => Some("border-right-width"),
        LonghandId::BorderBottomWidth => Some("border-bottom-width"),
        LonghandId::BorderLeftWidth => Some("border-left-width"),
        _ => None,
    }
}

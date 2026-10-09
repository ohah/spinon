use style::{
    properties::PropertyDeclarationId, shared_lock::SharedRwLockReadGuard, stylesheets::CssRule,
};

use super::{CssOrigin, StylesheetRegistry};

pub(super) fn first_unsupported_author_feature(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_layer_rules(registry, allowed_properties, false)
}

pub(super) fn first_unsupported_author_feature_with_layers(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_layer_rules(registry, allowed_properties, true)
}

fn first_unsupported_author_feature_with_layer_rules(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
    allow_layers: bool,
) -> Option<(String, String)> {
    let guard = registry.shared_lock.read();
    for stylesheet in registry
        .stylesheets
        .iter()
        .filter(|stylesheet| stylesheet.origin == CssOrigin::Author)
    {
        let contents = stylesheet.sheet.0.contents.read_with(&guard);
        let rules = contents.rules.read_with(&guard);
        for rule in &rules.0 {
            if let Some(feature) =
                unsupported_rule_feature(rule, &guard, allowed_properties, allow_layers)
            {
                return Some((stylesheet.id.clone(), feature));
            }
        }
    }
    None
}

fn unsupported_rule_feature(
    rule: &CssRule,
    guard: &SharedRwLockReadGuard<'_>,
    allowed_properties: &[&str],
    allow_layers: bool,
) -> Option<String> {
    match rule {
        CssRule::LayerStatement(_) if allow_layers => None,
        CssRule::LayerBlock(layer) if allow_layers => {
            let rules = layer.rules.read_with(guard);
            rules.0.iter().find_map(|nested| {
                unsupported_rule_feature(nested, guard, allowed_properties, allow_layers)
            })
        }
        CssRule::Style(rule) => {
            let rule = rule.read_with(guard);
            if rule.rules.is_some() {
                return Some("중첩 CSS 규칙".to_owned());
            }
            let block = rule.block.read_with(guard);
            for property in block.property_ids().iter() {
                let name = match property {
                    PropertyDeclarationId::Longhand(id) => id.name().to_owned(),
                    PropertyDeclarationId::Custom(_) => "사용자 지정 속성".to_owned(),
                };
                if !allowed_properties.contains(&name.as_str()) {
                    return Some(format!("지원하지 않는 CSS 선언 {name}"));
                }
            }
            None
        }
        _ => Some("at-rule 또는 비스타일 규칙".to_owned()),
    }
}

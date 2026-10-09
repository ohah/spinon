use style::{
    properties::PropertyDeclarationId, shared_lock::SharedRwLockReadGuard, stylesheets::CssRule,
};
use style_traits::ToCss;

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

pub(super) fn first_unsupported_author_feature_with_media(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_rules(
        registry,
        allowed_properties,
        false,
        true,
        false,
        false,
    )
}

pub(super) fn first_unsupported_runtime_author_feature(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
    allow_custom_properties: bool,
    allow_background_color: bool,
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_rules(
        registry,
        allowed_properties,
        false,
        false,
        allow_custom_properties,
        allow_background_color,
    )
}

fn first_unsupported_author_feature_with_layer_rules(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
    allow_layers: bool,
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_rules(
        registry,
        allowed_properties,
        allow_layers,
        false,
        false,
        false,
    )
}

fn first_unsupported_author_feature_with_rules(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
    allow_layers: bool,
    allow_media: bool,
    allow_custom_properties: bool,
    allow_background_color: bool,
) -> Option<(String, String)> {
    let guard = registry.shared_lock.read();
    for stylesheet in registry
        .stylesheets
        .iter()
        .filter(|stylesheet| stylesheet.origin == CssOrigin::Author)
    {
        // Stylo 0.22 exposes these typed parser events through stable diagnostic prefixes.
        if allow_media
            && stylesheet
                .diagnostics
                .iter()
                .any(|diagnostic| is_unsupported_media_or_at_rule_diagnostic(&diagnostic.message))
        {
            return Some((
                stylesheet.id.clone(),
                "지원하지 않거나 파싱할 수 없는 media query·at-rule".to_owned(),
            ));
        }
        let contents = stylesheet.sheet.0.contents.read_with(&guard);
        let rules = contents.rules.read_with(&guard);
        for rule in &rules.0 {
            if let Some(feature) = unsupported_rule_feature(
                rule,
                &guard,
                allowed_properties,
                allow_layers,
                allow_media,
                allow_custom_properties,
                allow_background_color,
            ) {
                return Some((stylesheet.id.clone(), feature));
            }
        }
    }
    None
}

fn is_unsupported_media_or_at_rule_diagnostic(message: &str) -> bool {
    message.starts_with("Invalid media rule:")
        || message.starts_with("Invalid rule: '@")
        || message.starts_with("Unsupported rule: '@")
}

fn unsupported_rule_feature(
    rule: &CssRule,
    guard: &SharedRwLockReadGuard<'_>,
    allowed_properties: &[&str],
    allow_layers: bool,
    allow_media: bool,
    allow_custom_properties: bool,
    allow_background_color: bool,
) -> Option<String> {
    match rule {
        CssRule::LayerStatement(_) if allow_layers => None,
        CssRule::LayerBlock(layer) if allow_layers => {
            let rules = layer.rules.read_with(guard);
            rules.0.iter().find_map(|nested| {
                unsupported_rule_feature(
                    nested,
                    guard,
                    allowed_properties,
                    allow_layers,
                    allow_media,
                    allow_custom_properties,
                    allow_background_color,
                )
            })
        }
        CssRule::Media(media) if allow_media => {
            let media_queries = media.media_queries.read_with(guard);
            let serialized = media_queries.to_css_string();
            if !supported_media_query_surface(&serialized) {
                return Some("지원 범위 밖 media query feature".to_owned());
            }
            let rules = media.rules.read_with(guard);
            rules.0.iter().find_map(|nested| {
                unsupported_rule_feature(
                    nested,
                    guard,
                    allowed_properties,
                    allow_layers,
                    allow_media,
                    allow_custom_properties,
                    allow_background_color,
                )
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
                let allowed = match property {
                    PropertyDeclarationId::Custom(_) => allow_custom_properties,
                    PropertyDeclarationId::Longhand(_) => {
                        allowed_properties.contains(&name.as_str())
                            || (allow_background_color && name == "background-color")
                    }
                };
                if !allowed {
                    return Some(format!("지원하지 않는 CSS 선언 {name}"));
                }
            }
            None
        }
        _ => Some("at-rule 또는 비스타일 규칙".to_owned()),
    }
}

fn supported_media_query_surface(source: &str) -> bool {
    const ALLOWED_IDENTIFIERS: &[&str] = &[
        "all",
        "and",
        "any-hover",
        "any-pointer",
        "coarse",
        "dark",
        "fine",
        "hover",
        "light",
        "none",
        "not",
        "only",
        "pointer",
        "prefers-color-scheme",
        "screen",
    ];

    let mut parser = cssparser::Parser::new(source);
    while !parser.is_exhausted() {
        let token = match parser.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(_) => return false,
        };
        match token {
            cssparser::Token::Ident(value) if ALLOWED_IDENTIFIERS.contains(&value.as_ref()) => {}
            cssparser::Token::ParenthesisBlock => {
                if parser
                    .parse_nested_block(supported_media_query_block)
                    .is_err()
                {
                    return false;
                }
            }
            cssparser::Token::WhiteSpace(_)
            | cssparser::Token::Comment(_)
            | cssparser::Token::Comma => {}
            cssparser::Token::Colon => {}
            _ => return false,
        }
    }
    true
}

fn supported_media_query_block<'i>(
    parser: &mut cssparser::Parser<'i>,
) -> Result<(), cssparser::ParseError<()>> {
    let mut feature = None;
    let mut value = None;
    let mut saw_colon = false;
    while !parser.is_exhausted() {
        let token = parser.next_including_whitespace_and_comments()?.clone();
        match token {
            cssparser::Token::Ident(ident) if !saw_colon && feature.is_none() => {
                feature = Some(ident.to_ascii_lowercase());
            }
            cssparser::Token::Ident(ident) if saw_colon && value.is_none() => {
                value = Some(ident.to_ascii_lowercase());
            }
            cssparser::Token::WhiteSpace(_) | cssparser::Token::Comment(_) => {}
            cssparser::Token::Colon if feature.is_some() && !saw_colon => saw_colon = true,
            _ => return Err(cssparser::ParseError::custom(())),
        }
    }
    match (feature.as_deref(), value.as_deref()) {
        (Some("prefers-color-scheme"), Some("light" | "dark"))
        | (Some("pointer" | "any-pointer"), Some("none" | "coarse" | "fine"))
        | (Some("hover" | "any-hover"), Some("none" | "hover")) => Ok(()),
        _ => Err(cssparser::ParseError::custom(())),
    }
}

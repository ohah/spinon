use cssparser::{Parser, Token};
use style::{
    properties::{PropertyDeclaration, PropertyDeclarationId},
    shared_lock::SharedRwLockReadGuard,
    stylesheets::CssRule,
};
use style_traits::ToCss;

use super::{CssOrigin, StylesheetRegistry};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy)]
struct AuthorFeaturePolicy {
    allow_layers: bool,
    allow_media: bool,
    allow_viewport_dimensions: bool,
    allow_custom_properties: bool,
    custom_property_allowlist: Option<&'static [&'static str]>,
    allow_background_color: bool,
    allow_property_registration: bool,
    allow_border_image_initial_resets: bool,
    allow_visibility_collapse: bool,
}

pub(super) fn first_unsupported_author_feature(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_layer_rules(registry, allowed_properties, false, false)
}

pub(super) fn first_unsupported_author_feature_with_visibility_collapse(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_layer_rules(registry, allowed_properties, false, true)
}

pub(super) fn first_unsupported_runtime_block_image_value(
    registry: &StylesheetRegistry,
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
            let CssRule::Style(rule) = rule else {
                continue;
            };
            let rule = rule.read_with(&guard);
            let block = rule.block.read_with(&guard);
            for declaration in block.declarations() {
                if declaration.id().name().as_ref() != "object-position" {
                    continue;
                }
                let mut value = String::new();
                if declaration.to_css(&mut value).is_err()
                    || !supports_runtime_image_object_position(&value)
                {
                    return Some((
                        stylesheet.id.clone(),
                        format!("지원하지 않는 object-position value {value}"),
                    ));
                }
            }
        }
    }
    None
}

fn supports_runtime_image_object_position(value: &str) -> bool {
    let mut components = value.split_whitespace();
    let Some(x) = components.next() else {
        return false;
    };
    let Some(y) = components.next() else {
        return false;
    };
    if components.next().is_some() {
        return false;
    }
    [x, y].into_iter().all(|component| {
        component
            .strip_suffix('%')
            .and_then(|number| number.parse::<f32>().ok())
            .is_some_and(f32::is_finite)
    })
}

pub(super) fn first_unsupported_author_feature_with_layers(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_layer_rules(registry, allowed_properties, true, false)
}

pub(super) fn first_unsupported_author_feature_with_media(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_rules(
        registry,
        allowed_properties,
        AuthorFeaturePolicy {
            allow_layers: false,
            allow_media: true,
            allow_viewport_dimensions: false,
            allow_custom_properties: false,
            custom_property_allowlist: None,
            allow_background_color: false,
            allow_property_registration: false,
            allow_border_image_initial_resets: false,
            allow_visibility_collapse: false,
        },
    )
}

pub(super) fn first_unsupported_runtime_author_feature_with_media(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_rules(
        registry,
        allowed_properties,
        AuthorFeaturePolicy {
            allow_layers: false,
            allow_media: true,
            allow_viewport_dimensions: true,
            allow_custom_properties: true,
            custom_property_allowlist: None,
            allow_background_color: false,
            allow_property_registration: false,
            allow_border_image_initial_resets: true,
            allow_visibility_collapse: false,
        },
    )
}

pub(super) fn first_unsupported_runtime_root_html_system_color(
    registry: &StylesheetRegistry,
) -> Option<(String, String)> {
    let guard = registry.shared_lock.read();
    for stylesheet in registry
        .stylesheets
        .iter()
        .filter(|stylesheet| stylesheet.origin == CssOrigin::Author)
    {
        let contents = stylesheet.sheet.0.contents.read_with(&guard);
        let rules = contents.rules.read_with(&guard);
        if rules
            .0
            .iter()
            .any(|rule| rule_contains_system_color(rule, &guard))
        {
            return Some((
                stylesheet.id.clone(),
                "root HTML Canvas 배경 profile에서 CSS system color와 이를 담는 사용자 지정 속성을 지원하지 않습니다".to_owned(),
            ));
        }
    }
    None
}

fn rule_contains_system_color(rule: &CssRule, guard: &SharedRwLockReadGuard<'_>) -> bool {
    match rule {
        CssRule::Style(rule) => {
            let rule = rule.read_with(guard);
            let block = rule.block.read_with(guard);
            block.declarations().iter().any(|declaration| {
                let relevant = match declaration.id() {
                    PropertyDeclarationId::Longhand(id) => id.name() == "background-color",
                    PropertyDeclarationId::Custom(_) => true,
                };
                relevant && declaration_contains_css_system_color(declaration)
            })
        }
        CssRule::Media(media) => {
            let rules = media.rules.read_with(guard);
            rules
                .0
                .iter()
                .any(|nested| rule_contains_system_color(nested, guard))
        }
        _ => false,
    }
}

pub(crate) fn declaration_contains_css_system_color(declaration: &PropertyDeclaration) -> bool {
    let mut serialized = String::new();
    if declaration.to_css(&mut serialized).is_err() {
        return true;
    }
    css_contains_system_color_identifier(&serialized)
}

pub(crate) fn supports_runtime_visibility_value(declaration: &PropertyDeclaration) -> bool {
    let mut serialized = String::new();
    if declaration.to_css(&mut serialized).is_err() {
        return false;
    }
    matches!(
        serialized.as_str(),
        "visible" | "hidden" | "inherit" | "initial" | "unset" | "revert" | "revert-layer"
    )
}

pub(crate) fn supports_runtime_visibility_collapse_value(
    declaration: &PropertyDeclaration,
) -> bool {
    let mut serialized = String::new();
    declaration.to_css(&mut serialized).is_ok() && serialized == "collapse"
}

const CSS_SYSTEM_COLOR_KEYWORDS: &[&str] = &[
    "accentcolor",
    "accentcolortext",
    "activetext",
    "linktext",
    "visitedtext",
    "buttonborder",
    "buttonface",
    "buttontext",
    "canvas",
    "canvastext",
    "field",
    "fieldtext",
    "graytext",
    "highlight",
    "highlighttext",
    "mark",
    "marktext",
    "selecteditem",
    "selecteditemtext",
    "activeborder",
    "inactiveborder",
    "threeddarkshadow",
    "threedhighlight",
    "threedlightshadow",
    "threedshadow",
    "windowframe",
    "buttonhighlight",
    "buttonshadow",
    "threedface",
    "activecaption",
    "appworkspace",
    "background",
    "inactivecaption",
    "infobackground",
    "menu",
    "scrollbar",
    "window",
    "captiontext",
    "infotext",
    "menutext",
    "windowtext",
    "inactivecaptiontext",
];

fn css_contains_system_color_identifier(source: &str) -> bool {
    let mut parser = Parser::new(source);
    parser_contains_system_color_identifier(&mut parser)
}

fn parser_contains_system_color_identifier(parser: &mut Parser<'_>) -> bool {
    while !parser.is_exhausted() {
        let token = match parser.next_including_whitespace_and_comments() {
            Ok(token) => token,
            Err(_) => return true,
        };
        let contains_system_color = match token {
            Token::Ident(value)
                if CSS_SYSTEM_COLOR_KEYWORDS
                    .iter()
                    .any(|system_color| value.eq_ignore_ascii_case(system_color)) =>
            {
                true
            }
            Token::Function(_)
            | Token::ParenthesisBlock
            | Token::CurlyBracketBlock
            | Token::SquareBracketBlock => parser
                .parse_nested_block(|nested| {
                    Ok::<bool, cssparser::ParseError<()>>(parser_contains_system_color_identifier(
                        nested,
                    ))
                })
                .unwrap_or(true),
            _ => false,
        };
        if contains_system_color {
            return true;
        }
    }
    false
}

pub(super) fn first_unsupported_runtime_author_feature(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
    allow_layers: bool,
    allow_custom_properties: bool,
    allow_background_color: bool,
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_rules(
        registry,
        allowed_properties,
        AuthorFeaturePolicy {
            allow_layers,
            allow_media: false,
            allow_viewport_dimensions: false,
            allow_custom_properties,
            custom_property_allowlist: None,
            allow_background_color,
            allow_property_registration: false,
            allow_border_image_initial_resets: true,
            allow_visibility_collapse: false,
        },
    )
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
pub(super) fn first_unsupported_runtime_hyphens_author_feature(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_rules(
        registry,
        allowed_properties,
        AuthorFeaturePolicy {
            allow_layers: false,
            allow_media: false,
            allow_viewport_dimensions: false,
            allow_custom_properties: true,
            custom_property_allowlist: Some(&["shyph"]),
            allow_background_color: false,
            allow_property_registration: false,
            allow_border_image_initial_resets: true,
            allow_visibility_collapse: false,
        },
    )
}

pub(super) fn first_unsupported_runtime_registered_properties_author_feature(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
    allow_background_color: bool,
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_rules(
        registry,
        allowed_properties,
        AuthorFeaturePolicy {
            allow_layers: false,
            allow_media: false,
            allow_viewport_dimensions: false,
            allow_custom_properties: true,
            custom_property_allowlist: None,
            allow_background_color,
            allow_property_registration: true,
            allow_border_image_initial_resets: true,
            allow_visibility_collapse: false,
        },
    )
}

pub(super) fn first_unsupported_runtime_layered_registered_properties_author_feature(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
    allow_background_color: bool,
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_rules(
        registry,
        allowed_properties,
        AuthorFeaturePolicy {
            allow_layers: true,
            allow_media: false,
            allow_viewport_dimensions: false,
            allow_custom_properties: true,
            custom_property_allowlist: None,
            allow_background_color,
            allow_property_registration: true,
            allow_border_image_initial_resets: true,
            allow_visibility_collapse: false,
        },
    )
}

fn first_unsupported_author_feature_with_layer_rules(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
    allow_layers: bool,
    allow_visibility_collapse: bool,
) -> Option<(String, String)> {
    first_unsupported_author_feature_with_rules(
        registry,
        allowed_properties,
        AuthorFeaturePolicy {
            allow_layers,
            allow_media: false,
            allow_viewport_dimensions: false,
            allow_custom_properties: false,
            custom_property_allowlist: None,
            allow_background_color: false,
            allow_property_registration: false,
            allow_border_image_initial_resets: false,
            allow_visibility_collapse,
        },
    )
}

fn first_unsupported_author_feature_with_rules(
    registry: &StylesheetRegistry,
    allowed_properties: &[&str],
    policy: AuthorFeaturePolicy,
) -> Option<(String, String)> {
    let guard = registry.shared_lock.read();
    for stylesheet in registry
        .stylesheets
        .iter()
        .filter(|stylesheet| stylesheet.origin == CssOrigin::Author)
    {
        // Stylo 0.22 exposes these typed parser events through stable diagnostic prefixes.
        if policy.allow_media
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
            if let Some(feature) =
                unsupported_rule_feature(rule, &guard, allowed_properties, policy)
            {
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

pub(crate) fn is_invalid_fixed_px_gap_declaration_diagnostic(message: &str) -> bool {
    let Some(declaration) = message
        .strip_prefix("Unsupported property declaration: '")
        .and_then(|message| message.strip_suffix("', OtherInvalidValue"))
    else {
        return false;
    };
    let Some(declaration) = remove_css_comments(declaration) else {
        return false;
    };
    let Some((property, value)) = declaration.split_once(':') else {
        return false;
    };
    if !["gap", "row-gap", "column-gap"]
        .iter()
        .any(|supported| property.trim().eq_ignore_ascii_case(supported))
    {
        return false;
    }

    let mut components = value
        .trim()
        .trim_end_matches(';')
        .split_whitespace()
        .collect::<Vec<_>>();
    if components
        .last()
        .is_some_and(|component| component.eq_ignore_ascii_case("!important"))
    {
        components.pop();
    } else if components.len() >= 2
        && components[components.len() - 2] == "!"
        && components
            .last()
            .is_some_and(|component| component.eq_ignore_ascii_case("important"))
    {
        components.truncate(components.len() - 2);
    }
    if !(1..=2).contains(&components.len()) {
        return false;
    }

    let Some(values) = components
        .into_iter()
        .map(parse_px_component)
        .collect::<Option<Vec<_>>>()
    else {
        return false;
    };
    values.into_iter().any(|value| value < 0.0)
}

pub(crate) fn is_invalid_root_html_named_grid_declaration_diagnostic(message: &str) -> bool {
    let Some(declaration) = message
        .strip_prefix("Unsupported property declaration: '")
        .and_then(|message| message.strip_suffix("', OtherInvalidValue"))
    else {
        return false;
    };
    let Some(declaration) = remove_css_comments(declaration) else {
        return false;
    };
    let Some((property, _)) = declaration.split_once(':') else {
        return false;
    };
    [
        "grid-template-areas",
        "grid-area",
        "grid-row",
        "grid-column",
        "grid-row-start",
        "grid-row-end",
        "grid-column-start",
        "grid-column-end",
    ]
    .iter()
    .any(|supported| property.trim().eq_ignore_ascii_case(supported))
}

fn remove_css_comments(input: &str) -> Option<String> {
    let mut remaining = input;
    let mut output = String::with_capacity(input.len());
    while let Some(comment_start) = remaining.find("/*") {
        output.push_str(&remaining[..comment_start]);
        let comment_body = &remaining[comment_start + 2..];
        let comment_end = comment_body.find("*/")?;
        output.push(' ');
        remaining = &comment_body[comment_end + 2..];
    }
    output.push_str(remaining);
    Some(output)
}

fn parse_px_component(value: &str) -> Option<f32> {
    let value = value.trim_end_matches(';');
    let unit_start = value.len().checked_sub(2)?;
    let (number, unit) = value.split_at(unit_start);
    if !unit.eq_ignore_ascii_case("px") {
        return None;
    }
    number
        .parse::<f32>()
        .ok()
        .filter(|number| number.is_finite())
}

fn unsupported_rule_feature(
    rule: &CssRule,
    guard: &SharedRwLockReadGuard<'_>,
    allowed_properties: &[&str],
    policy: AuthorFeaturePolicy,
) -> Option<String> {
    match rule {
        CssRule::LayerStatement(_) if policy.allow_layers => None,
        CssRule::LayerBlock(layer) if policy.allow_layers => {
            let rules = layer.rules.read_with(guard);
            rules.0.iter().find_map(|nested| {
                unsupported_rule_feature(nested, guard, allowed_properties, policy)
            })
        }
        CssRule::Media(media) if policy.allow_media => {
            let media_queries = media.media_queries.read_with(guard);
            let serialized = media_queries.to_css_string();
            let supported = if policy.allow_viewport_dimensions {
                supported_root_html_media_query_surface(&serialized)
            } else {
                supported_media_query_surface(&serialized)
            };
            if !supported {
                return Some("지원 범위 밖 media query feature".to_owned());
            }
            let rules = media.rules.read_with(guard);
            rules.0.iter().find_map(|nested| {
                unsupported_rule_feature(nested, guard, allowed_properties, policy)
            })
        }
        CssRule::Style(rule) => {
            let rule = rule.read_with(guard);
            if rule.rules.is_some() {
                return Some("중첩 CSS 규칙".to_owned());
            }
            let block = rule.block.read_with(guard);
            for declaration in block.declarations() {
                let property = declaration.id();
                let name = match property {
                    PropertyDeclarationId::Longhand(id) => id.name().to_owned(),
                    PropertyDeclarationId::Custom(_) => "사용자 지정 속성".to_owned(),
                };
                let allowed = match property {
                    PropertyDeclarationId::Custom(name) => {
                        policy.allow_custom_properties
                            && policy
                                .custom_property_allowlist
                                .is_none_or(|allowed| allowed.contains(&name.as_ref()))
                    }
                    PropertyDeclarationId::Longhand(_) => {
                        allowed_properties.contains(&name.as_str())
                            || (policy.allow_background_color && name == "background-color")
                    }
                };
                let unsupported_visibility_value = name == "visibility"
                    && allowed_properties.contains(&"visibility")
                    && !supports_runtime_visibility_value(declaration)
                    && !(policy.allow_visibility_collapse
                        && supports_runtime_visibility_collapse_value(declaration));
                if !(allowed
                    || (policy.allow_border_image_initial_resets
                        && is_border_image_initial(declaration, &name)))
                    || unsupported_visibility_value
                {
                    if unsupported_visibility_value {
                        return Some("지원하지 않는 CSS 선언 visibility 값".to_owned());
                    }
                    return Some(format!("지원하지 않는 CSS 선언 {name}"));
                }
            }
            None
        }
        CssRule::Property(_) if policy.allow_property_registration => None,
        _ => Some("at-rule 또는 비스타일 규칙".to_owned()),
    }
}

pub(crate) fn is_border_image_initial(declaration: &PropertyDeclaration, name: &str) -> bool {
    let initial = match name {
        "border-image-outset" => "0",
        "border-image-repeat" => "stretch",
        "border-image-slice" => "100%",
        "border-image-source" => "none",
        "border-image-width" => "1",
        _ => return false,
    };
    let mut serialized = String::new();
    declaration.to_css(&mut serialized).unwrap();
    serialized == initial
}

fn supported_media_query_surface(source: &str) -> bool {
    supported_media_query_surface_with_viewport_dimensions(source, false)
}

fn supported_root_html_media_query_surface(source: &str) -> bool {
    #[derive(Clone, Copy)]
    enum QueryState {
        Start,
        AfterNot,
        AfterOnly,
        AfterType,
        NeedFeature,
        AfterFeature,
    }

    let mut parser = cssparser::Parser::new(source);
    let mut state = QueryState::Start;
    while !parser.is_exhausted() {
        let token = match parser.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(_) => return false,
        };
        match token {
            cssparser::Token::WhiteSpace(_) | cssparser::Token::Comment(_) => {}
            cssparser::Token::Comma
                if matches!(state, QueryState::AfterType | QueryState::AfterFeature) =>
            {
                state = QueryState::Start;
            }
            cssparser::Token::Ident(value)
                if value.eq_ignore_ascii_case("not") && matches!(state, QueryState::Start) =>
            {
                state = QueryState::AfterNot;
            }
            cssparser::Token::Ident(value)
                if value.eq_ignore_ascii_case("only") && matches!(state, QueryState::Start) =>
            {
                state = QueryState::AfterOnly;
            }
            cssparser::Token::Ident(value)
                if (value.eq_ignore_ascii_case("screen") || value.eq_ignore_ascii_case("all"))
                    && matches!(
                        state,
                        QueryState::Start | QueryState::AfterNot | QueryState::AfterOnly
                    ) =>
            {
                state = QueryState::AfterType;
            }
            cssparser::Token::Ident(value)
                if value.eq_ignore_ascii_case("and")
                    && matches!(state, QueryState::AfterType | QueryState::AfterFeature) =>
            {
                state = QueryState::NeedFeature;
            }
            cssparser::Token::ParenthesisBlock
                if matches!(
                    state,
                    QueryState::Start | QueryState::AfterNot | QueryState::NeedFeature
                ) =>
            {
                if parser
                    .parse_nested_block(|nested| supported_media_query_block(nested, true))
                    .is_err()
                {
                    return false;
                }
                state = QueryState::AfterFeature;
            }
            _ => return false,
        }
    }
    matches!(state, QueryState::AfterType | QueryState::AfterFeature)
}

fn supported_media_query_surface_with_viewport_dimensions(
    source: &str,
    allow_viewport_dimensions: bool,
) -> bool {
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
                    .parse_nested_block(|nested| {
                        supported_media_query_block(nested, allow_viewport_dimensions)
                    })
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
    allow_viewport_dimensions: bool,
) -> Result<(), cssparser::ParseError<()>> {
    let mut feature = None;
    let mut identifier_value = None;
    let mut pixel_value = None;
    let mut saw_colon = false;
    while !parser.is_exhausted() {
        let token = parser.next_including_whitespace_and_comments()?.clone();
        match token {
            cssparser::Token::Ident(ident) if !saw_colon && feature.is_none() => {
                feature = Some(ident.to_ascii_lowercase());
            }
            cssparser::Token::Ident(ident)
                if saw_colon && identifier_value.is_none() && pixel_value.is_none() =>
            {
                identifier_value = Some(ident.to_ascii_lowercase());
            }
            cssparser::Token::Dimension { value, unit, .. }
                if saw_colon && identifier_value.is_none() && pixel_value.is_none() =>
            {
                if !unit.eq_ignore_ascii_case("px") || !value.is_finite() || value < 0.0 {
                    return Err(cssparser::ParseError::custom(()));
                }
                pixel_value = Some(value);
            }
            cssparser::Token::Number { value, .. }
                if saw_colon
                    && identifier_value.is_none()
                    && pixel_value.is_none()
                    && value == 0.0 =>
            {
                pixel_value = Some(value);
            }
            cssparser::Token::WhiteSpace(_) | cssparser::Token::Comment(_) => {}
            cssparser::Token::Colon if feature.is_some() && !saw_colon => saw_colon = true,
            _ => return Err(cssparser::ParseError::custom(())),
        }
    }
    match (feature.as_deref(), identifier_value.as_deref(), pixel_value) {
        (Some("prefers-color-scheme"), Some("light" | "dark"), None)
        | (Some("pointer" | "any-pointer"), Some("none" | "coarse" | "fine"), None)
        | (Some("hover" | "any-hover"), Some("none" | "hover"), None) => Ok(()),
        (
            Some("width" | "min-width" | "max-width" | "height" | "min-height" | "max-height"),
            None,
            Some(_),
        ) if allow_viewport_dimensions => Ok(()),
        _ => Err(cssparser::ParseError::custom(())),
    }
}

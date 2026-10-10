use std::collections::{BTreeMap, BTreeSet, HashMap};

use cssparser::{Parser, Token};
use style::{
    context::QuirksMode,
    custom_properties::ComputedCustomProperties,
    parser::{Parse, ParserContext},
    shared_lock::SharedRwLock,
    stylesheets::{Origin, UrlExtraData},
    values::specified::BorderSideWidth,
};
use style_traits::ParsingMode;
use url::Url;

use crate::stylesheet_registry::parse_inline_style_attribute;

use super::super::border_width_property;

pub(super) fn recovered_border_widths(
    source: &str,
    custom_properties: &ComputedCustomProperties,
    quirks_mode: QuirksMode,
    base_url: &Url,
    shared_lock: &SharedRwLock,
) -> BTreeMap<(&'static str, bool), String> {
    let mut declarations = Vec::new();
    let mut raw_width_candidates = HashMap::<(&'static str, bool), Vec<String>>::new();
    for (name, value) in inline_declarations(source) {
        if !name.starts_with("border") {
            continue;
        }
        let mut budget = 256;
        let mut resolving = BTreeSet::new();
        let Some(value) =
            super::resolve_variables(&value, custom_properties, &mut resolving, 0, &mut budget)
        else {
            continue;
        };
        let css = format!("{name}:{value}");
        let (single_block, _) =
            parse_inline_style_attribute(&css, base_url, shared_lock, quirks_mode);
        {
            let guard = shared_lock.read();
            let single_block = single_block.read_with(&guard);
            for (declaration, importance) in single_block.declaration_importance_iter() {
                let Some(property) = border_width_property(declaration) else {
                    continue;
                };
                let candidate =
                    raw_width_value_for_declaration(&name, &value, property, base_url, quirks_mode)
                        .or_else(|| {
                            let mut serialized = String::new();
                            declaration
                                .to_css(&mut serialized)
                                .is_ok()
                                .then_some(serialized)
                                .filter(|serialized| !serialized.is_empty())
                        });
                if let Some(candidate) = candidate {
                    raw_width_candidates
                        .entry((property, importance.important()))
                        .or_default()
                        .push(candidate);
                }
            }
        }
        declarations.push(css);
    }
    if declarations.is_empty() {
        return BTreeMap::new();
    }

    let css = declarations.join(";");
    let (block, _) = parse_inline_style_attribute(&css, base_url, shared_lock, quirks_mode);
    let guard = shared_lock.read();
    let block = block.read_with(&guard);
    let mut seen = BTreeSet::new();
    let mut output = BTreeMap::new();
    for (declaration, importance) in block
        .declaration_importance_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        let Some(property) = border_width_property(declaration) else {
            continue;
        };
        let important = importance.important();
        if !seen.insert((property, important)) {
            continue;
        }
        let key = (property, important);
        let value = raw_width_candidates
            .get_mut(&key)
            .and_then(Vec::pop)
            .or_else(|| {
                let mut serialized = String::new();
                declaration
                    .to_css(&mut serialized)
                    .is_ok()
                    .then_some(serialized)
                    .filter(|serialized| !serialized.is_empty())
            });
        if let Some(value) = value {
            output.insert(key, value);
        }
    }
    output
}

fn raw_width_value_for_declaration(
    name: &str,
    value: &str,
    property: &'static str,
    base_url: &Url,
    quirks_mode: QuirksMode,
) -> Option<String> {
    let mut components = top_level_components(value)?;
    if components.len() >= 2
        && components[components.len() - 2] == "!"
        && components[components.len() - 1].eq_ignore_ascii_case("important")
    {
        components.truncate(components.len() - 2);
    }

    if name == "border-width" {
        if !(1..=4).contains(&components.len())
            || !components
                .iter()
                .all(|component| is_border_width_component(component, base_url, quirks_mode))
        {
            return None;
        }
        let count = components.len();
        let index = match property {
            "border-top-width" => 0,
            "border-right-width" => {
                if count == 1 {
                    0
                } else {
                    1
                }
            }
            "border-bottom-width" => {
                if count < 3 {
                    0
                } else {
                    2
                }
            }
            "border-left-width" => match count {
                1 => 0,
                2 | 3 => 1,
                _ => 3,
            },
            _ => return None,
        };
        return components.get(index).cloned();
    }

    if matches!(
        name,
        "border-top-width" | "border-right-width" | "border-bottom-width" | "border-left-width"
    ) {
        return (components.len() == 1
            && is_border_width_component(&components[0], base_url, quirks_mode))
        .then(|| components[0].clone());
    }

    if matches!(
        name,
        "border" | "border-top" | "border-right" | "border-bottom" | "border-left"
    ) {
        return components
            .into_iter()
            .find(|component| is_border_width_component(component, base_url, quirks_mode));
    }

    None
}

fn top_level_components(source: &str) -> Option<Vec<String>> {
    let mut parser = Parser::new(source);
    let mut components = Vec::new();
    loop {
        parser.skip_whitespace();
        if parser.is_exhausted() {
            break;
        }
        let start = parser.position();
        let token = parser.next_including_whitespace_and_comments().ok()?;
        if matches!(token, Token::WhiteSpace(_) | Token::Comment(_)) {
            continue;
        }
        if has_nested_block(token) {
            consume_nested_block(&mut parser).ok()?;
        }
        let component = parser.slice(start..parser.position()).trim();
        if !component.is_empty() {
            components.push(component.to_owned());
        }
    }
    Some(components)
}

fn is_border_width_component(source: &str, base_url: &Url, quirks_mode: QuirksMode) -> bool {
    let url_data = UrlExtraData::from(base_url.clone());
    let context = ParserContext::new(
        Origin::Author,
        &url_data,
        None,
        ParsingMode::DEFAULT,
        quirks_mode,
        Default::default(),
        None,
        None,
        Default::default(),
    );
    let mut parser = Parser::new(source);
    parser
        .parse_entirely(|parser| BorderSideWidth::parse(&context, parser))
        .is_ok()
}

fn inline_declarations(source: &str) -> Vec<(String, String)> {
    let mut parser = Parser::new(source);
    let mut declarations = Vec::new();
    loop {
        parser.skip_whitespace();
        let start = parser.state();
        if matches!(
            parser.next_including_whitespace_and_comments(),
            Ok(Token::Comment(_))
        ) {
            continue;
        }
        parser.reset(&start);
        let name = match parser.expect_ident() {
            Ok(name) => name.to_ascii_lowercase(),
            Err(_) => {
                skip_inline_declaration(&mut parser);
                if parser.is_exhausted() {
                    break;
                }
                continue;
            }
        };
        if parser.expect_colon().is_err() {
            skip_inline_declaration(&mut parser);
            if parser.is_exhausted() {
                break;
            }
            continue;
        }
        let value_start = parser.position();
        let (value_end, consumed_semicolon) = loop {
            let token_start = parser.position();
            match parser.next_including_whitespace_and_comments() {
                Ok(Token::Semicolon) => break (token_start, true),
                Ok(token) if has_nested_block(token) => {
                    if consume_nested_block(&mut parser).is_err() {
                        break (parser.position(), false);
                    }
                }
                Ok(_) => {}
                Err(_) => break (parser.position(), false),
            }
        };
        let value = parser.slice(value_start..value_end).trim().to_owned();
        if !value.is_empty() {
            declarations.push((name, value));
        }
        if !consumed_semicolon || parser.is_exhausted() {
            break;
        }
    }
    declarations
}

fn skip_inline_declaration(parser: &mut Parser<'_>) {
    loop {
        match parser.next_including_whitespace_and_comments() {
            Ok(Token::Semicolon) | Err(_) => break,
            Ok(token) if has_nested_block(token) => {
                if consume_nested_block(parser).is_err() {
                    break;
                }
            }
            Ok(_) => {}
        }
    }
}

fn has_nested_block(token: &Token<'_>) -> bool {
    matches!(
        token,
        Token::Function(_)
            | Token::ParenthesisBlock
            | Token::SquareBracketBlock
            | Token::CurlyBracketBlock
    )
}

fn consume_nested_block(parser: &mut Parser<'_>) -> Result<(), cssparser::ParseError<()>> {
    parser.parse_nested_block(|block| {
        while block.next_including_whitespace_and_comments().is_ok() {}
        Ok(())
    })
}

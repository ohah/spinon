use std::collections::{BTreeMap, BTreeSet};

use cssparser::{Parser, Token};
use style::{
    Atom,
    context::QuirksMode,
    custom_properties::{ComputedCustomProperties, Name},
    parser::{Parse, ParserContext},
    properties::ComputedValues,
    shared_lock::SharedRwLock,
    stylesheets::{Origin, UrlExtraData},
    typed_om::{ToTyped, TypedValue},
    values::specified::BorderSideWidth,
};
use style_traits::ParsingMode;
use url::Url;

use super::{
    BorderWidthSources, CssLengthContext,
    numeric::{CssNumeric, evaluate_css_length},
};
use recovery::recovered_border_widths;

#[path = "values/recovery.rs"]
mod recovery;

pub fn computed_border_width_values(
    sources: &BorderWidthSources,
    computed: &ComputedValues,
    quirks_mode: QuirksMode,
    inline_style_text: Option<&str>,
    base_url: &Url,
    shared_lock: &SharedRwLock,
    length_context: CssLengthContext,
) -> BTreeMap<&'static str, f32> {
    let custom_properties = computed.custom_properties();
    let mut recovered_sources = BTreeMap::new();
    sources
        .iter()
        .filter_map(|(property, source)| {
            let raw = if source.from_inline_style {
                inline_style_text
            } else {
                source.raw_declarations.as_deref()
            };
            let recovered = raw.and_then(|raw| {
                let declarations = recovered_sources.entry(raw.to_owned()).or_insert_with(|| {
                    recovered_border_widths(
                        raw,
                        custom_properties,
                        quirks_mode,
                        base_url,
                        shared_lock,
                    )
                });
                declarations
                    .get(&(*property, source.important))
                    .map(String::as_str)
            });
            let css = recovered.or(source.css.as_deref())?;
            let width = specified_css_px(css, custom_properties, quirks_mode, length_context);
            width.map(|width| (*property, width))
        })
        .collect()
}

fn specified_css_px(
    source: &str,
    custom_properties: &ComputedCustomProperties,
    quirks_mode: QuirksMode,
    length_context: CssLengthContext,
) -> Option<f32> {
    let mut budget = 256;
    let mut resolving = BTreeSet::new();
    let resolved = resolve_variables(source, custom_properties, &mut resolving, 0, &mut budget)?;
    if resolved.is_empty() || resolved.len() > 2 * 1024 * 1024 {
        return None;
    }

    let url_data = UrlExtraData::from(Url::parse("https://spinon.invalid/").ok()?);
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
    let mut parser = Parser::new(&resolved);
    let width = parser
        .parse_entirely(|parser| BorderSideWidth::parse(&context, parser))
        .ok()?;

    match width.to_typed_value()? {
        TypedValue::Keyword(keyword) => match &*keyword.0 {
            "thin" => Some(1.0),
            "medium" => Some(3.0),
            "thick" => Some(5.0),
            _ => None,
        },
        TypedValue::Numeric(value) => {
            match evaluate_css_length(&value, length_context, 0, &mut 256)? {
                CssNumeric::Length(px) if px.is_finite() && px >= 0.0 => Some(px as f32),
                _ => None,
            }
        }
        _ => None,
    }
}

fn resolve_variables(
    source: &str,
    custom_properties: &ComputedCustomProperties,
    resolving: &mut BTreeSet<String>,
    depth: usize,
    budget: &mut usize,
) -> Option<String> {
    if depth > 32 || source.len() > 2 * 1024 * 1024 || *budget == 0 {
        return None;
    }
    *budget -= 1;

    let mut parser = Parser::new(source);
    let mut output = String::with_capacity(source.len());
    let mut copied_until = parser.position();

    loop {
        let token_start = parser.position();
        let Ok(token) = parser.next_including_whitespace_and_comments() else {
            break;
        };
        let (close, is_var) = match token {
            Token::Function(name) => (")", name.eq_ignore_ascii_case("var")),
            Token::ParenthesisBlock => (")", false),
            Token::SquareBracketBlock => ("]", false),
            Token::CurlyBracketBlock => ("}", false),
            _ => continue,
        };
        let body_start = parser.position();
        let body = parser
            .parse_nested_block(|block| {
                let start = block.position();
                while block.next_including_whitespace_and_comments().is_ok() {}
                let end = block.position();
                Ok::<_, cssparser::ParseError<()>>(block.slice(start..end).to_owned())
            })
            .ok()?;
        let block_end = parser.position();

        output.push_str(parser.slice(copied_until..token_start));
        if is_var {
            output.push_str(&resolve_var_body(
                &body,
                custom_properties,
                resolving,
                depth + 1,
                budget,
            )?);
        } else {
            output.push_str(parser.slice(token_start..body_start));
            output.push_str(&resolve_variables(
                &body,
                custom_properties,
                resolving,
                depth + 1,
                budget,
            )?);
            output.push_str(close);
        }
        copied_until = block_end;
    }

    output.push_str(parser.slice_from(copied_until));
    Some(output)
}

fn resolve_var_body(
    body: &str,
    custom_properties: &ComputedCustomProperties,
    resolving: &mut BTreeSet<String>,
    depth: usize,
    budget: &mut usize,
) -> Option<String> {
    let mut parser = Parser::new(body);
    let body_start = parser.position();
    let mut comma_position = None;
    loop {
        let token_start = parser.position();
        let Ok(token) = parser.next_including_whitespace_and_comments() else {
            break;
        };
        if matches!(token, Token::Comma) && comma_position.is_none() {
            comma_position = Some((token_start, parser.position()));
        }
    }

    let (name_source, fallback) = match comma_position {
        Some((comma_start, comma_end)) => (
            parser.slice(body_start..comma_start).trim(),
            Some(parser.slice(comma_end..parser.position()).to_owned()),
        ),
        None => (body.trim(), None),
    };
    let name = custom_property_name(name_source);
    if let Some(name) = name {
        let name = name.to_owned();
        if resolving.insert(name.clone()) {
            let property_name = Name::from(Atom::from(name.as_str()));
            let value = custom_properties
                .get_for_cssom(&property_name)
                .map(|value| value.to_variable_value().css_text().to_owned());
            if let Some(value) = value {
                let resolved =
                    resolve_variables(&value, custom_properties, resolving, depth + 1, budget);
                resolving.remove(&name);
                if let Some(resolved) = resolved {
                    return Some(resolved);
                }
            } else {
                resolving.remove(&name);
            }
        }
    }

    fallback.and_then(|fallback| {
        resolve_variables(&fallback, custom_properties, resolving, depth + 1, budget)
    })
}

fn custom_property_name(source: &str) -> Option<String> {
    let mut parser = Parser::new(source);
    let name = parser.expect_ident().ok()?.to_string();
    parser.expect_exhausted().ok()?;
    name.strip_prefix("--")
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
}

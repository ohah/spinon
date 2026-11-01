use std::ops::Range;

use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser, Token,
};

const MAX_NESTED_RULE_DEPTH: usize = 64;
const RESERVED_ALIAS: &str = "--shyph";

/// Rewrites `hyphens` declarations to a Stylo-cascaded custom property without moving source
/// offsets. Unsupported alias collisions and dynamic values fail closed for the fixture profile.
pub(crate) fn normalize_css_hyphens(source: &str, stylesheet: bool) -> Result<String, String> {
    let mut scanner = HyphensScanner {
        source,
        ..HyphensScanner::default()
    };
    if stylesheet {
        let mut input = Parser::new(source);
        for _ in StyleSheetParser::new(&mut input, &mut scanner) {}
    } else {
        let mut input = Parser::new(source);
        for _ in RuleBodyParser::new(&mut input, &mut scanner) {}
    }

    if let Some(error) = scanner.error {
        return Err(error);
    }
    if scanner.exceeded_depth {
        return Err("C11.4 hyphens CSS 중첩 깊이가 제한을 넘었습니다".to_owned());
    }

    let mut replacements = scanner
        .property_ranges
        .into_iter()
        .map(|range| (range, RESERVED_ALIAS.as_bytes().to_vec()))
        .chain(scanner.initial_ranges.into_iter().map(|range| {
            let replacement = padded_manual_value(range.end - range.start);
            (range, replacement)
        }))
        .collect::<Vec<_>>();
    replacements.sort_by_key(|(range, _)| range.start);

    let mut bytes = source.as_bytes().to_vec();
    for (range, replacement) in replacements {
        if range.end > bytes.len() || range.end - range.start != replacement.len() {
            return Err(
                "C11.4 hyphens CSS adapter가 원본 byte 범위를 보존하지 못했습니다".to_owned(),
            );
        }
        bytes[range.clone()].copy_from_slice(&replacement);
    }
    String::from_utf8(bytes)
        .map_err(|_| "C11.4 hyphens CSS adapter 결과가 UTF-8이 아닙니다".to_owned())
}

fn padded_manual_value(length: usize) -> Vec<u8> {
    let mut value = vec![b' '; length];
    if length >= b"manual".len() {
        value[..b"manual".len()].copy_from_slice(b"manual");
    }
    value
}

#[derive(Default)]
struct HyphensScanner<'s> {
    source: &'s str,
    property_ranges: Vec<Range<usize>>,
    initial_ranges: Vec<Range<usize>>,
    nested_rule_depth: usize,
    exceeded_depth: bool,
    error: Option<String>,
}

impl<'i> HyphensScanner<'i> {
    fn scan_nested_rule_list(&mut self, input: &mut Parser<'i>) -> Result<(), ParseError<()>> {
        if self.nested_rule_depth >= MAX_NESTED_RULE_DEPTH {
            self.exceeded_depth = true;
            return drain_component_values(input);
        }
        self.nested_rule_depth += 1;
        for _ in StyleSheetParser::new(input, self) {}
        self.nested_rule_depth -= 1;
        Ok(())
    }

    fn scan_rule_body(&mut self, input: &mut Parser<'i>) -> Result<(), ParseError<()>> {
        if self.nested_rule_depth >= MAX_NESTED_RULE_DEPTH {
            self.exceeded_depth = true;
            return drain_component_values(input);
        }
        self.nested_rule_depth += 1;
        for _ in RuleBodyParser::new(input, self) {}
        self.nested_rule_depth -= 1;
        Ok(())
    }

    fn reject(&mut self, reason: &str) {
        if self.error.is_none() {
            self.error = Some(reason.to_owned());
        }
    }
}

impl<'i> DeclarationParser<'i> for HyphensScanner<'i> {
    type Declaration = ();
    type Error = ();

    fn parse_value(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i>,
        declaration_start: &ParserState,
    ) -> Result<Self::Declaration, ParseError<Self::Error>> {
        if name == RESERVED_ALIAS {
            self.reject("C11.4 예약 custom property --shyph와 작성자 속성이 충돌합니다");
            drain_component_values(input)?;
            return Ok(());
        }
        if name.eq_ignore_ascii_case("all") {
            self.reject("C11.4 hyphens 제한 profile은 all shorthand를 거부합니다");
            drain_component_values(input)?;
            return Ok(());
        }
        if !name.eq_ignore_ascii_case("hyphens") {
            drain_component_values(input)?;
            return Ok(());
        }

        let Some(property_range) = raw_declaration_name_range(self.source, declaration_start)
        else {
            self.reject("C11.4 hyphens property 원문 범위를 확인하지 못했습니다");
            drain_component_values(input)?;
            return Ok(());
        };
        if self.source[property_range.clone()].eq_ignore_ascii_case("hyphens") {
            self.property_ranges.push(property_range);
        } else {
            self.reject("C11.4 hyphens escaped property 이름은 제한 profile에서 지원하지 않습니다");
        }

        let mut value_token_count = 0usize;
        let mut initial_range = None;
        let mut has_dynamic_value = false;
        while !input.is_exhausted() {
            input.skip_whitespace();
            if input.is_exhausted() {
                break;
            }
            if input.try_parse(cssparser::parse_important).is_ok() {
                continue;
            }
            let start = input.position().byte_index();
            match input.next_including_whitespace_and_comments()?.clone() {
                Token::WhiteSpace(_) | Token::Comment(_) => {}
                Token::Ident(value) => {
                    value_token_count += 1;
                    initial_range = value
                        .eq_ignore_ascii_case("initial")
                        .then(|| start..input.position().byte_index());
                }
                Token::Function(_)
                | Token::ParenthesisBlock
                | Token::CurlyBracketBlock
                | Token::SquareBracketBlock => {
                    input.parse_nested_block(drain_component_values)?;
                    value_token_count += 1;
                    has_dynamic_value = true;
                    initial_range = None;
                }
                _ => {
                    value_token_count += 1;
                    initial_range = None;
                }
            }
        }

        if has_dynamic_value {
            self.reject("C11.4 hyphens 값의 함수·중첩 token은 제한 profile에서 지원하지 않습니다");
        } else if value_token_count == 1
            && let Some(range) = initial_range
        {
            self.initial_ranges.push(range);
        }
        Ok(())
    }
}

impl<'i> AtRuleParser<'i> for HyphensScanner<'i> {
    type Prelude = bool;
    type AtRule = ();
    type Error = ();

    fn parse_prelude(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i>,
    ) -> Result<Self::Prelude, ParseError<()>> {
        let is_property = name.eq_ignore_ascii_case("property");
        if is_property
            && let Ok(Token::Ident(property)) = input.next()
            && property.as_ref() == RESERVED_ALIAS
        {
            self.reject("C11.4 예약 custom property --shyph의 @property 등록은 지원하지 않습니다");
        }
        drain_component_values(input)?;
        Ok(is_property)
    }

    fn parse_block(
        &mut self,
        is_property: Self::Prelude,
        _: &ParserState,
        input: &mut Parser<'i>,
    ) -> Result<Self::AtRule, ParseError<()>> {
        if is_property {
            drain_component_values(input)
        } else {
            self.scan_nested_rule_list(input)
        }
    }
}

impl<'i> QualifiedRuleParser<'i> for HyphensScanner<'i> {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = ();

    fn parse_prelude(&mut self, input: &mut Parser<'i>) -> Result<Self::Prelude, ParseError<()>> {
        drain_component_values(input)?;
        Ok(())
    }

    fn parse_block(
        &mut self,
        _: Self::Prelude,
        _: &ParserState,
        input: &mut Parser<'i>,
    ) -> Result<Self::QualifiedRule, ParseError<()>> {
        self.scan_rule_body(input)
    }
}

impl<'i> RuleBodyItemParser<'i, (), ()> for HyphensScanner<'i> {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        true
    }
}

fn raw_declaration_name_range(
    source: &str,
    declaration_start: &ParserState,
) -> Option<Range<usize>> {
    let start = declaration_start.position().byte_index();
    let mut input = Parser::new(source.get(start..)?);
    match input.next().ok()? {
        Token::Ident(_) => Some(start..start.checked_add(input.position().byte_index())?),
        _ => None,
    }
}

fn drain_component_values(input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
    while !input.is_exhausted() {
        match input.next_including_whitespace_and_comments()? {
            Token::Function(_)
            | Token::ParenthesisBlock
            | Token::CurlyBracketBlock
            | Token::SquareBracketBlock => input.parse_nested_block(drain_component_values)?,
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::normalize_css_hyphens;

    #[test]
    fn rewrites_hyphens_and_initial_without_moving_source_offsets() {
        let source = "color:red;hyphens: initial !important; width:10px";
        let output = normalize_css_hyphens(source, false).unwrap();
        assert_eq!(output, "color:red;--shyph: manual  !important; width:10px");
        assert_eq!(source.len(), output.len());
    }

    #[test]
    fn rewrites_nested_stylesheet_declarations_but_not_strings() {
        let source = concat!(
            ".a { content:'hyphens: none'; hyphens: manual; }",
            "@media screen { .b { hyphens: initial; } }",
        );
        let output = normalize_css_hyphens(source, true).unwrap();
        assert!(output.contains("content:'hyphens: none'"));
        assert!(output.contains("--shyph: manual"));
        assert!(output.contains("--shyph: manual "));
        assert_eq!(source.len(), output.len());
    }

    #[test]
    fn rejects_reserved_alias_collision_all_and_alias_registration() {
        for source in [
            "--shyph: none",
            ".a { --shyph: manual; }",
            "all: initial",
            "@media screen { .a { all: unset; } }",
            "@property --shyph { syntax: '<custom-ident>'; inherits: true; }",
        ] {
            assert!(
                normalize_css_hyphens(
                    source,
                    !source.starts_with("--") && !source.starts_with("all:")
                )
                .is_err(),
                "{source:?}"
            );
        }
    }

    #[test]
    fn rejects_dynamic_hyphens_values_but_preserves_invalid_static_keywords_for_cascade_check() {
        assert!(normalize_css_hyphens("hyphens: var(--mode)", false).is_err());
        let output = normalize_css_hyphens("hyphens: balanced", false).unwrap();
        assert_eq!(output, "--shyph: balanced");
    }
}

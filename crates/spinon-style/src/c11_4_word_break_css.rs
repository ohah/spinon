use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser, Token,
};

const MAX_NESTED_RULE_DEPTH: usize = 64;

/// Lets Servo Stylo parse the legacy `word-break: break-word` alias as `normal`.
///
/// The winning declaration is recovered from the original stylesheet during cascade output, so
/// this same-length parser input rewrite does not lose the computed alias exposed to callers.
pub(crate) fn normalize_css_word_break_break_word(source: &str, stylesheet: bool) -> String {
    let mut scanner = WordBreakScanner::default();
    if stylesheet {
        let mut input = Parser::new(source);
        for _ in StyleSheetParser::new(&mut input, &mut scanner) {}
    } else {
        let mut input = Parser::new(source);
        for _ in RuleBodyParser::new(&mut input, &mut scanner) {}
    }

    if scanner.exceeded_depth || scanner.ranges.is_empty() {
        return source.to_owned();
    }

    let mut bytes = source.as_bytes().to_vec();
    for (start, end) in scanner.ranges {
        let Some(length) = end.checked_sub(start) else {
            continue;
        };
        if length < b"normal".len() || end > bytes.len() {
            continue;
        }
        bytes[start..end].fill(b' ');
        bytes[start..start + b"normal".len()].copy_from_slice(b"normal");
    }
    String::from_utf8(bytes).unwrap_or_else(|_| source.to_owned())
}

#[derive(Default)]
struct WordBreakScanner {
    ranges: Vec<(usize, usize)>,
    nested_rule_depth: usize,
    exceeded_depth: bool,
}

impl WordBreakScanner {
    fn scan_nested_rule_list(&mut self, input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
        if self.nested_rule_depth >= MAX_NESTED_RULE_DEPTH {
            self.exceeded_depth = true;
            return drain_component_values(input);
        }

        self.nested_rule_depth += 1;
        for _ in StyleSheetParser::new(input, self) {}
        self.nested_rule_depth -= 1;
        Ok(())
    }

    fn scan_rule_body(&mut self, input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
        if self.nested_rule_depth >= MAX_NESTED_RULE_DEPTH {
            self.exceeded_depth = true;
            return drain_component_values(input);
        }

        self.nested_rule_depth += 1;
        for _ in RuleBodyParser::new(input, self) {}
        self.nested_rule_depth -= 1;
        Ok(())
    }
}

impl<'i> DeclarationParser<'i> for WordBreakScanner {
    type Declaration = ();
    type Error = ();

    fn parse_value(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i>,
        _: &ParserState,
    ) -> Result<Self::Declaration, ParseError<Self::Error>> {
        if !name.eq_ignore_ascii_case("word-break") {
            drain_component_values(input)?;
            return Ok(());
        }

        let mut alias_range = None;
        let mut value_token_count = 0usize;
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
                    alias_range = value
                        .eq_ignore_ascii_case("break-word")
                        .then(|| (start, input.position().byte_index()));
                }
                Token::Function(_)
                | Token::ParenthesisBlock
                | Token::CurlyBracketBlock
                | Token::SquareBracketBlock => {
                    input.parse_nested_block(drain_component_values)?;
                    value_token_count += 1;
                    alias_range = None;
                }
                _ => {
                    value_token_count += 1;
                    alias_range = None;
                }
            }
        }

        if value_token_count == 1
            && let Some(range) = alias_range
        {
            self.ranges.push(range);
        }
        Ok(())
    }
}

impl<'i> AtRuleParser<'i> for WordBreakScanner {
    type Prelude = ();
    type AtRule = ();
    type Error = ();

    fn parse_prelude(
        &mut self,
        _: CowRcStr<'i>,
        input: &mut Parser<'i>,
    ) -> Result<Self::Prelude, ParseError<()>> {
        drain_component_values(input)?;
        Ok(())
    }

    fn parse_block(
        &mut self,
        _: Self::Prelude,
        _: &ParserState,
        input: &mut Parser<'i>,
    ) -> Result<Self::AtRule, ParseError<()>> {
        self.scan_nested_rule_list(input)?;
        Ok(())
    }
}

impl<'i> QualifiedRuleParser<'i> for WordBreakScanner {
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
        self.scan_rule_body(input)?;
        Ok(())
    }
}

impl RuleBodyItemParser<'_, (), ()> for WordBreakScanner {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        true
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
    use super::normalize_css_word_break_break_word;

    #[test]
    fn inline_alias_is_rewritten_without_moving_following_source_offsets() {
        let source = "color:red;word-break: BREAK-WORD !important;overflow-wrap:anywhere";
        let output = normalize_css_word_break_break_word(source, false);
        assert_eq!(
            output,
            "color:red;word-break: normal     !important;overflow-wrap:anywhere"
        );
        assert_eq!(source.len(), output.len());
    }

    #[test]
    fn stylesheet_alias_is_rewritten_in_nested_rules_but_not_strings_or_other_properties() {
        let source = concat!(
            ".a { content:'word-break: break-word'; --x: break-word;",
            " word-break: break-word; width: 10px; }",
            "@media screen { .b { word-break: break-word; } }",
        );
        let output = normalize_css_word_break_break_word(source, true);
        assert!(output.contains("content:'word-break: break-word'"));
        assert!(output.contains("--x: break-word"));
        assert!(output.contains("word-break: normal    ; width: 10px"));
        assert!(output.contains("@media screen { .b { word-break: normal    ; } }"));
        assert_eq!(source.len(), output.len());
    }

    #[test]
    fn nonliteral_values_and_other_word_break_values_are_unchanged() {
        for source in [
            "word-break:normal",
            "word-break:break-all",
            "word-break:keep-all",
            "word-break:var(--break-mode)",
            "--word-break:break-word",
            "content:'break-word'",
        ] {
            assert_eq!(
                normalize_css_word_break_break_word(source, false),
                source,
                "{source}"
            );
        }
    }
}

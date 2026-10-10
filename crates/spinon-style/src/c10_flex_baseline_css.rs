use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser, Token,
};

const MAX_NESTED_RULE_DEPTH: usize = 64;

/// 고정 Chrome 기준에서 무효인 `last baseline` 선언을 Stylo 입력에서 무효화합니다.
///
/// Stylo parser가 Chrome 154보다 넓은 baseline alignment 문법을 받으므로,
/// 해당 선언의 첫 토큰만 같은 길이의 알 수 없는 값으로 바꾸어 이전 cascade
/// 선언이 그대로 승리하게 합니다. 바이트 위치와 줄바꿈은 바뀌지 않습니다.
pub(crate) fn invalidate_chrome_unsupported_last_baseline(
    source: &str,
    stylesheet: bool,
) -> String {
    let mut scanner = LastBaselineScanner::default();
    if stylesheet {
        let mut input = Parser::new(source);
        for _ in StyleSheetParser::new(&mut input, &mut scanner) {}
    } else {
        let mut input = Parser::new(source);
        for _ in RuleBodyParser::new(&mut input, &mut scanner) {}
    }

    if scanner.exceeded_depth || scanner.offsets.is_empty() {
        return source.to_owned();
    }

    let mut bytes = source.as_bytes().to_vec();
    for offset in scanner.offsets {
        if let Some(byte) = bytes.get_mut(offset) {
            // `last`와 escaped identifier는 모두 ASCII 원문 바이트로 시작합니다.
            // 한 바이트만 바꾸어 뒤쪽 원문 위치를 보존합니다.
            if byte.is_ascii() {
                *byte = b'x';
            }
        }
    }
    String::from_utf8(bytes).unwrap_or_else(|_| source.to_owned())
}

#[derive(Default)]
struct LastBaselineScanner {
    offsets: Vec<usize>,
    nested_rule_depth: usize,
    exceeded_depth: bool,
}

impl LastBaselineScanner {
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

impl<'i> DeclarationParser<'i> for LastBaselineScanner {
    type Declaration = ();
    type Error = ();

    fn parse_value(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i>,
        _: &ParserState,
    ) -> Result<Self::Declaration, ParseError<Self::Error>> {
        if name.eq_ignore_ascii_case("align-content") || name.eq_ignore_ascii_case("place-content")
        {
            if let Some(offset) = last_baseline_token_offset(input)? {
                self.offsets.push(offset);
            }
        } else {
            drain_component_values(input)?;
        }
        Ok(())
    }
}

impl<'i> AtRuleParser<'i> for LastBaselineScanner {
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

impl<'i> QualifiedRuleParser<'i> for LastBaselineScanner {
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

impl RuleBodyItemParser<'_, (), ()> for LastBaselineScanner {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        true
    }
}

fn last_baseline_token_offset(input: &mut Parser<'_>) -> Result<Option<usize>, ParseError<()>> {
    let mut identifiers = Vec::new();
    while !input.is_exhausted() {
        input.skip_whitespace();
        if input.is_exhausted() {
            break;
        }
        if input.try_parse(cssparser::parse_important).is_ok() {
            input.skip_whitespace();
            if !input.is_exhausted() {
                drain_component_values(input)?;
                return Ok(None);
            }
            break;
        }

        let start = input.position().byte_index();
        match input.next_including_whitespace_and_comments()?.clone() {
            Token::Ident(value) => {
                let normalized = value.to_ascii_lowercase();
                identifiers.push((normalized, Some(start)));
            }
            Token::WhiteSpace(_) | Token::Comment(_) => {}
            Token::Function(_)
            | Token::ParenthesisBlock
            | Token::CurlyBracketBlock
            | Token::SquareBracketBlock => {
                input.parse_nested_block(drain_component_values)?;
                // 중첩 구성 요소는 이 리터럴 호환 변환 대상이 아닙니다.
                identifiers.push((String::new(), None));
            }
            _ => identifiers.push((String::new(), None)),
        }
    }

    Ok(identifiers.windows(2).find_map(|pair| {
        (pair[0].0 == "last" && pair[1].0 == "baseline")
            .then_some(pair[0].1)
            .flatten()
    }))
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
mod tests;

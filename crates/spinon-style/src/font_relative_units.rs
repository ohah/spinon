use cssparser::{ParseError, Parser, Token};

const FONT_METRIC_UNITS: &[&str] = &[
    "ex", "rex", "ch", "rch", "cap", "rcap", "ic", "ric", "lh", "rlh",
];

/// CSS token stream에서 실제 폰트·line-height metric이 필요한 첫 단위를 반환합니다.
/// 문자열/주석 텍스트는 무시하고 함수·괄호·규칙 블록은 재귀적으로 검사합니다.
pub(crate) fn first_font_metric_unit(css: &str) -> Result<Option<String>, ()> {
    scan_tokens(&mut Parser::new(css)).map_err(|_| ())
}

fn scan_tokens<'i>(parser: &mut Parser<'i>) -> Result<Option<String>, ParseError<()>> {
    let mut found = None;
    while !parser.is_exhausted() {
        let token = parser.next_including_whitespace_and_comments()?.clone();
        match token {
            Token::BadString(_)
            | Token::BadUrl(_)
            | Token::CloseParenthesis
            | Token::CloseSquareBracket
            | Token::CloseCurlyBracket => return Err(ParseError::unexpected_token()),
            Token::Dimension { unit, .. } => {
                if FONT_METRIC_UNITS
                    .iter()
                    .any(|candidate| unit.as_ref().eq_ignore_ascii_case(candidate))
                    && found.is_none()
                {
                    found = Some(unit.to_ascii_lowercase());
                }
            }
            Token::Function(_)
            | Token::ParenthesisBlock
            | Token::CurlyBracketBlock
            | Token::SquareBracketBlock => {
                if let Some(unit) = parser.parse_nested_block(scan_tokens)?
                    && found.is_none()
                {
                    found = Some(unit);
                }
            }
            _ => {}
        }
    }
    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::first_font_metric_unit;

    #[test]
    fn finds_metric_units_in_declarations_custom_properties_and_nested_blocks() {
        assert_eq!(
            first_font_metric_unit("#a { width: 1ch; }").unwrap(),
            Some("ch".to_owned())
        );
        assert_eq!(
            first_font_metric_unit(":root { --size: var(--fallback, calc(2em + 1RLH)); }").unwrap(),
            Some("rlh".to_owned())
        );
        assert_eq!(
            first_font_metric_unit("#a { width: 1c\\68; }").unwrap(),
            Some("ch".to_owned())
        );
    }

    #[test]
    fn ignores_non_metric_lengths_comments_and_strings() {
        assert_eq!(
            first_font_metric_unit(
                r#"/* 1ch */ #a { width: 2em; --x: "1ch"; content: '2rlh'; height: 1rem; }"#,
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn malformed_token_stream_fails_closed() {
        assert!(first_font_metric_unit("#a { width: 1ch; content: \"bad\nstring\"; }").is_err());
    }
}

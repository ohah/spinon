use cssparser::{ParseError, Parser, Token};

const CONTAINER_RELATIVE_UNITS: &[&str] = &["cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax"];

/// C21 container-size 규칙이 준비되기 전 CSS token stream에서 미지원 단위를 찾습니다.
pub(crate) fn first_container_relative_unit(css: &str) -> Result<Option<String>, ()> {
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
                if CONTAINER_RELATIVE_UNITS
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
    use super::first_container_relative_unit;

    #[test]
    fn finds_container_units_in_declarations_and_nested_custom_properties() {
        for unit in ["cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax"] {
            let css = format!("#target {{ width: calc(2px + 1{unit}); }}");
            assert_eq!(
                first_container_relative_unit(&css).unwrap(),
                Some(unit.to_owned())
            );
        }
        assert_eq!(
            first_container_relative_unit(
                ":root { --size: var(--fallback, calc(2px + 1C\\71W)); }"
            )
            .unwrap(),
            Some("cqw".to_owned())
        );
    }

    #[test]
    fn ignores_comments_strings_and_unit_like_identifiers() {
        assert_eq!(
            first_container_relative_unit(
                r#"/* 1cqw */ #target { --text: "2cqh"; width: 3px; } .cqi-name { height: 4px; }"#,
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn malformed_token_stream_fails_closed() {
        assert!(
            first_container_relative_unit("#target { width: 1cqw; content: \"bad\nstring\"; }")
                .is_err()
        );
    }
}

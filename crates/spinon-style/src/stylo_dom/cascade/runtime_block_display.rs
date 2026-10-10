use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser, Token,
};
use spinon_core::NodeId;

use super::StyloDocumentView;

const MAX_NESTED_RULE_DEPTH: usize = 64;

/// C09 Block profile이 계산 전에 거부해야 하는 `display` 값을 찾습니다.
pub fn first_unsupported_runtime_block_display_inline_value(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    view.inline_style_sources()
        .find_map(|(node, css)| first_unsupported_declaration_value(css).map(|value| (node, value)))
}

/// C09.1의 Block profile이 author stylesheet에서 거부해야 하는 `display` 값을 찾습니다.
pub fn first_unsupported_runtime_block_display_stylesheet_value(css: &str) -> Option<String> {
    let mut scanner = BlockDisplayScanner::default();
    let mut input = Parser::new(css);
    for _ in StyleSheetParser::new(&mut input, &mut scanner) {}
    scanner.unsupported
}

fn first_unsupported_declaration_value(css: &str) -> Option<String> {
    let mut scanner = BlockDisplayScanner::default();
    let mut input = Parser::new(css);
    for _ in RuleBodyParser::new(&mut input, &mut scanner) {}
    scanner.unsupported
}

#[derive(Default)]
struct BlockDisplayScanner {
    unsupported: Option<String>,
    nested_rule_depth: usize,
}

impl BlockDisplayScanner {
    fn scan_nested_rule_list(&mut self, input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
        if self.nested_rule_depth >= MAX_NESTED_RULE_DEPTH {
            if self.unsupported.is_none() {
                self.unsupported = Some(format!(
                    "중첩 CSS 규칙 깊이가 {MAX_NESTED_RULE_DEPTH}단계를 초과합니다"
                ));
            }
            return drain_component_values(input);
        }

        self.nested_rule_depth += 1;
        for _ in StyleSheetParser::new(input, self) {}
        self.nested_rule_depth -= 1;
        Ok(())
    }

    fn scan_rule_body(&mut self, input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
        if self.nested_rule_depth >= MAX_NESTED_RULE_DEPTH {
            if self.unsupported.is_none() {
                self.unsupported = Some(format!(
                    "중첩 CSS 규칙 깊이가 {MAX_NESTED_RULE_DEPTH}단계를 초과합니다"
                ));
            }
            return drain_component_values(input);
        }

        self.nested_rule_depth += 1;
        for _ in RuleBodyParser::new(input, self) {}
        self.nested_rule_depth -= 1;
        Ok(())
    }
}

impl<'i> DeclarationParser<'i> for BlockDisplayScanner {
    type Declaration = ();
    type Error = ();

    fn parse_value(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i>,
        _: &ParserState,
    ) -> Result<Self::Declaration, ParseError<Self::Error>> {
        let unsupported = if name.eq_ignore_ascii_case("display") {
            unsupported_display_value(input)?
        } else {
            drain_component_values(input)?;
            None
        };
        if self.unsupported.is_none() {
            self.unsupported = unsupported;
        }
        Ok(())
    }
}

impl<'i> AtRuleParser<'i> for BlockDisplayScanner {
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
        // Grouping at-rules such as @media and @supports contain ordinary style rules.
        // Parse nested rule lists so unsupported display values cannot hide there.
        self.scan_nested_rule_list(input)?;
        Ok(())
    }
}

impl<'i> QualifiedRuleParser<'i> for BlockDisplayScanner {
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

impl RuleBodyItemParser<'_, (), ()> for BlockDisplayScanner {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        true
    }
}

fn unsupported_display_value(input: &mut Parser<'_>) -> Result<Option<String>, ParseError<()>> {
    let mut identifiers = Vec::new();
    let mut unsupported_function = None;

    while !input.is_exhausted() {
        input.skip_whitespace();
        if input.is_exhausted() {
            break;
        }
        if input.try_parse(cssparser::parse_important).is_ok() {
            input.skip_whitespace();
            if !input.is_exhausted() {
                drain_component_values(input)?;
            }
            break;
        }

        match input.next_including_whitespace_and_comments()?.clone() {
            Token::Ident(value) => identifiers.push(value.to_ascii_lowercase()),
            Token::WhiteSpace(_) | Token::Comment(_) => {}
            Token::Function(name) => {
                if name.eq_ignore_ascii_case("var") && unsupported_function.is_none() {
                    unsupported_function = Some("var()".to_owned());
                }
                input.parse_nested_block(drain_component_values)?;
            }
            Token::ParenthesisBlock | Token::CurlyBracketBlock | Token::SquareBracketBlock => {
                input.parse_nested_block(drain_component_values)?;
            }
            _ => {}
        }
    }

    if let Some(function) = unsupported_function {
        return Ok(Some(function));
    }

    let unsupported_keyword = identifiers
        .iter()
        .any(|identifier| is_unsupported_display_keyword(identifier));
    if unsupported_keyword {
        return Ok(Some(identifiers.join(" ")));
    }
    Ok(None)
}

fn is_unsupported_display_keyword(value: &str) -> bool {
    matches!(
        value,
        "contents"
            | "flow"
            | "inherit"
            | "initial"
            | "inline"
            | "inline-block"
            | "inline-flex"
            | "inline-grid"
            | "inline-table"
            | "list-item"
            | "marker"
            | "revert"
            | "revert-layer"
            | "ruby"
            | "ruby-base"
            | "ruby-base-container"
            | "ruby-text"
            | "ruby-text-container"
            | "run-in"
            | "table"
            | "table-caption"
            | "table-cell"
            | "table-column"
            | "table-column-group"
            | "table-footer-group"
            | "table-header-group"
            | "table-row"
            | "table-row-group"
            | "unset"
            | "flex"
            | "grid"
            | "-webkit-box"
            | "-webkit-flex"
            | "-webkit-inline-box"
            | "-webkit-inline-flex"
    )
}

fn drain_component_values(input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
    while !input.is_exhausted() {
        match input.next_including_whitespace_and_comments()?.clone() {
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
    use super::{
        MAX_NESTED_RULE_DEPTH, first_unsupported_declaration_value,
        first_unsupported_runtime_block_display_stylesheet_value,
    };

    #[test]
    fn finds_unsupported_inline_values_without_matching_comments_or_strings() {
        assert_eq!(
            first_unsupported_declaration_value(
                r#"content:"display:grid"; /* display:flex */ width: 2px"#
            ),
            None
        );
        assert_eq!(
            first_unsupported_declaration_value("display: block; display: none !important"),
            None
        );
        assert_eq!(
            first_unsupported_declaration_value("display: flow-root"),
            None,
            "C09 formatting profile에서 flow-root를 허용합니다"
        );
        assert_eq!(
            first_unsupported_declaration_value("display: unknown-value"),
            None,
            "CSS 문법상 무효한 keyword는 Stylo의 정상 cascade recovery에 맡깁니다"
        );
        assert_eq!(
            first_unsupported_declaration_value(r"display: gr\69 d !important"),
            Some("grid".to_owned())
        );
        assert_eq!(
            first_unsupported_declaration_value("display: var(--layout)"),
            Some("var()".to_owned())
        );
        assert_eq!(
            first_unsupported_declaration_value("display: inline flow"),
            Some("inline flow".to_owned())
        );
    }

    #[test]
    fn finds_unsupported_values_in_stylesheet_declaration_blocks_only() {
        assert_eq!(
            first_unsupported_runtime_block_display_stylesheet_value(
                r#".display:grid::before { content: "display:flex"; } .box { display: GRID !important; }"#,
            ),
            Some("grid".to_owned())
        );
        assert_eq!(
            first_unsupported_runtime_block_display_stylesheet_value(
                r#".display:grid::before { content: "display:flex"; } .box { display:block; }"#,
            ),
            None
        );
    }

    #[test]
    fn finds_unsupported_values_inside_nested_grouping_at_rules() {
        assert_eq!(
            first_unsupported_runtime_block_display_stylesheet_value(
                "@media (min-width: 1px) { .box { display: grid; } }",
            ),
            Some("grid".to_owned())
        );
        assert_eq!(
            first_unsupported_runtime_block_display_stylesheet_value(
                "@supports (display: grid) { @layer app { .box { display: flow-root; } } }",
            ),
            None
        );
    }

    #[test]
    fn nested_preludes_and_non_rule_at_rule_bodies_are_not_declarations() {
        assert_eq!(
            first_unsupported_runtime_block_display_stylesheet_value(
                "@supports (display: grid) { .box { display: block; } }",
            ),
            None
        );
        assert_eq!(
            first_unsupported_runtime_block_display_stylesheet_value(
                "@font-face { font-family: Example; display: grid; }",
            ),
            None
        );
    }

    #[test]
    fn finds_unsupported_values_in_nested_style_rules_and_bounds_rule_depth() {
        assert_eq!(
            first_unsupported_runtime_block_display_stylesheet_value(
                ".container { & .box { display: grid; } }",
            ),
            Some("grid".to_owned())
        );

        let css = (0..=MAX_NESTED_RULE_DEPTH)
            .map(|_| "@media all {")
            .collect::<String>()
            + ".box { display: block; }"
            + &"}".repeat(MAX_NESTED_RULE_DEPTH + 1);
        assert_eq!(
            first_unsupported_runtime_block_display_stylesheet_value(&css),
            Some(format!(
                "중첩 CSS 규칙 깊이가 {MAX_NESTED_RULE_DEPTH}단계를 초과합니다"
            ))
        );
    }
}

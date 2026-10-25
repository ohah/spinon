use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser, Token,
};

const MAX_NESTED_AUTHOR_RULE_DEPTH: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextOrientationScan {
    Clean,
    Found,
    Incomplete,
}

pub fn scan_text_orientation_declaration_list(css: &str) -> TextOrientationScan {
    let mut scanner = TextOrientationDeclarationScanner::default();
    let mut input = Parser::new(css);
    let mut parse_error = false;
    {
        for result in RuleBodyParser::new(&mut input, &mut scanner) {
            parse_error |= result.is_err();
        }
    }
    scanner.incomplete |= parse_error;
    scanner.result()
}

pub fn scan_text_orientation_stylesheet(css: &str) -> TextOrientationScan {
    let mut scanner = TextOrientationDeclarationScanner::default();
    let mut input = Parser::new(css);
    let mut parse_error = false;
    {
        for result in StyleSheetParser::new(&mut input, &mut scanner) {
            parse_error |= result.is_err();
        }
    }
    scanner.incomplete |= parse_error;
    scanner.result()
}

#[derive(Default)]
struct TextOrientationDeclarationScanner {
    found: bool,
    incomplete: bool,
    nested_rule_depth: usize,
}

impl TextOrientationDeclarationScanner {
    fn scan_rule_body(&mut self, input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
        if self.nested_rule_depth >= MAX_NESTED_AUTHOR_RULE_DEPTH {
            self.incomplete = true;
            return drain_component_values(input);
        }
        self.nested_rule_depth += 1;
        let mut parse_error = false;
        {
            for result in RuleBodyParser::new(input, self) {
                parse_error |= result.is_err();
            }
        }
        self.incomplete |= parse_error;
        self.nested_rule_depth -= 1;
        Ok(())
    }

    fn result(&self) -> TextOrientationScan {
        if self.found {
            TextOrientationScan::Found
        } else if self.incomplete {
            TextOrientationScan::Incomplete
        } else {
            TextOrientationScan::Clean
        }
    }
}

impl<'i> DeclarationParser<'i> for TextOrientationDeclarationScanner {
    type Declaration = ();
    type Error = ();

    fn parse_value(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i>,
        _: &ParserState,
    ) -> Result<Self::Declaration, ParseError<Self::Error>> {
        if name.eq_ignore_ascii_case("text-orientation") {
            self.found = true;
        }
        drain_component_values(input)?;
        Ok(())
    }
}

impl<'i> AtRuleParser<'i> for TextOrientationDeclarationScanner {
    type Prelude = ();
    type AtRule = ();
    type Error = ();

    fn parse_prelude(
        &mut self,
        _: CowRcStr<'i>,
        input: &mut Parser<'i>,
    ) -> Result<Self::Prelude, ParseError<Self::Error>> {
        drain_component_values(input)?;
        Ok(())
    }

    fn parse_block(
        &mut self,
        _: Self::Prelude,
        _: &ParserState,
        input: &mut Parser<'i>,
    ) -> Result<Self::AtRule, ParseError<Self::Error>> {
        self.scan_rule_body(input)?;
        Ok(())
    }
}

impl<'i> QualifiedRuleParser<'i> for TextOrientationDeclarationScanner {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = ();

    fn parse_prelude(
        &mut self,
        input: &mut Parser<'i>,
    ) -> Result<Self::Prelude, ParseError<Self::Error>> {
        drain_component_values(input)?;
        Ok(())
    }

    fn parse_block(
        &mut self,
        _: Self::Prelude,
        _: &ParserState,
        input: &mut Parser<'i>,
    ) -> Result<Self::QualifiedRule, ParseError<Self::Error>> {
        self.scan_rule_body(input)?;
        Ok(())
    }
}

impl RuleBodyItemParser<'_, (), ()> for TextOrientationDeclarationScanner {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        true
    }
}

fn drain_component_values(input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
    while !input.is_exhausted() {
        let token = input.next_including_whitespace_and_comments()?.clone();
        match token {
            Token::Function(_)
            | Token::ParenthesisBlock
            | Token::CurlyBracketBlock
            | Token::SquareBracketBlock => input.parse_nested_block(drain_component_values)?,
            _ => {}
        }
    }
    Ok(())
}

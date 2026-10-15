use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser, Token,
};
use spinon_core::NodeId;
use style::properties::LonghandId;

use super::{
    ComputedStyleProfile, CssCascadeError, CssViewport, StyloDocumentView, compute_cascade,
    device::enable_scroll_gesture_preferences,
    runtime_overflow::{
        runtime_block_flex_overflow_clip_author_properties,
        runtime_block_flex_overflow_clip_properties,
    },
};
use crate::StylesheetSource;

const MAX_NESTED_RULE_DEPTH: usize = 64;

pub(super) fn runtime_block_flex_scroll_gesture_properties() -> Vec<(&'static str, LonghandId)> {
    let mut properties = runtime_block_flex_overflow_clip_properties();
    for property in [
        ("overscroll-behavior-x", LonghandId::OverscrollBehaviorX),
        ("overscroll-behavior-y", LonghandId::OverscrollBehaviorY),
        ("touch-action", LonghandId::TouchAction),
    ] {
        if !properties.iter().any(|(name, _)| *name == property.0) {
            properties.push(property);
        }
    }
    properties
}

pub(super) fn runtime_block_flex_scroll_gesture_author_properties() -> Vec<&'static str> {
    let mut properties = runtime_block_flex_overflow_clip_author_properties();
    for property in [
        "overscroll-behavior",
        "overscroll-behavior-x",
        "overscroll-behavior-y",
        "touch-action",
    ] {
        if !properties.contains(&property) {
            properties.push(property);
        }
    }
    properties
}

/// C13.3 제한 profile의 scroll input CSS를 기존 C13.1 cascade와 함께 계산합니다.
pub fn compute_runtime_block_flex_scroll_gesture_cascade_with_stylesheets(
    view: &StyloDocumentView,
    author_stylesheets: &[StylesheetSource],
    viewport: CssViewport,
    style_revision: spinon_core::StyleRevision,
) -> Result<super::ComputedStyleSnapshot, CssCascadeError> {
    for (node, css) in view.inline_style_sources() {
        if let Some(value) = first_unsupported_directional_touch_action(css, false) {
            return Err(CssCascadeError::UnsupportedTouchActionValue {
                source_id: format!("inline:{node}"),
                node: Some(node),
                value,
            });
        }
    }
    for source in author_stylesheets {
        if let Some(value) = first_unsupported_directional_touch_action(&source.css, true) {
            return Err(CssCascadeError::UnsupportedTouchActionValue {
                source_id: source.id.clone(),
                node: None,
                value,
            });
        }
    }
    let properties = runtime_block_flex_scroll_gesture_properties();
    let profile_view = {
        let _preferences = enable_scroll_gesture_preferences();
        view.rebuilt_with_current_stylo_preferences()
            .map_err(CssCascadeError::StyloViewRebuild)?
    };
    compute_cascade(
        &profile_view,
        author_stylesheets,
        viewport,
        style_revision,
        &properties,
        ComputedStyleProfile::RuntimeBlockFlexScrollGestureV1,
    )
}

/// C13.3 style profile에서 계산된 inline declaration 밖의 속성을 찾습니다.
pub fn first_unsupported_runtime_block_flex_scroll_gesture_inline_property(
    view: &StyloDocumentView,
) -> Option<(NodeId, String)> {
    let allowed = runtime_block_flex_scroll_gesture_author_properties();
    let _preferences = enable_scroll_gesture_preferences();
    let view = view
        .rebuilt_with_current_stylo_preferences()
        .expect("유효한 Stylo view의 고정 snapshot과 root는 다시 구성할 수 있어야 합니다");
    super::runtime_layout::first_unsupported_inline_property(&view, |property| {
        property.starts_with("--") || allowed.contains(&property)
    })
}

fn first_unsupported_directional_touch_action(css: &str, stylesheet: bool) -> Option<String> {
    let mut scanner = TouchActionScanner::default();
    let mut input = Parser::new(css);
    if stylesheet {
        for _ in StyleSheetParser::new(&mut input, &mut scanner) {}
    } else {
        for _ in RuleBodyParser::new(&mut input, &mut scanner) {}
    }
    scanner.unsupported
}

#[derive(Default)]
struct TouchActionScanner {
    unsupported: Option<String>,
    nested_rule_depth: usize,
}

impl TouchActionScanner {
    fn scan_nested_rule_list(&mut self, input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
        if self.nested_rule_depth >= MAX_NESTED_RULE_DEPTH {
            return drain_component_values(input);
        }
        self.nested_rule_depth += 1;
        for _ in StyleSheetParser::new(input, self) {}
        self.nested_rule_depth -= 1;
        Ok(())
    }

    fn scan_rule_body(&mut self, input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
        if self.nested_rule_depth >= MAX_NESTED_RULE_DEPTH {
            return drain_component_values(input);
        }
        self.nested_rule_depth += 1;
        for _ in RuleBodyParser::new(input, self) {}
        self.nested_rule_depth -= 1;
        Ok(())
    }
}

impl<'i> DeclarationParser<'i> for TouchActionScanner {
    type Declaration = ();
    type Error = ();

    fn parse_value(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i>,
        _: &ParserState,
    ) -> Result<Self::Declaration, ParseError<Self::Error>> {
        let unsupported = if name.eq_ignore_ascii_case("touch-action") {
            unsupported_directional_pan_value(input)?
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

impl<'i> AtRuleParser<'i> for TouchActionScanner {
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

impl<'i> QualifiedRuleParser<'i> for TouchActionScanner {
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

impl RuleBodyItemParser<'_, (), ()> for TouchActionScanner {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        true
    }
}

fn unsupported_directional_pan_value(
    input: &mut Parser<'_>,
) -> Result<Option<String>, ParseError<()>> {
    let mut keywords = Vec::new();
    let mut invalid = false;
    let mut dynamic = false;
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
            Token::Ident(value) => keywords.push(value.to_ascii_lowercase()),
            Token::WhiteSpace(_) | Token::Comment(_) => {}
            Token::Function(name) => {
                dynamic |= name.eq_ignore_ascii_case("var");
                input.parse_nested_block(drain_component_values)?;
            }
            Token::ParenthesisBlock | Token::CurlyBracketBlock | Token::SquareBracketBlock => {
                input.parse_nested_block(drain_component_values)?;
                invalid = true;
            }
            _ => invalid = true,
        }
    }
    if invalid {
        return Ok(None);
    }
    if dynamic {
        return Ok(Some("var()".to_owned()));
    }
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut pinch_zoom = 0;
    let mut has_directional_pan = false;
    for keyword in &keywords {
        match keyword.as_str() {
            "auto" | "none" | "manipulation" if keywords.len() == 1 => {}
            "auto" | "none" | "manipulation" => return Ok(None),
            "pan-x" | "pan-left" | "pan-right" => {
                horizontal += 1;
                has_directional_pan |= keyword != "pan-x";
            }
            "pan-y" | "pan-up" | "pan-down" => {
                vertical += 1;
                has_directional_pan |= keyword != "pan-y";
            }
            "pinch-zoom" => pinch_zoom += 1,
            _ => return Ok(None),
        }
    }
    let valid = !keywords.is_empty()
        && horizontal <= 1
        && vertical <= 1
        && pinch_zoom <= 1
        && (horizontal > 0
            || vertical > 0
            || pinch_zoom > 0
            || keywords == ["auto"]
            || keywords == ["none"]
            || keywords == ["manipulation"]);
    Ok((valid && has_directional_pan).then(|| keywords.join(" ")))
}

fn drain_component_values(input: &mut Parser<'_>) -> Result<(), ParseError<()>> {
    while let Ok(token) = input.next_including_whitespace_and_comments() {
        match token {
            Token::Function(_)
            | Token::ParenthesisBlock
            | Token::CurlyBracketBlock
            | Token::SquareBracketBlock => {
                input.parse_nested_block(drain_component_values)?;
            }
            _ => {}
        }
    }
    Ok(())
}

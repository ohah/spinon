use style::{
    applicable_declarations::{CascadePriority, RevertKind},
    properties::{
        LonghandId, PropertyDeclaration, PropertyDeclarationBlock, PropertyDeclarationId,
    },
    rule_tree::StrongRuleNode,
    servo_arc::Arc,
    shared_lock::{Locked, StylesheetGuards},
};
#[cfg(test)]
use style::{
    rule_tree::{CascadeLevel, CascadeOrigin, RuleCascadeFlags},
    stylesheets::layer_rule::LayerOrder,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WordBreakSource {
    BreakWord,
    Inherit,
    Revert,
    RevertLayer,
    Other,
}

/// Finds the authored value for the winning `word-break` declaration.
///
/// Servo Stylo does not parse the legacy `break-word` keyword. The opt-in C11.4 parser adapter
/// rewrites that token to `normal` while preserving source offsets; this recovers the original
/// value from the declaration block that actually won the cascade.
pub(super) fn winning_word_break_source(
    rules: &StrongRuleNode,
    guards: &StylesheetGuards<'_>,
    inline_style: Option<&Arc<Locked<PropertyDeclarationBlock>>>,
    inline_style_text: Option<&str>,
    stylesheet_sources: &std::collections::HashMap<usize, String>,
) -> Option<WordBreakSource> {
    let mut candidates = Vec::new();

    for node in rules.self_and_ancestors() {
        let Some(source) = node.style_source() else {
            continue;
        };
        let guard = node.cascade_level().guard(guards);
        let block = source.read(guard);
        for (declaration, importance) in block
            .declaration_importance_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            if node.cascade_level().is_important() != importance.important() {
                continue;
            }
            if !is_word_break(declaration) {
                continue;
            }

            let is_inline = inline_style.is_some_and(|inline| Arc::ptr_eq(source.get(), inline));
            let raw = if is_inline {
                inline_style_text
            } else {
                stylesheet_sources
                    .get(&(source.get().raw_ptr().as_ptr() as usize))
                    .map(String::as_str)
            };
            let Some(value) =
                raw.and_then(|raw| source_word_break_value(raw, importance.important()))
            else {
                // If the source cannot be recovered, don't let an inherited alias
                // override the value Stylo computed from this declaration.
                return Some(WordBreakSource::Other);
            };

            candidates.push((node.cascade_priority(), value));
        }
    }

    resolve_word_break_source(candidates)
}

fn resolve_word_break_source(
    candidates: impl IntoIterator<Item = (CascadePriority, WordBreakSource)>,
) -> Option<WordBreakSource> {
    let mut reverted = Vec::<(CascadePriority, RevertKind)>::new();
    let mut saw_revert = false;

    for (priority, value) in candidates {
        if reverted
            .iter()
            .any(|(revert_priority, kind)| !revert_priority.allows_when_reverted(&priority, *kind))
        {
            continue;
        }

        match value {
            WordBreakSource::Revert => {
                saw_revert = true;
                reverted.push((priority, RevertKind::Origin));
            }
            WordBreakSource::RevertLayer => {
                saw_revert = true;
                reverted.push((priority, RevertKind::Layer));
            }
            value => return Some(value),
        }
    }

    // `word-break` is inherited, so if a rollback finds no lower-origin or
    // lower-layer declaration, the computed value comes from the parent.
    saw_revert.then_some(WordBreakSource::Inherit)
}

#[cfg(test)]
fn test_priority(origin: CascadeOrigin, layer_order: LayerOrder) -> CascadePriority {
    CascadePriority::new(
        CascadeLevel::new(origin),
        layer_order,
        RuleCascadeFlags::empty(),
    )
}

fn is_word_break(declaration: &PropertyDeclaration) -> bool {
    matches!(
        declaration.id(),
        PropertyDeclarationId::Longhand(LonghandId::WordBreak)
    )
}

fn source_word_break_value(source: &str, important: bool) -> Option<WordBreakSource> {
    let mut scanner = WordBreakSourceScanner::default();
    let mut input = cssparser::Parser::new(source);
    for _ in cssparser::RuleBodyParser::new(&mut input, &mut scanner) {}
    scanner
        .values
        .into_iter()
        .rev()
        .find_map(|(is_important, value)| (is_important == important).then_some(value))
}

#[derive(Default)]
struct WordBreakSourceScanner {
    values: Vec<(bool, WordBreakSource)>,
}

impl<'i> cssparser::DeclarationParser<'i> for WordBreakSourceScanner {
    type Declaration = ();
    type Error = ();

    fn parse_value(
        &mut self,
        name: cssparser::CowRcStr<'i>,
        input: &mut cssparser::Parser<'i>,
        _: &cssparser::ParserState,
    ) -> Result<Self::Declaration, cssparser::ParseError<Self::Error>> {
        if !name.eq_ignore_ascii_case("word-break") {
            drain_component_values(input)?;
            return Ok(());
        }

        let mut important = false;
        let mut tokens = Vec::new();
        while !input.is_exhausted() {
            input.skip_whitespace();
            if input.is_exhausted() {
                break;
            }
            if input.try_parse(cssparser::parse_important).is_ok() {
                important = true;
                continue;
            }
            match input.next_including_whitespace_and_comments()?.clone() {
                cssparser::Token::WhiteSpace(_) | cssparser::Token::Comment(_) => {}
                cssparser::Token::Ident(value) => tokens.push(value.to_ascii_lowercase()),
                cssparser::Token::Function(_)
                | cssparser::Token::ParenthesisBlock
                | cssparser::Token::CurlyBracketBlock
                | cssparser::Token::SquareBracketBlock => {
                    input.parse_nested_block(drain_component_values)?;
                    tokens.push(String::new());
                }
                _ => tokens.push(String::new()),
            }
        }

        let value = match tokens.as_slice() {
            [value] if value == "break-word" => WordBreakSource::BreakWord,
            [value] if value == "inherit" || value == "unset" => WordBreakSource::Inherit,
            [value] if value == "revert" => WordBreakSource::Revert,
            [value] if value == "revert-layer" => WordBreakSource::RevertLayer,
            _ => WordBreakSource::Other,
        };
        self.values.push((important, value));
        Ok(())
    }
}

impl<'i> cssparser::AtRuleParser<'i> for WordBreakSourceScanner {
    type Prelude = ();
    type AtRule = ();
    type Error = ();

    fn parse_prelude(
        &mut self,
        _: cssparser::CowRcStr<'i>,
        input: &mut cssparser::Parser<'i>,
    ) -> Result<Self::Prelude, cssparser::ParseError<()>> {
        drain_component_values(input)?;
        Ok(())
    }

    fn parse_block(
        &mut self,
        _: Self::Prelude,
        _: &cssparser::ParserState,
        input: &mut cssparser::Parser<'i>,
    ) -> Result<Self::AtRule, cssparser::ParseError<()>> {
        drain_component_values(input)?;
        Ok(())
    }
}

impl<'i> cssparser::QualifiedRuleParser<'i> for WordBreakSourceScanner {
    type Prelude = ();
    type QualifiedRule = ();
    type Error = ();

    fn parse_prelude(
        &mut self,
        input: &mut cssparser::Parser<'i>,
    ) -> Result<Self::Prelude, cssparser::ParseError<()>> {
        drain_component_values(input)?;
        Ok(())
    }

    fn parse_block(
        &mut self,
        _: Self::Prelude,
        _: &cssparser::ParserState,
        input: &mut cssparser::Parser<'i>,
    ) -> Result<Self::QualifiedRule, cssparser::ParseError<()>> {
        drain_component_values(input)?;
        Ok(())
    }
}

impl cssparser::RuleBodyItemParser<'_, (), ()> for WordBreakSourceScanner {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        false
    }
}

fn drain_component_values(
    input: &mut cssparser::Parser<'_>,
) -> Result<(), cssparser::ParseError<()>> {
    while !input.is_exhausted() {
        match input.next_including_whitespace_and_comments()? {
            cssparser::Token::Function(_)
            | cssparser::Token::ParenthesisBlock
            | cssparser::Token::CurlyBracketBlock
            | cssparser::Token::SquareBracketBlock => {
                input.parse_nested_block(drain_component_values)?;
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        WordBreakSource, resolve_word_break_source, source_word_break_value, test_priority,
    };
    use style::{rule_tree::CascadeOrigin, stylesheets::layer_rule::LayerOrder};

    #[test]
    fn source_scanner_tracks_last_value_at_each_importance_level() {
        assert_eq!(
            source_word_break_value("word-break: break-word; word-break:normal", false),
            Some(WordBreakSource::Other)
        );
        assert_eq!(
            source_word_break_value("word-break: normal; word-break:break-word", false),
            Some(WordBreakSource::BreakWord)
        );
        assert_eq!(
            source_word_break_value("word-break: break-word; word-break:normal !important", true),
            Some(WordBreakSource::Other)
        );
    }

    #[test]
    fn source_scanner_preserves_inherited_css_wide_keywords() {
        for value in ["inherit", "unset"] {
            assert_eq!(
                source_word_break_value(&format!("word-break:{value}"), false),
                Some(WordBreakSource::Inherit)
            );
        }
        assert_eq!(
            source_word_break_value("word-break:revert", false),
            Some(WordBreakSource::Revert)
        );
        assert_eq!(
            source_word_break_value("word-break:revert-layer", false),
            Some(WordBreakSource::RevertLayer)
        );
    }

    #[test]
    fn revert_uses_a_lower_cascade_origin_before_parent_inheritance() {
        assert_eq!(
            resolve_word_break_source([
                (
                    test_priority(CascadeOrigin::Author, LayerOrder::root()),
                    WordBreakSource::Revert,
                ),
                (
                    test_priority(CascadeOrigin::UA, LayerOrder::root()),
                    WordBreakSource::Other,
                ),
            ]),
            Some(WordBreakSource::Other)
        );
    }

    #[test]
    fn revert_layer_uses_a_lower_layer_alias() {
        let lower_layer = LayerOrder::first();
        let mut upper_layer = lower_layer;
        upper_layer.inc();
        assert_eq!(
            resolve_word_break_source([
                (
                    test_priority(CascadeOrigin::Author, upper_layer),
                    WordBreakSource::RevertLayer,
                ),
                (
                    test_priority(CascadeOrigin::Author, lower_layer),
                    WordBreakSource::BreakWord,
                ),
            ]),
            Some(WordBreakSource::BreakWord)
        );
    }

    #[test]
    fn rollback_without_a_lower_declaration_inherits() {
        assert_eq!(
            resolve_word_break_source([(
                test_priority(CascadeOrigin::Author, LayerOrder::root()),
                WordBreakSource::Revert,
            )]),
            Some(WordBreakSource::Inherit)
        );
    }
}

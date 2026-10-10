use std::collections::HashMap;

use cssparser::SourceLocation;
use style::{
    properties::{LonghandId, PropertyDeclarationBlock, PropertyDeclarationId},
    servo_arc::Arc,
    shared_lock::{Locked, SharedRwLockReadGuard},
    stylesheets::{CssRule, StylesheetInDocument},
};

use super::RegisteredStylesheet;

const SOURCE_OFFSET_CHECKPOINT_STRIDE: u32 = 32;

pub(super) fn declaration_sources(stylesheets: &[RegisteredStylesheet]) -> HashMap<usize, String> {
    let mut sources = HashMap::new();
    for stylesheet in stylesheets {
        let source = stylesheet.source();
        let offsets = SourceOffsetIndex::new(source);
        let guard = stylesheet.sheet.0.shared_lock.read();
        let rules = stylesheet.sheet.contents(&guard).rules.read_with(&guard);
        collect_rule_sources(&rules.0, source, &offsets, &guard, &mut sources);
    }
    sources
}

fn collect_rule_sources(
    rules: &[CssRule],
    source: &str,
    offsets: &SourceOffsetIndex,
    guard: &SharedRwLockReadGuard<'_>,
    sources: &mut HashMap<usize, String>,
) {
    for rule in rules {
        if let CssRule::Style(style_rule) = rule {
            let style_rule = style_rule.read_with(guard);
            if has_border_width(&style_rule.block, guard)
                && let Some(declarations) =
                    declaration_block_source(source, style_rule.source_location, offsets)
            {
                sources.insert(style_rule.block.raw_ptr().as_ptr() as usize, declarations);
            }
        }
        collect_rule_sources(rule.children(guard), source, offsets, guard, sources);
    }
}

fn has_border_width(
    block: &Arc<Locked<PropertyDeclarationBlock>>,
    guard: &SharedRwLockReadGuard<'_>,
) -> bool {
    block
        .read_with(guard)
        .declaration_importance_iter()
        .any(|(declaration, _)| {
            let PropertyDeclarationId::Longhand(id) = declaration.id() else {
                return false;
            };
            if !matches!(
                id,
                LonghandId::BorderTopWidth
                    | LonghandId::BorderRightWidth
                    | LonghandId::BorderBottomWidth
                    | LonghandId::BorderLeftWidth
            ) {
                return false;
            }
            true
        })
}

fn declaration_block_source(
    source: &str,
    location: SourceLocation,
    offsets: &SourceOffsetIndex,
) -> Option<String> {
    let offset = source_offset(source, location, offsets)?;
    let open = find_rule_open_brace(source.as_bytes(), offset)?;
    let close = matching_brace(source.as_bytes(), open)?;
    source.get(open + 1..close).map(str::to_owned)
}

fn find_rule_open_brace(source: &[u8], mut index: usize) -> Option<usize> {
    let mut parentheses = 0usize;
    let mut brackets = 0usize;
    let mut quote = None;
    let mut comment = false;
    while index < source.len() {
        let byte = source[index];
        if comment {
            if byte == b'*' && source.get(index + 1) == Some(&b'/') {
                comment = false;
                index += 2;
            } else {
                index += 1;
            }
            continue;
        }
        if let Some(quote_byte) = quote {
            if byte == b'\\' {
                index = after_escape(source, index);
            } else {
                if byte == quote_byte {
                    quote = None;
                }
                index += 1;
            }
            continue;
        }
        if byte == b'/' && source.get(index + 1) == Some(&b'*') {
            comment = true;
            index += 2;
        } else if byte == b'\\' {
            index = after_escape(source, index);
        } else if byte == b'\'' || byte == b'"' {
            quote = Some(byte);
            index += 1;
        } else {
            match byte {
                b'(' => parentheses += 1,
                b')' => parentheses = parentheses.saturating_sub(1),
                b'[' => brackets += 1,
                b']' => brackets = brackets.saturating_sub(1),
                b'{' if parentheses == 0 && brackets == 0 => return Some(index),
                _ => {}
            }
            index += 1;
        }
    }
    None
}

fn matching_brace(source: &[u8], open: usize) -> Option<usize> {
    let mut depth = 1usize;
    let mut index = open + 1;
    let mut quote = None;
    let mut comment = false;
    while index < source.len() {
        let byte = source[index];
        if comment {
            if byte == b'*' && source.get(index + 1) == Some(&b'/') {
                comment = false;
                index += 2;
            } else {
                index += 1;
            }
            continue;
        }
        if let Some(quote_byte) = quote {
            if byte == b'\\' {
                index = after_escape(source, index);
            } else {
                if byte == quote_byte {
                    quote = None;
                }
                index += 1;
            }
            continue;
        }
        if byte == b'/' && source.get(index + 1) == Some(&b'*') {
            comment = true;
            index += 2;
        } else if byte == b'\\' {
            index = after_escape(source, index);
        } else if byte == b'\'' || byte == b'"' {
            quote = Some(byte);
            index += 1;
        } else {
            match byte {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(index);
                    }
                }
                _ => {}
            }
            index += 1;
        }
    }
    None
}

fn after_escape(source: &[u8], slash: usize) -> usize {
    let next = slash + 1;
    if source.get(next) == Some(&b'\r') && source.get(next + 1) == Some(&b'\n') {
        next + 2
    } else {
        (next + 1).min(source.len())
    }
}

struct SourceOffsetIndex {
    lines: Vec<SourceLineOffsets>,
}

struct SourceLineOffsets {
    checkpoints: Vec<(u32, usize)>,
    end: usize,
}

impl SourceOffsetIndex {
    fn new(source: &str) -> Self {
        let mut lines = Vec::new();
        let mut checkpoints = vec![(1, 0)];
        let mut column = 1_u32;
        let mut next_checkpoint = 1 + SOURCE_OFFSET_CHECKPOINT_STRIDE;
        let mut chars = source.char_indices().peekable();

        while let Some((offset, character)) = chars.next() {
            let next_line_start = match character {
                '\r' => {
                    let mut next = offset + character.len_utf8();
                    if chars.peek().is_some_and(|(_, next)| *next == '\n')
                        && let Some((newline_offset, newline)) = chars.next()
                    {
                        next = newline_offset + newline.len_utf8();
                    }
                    Some((offset, next))
                }
                '\n' | '\u{000c}' => Some((offset, offset + character.len_utf8())),
                _ => None,
            };

            if let Some((line_end, next_start)) = next_line_start {
                lines.push(SourceLineOffsets {
                    checkpoints,
                    end: line_end,
                });
                checkpoints = vec![(1, next_start)];
                column = 1;
                next_checkpoint = 1 + SOURCE_OFFSET_CHECKPOINT_STRIDE;
                continue;
            }

            while column >= next_checkpoint {
                checkpoints.push((column, offset));
                next_checkpoint += SOURCE_OFFSET_CHECKPOINT_STRIDE;
            }
            column += character.len_utf16() as u32;
        }

        lines.push(SourceLineOffsets {
            checkpoints,
            end: source.len(),
        });
        Self { lines }
    }
}

fn source_offset(
    source: &str,
    location: SourceLocation,
    index: &SourceOffsetIndex,
) -> Option<usize> {
    let line = index.lines.get(location.line as usize)?;
    let target_column = location.column;
    let checkpoint = line
        .checkpoints
        .partition_point(|(column, _)| *column <= target_column)
        .checked_sub(1)?;
    let (mut column, mut offset) = line.checkpoints[checkpoint];
    for character in source.get(offset..line.end)?.chars() {
        if column == target_column {
            return Some(offset);
        }
        column += character.len_utf16() as u32;
        offset += character.len_utf8();
        if column > target_column {
            return None;
        }
    }
    (column == target_column).then_some(line.end)
}

#[cfg(test)]
mod tests {
    use style::{properties::PropertyDeclarationId, shared_lock::SharedRwLockReadGuard};

    use crate::{CssOrigin, StylesheetSource};

    use super::*;

    #[test]
    fn declaration_sources_track_nested_rules_and_utf16_locations() {
        let generated_rules = (1..=64)
            .map(|width| format!(".rule-{width}{{border-width:{width}px}}"))
            .collect::<String>();
        let mut css = "/* { } */\r\n.😀 { content: \"{ }\"; }\r\n@media (min-width: 1px) {\r\n  div { /* } */ content: \"}\"; --edge: 1.999px; border-width: var(--edge); border-style: solid; }\r\n  .precise { border-width: 0.3333333333vh; border-style: solid; }\r\n}\r\n".to_owned();
        css.push_str(&generated_rules);
        let mut registry = super::super::StylesheetRegistry::new();
        registry
            .append(StylesheetSource {
                id: "source-map".to_owned(),
                base_url: "https://spinon.invalid/source-map.css".to_owned(),
                origin: CssOrigin::Author,
                css,
            })
            .unwrap();

        let sources = registry.raw_style_declaration_sources();
        let stylesheet = registry.iter().next().unwrap();
        let guard = stylesheet.stylo_sheet().0.shared_lock.read();
        let rules = stylesheet
            .stylo_sheet()
            .contents(&guard)
            .rules
            .read_with(&guard);
        let source = find_border_rule_source(&rules.0, &guard, &sources).unwrap();

        assert!(source.contains("border-width: var(--edge)"));
        assert!(source.contains("border-style: solid"), "{source:?}");
        assert!(source.contains("content: \"}\""), "{source:?}");
        assert_eq!(sources.len(), 66);
        assert!(
            sources
                .values()
                .any(|source| { source.contains("border-width: 0.3333333333vh") })
        );
    }

    fn find_border_rule_source(
        rules: &[CssRule],
        guard: &SharedRwLockReadGuard<'_>,
        sources: &HashMap<usize, String>,
    ) -> Option<String> {
        for rule in rules {
            if let CssRule::Style(style_rule) = rule {
                let style_rule = style_rule.read_with(guard);
                if style_rule
                    .block
                    .read_with(guard)
                    .declaration_importance_iter()
                    .any(|(declaration, _)| {
                        declaration.id()
                            == PropertyDeclarationId::Longhand(
                                style::properties::LonghandId::BorderLeftWidth,
                            )
                    })
                {
                    let key = style_rule.block.raw_ptr().as_ptr() as usize;
                    return sources.get(&key).cloned();
                }
            }
            if let Some(source) = find_border_rule_source(rule.children(guard), guard, sources) {
                return Some(source);
            }
        }
        None
    }
}

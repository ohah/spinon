use std::collections::BTreeSet;

#[test]
fn incremental_ua_profile_uses_only_the_pinned_type_selectors() {
    let mut source = String::new();
    let mut characters = super::UA_STYLESHEET.chars().peekable();
    while let Some(character) = characters.next() {
        if character == '/' && characters.peek() == Some(&'*') {
            characters.next();
            while let Some(comment_character) = characters.next() {
                if comment_character == '*' && characters.peek() == Some(&'/') {
                    characters.next();
                    break;
                }
            }
        } else {
            source.push(character);
        }
    }
    let mut selectors = BTreeSet::new();
    let mut rule_start = 0;
    for (index, character) in source.char_indices() {
        if character == '}' {
            rule_start = index + character.len_utf8();
        } else if character == '{' {
            let prelude = source[rule_start..index]
                .rsplit(';')
                .next()
                .unwrap_or_default()
                .trim();
            if !prelude.is_empty() {
                for selector in prelude.split(',') {
                    let selector = selector.trim();
                    assert!(
                        !selector.chars().any(|character| {
                            matches!(
                                character,
                                '[' | ']' | ':' | '*' | '+' | '>' | '~' | '#' | '.'
                            )
                        }),
                        "증분 UA profile에는 단순 type selector만 허용합니다: {selector}"
                    );
                    assert!(
                        selector.chars().all(|character| {
                            character.is_ascii_alphanumeric()
                                || matches!(character, '-' | '_' | '|')
                        }),
                        "예상하지 않은 UA selector 문법입니다: {selector}"
                    );
                    selectors.insert(selector.to_owned());
                }
            }
        }
    }

    assert_eq!(
        selectors,
        BTreeSet::from([
            "a".to_owned(),
            "button".to_owned(),
            "div".to_owned(),
            "img".to_owned(),
            "input".to_owned(),
            "li".to_owned(),
            "p".to_owned(),
            "span".to_owned(),
            "style".to_owned(),
            "ul".to_owned(),
        ])
    );
}

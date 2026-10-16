use crate::{
    InlineBoxStyle, InlineEdges, TextLayoutError, TextSource, WhiteSpaceMode, WordBreakMode,
};

use super::{close_to, engine, node, paragraph};

#[test]
fn roboto_metrics_keep_the_chrome_20px_24px_geometry() {
    let mut engine = engine();
    let mut input = paragraph(
        "",
        "Roboto",
        20.0,
        24.0,
        WhiteSpaceMode::NoWrap,
        WordBreakMode::Normal,
    );
    input.sources = vec![
        TextSource {
            node: node(2),
            inline_ancestors: Vec::new(),
            text: "Hgyp".to_owned(),
        },
        TextSource {
            node: node(3),
            inline_ancestors: Vec::new(),
            text: "Hgyp".to_owned(),
        },
    ];
    let marker_node = node(4);
    let layout = engine
        .layout_paragraph_with_test_baseline_marker(&input, 100.0, 4, marker_node)
        .unwrap();

    assert_eq!(layout.lines.len(), 1);
    close_to(layout.width, 93.4063);
    close_to(layout.height, 24.0);
    close_to(layout.lines[0].baseline, 19.0);
    assert!(layout.runs.iter().all(|run| {
        run.font_metrics.ascent.is_finite()
            && run.font_metrics.ascent > 0.0
            && run.font_metrics.descent.is_finite()
            && run.font_metrics.descent >= 0.0
    }));
    close_to(layout.runs[0].font_metrics.ascent, 18.5547);
    close_to(layout.runs[0].font_metrics.descent, 4.8828);
    assert_eq!(layout.sources.len(), 8);
    for (source, expected_x) in [(node(2), 0.0), (node(3), 47.2031)] {
        let fragment = layout
            .inline_fragments
            .iter()
            .find(|fragment| fragment.node == source)
            .expect("Roboto text source has one fragment");
        close_to(fragment.rect.x, expected_x);
        close_to(fragment.rect.width, 46.2031);
    }
    let marker = layout
        .inline_fragments
        .iter()
        .find(|fragment| fragment.node == marker_node)
        .expect("the test baseline marker has one fragment");
    close_to(marker.rect.x, 46.2031);
    close_to(marker.rect.y, 19.0);
    close_to(marker.rect.width, 1.0);
    close_to(marker.rect.height, 0.0);
}

#[test]
fn ahem_baseline_keeps_the_chrome_10px_10px_geometry() {
    let mut engine = engine();
    let input = paragraph(
        "X",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    let layout = engine.layout_paragraph(&input, 30.0).unwrap();

    assert_eq!(layout.lines.len(), 1);
    close_to(layout.width, 10.0);
    close_to(layout.height, 10.0);
    close_to(layout.lines[0].baseline - layout.lines[0].line_top, 8.0);
    close_to(layout.runs[0].glyphs[0].y, 8.0);
}

#[test]
fn invalid_inline_edges_and_overflowing_edge_sums_fail_closed() {
    let owner = node(3);
    let invalid_styles = [
        InlineBoxStyle {
            border: InlineEdges {
                left: -1.0,
                ..InlineEdges::default()
            },
            ..InlineBoxStyle::default()
        },
        InlineBoxStyle {
            padding: InlineEdges {
                top: f32::NAN,
                ..InlineEdges::default()
            },
            ..InlineBoxStyle::default()
        },
        InlineBoxStyle {
            border: InlineEdges {
                right: f32::MAX,
                ..InlineEdges::default()
            },
            padding: InlineEdges {
                right: f32::MAX,
                ..InlineEdges::default()
            },
        },
        InlineBoxStyle::default(),
    ];
    let mut engine = engine();

    for inline_style in invalid_styles {
        let mut input = paragraph(
            "X",
            "Ahem",
            10.0,
            10.0,
            WhiteSpaceMode::Normal,
            WordBreakMode::Normal,
        );
        input.sources[0].inline_ancestors.push(owner);
        input.inline_styles.insert(owner, inline_style);
        assert!(matches!(
            engine.layout_paragraph(&input, 30.0),
            Err(TextLayoutError::InvalidInlineBoxStyle(node)) if node == owner
        ));
    }
}

#[test]
fn nested_inline_width_overflow_from_separate_valid_styles_fails_closed() {
    let outer = node(3);
    let inner = node(4);
    let mut input = paragraph(
        "X",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    input.sources[0].inline_ancestors = vec![outer, inner];
    for owner in [outer, inner] {
        input.inline_styles.insert(
            owner,
            InlineBoxStyle {
                border: InlineEdges {
                    left: f32::MAX,
                    ..InlineEdges::default()
                },
                ..InlineBoxStyle::default()
            },
        );
    }
    let mut engine = engine();

    let result = engine.layout_paragraph(&input, 100.0);
    assert!(
        matches!(
            result,
            Err(TextLayoutError::InvalidGlyphLayout(node)) if node == input.node
        ),
        "unexpected layout result: {result:?}"
    );
}

#[test]
fn decorated_inline_owner_without_visible_text_is_rejected() {
    let owner = node(3);
    let mut input = paragraph(
        "",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    input.sources[0].inline_ancestors.push(owner);
    input.inline_styles.insert(
        owner,
        InlineBoxStyle {
            border: InlineEdges {
                left: 1.0,
                ..InlineEdges::default()
            },
            ..InlineBoxStyle::default()
        },
    );
    let mut engine = engine();

    assert!(matches!(
        engine.layout_paragraph(&input, 30.0),
        Err(TextLayoutError::InlineBoxWithoutText(node)) if node == owner
    ));
}

#[test]
fn noncontiguous_inline_owner_is_rejected_instead_of_decorating_intervening_text() {
    let owner = node(3);
    let mut input = paragraph(
        "",
        "Ahem",
        10.0,
        10.0,
        WhiteSpaceMode::Normal,
        WordBreakMode::Normal,
    );
    input.sources = vec![
        TextSource {
            node: node(2),
            inline_ancestors: vec![owner],
            text: "X".to_owned(),
        },
        TextSource {
            node: node(4),
            inline_ancestors: Vec::new(),
            text: "X".to_owned(),
        },
        TextSource {
            node: node(5),
            inline_ancestors: vec![owner],
            text: "X".to_owned(),
        },
    ];
    input.inline_styles.insert(
        owner,
        InlineBoxStyle {
            border: InlineEdges {
                left: 1.0,
                right: 1.0,
                ..InlineEdges::default()
            },
            ..InlineBoxStyle::default()
        },
    );
    let mut engine = engine();

    assert!(matches!(
        engine.layout_paragraph(&input, 100.0),
        Err(TextLayoutError::InvalidInlineHierarchy(node)) if node == owner
    ));
}

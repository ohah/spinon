use super::node;
use crate::{
    FlexDirection, LayoutAlignItems, LayoutDimension, LayoutDisplay, TaffyLayoutEngine,
    TextLayoutEngine, TextParagraphDirection, WhiteSpaceMode,
};

const NOTO_SANS_KR: &[u8] =
    include_bytes!("../../../../tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf");

fn noto_text_engine() -> TextLayoutEngine {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Noto Sans KR", NOTO_SANS_KR)
        .expect("the pinned Noto Sans KR fixture should register");
    engine
}

#[test]
fn c15_3_flex_max_content_item_matches_noto_intrinsic_text_oracle() {
    // The saved Chrome fixture checks the max-content Flex box, not Canvas glyph advance.
    let mut input = super::input(0.0, "한글字", WhiteSpaceMode::NoWrap);
    input.root_sizing = crate::RootSizingPolicy::ResolveWithin;

    let flex = &mut input.nodes[0].style;
    flex.display = LayoutDisplay::Flex;
    flex.width = LayoutDimension::MaxContent;
    flex.height = LayoutDimension::MaxContent;
    flex.flex_direction = FlexDirection::Row;
    flex.align_items = LayoutAlignItems::FlexStart;

    let item = &mut input.nodes[1].style;
    item.width = LayoutDimension::Auto;
    item.height = LayoutDimension::Auto;
    item.flex_basis = LayoutDimension::Auto;

    let crate::TextParagraphInput::Paragraph(paragraph) =
        input.text_paragraphs.get_mut(&node(2)).unwrap()
    else {
        panic!("the Flex text item should have a paragraph input");
    };
    paragraph.style.font_family = "Noto Sans KR".to_owned();
    paragraph.style.font_size = 20.0;
    paragraph.style.line_height = 24.0;
    paragraph.style.direction = TextParagraphDirection::Ltr;

    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &noto_text_engine())
        .expect("the fixed-font Flex intrinsic sizing fixture should lay out");
    let flex_frame = output.frames[&node(1)];
    let item_frame = output.frames[&node(2)];

    assert!((flex_frame.width - 56.8125).abs() <= 0.05);
    assert!((flex_frame.height - 24.0).abs() <= 0.05);
    assert!((item_frame.width - 56.8125).abs() <= 0.05);
    assert!((item_frame.height - 24.0).abs() <= 0.05);

    let text = output
        .text_layouts
        .get(&node(2))
        .expect("the measured Flex text item should retain its final text snapshot");
    let paragraph = input.text_paragraphs[&node(2)].paragraph().unwrap();
    assert_eq!(text.revision, output.revision);
    assert_eq!(text.input.node, node(2));
    assert_eq!(text.input.sources, paragraph.sources);
    assert_eq!(text.input.style, paragraph.style);
    assert_eq!(text.layout.lines.len(), 1);
    assert_eq!(text.layout.height, 24.0);
    assert_eq!(text.layout.available_width, item_frame.width);
}

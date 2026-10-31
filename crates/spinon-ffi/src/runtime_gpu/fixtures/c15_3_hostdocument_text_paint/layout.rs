use std::collections::BTreeMap;

use spinon_core::{
    DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::{
    LayoutDimension, LayoutDisplay, LayoutInput, LayoutOutput, LayoutOverflow,
    LayoutSourceRevision, LayoutStyle, LayoutTextSnapshot, ParagraphStyle, TaffyLayoutEngine,
    Viewport, WhiteSpaceMode, WordBreakMode,
};
use spinon_text_layout::{LigatureMode, LineBreakMode, TextDirection, TextLayoutEngine};

use super::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

const AHEM: &[u8] = include_bytes!("../../../../../../tests/fixtures/resources/c15/Ahem.ttf");
const ROOT_WIDTH: f32 = 320.0;
const ROOT_HEIGHT: f32 = 240.0;
const FONT_SIZE: f32 = 10.0;
const PARAGRAPH_WIDTH: f32 = 15.0;
const TEXT: &str = "XX";

pub(super) struct FixtureDocument {
    pub(super) snapshot: spinon_core::HostDocumentSnapshot,
    pub(super) root: HostNodeHandle,
    pub(super) paragraph: HostNodeHandle,
    pub(super) text: HostNodeHandle,
}

pub(super) fn fixture_document() -> Result<FixtureDocument, String> {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let mut document = HostDocument::new()
        .map_err(|error| format!("C15.3 fixture HostDocument 생성 실패: {error}"))?;
    let owner = OwnerId::new(1)
        .ok_or_else(|| "C15.3 fixture HostDocument owner ID가 잘못되었습니다".to_owned())?;
    let root = document
        .reserve_node_handle()
        .map_err(|error| format!("C15.3 fixture root handle 생성 실패: {error}"))?;
    let paragraph = document
        .reserve_node_handle()
        .map_err(|error| format!("C15.3 fixture paragraph handle 생성 실패: {error}"))?;
    let text = document
        .reserve_node_handle()
        .map_err(|error| format!("C15.3 fixture text handle 생성 실패: {error}"))?;
    let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
    batch
        .push(DocumentOperation::CreateElement {
            node: root,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        })
        .push(DocumentOperation::CreateElement {
            node: paragraph,
            namespace: HTML.to_owned(),
            local_name: "p".to_owned(),
        })
        .push(DocumentOperation::CreateText {
            node: text,
            data: TEXT.into(),
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Root,
            node: root,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: paragraph,
            before: None,
        })
        .push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(paragraph),
            node: text,
            before: None,
        });
    document
        .commit(batch)
        .map_err(|error| format!("C15.3 fixture HostDocument commit 실패: {error}"))?;
    Ok(FixtureDocument {
        snapshot: document.snapshot(),
        root,
        paragraph,
        text,
    })
}

pub(super) fn layout_fixture(
    fixture: &FixtureDocument,
) -> Result<
    (
        LayoutInput,
        LayoutOutput,
        LayoutTextSnapshot,
        TextLayoutEngine,
    ),
    String,
> {
    let styles = BTreeMap::from([
        (
            fixture.root.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(ROOT_WIDTH),
                height: LayoutDimension::Fixed(ROOT_HEIGHT),
                display: LayoutDisplay::Block,
                overflow_x: LayoutOverflow::Visible,
                overflow_y: LayoutOverflow::Visible,
                ..LayoutStyle::default()
            },
        ),
        (
            fixture.paragraph.id(),
            LayoutStyle {
                width: LayoutDimension::Fixed(PARAGRAPH_WIDTH),
                height: LayoutDimension::Auto,
                display: LayoutDisplay::Block,
                overflow_x: LayoutOverflow::Visible,
                overflow_y: LayoutOverflow::Visible,
                ..LayoutStyle::default()
            },
        ),
    ]);
    let paragraphs = BTreeMap::from([(
        fixture.paragraph.id(),
        ParagraphStyle {
            writing_mode: spinon_core::WritingMode::HorizontalTb,
            text_orientation: spinon_core::TextOrientation::Mixed,
            font_family: "Ahem".to_owned(),
            font_size: FONT_SIZE,
            line_height: FONT_SIZE,
            white_space: WhiteSpaceMode::Normal,
            word_break: WordBreakMode::Normal,
            locale: None,
            direction: TextDirection::Ltr,
            line_break: LineBreakMode::Strict,
            ligatures: LigatureMode::Normal,
            letter_spacing: 0.0,
            text_indent: 0.0,
        },
    )]);
    let input = LayoutInput::from_host_document_with_text_paragraphs(
        &fixture.snapshot,
        fixture.root,
        Viewport {
            width: VIEWPORT_WIDTH,
            height: VIEWPORT_HEIGHT,
        },
        &styles,
        &paragraphs,
        StyleRevision::INITIAL,
        EnvironmentRevision::INITIAL,
    )
    .map_err(|error| format!("C15.3 fixture HostDocument layout input 실패: {error}"))?;
    let text_engine = text_engine()?;
    let output = TaffyLayoutEngine
        .compute_with_text_layout_engine(&input, &text_engine)
        .map_err(|error| format!("C15.3 fixture HostDocument Taffy layout 실패: {error}"))?;
    let text_snapshot = output
        .text_layouts
        .get(&fixture.paragraph.id())
        .ok_or_else(|| "C15.3 fixture paragraph text layout snapshot이 없습니다".to_owned())?
        .clone();
    Ok((input, output, text_snapshot, text_engine))
}

fn text_engine() -> Result<TextLayoutEngine, String> {
    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", AHEM)
        .map_err(|error| format!("C15.3 fixture Ahem 등록 실패: {error}"))?;
    Ok(engine)
}

pub(super) fn validate_layout_snapshot(
    fixture: &FixtureDocument,
    input: &LayoutInput,
    output: &LayoutOutput,
    text_snapshot: &LayoutTextSnapshot,
    paragraph_frame: spinon_layout::LayoutFrame,
) -> Result<(), String> {
    if input.revision() != output.revision || text_snapshot.revision != input.revision() {
        return Err("C15.3 fixture text layout revision이 layout 입력과 다릅니다".to_owned());
    }
    let expected_source = LayoutSourceRevision::HostDocument {
        generation: fixture.snapshot.generation(),
        document: fixture.snapshot.document_revision(),
        render_tree: fixture.snapshot.render_tree_revision(),
    };
    if input.source_revision() != expected_source {
        return Err(
            "C15.3 fixture layout source가 자체 HostDocument snapshot과 다릅니다".to_owned(),
        );
    }
    if text_snapshot.input.node != fixture.paragraph.id()
        || text_snapshot.input.sources.len() != 1
        || text_snapshot.input.sources[0].node != fixture.text.id()
        || text_snapshot.input.sources[0].text != TEXT
        || !text_snapshot.input.sources[0].inline_ancestors.is_empty()
    {
        return Err(
            "C15.3 fixture paragraph layout이 HostDocument Text source와 다릅니다".to_owned(),
        );
    }
    let source = text_snapshot.layout.sources.as_slice();
    if text_snapshot.layout.paragraph != fixture.paragraph.id()
        || text_snapshot.layout.text != TEXT
        || source.len() != 2
        || source.iter().enumerate().any(|(index, source)| {
            source.node != fixture.text.id()
                || source.source_utf16 != (index..index + 1)
                || source.paragraph_utf8 != (index..index + 1)
        })
        || (text_snapshot.layout.available_width - PARAGRAPH_WIDTH).abs() > 0.05
        || (text_snapshot.layout.width - 20.0).abs() > 0.05
        || (text_snapshot.layout.height - FONT_SIZE).abs() > 0.05
        || text_snapshot.layout.lines.len() != 1
        || text_snapshot.layout.lines[0].text_range != (0..2)
        || (text_snapshot.layout.lines[0].advance - 20.0).abs() > 0.05
        || (text_snapshot.layout.lines[0].baseline - 8.0).abs() > 0.05
    {
        return Err(format!(
            "C15.3 fixture shaped layout이 long-word 사전 oracle과 다릅니다: available={} width={} height={} lines={:?} sources={:?} runs={:?}",
            text_snapshot.layout.available_width,
            text_snapshot.layout.width,
            text_snapshot.layout.height,
            text_snapshot.layout.lines,
            text_snapshot.layout.sources,
            text_snapshot.layout.runs,
        ));
    }
    let shaped_glyphs = text_snapshot
        .layout
        .runs
        .iter()
        .flat_map(|run| run.glyphs.iter())
        .collect::<Vec<_>>();
    if shaped_glyphs.len() != 2
        || shaped_glyphs.iter().enumerate().any(|(index, glyph)| {
            (glyph.advance - FONT_SIZE).abs() > 0.05
                || (glyph.x - index as f32 * FONT_SIZE).abs() > 0.05
                || glyph.sources.len() != 1
                || glyph.sources[0].node != fixture.text.id()
                || glyph.sources[0].source_utf16 != (index..index + 1)
        })
    {
        return Err("C15.3 fixture glyph source mapping 또는 advance가 예상과 다릅니다".to_owned());
    }
    let root_frame = output
        .frames
        .get(&fixture.root.id())
        .ok_or_else(|| "C15.3 fixture root LayoutFrame이 없습니다".to_owned())?;
    if !close(root_frame.x, 0.0)
        || !close(root_frame.y, 0.0)
        || !close(root_frame.width, ROOT_WIDTH)
        || !close(root_frame.height, ROOT_HEIGHT)
        || !close(paragraph_frame.x, 0.0)
        || !close(paragraph_frame.y, 0.0)
        || !close(paragraph_frame.width, PARAGRAPH_WIDTH)
        || !close(paragraph_frame.height, FONT_SIZE)
    {
        return Err("C15.3 fixture root 또는 paragraph frame이 사전 oracle과 다릅니다".to_owned());
    }
    Ok(())
}

fn close(actual: f32, expected: f32) -> bool {
    actual.is_finite() && (actual - expected).abs() <= 0.05
}

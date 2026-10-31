use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{DocumentChangeBatch, DocumentOperation, HostParent, OwnerId, StyleRevision};
use spinon_layout::{ParagraphStyle, TextLayoutEngine, WhiteSpaceMode, WordBreakMode};

use super::super::{
    Fixture, compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};
use crate::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs;

const AHEM: &[u8] = include_bytes!("../../../../../tests/fixtures/resources/c15/Ahem.ttf");
const TRACK: &str = "minmax(calc(10% + 2px),max-content)";

#[derive(Clone, Copy)]
pub(super) struct Item {
    pub(super) id: &'static str,
    pub(super) key: &'static str,
    pub(super) text: &'static str,
    pub(super) placement: &'static str,
    pub(super) width: &'static str,
    pub(super) height: &'static str,
}

pub(super) const A: Item = Item {
    id: "item-a",
    key: "a",
    text: "AA A A",
    placement: "grid-column:1 / 3;grid-row:1 / 3",
    width: "auto",
    height: "auto",
};
pub(super) const B: Item = Item {
    id: "item-b",
    key: "b",
    text: "XXXX",
    placement: "grid-column:2 / 4;grid-row:1 / 3",
    width: "40px",
    height: "10px",
};
pub(super) const C: Item = Item {
    id: "item-c",
    key: "c",
    text: "AA AA AA AA",
    placement: "grid-column:1 / 4;grid-row:1 / 3",
    width: "auto",
    height: "auto",
};
pub(super) const ITEMS: [Item; 3] = [A, B, C];

pub(super) struct Case {
    pub(super) id: &'static str,
    pub(super) width: &'static str,
    pub(super) order: [Item; 3],
    pub(super) root: [f32; 2],
    pub(super) columns: [f32; 3],
    pub(super) rows: [f32; 2],
    pub(super) item_frames: [[f32; 4]; 3],
    pub(super) line_counts: [usize; 3],
    pub(super) minimum_inline: [f32; 3],
    pub(super) minimum_block: [f32; 3],
    pub(super) block_contributions: [f32; 3],
}

const INTRINSIC_COLUMNS: [f32; 3] = [15.671875, 15.671875, 15.65625];
const INTRINSIC_FRAMES: [[f32; 4]; 3] = [
    [2.0, 2.0, 33.34375, 20.0],
    [19.671875, 2.0, 40.0, 10.0],
    [2.0, 2.0, 51.0, 20.0],
];
const DEFINITE_FRAMES: [[f32; 4]; 3] = [
    [2.0, 2.0, 26.0, 40.0],
    [16.0, 2.0, 40.0, 10.0],
    [2.0, 2.0, 40.0, 40.0],
];
const MAX_CONTENT_COLUMNS: [f32; 3] = [107.0 / 3.0, 125.0 / 3.0, 86.0 / 3.0];
const MAX_CONTENT_FRAMES: [[f32; 4]; 3] = [
    [2.0, 2.0, 238.0 / 3.0, 10.0],
    [119.0 / 3.0, 2.0, 40.0, 10.0],
    [2.0, 2.0, 110.0, 10.0],
];
const DEFINITE_80_FRAMES: [[f32; 4]; 3] = [
    [2.0, 2.0, 158.0 / 3.0, 20.0],
    [88.0 / 3.0, 2.0, 40.0, 10.0],
    [2.0, 2.0, 80.0, 20.0],
];
pub(super) const CASES: [Case; 8] = [
    Case {
        id: "cyclic-minmax-cross-axis-mixed-span-groups-intrinsic-forward",
        width: "min-content",
        order: [A, B, C],
        root: [55.0, 24.0],
        columns: INTRINSIC_COLUMNS,
        rows: [9.0, 9.0],
        item_frames: INTRINSIC_FRAMES,
        line_counts: [2, 1, 2],
        minimum_inline: [20.0, 40.0, 20.0],
        minimum_block: [20.0, 10.0, 20.0],
        block_contributions: [20.0, 10.0, 20.0],
    },
    Case {
        id: "cyclic-minmax-cross-axis-mixed-span-groups-intrinsic-reverse",
        width: "min-content",
        order: [C, B, A],
        root: [55.0, 24.0],
        columns: INTRINSIC_COLUMNS,
        rows: [9.0, 9.0],
        item_frames: INTRINSIC_FRAMES,
        line_counts: [2, 1, 2],
        minimum_inline: [20.0, 40.0, 20.0],
        minimum_block: [20.0, 10.0, 20.0],
        block_contributions: [20.0, 10.0, 20.0],
    },
    Case {
        id: "cyclic-minmax-cross-axis-max-content-span-groups-forward",
        width: "max-content",
        order: [A, B, C],
        root: [114.0, 14.0],
        columns: MAX_CONTENT_COLUMNS,
        rows: [4.0, 4.0],
        item_frames: MAX_CONTENT_FRAMES,
        line_counts: [1, 1, 1],
        minimum_inline: [20.0, 40.0, 20.0],
        minimum_block: [10.0, 10.0, 10.0],
        block_contributions: [10.0, 10.0, 10.0],
    },
    Case {
        id: "cyclic-minmax-cross-axis-max-content-span-groups-reverse",
        width: "max-content",
        order: [C, B, A],
        root: [114.0, 14.0],
        columns: MAX_CONTENT_COLUMNS,
        rows: [4.0, 4.0],
        item_frames: MAX_CONTENT_FRAMES,
        line_counts: [1, 1, 1],
        minimum_inline: [20.0, 40.0, 20.0],
        minimum_block: [10.0, 10.0, 10.0],
        block_contributions: [10.0, 10.0, 10.0],
    },
    Case {
        id: "definite-minmax-cross-axis-mixed-span-groups-forward",
        width: "40px",
        order: [A, B, C],
        root: [44.0, 44.0],
        columns: [12.0, 12.0, 12.0],
        rows: [19.0, 19.0],
        item_frames: DEFINITE_FRAMES,
        line_counts: [3, 1, 4],
        minimum_inline: [0.0, 40.0, 0.0],
        minimum_block: [0.0, 10.0, 0.0],
        block_contributions: [30.0, 10.0, 40.0],
    },
    Case {
        id: "definite-minmax-cross-axis-mixed-span-groups-reverse",
        width: "40px",
        order: [C, B, A],
        root: [44.0, 44.0],
        columns: [12.0, 12.0, 12.0],
        rows: [19.0, 19.0],
        item_frames: DEFINITE_FRAMES,
        line_counts: [3, 1, 4],
        minimum_inline: [0.0, 40.0, 0.0],
        minimum_block: [0.0, 10.0, 0.0],
        block_contributions: [30.0, 10.0, 40.0],
    },
    Case {
        id: "definite-minmax-cross-axis-max-content-span-groups-forward",
        width: "80px",
        order: [A, B, C],
        root: [84.0, 24.0],
        columns: [76.0 / 3.0; 3],
        rows: [9.0, 9.0],
        item_frames: DEFINITE_80_FRAMES,
        line_counts: [2, 1, 2],
        minimum_inline: [0.0, 40.0, 0.0],
        minimum_block: [0.0, 10.0, 0.0],
        block_contributions: [20.0, 10.0, 20.0],
    },
    Case {
        id: "definite-minmax-cross-axis-max-content-span-groups-reverse",
        width: "80px",
        order: [C, B, A],
        root: [84.0, 24.0],
        columns: [76.0 / 3.0; 3],
        rows: [9.0, 9.0],
        item_frames: DEFINITE_80_FRAMES,
        line_counts: [2, 1, 2],
        minimum_inline: [0.0, 40.0, 0.0],
        minimum_block: [0.0, 10.0, 0.0],
        block_contributions: [20.0, 10.0, 20.0],
    },
];

pub(super) fn fixture(case: &Case) -> Fixture {
    let columns = format!("{TRACK} {TRACK} {TRACK}");
    let rows = format!("{TRACK} {TRACK}");
    let root_style = format!(
        "box-sizing:content-box;display:grid;width:{};height:max-content;margin:0;padding:0;border:2px solid #f0c674;column-gap:2px;row-gap:2px;align-content:start;justify-content:start;grid-template-columns:{columns};grid-template-rows:{rows};background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d",
        case.width
    );
    let item_styles = case
        .order
        .iter()
        .map(|item| {
            (
                item.id,
                format!(
                    "box-sizing:border-box;display:block;width:{};height:{};min-width:auto;min-height:auto;margin:0;padding:0;border:0;{};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;background-color:#e5484d",
                    item.width, item.height, item.placement
                ),
            )
        })
        .collect::<Vec<_>>();
    let item_style_refs = item_styles
        .iter()
        .map(|(id, style)| (*id, style.as_str()))
        .collect::<Vec<_>>();
    let mut fixture = Fixture::new(&root_style, &item_style_refs);
    let text_nodes = case
        .order
        .iter()
        .map(|item| {
            (
                fixture.nodes[item.id],
                fixture.document.reserve_node_handle().unwrap(),
                item.text,
            )
        })
        .collect::<Vec<_>>();
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_234).unwrap(),
        fixture.document.document_revision(),
    );
    for (item, text_node, text) in text_nodes {
        batch
            .push(DocumentOperation::CreateText {
                node: text_node,
                data: text.to_owned().into(),
            })
            .push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(item),
                node: text_node,
                before: None,
            });
    }
    fixture.document.commit(batch).unwrap();
    fixture
}

pub(super) fn compute(fixture: &Fixture) -> crate::StyleLayoutOutput {
    let snapshot = fixture.document.snapshot();
    let view = spinon_style::StyloDocumentView::new_html_runtime_mount_shared(
        Arc::new(snapshot.clone()),
        fixture.root,
    )
    .unwrap();
    let viewport = spinon_style::CssViewport {
        width_css_px: 240.0,
        height_css_px: 160.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: spinon_style::CssMediaEnvironment::MOBILE,
    };
    let computed_styles = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let paragraphs = BTreeMap::from(ITEMS.map(|item| {
        (
            fixture.nodes[item.id].id(),
            ParagraphStyle {
                overflow_wrap: Default::default(),
                writing_mode: spinon_core::WritingMode::HorizontalTb,
                text_orientation: spinon_core::TextOrientation::Mixed,
                font_family: "Ahem".to_owned(),
                font_size: 10.0,
                line_height: 10.0,
                white_space: WhiteSpaceMode::Normal,
                word_break: WordBreakMode::Normal,
                ..ParagraphStyle::default()
            },
        )
    }));
    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs(
        &snapshot,
        fixture.root,
        computed_styles,
        viewport,
        &paragraphs,
        &text_engine,
    )
    .unwrap()
}

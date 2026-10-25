use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostParent, IntrinsicSizeRevision,
    OwnerId, StyleRevision,
};
use spinon_layout::{
    LayoutIntrinsicSize, LayoutIntrinsicSizeSnapshot, LayoutSourceRevision, ParagraphStyle,
    TextLayoutEngine, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};

use crate::{
    StyleLayoutError,
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes,
    compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes_and_two_axis_grid_fixture,
};

use super::super::Fixture;

const AHEM: &[u8] = include_bytes!("../../../../../tests/fixtures/resources/c15/Ahem.ttf");
const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const TRACK: &str = "minmax(calc(10% + 2px),max-content)";

#[derive(Clone, Copy)]
pub(super) struct Item {
    pub(super) id: &'static str,
    pub(super) key: &'static str,
    pub(super) index: usize,
    pub(super) span: u16,
    tag: &'static str,
    placement: &'static str,
    text: Option<&'static str>,
}

pub(super) const A: Item = Item {
    id: "item-a",
    key: "a",
    index: 0,
    span: 2,
    tag: "div",
    placement: "grid-column:1 / 3;grid-row:1 / 3",
    text: Some("AA A A"),
};
pub(super) const B: Item = Item {
    id: "item-b",
    key: "b",
    index: 1,
    span: 2,
    tag: "img",
    placement: "grid-column:2 / 4;grid-row:1 / 3",
    text: None,
};
pub(super) const C: Item = Item {
    id: "item-c",
    key: "c",
    index: 2,
    span: 3,
    tag: "div",
    placement: "grid-column:1 / 4;grid-row:1 / 3",
    text: Some("AA AA AA AA"),
};

const FORWARD: [Item; 3] = [A, B, C];
const REVERSE: [Item; 3] = [C, B, A];
const INTRINSIC_LINES: [&[f32]; 2] = [&[20.0, 30.0], &[50.0, 50.0]];
const DEFINITE_LINES: [&[f32]; 2] = [&[20.0, 10.0, 10.0], &[20.0, 20.0, 20.0, 20.0]];
const INTRINSIC_FRAMES: [[f32; 4]; 3] = [
    [2.0, 2.0, 33.34375, 80.0 / 3.0],
    [19.671875, 2.0, 40.0, 80.0 / 3.0],
    [2.0, 2.0, 51.0, 80.0 / 3.0],
];
const DEFINITE_FRAMES: [[f32; 4]; 3] = [
    [2.0, 2.0, 26.0, 40.0],
    [16.0, 2.0, 40.0, 80.0 / 3.0],
    [2.0, 2.0, 40.0, 40.0],
];

#[derive(Clone, Copy)]
pub(super) struct Case {
    pub(super) id: &'static str,
    pub(super) root_width: &'static str,
    pub(super) intrinsic: bool,
    pub(super) order: &'static [Item; 3],
    pub(super) root: [f32; 2],
    pub(super) columns: [f32; 3],
    pub(super) rows: [f32; 2],
    pub(super) frames: [[f32; 4]; 3],
    pub(super) lines: [&'static [f32]; 2],
    pub(super) line_heights: [f32; 2],
}

pub(super) const CASES: [Case; 4] = [
    Case {
        id: "cyclic-minmax-calc-mixed-text-image-span-intrinsic-forward",
        root_width: "min-content",
        intrinsic: true,
        order: &FORWARD,
        root: [55.0, 92.0 / 3.0],
        columns: [15.671875, 15.671875, 15.65625],
        rows: [37.0 / 3.0, 37.0 / 3.0],
        frames: INTRINSIC_FRAMES,
        lines: INTRINSIC_LINES,
        line_heights: [20.0, 20.0],
    },
    Case {
        id: "cyclic-minmax-calc-mixed-text-image-span-intrinsic-reverse",
        root_width: "min-content",
        intrinsic: true,
        order: &REVERSE,
        root: [55.0, 92.0 / 3.0],
        columns: [15.671875, 15.671875, 15.65625],
        rows: [37.0 / 3.0, 37.0 / 3.0],
        frames: INTRINSIC_FRAMES,
        lines: INTRINSIC_LINES,
        line_heights: [20.0, 20.0],
    },
    Case {
        id: "definite-minmax-calc-mixed-text-image-span-forward",
        root_width: "40px",
        intrinsic: false,
        order: &FORWARD,
        root: [44.0, 44.0],
        columns: [12.0, 12.0, 12.0],
        rows: [19.0, 19.0],
        frames: DEFINITE_FRAMES,
        lines: DEFINITE_LINES,
        line_heights: [30.0, 40.0],
    },
    Case {
        id: "definite-minmax-calc-mixed-text-image-span-reverse",
        root_width: "40px",
        intrinsic: false,
        order: &REVERSE,
        root: [44.0, 44.0],
        columns: [12.0, 12.0, 12.0],
        rows: [19.0, 19.0],
        frames: DEFINITE_FRAMES,
        lines: DEFINITE_LINES,
        line_heights: [30.0, 40.0],
    },
];

const MAX_CONTENT_LINES: [&[f32]; 2] = [&[60.0], &[110.0]];
const MAX_CONTENT_FRAMES: [[f32; 4]; 3] = [
    [2.0, 2.0, 238.0 / 3.0, 80.0 / 3.0],
    [119.0 / 3.0, 2.0, 40.0, 80.0 / 3.0],
    [2.0, 2.0, 110.0, 80.0 / 3.0],
];

pub(super) const MAX_CONTENT_CASES: [Case; 2] = [
    Case {
        id: "max-content-forward",
        root_width: "max-content",
        intrinsic: true,
        order: &FORWARD,
        root: [114.0, 92.0 / 3.0],
        columns: [107.0 / 3.0, 125.0 / 3.0, 86.0 / 3.0],
        rows: [37.0 / 3.0, 37.0 / 3.0],
        frames: MAX_CONTENT_FRAMES,
        lines: MAX_CONTENT_LINES,
        line_heights: [10.0, 10.0],
    },
    Case {
        id: "max-content-reverse",
        root_width: "max-content",
        intrinsic: true,
        order: &REVERSE,
        root: [114.0, 92.0 / 3.0],
        columns: [107.0 / 3.0, 125.0 / 3.0, 86.0 / 3.0],
        rows: [37.0 / 3.0, 37.0 / 3.0],
        frames: MAX_CONTENT_FRAMES,
        lines: MAX_CONTENT_LINES,
        line_heights: [10.0, 10.0],
    },
];

pub(super) fn fixture(case: &Case) -> Fixture {
    let root_style = format!(
        "box-sizing:content-box;display:grid;width:{};height:max-content;margin:0;padding:0;border:2px solid #f0c674;column-gap:2px;row-gap:2px;align-content:start;justify-content:start;grid-template-columns:repeat(3,{TRACK});grid-template-rows:repeat(2,{TRACK});background-color:#101820;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d",
        case.root_width
    );
    let item_styles = case.order.iter().map(|item| {
        let (width, min_width) = if item.key == "b" { ("40px", "40px") } else { ("auto", "auto") };
        let text_style = if item.text.is_some() {
            "font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;color:#e5484d;"
        } else {
            ""
        };
        (
            item.id,
            item.tag,
            format!(
                "box-sizing:border-box;display:block;width:{width};height:auto;min-width:{min_width};min-height:auto;align-self:{};margin:0;padding:0;border:0;{};{text_style}background-color:#e5484d",
                if item.key == "b" { "start" } else { "auto" },
                item.placement
            ),
        )
    }).collect::<Vec<_>>();
    let item_style_refs = item_styles
        .iter()
        .map(|(id, tag, style)| (*id, *tag, style.as_str()))
        .collect::<Vec<_>>();
    let mut fixture = Fixture::new_with_item_tags(&root_style, &item_style_refs);
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_234).unwrap(),
        fixture.document.document_revision(),
    );
    for item in case.order {
        if item.key == "b" {
            batch.push(DocumentOperation::SetAttribute {
                node: fixture.nodes[item.id],
                name: AttributeName::new(None, "src").unwrap(),
                value: IMAGE_URL.to_owned().into(),
            });
        }
        if let Some(text) = item.text {
            let text_node = fixture.document.reserve_node_handle().unwrap();
            batch.push(DocumentOperation::CreateText {
                node: text_node,
                data: text.to_owned().into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent: HostParent::Node(fixture.nodes[item.id]),
                node: text_node,
                before: None,
            });
        }
    }
    fixture.document.commit(batch).unwrap();
    fixture
}

pub(super) fn compute(fixture: &Fixture) -> crate::StyleLayoutOutput {
    compute_with_projection(fixture, true).unwrap()
}

pub(super) fn compute_without_fixture_alignment_opt_in(
    fixture: &Fixture,
) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
    compute_with_projection(fixture, false)
}

fn compute_with_projection(
    fixture: &Fixture,
    allow_two_axis_grid_fixture: bool,
) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
    let snapshot = fixture.document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), fixture.root)
            .unwrap();
    let viewport = CssViewport {
        width_css_px: 240.0,
        height_css_px: 160.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let computed_styles = compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        &view,
        &[],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    let paragraphs = BTreeMap::from([
        (fixture.nodes[A.id].id(), paragraph_style()),
        (fixture.nodes[C.id].id(), paragraph_style()),
    ]);
    let mut text_engine = TextLayoutEngine::new_without_system_fonts();
    text_engine.register_font("Ahem", AHEM).unwrap();
    let intrinsic_sizes = LayoutIntrinsicSizeSnapshot::new(
        LayoutSourceRevision::HostDocument {
            generation: snapshot.generation(),
            document: snapshot.document_revision(),
            render_tree: snapshot.render_tree_revision(),
        },
        IntrinsicSizeRevision::INITIAL.checked_next().unwrap(),
        [(
            fixture.nodes[B.id].id(),
            LayoutIntrinsicSize::new(24.0, 16.0),
        )],
    )
    .unwrap();
    if allow_two_axis_grid_fixture {
        compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes_and_two_axis_grid_fixture(
            &snapshot, fixture.root, computed_styles, viewport, &paragraphs, &text_engine, intrinsic_sizes,
        )
    } else {
        compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
            &snapshot,
            fixture.root,
            computed_styles,
            viewport,
            &paragraphs,
            &text_engine,
            intrinsic_sizes,
        )
    }
}

fn paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        white_space: WhiteSpaceMode::Normal,
        word_break: WordBreakMode::Normal,
        ..ParagraphStyle::default()
    }
}

pub(super) fn assert_frame(actual: [f32; 4], expected: [f32; 4], scene: &str, label: &str) {
    for (index, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert_close(
            actual,
            expected,
            scene,
            &format!("{label} frame component {index}"),
        );
    }
}

pub(super) fn assert_size(actual: [f32; 2], expected: [f32; 2], scene: &str, label: &str) {
    for (index, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert_close(
            actual,
            expected,
            scene,
            &format!("{label} component {index}"),
        );
    }
}

pub(super) fn assert_close(actual: f32, expected: f32, scene: &str, label: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{scene}: {label}: expected {expected}, got {actual}"
    );
}

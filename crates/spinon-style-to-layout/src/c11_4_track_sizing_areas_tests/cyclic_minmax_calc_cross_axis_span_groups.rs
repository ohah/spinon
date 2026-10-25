use std::{collections::BTreeMap, sync::Arc};

use spinon_core::{DocumentChangeBatch, DocumentOperation, HostParent, OwnerId, StyleRevision};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, ParagraphStyle, TextLayoutEngine, WhiteSpaceMode,
    WordBreakMode,
};

use super::{Fixture, compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets};
use crate::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs;

const AHEM: &[u8] = include_bytes!("../../../../tests/fixtures/resources/c15/Ahem.ttf");
const TRACK: &str = "minmax(calc(10% + 2px),max-content)";

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    text: &'static str,
    placement: &'static str,
    width: &'static str,
    height: &'static str,
}

const A: Item = Item {
    id: "item-a",
    text: "AA A A",
    placement: "grid-column:1 / 3;grid-row:1 / 3",
    width: "auto",
    height: "auto",
};
const B: Item = Item {
    id: "item-b",
    text: "XXXX",
    placement: "grid-column:2 / 4;grid-row:1 / 3",
    width: "40px",
    height: "10px",
};

struct Case {
    id: &'static str,
    width: &'static str,
    order: [Item; 2],
    root: [f32; 2],
    columns: [f32; 3],
    rows: [f32; 2],
    item_a: [f32; 4],
    item_b: [f32; 4],
    item_a_rows: f32,
    item_a_minimum: f32,
}

const INTRINSIC_COLUMNS: [f32; 3] = [15.671875, 15.671875, 15.65625];
const CASES: [Case; 4] = [
    Case {
        id: "cyclic-minmax-calc-cross-axis-span-groups-intrinsic-forward",
        width: "min-content",
        order: [A, B],
        root: [55.0, 24.0],
        columns: INTRINSIC_COLUMNS,
        rows: [9.0, 9.0],
        item_a: [2.0, 2.0, 33.34375, 20.0],
        item_b: [19.671875, 2.0, 40.0, 10.0],
        item_a_rows: 20.0,
        item_a_minimum: 20.0,
    },
    Case {
        id: "cyclic-minmax-calc-cross-axis-span-groups-intrinsic-reverse",
        width: "min-content",
        order: [B, A],
        root: [55.0, 24.0],
        columns: INTRINSIC_COLUMNS,
        rows: [9.0, 9.0],
        item_a: [2.0, 2.0, 33.34375, 20.0],
        item_b: [19.671875, 2.0, 40.0, 10.0],
        item_a_rows: 20.0,
        item_a_minimum: 20.0,
    },
    Case {
        id: "definite-minmax-calc-cross-axis-span-groups-forward",
        width: "40px",
        order: [A, B],
        root: [44.0, 34.0],
        columns: [12.0, 12.0, 12.0],
        rows: [14.0, 14.0],
        item_a: [2.0, 2.0, 26.0, 30.0],
        item_b: [16.0, 2.0, 40.0, 10.0],
        item_a_rows: 30.0,
        item_a_minimum: 0.0,
    },
    Case {
        id: "definite-minmax-calc-cross-axis-span-groups-reverse",
        width: "40px",
        order: [B, A],
        root: [44.0, 34.0],
        columns: [12.0, 12.0, 12.0],
        rows: [14.0, 14.0],
        item_a: [2.0, 2.0, 26.0, 30.0],
        item_b: [16.0, 2.0, 40.0, 10.0],
        item_a_rows: 30.0,
        item_a_minimum: 0.0,
    },
];

fn fixture(case: &Case) -> Fixture {
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

fn compute(fixture: &Fixture) -> crate::StyleLayoutOutput {
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
    let paragraphs = BTreeMap::from([A.id, B.id].map(|id| {
        (
            fixture.nodes[id].id(),
            ParagraphStyle {
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

#[test]
fn cyclic_minmax_calc_cross_axis_span_groups_are_source_order_independent() {
    for case in &CASES {
        let fixture = fixture(case);
        let output = compute(&fixture);
        let root = output.layout.frames[&fixture.root.id()];
        let item_a = output.layout.frames[&fixture.nodes[A.id].id()];
        let item_b = output.layout.frames[&fixture.nodes[B.id].id()];
        assert_frame(
            [root.width, root.height],
            case.root,
            case.id,
            "root border box",
        );
        assert_frame(
            [item_a.x, item_a.y, item_a.width, item_a.height],
            case.item_a,
            case.id,
            "item A frame",
        );
        assert_frame(
            [item_b.x, item_b.y, item_b.width, item_b.height],
            case.item_b,
            case.id,
            "item B frame",
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Columns,
            &case.columns,
            case.id,
        );
        assert_axis_tracks(
            &output,
            fixture.root.id(),
            LayoutGridTrackSizingAxis::Rows,
            &case.rows,
            case.id,
        );
        assert_item_intrinsic_sizes(&output, &fixture, case.id);
        assert_item_contributions(&output, &fixture, case);
        if case.width == "min-content" {
            assert_min_content_constraint_bases(&output, &fixture, case.id);
        }
    }
}

fn assert_item_intrinsic_sizes(output: &crate::StyleLayoutOutput, fixture: &Fixture, scene: &str) {
    for (id, expected_min, expected_max, expected_min_height) in
        [(A.id, 20.0, 60.0, 30.0), (B.id, 40.0, 40.0, 10.0)]
    {
        let input = output
            .layout
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == fixture.nodes[id].id())
            .unwrap_or_else(|| panic!("{scene}: missing intrinsic input for {id}"));
        let LayoutGridItemIntrinsicSource::Text { sizes } = &input.source else {
            panic!("{scene}: {id} must use direct text intrinsic input");
        };
        assert_close(
            sizes.min_content(),
            expected_min,
            scene,
            &format!("{id} min-content"),
        );
        assert_close(
            sizes.max_content(),
            expected_max,
            scene,
            &format!("{id} max-content"),
        );
        assert_close(
            sizes.min_content_height(),
            expected_min_height,
            scene,
            &format!("{id} min-content block contribution"),
        );
        assert_close(
            sizes.max_content_height(),
            10.0,
            scene,
            &format!("{id} max-content block contribution"),
        );
        let expected_order = if scene.ends_with("reverse") {
            usize::from(id == A.id)
        } else {
            usize::from(id == B.id)
        };
        assert_eq!(
            input.source_order, expected_order,
            "{scene}: {id} source order"
        );
        assert!(
            sizes.font_registry_revision().0 > 0,
            "{scene}: {id} font revision"
        );
    }
}

fn assert_item_contributions(output: &crate::StyleLayoutOutput, fixture: &Fixture, case: &Case) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .unwrap_or_else(|| panic!("{}: missing Grid track-sizing trace", case.id));
    let columns = trace
        .passes
        .iter()
        .rposition(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{}: missing Columns pass", case.id));
    let rows = trace
        .passes
        .iter()
        .rposition(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .unwrap_or_else(|| panic!("{}: missing Rows pass", case.id));
    assert!(columns < rows, "{}: Columns must size before Rows", case.id);
    let expected_items = [
        (A.id, fixture.nodes[A.id].id()),
        (B.id, fixture.nodes[B.id].id()),
    ];
    for (axis, kind, values) in [
        (
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingContributionKind::MinContent,
            [(A.id, 20.0), (B.id, 40.0)],
        ),
        (
            LayoutGridTrackSizingAxis::Columns,
            LayoutGridTrackSizingContributionKind::MaxContent,
            [(A.id, 60.0), (B.id, 40.0)],
        ),
        (
            LayoutGridTrackSizingAxis::Rows,
            LayoutGridTrackSizingContributionKind::MinContent,
            [(A.id, case.item_a_rows), (B.id, 10.0)],
        ),
        (
            LayoutGridTrackSizingAxis::Rows,
            LayoutGridTrackSizingContributionKind::MaxContent,
            [(A.id, case.item_a_rows), (B.id, 10.0)],
        ),
    ] {
        let passes = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == axis)
            .collect::<Vec<_>>();
        assert!(!passes.is_empty(), "{}: missing {axis:?} pass", case.id);
        for (item_key, value) in values {
            let item_node = expected_items
                .iter()
                .find(|(id, _)| *id == item_key)
                .unwrap()
                .1;
            assert!(
                passes
                    .iter()
                    .any(|pass| pass.contributions.iter().any(|contribution| {
                        contribution.item == item_node
                            && contribution.kind == kind
                            && contribution.span == 2
                            && (contribution.value - value).abs() <= 0.05
                    })),
                "{}: {axis:?} {item_key} {kind:?} span-2 contribution expected {value}",
                case.id
            );
        }
    }
    for (axis, item_key, value) in [
        (
            LayoutGridTrackSizingAxis::Columns,
            A.id,
            case.item_a_minimum,
        ),
        (LayoutGridTrackSizingAxis::Columns, B.id, 40.0),
        (LayoutGridTrackSizingAxis::Rows, A.id, case.item_a_minimum),
        (LayoutGridTrackSizingAxis::Rows, B.id, 10.0),
    ] {
        let item_node = expected_items
            .iter()
            .find(|(id, _)| *id == item_key)
            .unwrap()
            .1;
        assert!(
            trace
                .passes
                .iter()
                .filter(|pass| pass.axis == axis)
                .any(|pass| {
                    pass.contributions.iter().any(|contribution| {
                        contribution.item == item_node
                            && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
                            && contribution.span == 2
                            && (contribution.value - value).abs() <= 0.05
                    })
                }),
            "{}: {axis:?} {item_key} Minimum contribution expected {value}",
            case.id
        );
    }
}

fn assert_min_content_constraint_bases(
    output: &crate::StyleLayoutOutput,
    fixture: &Fixture,
    scene: &str,
) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == fixture.root.id())
        .unwrap_or_else(|| panic!("{scene}: missing Grid trace"));
    let columns = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{scene}: initial Columns pass missing"));
    let initialized = columns
        .stages
        .iter()
        .find(|stage| stage.stage == spinon_layout::LayoutGridTrackSizingStageKind::Initialize)
        .unwrap_or_else(|| panic!("{scene}: initial Columns state missing"))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    let min_content_batch = columns
        .stages
        .iter()
        .find(|stage| {
            matches!(
                stage.stage,
                spinon_layout::LayoutGridTrackSizingStageKind::IntrinsicItemBatch { span: 2, .. }
            )
        })
        .unwrap_or_else(|| panic!("{scene}: min-content span batch bases missing"));
    let observed = min_content_batch
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(initialized.len(), 3, "{scene}: initialize column count");
    for (index, expected) in [2.0, 2.0, 2.0].into_iter().enumerate() {
        assert_close(
            initialized[index],
            expected,
            scene,
            &format!("cyclic calc initial base {}", index + 1),
        );
    }
    assert_eq!(observed.len(), 3, "{scene}: min-content column count");
    for (index, expected) in [9.0, 19.0, 19.0].into_iter().enumerate() {
        assert_close(
            observed[index],
            expected,
            scene,
            &format!("min-content base {}", index + 1),
        );
    }
    let minimum_for = |node| {
        columns
            .contributions
            .iter()
            .find(|contribution| {
                contribution.item == node
                    && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
                    && contribution.span == 2
            })
            .map(|contribution| contribution.value)
            .unwrap_or_else(|| panic!("{scene}: item {node:?} Minimum contribution missing"))
    };
    let planned_a =
        (minimum_for(fixture.nodes[A.id].id()) - initialized[0] - initialized[1] - 2.0) / 2.0;
    let planned_b =
        (minimum_for(fixture.nodes[B.id].id()) - initialized[1] - initialized[2] - 2.0) / 2.0;
    assert_close(
        planned_a,
        7.0,
        scene,
        "item A planned increase per spanned track",
    );
    assert_close(
        planned_b,
        17.0,
        scene,
        "item B planned increase per spanned track",
    );
    // CSS Grid §11.5.1 keeps the per-track maximum from the overlapping item increases.
    let planned_increases = [
        observed[0] - initialized[0],
        observed[1] - initialized[1],
        observed[2] - initialized[2],
    ];
    for (index, expected) in [planned_a, planned_b, planned_b].into_iter().enumerate() {
        assert_close(
            planned_increases[index],
            expected,
            scene,
            &format!("per-track planned increase {}", index + 1),
        );
    }
}

fn assert_axis_tracks(
    output: &crate::StyleLayoutOutput,
    grid: spinon_core::NodeId,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
    scene: &str,
) {
    let trace = output
        .layout
        .grid_track_sizing_traces
        .iter()
        .find(|trace| trace.node == grid)
        .unwrap_or_else(|| panic!("{scene}: missing Grid track-sizing trace"));
    let pass = trace
        .passes
        .iter()
        .rfind(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} pass"));
    let stage = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} stage"));
    let tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    assert_eq!(
        tracks.len(),
        expected.len(),
        "{scene}: {axis:?} track count"
    );
    for (index, (track, expected)) in tracks.iter().zip(expected).enumerate() {
        assert_close(
            track.base_size,
            *expected,
            scene,
            &format!("{axis:?} track {}", index + 1),
        );
    }
}

fn assert_frame<const N: usize>(actual: [f32; N], expected: [f32; N], scene: &str, label: &str) {
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_close(
            *actual,
            expected,
            scene,
            &format!("{label} component {index}"),
        );
    }
}

fn assert_close(actual: f32, expected: f32, scene: &str, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{scene}: {label} expected {expected}, got {actual}"
    );
}

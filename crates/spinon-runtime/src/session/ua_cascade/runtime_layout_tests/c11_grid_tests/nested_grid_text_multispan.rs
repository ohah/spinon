use super::*;

use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingTrace,
};

#[path = "nested_grid_text_multispan/fixture.rs"]
mod fixture;
use fixture::{Mutation, fixture};

#[derive(Clone, Copy)]
struct Item {
    suffix: &'static str,
    text: &'static str,
    column: u8,
    span: u8,
    row: u8,
    min_content: f32,
    max_content: f32,
}

#[derive(Clone, Copy)]
pub(super) struct Scene {
    id: &'static str,
    items: &'static [Item],
    columns: &'static [f32],
    width: f32,
}

const A: Item = Item {
    suffix: "a",
    text: "XX XXXXX",
    column: 1,
    span: 2,
    row: 1,
    min_content: 50.0,
    max_content: 80.0,
};
const B: Item = Item {
    suffix: "b",
    text: "XXXX XXXXXXX",
    column: 2,
    span: 2,
    row: 2,
    min_content: 70.0,
    max_content: 120.0,
};
const C: Item = Item {
    suffix: "c",
    text: "XXXXXXXXXXXXXXXXXX",
    column: 1,
    span: 3,
    row: 3,
    min_content: 180.0,
    max_content: 180.0,
};
const TWO_ITEM_COLUMNS: [f32; 4] = [25.0, 35.0, 35.0, 0.0];
const THREE_ITEM_COLUMNS: [f32; 4] = [160.0 / 3.0, 190.0 / 3.0, 190.0 / 3.0, 0.0];
const SCENES: [Scene; 4] = [
    Scene {
        id: "nested-text-span2-forward",
        items: &[A, B],
        columns: &TWO_ITEM_COLUMNS,
        width: 95.0,
    },
    Scene {
        id: "nested-text-span2-reversed",
        items: &[B, A],
        columns: &TWO_ITEM_COLUMNS,
        width: 95.0,
    },
    Scene {
        id: "nested-text-span2-before-span3",
        items: &[A, B, C],
        columns: &THREE_ITEM_COLUMNS,
        width: 180.0,
    },
    Scene {
        id: "nested-text-span3-before-span2",
        items: &[C, B, A],
        columns: &THREE_ITEM_COLUMNS,
        width: 180.0,
    },
];

#[test]
fn c114_nested_grid_text_multispan_matches_independent_track_and_frame_expectations() {
    for (index, scene) in SCENES.iter().enumerate() {
        run_scene(scene, 12_400 + index as u64);
    }
}

#[test]
fn c114_nested_grid_text_multispan_rejects_unlisted_and_malformed_inputs() {
    let scene = &SCENES[0];
    for (index, mutation) in [
        Mutation::UnlistedScene,
        Mutation::WrongItemPlacement,
        Mutation::WrongText,
        Mutation::WrongGridTracks,
        Mutation::ExtraInnerItem,
    ]
    .into_iter()
    .enumerate()
    {
        let (snapshot, _) = fixture(scene, 12_500 + index as u64, mutation);
        let result = compute_request_for_block_flex_grid_track_sizing(&request(snapshot));
        let rejected = match result {
            Err(_) => true,
            Ok(calculation) => calculation.layout.is_err(),
        };
        assert!(rejected, "{mutation:?} input must fail closed");
    }
}

fn run_scene(scene: &Scene, owner: u64) {
    let (snapshot, nodes) = fixture(scene, owner, Mutation::Valid);
    let calculation = compute_request_for_block_flex_grid_track_sizing(&request(snapshot))
        .unwrap_or_else(|error| panic!("{}: calculation failed: {error:?}", scene.id));
    let layout = calculation
        .layout
        .unwrap_or_else(|error| panic!("{}: layout failed: {error:?}", scene.id));
    let frame = |node: HostNodeHandle| {
        layout
            .frames
            .iter()
            .find(|frame| frame.node_id == node.id().get())
            .unwrap_or_else(|| panic!("{}: missing frame {}", scene.id, node.id().get()))
    };

    for grid in &nodes.grids {
        assert_close(frame(*grid).x, 0.0, scene.id, "nested Grid x");
        assert_close(frame(*grid).y, 0.0, scene.id, "nested Grid y");
        assert_close(
            frame(*grid).width,
            scene.width,
            scene.id,
            "nested Grid width",
        );
        assert_close(frame(*grid).height, 60.0, scene.id, "nested Grid height");
    }
    let diagnostics = layout
        .grid_track_sizing_diagnostics
        .as_ref()
        .unwrap_or_else(|| panic!("{}: missing track sizing trace", scene.id));
    assert_eq!(
        diagnostics.traces.len(),
        4,
        "{}: one trace per Grid",
        scene.id
    );

    let inner_trace = trace_for(diagnostics, nodes.grids[3]);
    assert_tracks(
        inner_trace,
        LayoutGridTrackSizingAxis::Columns,
        scene.columns,
        scene.id,
    );
    assert_tracks(
        inner_trace,
        LayoutGridTrackSizingAxis::Rows,
        &[20.0, 20.0, 20.0],
        scene.id,
    );
    for (source_order, (item, item_node)) in scene.items.iter().zip(&nodes.items).enumerate() {
        let input = diagnostics
            .item_inputs
            .iter()
            .filter(|input| input.node == item_node.id())
            .collect::<Vec<_>>();
        assert_eq!(
            input.len(),
            1,
            "{}: one intrinsic input per text item",
            scene.id
        );
        let input = input[0];
        assert_eq!(input.grid_container, nodes.grids[3].id());
        assert_eq!(input.source_order, source_order);
        let LayoutGridItemIntrinsicSource::Text { sizes } = &input.source else {
            panic!(
                "{}: item {} must be measured as text",
                scene.id, item.suffix
            );
        };
        assert_close(
            sizes.min_content(),
            item.min_content,
            scene.id,
            "text min-content",
        );
        assert_close(
            sizes.max_content(),
            item.max_content,
            scene.id,
            "text max-content",
        );
        assert_eq!(
            sizes.font_registry_revision().0,
            1,
            "{}: Ahem revision",
            scene.id
        );

        let expected_x = scene.columns[..item.column as usize - 1]
            .iter()
            .sum::<f32>();
        let expected_width = scene.columns
            [item.column as usize - 1..(item.column + item.span) as usize - 1]
            .iter()
            .sum::<f32>();
        assert_close(frame(*item_node).x, expected_x, scene.id, "text item x");
        assert_close(
            frame(*item_node).y,
            (item.row as f32 - 1.0) * 20.0,
            scene.id,
            "text item y",
        );
        assert_close(
            frame(*item_node).width,
            expected_width,
            scene.id,
            "text item width",
        );
        let height = if scene.width == 95.0 { 20.0 } else { 10.0 };
        assert_close(
            frame(*item_node).height,
            height,
            scene.id,
            "text item height",
        );

        let contributions = inner_trace
            .passes
            .iter()
            .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .flat_map(|pass| &pass.contributions)
            .filter(|contribution| contribution.item == item_node.id())
            .collect::<Vec<_>>();
        assert!(
            contributions.iter().any(|contribution| {
                contribution.kind == LayoutGridTrackSizingContributionKind::MinContent
                    && contribution.span == item.span as u16
                    && (contribution.value - item.min_content).abs() <= 1.0 / 64.0
            }),
            "{}: missing span-{} min-content trace for {}: {contributions:#?}",
            scene.id,
            item.span,
            item.suffix,
        );
    }

    let expected_contribution = LayoutGridTrackSizingContributionKind::MinContent;
    for index in 0..3 {
        let trace = trace_for(diagnostics, nodes.grids[index]);
        assert_tracks(
            trace,
            LayoutGridTrackSizingAxis::Columns,
            &[scene.width],
            scene.id,
        );
        assert_tracks(trace, LayoutGridTrackSizingAxis::Rows, &[60.0], scene.id);
        let child = nodes.grids[index + 1];
        let contributions = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .flat_map(|pass| &pass.contributions)
            .filter(|contribution| contribution.item == child.id())
            .collect::<Vec<_>>();
        assert!(
            contributions.iter().any(|contribution| {
                contribution.kind == expected_contribution
                    && contribution.span == 1
                    && (contribution.value - scene.width).abs() <= 1.0 / 64.0
            }),
            "{}: nested Grid {} must contribute once through parent {}: {contributions:#?}",
            scene.id,
            child.id().get(),
            nodes.grids[index].id().get(),
        );
    }
}

fn request(snapshot: Arc<HostDocumentSnapshot>) -> WorkRequest {
    WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 400.0,
            height_css_px: 200.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    }
}

fn trace_for(
    diagnostics: &crate::session::ua_cascade::runtime_layout::RuntimeGridTrackSizingDiagnostics,
    grid: HostNodeHandle,
) -> &LayoutGridTrackSizingTrace {
    diagnostics
        .traces
        .iter()
        .find(|trace| trace.node == grid.id())
        .unwrap_or_else(|| panic!("missing Grid trace {}", grid.id().get()))
}

fn assert_tracks(
    trace: &LayoutGridTrackSizingTrace,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
    scene: &str,
) {
    let pass = trace
        .passes
        .iter()
        .rev()
        .find(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{scene}: missing {axis:?} pass for {}", trace.node.get()));
    let actual = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{scene}: missing final {axis:?} stage"))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(
        actual.len(),
        expected.len(),
        "{scene}: {axis:?} track count"
    );
    for (actual, expected) in actual.iter().zip(expected) {
        assert_close(*actual, *expected, scene, &format!("{axis:?} track"));
    }
}

fn assert_close(actual: f32, expected: f32, scene: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{scene}: {property} expected {expected}, got {actual}"
    );
}

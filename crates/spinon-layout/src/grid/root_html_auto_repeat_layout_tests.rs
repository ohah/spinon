use super::{grid_input, id};
use crate::{
    GridAutoRepeat, GridAutoRepeatMode, GridPlacementMode, GridTrackMaxSizingFunction,
    GridTrackMinSizingFunction, GridTrackSize, LayoutDimension, LayoutEngine, LayoutError,
    LayoutGap, LayoutLengthPercentage, TaffyLayoutEngine,
};

fn root_html_auto_repeat_input(
    minima: &[f32],
    mode: GridAutoRepeatMode,
    width: f32,
    gap: f32,
    item_count: usize,
    item_width: LayoutDimension,
) -> crate::LayoutInput {
    let columns = minima
        .iter()
        .copied()
        .map(|minimum| GridTrackSize::MinMax {
            min: GridTrackMinSizingFunction::LengthPx(minimum),
            max: GridTrackMaxSizingFunction::Fraction(1.0),
        })
        .collect::<Vec<_>>();
    let repeat = GridAutoRepeat {
        mode,
        start: 0,
        end: columns.len(),
        line_names: Vec::new(),
    };
    let mut input = grid_input(
        columns.clone(),
        vec![GridTrackSize::LengthPx(24.0)],
        item_count,
        LayoutGap {
            column: LayoutLengthPercentage::LengthPx(gap),
            ..LayoutGap::default()
        },
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.nodes[0].style.height = LayoutDimension::Fixed(24.0);
    input.viewport.width = width;
    input.viewport.height = 24.0;
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns_auto_repeat = Some(repeat);
    container.placement_mode = GridPlacementMode::RootHtmlAutoRepeatV1;
    for item in input.nodes.iter_mut().skip(1) {
        item.style.width = item_width;
        item.style.height = LayoutDimension::Fixed(10.0);
    }
    input
}

#[test]
fn auto_fill_zero_minimum_uses_the_ua_floor_for_count_and_keeps_track_origins() {
    let input = root_html_auto_repeat_input(
        &[0.0],
        GridAutoRepeatMode::AutoFill,
        11.0,
        1.0,
        6,
        LayoutDimension::Fixed(0.0),
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    for (index, child) in (2..=7).map(id).enumerate() {
        assert_eq!(output.frames[&child].x, (index * 2) as f32);
    }
}

#[test]
fn auto_fit_zero_minimum_collapses_empty_tracks_after_item_placement() {
    let input = root_html_auto_repeat_input(
        &[0.0],
        GridAutoRepeatMode::AutoFit,
        11.0,
        0.0,
        1,
        LayoutDimension::Auto,
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let item = output.frames[&id(2)];
    assert_eq!((item.x, item.width), (0.0, 11.0));
}

#[test]
fn over_limit_zero_minimum_rejects_before_producing_layout_output() {
    let input = root_html_auto_repeat_input(
        &[0.0],
        GridAutoRepeatMode::AutoFill,
        10_001.0,
        0.0,
        0,
        LayoutDimension::Fixed(0.0),
    );

    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::GridTrackExpansionOverflow(node)) if node == id(1)
    ));
}

use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_layout::{
    GridContainerInput, GridPlacementMode, GridTrackMaxSizingFunction, GridTrackMinSizingFunction,
    GridTrackSize, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingMinimumContributionSource, LayoutGridTrackSizingPassTrace,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssPosition, ComputedElementStyle,
};

use super::super::super::{
    RuntimeC11_4AutoMinmaxSpanOverflowObservation, RuntimeLayoutCompleted, RuntimeLayoutFailure,
    layout_failure,
};
use super::super::c11_4_nested_grid_replaced_item::{spacing_is, zero_edges};
use super::{
    Case, GRID_HEIGHT, GRID_WIDTH, MAX_CONTENT_CONTRIBUTION, failure, single_track, unsupported,
};

pub(in crate::session::ua_cascade::runtime_layout) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| failure("Grid track sizing diagnostics are missing"))?;
    let span_observation = diagnostics.c11_4_overflow_auto_minimum_fixture.as_ref();
    let single_track_observation = diagnostics
        .c11_4_single_track_overflow_auto_minimum_fixture
        .as_ref();
    match (span_observation, single_track_observation) {
        (None, Some(_)) => return single_track::validate_output(completed),
        (Some(_), Some(_)) => return Err(failure("multiple overflow fixture observations")),
        (None, None) => return Err(failure("overflow fixture observation is missing")),
        (Some(_), None) => {}
    }
    let observation = span_observation.expect("checked span fixture observation");
    if observation.style_snapshot.style_revision != completed.key.style_revision
        || observation.style_snapshot.generation.get() != completed.key.generation
        || observation.style_snapshot.document_revision.get() != completed.key.document_revision
        || observation.style_snapshot.render_tree_revision.get()
            != completed.key.render_tree_revision
    {
        return Err(failure("style and layout revisions differ"));
    }
    validate_grid_input(
        observation.grid_root,
        &observation.grid_container_input,
        &observation.scene_id,
    )?;
    let frames = completed
        .frames
        .iter()
        .map(|frame| (frame.node_id, (frame.x, frame.y, frame.width, frame.height)))
        .collect::<BTreeMap<_, _>>();
    if completed.frames.len() != 2 || frames.len() != 2 {
        return Err(failure(
            "expected exactly one Grid root and one direct item frame",
        ));
    }
    for (node, expected) in [
        (observation.grid_root, (0.0, 0.0, GRID_WIDTH, GRID_HEIGHT)),
        (
            observation.item,
            (0.0, 0.0, observation.expected_item_width, GRID_HEIGHT),
        ),
    ] {
        if frames.get(&node.get()) != Some(&expected) {
            return Err(failure(
                "root or item frame differs from the fixed CSS-pixel geometry",
            ));
        }
    }
    let root_traces = diagnostics
        .traces
        .iter()
        .filter(|trace| trace.node == observation.grid_root)
        .collect::<Vec<_>>();
    if diagnostics.traces.len() != 1 || root_traces.len() != 1 || root_traces[0].passes.len() != 2 {
        return Err(failure(
            "expected one Grid trace with Columns and Rows passes",
        ));
    }
    let columns = root_traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .collect::<Vec<_>>();
    let rows = root_traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    if columns.len() != 1 || rows.len() != 1 {
        return Err(failure("expected exactly one Columns and one Rows pass"));
    }
    validate_columns(columns[0], observation)?;
    Ok(())
}

fn validate_columns(
    pass: &LayoutGridTrackSizingPassTrace,
    observation: &RuntimeC11_4AutoMinmaxSpanOverflowObservation,
) -> Result<(), RuntimeLayoutFailure> {
    let final_stage = pass
        .stages
        .last()
        .ok_or_else(|| failure("final Columns stage is missing"))?;
    let tracks = final_stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    if tracks.len() != 2
        || tracks
            .iter()
            .zip(observation.expected_track_base_sizes)
            .any(|(track, expected)| track.base_size != expected)
    {
        return Err(failure("final two track base sizes differ"));
    }
    let minimums = pass
        .contributions
        .iter()
        .filter(|contribution| {
            contribution.item == observation.item
                && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
        })
        .collect::<Vec<_>>();
    if minimums.len() != 1 || minimums[0].value != observation.expected_minimum_value {
        return Err(failure("item Minimum contribution is missing or differs"));
    }
    let source_matches = if observation.expects_content_based_minimum {
        matches!(
            minimums[0].minimum_source,
            Some(
                LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                    spans_auto_min_track: true,
                    spans_multiple_tracks: true,
                    spans_flexible_max_track: false,
                    content_based: true,
                    ..
                }
            )
        )
    } else {
        minimums[0].minimum_source
            == Some(LayoutGridTrackSizingMinimumContributionSource::OverflowAutomaticMinimum)
    };
    if !source_matches {
        return Err(failure("item Minimum contribution source differs"));
    }
    let max_contents = pass
        .contributions
        .iter()
        .filter(|contribution| {
            contribution.item == observation.item
                && contribution.kind == LayoutGridTrackSizingContributionKind::MaxContent
        })
        .collect::<Vec<_>>();
    if max_contents.len() != 1 || max_contents[0].value != MAX_CONTENT_CONTRIBUTION {
        return Err(failure("item MaxContent contribution must remain 100px"));
    }
    Ok(())
}

pub(super) fn validate_root_style(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "c114_overflow_auto_minimum_missing_grid_style",
            Some(style.node_id.get()),
            Some(scene_id.to_owned()),
        )
    })?;
    let expected_track = ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::Auto,
        max: ComputedCssGridTrackBreadth::MaxContent,
    };
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(GRID_WIDTH)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(GRID_HEIGHT)
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || !spacing_is(style.layout_spacing.column_gap, 0.0)
        || style.layout_border != Default::default()
        || grid.columns != [expected_track.clone(), expected_track]
        || grid.rows != [ComputedCssGridTrackSize::LengthPx(GRID_HEIGHT)]
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
    {
        return Err(unsupported(
            style.node_id,
            scene_id,
            "Grid root computed style",
        ));
    }
    Ok(())
}

pub(super) fn validate_item_style(
    style: &ComputedElementStyle,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    let placement = style.grid_item_placement.as_ref().ok_or_else(|| {
        layout_failure(
            "c114_overflow_auto_minimum_missing_item_placement",
            Some(style.node_id.get()),
            Some(case.scene_id.to_owned()),
        )
    })?;
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_position != ComputedCssPosition::Static
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.font_size_css_px != 10.0
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
        || placement.row_start != ComputedCssGridPlacement::Line(1)
        || placement.row_end != ComputedCssGridPlacement::Auto
        || placement.column_start != ComputedCssGridPlacement::Line(1)
        || placement.column_end != ComputedCssGridPlacement::Span(2)
        || style.layout_overflow_x != Some(case.computed_overflow_x)
        || style.layout_overflow_y != Some(case.computed_overflow_y)
    {
        return Err(unsupported(
            style.node_id,
            case.scene_id,
            &format!(
                "item computed style or overflow: position={:?} dimensions={:?} font_size={} spacing={:?} border={:?} placement={:?} overflow=({:?},{:?})",
                style.layout_position,
                style.layout_dimensions,
                style.font_size_css_px,
                style.layout_spacing,
                style.layout_border,
                placement,
                style.layout_overflow_x,
                style.layout_overflow_y,
            ),
        ));
    }
    Ok(())
}

pub(super) fn validate_grid_input(
    root: NodeId,
    input: &GridContainerInput,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let track = GridTrackSize::MinMax {
        min: GridTrackMinSizingFunction::Auto,
        max: GridTrackMaxSizingFunction::MaxContent,
    };
    if input.columns != [track, track]
        || input.rows != [GridTrackSize::LengthPx(GRID_HEIGHT)]
        || input.placement_mode != GridPlacementMode::TrackSizing
        || input.auto_flow != spinon_layout::GridAutoFlow::Row
        || input.auto_rows != spinon_layout::GridAutoRows::Auto
        || input.auto_columns != spinon_layout::GridAutoColumns::Auto
        || input
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || input.rows_line_names.iter().any(|names| !names.is_empty())
        || input.columns_auto_repeat.is_some()
        || input.rows_auto_repeat.is_some()
        || input.template_areas.is_some()
    {
        return Err(unsupported(
            root,
            scene_id,
            "typed LayoutInput Grid container",
        ));
    }
    Ok(())
}

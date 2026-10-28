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
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedElementStyle,
};

use super::super::super::super::{
    RuntimeC11_4OverflowMinimumSource, RuntimeC11_4SingleTrackOverflowAutoMinimumObservation,
    RuntimeLayoutCompleted, RuntimeLayoutFailure,
};
use super::super::super::c11_4_nested_grid_replaced_item::{spacing_is, zero_edges};
use super::{
    COLUMNS_GRID_HEIGHT, COLUMNS_GRID_WIDTH, Case, ItemKind, MinimumSource, ROWS_GRID_HEIGHT,
    ROWS_GRID_WIDTH, failure, unsupported,
};
pub(in crate::session::ua_cascade::runtime_layout) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| failure("Grid track sizing diagnostics are missing"))?;
    let observation = diagnostics
        .c11_4_single_track_overflow_auto_minimum_fixture
        .as_ref()
        .ok_or_else(|| failure("single-track overflow fixture observation is missing"))?;
    if observation.style_snapshot.style_revision != completed.key.style_revision
        || observation.style_snapshot.generation.get() != completed.key.generation
        || observation.style_snapshot.document_revision.get() != completed.key.document_revision
        || observation.style_snapshot.render_tree_revision.get()
            != completed.key.render_tree_revision
    {
        return Err(failure("style and layout revisions differ"));
    }
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
        (
            observation.grid_root,
            (
                0.0,
                0.0,
                grid_width(observation.axis),
                grid_height(observation.axis),
            ),
        ),
        (
            observation.item,
            (
                0.0,
                0.0,
                observation.expected_item_width,
                observation.expected_item_height,
            ),
        ),
    ] {
        if frames.get(&node.get()) != Some(&expected) {
            return Err(failure(
                "root or item frame differs from fixed CSS-pixel geometry",
            ));
        }
    }
    let root_traces = diagnostics
        .traces
        .iter()
        .filter(|trace| trace.node == observation.grid_root)
        .collect::<Vec<_>>();
    if diagnostics.traces.len() != 1 || root_traces.len() != 1 {
        return Err(failure("expected exactly one Grid track-sizing trace"));
    }
    let passes = &root_traces[0].passes;
    let has_columns = passes
        .iter()
        .any(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns);
    let has_rows = passes
        .iter()
        .any(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows);
    if passes.len() < 2
        || passes[0].axis != LayoutGridTrackSizingAxis::Columns
        || !has_columns
        || !has_rows
        || passes
            .iter()
            .enumerate()
            .any(|(index, pass)| usize::from(pass.sequence) != index)
    {
        return Err(failure(
            "Grid trace must contain ordered Columns and Rows passes",
        ));
    }
    let fixed_track_size = if observation.axis == LayoutGridTrackSizingAxis::Columns {
        observation.expected_item_height
    } else {
        observation.expected_item_width
    };
    for pass in passes {
        if pass.axis == observation.axis {
            validate_pass(pass, observation)?;
        } else {
            validate_fixed_axis_pass(pass, fixed_track_size)?;
        }
    }
    Ok(())
}

fn validate_fixed_axis_pass(
    pass: &LayoutGridTrackSizingPassTrace,
    expected_track_size: f32,
) -> Result<(), RuntimeLayoutFailure> {
    let tracks = pass
        .stages
        .last()
        .ok_or_else(|| failure("final fixed-axis stage is missing"))?
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    if tracks.len() != 1 || tracks[0].base_size != expected_track_size {
        return Err(failure("fixed-axis track base size differs"));
    }
    if !pass.contributions.is_empty() {
        return Err(failure(
            "fixed-axis pass must not query item intrinsic contributions",
        ));
    }
    Ok(())
}

fn validate_pass(
    pass: &LayoutGridTrackSizingPassTrace,
    observation: &RuntimeC11_4SingleTrackOverflowAutoMinimumObservation,
) -> Result<(), RuntimeLayoutFailure> {
    let tracks = pass
        .stages
        .last()
        .ok_or_else(|| failure("final target stage is missing"))?
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    if tracks.len() != 1 || tracks[0].base_size != observation.expected_track_base_size {
        return Err(failure("final target track base size differs"));
    }
    if pass.contributions.iter().any(|contribution| {
        contribution.item != observation.item
            || contribution.span != 1
            || contribution.kind != LayoutGridTrackSizingContributionKind::Minimum
            || !contribution.crosses_flexible_track
    }) {
        return Err(failure(
            "target pass contains an unexpected item, span, contribution kind, or track maximum",
        ));
    }
    let minimums = pass
        .contributions
        .iter()
        .filter(|contribution| {
            contribution.item == observation.item
                && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
        })
        .collect::<Vec<_>>();
    match observation.expected_minimum_source {
        RuntimeC11_4OverflowMinimumSource::TrackMinimum => {
            if !minimums.is_empty() || observation.expected_minimum_value.is_some() {
                return Err(failure("zero track minimum must not query item Minimum"));
            }
        }
        expected_source => {
            let minimum = minimums
                .first()
                .ok_or_else(|| failure("item Minimum contribution is missing"))?;
            if minimums.len() != 1 || Some(minimum.value) != observation.expected_minimum_value {
                return Err(failure("item Minimum contribution count or value differs"));
            }
            let source_matches = match expected_source {
                RuntimeC11_4OverflowMinimumSource::ContentBased => matches!(
                    minimum.minimum_source,
                    Some(
                        LayoutGridTrackSizingMinimumContributionSource::AutomaticMinimum {
                            spans_auto_min_track: true,
                            spans_multiple_tracks: false,
                            spans_flexible_max_track: true,
                            content_based: true,
                            ..
                        }
                    )
                ),
                RuntimeC11_4OverflowMinimumSource::Overflow => minimum.minimum_source
                    == Some(
                        LayoutGridTrackSizingMinimumContributionSource::OverflowAutomaticMinimum,
                    ),
                RuntimeC11_4OverflowMinimumSource::Specified => {
                    minimum.minimum_source
                        == Some(LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum)
                }
                RuntimeC11_4OverflowMinimumSource::TrackMinimum => false,
            };
            if !source_matches {
                return Err(failure("item Minimum contribution source differs"));
            }
        }
    }
    let max_contents = pass
        .contributions
        .iter()
        .filter(|contribution| {
            contribution.item == observation.item
                && contribution.kind == LayoutGridTrackSizingContributionKind::MaxContent
        })
        .collect::<Vec<_>>();
    if !max_contents.is_empty() {
        let values = max_contents
            .iter()
            .map(|contribution| contribution.value)
            .collect::<Vec<_>>();
        return Err(failure(&format!(
            "flexible max unexpectedly requested item MaxContent contribution: count={}, values={values:?}",
            max_contents.len(),
        )));
    }
    Ok(())
}

pub(super) fn validate_root_style(
    style: &ComputedElementStyle,
    case: &Case,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| failure("Grid root style missing"))?;
    let track = ComputedCssGridTrackSize::MinMax {
        min: if case.zero_track_minimum {
            ComputedCssGridTrackBreadth::LengthPx(0.0)
        } else {
            ComputedCssGridTrackBreadth::Auto
        },
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    };
    let (columns, rows) = if case.axis == LayoutGridTrackSizingAxis::Columns {
        (
            vec![track],
            vec![ComputedCssGridTrackSize::LengthPx(COLUMNS_GRID_HEIGHT)],
        )
    } else {
        (vec![ComputedCssGridTrackSize::LengthPx(10.0)], vec![track])
    };
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("align-items").map(String::as_str) != Some("stretch")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(grid_width(case.axis))
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(grid_height(case.axis))
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || !spacing_is(style.layout_spacing.column_gap, 0.0)
        || style.layout_border != Default::default()
        || grid.columns != columns
        || grid.rows != rows
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
    {
        return Err(unsupported(
            style.node_id,
            case.scene_id,
            "Grid root computed style",
        ));
    }
    Ok(())
}

pub(super) fn validate_item_style(
    style: &ComputedElementStyle,
    case: &Case,
) -> Result<(), RuntimeLayoutFailure> {
    let placement = style
        .grid_item_placement
        .as_ref()
        .ok_or_else(|| failure("Grid item placement missing"))?;
    let expected_width = if matches!(case.item_kind, ItemKind::Image) {
        ComputedCssDimension::LengthPx(10.0)
    } else {
        ComputedCssDimension::Auto
    };
    let expected_min_width =
        if case.zero_item_minimum && case.axis == LayoutGridTrackSizingAxis::Columns {
            ComputedCssDimension::LengthPx(0.0)
        } else {
            ComputedCssDimension::Auto
        };
    let expected_min_height =
        if case.zero_item_minimum && case.axis == LayoutGridTrackSizingAxis::Rows {
            ComputedCssDimension::LengthPx(0.0)
        } else {
            ComputedCssDimension::Auto
        };
    if style.layout_position != spinon_style::ComputedCssPosition::Static
        || style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != expected_width
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != expected_min_width
        || style.layout_dimensions.min_height != expected_min_height
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
        || placement.row_start != ComputedCssGridPlacement::Line(1)
        || placement.row_end != ComputedCssGridPlacement::Line(2)
        || placement.column_start != ComputedCssGridPlacement::Line(1)
        || placement.column_end != ComputedCssGridPlacement::Line(2)
        || style.layout_overflow_x != Some(case.computed_overflow_x)
        || style.layout_overflow_y != Some(case.computed_overflow_y)
    {
        return Err(unsupported(
            style.node_id,
            case.scene_id,
            "Grid item computed style, placement, or overflow",
        ));
    }
    if matches!(case.item_kind, ItemKind::Text) && style.font_size_css_px != 10.0 {
        return Err(unsupported(
            style.node_id,
            case.scene_id,
            "fixed Ahem font size",
        ));
    }
    Ok(())
}

pub(super) fn validate_grid_input(
    root: NodeId,
    input: &GridContainerInput,
    case: &Case,
) -> Result<(), RuntimeLayoutFailure> {
    let track = GridTrackSize::MinMax {
        min: if case.zero_track_minimum {
            GridTrackMinSizingFunction::LengthPx(0.0)
        } else {
            GridTrackMinSizingFunction::Auto
        },
        max: GridTrackMaxSizingFunction::Fraction(1.0),
    };
    let (columns, rows) = if case.axis == LayoutGridTrackSizingAxis::Columns {
        (
            vec![track],
            vec![GridTrackSize::LengthPx(COLUMNS_GRID_HEIGHT)],
        )
    } else {
        (vec![GridTrackSize::LengthPx(10.0)], vec![track])
    };
    if input.columns != columns
        || input.rows != rows
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
            case.scene_id,
            "typed single-track LayoutInput",
        ));
    }
    Ok(())
}

pub(super) fn to_runtime_source(source: MinimumSource) -> RuntimeC11_4OverflowMinimumSource {
    match source {
        MinimumSource::ContentBased => RuntimeC11_4OverflowMinimumSource::ContentBased,
        MinimumSource::Overflow => RuntimeC11_4OverflowMinimumSource::Overflow,
        MinimumSource::Specified => RuntimeC11_4OverflowMinimumSource::Specified,
        MinimumSource::TrackMinimum => RuntimeC11_4OverflowMinimumSource::TrackMinimum,
    }
}

fn grid_width(axis: LayoutGridTrackSizingAxis) -> f32 {
    if axis == LayoutGridTrackSizingAxis::Columns {
        COLUMNS_GRID_WIDTH
    } else {
        ROWS_GRID_WIDTH
    }
}

fn grid_height(axis: LayoutGridTrackSizingAxis) -> f32 {
    if axis == LayoutGridTrackSizingAxis::Columns {
        COLUMNS_GRID_HEIGHT
    } else {
        ROWS_GRID_HEIGHT
    }
}

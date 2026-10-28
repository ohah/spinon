use spinon_core::{HostDocumentSnapshot, HostNodeHandle, NodeId};
use spinon_layout::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackLengthPercentage,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssOverflow, ComputedCssPosition,
    ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::super::{
    RuntimeLayoutCompleted, RuntimeLayoutFailure, RuntimeUaCascadeKey, layout_failure,
};
use super::super::{
    c11_4_image::element_identity,
    c11_4_nested_grid_replaced_item::{HTML_NAMESPACE, spacing_is, unsupported, zero_edges},
};
use super::{CHILD_HEIGHT, EPSILON, FIT_LIMIT, HEIGHT, SCENE_ID, WIDTH};

pub(super) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| failure("layout diagnostics missing"))?;
    let observation = diagnostics
        .fit_content_spanning
        .as_ref()
        .ok_or_else(|| failure("computed style and typed Grid input observation missing"))?;
    if !revisions_match(
        observation.style_snapshot.style_revision,
        observation.style_snapshot.generation.get(),
        observation.style_snapshot.document_revision.get(),
        observation.style_snapshot.render_tree_revision.get(),
        diagnostics.revision,
        completed.key,
    ) {
        return Err(failure(
            "style snapshot revision does not match layout output",
        ));
    }
    if observation.style_snapshot.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || observation.node_ids[0] != observation.grid_root
        || observation
            .node_ids
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != observation.node_ids.len()
    {
        return Err(failure(
            "fixture identity or computed style profile changed",
        ));
    }
    validate_root_style(style_for_node(
        &observation.style_snapshot,
        observation.grid_root,
    )?)?;
    validate_item_style(style_for_node(
        &observation.style_snapshot,
        observation.node_ids[1],
    )?)?;
    validate_child_style(style_for_node(
        &observation.style_snapshot,
        observation.node_ids[2],
    )?)?;
    validate_grid_input(observation.grid_root, &observation.grid_container_input)?;

    let relevant = completed
        .frames
        .iter()
        .filter(|frame| {
            observation
                .node_ids
                .iter()
                .any(|node| node.get() == frame.node_id)
        })
        .collect::<Vec<_>>();
    let frames = relevant
        .iter()
        .map(|frame| (frame.node_id, (frame.x, frame.y, frame.width, frame.height)))
        .collect::<std::collections::BTreeMap<_, _>>();
    if relevant.len() != observation.node_ids.len() || frames.len() != observation.node_ids.len() {
        return Err(failure("missing, duplicate, or extra fixture node frames"));
    }
    for (node, expected) in observation.node_ids.iter().zip([
        (0.0, 0.0, WIDTH, HEIGHT),
        (0.0, 0.0, WIDTH, FIT_LIMIT * 2.0),
        (0.0, 0.0, WIDTH, CHILD_HEIGHT),
    ]) {
        if !frames
            .get(&node.get())
            .is_some_and(|actual| frame_matches(*actual, expected))
        {
            return Err(failure(
                "layout frame differs from pinned CSS-pixel geometry",
            ));
        }
    }

    if diagnostics.traces.len() != 1
        || diagnostics.traces[0].node != observation.grid_root
        || diagnostics.traces[0].passes.len() != 2
    {
        return Err(failure(
            "expected one root trace with Columns and Rows passes",
        ));
    }
    let columns = diagnostics.traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .count();
    let mut rows = diagnostics.traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows);
    let row_pass = rows
        .next()
        .ok_or_else(|| failure("row sizing pass missing"))?;
    if columns != 1 || rows.next().is_some() {
        return Err(failure("expected exactly one Columns and one Rows pass"));
    }
    validate_rows_pass(row_pass, observation.node_ids[1])
}

pub(super) fn validate_identity(
    snapshot: &HostDocumentSnapshot,
    node: HostNodeHandle,
    expected_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let (namespace, tag, id) = element_identity(snapshot, node)?;
    if namespace != HTML_NAMESPACE || tag != "div" || id != expected_id {
        return Err(unsupported(
            node.id(),
            SCENE_ID,
            "element namespace, tag, or ID changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_root_style(
    style: &ComputedElementStyle,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_computed_grid_layout",
            Some(style.node_id.get()),
            None,
        )
    })?;
    let auto_rows = ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::FitContent(
        ComputedCssGridTrackLengthPercentage::LengthPx(FIT_LIMIT),
    )]);
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("width").map(String::as_str) != Some("60px")
        || style.properties.get("height").map(String::as_str) != Some("300px")
        || style
            .properties
            .get("grid-template-columns")
            .map(String::as_str)
            != Some("60px")
        || style
            .properties
            .get("grid-template-rows")
            .map(String::as_str)
            != Some("none")
        || style.properties.get("grid-auto-rows").map(String::as_str) != Some("fit-content(40px)")
        || style.properties.get("grid-auto-flow").map(String::as_str) != Some("row")
        || style.properties.get("align-content").map(String::as_str) != Some("normal")
        || style.layout_position != ComputedCssPosition::Static
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(WIDTH)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(HEIGHT)
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_overflow_x != Some(ComputedCssOverflow::Visible)
        || style.layout_overflow_y != Some(ComputedCssOverflow::Visible)
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || !spacing_is(style.layout_spacing.column_gap, 0.0)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || grid.columns != [ComputedCssGridTrackSize::LengthPx(WIDTH)]
        || !grid.rows.is_empty()
        || grid.auto_rows != auto_rows
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || !empty_line_name_groups(&grid.columns_line_names, 2)
        || !grid.rows_line_names.is_empty()
    {
        return Err(unsupported(
            style.node_id,
            SCENE_ID,
            "computed root style changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_item_style(
    style: &ComputedElementStyle,
) -> Result<(), RuntimeLayoutFailure> {
    let placement_matches = matches!(style.grid_item_placement.as_ref(), Some(placement)
        if placement.row_start == ComputedCssGridPlacement::Span(2)
            && placement.row_end == ComputedCssGridPlacement::Auto
            && placement.column_start == ComputedCssGridPlacement::Auto
            && placement.column_end == ComputedCssGridPlacement::Auto);
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_position != ComputedCssPosition::Static
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::LengthPx(0.0)
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_overflow_x != Some(ComputedCssOverflow::Visible)
        || style.layout_overflow_y != Some(ComputedCssOverflow::Visible)
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || !placement_matches
    {
        return Err(unsupported(
            style.node_id,
            SCENE_ID,
            &format!(
                "grid item style changed: position={:?}, dimensions={:?}, overflow=({:?},{:?}), placement={:?}",
                style.layout_position,
                style.layout_dimensions,
                style.layout_overflow_x,
                style.layout_overflow_y,
                style.grid_item_placement,
            ),
        ));
    }
    Ok(())
}

pub(super) fn validate_child_style(
    style: &ComputedElementStyle,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_position != ComputedCssPosition::Static
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(WIDTH)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(CHILD_HEIGHT)
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
    {
        return Err(unsupported(
            style.node_id,
            SCENE_ID,
            "nested child style changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_grid_input(
    root: NodeId,
    input: &GridContainerInput,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_rows = GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::LengthPx(FIT_LIMIT),
    )]);
    if input.columns != [GridTrackSize::LengthPx(WIDTH)]
        || !input.rows.is_empty()
        || input.auto_rows != expected_rows
        || input.auto_flow != GridAutoFlow::Row
        || input.placement_mode != GridPlacementMode::TrackSizing
        || input.columns_auto_repeat.is_some()
        || input.rows_auto_repeat.is_some()
        || input.template_areas.is_some()
        || !empty_line_name_groups(&input.columns_line_names, 2)
        || !input.rows_line_names.is_empty()
    {
        return Err(unsupported(
            root,
            SCENE_ID,
            "typed LayoutInput Grid container changed",
        ));
    }
    Ok(())
}

fn validate_rows_pass(
    pass: &LayoutGridTrackSizingPassTrace,
    item: NodeId,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_stages = [
        (LayoutGridTrackSizingStageKind::Initialize, [0.0, 0.0]),
        (
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                span: 2,
                crosses_flexible_track: false,
            },
            [0.0, 0.0],
        ),
        (
            LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
            [0.0, 0.0],
        ),
        (
            LayoutGridTrackSizingStageKind::MaximizeTracks,
            [FIT_LIMIT, FIT_LIMIT],
        ),
        (
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
            [FIT_LIMIT, FIT_LIMIT],
        ),
        (
            LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
            [FIT_LIMIT, FIT_LIMIT],
        ),
    ];
    if pass.stages.len() != expected_stages.len() {
        return Err(failure(
            "row sizing stage count differs from the pinned span trace",
        ));
    }
    for (stage, (expected_kind, expected_sizes)) in pass.stages.iter().zip(expected_stages) {
        if stage.stage != expected_kind
            || !track_sizes_match(stage, expected_sizes)
            || !track_entries_match(stage)
        {
            return Err(failure(
                "row sizing stage sequence or fit-content bases differ",
            ));
        }
    }
    let max_content_matches = pass.contributions.iter().any(|contribution| {
        contribution.item == item
            && contribution.span == 2
            && contribution.kind == LayoutGridTrackSizingContributionKind::MaxContent
            && (contribution.value - CHILD_HEIGHT).abs() <= EPSILON
    });
    if !max_content_matches {
        return Err(failure(
            "span-2 max-content contribution differs from the fixed child",
        ));
    }
    Ok(())
}

fn track_sizes_match(
    stage: &spinon_layout::LayoutGridTrackSizingStageTrace,
    expected: [f32; 2],
) -> bool {
    let tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    tracks.len() == expected.len()
        && tracks
            .iter()
            .zip(expected)
            .all(|(track, expected)| (track.base_size - expected).abs() <= EPSILON)
}

fn track_entries_match(stage: &spinon_layout::LayoutGridTrackSizingStageTrace) -> bool {
    if stage.tracks.len() != 5 {
        return false;
    }
    stage.tracks.iter().enumerate().all(|(index, track)| {
        let is_gutter = index % 2 == 0;
        let is_boundary = index == 0 || index + 1 == stage.tracks.len();
        let expected_growth_limit = if is_gutter {
            Some(0.0)
        } else if stage.stage == LayoutGridTrackSizingStageKind::Initialize {
            None
        } else {
            Some(FIT_LIMIT)
        };
        track.is_gutter == is_gutter
            && track.is_collapsed == (is_gutter && is_boundary)
            && (!is_gutter || track.base_size.abs() <= EPSILON)
            && track.growth_limit == expected_growth_limit
            && !track.infinitely_growable
    })
}

fn style_for_node(
    styles: &ComputedStyleSnapshot,
    node: NodeId,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    let mut matches = styles.elements.iter().filter(|style| style.node_id == node);
    let style = matches
        .next()
        .ok_or_else(|| layout_failure("missing_computed_style", Some(node.get()), None))?;
    if matches.next().is_some() {
        return Err(layout_failure(
            "duplicate_computed_style",
            Some(node.get()),
            None,
        ));
    }
    Ok(style)
}

fn empty_line_name_groups(groups: &[Vec<String>], count: usize) -> bool {
    groups.len() == count && groups.iter().all(Vec::is_empty)
}

pub(super) fn require_count(
    node: NodeId,
    actual: usize,
    expected: usize,
    detail: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if actual != expected {
        return Err(unsupported(node, SCENE_ID, detail));
    }
    Ok(())
}

fn frame_matches(actual: (f32, f32, f32, f32), expected: (f32, f32, f32, f32)) -> bool {
    [actual.0, actual.1, actual.2, actual.3]
        .into_iter()
        .zip([expected.0, expected.1, expected.2, expected.3])
        .all(|(actual, expected)| (actual - expected).abs() <= EPSILON)
}

fn revisions_match(
    style_revision: spinon_core::StyleRevision,
    generation: u64,
    document_revision: u64,
    render_tree_revision: u64,
    layout_revision: spinon_layout::LayoutInputRevision,
    key: RuntimeUaCascadeKey,
) -> bool {
    let source_matches = match layout_revision.source() {
        spinon_layout::LayoutSourceRevision::HostDocument {
            generation: layout_generation,
            document,
            render_tree,
        } => {
            layout_generation.get() == generation
                && document.get() == document_revision
                && render_tree.get() == render_tree_revision
                && generation == key.generation
                && document_revision == key.document_revision
                && render_tree_revision == key.render_tree_revision
        }
        spinon_layout::LayoutSourceRevision::Tree(_) => false,
    };
    source_matches
        && style_revision == layout_revision.style()
        && style_revision == key.style_revision
        && layout_revision.environment().get() == key.environment_revision
}

fn failure(detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "c114_fit_content_spanning_output_mismatch",
        None,
        Some(detail.to_owned()),
    )
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

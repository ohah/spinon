use std::collections::BTreeMap;

use spinon_core::NodeId;
use spinon_layout::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
    GridTrackSize, LayoutGridTrackSizingAxis, LayoutGridTrackSizingPassTrace,
    LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedElementStyle, ComputedStyleProfile,
};

use super::super::super::{
    RuntimeLayoutCompleted, RuntimeLayoutFailure, RuntimeUaCascadeKey, layout_failure,
};
use super::{SCENE_ID, spacing_is, unsupported, zero_edges};

type ObservedFrames = BTreeMap<u64, (f32, f32, f32, f32)>;

pub(super) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| failure("layout diagnostics missing"))?;
    let observation = diagnostics
        .grid_layout_auto_tracks
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
    {
        return Err(failure("computed style profile changed"));
    }
    validate_grid_input(observation.grid_root, &observation.grid_container_input)?;
    let frames = observed_frames(completed, &observation.node_ids)?;
    for (node, expected) in observation.node_ids.iter().zip([
        (0.0, 0.0, 100.0, 20.0),
        (0.0, 0.0, 100.0, 10.0),
        (100.0, 0.0, 50.0, 10.0),
        (100.0, 0.0, 50.0, 10.0),
        (0.0, 10.0, 100.0, 10.0),
    ]) {
        if frames.get(&node.get()) != Some(&expected) {
            return Err(failure(
                "layout frame differs from pinned CSS-pixel geometry",
            ));
        }
    }

    let traces = &diagnostics.traces;
    if traces.len() != 1 || traces[0].node != observation.grid_root || traces[0].passes.len() != 2 {
        return Err(failure(
            "expected one root trace with one Columns and one Rows pass",
        ));
    }
    let columns = traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .collect::<Vec<_>>();
    let rows = traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    if columns.len() != 1 || rows.len() != 1 {
        return Err(failure("expected exactly one Columns and one Rows pass"));
    }
    validate_axis_pass(columns[0], [100.0, 50.0])?;
    validate_axis_pass(rows[0], [10.0, 10.0])?;
    let column_minimums = columns[0]
        .contributions
        .iter()
        .filter(|entry| entry.kind == spinon_layout::LayoutGridTrackSizingContributionKind::Minimum)
        .collect::<Vec<_>>();
    if column_minimums.len() != 1
        || column_minimums[0].item != observation.node_ids[2]
        || column_minimums[0].value != 50.0
    {
        return Err(failure(
            "implicit column minimum contribution differs from B at 50px",
        ));
    }
    let row_minimum_entries = rows[0]
        .contributions
        .iter()
        .filter(|entry| entry.kind == spinon_layout::LayoutGridTrackSizingContributionKind::Minimum)
        .collect::<Vec<_>>();
    let row_minimums = row_minimum_entries
        .iter()
        .copied()
        .map(|entry| (entry.item, entry.value))
        .collect::<BTreeMap<_, _>>();
    if row_minimum_entries.len() != 3
        || row_minimums.len() != 3
        || observation
            .node_ids
            .iter()
            .skip(1)
            .filter(|node| row_minimums.get(node) == Some(&10.0))
            .count()
            != 3
    {
        return Err(failure(
            "row minimum contributions differ from A/B/C at 10px",
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
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("align-items").map(String::as_str) != Some("start")
        || style.properties.get("grid-auto-flow").map(String::as_str) != Some("row")
        || style.properties.get("direction").map(String::as_str) != Some("ltr")
        || style.properties.get("writing-mode").map(String::as_str) != Some("horizontal-tb")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(100.0)
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || !spacing_is(style.layout_spacing.column_gap, 0.0)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || grid.columns != [ComputedCssGridTrackSize::LengthPx(100.0)]
        || grid.rows != [ComputedCssGridTrackSize::Auto]
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || !empty_line_name_groups(&grid.columns_line_names, 2)
        || !empty_line_name_groups(&grid.rows_line_names, 2)
    {
        return Err(unsupported(
            style.node_id,
            SCENE_ID,
            "computed root style differs from allowlist",
        ));
    }
    Ok(())
}

pub(super) fn validate_grid_item_style(
    style: &ComputedElementStyle,
    expected_id: &str,
    row: i32,
    column: i32,
) -> Result<(), RuntimeLayoutFailure> {
    let expected = spinon_style::ComputedCssGridItemPlacement {
        row_start: ComputedCssGridPlacement::Line(row),
        row_end: ComputedCssGridPlacement::Line(row + 1),
        column_start: ComputedCssGridPlacement::Line(column),
        column_end: ComputedCssGridPlacement::Line(column + 1),
    };
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || style.grid_item_placement.as_ref() != Some(&expected)
    {
        return Err(unsupported(style.node_id, SCENE_ID, expected_id));
    }
    Ok(())
}

pub(super) fn validate_text_block_style(
    style: &ComputedElementStyle,
    width: ComputedCssDimension,
    expected_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    for (property, expected) in [
        ("box-sizing", "border-box"),
        ("font-family", "Ahem"),
        ("font-size", "10px"),
        ("font-weight", "400"),
        ("font-style", "normal"),
        ("line-height", "10px"),
        ("white-space-collapse", "collapse"),
        ("text-wrap-mode", "wrap"),
        ("word-break", "normal"),
        ("direction", "ltr"),
        ("line-break", "auto"),
        ("text-align", "start"),
        ("overflow-wrap", "normal"),
        ("text-transform", "none"),
        ("writing-mode", "horizontal-tb"),
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
    ] {
        if style.properties.get(property).map(String::as_str) != Some(expected) {
            return Err(unsupported(style.node_id, SCENE_ID, expected_id));
        }
    }
    if style.font_size_css_px != 10.0
        || style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != width
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !zero_border(style.layout_border)
    {
        return Err(unsupported(style.node_id, SCENE_ID, expected_id));
    }
    Ok(())
}

pub(super) fn validate_grid_input(
    root: NodeId,
    input: &GridContainerInput,
) -> Result<(), RuntimeLayoutFailure> {
    if input.columns != [GridTrackSize::LengthPx(100.0)]
        || input.rows != [GridTrackSize::Auto]
        || input.auto_rows != GridAutoRows::Auto
        || input.auto_columns != GridAutoColumns::Auto
        || input.auto_flow != GridAutoFlow::Row
        || input.placement_mode != GridPlacementMode::TrackSizing
        || input.columns_auto_repeat.is_some()
        || input.rows_auto_repeat.is_some()
        || input.template_areas.is_some()
        || !empty_line_name_groups(&input.columns_line_names, 2)
        || !empty_line_name_groups(&input.rows_line_names, 2)
    {
        return Err(unsupported(
            root,
            SCENE_ID,
            "typed LayoutInput Grid container changed",
        ));
    }
    Ok(())
}

fn validate_axis_pass(
    pass: &LayoutGridTrackSizingPassTrace,
    expected: [f32; 2],
) -> Result<(), RuntimeLayoutFailure> {
    let expected_stages = [
        LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
        LayoutGridTrackSizingStageKind::MaximizeTracks,
        LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
    ];
    let mut indices = Vec::with_capacity(4);
    for expected_stage in expected_stages {
        let matching = pass
            .stages
            .iter()
            .enumerate()
            .filter(|(_, stage)| stage.stage == expected_stage)
            .collect::<Vec<_>>();
        if matching.len() != 1 || !track_bases_match(matching[0].1, expected) {
            return Err(failure("pre-final track sizing stage differs from oracle"));
        }
        indices.push(matching[0].0);
    }
    let stretch = pass
        .stages
        .iter()
        .enumerate()
        .filter(|(_, stage)| {
            matches!(
                stage.stage,
                LayoutGridTrackSizingStageKind::StretchAutoTracks { .. }
            )
        })
        .collect::<Vec<_>>();
    if stretch.len() != 1
        || stretch[0].0 != pass.stages.len() - 1
        || !track_bases_match(stretch[0].1, expected)
    {
        let observed = pass
            .stages
            .iter()
            .map(|stage| {
                (
                    stage.stage,
                    stage
                        .tracks
                        .iter()
                        .filter(|track| !track.is_gutter && !track.is_collapsed)
                        .map(|track| track.base_size)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        return Err(failure(&format!(
            "final auto track stage differs from oracle: axis={:?}, expected={expected:?}, observed={observed:?}",
            pass.axis
        )));
    }
    indices.push(stretch[0].0);
    if !indices.windows(2).all(|pair| pair[0] < pair[1]) {
        return Err(failure("track sizing stages are out of order"));
    }
    Ok(())
}

fn observed_frames(
    completed: &RuntimeLayoutCompleted,
    node_ids: &[NodeId; 5],
) -> Result<ObservedFrames, RuntimeLayoutFailure> {
    let frames = completed
        .frames
        .iter()
        .filter(|frame| node_ids.iter().any(|node| node.get() == frame.node_id))
        .map(|frame| (frame.node_id, (frame.x, frame.y, frame.width, frame.height)))
        .collect::<BTreeMap<_, _>>();
    let relevant_count = completed
        .frames
        .iter()
        .filter(|frame| node_ids.iter().any(|node| node.get() == frame.node_id))
        .count();
    if relevant_count != node_ids.len() || frames.len() != node_ids.len() {
        return Err(failure("missing, duplicate, or extra fixture node frames"));
    }
    Ok(frames)
}

fn track_bases_match(stage: &LayoutGridTrackSizingStageTrace, expected: [f32; 2]) -> bool {
    let tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    tracks.len() == expected.len()
        && tracks
            .iter()
            .zip(expected)
            .all(|(track, size)| track.base_size == size)
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

fn zero_border(border: spinon_style::ComputedLayoutBorder) -> bool {
    border.top == 0.0 && border.right == 0.0 && border.bottom == 0.0 && border.left == 0.0
}

fn empty_line_name_groups(groups: &[Vec<String>], expected_count: usize) -> bool {
    groups.len() == expected_count && groups.iter().all(Vec::is_empty)
}

fn failure(detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "c114_grid_auto_tracks_output_mismatch",
        None,
        Some(detail.to_owned()),
    )
}

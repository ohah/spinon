use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostParent, NodeId};
use spinon_layout::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutGridTrackSizingAxis, LayoutGridTrackSizingPassTrace,
    LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssOverflow, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::{
    RuntimeC11_4FitContentAutoMinFloorObservation, RuntimeLayoutCompleted, RuntimeLayoutFailure,
    RuntimeUaCascadeKey, layout_failure,
};
use super::c11_4_image::element_identity;
use super::c11_4_nested_grid_replaced_item::{
    HTML_NAMESPACE, children, spacing_is, style_for, unsupported, zero_edges,
};

const WPT_SCENE: &str = "fit-content-auto-min-floor-wpt";
const ZERO_SCENE: &str = "fit-content-auto-min-floor-zero-control";
const WIDTH: f32 = 60.0;
const HEIGHT: f32 = 300.0;
const FIT_LIMIT: f32 = 40.0;
const EPSILON: f32 = 1.0e-4;

fn scene_for_root(id: &str) -> Option<&'static str> {
    match id {
        "c114-fit-content-auto-min-floor-wpt-grid" => Some(WPT_SCENE),
        "c114-fit-content-auto-min-floor-zero-grid" => Some(ZERO_SCENE),
        _ => None,
    }
}

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<(&'static str, [NodeId; 5])>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene_id) = scene_for_root(&root_id) else {
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(root.id(), scene_id, "profile or root identity"));
    }
    let suffix = if scene_id == WPT_SCENE { "wpt" } else { "zero" };
    let root_children = children(snapshot, root)?;
    require_count(
        root.id(),
        root_children.len(),
        2,
        scene_id,
        "direct grid item count",
    )?;
    let first_item = root_children[0];
    let second_item = root_children[1];
    for item in [first_item, second_item] {
        if snapshot.parent(item) != Some(HostParent::Node(root)) {
            return Err(unsupported(
                item.id(),
                scene_id,
                "grid item is not a direct child",
            ));
        }
    }
    validate_identity(
        snapshot,
        first_item,
        &format!("c114-fit-content-auto-min-floor-{suffix}-first-item"),
        scene_id,
    )?;
    validate_identity(
        snapshot,
        second_item,
        &format!("c114-fit-content-auto-min-floor-{suffix}-second-item"),
        scene_id,
    )?;
    let first_children = children(snapshot, first_item)?;
    let second_children = children(snapshot, second_item)?;
    require_count(
        first_item.id(),
        first_children.len(),
        1,
        scene_id,
        "first item child count",
    )?;
    require_count(
        second_item.id(),
        second_children.len(),
        1,
        scene_id,
        "second item child count",
    )?;
    let first_child = first_children[0];
    let second_child = second_children[0];
    if snapshot.parent(first_child) != Some(HostParent::Node(first_item))
        || snapshot.parent(second_child) != Some(HostParent::Node(second_item))
    {
        return Err(unsupported(
            root.id(),
            scene_id,
            "nested child ownership changed",
        ));
    }
    validate_identity(
        snapshot,
        first_child,
        &format!("c114-fit-content-auto-min-floor-{suffix}-first-child"),
        scene_id,
    )?;
    validate_identity(
        snapshot,
        second_child,
        &format!("c114-fit-content-auto-min-floor-{suffix}-second-child"),
        scene_id,
    )?;

    let node_ids = [
        root.id(),
        first_item.id(),
        first_child.id(),
        second_item.id(),
        second_child.id(),
    ];
    let styles_for_nodes = [
        style_for(styles, root)?,
        style_for(styles, first_item)?,
        style_for(styles, first_child)?,
        style_for(styles, second_item)?,
        style_for(styles, second_child)?,
    ];
    validate_root_style(styles_for_nodes[0], scene_id)?;
    validate_item_style(styles_for_nodes[1], false, scene_id)?;
    validate_child_style(styles_for_nodes[2], 10.0, scene_id)?;
    validate_item_style(styles_for_nodes[3], scene_id == ZERO_SCENE, scene_id)?;
    validate_child_style(styles_for_nodes[4], 100.0, scene_id)?;
    let style_ids = styles
        .elements
        .iter()
        .map(|style| style.node_id)
        .collect::<std::collections::BTreeSet<_>>();
    if styles_ids_do_not_match(&style_ids, &node_ids) || styles.elements.len() != style_ids.len() {
        return Err(unsupported(
            root.id(),
            scene_id,
            "computed style node set changed",
        ));
    }
    Ok(Some((scene_id, node_ids)))
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    scene_id: &'static str,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    grid_inputs: &std::collections::BTreeMap<NodeId, GridContainerInput>,
    node_ids: [NodeId; 5],
) -> Result<RuntimeC11_4FitContentAutoMinFloorObservation, RuntimeLayoutFailure> {
    validate_scene(scene_id)?;
    validate_root_style(style_for(styles, root)?, scene_id)?;
    let grid_input = grid_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_fit_content_auto_min_floor_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validate_grid_input(root.id(), grid_input, scene_id)?;
    if grid_inputs.len() != 1 || node_ids[0] != root.id() {
        return Err(unsupported(
            root.id(),
            scene_id,
            "unexpected Grid input or root identity",
        ));
    }
    Ok(RuntimeC11_4FitContentAutoMinFloorObservation {
        scene_id,
        style_snapshot: styles.clone(),
        grid_root: root.id(),
        grid_container_input: grid_input.clone(),
        node_ids,
    })
}

pub(in crate::session::ua_cascade::runtime_layout) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| failure("layout diagnostics missing"))?;
    let observation = diagnostics
        .fit_content_auto_min_floor
        .as_ref()
        .ok_or_else(|| failure("computed style and typed Grid input observation missing"))?;
    validate_scene(observation.scene_id)?;
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
    let styles = &observation.style_snapshot;
    let style_ids = styles
        .elements
        .iter()
        .map(|style| style.node_id)
        .collect::<std::collections::BTreeSet<_>>();
    if styles_ids_do_not_match(&style_ids, &observation.node_ids)
        || styles.elements.len() != style_ids.len()
    {
        return Err(failure("computed style node set changed"));
    }
    validate_root_style(
        style_for_node(styles, observation.grid_root)?,
        observation.scene_id,
    )?;
    validate_item_style(
        style_for_node(styles, observation.node_ids[1])?,
        false,
        observation.scene_id,
    )?;
    validate_child_style(
        style_for_node(styles, observation.node_ids[2])?,
        10.0,
        observation.scene_id,
    )?;
    validate_item_style(
        style_for_node(styles, observation.node_ids[3])?,
        observation.scene_id == ZERO_SCENE,
        observation.scene_id,
    )?;
    validate_child_style(
        style_for_node(styles, observation.node_ids[4])?,
        100.0,
        observation.scene_id,
    )?;
    validate_grid_input(
        observation.grid_root,
        &observation.grid_container_input,
        observation.scene_id,
    )?;

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
    if relevant.len() != observation.node_ids.len() {
        return Err(failure("missing, duplicate, or extra fixture node frames"));
    }
    let by_id = relevant
        .iter()
        .map(|frame| (frame.node_id, (frame.x, frame.y, frame.width, frame.height)))
        .collect::<std::collections::BTreeMap<_, _>>();
    if by_id.len() != observation.node_ids.len() {
        return Err(failure("duplicate fixture node frames"));
    }
    let second_track = if observation.scene_id == WPT_SCENE {
        100.0
    } else {
        FIT_LIMIT
    };
    let expected_frames = [
        (0.0, 0.0, WIDTH, HEIGHT),
        (0.0, 0.0, WIDTH, 10.0),
        (0.0, 0.0, WIDTH, 10.0),
        (0.0, 10.0, WIDTH, second_track),
        (0.0, 10.0, WIDTH, 100.0),
    ];
    for (node, expected) in observation.node_ids.iter().zip(expected_frames) {
        let actual = by_id
            .get(&node.get())
            .ok_or_else(|| failure("fixture node frame missing"))?;
        if !frame_matches(*actual, expected) {
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
    validate_rows_pass(row_pass, observation.scene_id == ZERO_SCENE)
}

fn validate_scene(scene_id: &str) -> Result<(), RuntimeLayoutFailure> {
    if scene_id == WPT_SCENE || scene_id == ZERO_SCENE {
        Ok(())
    } else {
        Err(failure("unknown scene id"))
    }
}

fn validate_identity(
    snapshot: &HostDocumentSnapshot,
    node: HostNodeHandle,
    expected: &str,
    scene: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let (namespace, tag, id) = element_identity(snapshot, node)?;
    if namespace != HTML_NAMESPACE || tag != "div" || id != expected {
        return Err(unsupported(
            node.id(),
            scene,
            "element namespace, tag, or ID changed",
        ));
    }
    Ok(())
}

fn validate_root_style(
    style: &ComputedElementStyle,
    scene: &str,
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
        || style.properties.get("height").map(String::as_str) != Some("300px")
        || style.properties.get("grid-auto-rows").map(String::as_str) != Some("fit-content(40px)")
        || style.properties.get("align-content").map(String::as_str) != Some("normal")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(WIDTH)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(HEIGHT)
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_overflow_x != Some(ComputedCssOverflow::Visible)
        || style.layout_overflow_y != Some(ComputedCssOverflow::Visible)
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
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
            scene,
            "computed root style changed",
        ));
    }
    Ok(())
}

fn validate_item_style(
    style: &ComputedElementStyle,
    min_height_zero: bool,
    scene: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_min_height = if min_height_zero {
        ComputedCssDimension::LengthPx(0.0)
    } else {
        ComputedCssDimension::Auto
    };
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != expected_min_height
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_overflow_x != Some(ComputedCssOverflow::Visible)
        || style.layout_overflow_y != Some(ComputedCssOverflow::Visible)
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || !matches!(style.grid_item_placement.as_ref(), Some(placement)
            if placement.row_start == spinon_style::ComputedCssGridPlacement::Auto
                && placement.row_end == spinon_style::ComputedCssGridPlacement::Auto
                && placement.column_start == spinon_style::ComputedCssGridPlacement::Auto
                && placement.column_end == spinon_style::ComputedCssGridPlacement::Auto)
    {
        return Err(unsupported(style.node_id, scene, "grid item style changed"));
    }
    Ok(())
}

fn validate_child_style(
    style: &ComputedElementStyle,
    height: f32,
    scene: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(WIDTH)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(height)
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
    {
        return Err(unsupported(
            style.node_id,
            scene,
            "nested child style changed",
        ));
    }
    Ok(())
}

fn validate_grid_input(
    root: NodeId,
    input: &GridContainerInput,
    scene: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if input.columns != [GridTrackSize::LengthPx(WIDTH)]
        || !input.rows.is_empty()
        || input.auto_rows
            != GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::LengthPx(FIT_LIMIT),
            )])
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
            scene,
            "typed LayoutInput Grid container changed",
        ));
    }
    Ok(())
}

fn validate_rows_pass(
    pass: &LayoutGridTrackSizingPassTrace,
    zero_control: bool,
) -> Result<(), RuntimeLayoutFailure> {
    let intrinsic = if zero_control {
        [10.0, 0.0]
    } else {
        [10.0, 100.0]
    };
    let final_sizes = if zero_control {
        [10.0, FIT_LIMIT]
    } else {
        [10.0, 100.0]
    };
    let expected = [
        (LayoutGridTrackSizingStageKind::Initialize, [0.0, 0.0]),
        (
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                span: 1,
                crosses_flexible_track: false,
            },
            intrinsic,
        ),
        (
            LayoutGridTrackSizingStageKind::IntrinsicSizingComplete,
            intrinsic,
        ),
        (LayoutGridTrackSizingStageKind::MaximizeTracks, final_sizes),
        (
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
            final_sizes,
        ),
        (
            LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
            final_sizes,
        ),
    ];
    if pass.stages.len() != expected.len() {
        return Err(failure("row sizing stage count differs from pinned trace"));
    }
    for (stage, (expected_kind, expected_sizes)) in pass.stages.iter().zip(expected) {
        if stage.stage != expected_kind
            || !track_sizes_match(stage, expected_sizes)
            || !track_entries_match(stage)
        {
            return Err(failure(
                "row sizing stage sequence or fit-content base differs from pinned trace",
            ));
        }
    }
    Ok(())
}

fn track_entries_match(stage: &LayoutGridTrackSizingStageTrace) -> bool {
    if stage.tracks.len() != 5 {
        return false;
    }
    stage.tracks.iter().enumerate().all(|(index, track)| {
        let is_gutter = index % 2 == 0;
        let is_boundary = index == 0 || index + 1 == stage.tracks.len();
        track.is_gutter == is_gutter
            && track.is_collapsed == (is_gutter && is_boundary)
            && (!is_gutter || track.base_size.abs() <= EPSILON)
    })
}

fn track_sizes_match(stage: &LayoutGridTrackSizingStageTrace, expected: [f32; 2]) -> bool {
    let mut tracks = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed);
    expected.iter().all(|value| {
        tracks.next().is_some_and(|track| {
            (track.base_size - value).abs() <= EPSILON && track.base_size.is_finite()
        })
    }) && tracks.next().is_none()
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

fn styles_ids_do_not_match(ids: &std::collections::BTreeSet<NodeId>, nodes: &[NodeId; 5]) -> bool {
    ids.len() != nodes.len() || nodes.iter().any(|node| !ids.contains(node))
}

fn empty_line_name_groups(groups: &[Vec<String>], expected: usize) -> bool {
    groups.len() == expected && groups.iter().all(Vec::is_empty)
}

fn require_count(
    node: NodeId,
    actual: usize,
    expected: usize,
    scene: &str,
    detail: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if actual == expected {
        Ok(())
    } else {
        Err(unsupported(node, scene, detail))
    }
}

fn frame_matches(actual: (f32, f32, f32, f32), expected: (f32, f32, f32, f32)) -> bool {
    [actual.0, actual.1, actual.2, actual.3]
        .iter()
        .zip([expected.0, expected.1, expected.2, expected.3])
        .all(|(actual, expected)| actual.is_finite() && (*actual - expected).abs() <= EPSILON)
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
        "c114_fit_content_auto_min_floor_output_mismatch",
        None,
        Some(detail.to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::{FIT_LIMIT, ZERO_SCENE, validate_grid_input};
    use spinon_core::NodeId;
    use spinon_layout::{
        GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
        GridTrackLengthPercentage, GridTrackSize,
    };

    fn expected() -> GridContainerInput {
        GridContainerInput {
            columns: vec![GridTrackSize::LengthPx(60.0)],
            columns_line_names: vec![vec![], vec![]],
            auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
                GridTrackLengthPercentage::LengthPx(FIT_LIMIT),
            )]),
            auto_flow: GridAutoFlow::Row,
            placement_mode: GridPlacementMode::TrackSizing,
            ..GridContainerInput::default()
        }
    }

    #[test]
    fn accepts_only_the_pinned_fit_content_auto_minimum_grid_input() {
        let root = NodeId::new(1).unwrap();
        assert!(validate_grid_input(root, &expected(), ZERO_SCENE).is_ok());
        let mut altered_limit = expected();
        altered_limit.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(41.0),
        )]);
        assert!(validate_grid_input(root, &altered_limit, ZERO_SCENE).is_err());
        let mut explicit_row = expected();
        explicit_row.rows = vec![GridTrackSize::LengthPx(40.0)];
        assert!(validate_grid_input(root, &explicit_row, ZERO_SCENE).is_err());
        let mut altered_columns = expected();
        altered_columns.columns[0] = GridTrackSize::LengthPx(61.0);
        assert!(validate_grid_input(root, &altered_columns, ZERO_SCENE).is_err());
    }
}

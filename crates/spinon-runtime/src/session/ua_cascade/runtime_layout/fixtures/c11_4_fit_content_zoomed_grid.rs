use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostParent, NodeId};
use spinon_layout::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
    GridTrackLengthPercentage, GridTrackSize, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingMinimumContributionSource,
    LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackLengthPercentage,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::{
    RuntimeC11_4FitContentZoomedGridObservation, RuntimeLayoutCompleted, RuntimeLayoutFailure,
    RuntimeUaCascadeKey, layout_failure,
};
use super::c11_4_image::element_identity;
use super::c11_4_nested_grid_replaced_item::{
    HTML_NAMESPACE, children, spacing_is, style_for, unsupported, zero_edges,
};

const SCENE_ID: &str = "fit-content-zoomed-grid-wpt";
const ROOT_ID: &str = "c114-fit-content-zoomed-grid";
const ITEM_ID: &str = "c114-fit-content-zoomed-grid-item";
const CHILD_ID: &str = "c114-fit-content-zoomed-grid-child";
const ZOOM: f32 = 2.0;
const ROOT_WIDTH: f32 = 120.0;
const ROOT_HEIGHT: f32 = 600.0;
const TRACK_LIMIT: f32 = 80.0;
const CHILD_HEIGHT: f32 = 800.0;
const EPSILON: f32 = 1.0e-4;

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<[NodeId; 3]>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    if root_id != ROOT_ID {
        return Ok(None);
    }
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(root.id(), SCENE_ID, "profile or root identity"));
    }

    let root_children = children(snapshot, root)?;
    require_count(root.id(), root_children.len(), 1, "direct grid item count")?;
    let item = root_children[0];
    if snapshot.parent(item) != Some(HostParent::Node(root)) {
        return Err(unsupported(
            item.id(),
            SCENE_ID,
            "grid item is not a direct child",
        ));
    }
    validate_identity(snapshot, item, ITEM_ID)?;
    let item_children = children(snapshot, item)?;
    require_count(
        item.id(),
        item_children.len(),
        1,
        "nested block child count",
    )?;
    let child = item_children[0];
    if snapshot.parent(child) != Some(HostParent::Node(item)) {
        return Err(unsupported(
            child.id(),
            SCENE_ID,
            "nested child ownership changed",
        ));
    }
    validate_identity(snapshot, child, CHILD_ID)?;

    let node_ids = [root.id(), item.id(), child.id()];
    validate_root_style(style_for(styles, root)?)?;
    validate_item_style(style_for(styles, item)?)?;
    validate_child_style(style_for(styles, child)?)?;
    let style_ids = styles
        .elements
        .iter()
        .map(|style| style.node_id)
        .collect::<BTreeSet<_>>();
    if style_ids.len() != node_ids.len()
        || node_ids.iter().any(|node| !style_ids.contains(node))
        || styles.elements.len() != style_ids.len()
    {
        return Err(unsupported(
            root.id(),
            SCENE_ID,
            "computed style node set changed",
        ));
    }
    Ok(Some(node_ids))
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    grid_inputs: &BTreeMap<NodeId, GridContainerInput>,
    node_ids: [NodeId; 3],
) -> Result<RuntimeC11_4FitContentZoomedGridObservation, RuntimeLayoutFailure> {
    validate_root_style(style_for(styles, root)?)?;
    let grid_input = grid_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_fit_content_zoomed_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validate_grid_input(root.id(), grid_input)?;
    if grid_inputs.len() != 1 || node_ids[0] != root.id() {
        return Err(unsupported(
            root.id(),
            SCENE_ID,
            "unexpected Grid input or root identity",
        ));
    }
    Ok(RuntimeC11_4FitContentZoomedGridObservation {
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
        .fit_content_zoomed_grid
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
            .collect::<BTreeSet<_>>()
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

    let frames = completed
        .frames
        .iter()
        .filter(|frame| {
            observation
                .node_ids
                .iter()
                .any(|node| node.get() == frame.node_id)
        })
        .map(|frame| (frame.node_id, (frame.x, frame.y, frame.width, frame.height)))
        .collect::<BTreeMap<_, _>>();
    if completed.frames.len() != observation.node_ids.len()
        || frames.len() != observation.node_ids.len()
    {
        return Err(failure("missing, duplicate, or extra fixture node frames"));
    }
    for (node, expected) in observation.node_ids.iter().zip([
        (0.0, 0.0, ROOT_WIDTH, ROOT_HEIGHT),
        (0.0, 0.0, ROOT_WIDTH, TRACK_LIMIT),
        (0.0, 0.0, ROOT_WIDTH, CHILD_HEIGHT),
    ]) {
        if !frames
            .get(&node.get())
            .is_some_and(|actual| frame_matches(*actual, expected))
        {
            return Err(failure("layout frame differs from pinned zoomed geometry"));
        }
    }

    let trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node == observation.grid_root)
        .ok_or_else(|| failure("root track sizing trace missing"))?;
    if diagnostics.traces.len() != 1 || trace.passes.len() != 2 {
        return Err(failure(
            "expected one root trace with Columns and Rows passes",
        ));
    }
    let rows = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .ok_or_else(|| failure("row sizing pass missing"))?;
    validate_rows_pass(rows, observation.node_ids[1])
}

fn validate_identity(
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

fn validate_root_style(style: &ComputedElementStyle) -> Result<(), RuntimeLayoutFailure> {
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_computed_grid_layout",
            Some(style.node_id.get()),
            None,
        )
    })?;
    let expected_auto_rows =
        ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::FitContent(
            ComputedCssGridTrackLengthPercentage::LengthPx(TRACK_LIMIT),
        )]);
    if !zoom_matches(style, ZOOM, ZOOM)
        || style.properties.get("zoom").map(String::as_str) != Some("2")
        || style.properties.get("display").map(String::as_str) != Some("grid")
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
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(ROOT_WIDTH)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(ROOT_HEIGHT)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || !spacing_is(style.layout_spacing.column_gap, 0.0)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || grid.columns != [ComputedCssGridTrackSize::LengthPx(ROOT_WIDTH)]
        || !grid.rows.is_empty()
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.auto_rows != expected_auto_rows
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

fn validate_item_style(style: &ComputedElementStyle) -> Result<(), RuntimeLayoutFailure> {
    let placement_matches = matches!(style.grid_item_placement.as_ref(), Some(placement)
        if placement.row_start == ComputedCssGridPlacement::Auto
            && placement.row_end == ComputedCssGridPlacement::Auto
            && placement.column_start == ComputedCssGridPlacement::Auto
            && placement.column_end == ComputedCssGridPlacement::Auto);
    if !zoom_matches(style, 1.0, ZOOM)
        || style.properties.get("zoom").map(String::as_str) != Some("1")
        || style.properties.get("display").map(String::as_str) != Some("block")
        || style.properties.get("min-height").map(String::as_str) != Some("0px")
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::LengthPx(0.0)
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || !placement_matches
    {
        return Err(unsupported(
            style.node_id,
            SCENE_ID,
            "grid item style changed",
        ));
    }
    Ok(())
}

fn validate_child_style(style: &ComputedElementStyle) -> Result<(), RuntimeLayoutFailure> {
    if !zoom_matches(style, 1.0, ZOOM)
        || style.properties.get("zoom").map(String::as_str) != Some("1")
        || style.properties.get("display").map(String::as_str) != Some("block")
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(CHILD_HEIGHT)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style.layout_spacing.padding)
        || !zero_edges(style.layout_spacing.margin)
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

fn zoom_matches(style: &ComputedElementStyle, specified: f32, effective: f32) -> bool {
    style.c11_4_zoom.is_some_and(|zoom| {
        zoom.specified.is_finite()
            && zoom.effective.is_finite()
            && (zoom.specified - specified).abs() <= EPSILON
            && (zoom.effective - effective).abs() <= EPSILON
    })
}

fn validate_grid_input(
    root: NodeId,
    input: &GridContainerInput,
) -> Result<(), RuntimeLayoutFailure> {
    let auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::LengthPx(TRACK_LIMIT),
    )]);
    if input.columns != [GridTrackSize::LengthPx(ROOT_WIDTH)]
        || !input.rows.is_empty()
        || input.auto_columns != GridAutoColumns::Auto
        || input.auto_rows != auto_rows
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
        (LayoutGridTrackSizingStageKind::Initialize, 0.0),
        (
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                span: 1,
                crosses_flexible_track: false,
            },
            0.0,
        ),
        (LayoutGridTrackSizingStageKind::IntrinsicSizingComplete, 0.0),
        (LayoutGridTrackSizingStageKind::MaximizeTracks, TRACK_LIMIT),
        (
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
            TRACK_LIMIT,
        ),
        (
            LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
            TRACK_LIMIT,
        ),
    ];
    if pass.axis != LayoutGridTrackSizingAxis::Rows || pass.sequence != 1 {
        return Err(failure("row sizing pass identity differs"));
    }
    if pass.stages.len() != expected_stages.len()
        || pass.fixture_phase.is_some()
        || pass.fixture_auto_repeat_count.is_some()
        || pass.auto_repeat_count.is_some()
        || pass.fixture_percentage_basis.is_some()
    {
        return Err(failure(
            "row sizing stage count differs from the pinned trace",
        ));
    }
    for (stage, (expected_kind, expected_base)) in pass.stages.iter().zip(expected_stages) {
        let structure_matches = stage.tracks.len() == 3
            && stage.tracks.iter().enumerate().all(|(index, track)| {
                let boundary_gutter = index != 1;
                track.internal_index as usize == index
                    && track.is_gutter == boundary_gutter
                    && track.is_collapsed == boundary_gutter
                    && !track.infinitely_growable
                    && if boundary_gutter {
                        track.base_size.abs() <= EPSILON
                            && track
                                .growth_limit
                                .is_some_and(|limit| limit.abs() <= EPSILON)
                    } else {
                        (track.base_size - expected_base).abs() <= EPSILON
                    }
            });
        if stage.stage != expected_kind || !structure_matches {
            return Err(failure(
                "row sizing stage sequence or fit-content base differs",
            ));
        }
    }
    let expected = [
        (
            LayoutGridTrackSizingContributionKind::Minimum,
            0.0,
            Some(LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum),
        ),
        (
            LayoutGridTrackSizingContributionKind::MinContent,
            CHILD_HEIGHT,
            None,
        ),
        (
            LayoutGridTrackSizingContributionKind::MaxContent,
            CHILD_HEIGHT,
            None,
        ),
    ];
    if pass.contributions.len() != expected.len()
        || pass
            .contributions
            .iter()
            .zip(expected)
            .any(|(actual, (kind, value, minimum_source))| {
                actual.item != item
                    || actual.kind != kind
                    || actual.span != 1
                    || actual.crosses_flexible_track
                    || (actual.value - value).abs() > EPSILON
                    || actual.minimum_source != minimum_source
            })
    {
        return Err(failure(
            "row minimum/min-content/max-content contributions differ",
        ));
    }
    Ok(())
}

fn empty_line_name_groups(groups: &[Vec<String>], count: usize) -> bool {
    groups.len() == count && groups.iter().all(Vec::is_empty)
}

fn require_count(
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

fn frame_matches(actual: (f32, f32, f32, f32), expected: (f32, f32, f32, f32)) -> bool {
    [actual.0, actual.1, actual.2, actual.3]
        .into_iter()
        .zip([expected.0, expected.1, expected.2, expected.3])
        .all(|(actual, expected)| (actual - expected).abs() <= EPSILON)
}

fn failure(detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "c114_fit_content_zoomed_grid_output_mismatch",
        None,
        Some(detail.to_owned()),
    )
}

#[cfg(test)]
#[path = "c11_4_fit_content_zoomed_grid/tests.rs"]
mod tests;

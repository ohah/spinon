use spinon_core::{HostDocumentSnapshot, HostNodeHandle, NodeId};
use spinon_layout::{
    GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode, GridTrackLengthPercentage,
    GridTrackSize, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingPassTrace, LayoutGridTrackSizingStageKind,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssOverflow, ComputedCssPosition, ComputedElementStyle,
    ComputedStyleSnapshot,
};

use super::super::super::{
    RuntimeLayoutCompleted, RuntimeLayoutFailure, RuntimeUaCascadeKey, layout_failure,
};
use super::super::c11_4_nested_grid_replaced_item::{
    HTML_NAMESPACE, spacing_is, unsupported, zero_edges,
};
use super::Case;

const WIDTH: f32 = 60.0;
const HEIGHT: f32 = 300.0;
const CHILD_HEIGHT: f32 = 100.0;
const EPSILON: f32 = 1.0e-4;

pub(super) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| failure("layout diagnostics missing"))?;
    let observation = diagnostics
        .fit_content_limit_boundaries
        .as_ref()
        .ok_or_else(|| failure("computed style and typed Grid input observation missing"))?;
    let case = Case {
        scene_id: observation.scene_id,
        root_id: "",
        item_id: "",
        child_id: "",
        limit: observation.limit_css_px,
        expected_row: observation.expected_row_css_px,
    };
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
    if observation.style_snapshot.profile
        != spinon_style::ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
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
    let root_style = style_for_node(&observation.style_snapshot, observation.grid_root)?;
    let item_style = style_for_node(&observation.style_snapshot, observation.node_ids[1])?;
    let child_style = style_for_node(&observation.style_snapshot, observation.node_ids[2])?;
    validate_root_style(root_style, case)?;
    validate_item_style(item_style, case)?;
    validate_child_style(child_style, case)?;
    validate_grid_input(
        observation.grid_root,
        &observation.grid_container_input,
        case,
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
    let frames = relevant
        .iter()
        .map(|frame| (frame.node_id, (frame.x, frame.y, frame.width, frame.height)))
        .collect::<std::collections::BTreeMap<_, _>>();
    if relevant.len() != observation.node_ids.len() || frames.len() != observation.node_ids.len() {
        return Err(failure("missing, duplicate, or extra fixture node frames"));
    }
    for (node, expected) in observation.node_ids.iter().zip([
        (0.0, 0.0, WIDTH, HEIGHT),
        (0.0, 0.0, WIDTH, case.expected_row),
        (0.0, 0.0, WIDTH, CHILD_HEIGHT),
    ]) {
        if !frames
            .get(&node.get())
            .is_some_and(|actual| frame_matches(*actual, expected))
        {
            return Err(failure(
                "layout frame differs from the pinned CSS Grid formula",
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
    let row_passes = diagnostics.traces[0]
        .passes
        .iter()
        .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .collect::<Vec<_>>();
    if row_passes.len() != 1 {
        return Err(failure("expected exactly one Rows sizing pass"));
    }
    validate_rows_pass(row_passes[0], observation.node_ids[1], case)
}

pub(super) fn validate_identity(
    snapshot: &HostDocumentSnapshot,
    node: HostNodeHandle,
    expected_id: &str,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let (namespace, tag, id) = super::super::c11_4_image::element_identity(snapshot, node)?;
    if namespace != HTML_NAMESPACE || tag != "div" || id != expected_id {
        return Err(unsupported(
            node.id(),
            scene_id,
            "element namespace, tag, or ID changed",
        ));
    }
    Ok(())
}

pub(super) fn require_count(
    node: NodeId,
    actual: usize,
    expected: usize,
    scene_id: &str,
    detail: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if actual != expected {
        return Err(unsupported(node, scene_id, detail));
    }
    Ok(())
}

pub(super) fn validate_root_style(
    style: &ComputedElementStyle,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        layout_failure(
            "missing_computed_grid_layout",
            Some(style.node_id.get()),
            None,
        )
    })?;
    let fit_limit = ComputedCssGridAutoRows::TrackList(vec![ComputedCssGridTrackSize::FitContent(
        ComputedCssGridTrackLengthPercentage::LengthPx(case.limit),
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
        || style.properties.get("grid-auto-rows").map(String::as_str)
            != Some(if case.limit == 200.0 {
                "fit-content(200px)"
            } else if case.limit == 40.0 {
                "fit-content(40px)"
            } else {
                "fit-content(30px)"
            })
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
        || grid.auto_rows != fit_limit
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.columns_line_names.len() != 2
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || !grid.rows_line_names.is_empty()
    {
        return Err(unsupported(
            style.node_id,
            case.scene_id,
            "computed root style changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_item_style(
    style: &ComputedElementStyle,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
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
    {
        return Err(unsupported(
            style.node_id,
            case.scene_id,
            "grid item style changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_child_style(
    style: &ComputedElementStyle,
    case: Case,
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
            case.scene_id,
            "nested child style changed",
        ));
    }
    Ok(())
}

pub(super) fn validate_grid_input(
    root: NodeId,
    input: &GridContainerInput,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_rows = GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::LengthPx(case.limit),
    )]);
    if input.columns != [GridTrackSize::LengthPx(WIDTH)]
        || !input.rows.is_empty()
        || input.auto_rows != expected_rows
        || input.auto_flow != GridAutoFlow::Row
        || input.placement_mode != GridPlacementMode::TrackSizing
        || input.columns_auto_repeat.is_some()
        || input.rows_auto_repeat.is_some()
        || input.template_areas.is_some()
        || input.columns_line_names.len() != 2
        || input
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || !input.rows_line_names.is_empty()
    {
        return Err(unsupported(
            root,
            case.scene_id,
            "typed LayoutInput Grid container changed",
        ));
    }
    Ok(())
}

fn validate_rows_pass(
    pass: &LayoutGridTrackSizingPassTrace,
    item: NodeId,
    case: Case,
) -> Result<(), RuntimeLayoutFailure> {
    if pass.stages.is_empty() {
        return Err(failure("row sizing stages are missing"));
    }
    let max_content_contributions = pass
        .contributions
        .iter()
        .filter(|contribution| {
            contribution.item == item
                && contribution.span == 1
                && contribution.kind == LayoutGridTrackSizingContributionKind::MaxContent
        })
        .collect::<Vec<_>>();
    if max_content_contributions.len() != 1
        || (max_content_contributions[0].value - CHILD_HEIGHT).abs() > EPSILON
    {
        return Err(failure(
            "max-content contribution differs from the fixed 100px child",
        ));
    }
    let maximize = pass
        .stages
        .iter()
        .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::MaximizeTracks)
        .ok_or_else(|| failure("MaximizeTracks stage is missing"))?;
    let tracks = maximize
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .collect::<Vec<_>>();
    if tracks.len() != 1 {
        return Err(failure(&format!(
            "MaximizeTracks has {} active rows, expected one",
            tracks.len()
        )));
    }
    let track = tracks[0];
    // fit-content() caps the base-size distribution at its argument. The raw
    // growth limit can still be the finite max-content contribution.
    let expected_growth_limit = max_content_contributions[0].value;
    if (track.base_size - case.expected_row).abs() > EPSILON
        || track.growth_limit != Some(expected_growth_limit)
        || track.infinitely_growable
    {
        return Err(failure(&format!(
            "fit-content row stage differs: limit={} expected_base={} actual_base={} expected_growth_limit={} actual_growth_limit={:?} infinitely_growable={}",
            case.limit,
            case.expected_row,
            track.base_size,
            expected_growth_limit,
            track.growth_limit,
            track.infinitely_growable,
        )));
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
        "c114_fit_content_limit_boundaries_output_mismatch",
        None,
        Some(detail.to_owned()),
    )
}

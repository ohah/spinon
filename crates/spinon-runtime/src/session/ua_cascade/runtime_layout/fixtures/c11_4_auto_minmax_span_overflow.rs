use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::{
    GridContainerInput, GridPlacementMode, GridTrackMaxSizingFunction, GridTrackMinSizingFunction,
    GridTrackSize, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
    LayoutGridTrackSizingMinimumContributionSource, LayoutGridTrackSizingPassTrace, ParagraphStyle,
    TextLayoutEngine,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssOverflow, ComputedCssPosition,
    ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{
    RuntimeC11_4AutoMinmaxSpanOverflowObservation, RuntimeLayoutCompleted, RuntimeLayoutFailure,
    layout_failure,
};
use super::c11_4_image::element_identity;
use super::c11_4_nested_grid_replaced_item::{
    HTML_NAMESPACE, ahem_paragraph_style, children, spacing_is, style_for, unsupported, zero_edges,
};
use super::c11_4_text::AHEM;

const SCENE_PREFIX: &str = "c114-auto-minmax-span-overflow-";
const ITEM_ID: &str = "c114-auto-minmax-span-overflow-item";
const TEXT: &str = "XXXXXXXXXX";
const GRID_WIDTH: f32 = 80.0;
const GRID_HEIGHT: f32 = 20.0;
const MAX_CONTENT_CONTRIBUTION: f32 = 100.0;

#[derive(Clone, Copy)]
struct Case {
    scene_id: &'static str,
    root_id: &'static str,
    authored_overflow_x: &'static str,
    computed_overflow_x: ComputedCssOverflow,
    computed_overflow_y: ComputedCssOverflow,
    expected_tracks: [f32; 2],
    expected_item_width: f32,
    expected_minimum: f32,
    minimum_source: MinimumSource,
}

#[derive(Clone, Copy)]
enum MinimumSource {
    ContentBased,
    Overflow,
}

const CASES: [Case; 5] = [
    Case {
        scene_id: "columns-span2-visible-content-minimum",
        root_id: "c114-auto-minmax-span-overflow-visible",
        authored_overflow_x: "visible",
        computed_overflow_x: ComputedCssOverflow::Visible,
        computed_overflow_y: ComputedCssOverflow::Visible,
        expected_tracks: [50.0, 50.0],
        expected_item_width: 100.0,
        expected_minimum: 100.0,
        minimum_source: MinimumSource::ContentBased,
    },
    Case {
        scene_id: "columns-span2-clip-content-minimum",
        root_id: "c114-auto-minmax-span-overflow-clip",
        authored_overflow_x: "clip",
        computed_overflow_x: ComputedCssOverflow::Clip,
        computed_overflow_y: ComputedCssOverflow::Visible,
        expected_tracks: [50.0, 50.0],
        expected_item_width: 100.0,
        expected_minimum: 100.0,
        minimum_source: MinimumSource::ContentBased,
    },
    Case {
        scene_id: "columns-span2-hidden-zero-minimum",
        root_id: "c114-auto-minmax-span-overflow-hidden",
        authored_overflow_x: "hidden",
        computed_overflow_x: ComputedCssOverflow::Hidden,
        computed_overflow_y: ComputedCssOverflow::Auto,
        expected_tracks: [40.0, 40.0],
        expected_item_width: 80.0,
        expected_minimum: 0.0,
        minimum_source: MinimumSource::Overflow,
    },
    Case {
        scene_id: "columns-span2-auto-zero-minimum",
        root_id: "c114-auto-minmax-span-overflow-auto",
        authored_overflow_x: "auto",
        computed_overflow_x: ComputedCssOverflow::Auto,
        computed_overflow_y: ComputedCssOverflow::Auto,
        expected_tracks: [40.0, 40.0],
        expected_item_width: 80.0,
        expected_minimum: 0.0,
        minimum_source: MinimumSource::Overflow,
    },
    Case {
        scene_id: "columns-span2-scroll-zero-minimum",
        root_id: "c114-auto-minmax-span-overflow-scroll",
        authored_overflow_x: "scroll",
        computed_overflow_x: ComputedCssOverflow::Scroll,
        computed_overflow_y: ComputedCssOverflow::Auto,
        expected_tracks: [40.0, 40.0],
        expected_item_width: 80.0,
        expected_minimum: 0.0,
        minimum_source: MinimumSource::Overflow,
    },
];

pub(in crate::session::ua_cascade::runtime_layout) struct Input {
    case: &'static Case,
    pub(in crate::session::ua_cascade::runtime_layout) item: HostNodeHandle,
    pub(in crate::session::ua_cascade::runtime_layout) paragraph_styles:
        BTreeMap<NodeId, ParagraphStyle>,
    pub(in crate::session::ua_cascade::runtime_layout) engine: TextLayoutEngine,
}

pub(in crate::session::ua_cascade::runtime_layout) fn has_root_marker(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> bool {
    element_identity(snapshot, root).is_ok_and(|(_, _, id)| id.starts_with(SCENE_PREFIX))
}

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<Input>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    if !root_id.starts_with(SCENE_PREFIX) {
        return Ok(None);
    }
    let case = CASES
        .iter()
        .find(|case| case.root_id == root_id)
        .ok_or_else(|| {
            unsupported(
                root.id(),
                "overflow automatic minimum",
                "unknown fixed scene ID",
            )
        })?;
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(
            root.id(),
            case.scene_id,
            "profile or root identity",
        ));
    }

    let root_children = children(snapshot, root)?;
    require_child_count(
        root.id(),
        root_children.len(),
        1,
        case.scene_id,
        "Grid item count",
    )?;
    let item = root_children[0];
    if snapshot.parent(item) != Some(HostParent::Node(root)) {
        return Err(unsupported(
            item.id(),
            case.scene_id,
            "item is not a direct Grid child",
        ));
    }
    let (item_namespace, item_tag, item_id) = element_identity(snapshot, item)?;
    if item_namespace != HTML_NAMESPACE || item_tag != "div" || item_id != ITEM_ID {
        return Err(unsupported(
            item.id(),
            case.scene_id,
            "fixed direct item identity",
        ));
    }
    let text_children = children(snapshot, item)?;
    require_child_count(
        item.id(),
        text_children.len(),
        1,
        case.scene_id,
        "text child count",
    )?;
    let text_node = text_children[0];
    if snapshot.parent(text_node) != Some(HostParent::Node(item))
        || !snapshot.node(text_node).is_some_and(|node| {
            matches!(node.kind(), HostNodeKind::Text(value) if value.to_string_lossy() == TEXT)
        })
    {
        return Err(unsupported(text_node.id(), case.scene_id, "fixed Ahem text content"));
    }

    let root_style = style_for(styles, root)?;
    validate_root_style(root_style, case.scene_id)?;
    let item_style = style_for(styles, item)?;
    validate_item_style(item_style, *case)?;
    if styles.writing_style_for(item.id())
        != Some((
            spinon_core::WritingMode::HorizontalTb,
            spinon_core::TextOrientation::Mixed,
        ))
    {
        return Err(unsupported(
            item.id(),
            case.scene_id,
            "horizontal Ahem writing style",
        ));
    }

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c114_overflow_auto_minimum_font_registration",
            Some(root.id().get()),
            Some(error.to_string()),
        )
    })?;
    Ok(Some(Input {
        case,
        item,
        paragraph_styles: BTreeMap::from([(item.id(), ahem_paragraph_style())]),
        engine,
    }))
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    input: &Input,
    grid_container_inputs: &BTreeMap<NodeId, GridContainerInput>,
) -> Result<RuntimeC11_4AutoMinmaxSpanOverflowObservation, RuntimeLayoutFailure> {
    let root_style = style_for(styles, root)?;
    validate_root_style(root_style, input.case.scene_id)?;
    validate_item_style(style_for(styles, input.item)?, *input.case)?;
    let grid_container_input = grid_container_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_overflow_auto_minimum_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validate_grid_input(root.id(), grid_container_input, input.case.scene_id)?;
    if grid_container_inputs.len() != 1 {
        return Err(unsupported(
            root.id(),
            input.case.scene_id,
            "unexpected Grid container input",
        ));
    }
    Ok(RuntimeC11_4AutoMinmaxSpanOverflowObservation {
        style_snapshot: styles.clone(),
        grid_root: root.id(),
        grid_container_input: grid_container_input.clone(),
        item: input.item.id(),
        scene_id: input.case.scene_id.to_owned(),
        authored_overflow_x: input.case.authored_overflow_x.to_owned(),
        computed_overflow_x: input.case.computed_overflow_x,
        computed_overflow_y: input.case.computed_overflow_y,
        expected_track_base_sizes: input.case.expected_tracks,
        expected_item_width: input.case.expected_item_width,
        expected_minimum_value: input.case.expected_minimum,
        expects_content_based_minimum: matches!(
            input.case.minimum_source,
            MinimumSource::ContentBased
        ),
    })
}

pub(in crate::session::ua_cascade::runtime_layout) fn validate_output(
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| failure("Grid track sizing diagnostics are missing"))?;
    let observation = diagnostics
        .c11_4_overflow_auto_minimum_fixture
        .as_ref()
        .ok_or_else(|| failure("overflow fixture observation is missing"))?;
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

fn validate_root_style(
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

fn validate_item_style(
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

fn validate_grid_input(
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

fn require_child_count(
    node: NodeId,
    actual: usize,
    expected: usize,
    scene_id: &str,
    description: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if actual == expected {
        Ok(())
    } else {
        Err(unsupported(node, scene_id, description))
    }
}

fn failure(description: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "c114_overflow_auto_minimum_validation",
        None,
        Some(description.to_owned()),
    )
}

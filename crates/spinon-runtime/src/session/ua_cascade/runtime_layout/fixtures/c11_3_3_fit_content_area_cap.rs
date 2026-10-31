use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::{
    FontStyleMode, GridAutoColumns, GridPlacementMode, GridTrackLengthPercentage, GridTrackSize,
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LineBreakMode, ParagraphInput,
    ParagraphStyle, TextLayoutEngine, TextParagraphDirection, TextSource, WhiteSpaceMode,
    WordBreakMode,
};
use spinon_style::{
    ComputedCssGridAutoColumns, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
    ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutCompleted, RuntimeLayoutFailure, layout_failure};
use super::C11_3_3FitContentAreaCapInput;

const SCENE_ID: &str = "fit-content-area-cap";
const ROOT_ID: &str = "c1133-fit-content-area-cap-grid";
const FIXED_ID: &str = "c1133-fit-content-area-cap-fixed";
const ITEM_ID: &str = "c1133-fit-content-area-cap-item";
const TEXT: &str = "AA AA AA AA";
const AHEM: &[u8] = include_bytes!("../../../../../../../tests/fixtures/resources/c15/Ahem.ttf");

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_3_3FitContentAreaCapInput>, RuntimeLayoutFailure> {
    let (root_tag, root_id) = element_identity(snapshot, root)?;
    if root_id != ROOT_ID {
        return Ok(None);
    }
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTemplateV1
        || root_tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(root.id(), "root identity or profile"));
    }
    validate_grid_style(style_for(styles, root.id())?)?;

    let children = snapshot
        .children(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?
        .collect::<Vec<_>>();
    if children.len() != 2
        || children
            .iter()
            .any(|child| snapshot.parent(*child) != Some(HostParent::Node(root)))
    {
        return Err(unsupported(
            root.id(),
            "direct grid item identity or ownership",
        ));
    }
    let (fixed_tag, fixed_id) = element_identity(snapshot, children[0])?;
    let (item_tag, item_id) = element_identity(snapshot, children[1])?;
    if fixed_tag != "div" || fixed_id != FIXED_ID || item_tag != "div" || item_id != ITEM_ID {
        return Err(unsupported(root.id(), "direct grid item tag or ID"));
    }
    if snapshot
        .children(children[0])
        .is_none_or(|fixed_children| fixed_children.count() != 0)
    {
        return Err(unsupported(
            children[0].id(),
            "fixed area item must be empty",
        ));
    }
    let item_children = snapshot
        .children(children[1])
        .ok_or_else(|| layout_failure("missing_node", Some(children[1].id().get()), None))?
        .collect::<Vec<_>>();
    if item_children.len() != 1
        || snapshot.parent(item_children[0]) != Some(HostParent::Node(children[1]))
    {
        return Err(unsupported(
            children[1].id(),
            "text item must own one direct text node",
        ));
    }
    let text_node = item_children[0];
    let source = snapshot
        .node(text_node)
        .and_then(|node| match node.kind() {
            HostNodeKind::Text(text) => Some(text.to_string_lossy()),
            HostNodeKind::Element(_) => None,
        })
        .ok_or_else(|| unsupported(text_node.id(), "text source node kind"))?;
    if source != TEXT {
        return Err(unsupported(
            text_node.id(),
            "text content differs from fixed source",
        ));
    }

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        layout_failure(
            "c1133_fit_content_area_cap_font_registration",
            Some(root.id().get()),
            Some(error.to_string()),
        )
    })?;
    let paragraph_style = ahem_paragraph_style();
    let paragraph = ParagraphInput {
        node: children[1].id(),
        sources: vec![TextSource {
            node: text_node.id(),
            inline_ancestors: Vec::new(),
            text: source,
        }],
        style: paragraph_style.clone(),
        inline_styles: BTreeMap::new(),
        inline_run_styles: BTreeMap::new(),
    };
    let intrinsic = engine.intrinsic_inline_sizes(&paragraph).map_err(|error| {
        layout_failure(
            "c1133_fit_content_area_cap_intrinsic_measurement",
            Some(children[1].id().get()),
            Some(error.to_string()),
        )
    })?;
    if (intrinsic.min_content() - 20.0).abs() > f32::EPSILON
        || (intrinsic.max_content() - 110.0).abs() > f32::EPSILON
    {
        return Err(unsupported(
            root.id(),
            "Ahem min/max-content must be 20px/110px",
        ));
    }

    let node_ids = [root.id(), children[0].id(), children[1].id()];
    let style_ids = styles
        .elements
        .iter()
        .map(|style| style.node_id)
        .collect::<BTreeSet<_>>();
    if style_ids.len() != node_ids.len()
        || node_ids.iter().any(|node| !style_ids.contains(node))
        || styles.elements.len() != style_ids.len()
    {
        return Err(unsupported(root.id(), "computed style node set changed"));
    }

    Ok(Some(C11_3_3FitContentAreaCapInput {
        paragraph_styles: BTreeMap::from([(children[1].id(), paragraph_style)]),
        engine,
        node_ids,
        text_node: text_node.id(),
        min_content: intrinsic.min_content(),
        max_content: intrinsic.max_content(),
    }))
}

pub(in crate::session::ua_cascade::runtime_layout) fn is_root(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> bool {
    element_identity(snapshot, root).is_ok_and(|(tag, id)| tag == "div" && id == ROOT_ID)
}

pub(in crate::session::ua_cascade::runtime_layout) fn validate_layout(
    input: &C11_3_3FitContentAreaCapInput,
    output: &spinon_style_to_layout::StyleLayoutOutput,
) -> Result<(), RuntimeLayoutFailure> {
    let container = output
        .grid_container_inputs
        .get(&input.node_ids[0])
        .ok_or_else(|| unsupported(input.node_ids[0], "typed root Grid input missing"))?;
    let has_expected_auto_column = matches!(
        &container.auto_columns,
        GridAutoColumns::TrackList(tracks)
            if tracks.as_slice() == [GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0))]
    );
    let area_is_expected = container.template_areas.as_ref().is_some_and(|areas| {
        areas.rows == 1
            && areas.columns == 2
            && areas.areas.len() == 2
            && areas.areas.iter().any(|area| {
                area.name == "fixed"
                    && area.row_start == 0
                    && area.row_end == 1
                    && area.column_start == 0
                    && area.column_end == 1
            })
            && areas.areas.iter().any(|area| {
                area.name == "fit"
                    && area.row_start == 0
                    && area.row_end == 1
                    && area.column_start == 1
                    && area.column_end == 2
            })
    });
    let placement_is_expected = container.placement_mode == GridPlacementMode::TemplateGrid;
    let columns_are_expected = container.columns
        == [
            GridTrackSize::LengthPx(20.0),
            GridTrackSize::FitContent(GridTrackLengthPercentage::LengthPx(40.0)),
        ];
    let rows_are_expected = container.rows == [GridTrackSize::LengthPx(60.0)];
    if !placement_is_expected
        || !columns_are_expected
        || !rows_are_expected
        || !has_expected_auto_column
        || !area_is_expected
        || !container.track_sizing_trace_enabled
    {
        let trace = output
            .layout
            .grid_track_sizing_traces
            .first()
            .map(|trace| {
                format!(
                    "{} explicit={} implicit={}",
                    trace.used_columns.used_track_list,
                    trace.used_columns.explicit_track_count,
                    trace.used_columns.positive_implicit_track_count,
                )
            })
            .unwrap_or_else(|| "missing".to_owned());
        let detail = format!(
            "typed area-created fit-content input differs: placement={:?}, columns={:?}, rows={:?}, auto-columns={:?}, areas={area_is_expected}, trace_enabled={}, trace={trace}",
            container.placement_mode,
            container.columns,
            container.rows,
            container.auto_columns,
            container.track_sizing_trace_enabled,
        );
        return Err(unsupported(input.node_ids[0], &detail));
    }

    if output.layout.grid_track_sizing_traces.len() != 1 {
        return Err(unsupported(
            input.node_ids[0],
            "expected exactly one area track trace",
        ));
    }
    let trace = &output.layout.grid_track_sizing_traces[0];
    if trace.node != input.node_ids[0]
        || trace.used_columns.used_track_list != "20px 40px"
        || trace.used_columns.explicit_track_count != 2
        || trace.used_columns.positive_implicit_track_count != 0
        || !trace
            .passes
            .iter()
            .any(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        || !trace
            .passes
            .iter()
            .any(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
    {
        return Err(unsupported(
            input.node_ids[0],
            "area-created used columns or typed trace differ",
        ));
    }

    if output.layout.grid_item_intrinsic_inputs.len() != 1 {
        return Err(unsupported(
            input.node_ids[0],
            "text intrinsic input count differs",
        ));
    }
    let item_input = &output.layout.grid_item_intrinsic_inputs[0];
    let intrinsic_matches = item_input.node == input.node_ids[2]
        && item_input.grid_container == input.node_ids[0]
        && matches!(
        &item_input.source,
        LayoutGridItemIntrinsicSource::Text { sizes }
            if (sizes.min_content() - input.min_content).abs() <= f32::EPSILON
                && (sizes.max_content() - input.max_content).abs() <= f32::EPSILON
        );
    if !intrinsic_matches {
        return Err(unsupported(
            input.node_ids[2],
            "typed text min/max-content source differs",
        ));
    }

    for (node, expected) in input.node_ids.iter().zip([
        (0.0, 0.0, 200.0, 60.0),
        (0.0, 0.0, 20.0, 60.0),
        (20.0, 0.0, 40.0, 60.0),
    ]) {
        let frame = output
            .layout
            .frames
            .get(node)
            .ok_or_else(|| unsupported(*node, "layout frame missing"))?;
        if !frame_matches((frame.x, frame.y, frame.width, frame.height), expected) {
            return Err(unsupported(
                *node,
                "frame differs from Chrome CSS-pixel oracle",
            ));
        }
    }
    Ok(())
}

pub(in crate::session::ua_cascade::runtime_layout) fn validate_completed(
    input: &C11_3_3FitContentAreaCapInput,
    completed: &RuntimeLayoutCompleted,
) -> Result<(), RuntimeLayoutFailure> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .ok_or_else(|| unsupported(input.node_ids[0], "runtime trace diagnostics missing"))?;
    if diagnostics.traces.len() != 1
        || diagnostics.traces[0].node != input.node_ids[0]
        || diagnostics.item_inputs.len() != 1
        || diagnostics.item_inputs[0].node != input.node_ids[2]
        || diagnostics.item_text_layouts.len() != 1
        || diagnostics.item_text_layouts[0].input.node != input.node_ids[2]
        || diagnostics.item_text_layouts[0].input.sources.len() != 1
        || diagnostics.item_text_layouts[0].input.sources[0].node != input.text_node
        || diagnostics.item_text_layouts[0].input.sources[0]
            .text
            .as_str()
            != TEXT
    {
        return Err(unsupported(
            input.node_ids[0],
            "runtime text owner/source trace differs",
        ));
    }
    Ok(())
}

fn validate_grid_style(style: &ComputedElementStyle) -> Result<(), RuntimeLayoutFailure> {
    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| unsupported(style.node_id, "computed typed Grid style missing"))?;
    let expected_auto_column = matches!(
        &grid.auto_columns,
        ComputedCssGridAutoColumns::TrackList(tracks)
            if tracks.as_slice() == [ComputedCssGridTrackSize::FitContent(ComputedCssGridTrackLengthPercentage::LengthPx(40.0))]
    );
    let areas = grid.template_areas.as_ref();
    let expected_areas =
        areas.is_some_and(|areas| {
            areas.rows == 1
                && areas.columns == 2
                && areas.areas.len() == 2
                && areas.areas.iter().any(|area| {
                    area.name == "fixed" && area.column_start == 0 && area.column_end == 1
                })
                && areas.areas.iter().any(|area| {
                    area.name == "fit" && area.column_start == 1 && area.column_end == 2
                })
        });
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || grid.columns != [ComputedCssGridTrackSize::LengthPx(20.0)]
        || grid.rows != [ComputedCssGridTrackSize::LengthPx(60.0)]
        || !expected_auto_column
        || !expected_areas
    {
        return Err(unsupported(
            style.node_id,
            "computed area or fit-content style differs",
        ));
    }
    Ok(())
}

fn style_for(
    styles: &ComputedStyleSnapshot,
    node_id: NodeId,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == node_id)
        .ok_or_else(|| layout_failure("missing_computed_style", Some(node_id.get()), None))
}

fn element_identity(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
) -> Result<(String, String), RuntimeLayoutFailure> {
    let node = snapshot
        .node(handle)
        .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
    let HostNodeKind::Element(element) = node.kind() else {
        return Err(unsupported(handle.id(), "expected element"));
    };
    let id = element
        .attributes()
        .iter()
        .find_map(|(name, value)| {
            (name.namespace().is_none() && name.local_name() == "id")
                .then(|| value.to_string_lossy())
        })
        .unwrap_or_default();
    Ok((element.local_name().to_owned(), id))
}

fn ahem_paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size: 10.0,
        line_height: 10.0,
        line_height_is_normal: false,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space: WhiteSpaceMode::Normal,
        word_break: WordBreakMode::Normal,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: spinon_layout::LigatureMode::Normal,
        letter_spacing: 0.0,
        word_spacing: 0.0,
        text_indent: 0.0,
    }
}

fn frame_matches(actual: (f32, f32, f32, f32), expected: (f32, f32, f32, f32)) -> bool {
    let (x, y, width, height) = actual;
    let (expected_x, expected_y, expected_width, expected_height) = expected;
    [x, y, width, height]
        .into_iter()
        .zip([expected_x, expected_y, expected_width, expected_height])
        .all(|(actual, expected)| (actual - expected).abs() <= 1.0e-4)
}

fn unsupported(node: NodeId, reason: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c1133_fit_content_area_cap_fixture",
        Some(node.get()),
        Some(format!("{SCENE_ID}: {reason}")),
    )
}

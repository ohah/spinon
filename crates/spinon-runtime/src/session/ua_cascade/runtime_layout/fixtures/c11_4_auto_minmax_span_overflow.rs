use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::{
    GridContainerInput, LayoutIntrinsicSizeSnapshot, ParagraphStyle, TextLayoutEngine,
};
use spinon_style::{ComputedCssOverflow, ComputedStyleProfile, ComputedStyleSnapshot};

use super::super::{
    RuntimeC11_4AutoMinmaxSpanOverflowObservation, RuntimeLayoutFailure, layout_failure,
};
use super::c11_4_image::element_identity;
use super::c11_4_nested_grid_replaced_item::{
    HTML_NAMESPACE, ahem_paragraph_style, children, style_for, unsupported,
};
use super::c11_4_text::AHEM;

const SCENE_PREFIX: &str = "c114-auto-minmax-span-overflow-";
const ITEM_ID: &str = "c114-auto-minmax-span-overflow-item";
const TEXT: &str = "XXXXXXXXXX";
const GRID_WIDTH: f32 = 80.0;
const GRID_HEIGHT: f32 = 20.0;
const MAX_CONTENT_CONTRIBUTION: f32 = 100.0;

#[path = "c11_4_auto_minmax_span_overflow/single_track.rs"]
mod single_track;
pub(in crate::session::ua_cascade::runtime_layout) use single_track::observation as single_track_observation;

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
    case: Option<&'static Case>,
    single_track_case: Option<&'static single_track::Case>,
    pub(in crate::session::ua_cascade::runtime_layout) item: HostNodeHandle,
    pub(in crate::session::ua_cascade::runtime_layout) paragraph_styles:
        BTreeMap<NodeId, ParagraphStyle>,
    pub(in crate::session::ua_cascade::runtime_layout) engine: TextLayoutEngine,
    pub(in crate::session::ua_cascade::runtime_layout) intrinsic_sizes:
        Option<LayoutIntrinsicSizeSnapshot>,
}

impl Input {
    pub(in crate::session::ua_cascade::runtime_layout) fn has_span_case(&self) -> bool {
        self.case.is_some()
    }

    pub(in crate::session::ua_cascade::runtime_layout) fn has_single_track_case(&self) -> bool {
        self.single_track_case.is_some()
    }
}

pub(in crate::session::ua_cascade::runtime_layout) fn has_root_marker(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
) -> bool {
    element_identity(snapshot, root)
        .is_ok_and(|(_, _, id)| id.starts_with(SCENE_PREFIX) || single_track::has_root_id(&id))
}

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<Input>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    if single_track::has_root_id(&root_id) {
        return single_track::prepare(snapshot, root, styles, namespace, tag, &root_id);
    }
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
        case: Some(case),
        single_track_case: None,
        item,
        paragraph_styles: BTreeMap::from([(item.id(), ahem_paragraph_style())]),
        engine,
        intrinsic_sizes: None,
    }))
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    input: &Input,
    grid_container_inputs: &BTreeMap<NodeId, GridContainerInput>,
) -> Result<RuntimeC11_4AutoMinmaxSpanOverflowObservation, RuntimeLayoutFailure> {
    let case = input.case.ok_or_else(|| {
        layout_failure(
            "c114_overflow_span_observation_wrong_fixture",
            Some(root.id().get()),
            None,
        )
    })?;
    let root_style = style_for(styles, root)?;
    validate_root_style(root_style, case.scene_id)?;
    validate_item_style(style_for(styles, input.item)?, *case)?;
    let grid_container_input = grid_container_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_overflow_auto_minimum_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validate_grid_input(root.id(), grid_container_input, case.scene_id)?;
    if grid_container_inputs.len() != 1 {
        return Err(unsupported(
            root.id(),
            case.scene_id,
            "unexpected Grid container input",
        ));
    }
    Ok(RuntimeC11_4AutoMinmaxSpanOverflowObservation {
        style_snapshot: styles.clone(),
        grid_root: root.id(),
        grid_container_input: grid_container_input.clone(),
        item: input.item.id(),
        scene_id: case.scene_id.to_owned(),
        authored_overflow_x: case.authored_overflow_x.to_owned(),
        computed_overflow_x: case.computed_overflow_x,
        computed_overflow_y: case.computed_overflow_y,
        expected_track_base_sizes: case.expected_tracks,
        expected_item_width: case.expected_item_width,
        expected_minimum_value: case.expected_minimum,
        expects_content_based_minimum: matches!(case.minimum_source, MinimumSource::ContentBased),
    })
}

#[path = "c11_4_auto_minmax_span_overflow/span_validation.rs"]
mod span_validation;
pub(in crate::session::ua_cascade::runtime_layout) use span_validation::validate_output;
use span_validation::{validate_grid_input, validate_item_style, validate_root_style};

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

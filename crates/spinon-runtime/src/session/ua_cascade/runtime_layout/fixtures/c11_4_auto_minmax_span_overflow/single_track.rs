use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision, NodeId,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::{GridContainerInput, LayoutIntrinsicSizeSnapshot, TextLayoutEngine};
use spinon_style::{ComputedStyleProfile, ComputedStyleSnapshot};

use super::super::super::{
    RuntimeC11_4SingleTrackOverflowAutoMinimumObservation, RuntimeLayoutFailure, layout_failure,
};
use super::super::c11_4_image::element_identity;
use super::super::c11_4_nested_grid_replaced_item::{
    HTML_NAMESPACE, children, failure as fixture_failure, style_for,
};
use super::super::c11_4_text::AHEM;
use super::{Input, ahem_paragraph_style, failure, require_child_count, unsupported};

const ROOT_PREFIX: &str = "c114-auto-min-scrollable-overflow-";
const ITEM_ID: &str = "c114-auto-min-scrollable-overflow-item";
const TEXT: &str = "XXXXXXXXXX";
const IMAGE_URL: &str = "https://spinon.invalid/c11-4-overflow-auto-minimum-10x10.png";
const PNG: &[u8] = include_bytes!(
    "../../../../../../../../tests/fixtures/resources/c11-4-overflow-auto-minimum-10x10.png"
);
const COLUMNS_GRID_WIDTH: f32 = 60.0;
const COLUMNS_GRID_HEIGHT: f32 = 20.0;
const ROWS_GRID_WIDTH: f32 = 30.0;
const ROWS_GRID_HEIGHT: f32 = 5.0;

#[path = "single_track/cases.rs"]
mod cases;
pub(in crate::session::ua_cascade::runtime_layout) use cases::Case;
use cases::{CASES, ItemKind, MinimumSource};

pub(super) fn has_root_id(root_id: &str) -> bool {
    root_id.starts_with(ROOT_PREFIX)
}

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    namespace: String,
    tag: String,
    root_id: &str,
) -> Result<Option<Input>, RuntimeLayoutFailure> {
    let case = CASES
        .iter()
        .find(|case| case.root_id == root_id)
        .ok_or_else(|| {
            unsupported(
                root.id(),
                "overflow automatic minimum",
                "unknown single-track scene ID",
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
    let expected_tag = match case.item_kind {
        ItemKind::Text => "div",
        ItemKind::Image => "img",
    };
    if item_namespace != HTML_NAMESPACE || item_tag != expected_tag || item_id != ITEM_ID {
        return Err(unsupported(
            item.id(),
            case.scene_id,
            "fixed direct item identity",
        ));
    }
    let children = children(snapshot, item)?;
    match case.item_kind {
        ItemKind::Text => {
            require_child_count(
                item.id(),
                children.len(),
                1,
                case.scene_id,
                "text child count",
            )?;
            if snapshot.parent(children[0]) != Some(HostParent::Node(item))
                || !snapshot.node(children[0]).is_some_and(|node| {
                    matches!(node.kind(), HostNodeKind::Text(value) if value.to_string_lossy() == TEXT)
                })
            {
                return Err(unsupported(children[0].id(), case.scene_id, "fixed Ahem text content"));
            }
        }
        ItemKind::Image => {
            require_child_count(
                item.id(),
                children.len(),
                0,
                case.scene_id,
                "image must be a leaf",
            )?;
            if super::super::c11_4_image::image_src(snapshot, item)?.as_deref() != Some(IMAGE_URL) {
                return Err(unsupported(item.id(), case.scene_id, "fixed image URL"));
            }
        }
    }
    validation::validate_root_style(style_for(styles, root)?, case)?;
    validation::validate_item_style(style_for(styles, item)?, case)?;
    if styles.writing_style_for(item.id())
        != Some((
            spinon_core::WritingMode::HorizontalTb,
            spinon_core::TextOrientation::Mixed,
        ))
    {
        return Err(unsupported(
            item.id(),
            case.scene_id,
            "horizontal item writing style",
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
    let paragraph_styles = if matches!(case.item_kind, ItemKind::Text) {
        BTreeMap::from([(item.id(), ahem_paragraph_style())])
    } else {
        BTreeMap::new()
    };
    let intrinsic_sizes = if matches!(case.item_kind, ItemKind::Image) {
        Some(decode_image(snapshot, root, item, case.scene_id)?)
    } else {
        None
    };
    Ok(Some(Input {
        case: None,
        single_track_case: Some(case),
        item,
        paragraph_styles,
        engine,
        intrinsic_sizes,
    }))
}

fn decode_image(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    image: HostNodeHandle,
    scene_id: &str,
) -> Result<LayoutIntrinsicSizeSnapshot, RuntimeLayoutFailure> {
    let completion = ImageRequest::from_document(snapshot, image)
        .map_err(|error| {
            fixture_failure(
                "c114_overflow_auto_minimum_image_request",
                root,
                scene_id,
                error,
            )
        })?
        .decode(PNG)
        .map_err(|error| {
            fixture_failure(
                "c114_overflow_auto_minimum_image_decode",
                root,
                scene_id,
                error,
            )
        })?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| unsupported(root.id(), scene_id, "intrinsic revision exhausted"))?;
    let resources =
        ImageResourceSnapshot::new(snapshot, revision, [completion]).map_err(|error| {
            fixture_failure(
                "c114_overflow_auto_minimum_image_snapshot",
                root,
                scene_id,
                error,
            )
        })?;
    let decoded = resources
        .images()
        .get(&image.id())
        .ok_or_else(|| unsupported(image.id(), scene_id, "decoded image metadata missing"))?;
    let dimensions = decoded.dimensions();
    if dimensions.width() != 10 || dimensions.height() != 10 {
        return Err(unsupported(
            image.id(),
            scene_id,
            "fixed 10x10 PNG dimensions",
        ));
    }
    resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| {
            fixture_failure(
                "c114_overflow_auto_minimum_intrinsic_snapshot",
                root,
                scene_id,
                error,
            )
        })
}

pub(in crate::session::ua_cascade::runtime_layout) fn observation(
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    input: &Input,
    grid_container_inputs: &BTreeMap<NodeId, GridContainerInput>,
) -> Result<RuntimeC11_4SingleTrackOverflowAutoMinimumObservation, RuntimeLayoutFailure> {
    let case = input
        .single_track_case
        .ok_or_else(|| failure("single-track case missing"))?;
    validation::validate_root_style(style_for(styles, root)?, case)?;
    validation::validate_item_style(style_for(styles, input.item)?, case)?;
    let grid = grid_container_inputs.get(&root.id()).ok_or_else(|| {
        layout_failure(
            "c114_overflow_auto_minimum_grid_input_missing",
            Some(root.id().get()),
            None,
        )
    })?;
    validation::validate_grid_input(root.id(), grid, case)?;
    if grid_container_inputs.len() != 1 {
        return Err(unsupported(
            root.id(),
            case.scene_id,
            "unexpected Grid container input",
        ));
    }
    Ok(RuntimeC11_4SingleTrackOverflowAutoMinimumObservation {
        style_snapshot: styles.clone(),
        grid_root: root.id(),
        grid_container_input: grid.clone(),
        item: input.item.id(),
        scene_id: case.scene_id.to_owned(),
        axis: case.axis,
        authored_overflow_x: case.authored_overflow_x.to_owned(),
        authored_overflow_y: case.authored_overflow_y.to_owned(),
        computed_overflow_x: case.computed_overflow_x,
        computed_overflow_y: case.computed_overflow_y,
        expected_track_base_size: case.expected_track_base_size,
        expected_item_width: case.expected_item_width,
        expected_item_height: case.expected_item_height,
        expected_minimum_value: case.expected_minimum_value,
        expected_minimum_source: validation::to_runtime_source(case.minimum_source),
    })
}

#[path = "single_track/validation.rs"]
mod validation;
pub(in crate::session::ua_cascade::runtime_layout) use validation::validate_output;

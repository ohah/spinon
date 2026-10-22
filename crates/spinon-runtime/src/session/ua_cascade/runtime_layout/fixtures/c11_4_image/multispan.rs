use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostParent, IntrinsicSizeRevision};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::{IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG, element_identity, image_src};

const GRID_WIDTH: f32 = 40.0;
const GRID_HEIGHT: f32 = 60.0;

#[derive(Clone, Copy)]
struct Item {
    id: &'static str,
    height: f32,
    column_start: i32,
    column_span: i32,
    row: i32,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    items: &'static [Item],
    #[cfg(test)]
    tracks: &'static [f32],
}

const A_FORWARD: Item = Item {
    id: "image-a-forward",
    height: 16.0,
    column_start: 1,
    column_span: 2,
    row: 1,
};
const B_FORWARD: Item = Item {
    id: "image-b-forward",
    height: 32.0,
    column_start: 2,
    column_span: 2,
    row: 2,
};
const B_REVERSED: Item = Item {
    id: "image-b-reversed",
    ..B_FORWARD
};
const A_REVERSED: Item = Item {
    id: "image-a-reversed",
    ..A_FORWARD
};
const A_THREE: Item = Item {
    id: "image-c-a",
    ..A_FORWARD
};
const B_THREE: Item = Item {
    id: "image-c-b",
    ..B_FORWARD
};
const C_THREE: Item = Item {
    id: "image-c-c",
    height: 48.0,
    column_start: 1,
    column_span: 3,
    row: 3,
};
const A_THREE_REVERSED: Item = Item {
    id: "image-r-a",
    ..A_FORWARD
};
const B_THREE_REVERSED: Item = Item {
    id: "image-r-b",
    ..B_FORWARD
};
const C_THREE_REVERSED: Item = Item {
    id: "image-r-c",
    ..C_THREE
};

const SPAN2_FORWARD_ITEMS: [Item; 2] = [A_FORWARD, B_FORWARD];
const SPAN2_REVERSED_ITEMS: [Item; 2] = [B_REVERSED, A_REVERSED];
const SPAN2_THEN_SPAN3_ITEMS: [Item; 3] = [A_THREE, B_THREE, C_THREE];
const SPAN3_THEN_SPAN2_ITEMS: [Item; 3] = [C_THREE_REVERSED, B_THREE_REVERSED, A_THREE_REVERSED];

const SCENES: [Scene; 4] = [
    Scene {
        id: "image-span2-forward",
        items: &SPAN2_FORWARD_ITEMS,
        #[cfg(test)]
        tracks: &[12.0, 24.0, 24.0, 0.0],
    },
    Scene {
        id: "image-span2-reversed",
        items: &SPAN2_REVERSED_ITEMS,
        #[cfg(test)]
        tracks: &[12.0, 24.0, 24.0, 0.0],
    },
    Scene {
        id: "image-span2-before-span3",
        items: &SPAN2_THEN_SPAN3_ITEMS,
        #[cfg(test)]
        tracks: &[16.0, 28.0, 28.0, 0.0],
    },
    Scene {
        id: "image-span3-before-span2",
        items: &SPAN3_THEN_SPAN2_ITEMS,
        #[cfg(test)]
        tracks: &[16.0, 28.0, 28.0, 0.0],
    },
];

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::super::C11_4ImageInput>, super::RuntimeLayoutFailure> {
    let (root_namespace, root_tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene_id) = root_id
        .strip_prefix("c114-")
        .and_then(|id| id.strip_suffix("-grid"))
    else {
        return Ok(None);
    };
    let Some(scene) = SCENES.iter().find(|scene| scene.id == scene_id).copied() else {
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 {
        return Err(failure(
            "c114_image_multispan_profile_mismatch",
            root.id(),
            None,
        ));
    }
    if root_namespace != super::HTML_NAMESPACE
        || root_tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(
            root.id().get(),
            scene,
            "root element or parent",
        ));
    }

    let children = snapshot
        .children(root)
        .ok_or_else(|| failure("missing_node", root.id(), None))?
        .collect::<Vec<_>>();
    if children.len() != scene.items.len() {
        return Err(unsupported(
            root.id().get(),
            scene,
            "direct image item count",
        ));
    }

    let root_style = style_for(styles, root)?;
    validate_grid_style(root_style, scene)?;

    let mut completions = Vec::with_capacity(scene.items.len());
    for (index, (item_handle, item)) in children.iter().zip(scene.items).enumerate() {
        let (namespace, tag, id) = element_identity(snapshot, *item_handle)?;
        if namespace != super::HTML_NAMESPACE
            || tag != "img"
            || id != item.id
            || snapshot.parent(*item_handle) != Some(HostParent::Node(root))
            || snapshot
                .children(*item_handle)
                .is_none_or(|mut descendants| descendants.next().is_some())
            || image_src(snapshot, *item_handle)?.as_deref() != Some(IMAGE_URL)
        {
            return Err(unsupported(
                item_handle.id().get(),
                scene,
                "image identity, source, or direct-leaf topology",
            ));
        }
        let style = style_for(styles, *item_handle)?;
        validate_item_style(style, scene, *item, index)?;

        let completion = ImageRequest::from_document(snapshot, *item_handle)
            .map_err(|error| {
                failure(
                    "c114_image_multispan_request",
                    item_handle.id(),
                    Some(error.to_string()),
                )
            })?
            .decode(PNG)
            .map_err(|error| {
                failure(
                    "c114_image_multispan_decode",
                    item_handle.id(),
                    Some(error.to_string()),
                )
            })?;
        completions.push(completion);
    }

    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| failure("c114_image_multispan_revision_exhausted", root.id(), None))?;
    let resources =
        ImageResourceSnapshot::new(snapshot, revision, completions).map_err(|error| {
            failure(
                "c114_image_multispan_resource_snapshot",
                root.id(),
                Some(error.to_string()),
            )
        })?;
    for item_handle in &children {
        let image = resources.images().get(&item_handle.id()).ok_or_else(|| {
            failure(
                "c114_image_multispan_metadata_missing",
                item_handle.id(),
                None,
            )
        })?;
        let dimensions = image.dimensions();
        if dimensions.width() != NATURAL_WIDTH || dimensions.height() != NATURAL_HEIGHT {
            return Err(failure(
                "c114_image_multispan_metadata_mismatch",
                item_handle.id(),
                Some(format!("{}x{}", dimensions.width(), dimensions.height())),
            ));
        }
    }

    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| {
            failure(
                "c114_image_multispan_intrinsic_snapshot",
                root.id(),
                Some(error.to_string()),
            )
        })?;
    Ok(Some(super::super::C11_4ImageInput {
        intrinsic_sizes,
        allow_two_axis_root_intrinsic: false,
    }))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
) -> Result<(), super::RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "grid"),
        ("box-sizing", "border-box"),
        ("align-items", "start"),
        ("align-content", "start"),
        ("justify-content", "start"),
    ] {
        super::require_property(style, name, expected)?;
    }
    if style.layout_dimensions.width != ComputedCssDimension::LengthPx(GRID_WIDTH)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(GRID_HEIGHT)
        || !super::zero_edges(style.layout_spacing.margin)
        || !super::edges_are(style.layout_spacing.padding, 1.0)
        || !super::spacing_is(style.layout_spacing.column_gap, 0.0)
        || !super::spacing_is(style.layout_spacing.row_gap, 0.0)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || style.layout_position != spinon_style::ComputedCssPosition::Static
    {
        return Err(unsupported(style.node_id.get(), scene, "root box geometry"));
    }

    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        failure(
            "missing_c114_image_multispan_grid_layout",
            style.node_id,
            Some(scene.id.to_owned()),
        )
    })?;
    if grid.columns.len() != 4
        || grid
            .columns
            .iter()
            .any(|track| !matches!(track, ComputedCssGridTrackSize::MinContent))
        || grid.rows.len() != 3
        || grid.rows.iter().any(|track| {
            !matches!(track, ComputedCssGridTrackSize::LengthPx(value) if super::approximately(*value, 20.0))
        })
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_line_names.iter().any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported(style.node_id.get(), scene, "grid track functions"));
    }
    Ok(())
}

fn validate_item_style(
    style: &ComputedElementStyle,
    scene: Scene,
    item: Item,
    source_order: usize,
) -> Result<(), super::RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "block"),
        ("box-sizing", "border-box"),
        ("justify-self", "auto"),
        ("align-self", "auto"),
    ] {
        super::require_property(style, name, expected)?;
    }
    let expected_row_start = item.row;
    let expected_column_start = item.column_start;
    let expected_column_span = item.column_span;
    let placement_matches = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.row_start == ComputedCssGridPlacement::Line(expected_row_start)
            && placement.row_end == ComputedCssGridPlacement::Auto
            && placement.column_start == ComputedCssGridPlacement::Line(expected_column_start)
            && placement.column_end == ComputedCssGridPlacement::Span(expected_column_span)
    });
    if style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(item.height)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_aspect_ratio.is_some()
        || !super::zero_edges(style.layout_spacing.margin)
        || !super::zero_edges(style.layout_spacing.padding)
        || style.layout_border != spinon_style::ComputedLayoutBorder::default()
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || !placement_matches
    {
        return Err(unsupported(
            style.node_id.get(),
            scene,
            "replaced item sizing, placement, or source order",
        ));
    }
    if !item.height.is_finite() || item.height <= 0.0 || source_order >= scene.items.len() {
        return Err(unsupported(
            style.node_id.get(),
            scene,
            "invalid intrinsic item size",
        ));
    }
    Ok(())
}

fn style_for(
    styles: &ComputedStyleSnapshot,
    node: HostNodeHandle,
) -> Result<&ComputedElementStyle, super::RuntimeLayoutFailure> {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == node.id())
        .ok_or_else(|| failure("missing_computed_style", node.id(), None))
}

fn failure(
    code: &'static str,
    node: spinon_core::NodeId,
    detail: Option<String>,
) -> super::RuntimeLayoutFailure {
    super::super::super::layout_failure(code, Some(node.get()), detail)
}

fn unsupported(node_id: u64, scene: Scene, detail: &str) -> super::RuntimeLayoutFailure {
    failure(
        "unsupported_c114_image_multispan_case",
        spinon_core::NodeId::new(node_id).expect("host node ID is non-zero"),
        Some(format!("{}: {detail}", scene.id)),
    )
}

#[cfg(test)]
mod tests {
    use super::SCENES;

    #[test]
    fn exposes_four_order_independent_direct_image_multispan_scenes() {
        assert_eq!(SCENES.len(), 4);
        for (index, scene) in SCENES.iter().enumerate() {
            assert!(SCENES[..index].iter().all(|prior| prior.id != scene.id));
            assert!(matches!(scene.items.len(), 2 | 3));
            assert_eq!(scene.tracks.len(), 4);
        }
        assert_eq!(SCENES[0].items[0].height, 16.0);
        assert_eq!(SCENES[0].items[1].height, 32.0);
        assert_eq!(SCENES[0].tracks, [12.0, 24.0, 24.0, 0.0]);
        assert_eq!(SCENES[1].tracks, SCENES[0].tracks);
        assert_eq!(SCENES[2].items[2].height, 48.0);
        assert_eq!(SCENES[2].tracks, [16.0, 28.0, 28.0, 0.0]);
        assert_eq!(SCENES[3].tracks, SCENES[2].tracks);
    }
}

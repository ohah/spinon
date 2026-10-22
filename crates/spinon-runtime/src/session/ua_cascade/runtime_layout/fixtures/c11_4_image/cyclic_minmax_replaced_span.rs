use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostParent, IntrinsicSizeRevision};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMath, ComputedCssMaxSize, ComputedCssSpacingValue,
    ComputedElementStyle, ComputedLayoutBorder, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::{IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG, element_identity, image_src};

#[derive(Clone, Copy, Debug)]
struct Item {
    id: &'static str,
    height: f32,
    column: i32,
    column_end: i32,
}

#[derive(Clone, Copy, Debug)]
struct Scene {
    id: &'static str,
    root_size: ComputedCssDimension,
    items: &'static [Item; 3],
    two_axis_root_intrinsic: bool,
}

const A: Item = Item {
    id: "image-a",
    height: 8.0,
    column: 1,
    column_end: 3,
};
const B: Item = Item {
    id: "image-b",
    height: 12.0,
    column: 2,
    column_end: 4,
};
const C: Item = Item {
    id: "image-c",
    height: 16.0,
    column: 1,
    column_end: 4,
};
const FORWARD: [Item; 3] = [A, B, C];
const REVERSE: [Item; 3] = [C, B, A];

const SCENES: [Scene; 4] = [
    Scene {
        id: "cyclic-minmax-replaced-image-span-intrinsic-forward",
        root_size: ComputedCssDimension::MinContent,
        items: &FORWARD,
        two_axis_root_intrinsic: true,
    },
    Scene {
        id: "cyclic-minmax-replaced-image-span-intrinsic-reverse",
        root_size: ComputedCssDimension::MinContent,
        items: &REVERSE,
        two_axis_root_intrinsic: true,
    },
    Scene {
        id: "cyclic-minmax-replaced-image-span-definite-forward",
        root_size: ComputedCssDimension::LengthPx(80.0),
        items: &FORWARD,
        two_axis_root_intrinsic: false,
    },
    Scene {
        id: "cyclic-minmax-replaced-image-span-definite-reverse",
        root_size: ComputedCssDimension::LengthPx(80.0),
        items: &REVERSE,
        two_axis_root_intrinsic: false,
    },
];

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::super::C11_4ImageInput>, super::super::super::RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
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
        return Err(unsupported(
            root.id(),
            scene,
            "C11.4 track-sizing profile required",
        ));
    }
    if namespace != super::HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(root.id(), scene, "root tag or parent"));
    }
    let children = snapshot
        .children(root)
        .ok_or_else(|| failure("missing_node", root.id(), None))?
        .collect::<Vec<_>>();
    if children.len() != scene.items.len() {
        return Err(unsupported(root.id(), scene, "direct image item count"));
    }

    let root_style = style_for(styles, root)?;
    validate_grid_style(root_style, scene)?;
    let mut completions = Vec::with_capacity(scene.items.len());
    for (handle, item) in children.iter().zip(scene.items) {
        let (namespace, tag, id) = element_identity(snapshot, *handle)?;
        let expected_id = format!("c114-{}-{}", scene.id, item.id);
        if namespace != super::HTML_NAMESPACE
            || tag != "img"
            || id != expected_id
            || snapshot.parent(*handle) != Some(HostParent::Node(root))
            || snapshot
                .children(*handle)
                .is_none_or(|mut descendants| descendants.next().is_some())
            || image_src(snapshot, *handle)?.as_deref() != Some(IMAGE_URL)
        {
            return Err(unsupported(
                handle.id(),
                scene,
                "image id, source, or direct-leaf topology",
            ));
        }
        let item_style = style_for(styles, *handle)?;
        validate_item_style(item_style, scene, *item)?;
        let completion = ImageRequest::from_document(snapshot, *handle)
            .map_err(|error| {
                failure(
                    "c114_cyclic_image_request",
                    handle.id(),
                    Some(error.to_string()),
                )
            })?
            .decode(PNG)
            .map_err(|error| {
                failure(
                    "c114_cyclic_image_decode",
                    handle.id(),
                    Some(error.to_string()),
                )
            })?;
        completions.push(completion);
    }

    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| failure("c114_cyclic_image_revision_exhausted", root.id(), None))?;
    let resources =
        ImageResourceSnapshot::new(snapshot, revision, completions).map_err(|error| {
            failure(
                "c114_cyclic_image_snapshot",
                root.id(),
                Some(error.to_string()),
            )
        })?;
    for handle in &children {
        let decoded = resources
            .images()
            .get(&handle.id())
            .ok_or_else(|| failure("c114_cyclic_image_metadata_missing", handle.id(), None))?;
        let dimensions = decoded.dimensions();
        if dimensions.width() != NATURAL_WIDTH || dimensions.height() != NATURAL_HEIGHT {
            return Err(failure(
                "c114_cyclic_image_metadata_mismatch",
                handle.id(),
                Some(format!("{}x{}", dimensions.width(), dimensions.height())),
            ));
        }
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| {
            failure(
                "c114_cyclic_intrinsic_sizes",
                root.id(),
                Some(error.to_string()),
            )
        })?;
    Ok(Some(super::super::C11_4ImageInput {
        intrinsic_sizes,
        allow_two_axis_root_intrinsic: scene.two_axis_root_intrinsic,
    }))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
) -> Result<(), super::super::super::RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "grid"),
        ("box-sizing", "content-box"),
        ("align-items", "start"),
        ("align-content", "start"),
        ("justify-content", "start"),
    ] {
        super::require_property(style, name, expected)?;
    }
    if style.layout_dimensions.width != scene.root_size
        || style.layout_dimensions.height != scene.root_size
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_aspect_ratio.is_some()
        || !super::zero_edges(style.layout_spacing.margin)
        || !super::zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.column_gap, 2.0)
        || !spacing_is(style.layout_spacing.row_gap, 2.0)
        || style.layout_border
            != (ComputedLayoutBorder {
                top: 2.0,
                right: 2.0,
                bottom: 2.0,
                left: 2.0,
            })
        || style.layout_position != spinon_style::ComputedCssPosition::Static
    {
        return Err(unsupported(
            style.node_id,
            scene,
            "root size or border geometry",
        ));
    }
    let grid = style.grid_layout.as_ref().ok_or_else(|| {
        failure(
            "c114_cyclic_grid_missing",
            style.node_id,
            Some(scene.id.to_owned()),
        )
    })?;
    if grid.columns.len() != 3
        || !grid.columns.iter().all(track_matches)
        || grid.rows.len() != 2
        || !grid.rows.iter().all(track_matches)
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported(
            style.node_id,
            scene,
            "track list or auto-placement",
        ));
    }
    Ok(())
}

fn track_matches(track: &ComputedCssGridTrackSize) -> bool {
    matches!(
        track,
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::Calc(ComputedCssMath::Sum(terms)),
            max: ComputedCssGridTrackBreadth::MaxContent,
        } if terms.len() == 2
            && matches!(terms[0], ComputedCssMath::Percentage(value) if approximately(value, 0.1))
            && matches!(terms[1], ComputedCssMath::LengthPx(value) if approximately(value, 2.0))
    )
}

fn validate_item_style(
    style: &ComputedElementStyle,
    scene: Scene,
    item: Item,
) -> Result<(), super::super::super::RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "block"),
        ("box-sizing", "content-box"),
        ("justify-self", "auto"),
        ("align-self", "auto"),
    ] {
        super::require_property(style, name, expected)?;
    }
    let placement = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.row_start == ComputedCssGridPlacement::Line(1)
            && placement.row_end == ComputedCssGridPlacement::Line(3)
            && placement.column_start == ComputedCssGridPlacement::Line(item.column)
            && placement.column_end == ComputedCssGridPlacement::Line(item.column_end)
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
        || style.layout_border != ComputedLayoutBorder::default()
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || !placement
    {
        return Err(unsupported(
            style.node_id,
            scene,
            "image size, placement, or box edges",
        ));
    }
    Ok(())
}

fn spacing_is(actual: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(actual, ComputedCssSpacingValue::LengthPx(value) if approximately(value, expected))
}

fn approximately(actual: f32, expected: f32) -> bool {
    (actual - expected).abs() <= 1.0 / 64.0
}

fn style_for(
    styles: &ComputedStyleSnapshot,
    node: HostNodeHandle,
) -> Result<&ComputedElementStyle, super::super::super::RuntimeLayoutFailure> {
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
) -> super::super::super::RuntimeLayoutFailure {
    super::super::super::layout_failure(code, Some(node.get()), detail)
}

fn unsupported(
    node: spinon_core::NodeId,
    scene: Scene,
    detail: &str,
) -> super::super::super::RuntimeLayoutFailure {
    failure(
        "unsupported_c114_cyclic_minmax_replaced_image_span",
        node,
        Some(format!("{}: {detail}", scene.id)),
    )
}

#[cfg(test)]
mod tests {
    use super::SCENES;

    #[test]
    fn fixes_four_scene_ids_and_reverse_order_by_item_identity() {
        assert_eq!(SCENES.len(), 4);
        for (index, scene) in SCENES.iter().enumerate() {
            assert!(SCENES[..index].iter().all(|prior| prior.id != scene.id));
            assert_eq!(scene.items.len(), 3);
            assert!(
                scene
                    .items
                    .iter()
                    .all(|item| matches!(item.id, "image-a" | "image-b" | "image-c"))
            );
        }
        assert_eq!(
            SCENES[0]
                .items
                .iter()
                .map(|item| item.id)
                .collect::<Vec<_>>(),
            ["image-a", "image-b", "image-c"]
        );
        assert_eq!(
            SCENES[1]
                .items
                .iter()
                .map(|item| item.id)
                .collect::<Vec<_>>(),
            ["image-c", "image-b", "image-a"]
        );
        assert!(SCENES[0].two_axis_root_intrinsic && SCENES[1].two_axis_root_intrinsic);
        assert!(!SCENES[2].two_axis_root_intrinsic && !SCENES[3].two_axis_root_intrinsic);
    }
}

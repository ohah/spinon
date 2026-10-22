use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostParent, IntrinsicSizeRevision};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::RuntimeLayoutFailure;
use super::c11_4_image::{
    IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG, element_identity, image_src,
};
use super::c11_4_nested_grid_replaced_item::{
    children, failure, spacing_is, style_for, unsupported, validate_image_style, zero_edges,
};

const PREFIX: &str = "nested-grid-implicit-row-";
const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone)]
struct Scene {
    id: &'static str,
    nested_row_end: ComputedCssGridPlacement,
}

struct FixedBlockExpectation<'a> {
    scene: &'a Scene,
    suffix: &'a str,
    dimensions: (f32, f32),
    row_start: i32,
    row_end: i32,
}

const SCENES: [Scene; 2] = [
    Scene {
        id: "nested-grid-implicit-row-span-two",
        nested_row_end: ComputedCssGridPlacement::Line(3),
    },
    Scene {
        id: "nested-grid-implicit-row-one-row",
        nested_row_end: ComputedCssGridPlacement::Line(2),
    },
];

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4ImageInput>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene_id) = root_id
        .strip_prefix("c114-")
        .and_then(|value| value.strip_suffix("-grid"))
    else {
        return Ok(None);
    };
    if !scene_id.starts_with(PREFIX) {
        return Ok(None);
    }
    let Some(scene) = SCENES.iter().find(|scene| scene.id == scene_id).cloned() else {
        return Err(unsupported(root.id(), scene_id, "scene is not allowlisted"));
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(root.id(), scene.id, "profile or root identity"));
    }

    let root_children = children(snapshot, root)?;
    if root_children.len() != 1 {
        return Err(unsupported(root.id(), scene.id, "one nested Grid required"));
    }
    let nested = root_children[0];
    let (nested_namespace, nested_tag, nested_id) = element_identity(snapshot, nested)?;
    if nested_namespace != HTML_NAMESPACE
        || nested_tag != "div"
        || nested_id != format!("c114-{}-nested", scene.id)
        || snapshot.parent(nested) != Some(HostParent::Node(root))
    {
        return Err(unsupported(
            nested.id(),
            scene.id,
            "nested Grid identity or owner",
        ));
    }

    let nested_children = children(snapshot, nested)?;
    if nested_children.len() != 3 {
        return Err(unsupported(
            nested.id(),
            scene.id,
            "explicit row, first implicit row, and second implicit row items required",
        ));
    }
    let [explicit_item, first_implicit_item, image] = nested_children.as_slice() else {
        return Err(unsupported(nested.id(), scene.id, "nested child sequence"));
    };

    validate_grid_style(style_for(styles, root)?, &scene, true)?;
    validate_grid_style(style_for(styles, nested)?, &scene, false)?;
    validate_placement(
        style_for(styles, nested)?,
        &scene,
        ComputedCssGridPlacement::Line(1),
        scene.nested_row_end.clone(),
        ComputedCssGridPlacement::Auto,
        ComputedCssGridPlacement::Auto,
    )?;

    validate_fixed_block(
        snapshot,
        styles,
        *explicit_item,
        FixedBlockExpectation {
            scene: &scene,
            suffix: "explicit",
            dimensions: (24.0, 8.0),
            row_start: 1,
            row_end: 2,
        },
    )?;
    validate_fixed_block(
        snapshot,
        styles,
        *first_implicit_item,
        FixedBlockExpectation {
            scene: &scene,
            suffix: "implicit-min",
            dimensions: (4.0, 8.0),
            row_start: 2,
            row_end: 3,
        },
    )?;
    validate_image(snapshot, styles, *image, nested, &scene)?;

    let completion = ImageRequest::from_document(snapshot, *image)
        .map_err(|error| {
            failure(
                "c114_nested_implicit_row_image_request",
                *image,
                scene.id,
                error,
            )
        })?
        .decode(PNG)
        .map_err(|error| {
            failure(
                "c114_nested_implicit_row_image_decode",
                *image,
                scene.id,
                error,
            )
        })?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| unsupported(root.id(), scene.id, "intrinsic revision exhausted"))?;
    let resources =
        ImageResourceSnapshot::new(snapshot, revision, [completion]).map_err(|error| {
            failure(
                "c114_nested_implicit_row_image_snapshot",
                root,
                scene.id,
                error,
            )
        })?;
    let decoded = resources
        .images()
        .get(&image.id())
        .ok_or_else(|| unsupported(image.id(), scene.id, "decoded image metadata missing"))?;
    let dimensions = decoded.dimensions();
    if dimensions.width() != NATURAL_WIDTH || dimensions.height() != NATURAL_HEIGHT {
        return Err(unsupported(
            image.id(),
            scene.id,
            "PNG natural size changed",
        ));
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| {
            failure(
                "c114_nested_implicit_row_intrinsic_snapshot",
                root,
                scene.id,
                error,
            )
        })?;

    Ok(Some(super::C11_4ImageInput {
        intrinsic_sizes,
        allow_two_axis_root_intrinsic: false,
    }))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: &Scene,
    is_outer: bool,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_rows = if is_outer {
        vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MinContent,
        ]
    } else {
        vec![ComputedCssGridTrackSize::MinContent]
    };
    let expected_auto_rows = if is_outer {
        ComputedCssGridAutoRows::Auto
    } else {
        ComputedCssGridAutoRows::TrackList(vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
        ])
    };
    let Some(grid) = style.grid_layout.as_ref() else {
        return Err(unsupported(style.node_id, scene.id, "Grid style missing"));
    };
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(24.0)
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.column_gap, 0.0)
        || !spacing_is(style.layout_spacing.row_gap, 4.0)
        || style.layout_border != Default::default()
        || grid.columns != [ComputedCssGridTrackSize::LengthPx(24.0)]
        || grid.rows != expected_rows
        || grid.auto_rows != expected_auto_rows
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported(
            style.node_id,
            scene.id,
            if is_outer {
                "outer Grid style"
            } else {
                "nested Grid style"
            },
        ));
    }
    if !is_outer
        && (style.properties.get("align-items").map(String::as_str) != Some("start")
            || style.properties.get("justify-items").map(String::as_str) != Some("normal"))
    {
        return Err(unsupported(
            style.node_id,
            scene.id,
            "nested Grid item alignment",
        ));
    }
    Ok(())
}

fn validate_fixed_block(
    snapshot: &HostDocumentSnapshot,
    styles: &ComputedStyleSnapshot,
    node: HostNodeHandle,
    expected: FixedBlockExpectation<'_>,
) -> Result<(), RuntimeLayoutFailure> {
    let FixedBlockExpectation {
        scene,
        suffix,
        dimensions,
        row_start,
        row_end,
    } = expected;
    let (namespace, tag, id) = element_identity(snapshot, node)?;
    if namespace != HTML_NAMESPACE
        || tag != "div"
        || id != format!("c114-{}-{suffix}", scene.id)
        || snapshot.parent(node).is_none()
        || snapshot
            .children(node)
            .is_none_or(|mut children| children.next().is_some())
    {
        return Err(unsupported(
            node.id(),
            scene.id,
            "fixed row item identity or leaf",
        ));
    }
    let style = style_for(styles, node)?;
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(dimensions.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(dimensions.1)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
    {
        return Err(unsupported(node.id(), scene.id, "fixed row item box"));
    }
    validate_placement(
        style,
        scene,
        ComputedCssGridPlacement::Line(row_start),
        ComputedCssGridPlacement::Line(row_end),
        ComputedCssGridPlacement::Line(1),
        ComputedCssGridPlacement::Auto,
    )
}

fn validate_image(
    snapshot: &HostDocumentSnapshot,
    styles: &ComputedStyleSnapshot,
    image: HostNodeHandle,
    nested: HostNodeHandle,
    scene: &Scene,
) -> Result<(), RuntimeLayoutFailure> {
    let (namespace, tag, id) = element_identity(snapshot, image)?;
    if namespace != HTML_NAMESPACE
        || tag != "img"
        || id != format!("c114-{}-implicit-max", scene.id)
        || snapshot.parent(image) != Some(HostParent::Node(nested))
        || snapshot
            .children(image)
            .is_none_or(|mut children| children.next().is_some())
        || image_src(snapshot, image)?.as_deref() != Some(IMAGE_URL)
    {
        return Err(unsupported(
            image.id(),
            scene.id,
            "fixed natural PNG image leaf required",
        ));
    }
    validate_image_style(style_for(styles, image)?, scene.id)?;
    validate_placement(
        style_for(styles, image)?,
        scene,
        ComputedCssGridPlacement::Line(3),
        ComputedCssGridPlacement::Line(4),
        ComputedCssGridPlacement::Line(1),
        ComputedCssGridPlacement::Auto,
    )
}

fn validate_placement(
    style: &ComputedElementStyle,
    scene: &Scene,
    row_start: ComputedCssGridPlacement,
    row_end: ComputedCssGridPlacement,
    column_start: ComputedCssGridPlacement,
    column_end: ComputedCssGridPlacement,
) -> Result<(), RuntimeLayoutFailure> {
    let valid = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.row_start == row_start
            && placement.row_end == row_end
            && placement.column_start == column_start
            && placement.column_end == column_end
    });
    if valid {
        Ok(())
    } else {
        Err(unsupported(
            style.node_id,
            scene.id,
            "explicit row/column placement",
        ))
    }
}

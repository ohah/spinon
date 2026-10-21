use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision, NodeId,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::TextLayoutEngine;
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};
use super::{
    C11_4NestedAnonymousImageInput,
    c11_4_anonymous_text_image::anonymous_paragraph_style,
    c11_4_image::{IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG, element_identity, image_src},
    c11_4_nested_grid_depth_two::{
        AHEM_TEXT, children, element_identity as nested_element_identity, has_empty_attribute,
        style_for,
    },
    c11_4_text::AHEM,
};

const SCENE_IDS: [&str; 4] = [
    "depth-three-min-content-text-before-image",
    "depth-three-min-content-image-before-text",
    "depth-three-max-content-text-before-image",
    "depth-three-max-content-image-before-text",
];

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    constraint: Constraint,
    image_before_text: bool,
}

#[derive(Clone, Copy)]
enum Constraint {
    MinContent,
    MaxContent,
}

impl Constraint {
    fn dimension(self) -> ComputedCssDimension {
        match self {
            Self::MinContent => ComputedCssDimension::MinContent,
            Self::MaxContent => ComputedCssDimension::MaxContent,
        }
    }

    fn track(self) -> ComputedCssGridTrackSize {
        match self {
            Self::MinContent => ComputedCssGridTrackSize::MinContent,
            Self::MaxContent => ComputedCssGridTrackSize::MaxContent,
        }
    }
}

const SCENES: [Scene; 4] = [
    Scene {
        id: SCENE_IDS[0],
        constraint: Constraint::MinContent,
        image_before_text: false,
    },
    Scene {
        id: SCENE_IDS[1],
        constraint: Constraint::MinContent,
        image_before_text: true,
    },
    Scene {
        id: SCENE_IDS[2],
        constraint: Constraint::MaxContent,
        image_before_text: false,
    },
    Scene {
        id: SCENE_IDS[3],
        constraint: Constraint::MaxContent,
        image_before_text: true,
    },
];

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4NestedAnonymousImageInput>, RuntimeLayoutFailure> {
    let (root_tag, root_id) = nested_element_identity(snapshot, root)?;
    let Some(scene) = SCENES
        .iter()
        .find(|scene| format!("c114-{}-root", scene.id) == root_id)
        .copied()
    else {
        if root_id.starts_with("c114-depth-three-") {
            return Err(unsupported(root.id(), "unknown depth-three mixed scene"));
        }
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || root_tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
        || !has_empty_attribute(snapshot, root, "data-node")
    {
        return Err(unsupported(root.id(), "root profile, tag, owner or marker"));
    }

    let grid_ids = [
        format!("c114-{}-root", scene.id),
        format!("c114-{}-middle", scene.id),
        format!("c114-{}-branch", scene.id),
        format!("c114-{}-inner", scene.id),
    ];
    let mut grids = Vec::with_capacity(grid_ids.len());
    grids.push(root);
    for index in 0..grid_ids.len() {
        let grid = grids[index];
        let (tag, id) = nested_element_identity(snapshot, grid)?;
        if tag != "div"
            || id != grid_ids[index]
            || !has_empty_attribute(snapshot, grid, "data-node")
        {
            return Err(unsupported(grid.id(), "fixed Grid identity or marker"));
        }
        let style = style_for(styles, grid.id())?;
        validate_grid_style(style, scene, if index == 3 { 2 } else { 1 })?;
        let direct_children = children(snapshot, grid)?;
        if index == 3 {
            validate_direct_items(snapshot, styles, scene, grid, &direct_children)?;
        } else {
            if direct_children.len() != 1 {
                return Err(unsupported(grid.id(), "each outer Grid needs one child"));
            }
            let child = direct_children[0];
            let expected_child_id = &grid_ids[index + 1];
            let (child_tag, child_id) = nested_element_identity(snapshot, child)?;
            if child_tag != "div"
                || child_id != *expected_child_id
                || snapshot.parent(child) != Some(HostParent::Node(grid))
            {
                return Err(unsupported(child.id(), "nested Grid chain or owner"));
            }
            validate_grid_item_placement(style_for(styles, child.id())?, scene.id)?;
            grids.push(child);
        }
    }

    let inner = grids[3];
    let image = children(snapshot, inner)?
        .into_iter()
        .find(|handle| element_identity(snapshot, *handle).is_ok())
        .ok_or_else(|| unsupported(inner.id(), "direct image item missing"))?;
    let completion = ImageRequest::from_document(snapshot, image)
        .map_err(|error| failure("c114_nested_anonymous_image_request", image, scene, error))?
        .decode(PNG)
        .map_err(|error| failure("c114_nested_anonymous_image_decode", image, scene, error))?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| unsupported(root.id(), "intrinsic revision exhausted"))?;
    let resources = ImageResourceSnapshot::new(snapshot, revision, [completion])
        .map_err(|error| failure("c114_nested_anonymous_image_snapshot", root, scene, error))?;
    let decoded = resources
        .images()
        .get(&image.id())
        .ok_or_else(|| unsupported(image.id(), "decoded image metadata missing"))?;
    let dimensions = decoded.dimensions();
    if dimensions.width() != NATURAL_WIDTH || dimensions.height() != NATURAL_HEIGHT {
        return Err(unsupported(image.id(), "PNG natural dimensions changed"));
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| failure("c114_nested_anonymous_image_intrinsic", root, scene, error))?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", AHEM)
        .map_err(|error| failure("c114_nested_anonymous_image_font", root, scene, error))?;
    Ok(Some(C11_4NestedAnonymousImageInput {
        paragraph_styles: BTreeMap::from([(inner.id(), anonymous_paragraph_style())]),
        engine,
        intrinsic_sizes,
    }))
}

fn validate_direct_items(
    snapshot: &HostDocumentSnapshot,
    styles: &ComputedStyleSnapshot,
    scene: Scene,
    inner: HostNodeHandle,
    direct_children: &[HostNodeHandle],
) -> Result<(), RuntimeLayoutFailure> {
    if direct_children.len() != 2 {
        return Err(unsupported(
            inner.id(),
            "inner Grid needs exactly two direct items",
        ));
    }
    let (image, text) = if scene.image_before_text {
        (direct_children[0], direct_children[1])
    } else {
        (direct_children[1], direct_children[0])
    };
    let text_value = snapshot.node(text).and_then(|node| match node.kind() {
        HostNodeKind::Text(value) => Some(value.to_string_lossy()),
        HostNodeKind::Element(_) => None,
    });
    if text_value.as_deref() != Some(AHEM_TEXT)
        || snapshot.parent(text) != Some(HostParent::Node(inner))
    {
        return Err(unsupported(
            text.id(),
            "anonymous Ahem text or source order",
        ));
    }

    let (_, image_tag, image_id) = element_identity(snapshot, image)?;
    if image_tag != "img"
        || image_id != format!("c114-{}-image", scene.id)
        || snapshot.parent(image) != Some(HostParent::Node(inner))
        || !has_empty_attribute(snapshot, image, "data-node")
        || image_src(snapshot, image)?.as_deref() != Some(IMAGE_URL)
        || !children(snapshot, image)?.is_empty()
    {
        return Err(unsupported(image.id(), "fixed direct HTML image leaf"));
    }
    validate_grid_item_placement(style_for(styles, image.id())?, scene.id)?;
    validate_natural_image_style(style_for(styles, image.id())?)?;
    Ok(())
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
    column_count: usize,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| unsupported(style.node_id, "Grid style missing"))?;
    let track = scene.constraint.track();
    let expected_columns = vec![track.clone(); column_count];
    let expected_line_names = column_count + 1;
    let dimension = scene.constraint.dimension();
    let spacing_zero = |value| matches!(value, ComputedCssSpacingValue::LengthPx(actual) if actual.abs() <= f32::EPSILON);
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.properties.get("align-items").map(String::as_str) != Some("start")
        || style.properties.get("font-family").map(String::as_str) != Some("Ahem")
        || style.properties.get("font-size").map(String::as_str) != Some("10px")
        || style.properties.get("font-weight").map(String::as_str) != Some("400")
        || style.properties.get("font-style").map(String::as_str) != Some("normal")
        || style.properties.get("line-height").map(String::as_str) != Some("10px")
        || style
            .properties
            .get("white-space-collapse")
            .map(String::as_str)
            != Some("collapse")
        || style.properties.get("text-wrap-mode").map(String::as_str) != Some("wrap")
        || style.properties.get("word-break").map(String::as_str) != Some("normal")
        || style.properties.get("direction").map(String::as_str) != Some("ltr")
        || style.properties.get("line-break").map(String::as_str) != Some("auto")
        || style.properties.get("text-align").map(String::as_str) != Some("start")
        || style.properties.get("overflow-wrap").map(String::as_str) != Some("normal")
        || style.properties.get("text-transform").map(String::as_str) != Some("none")
        || style.properties.get("letter-spacing").map(String::as_str) != Some("normal")
        || style.properties.get("word-spacing").map(String::as_str) != Some("0px")
        || style.properties.get("text-indent").map(String::as_str) != Some("0px")
        || style.layout_dimensions.width != dimension
        || style.layout_dimensions.height != dimension
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || !zero_edges(style.layout_spacing.margin, spacing_zero)
        || !zero_edges(style.layout_spacing.padding, spacing_zero)
        || !spacing_zero(style.layout_spacing.column_gap)
        || !spacing_zero(style.layout_spacing.row_gap)
        || style.layout_border != Default::default()
        || grid.columns != expected_columns
        || grid.rows != [track]
        || grid.columns_line_names.len() != expected_line_names
        || grid.rows_line_names.len() != 2
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
            "fixed intrinsic Grid style changed",
        ));
    }
    Ok(())
}

fn validate_natural_image_style(style: &ComputedElementStyle) -> Result<(), RuntimeLayoutFailure> {
    let dimension_auto = style.layout_dimensions.width == ComputedCssDimension::Auto
        && style.layout_dimensions.height == ComputedCssDimension::Auto
        && style.layout_dimensions.min_width == ComputedCssDimension::Auto
        && style.layout_dimensions.min_height == ComputedCssDimension::Auto
        && style.layout_dimensions.max_width == ComputedCssMaxSize::None
        && style.layout_dimensions.max_height == ComputedCssMaxSize::None;
    if !matches!(
        style.properties.get("display").map(String::as_str),
        Some("inline" | "block")
    ) || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || !dimension_auto
        || style.layout_aspect_ratio.is_some()
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || !zero_edges(
            style.layout_spacing.margin,
            |value| matches!(value, ComputedCssSpacingValue::LengthPx(actual) if actual.abs() <= f32::EPSILON),
        )
        || !zero_edges(
            style.layout_spacing.padding,
            |value| matches!(value, ComputedCssSpacingValue::LengthPx(actual) if actual.abs() <= f32::EPSILON),
        )
        || style.layout_border != Default::default()
        || !matches!(
            style.properties.get("align-self").map(String::as_str),
            Some("auto" | "start")
        )
        || !matches!(
            style.properties.get("justify-self").map(String::as_str),
            Some("auto" | "start")
        )
    {
        return Err(unsupported(
            style.node_id,
            "image must retain natural dimensions",
        ));
    }
    Ok(())
}

fn validate_grid_item_placement(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let placement = style
        .grid_item_placement
        .as_ref()
        .ok_or_else(|| unsupported(style.node_id, "Grid item placement missing"))?;
    if placement.row_start != ComputedCssGridPlacement::Auto
        || placement.row_end != ComputedCssGridPlacement::Auto
        || placement.column_start != ComputedCssGridPlacement::Auto
        || placement.column_end != ComputedCssGridPlacement::Auto
    {
        return Err(unsupported(style.node_id, scene_id));
    }
    Ok(())
}

fn zero_edges(
    edges: spinon_style::ComputedCssEdges,
    is_zero: impl Fn(ComputedCssSpacingValue) -> bool,
) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(is_zero)
}

fn unsupported(node: NodeId, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_nested_grid_depth_three_anonymous_image",
        Some(node.get()),
        Some(detail.to_owned()),
    )
}

fn failure(
    code: &'static str,
    node: HostNodeHandle,
    scene: Scene,
    error: impl std::fmt::Display,
) -> RuntimeLayoutFailure {
    layout_failure(
        code,
        Some(node.id().get()),
        Some(format!("{}: {error}", scene.id)),
    )
}

#[cfg(test)]
mod tests {
    use super::{Constraint, SCENE_IDS, SCENES};
    use std::collections::BTreeSet;

    #[test]
    fn scene_matrix_is_four_unique_depth_three_ordered_intrinsic_inputs() {
        assert_eq!(SCENES.len(), 4);
        assert_eq!(SCENE_IDS.iter().copied().collect::<BTreeSet<_>>().len(), 4);
        let actual = SCENES.map(|scene| {
            (
                scene.id,
                matches!(scene.constraint, Constraint::MinContent),
                scene.image_before_text,
            )
        });
        assert_eq!(
            actual,
            [
                (SCENE_IDS[0], true, false),
                (SCENE_IDS[1], true, true),
                (SCENE_IDS[2], false, false),
                (SCENE_IDS[3], false, true),
            ]
        );
    }
}

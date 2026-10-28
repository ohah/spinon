use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::TextLayoutEngine;
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::RuntimeLayoutFailure;
use super::c11_4_image::{
    IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG, element_identity, image_src,
};
use super::c11_4_nested_grid_replaced_item::{
    HTML_NAMESPACE, TEXT, ahem_paragraph_style, children, failure, spacing_is, style_for,
    unsupported, validate_image_style, validate_text_style, zero_edges,
};
use super::c11_4_text::AHEM;

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    root_height: RootHeight,
    outer_rows: OuterRows,
    nested_row_end: NestedRowEnd,
}

#[derive(Clone, Copy)]
enum RootHeight {
    Auto,
    Fixed60,
}

#[derive(Clone, Copy)]
enum OuterRows {
    MinContentPair,
    AutoMinFlexPair,
}

#[derive(Clone, Copy)]
enum NestedRowEnd {
    SpanTwo,
    LineTwo,
}

const SCENES: [Scene; 3] = [
    Scene {
        id: "nested-grid-text-image-row-span-two",
        root_height: RootHeight::Auto,
        outer_rows: OuterRows::MinContentPair,
        nested_row_end: NestedRowEnd::SpanTwo,
    },
    Scene {
        id: "nested-grid-text-image-row-one-track",
        root_height: RootHeight::Auto,
        outer_rows: OuterRows::MinContentPair,
        nested_row_end: NestedRowEnd::LineTwo,
    },
    Scene {
        id: "nested-grid-text-image-row-definite-flex-control",
        root_height: RootHeight::Fixed60,
        outer_rows: OuterRows::AutoMinFlexPair,
        nested_row_end: NestedRowEnd::SpanTwo,
    },
];

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
    scene_id: &str,
) -> Result<Option<super::C11_4MixedInput>, RuntimeLayoutFailure> {
    let Some(scene) = SCENES.iter().find(|scene| scene.id == scene_id).copied() else {
        return Err(unsupported(
            root.id(),
            scene_id,
            "row-axis scene is not allowlisted",
        ));
    };
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || namespace != HTML_NAMESPACE
        || tag != "div"
        || root_id != format!("c114-{}-grid", scene.id)
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(root.id(), scene.id, "profile or root identity"));
    }

    let root_children = children(snapshot, root)?;
    if root_children.len() != 1 {
        return Err(unsupported(
            root.id(),
            scene.id,
            "one nested Grid child required",
        ));
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
    if nested_children.len() != 2 {
        return Err(unsupported(
            nested.id(),
            scene.id,
            "one text item and one image item required",
        ));
    }
    let text_item = nested_children[0];
    let image = nested_children[1];
    validate_grid_style(style_for(styles, root)?, scene, true)?;
    validate_grid_style(style_for(styles, nested)?, scene, false)?;
    validate_placement(
        style_for(styles, nested)?,
        scene.id,
        ComputedCssGridPlacement::Line(1),
        match scene.nested_row_end {
            NestedRowEnd::SpanTwo => ComputedCssGridPlacement::Span(2),
            NestedRowEnd::LineTwo => ComputedCssGridPlacement::Line(2),
        },
        ComputedCssGridPlacement::Auto,
        ComputedCssGridPlacement::Auto,
    )?;

    let (text_namespace, text_tag, text_id) = element_identity(snapshot, text_item)?;
    if text_namespace != HTML_NAMESPACE
        || text_tag != "div"
        || text_id != format!("c114-{}-text", scene.id)
        || snapshot.parent(text_item) != Some(HostParent::Node(nested))
    {
        return Err(unsupported(
            text_item.id(),
            scene.id,
            "text item identity or owner",
        ));
    }
    let text_children = children(snapshot, text_item)?;
    if text_children.len() != 1
        || !snapshot.node(text_children[0]).is_some_and(
            |node| matches!(node.kind(), HostNodeKind::Text(text) if text.to_string_lossy() == TEXT),
        )
    {
        return Err(unsupported(text_item.id(), scene.id, "fixed direct Ahem text leaf required"));
    }
    validate_text_style(style_for(styles, text_item)?, scene.id)?;
    validate_placement(
        style_for(styles, text_item)?,
        scene.id,
        ComputedCssGridPlacement::Line(1),
        ComputedCssGridPlacement::Auto,
        ComputedCssGridPlacement::Line(1),
        ComputedCssGridPlacement::Auto,
    )?;

    let (image_namespace, image_tag, image_id) = element_identity(snapshot, image)?;
    if image_namespace != HTML_NAMESPACE
        || image_tag != "img"
        || image_id != format!("c114-{}-image", scene.id)
        || snapshot.parent(image) != Some(HostParent::Node(nested))
        || snapshot
            .children(image)
            .is_none_or(|mut descendants| descendants.next().is_some())
        || image_src(snapshot, image)?.as_deref() != Some(IMAGE_URL)
    {
        return Err(unsupported(
            image.id(),
            scene.id,
            "fixed direct HTML image leaf/source required",
        ));
    }
    validate_image_style(style_for(styles, image)?, scene.id)?;
    validate_placement(
        style_for(styles, image)?,
        scene.id,
        ComputedCssGridPlacement::Line(2),
        ComputedCssGridPlacement::Auto,
        ComputedCssGridPlacement::Line(2),
        ComputedCssGridPlacement::Auto,
    )?;

    let completion = ImageRequest::from_document(snapshot, image)
        .map_err(|error| {
            failure(
                "c114_nested_grid_rows_image_request",
                image,
                scene.id,
                error,
            )
        })?
        .decode(PNG)
        .map_err(|error| failure("c114_nested_grid_rows_image_decode", image, scene.id, error))?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| unsupported(root.id(), scene.id, "intrinsic revision exhausted"))?;
    let resources =
        ImageResourceSnapshot::new(snapshot, revision, [completion]).map_err(|error| {
            failure(
                "c114_nested_grid_rows_image_snapshot",
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
                "c114_nested_grid_rows_intrinsic_snapshot",
                root,
                scene.id,
                error,
            )
        })?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine.register_font("Ahem", AHEM).map_err(|error| {
        failure(
            "c114_nested_grid_rows_font_registration",
            root,
            scene.id,
            error,
        )
    })?;
    Ok(Some(super::C11_4MixedInput {
        paragraph_styles: BTreeMap::from([(text_item.id(), ahem_paragraph_style())]),
        engine,
        intrinsic_sizes,
        use_extended_fixture_projection: false,
    }))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
    is_outer: bool,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_width = if is_outer {
        ComputedCssDimension::LengthPx(98.0)
    } else {
        ComputedCssDimension::Auto
    };
    let expected_height = if is_outer {
        match scene.root_height {
            RootHeight::Auto => ComputedCssDimension::Auto,
            RootHeight::Fixed60 => ComputedCssDimension::LengthPx(60.0),
        }
    } else {
        ComputedCssDimension::Auto
    };
    let expected_columns = if is_outer {
        vec![ComputedCssGridTrackSize::LengthPx(98.0)]
    } else {
        vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
        ]
    };
    let expected_rows = if is_outer {
        match scene.outer_rows {
            OuterRows::MinContentPair => vec![
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::MinContent,
            ],
            OuterRows::AutoMinFlexPair => vec![auto_min_flex_track(), auto_min_flex_track()],
        }
    } else {
        vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MinContent,
        ]
    };
    let expected_column_gap = if is_outer { 0.0 } else { 4.0 };
    let Some(grid) = style.grid_layout.as_ref() else {
        return Err(unsupported(style.node_id, scene.id, "Grid style missing"));
    };
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || style.layout_dimensions.width != expected_width
        || style.layout_dimensions.height != expected_height
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.column_gap, expected_column_gap)
        || !spacing_is(style.layout_spacing.row_gap, 4.0)
        || style.layout_border != Default::default()
        || grid.columns != expected_columns
        || grid.rows != expected_rows
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
            scene.id,
            if is_outer {
                "outer row Grid style"
            } else {
                "nested row Grid style"
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
            "nested Grid alignment",
        ));
    }
    Ok(())
}

fn auto_min_flex_track() -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::Auto,
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    }
}

fn validate_placement(
    style: &ComputedElementStyle,
    scene_id: &str,
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
        Err(unsupported(style.node_id, scene_id, "row/column placement"))
    }
}

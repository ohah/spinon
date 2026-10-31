use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle,
    ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};
use super::c11_4_image::{
    IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG, element_identity, image_src,
};
use super::c11_4_text::AHEM;

pub(super) const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub(super) const TEXT: &str = "XX XXXXXXX";

#[derive(Clone)]
struct Scene {
    id: &'static str,
    width: RootWidth,
    columns: OuterColumns,
    nested_column_end: ComputedCssGridPlacement,
}

#[derive(Clone, Copy)]
enum RootWidth {
    MaxContent,
    Fixed120,
}

#[derive(Clone, Copy)]
enum OuterColumns {
    MinContentPair,
    AutoMinFlexPair,
}

const SCENES: [Scene; 3] = [
    Scene {
        id: "nested-grid-text-image-span-two",
        width: RootWidth::MaxContent,
        columns: OuterColumns::MinContentPair,
        nested_column_end: ComputedCssGridPlacement::Span(2),
    },
    Scene {
        id: "nested-grid-text-image-one-track",
        width: RootWidth::MaxContent,
        columns: OuterColumns::MinContentPair,
        nested_column_end: ComputedCssGridPlacement::Line(2),
    },
    Scene {
        id: "nested-grid-text-image-definite-flex-control",
        width: RootWidth::Fixed120,
        columns: OuterColumns::AutoMinFlexPair,
        nested_column_end: ComputedCssGridPlacement::Span(2),
    },
];

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4MixedInput>, RuntimeLayoutFailure> {
    let (namespace, tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene_id) = root_id
        .strip_prefix("c114-")
        .and_then(|value| value.strip_suffix("-grid"))
    else {
        return Ok(None);
    };
    if scene_id.starts_with("nested-grid-text-image-row-") {
        return super::c11_4_nested_grid_rows::prepare(snapshot, root, styles, scene_id);
    }
    let Some(scene) = SCENES.iter().find(|scene| scene.id == scene_id).cloned() else {
        return if scene_id.starts_with("nested-grid-text-image-") {
            Err(unsupported(root.id(), scene_id, "scene is not allowlisted"))
        } else {
            Ok(None)
        };
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
    validate_grid_style(style_for(styles, root)?, &scene, true)?;
    validate_grid_style(style_for(styles, nested)?, &scene, false)?;
    validate_placement(
        style_for(styles, nested)?,
        scene.id,
        ComputedCssGridPlacement::Line(1),
        scene.nested_column_end.clone(),
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
        || !snapshot
            .node(text_children[0])
            .is_some_and(|node| matches!(node.kind(), HostNodeKind::Text(text) if text.to_string_lossy() == TEXT))
    {
        return Err(unsupported(
            text_item.id(),
            scene.id,
            "fixed direct Ahem text leaf required",
        ));
    }
    validate_text_style(style_for(styles, text_item)?, scene.id)?;
    validate_placement(
        style_for(styles, text_item)?,
        scene.id,
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
    )?;

    let completion = ImageRequest::from_document(snapshot, image)
        .map_err(|error| failure("c114_nested_grid_image_request", image, scene.id, error))?
        .decode(PNG)
        .map_err(|error| failure("c114_nested_grid_image_decode", image, scene.id, error))?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| unsupported(root.id(), scene.id, "intrinsic revision exhausted"))?;
    let resources = ImageResourceSnapshot::new(snapshot, revision, [completion])
        .map_err(|error| failure("c114_nested_grid_image_snapshot", root, scene.id, error))?;
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
        .map_err(|error| failure("c114_nested_grid_intrinsic_snapshot", root, scene.id, error))?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", AHEM)
        .map_err(|error| failure("c114_nested_grid_font_registration", root, scene.id, error))?;
    Ok(Some(super::C11_4MixedInput {
        paragraph_styles: BTreeMap::from([(text_item.id(), ahem_paragraph_style())]),
        engine,
        intrinsic_sizes,
        use_extended_fixture_projection: false,
    }))
}

pub(super) fn children(
    snapshot: &HostDocumentSnapshot,
    node: HostNodeHandle,
) -> Result<Vec<HostNodeHandle>, RuntimeLayoutFailure> {
    snapshot
        .children(node)
        .map(|children| children.collect())
        .ok_or_else(|| layout_failure("missing_node", Some(node.id().get()), None))
}

pub(super) fn style_for(
    styles: &ComputedStyleSnapshot,
    node: HostNodeHandle,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == node.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(node.id().get()), None))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: &Scene,
    is_outer: bool,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_width = if is_outer {
        match scene.width {
            RootWidth::MaxContent => ComputedCssDimension::MaxContent,
            RootWidth::Fixed120 => ComputedCssDimension::LengthPx(120.0),
        }
    } else {
        ComputedCssDimension::Auto
    };
    let expected_columns = if is_outer {
        match scene.columns {
            OuterColumns::MinContentPair => vec![
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::MinContent,
            ],
            OuterColumns::AutoMinFlexPair => vec![auto_min_flex_track(), auto_min_flex_track()],
        }
    } else {
        vec![
            ComputedCssGridTrackSize::MinContent,
            ComputedCssGridTrackSize::MaxContent,
        ]
    };
    let Some(grid) = style.grid_layout.as_ref() else {
        return Err(unsupported(style.node_id, scene.id, "Grid style missing"));
    };
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || style.layout_dimensions.width != expected_width
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(20.0)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.column_gap, 4.0)
        || !spacing_is(style.layout_spacing.row_gap, 0.0)
        || style.layout_border != Default::default()
        || grid.columns != expected_columns
        || grid.rows != [ComputedCssGridTrackSize::Auto]
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
            "nested Grid alignment",
        ));
    }
    Ok(())
}

pub(super) fn auto_min_flex_track() -> ComputedCssGridTrackSize {
    ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::Auto,
        max: ComputedCssGridTrackBreadth::Fraction(1.0),
    }
}

pub(super) fn validate_text_style(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    for (name, expected) in [
        ("display", "block"),
        ("box-sizing", "border-box"),
        ("font-family", "Ahem"),
        ("font-size", "10px"),
        ("font-weight", "400"),
        ("font-style", "normal"),
        ("line-height", "10px"),
        ("white-space-collapse", "collapse"),
        ("text-wrap-mode", "wrap"),
        ("word-break", "normal"),
        ("direction", "ltr"),
        ("line-break", "auto"),
        ("text-align", "start"),
        ("overflow-wrap", "normal"),
        ("text-transform", "none"),
        ("writing-mode", "horizontal-tb"),
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
    ] {
        if style.properties.get(name).map(String::as_str) != Some(expected) {
            return Err(unsupported(style.node_id, scene_id, "Ahem text style"));
        }
    }
    if style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
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
        return Err(unsupported(style.node_id, scene_id, "text item box"));
    }
    Ok(())
}

pub(super) fn validate_image_style(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    if style.properties.get("display").map(String::as_str) != Some("block")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || style.layout_aspect_ratio.is_some()
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default()
    {
        return Err(unsupported(style.node_id, scene_id, "replaced image style"));
    }
    Ok(())
}

fn validate_placement(
    style: &ComputedElementStyle,
    scene_id: &str,
    expected_column_start: ComputedCssGridPlacement,
    expected_column_end: ComputedCssGridPlacement,
) -> Result<(), RuntimeLayoutFailure> {
    let valid = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.row_start == ComputedCssGridPlacement::Auto
            && placement.row_end == ComputedCssGridPlacement::Auto
            && placement.column_start == expected_column_start
            && placement.column_end == expected_column_end
    });
    if valid {
        Ok(())
    } else {
        Err(unsupported(style.node_id, scene_id, "Grid item placement"))
    }
}

pub(super) fn ahem_paragraph_style() -> ParagraphStyle {
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
        ligatures: LigatureMode::Normal,
        letter_spacing: 0.0,
    }
}

pub(super) fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| spacing_is(value, 0.0))
}

pub(super) fn spacing_is(value: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if (actual - expected).abs() <= f32::EPSILON)
}

pub(super) fn failure(
    code: &'static str,
    node: HostNodeHandle,
    scene_id: &str,
    error: impl std::fmt::Display,
) -> RuntimeLayoutFailure {
    layout_failure(
        code,
        Some(node.id().get()),
        Some(format!("{scene_id}: {error}")),
    )
}

pub(super) fn unsupported(
    node: spinon_core::NodeId,
    scene_id: &str,
    detail: &str,
) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_nested_grid_replaced_item",
        Some(node.get()),
        Some(format!("{scene_id}: {detail}")),
    )
}

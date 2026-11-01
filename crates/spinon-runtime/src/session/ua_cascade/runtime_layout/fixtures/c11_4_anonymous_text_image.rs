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
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackSize,
    ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};
use super::c11_4_image::{
    IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG, element_identity, image_src,
};
use super::c11_4_text::AHEM;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const SCENE_IDS: [&str; 5] = [
    "anonymous-text-image-text",
    "image-before-comment-group",
    "comment-group-before-image",
    "whitespace-around-image",
    "hidden-child-around-image",
];

#[derive(Clone, Copy)]
enum ExpectedChild {
    Text(&'static str),
    Image(&'static str),
    Hidden {
        id: &'static str,
        text: &'static str,
    },
}

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
    if !SCENE_IDS.contains(&scene_id) {
        return if scene_id.starts_with("anonymous-text-image-")
            || scene_id.starts_with("image-before-comment-")
            || scene_id.starts_with("comment-group-before-")
            || scene_id.starts_with("whitespace-around-")
            || scene_id.starts_with("hidden-child-around-")
        {
            Err(unsupported(
                root.id(),
                scene_id,
                "unknown anonymous text/image scene",
            ))
        } else {
            Ok(None)
        };
    }
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || namespace != HTML_NAMESPACE
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return Err(unsupported(root.id(), scene_id, "profile or root identity"));
    }

    let children = snapshot
        .children(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?
        .collect::<Vec<_>>();
    let expected = expected_children(scene_id);
    if children.len() != expected.len() {
        return Err(unsupported(root.id(), scene_id, "direct child count"));
    }
    validate_grid_style(style_for(styles, root)?, scene_id)?;

    let mut image_handles = Vec::new();
    for (&handle, expected_child) in children.iter().zip(expected) {
        if snapshot.parent(handle) != Some(HostParent::Node(root)) {
            return Err(unsupported(handle.id(), scene_id, "direct child ownership"));
        }
        match expected_child {
            ExpectedChild::Text(value) => {
                let actual = snapshot.node(handle).and_then(|node| match node.kind() {
                    HostNodeKind::Text(text) => Some(text.to_string_lossy()),
                    HostNodeKind::Element(_) => None,
                });
                if actual.as_deref() != Some(value) {
                    return Err(unsupported(handle.id(), scene_id, "direct text content"));
                }
            }
            ExpectedChild::Image(expected_id) => {
                let (item_namespace, item_tag, item_id) = element_identity(snapshot, handle)?;
                if item_namespace != HTML_NAMESPACE
                    || item_tag != "img"
                    || item_id != *expected_id
                    || image_src(snapshot, handle)?.as_deref() != Some(IMAGE_URL)
                    || snapshot
                        .children(handle)
                        .is_none_or(|mut descendants| descendants.next().is_some())
                {
                    return Err(unsupported(
                        handle.id(),
                        scene_id,
                        "fixed direct image leaf",
                    ));
                }
                validate_image_style(style_for(styles, handle)?, scene_id)?;
                image_handles.push(handle);
            }
            ExpectedChild::Hidden { id, text } => {
                let (_, hidden_tag, hidden_id) = element_identity(snapshot, handle)?;
                let hidden_style = style_for(styles, handle)?;
                let descendants = snapshot
                    .children(handle)
                    .ok_or_else(|| unsupported(handle.id(), scene_id, "hidden child list"))?
                    .collect::<Vec<_>>();
                if hidden_tag != "span"
                    || hidden_id != *id
                    || hidden_style.properties.get("display").map(String::as_str) != Some("none")
                    || descendants.len() != 1
                    || !snapshot.node(descendants[0]).is_some_and(|node| {
                        matches!(node.kind(), HostNodeKind::Text(value) if value.to_string_lossy() == *text)
                    })
                {
                    return Err(unsupported(handle.id(), scene_id, "hidden child identity/style"));
                }
            }
        }
    }
    if image_handles.len() != 1 {
        return Err(unsupported(
            root.id(),
            scene_id,
            "exactly one image is required",
        ));
    }

    let image = image_handles[0];
    let completion = ImageRequest::from_document(snapshot, image)
        .map_err(|error| failure("c114_anonymous_text_image_request", image, scene_id, error))?
        .decode(PNG)
        .map_err(|error| failure("c114_anonymous_text_image_decode", image, scene_id, error))?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| unsupported(root.id(), scene_id, "intrinsic revision exhausted"))?;
    let resources = ImageResourceSnapshot::new(snapshot, revision, [completion])
        .map_err(|error| failure("c114_anonymous_text_image_snapshot", root, scene_id, error))?;
    let decoded = resources
        .images()
        .get(&image.id())
        .ok_or_else(|| unsupported(image.id(), scene_id, "decoded image metadata missing"))?;
    let dimensions = decoded.dimensions();
    if dimensions.width() != NATURAL_WIDTH || dimensions.height() != NATURAL_HEIGHT {
        return Err(unsupported(
            image.id(),
            scene_id,
            "PNG natural dimensions changed",
        ));
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| failure("c114_anonymous_text_image_intrinsic", root, scene_id, error))?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", AHEM)
        .map_err(|error| failure("c114_anonymous_text_image_font", root, scene_id, error))?;
    Ok(Some(super::C11_4MixedInput {
        paragraph_styles: BTreeMap::from([(root.id(), anonymous_paragraph_style())]),
        engine,
        intrinsic_sizes,
        use_extended_fixture_projection: false,
    }))
}

fn expected_children(scene_id: &str) -> &'static [ExpectedChild] {
    use ExpectedChild::{Hidden, Image, Text};
    match scene_id {
        "anonymous-text-image-text" => &[
            Text("XX"),
            Image("c114-anonymous-text-image-middle-image"),
            Text("XX"),
        ],
        "image-before-comment-group" => &[
            Image("c114-image-before-comment-group-image"),
            Text("XX"),
            Text("XX"),
        ],
        "comment-group-before-image" => &[
            Text("XX"),
            Text("XX"),
            Image("c114-comment-group-before-image-image"),
        ],
        "whitespace-around-image" => &[
            Text(" \t\n"),
            Image("c114-whitespace-around-image-image"),
            Text("\n\tXX"),
        ],
        "hidden-child-around-image" => &[
            Text("XX"),
            Hidden {
                id: "c114-hidden-child-around-image-hidden",
                text: "ignored",
            },
            Image("c114-hidden-child-around-image-image"),
            Text("XX"),
        ],
        _ => &[],
    }
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| unsupported(style.node_id, scene_id, "Grid style missing"))?;
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.properties.get("align-items").map(String::as_str) != Some("start")
        || style.properties.get("align-content").map(String::as_str) != Some("start")
        || style.properties.get("justify-content").map(String::as_str) != Some("start")
        || style.properties.get("font-family").map(String::as_str) != Some("Ahem")
        || style.properties.get("font-size").map(String::as_str) != Some("10px")
        || style.properties.get("line-height").map(String::as_str) != Some("10px")
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(220.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(60.0)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_edges(style)
        || !zero_spacing(style.layout_spacing.row_gap)
        || !zero_spacing(style.layout_spacing.column_gap)
        || grid.columns != vec![ComputedCssGridTrackSize::MinContent; 4]
        || grid.rows != vec![ComputedCssGridTrackSize::LengthPx(60.0)]
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_line_names.len() != 5
        || grid.rows_line_names.len() != 2
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported(
            style.node_id,
            scene_id,
            "fixed 220x60 Grid style changed",
        ));
    }
    for (name, expected) in [
        ("white-space-collapse", "collapse"),
        ("text-wrap-mode", "wrap"),
        ("word-break", "normal"),
        ("direction", "ltr"),
        ("line-break", "auto"),
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
    ] {
        if style.properties.get(name).map(String::as_str) != Some(expected) {
            return Err(unsupported(
                style.node_id,
                scene_id,
                "inherited Ahem text style changed",
            ));
        }
    }
    Ok(())
}

fn validate_image_style(
    style: &ComputedElementStyle,
    scene_id: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let placement = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.column_start == ComputedCssGridPlacement::Span(2)
            && placement.column_end == ComputedCssGridPlacement::Auto
            && placement.row_start == ComputedCssGridPlacement::Auto
            && placement.row_end == ComputedCssGridPlacement::Auto
    });
    if !matches!(
        style.properties.get("display").map(String::as_str),
        Some("inline" | "block")
    ) || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || !placement
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(48.0)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_aspect_ratio.is_some()
        || !zero_edges(style)
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
            scene_id,
            "default inline image span style changed",
        ));
    }
    Ok(())
}

fn style_for(
    styles: &ComputedStyleSnapshot,
    handle: HostNodeHandle,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == handle.id())
        .ok_or_else(|| layout_failure("missing_computed_style", Some(handle.id().get()), None))
}

pub(super) fn anonymous_paragraph_style() -> ParagraphStyle {
    ParagraphStyle {
        overflow_wrap: Default::default(),
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
        locale: None,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: LigatureMode::Normal,
        letter_spacing: 0.0,
        word_spacing: 0.0,
        text_indent: 0.0,
        text_indent_flags: spinon_layout::TextIndentFlags::default(),
        text_transform: Default::default(),
    }
}

fn zero_edges(style: &ComputedElementStyle) -> bool {
    [
        style.layout_spacing.margin.top,
        style.layout_spacing.margin.right,
        style.layout_spacing.margin.bottom,
        style.layout_spacing.margin.left,
        style.layout_spacing.padding.top,
        style.layout_spacing.padding.right,
        style.layout_spacing.padding.bottom,
        style.layout_spacing.padding.left,
    ]
    .into_iter()
    .all(zero_spacing)
        && style.layout_border == Default::default()
}

fn zero_spacing(value: ComputedCssSpacingValue) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if actual.abs() <= f32::EPSILON)
}

fn unsupported(node_id: spinon_core::NodeId, scene_id: &str, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_anonymous_text_image_fixture",
        Some(node_id.get()),
        Some(format!("{scene_id}: {detail}")),
    )
}

fn failure(
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

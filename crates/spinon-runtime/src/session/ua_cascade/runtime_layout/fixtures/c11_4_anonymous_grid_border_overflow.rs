use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{
    ComputedBackgroundPaint, ComputedBorderPaint, ComputedCssDimension, ComputedCssGridAutoColumns,
    ComputedCssGridAutoFlow, ComputedCssGridAutoRows, ComputedCssGridPlacement,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssSpacingValue, ComputedElementStyle,
    ComputedLayoutBorder, ComputedStyleProfile, ComputedStyleSnapshot, OpaqueCssSrgb,
};

use super::super::{RuntimeLayoutFailure, layout_failure};
use super::c11_4_image::{
    IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG, element_identity, image_src,
};
use super::c11_4_text::AHEM;

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const SCENE_IDS: [&str; 4] = [
    "anonymous-runs-two-images-forward",
    "two-images-anonymous-runs-reversed",
    "two-anonymous-runs-three-images-forward",
    "three-images-two-anonymous-runs-reversed",
];

#[derive(Clone, Copy)]
struct ImageSpec {
    id: &'static str,
    height: f32,
    column: i32,
    column_span: i32,
    row: i32,
}

#[derive(Clone, Copy)]
enum ExpectedChild {
    Text,
    Image(ImageSpec),
}

const IMAGE_A_FORWARD: ExpectedChild = ExpectedChild::Image(ImageSpec {
    id: "c114-overflow-image-a-forward",
    height: 16.0,
    column: 1,
    column_span: 2,
    row: 1,
});
const IMAGE_B_FORWARD: ExpectedChild = ExpectedChild::Image(ImageSpec {
    id: "c114-overflow-image-b-forward",
    height: 32.0,
    column: 2,
    column_span: 2,
    row: 2,
});
const IMAGE_A_REVERSED: ExpectedChild = ExpectedChild::Image(ImageSpec {
    id: "c114-overflow-image-a-reversed",
    height: 16.0,
    column: 1,
    column_span: 2,
    row: 1,
});
const IMAGE_B_REVERSED: ExpectedChild = ExpectedChild::Image(ImageSpec {
    id: "c114-overflow-image-b-reversed",
    height: 32.0,
    column: 2,
    column_span: 2,
    row: 2,
});
const IMAGE_A_FORWARD_THREE: ExpectedChild = ExpectedChild::Image(ImageSpec {
    id: "c114-overflow-three-image-a-forward",
    height: 16.0,
    column: 1,
    column_span: 2,
    row: 1,
});
const IMAGE_B_FORWARD_THREE: ExpectedChild = ExpectedChild::Image(ImageSpec {
    id: "c114-overflow-three-image-b-forward",
    height: 32.0,
    column: 2,
    column_span: 2,
    row: 2,
});
const IMAGE_C_FORWARD_THREE: ExpectedChild = ExpectedChild::Image(ImageSpec {
    id: "c114-overflow-three-image-c-forward",
    height: 48.0,
    column: 1,
    column_span: 3,
    row: 3,
});
const IMAGE_A_REVERSED_THREE: ExpectedChild = ExpectedChild::Image(ImageSpec {
    id: "c114-overflow-three-image-a-reversed",
    height: 16.0,
    column: 1,
    column_span: 2,
    row: 1,
});
const IMAGE_B_REVERSED_THREE: ExpectedChild = ExpectedChild::Image(ImageSpec {
    id: "c114-overflow-three-image-b-reversed",
    height: 32.0,
    column: 2,
    column_span: 2,
    row: 2,
});
const IMAGE_C_REVERSED_THREE: ExpectedChild = ExpectedChild::Image(ImageSpec {
    id: "c114-overflow-three-image-c-reversed",
    height: 48.0,
    column: 1,
    column_span: 3,
    row: 3,
});
const FORWARD: [ExpectedChild; 4] = [
    ExpectedChild::Text,
    IMAGE_A_FORWARD,
    ExpectedChild::Text,
    IMAGE_B_FORWARD,
];
const REVERSED: [ExpectedChild; 4] = [
    IMAGE_B_REVERSED,
    ExpectedChild::Text,
    IMAGE_A_REVERSED,
    ExpectedChild::Text,
];
const FORWARD_THREE: [ExpectedChild; 5] = [
    ExpectedChild::Text,
    IMAGE_A_FORWARD_THREE,
    ExpectedChild::Text,
    IMAGE_B_FORWARD_THREE,
    IMAGE_C_FORWARD_THREE,
];
const REVERSED_THREE: [ExpectedChild; 5] = [
    IMAGE_C_REVERSED_THREE,
    ExpectedChild::Text,
    IMAGE_B_REVERSED_THREE,
    ExpectedChild::Text,
    IMAGE_A_REVERSED_THREE,
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
    if !SCENE_IDS.contains(&scene_id) {
        return if scene_id.starts_with("anonymous-runs-two-images-")
            || scene_id.starts_with("two-images-anonymous-runs-")
            || scene_id.starts_with("two-anonymous-runs-three-images-")
            || scene_id.starts_with("three-images-two-anonymous-runs-")
        {
            Err(unsupported(
                root.id(),
                scene_id,
                "unknown bordered Grid scene",
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

    let mut text_anchors = Vec::new();
    let mut image_handles = Vec::new();
    for (&handle, expected_child) in children.iter().zip(expected) {
        if snapshot.parent(handle) != Some(HostParent::Node(root)) {
            return Err(unsupported(handle.id(), scene_id, "direct child ownership"));
        }
        match expected_child {
            ExpectedChild::Text => {
                if !snapshot.node(handle).is_some_and(|node| {
                    matches!(node.kind(), HostNodeKind::Text(value) if value.to_string_lossy() == "XX")
                }) {
                    return Err(unsupported(handle.id(), scene_id, "direct XX text run"));
                }
                text_anchors.push(handle.id());
            }
            ExpectedChild::Image(spec) => {
                let (item_namespace, item_tag, item_id) = element_identity(snapshot, handle)?;
                if item_namespace != HTML_NAMESPACE
                    || item_tag != "img"
                    || item_id != spec.id
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
                validate_image_style(style_for(styles, handle)?, scene_id, *spec)?;
                image_handles.push(handle);
            }
        }
    }
    if text_anchors.len() != 2 || !matches!(image_handles.len(), 2 | 3) {
        return Err(unsupported(
            root.id(),
            scene_id,
            "two anonymous XX runs and two or three images are required",
        ));
    }
    let image_ids = image_handles
        .iter()
        .map(|image| image.id())
        .collect::<BTreeSet<_>>();
    if image_ids.len() != image_handles.len() {
        return Err(unsupported(root.id(), scene_id, "duplicate image node"));
    }

    let completions = image_handles
        .iter()
        .map(|&image| {
            ImageRequest::from_document(snapshot, image)
                .map_err(|error| {
                    failure("c114_bordered_grid_image_request", image, scene_id, error)
                })?
                .decode(PNG)
                .map_err(|error| failure("c114_bordered_grid_image_decode", image, scene_id, error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| unsupported(root.id(), scene_id, "intrinsic revision exhausted"))?;
    let resources = ImageResourceSnapshot::new(snapshot, revision, completions)
        .map_err(|error| failure("c114_bordered_grid_image_snapshot", root, scene_id, error))?;
    if resources.images().len() != image_handles.len() {
        return Err(unsupported(
            root.id(),
            scene_id,
            "decoded image set is incomplete",
        ));
    }
    for image in &image_handles {
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
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| failure("c114_bordered_grid_intrinsic", root, scene_id, error))?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", AHEM)
        .map_err(|error| failure("c114_bordered_grid_font", root, scene_id, error))?;
    Ok(Some(super::C11_4MixedInput {
        paragraph_styles: BTreeMap::from([(root.id(), anonymous_paragraph_style())]),
        engine,
        intrinsic_sizes,
        use_extended_fixture_projection: false,
    }))
}

fn expected_children(scene_id: &str) -> &'static [ExpectedChild] {
    match scene_id {
        "anonymous-runs-two-images-forward" => &FORWARD,
        "two-images-anonymous-runs-reversed" => &REVERSED,
        "two-anonymous-runs-three-images-forward" => &FORWARD_THREE,
        "three-images-two-anonymous-runs-reversed" => &REVERSED_THREE,
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
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(40.0)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(184.0)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || !zero_spacing_edges(style)
        || style.layout_border
            != (ComputedLayoutBorder {
                top: 2.0,
                right: 2.0,
                bottom: 2.0,
                left: 2.0,
            })
        || style.layout_border_paint != Some(border_paint())
        || style.background_paint != Some(background_paint())
        || !zero_spacing(style.layout_spacing.row_gap)
        || !zero_spacing(style.layout_spacing.column_gap)
        || grid.columns != vec![ComputedCssGridTrackSize::MinContent; 4]
        || grid.rows != vec![ComputedCssGridTrackSize::LengthPx(60.0); 3]
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.columns_line_names.len() != 5
        || grid.rows_line_names.len() != 4
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported(
            style.node_id,
            scene_id,
            "fixed bordered 40x184 Grid style changed",
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
    expected: ImageSpec,
) -> Result<(), RuntimeLayoutFailure> {
    let placement = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.column_start == ComputedCssGridPlacement::Line(expected.column)
            && placement.column_end == ComputedCssGridPlacement::Span(expected.column_span)
            && placement.row_start == ComputedCssGridPlacement::Line(expected.row)
            && placement.row_end == ComputedCssGridPlacement::Auto
    });
    if !matches!(
        style.properties.get("display").map(String::as_str),
        Some("inline" | "block")
    ) || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || !placement
        || style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(expected.height)
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_aspect_ratio.is_some()
        || !zero_box_edges(style)
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
            "fixed spanning image style changed",
        ));
    }
    Ok(())
}

fn zero_spacing_edges(style: &ComputedElementStyle) -> bool {
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
}

fn zero_box_edges(style: &ComputedElementStyle) -> bool {
    zero_spacing_edges(style) && style.layout_border == Default::default()
}

fn zero_spacing(value: ComputedCssSpacingValue) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if actual.abs() <= f32::EPSILON)
}

fn border_paint() -> ComputedBorderPaint {
    let color = ComputedBackgroundPaint::Opaque(OpaqueCssSrgb {
        red: 240,
        green: 198,
        blue: 116,
    });
    ComputedBorderPaint {
        top: color,
        right: color,
        bottom: color,
        left: color,
    }
}

fn background_paint() -> ComputedBackgroundPaint {
    ComputedBackgroundPaint::Opaque(OpaqueCssSrgb {
        red: 16,
        green: 24,
        blue: 32,
    })
}

fn anonymous_paragraph_style() -> ParagraphStyle {
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

fn unsupported(node_id: spinon_core::NodeId, scene_id: &str, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_anonymous_grid_border_overflow_fixture",
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

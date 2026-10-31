use std::collections::BTreeMap;

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::{
    FontStyleMode, LigatureMode, LineBreakMode, ParagraphStyle, TabSize, TextLayoutEngine,
    TextParagraphDirection, WhiteSpaceMode, WordBreakMode,
};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridItemPlacement, ComputedCssGridLayout,
    ComputedCssGridPlacement, ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedElementStyle,
    ComputedStyleProfile, ComputedStyleSnapshot,
};

use super::super::{RuntimeLayoutFailure, layout_failure};

#[path = "c11_4_anonymous_text/font_size.rs"]
mod font_size;
#[path = "c11_4_anonymous_text/line_height.rs"]
mod line_height;
use font_size::{FONT_SIZE_SCENE_IDS, validate_computed_font_size};
use line_height::{
    ComputedLineHeight, LINE_HEIGHT_SCENE_IDS, line_height_text, validate_computed_line_height,
};

const SCENE_IDS: [&str; 55] = [
    "anonymous-min-content-wrap",
    "anonymous-normal-min-content",
    "anonymous-nowrap-min-content",
    "anonymous-normal-newline-max-content",
    "anonymous-pre-line-max-content",
    "anonymous-comment-continuation",
    "anonymous-runs-around-block",
    "anonymous-hidden-child-continuation",
    "whitespace-does-not-create-item",
    "anonymous-normal-max-content-spaces",
    "anonymous-pre-wrap-max-content-spaces",
    "anonymous-pre-wrap-comment-split-max-content",
    "anonymous-pre-wrap-min-content-spaces",
    "anonymous-pre-wrap-soft-wrap",
    "anonymous-pre-wrap-forced-break",
    "anonymous-pre-wrap-whitespace-before-block",
    "anonymous-break-spaces-max-content",
    "anonymous-break-spaces-comment-split-max-content",
    "anonymous-break-spaces-min-content",
    "anonymous-break-spaces-soft-wrap-25",
    "anonymous-break-spaces-forced-break-max-content",
    "anonymous-break-spaces-forced-break-min-content",
    "anonymous-break-spaces-trailing-soft-wrap-25",
    "anonymous-break-spaces-trailing-max-content",
    "anonymous-break-spaces-whitespace-only",
    "anonymous-break-spaces-whitespace-before-block",
    "anonymous-pre-max-content-spaces",
    "anonymous-pre-min-content-spaces",
    "anonymous-pre-forced-break",
    "anonymous-normal-fixed-width-control",
    "anonymous-pre-fixed-width-nowrap",
    "anonymous-pre-tab-max-content",
    "anonymous-pre-tab-leading",
    "anonymous-pre-tab-exact-stop",
    "anonymous-pre-tab-consecutive",
    "anonymous-pre-tab-forced-break",
    "anonymous-pre-tab-trailing",
    "anonymous-pre-tab-min-content",
    "anonymous-pre-tab-fixed-width-nowrap",
    "anonymous-pre-tab-size-2-two-columns",
    "anonymous-pre-tab-size-2-leading",
    "anonymous-pre-tab-size-2-exact-stop",
    "anonymous-pre-tab-size-2-consecutive",
    "anonymous-pre-tab-size-2-reset",
    "anonymous-pre-tab-size-2-crlf-reset",
    "anonymous-pre-tab-size-2-trailing",
    "anonymous-pre-tab-size-2-min-content",
    "anonymous-pre-tab-size-2-fixed-overflow",
    "anonymous-pre-tab-size-1-leading",
    "anonymous-pre-tab-size-1-exact-stop",
    "anonymous-pre-tab-size-4-leading",
    "anonymous-pre-tab-size-4-exact-stop",
    "anonymous-pre-tab-size-0-negative",
    "anonymous-pre-tab-size-fractional-negative",
    "anonymous-pre-tab-size-length-negative",
];

const FULL_TAB_VALUE_IDS: [&str; 14] = [
    "number-zero",
    "number-quarter",
    "number-three-tenths",
    "number-half",
    "number-one-and-half",
    "number-two-and-half",
    "number-three-and-quarter",
    "length-zero",
    "length-three-tenths",
    "length-two",
    "length-five",
    "length-twelve",
    "length-em",
    "length-calc",
];

const FULL_TAB_CASE_IDS: [&str; 10] = [
    "leading",
    "after-one",
    "after-two",
    "exact-stop",
    "consecutive",
    "trailing",
    "lf-reset",
    "crlf-reset",
    "min-content",
    "fixed-overflow",
];

const FULL_TAB_COMPUTED_VALUES: [&str; 14] = [
    "0", "0.25", "0.3", "0.5", "1.5", "2.5", "3.25", "0px", "0.3px", "2px", "5px", "12px", "15px",
    "5px",
];

// expected-values.json의 10개 text case별 독립 tab-stop 산술 결과입니다.
#[cfg(test)]
const FULL_TAB_EXPECTED_WIDTHS: [[f32; 10]; 14] = [
    [10.0, 20.0, 30.0, 50.0, 20.0, 20.0, 40.0, 20.0, 40.0, 40.0],
    [15.0, 25.0, 35.0, 55.0, 27.5, 25.0, 45.0, 25.0, 45.0, 45.0],
    [16.0, 25.0, 34.0, 55.0, 28.0, 24.0, 44.0, 25.0, 44.0, 44.0],
    [15.0, 25.0, 35.0, 55.0, 30.0, 25.0, 45.0, 25.0, 45.0, 45.0],
    [25.0, 25.0, 40.0, 55.0, 40.0, 30.0, 50.0, 25.0, 50.0, 50.0],
    [35.0, 35.0, 35.0, 60.0, 60.0, 25.0, 45.0, 35.0, 45.0, 45.0],
    [42.5, 42.5, 42.5, 75.0, 75.0, 32.5, 52.5, 42.5, 52.5, 52.5],
    [10.0, 20.0, 30.0, 50.0, 20.0, 20.0, 40.0, 20.0, 40.0, 40.0],
    [10.6, 20.5, 30.4, 50.5, 20.8, 20.4, 40.4, 20.5, 40.4, 40.4],
    [14.0, 24.0, 34.0, 54.0, 26.0, 24.0, 44.0, 24.0, 44.0, 44.0],
    [15.0, 25.0, 35.0, 55.0, 30.0, 25.0, 45.0, 25.0, 45.0, 45.0],
    [22.0, 34.0, 46.0, 58.0, 46.0, 36.0, 56.0, 34.0, 56.0, 56.0],
    [25.0, 25.0, 40.0, 55.0, 40.0, 30.0, 50.0, 25.0, 50.0, 50.0],
    [15.0, 25.0, 35.0, 55.0, 30.0, 25.0, 45.0, 25.0, 45.0, 45.0],
];

#[cfg(test)]
const FULL_TAB_LF_SECOND_LINE_WIDTHS: [f32; 14] = [
    20.0, 25.0, 25.0, 25.0, 25.0, 35.0, 42.5, 20.0, 20.5, 24.0, 25.0, 34.0, 25.0, 25.0,
];

#[cfg(test)]
const FULL_TAB_CRLF_FIRST_LINE_WIDTHS: [f32; 14] = [
    10.0, 15.0, 15.0, 15.0, 15.0, 25.0, 32.5, 10.0, 10.5, 14.0, 15.0, 24.0, 15.0, 15.0,
];

pub(in crate::session::ua_cascade::runtime_layout) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<super::C11_4TextInput>, RuntimeLayoutFailure> {
    let (tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene_id) = root_id
        .strip_prefix("c114-")
        .and_then(|value| value.strip_suffix("-grid"))
    else {
        return Ok(None);
    };
    if !SCENE_IDS.contains(&scene_id)
        && !LINE_HEIGHT_SCENE_IDS.contains(&scene_id)
        && !FONT_SIZE_SCENE_IDS.contains(&scene_id)
        && full_tab_scene(scene_id).is_none()
    {
        return if scene_id.starts_with("anonymous-")
            || scene_id == "whitespace-does-not-create-item"
        {
            Err(unsupported(root.id(), "unknown anonymous text scene"))
        } else {
            Ok(None)
        };
    }
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1 || tag != "div" {
        return Err(unsupported(root.id(), "runtime profile or root tag"));
    }

    let grid_style = style_for(styles, root.id())?;
    let font_size = validate_computed_font_size(grid_style, scene_id)?;
    let (tab_size, line_height) = validate_grid_style(grid_style, scene_id, font_size)?;
    validate_direct_children(snapshot, root, scene_id, styles)?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", super::c11_4_text::AHEM)
        .map_err(|error| {
            layout_failure(
                "c114_anonymous_text_font_registration",
                Some(root.id().get()),
                Some(error.to_string()),
            )
        })?;
    Ok(Some(super::C11_4TextInput {
        paragraph_styles: BTreeMap::from([(
            root.id(),
            anonymous_paragraph_style(scene_id, tab_size, line_height, font_size),
        )]),
        engine,
        two_axis_nested_grid: false,
        content_box_ratio_node: None,
        aspect_ratio_minmax_node: None,
        definite_axis_ratio_node: None,
    }))
}

fn validate_direct_children(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    scene_id: &str,
    styles: &ComputedStyleSnapshot,
) -> Result<(), RuntimeLayoutFailure> {
    let children = snapshot
        .children(root)
        .ok_or_else(|| layout_failure("missing_node", Some(root.id().get()), None))?
        .collect::<Vec<_>>();
    let expected: &[ExpectedChild<'_>] = match scene_id {
        "anonymous-min-content-wrap"
        | "anonymous-normal-min-content"
        | "anonymous-nowrap-min-content" => &[ExpectedChild::Text("XX XX")],
        "anonymous-normal-newline-max-content" | "anonymous-pre-line-max-content" => {
            &[ExpectedChild::Text("XX\nXX")]
        }
        "anonymous-normal-max-content-spaces"
        | "anonymous-pre-wrap-max-content-spaces"
        | "anonymous-pre-wrap-min-content-spaces" => &[ExpectedChild::Text("XX  XX")],
        "anonymous-pre-max-content-spaces" => &[ExpectedChild::Text("XX  XX")],
        scene_id if is_pre_tab_scene(scene_id) => &[ExpectedChild::Text(pre_tab_text(scene_id))],
        "anonymous-pre-min-content-spaces"
        | "anonymous-normal-fixed-width-control"
        | "anonymous-pre-fixed-width-nowrap" => &[ExpectedChild::Text("XX XX")],
        "anonymous-pre-forced-break" => &[ExpectedChild::Text("XX\nXX")],
        "anonymous-pre-wrap-comment-split-max-content" => {
            &[ExpectedChild::Text("XX "), ExpectedChild::Text(" XX")]
        }
        "anonymous-pre-wrap-soft-wrap" => &[ExpectedChild::Text("XX XX")],
        "anonymous-pre-wrap-forced-break" => &[ExpectedChild::Text("XX\nXX")],
        "anonymous-comment-continuation" => &[ExpectedChild::Text("XX"), ExpectedChild::Text("XX")],
        "anonymous-runs-around-block" => &[
            ExpectedChild::Text("XX"),
            ExpectedChild::Block("c114-anonymous-runs-around-block-item"),
            ExpectedChild::Text("XXXX"),
        ],
        "anonymous-hidden-child-continuation" => &[
            ExpectedChild::Text("XX"),
            ExpectedChild::Hidden {
                id: "c114-anonymous-hidden-child-continuation-hidden",
                text: "ignored",
            },
            ExpectedChild::Text("XX"),
        ],
        "whitespace-does-not-create-item" => &[
            ExpectedChild::Block("c114-whitespace-does-not-create-item-first"),
            ExpectedChild::Text(" \t\n"),
            ExpectedChild::Block("c114-whitespace-does-not-create-item-second"),
        ],
        "anonymous-pre-wrap-whitespace-before-block" => &[
            ExpectedChild::Text("  "),
            ExpectedChild::Block("c114-anonymous-pre-wrap-whitespace-before-block-item"),
        ],
        "anonymous-break-spaces-max-content"
        | "anonymous-break-spaces-min-content"
        | "anonymous-break-spaces-soft-wrap-25" => &[ExpectedChild::Text("XX  XX")],
        "anonymous-break-spaces-comment-split-max-content" => {
            &[ExpectedChild::Text("XX "), ExpectedChild::Text(" XX")]
        }
        "anonymous-break-spaces-forced-break-max-content"
        | "anonymous-break-spaces-forced-break-min-content" => &[ExpectedChild::Text("XX  \nXX")],
        "anonymous-break-spaces-trailing-soft-wrap-25"
        | "anonymous-break-spaces-trailing-max-content" => &[ExpectedChild::Text("XX  ")],
        "anonymous-break-spaces-whitespace-only" => &[ExpectedChild::Text("  ")],
        "anonymous-break-spaces-whitespace-before-block" => &[
            ExpectedChild::Text("  "),
            ExpectedChild::Block("c114-anonymous-break-spaces-whitespace-before-block-item"),
        ],
        scene_id if LINE_HEIGHT_SCENE_IDS.contains(&scene_id) => {
            let text = line_height_text(scene_id)
                .ok_or_else(|| unsupported(root.id(), "line-height scene text missing"))?;
            &[ExpectedChild::Text(text)]
        }
        scene_id if FONT_SIZE_SCENE_IDS.contains(&scene_id) => &[ExpectedChild::Text("XX XX")],
        _ => return Err(unsupported(root.id(), "unrecognized anonymous text scene")),
    };
    if children.len() != expected.len() {
        return Err(unsupported(root.id(), "direct child count"));
    }
    for (&handle, expected_child) in children.iter().zip(expected) {
        if snapshot.parent(handle) != Some(HostParent::Node(root)) {
            return Err(unsupported(handle.id(), "direct child ownership"));
        }
        match expected_child {
            ExpectedChild::Text(expected_text) => {
                let actual_text = snapshot
                    .node(handle)
                    .and_then(|node| match node.kind() {
                        HostNodeKind::Text(value) => Some(value.to_string_lossy()),
                        HostNodeKind::Element(_) => None,
                    })
                    .ok_or_else(|| unsupported(handle.id(), "text node expected"))?;
                if actual_text != *expected_text {
                    return Err(unsupported(handle.id(), "direct text content"));
                }
            }
            ExpectedChild::Block(expected_id) => {
                let (tag, actual_id) = element_identity(snapshot, handle)?;
                if tag != "div" || actual_id != *expected_id {
                    return Err(unsupported(handle.id(), "block item identity"));
                }
                let style = style_for(styles, handle.id())?;
                let (width, height) = if scene_id == "anonymous-runs-around-block" {
                    (30.0, 10.0)
                } else {
                    (10.0, 10.0)
                };
                if style.properties.get("display").map(String::as_str) != Some("block")
                    || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
                    || style.properties.get("position").map(String::as_str) != Some("static")
                    || style.properties.get("order").map(String::as_str) != Some("0")
                    || style.layout_position != spinon_style::ComputedCssPosition::Static
                    || style.layout_dimensions.width != ComputedCssDimension::LengthPx(width)
                    || style.layout_dimensions.height != ComputedCssDimension::LengthPx(height)
                    || style.layout_dimensions.min_width != ComputedCssDimension::LengthPx(0.0)
                    || style.layout_dimensions.min_height != ComputedCssDimension::LengthPx(0.0)
                    || style.layout_dimensions.max_width != ComputedCssMaxSize::None
                    || style.layout_dimensions.max_height != ComputedCssMaxSize::None
                    || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
                    || style.layout_aspect_ratio.is_some()
                    || !grid_item_placement_is_auto(style.grid_item_placement.as_ref())
                    || !zero_edges(style)
                {
                    return Err(unsupported(handle.id(), "block item geometry"));
                }
                if snapshot
                    .children(handle)
                    .is_some_and(|mut descendants| descendants.next().is_some())
                {
                    return Err(unsupported(handle.id(), "block item must be empty"));
                }
            }
            ExpectedChild::Hidden { id, text } => {
                let (tag, actual_id) = element_identity(snapshot, handle)?;
                if tag != "span" || actual_id != *id {
                    return Err(unsupported(handle.id(), "hidden separator identity"));
                }
                let style = style_for(styles, handle.id())?;
                if style.properties.get("display").map(String::as_str) != Some("none")
                    || style.layout_position != spinon_style::ComputedCssPosition::Static
                    || style.properties.get("order").map(String::as_str) != Some("0")
                {
                    return Err(unsupported(handle.id(), "hidden separator style"));
                }
                let descendants = snapshot
                    .children(handle)
                    .ok_or_else(|| unsupported(handle.id(), "hidden separator children"))?
                    .collect::<Vec<_>>();
                if descendants.len() != 1
                    || !snapshot.node(descendants[0]).is_some_and(|node| {
                        matches!(node.kind(), HostNodeKind::Text(value) if value.to_string_lossy() == *text)
                    })
                {
                    return Err(unsupported(handle.id(), "hidden separator text content"));
                }
            }
        }
    }
    Ok(())
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene_id: &str,
    font_size: f32,
) -> Result<(TabSize, ComputedLineHeight), RuntimeLayoutFailure> {
    let expected_wrap_mode = if is_pre_tab_scene(scene_id)
        || matches!(
            scene_id,
            "anonymous-nowrap-min-content"
                | "anonymous-pre-max-content-spaces"
                | "anonymous-pre-min-content-spaces"
                | "anonymous-pre-forced-break"
                | "anonymous-pre-fixed-width-nowrap"
        ) {
        "nowrap"
    } else {
        "wrap"
    };
    let expected_white_space_collapse = match scene_id {
        "anonymous-pre-line-max-content" => "preserve-breaks",
        "anonymous-pre-wrap-max-content-spaces"
        | "anonymous-pre-wrap-comment-split-max-content"
        | "anonymous-pre-wrap-min-content-spaces"
        | "anonymous-pre-wrap-soft-wrap"
        | "anonymous-pre-wrap-forced-break"
        | "anonymous-pre-wrap-whitespace-before-block"
        | "anonymous-pre-max-content-spaces"
        | "anonymous-pre-min-content-spaces"
        | "anonymous-pre-forced-break"
        | "anonymous-pre-fixed-width-nowrap" => "preserve",
        scene_id if is_pre_tab_scene(scene_id) => "preserve",
        "anonymous-break-spaces-max-content"
        | "anonymous-break-spaces-comment-split-max-content"
        | "anonymous-break-spaces-min-content"
        | "anonymous-break-spaces-soft-wrap-25"
        | "anonymous-break-spaces-forced-break-max-content"
        | "anonymous-break-spaces-forced-break-min-content"
        | "anonymous-break-spaces-trailing-soft-wrap-25"
        | "anonymous-break-spaces-trailing-max-content"
        | "anonymous-break-spaces-whitespace-only"
        | "anonymous-break-spaces-whitespace-before-block" => "break-spaces",
        scene_id
            if LINE_HEIGHT_SCENE_IDS.contains(&scene_id) && !scene_id.ends_with("soft-wrap") =>
        {
            "preserve-breaks"
        }
        _ => "collapse",
    };
    let (expected_width, expected_height) = if let Some((_, case_index)) = full_tab_scene(scene_id)
    {
        let width = if case_index == 9 { 25.0 } else { 120.0 };
        (width, ComputedCssDimension::Auto)
    } else {
        match scene_id {
            "anonymous-normal-min-content" | "anonymous-nowrap-min-content" => {
                (20.0, ComputedCssDimension::Auto)
            }
            "anonymous-normal-newline-max-content" | "anonymous-pre-line-max-content" => {
                (20.0, ComputedCssDimension::Auto)
            }
            "anonymous-normal-max-content-spaces"
            | "anonymous-pre-wrap-max-content-spaces"
            | "anonymous-pre-wrap-comment-split-max-content"
            | "anonymous-pre-wrap-min-content-spaces"
            | "anonymous-pre-wrap-forced-break" => (20.0, ComputedCssDimension::Auto),
            "anonymous-pre-max-content-spaces"
            | "anonymous-pre-min-content-spaces"
            | "anonymous-pre-forced-break" => (20.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-max-content" | "anonymous-pre-tab-forced-break" => {
                (100.0, ComputedCssDimension::Auto)
            }
            "anonymous-pre-tab-leading" => (90.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-exact-stop" | "anonymous-pre-tab-consecutive" => {
                (170.0, ComputedCssDimension::Auto)
            }
            "anonymous-pre-tab-trailing" => (80.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-min-content" => (100.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-size-2-two-columns" | "anonymous-pre-tab-size-2-reset" => {
                (60.0, ComputedCssDimension::Auto)
            }
            "anonymous-pre-tab-size-2-leading" => (30.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-size-2-exact-stop" | "anonymous-pre-tab-size-2-consecutive" => {
                (70.0, ComputedCssDimension::Auto)
            }
            "anonymous-pre-tab-size-2-crlf-reset" => (30.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-size-2-trailing" => (40.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-size-2-min-content" => (60.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-size-2-fixed-overflow" => (25.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-size-1-leading" => (20.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-size-1-exact-stop" => (30.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-size-4-leading" => (50.0, ComputedCssDimension::Auto),
            "anonymous-pre-tab-size-4-exact-stop" => (90.0, ComputedCssDimension::Auto),
            "anonymous-normal-fixed-width-control" | "anonymous-pre-fixed-width-nowrap" => {
                (25.0, ComputedCssDimension::Auto)
            }
            "anonymous-pre-tab-fixed-width-nowrap" => (25.0, ComputedCssDimension::Auto),
            "anonymous-break-spaces-max-content"
            | "anonymous-break-spaces-comment-split-max-content" => {
                (60.0, ComputedCssDimension::Auto)
            }
            "anonymous-break-spaces-min-content"
            | "anonymous-break-spaces-forced-break-min-content" => {
                (30.0, ComputedCssDimension::Auto)
            }
            "anonymous-break-spaces-soft-wrap-25"
            | "anonymous-break-spaces-trailing-soft-wrap-25" => (25.0, ComputedCssDimension::Auto),
            "anonymous-break-spaces-forced-break-max-content" => (40.0, ComputedCssDimension::Auto),
            "anonymous-break-spaces-trailing-max-content" => (40.0, ComputedCssDimension::Auto),
            "anonymous-break-spaces-whitespace-only" => (0.0, ComputedCssDimension::Auto),
            "anonymous-break-spaces-whitespace-before-block" => {
                (40.0, ComputedCssDimension::LengthPx(10.0))
            }
            "anonymous-pre-wrap-soft-wrap" => (25.0, ComputedCssDimension::Auto),
            "anonymous-pre-wrap-whitespace-before-block" => {
                (40.0, ComputedCssDimension::LengthPx(10.0))
            }
            "anonymous-min-content-wrap" => (120.0, ComputedCssDimension::LengthPx(20.0)),
            scene_id if FONT_SIZE_SCENE_IDS.contains(&scene_id) => {
                (20.0, ComputedCssDimension::Auto)
            }
            scene_id if LINE_HEIGHT_SCENE_IDS.contains(&scene_id) => {
                (20.0, ComputedCssDimension::Auto)
            }
            _ => (120.0, ComputedCssDimension::LengthPx(10.0)),
        }
    };
    let tab_size = validate_computed_tab_size(style.node_id, &style.properties, scene_id)?;
    let line_height = validate_computed_line_height(style.node_id, &style.properties, scene_id)?;
    validate_text_profile_properties(
        style.node_id,
        &style.properties,
        expected_white_space_collapse,
        expected_wrap_mode,
        font_size,
    )?;
    if style.properties.get("display").map(String::as_str) != Some("grid")
        || style.properties.get("box-sizing").map(String::as_str) != Some("border-box")
        || style.properties.get("align-items").map(String::as_str) != Some("start")
        || style.properties.get("align-content").map(String::as_str) != Some("start")
        || style.properties.get("justify-content").map(String::as_str) != Some("start")
        || style.layout_position != spinon_style::ComputedCssPosition::Static
        || style.properties.get("font-family").map(String::as_str) != Some("Ahem")
        || style.properties.get("font-size").map(String::as_str)
            != Some(font_size::font_size_css_value(scene_id))
        || style
            .properties
            .get("white-space-collapse")
            .map(String::as_str)
            != Some(expected_white_space_collapse)
        || style.properties.get("text-wrap-mode").map(String::as_str) != Some(expected_wrap_mode)
        || style.properties.get("order").map(String::as_str) != Some("0")
        || style.layout_dimensions.width != ComputedCssDimension::LengthPx(expected_width)
        || style.layout_dimensions.height != expected_height
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !zero_edges(style)
        || !zero_spacing(style.layout_spacing.row_gap)
        || !zero_spacing(style.layout_spacing.column_gap)
    {
        return Err(unsupported(
            style.node_id,
            "Grid style or inherited text profile",
        ));
    }
    let grid = style
        .grid_layout
        .as_ref()
        .ok_or_else(|| unsupported(style.node_id, "missing Grid style"))?;
    if !grid_tracks_match(grid, scene_id) {
        return Err(unsupported(
            style.node_id,
            "Grid track functions or auto-placement defaults",
        ));
    }
    Ok((tab_size, line_height))
}

fn validate_computed_tab_size(
    node_id: NodeId,
    properties: &BTreeMap<String, String>,
    scene_id: &str,
) -> Result<TabSize, RuntimeLayoutFailure> {
    let unsupported_tab_size = || {
        layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some("tab-size".to_owned()),
        )
    };
    let computed_tab_size = properties
        .get("tab-size")
        .and_then(|value| parse_computed_tab_size(value))
        .ok_or_else(unsupported_tab_size)?;
    let expected_tab_size =
        expected_tab_size_for_scene(scene_id).ok_or_else(unsupported_tab_size)?;
    if computed_tab_size != expected_tab_size {
        return Err(unsupported_tab_size());
    }
    Ok(computed_tab_size)
}

fn parse_computed_tab_size(value: &str) -> Option<TabSize> {
    let (value, is_length) = if let Some(value) = value.strip_suffix("px") {
        (value, true)
    } else {
        (value, false)
    };
    let value = value.trim().parse::<f32>().ok()?;
    let tab_size = if is_length {
        TabSize::LengthPx(value)
    } else {
        TabSize::Number(value)
    };
    (value.is_finite() && value >= 0.0).then_some(tab_size)
}

fn expected_tab_size_for_scene(scene_id: &str) -> Option<TabSize> {
    if let Some((value_index, _)) = full_tab_scene(scene_id) {
        return parse_computed_tab_size(FULL_TAB_COMPUTED_VALUES[value_index]);
    }
    Some(match scene_id {
        "anonymous-pre-tab-size-1-leading" | "anonymous-pre-tab-size-1-exact-stop" => {
            TabSize::Number(1.0)
        }
        "anonymous-pre-tab-size-2-two-columns"
        | "anonymous-pre-tab-size-2-leading"
        | "anonymous-pre-tab-size-2-exact-stop"
        | "anonymous-pre-tab-size-2-consecutive"
        | "anonymous-pre-tab-size-2-reset"
        | "anonymous-pre-tab-size-2-crlf-reset"
        | "anonymous-pre-tab-size-2-trailing"
        | "anonymous-pre-tab-size-2-min-content"
        | "anonymous-pre-tab-size-2-fixed-overflow" => TabSize::Number(2.0),
        "anonymous-pre-tab-size-4-leading" | "anonymous-pre-tab-size-4-exact-stop" => {
            TabSize::Number(4.0)
        }
        "anonymous-pre-tab-size-0-negative" => TabSize::Number(0.0),
        "anonymous-pre-tab-size-fractional-negative" => TabSize::Number(1.5),
        "anonymous-pre-tab-size-length-negative" => TabSize::LengthPx(20.0),
        _ => TabSize::Number(8.0),
    })
}

fn is_pre_tab_scene(scene_id: &str) -> bool {
    scene_id.starts_with("anonymous-pre-tab-") || full_tab_scene(scene_id).is_some()
}

fn pre_tab_text(scene_id: &str) -> &'static str {
    if let Some((_, case_index)) = full_tab_scene(scene_id) {
        return match case_index {
            0 => "\tX",
            1 => "X\tX",
            2 => "XX\tX",
            3 => "XXXX\tX",
            4 => "X\t\tX",
            5 => "XX\t",
            6 => "XX\tXX\nX\tX",
            7 => "X\t\r\nX\tX",
            8 | 9 => "XX\tXX",
            _ => "XX\tXX",
        };
    }
    match scene_id {
        "anonymous-pre-tab-leading"
        | "anonymous-pre-tab-size-2-leading"
        | "anonymous-pre-tab-size-1-leading"
        | "anonymous-pre-tab-size-4-leading" => "\tX",
        "anonymous-pre-tab-exact-stop" => "XXXXXXXX\tX",
        "anonymous-pre-tab-size-4-exact-stop" => "XXXX\tX",
        "anonymous-pre-tab-size-2-exact-stop" => "XXXX\tX",
        "anonymous-pre-tab-size-1-exact-stop" => "X\tX",
        "anonymous-pre-tab-consecutive" | "anonymous-pre-tab-size-2-consecutive" => "XX\t\tX",
        "anonymous-pre-tab-forced-break" => "XX\tXX\nX\tX",
        "anonymous-pre-tab-trailing" | "anonymous-pre-tab-size-2-trailing" => "XX\t",
        "anonymous-pre-tab-size-2-reset" => "XX\tXX\nX\tX",
        "anonymous-pre-tab-size-2-crlf-reset" => "X\t\r\nX\tX",
        _ => "XX\tXX",
    }
}

fn full_tab_scene(scene_id: &str) -> Option<(usize, usize)> {
    let scene_id = scene_id.strip_prefix("anonymous-pre-tab-value-")?;
    for (value_index, value_id) in FULL_TAB_VALUE_IDS.iter().enumerate() {
        let Some(case_id) = scene_id
            .strip_prefix(value_id)
            .and_then(|value| value.strip_prefix('-'))
        else {
            continue;
        };
        if let Some(case_index) = FULL_TAB_CASE_IDS.iter().position(|case| *case == case_id) {
            return Some((value_index, case_index));
        }
    }
    None
}

#[cfg(test)]
fn full_tab_scene_id(value_id: &str, case_id: &str) -> String {
    format!("anonymous-pre-tab-value-{value_id}-{case_id}")
}

#[cfg(test)]
fn full_tab_expected_width(scene_id: &str) -> Option<f32> {
    let (value_index, case_index) = full_tab_scene(scene_id)?;
    Some(FULL_TAB_EXPECTED_WIDTHS[value_index][case_index])
}

fn grid_item_placement_is_auto(placement: Option<&ComputedCssGridItemPlacement>) -> bool {
    placement.is_some_and(|placement| {
        placement.row_start == ComputedCssGridPlacement::Auto
            && placement.row_end == ComputedCssGridPlacement::Auto
            && placement.column_start == ComputedCssGridPlacement::Auto
            && placement.column_end == ComputedCssGridPlacement::Auto
    })
}

fn grid_tracks_match(grid: &ComputedCssGridLayout, scene_id: &str) -> bool {
    let expected_columns = if let Some((_, case_index)) = full_tab_scene(scene_id) {
        vec![match case_index {
            8 => ComputedCssGridTrackSize::MinContent,
            9 => ComputedCssGridTrackSize::LengthPx(25.0),
            _ => ComputedCssGridTrackSize::MaxContent,
        }]
    } else {
        match scene_id {
            scene_id if LINE_HEIGHT_SCENE_IDS.contains(&scene_id) => {
                vec![ComputedCssGridTrackSize::LengthPx(20.0)]
            }
            scene_id if FONT_SIZE_SCENE_IDS.contains(&scene_id) => {
                vec![match scene_id.ends_with("-min-content") {
                    true => ComputedCssGridTrackSize::MinContent,
                    false => ComputedCssGridTrackSize::MaxContent,
                }]
            }
            "anonymous-min-content-wrap" => vec![
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::Fraction(1.0),
            ],
            "anonymous-normal-min-content" | "anonymous-nowrap-min-content" => {
                vec![ComputedCssGridTrackSize::MinContent]
            }
            "anonymous-normal-newline-max-content" | "anonymous-pre-line-max-content" => {
                vec![ComputedCssGridTrackSize::MaxContent]
            }
            "anonymous-normal-max-content-spaces"
            | "anonymous-pre-wrap-max-content-spaces"
            | "anonymous-pre-wrap-comment-split-max-content"
            | "anonymous-pre-wrap-forced-break" => vec![ComputedCssGridTrackSize::MaxContent],
            "anonymous-pre-max-content-spaces" | "anonymous-pre-forced-break" => {
                vec![ComputedCssGridTrackSize::MaxContent]
            }
            "anonymous-pre-tab-max-content"
            | "anonymous-pre-tab-leading"
            | "anonymous-pre-tab-exact-stop"
            | "anonymous-pre-tab-consecutive"
            | "anonymous-pre-tab-forced-break"
            | "anonymous-pre-tab-trailing"
            | "anonymous-pre-tab-size-2-two-columns"
            | "anonymous-pre-tab-size-2-leading"
            | "anonymous-pre-tab-size-2-exact-stop"
            | "anonymous-pre-tab-size-2-consecutive"
            | "anonymous-pre-tab-size-2-reset"
            | "anonymous-pre-tab-size-2-crlf-reset"
            | "anonymous-pre-tab-size-2-trailing"
            | "anonymous-pre-tab-size-1-leading"
            | "anonymous-pre-tab-size-1-exact-stop"
            | "anonymous-pre-tab-size-4-leading"
            | "anonymous-pre-tab-size-4-exact-stop" => {
                vec![ComputedCssGridTrackSize::MaxContent]
            }
            "anonymous-break-spaces-max-content"
            | "anonymous-break-spaces-comment-split-max-content"
            | "anonymous-break-spaces-forced-break-max-content"
            | "anonymous-break-spaces-trailing-max-content" => {
                vec![ComputedCssGridTrackSize::MaxContent]
            }
            "anonymous-pre-wrap-min-content-spaces" => vec![ComputedCssGridTrackSize::MinContent],
            "anonymous-pre-min-content-spaces"
            | "anonymous-pre-tab-min-content"
            | "anonymous-pre-tab-size-2-min-content" => {
                vec![ComputedCssGridTrackSize::MinContent]
            }
            "anonymous-break-spaces-min-content"
            | "anonymous-break-spaces-forced-break-min-content" => {
                vec![ComputedCssGridTrackSize::MinContent]
            }
            "anonymous-pre-wrap-soft-wrap" => vec![ComputedCssGridTrackSize::LengthPx(25.0)],
            "anonymous-normal-fixed-width-control"
            | "anonymous-pre-fixed-width-nowrap"
            | "anonymous-pre-tab-fixed-width-nowrap"
            | "anonymous-pre-tab-size-2-fixed-overflow" => {
                vec![ComputedCssGridTrackSize::LengthPx(25.0)]
            }
            "anonymous-break-spaces-soft-wrap-25"
            | "anonymous-break-spaces-trailing-soft-wrap-25" => {
                vec![ComputedCssGridTrackSize::LengthPx(25.0)]
            }
            "anonymous-break-spaces-whitespace-only" => vec![ComputedCssGridTrackSize::Auto],
            "anonymous-pre-wrap-whitespace-before-block" => vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(20.0),
            ],
            "anonymous-break-spaces-whitespace-before-block" => vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(20.0),
            ],
            "anonymous-comment-continuation" => vec![
                ComputedCssGridTrackSize::MaxContent,
                ComputedCssGridTrackSize::Fraction(1.0),
            ],
            "anonymous-hidden-child-continuation" => vec![
                ComputedCssGridTrackSize::MaxContent,
                ComputedCssGridTrackSize::Fraction(1.0),
            ],
            "anonymous-runs-around-block" => vec![
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::MinContent,
            ],
            "whitespace-does-not-create-item" => vec![
                ComputedCssGridTrackSize::LengthPx(20.0),
                ComputedCssGridTrackSize::LengthPx(30.0),
                ComputedCssGridTrackSize::LengthPx(40.0),
            ],
            _ => return false,
        }
    };
    let expected_rows = if full_tab_scene(scene_id).is_some()
        || LINE_HEIGHT_SCENE_IDS.contains(&scene_id)
        || FONT_SIZE_SCENE_IDS.contains(&scene_id)
        || matches!(
            scene_id,
            "anonymous-min-content-wrap"
                | "anonymous-normal-min-content"
                | "anonymous-nowrap-min-content"
                | "anonymous-normal-newline-max-content"
                | "anonymous-pre-line-max-content"
                | "anonymous-normal-max-content-spaces"
                | "anonymous-pre-wrap-max-content-spaces"
                | "anonymous-pre-wrap-comment-split-max-content"
                | "anonymous-pre-wrap-min-content-spaces"
                | "anonymous-pre-wrap-soft-wrap"
                | "anonymous-pre-wrap-forced-break"
                | "anonymous-pre-max-content-spaces"
                | "anonymous-pre-min-content-spaces"
                | "anonymous-pre-forced-break"
                | "anonymous-normal-fixed-width-control"
                | "anonymous-pre-fixed-width-nowrap"
                | "anonymous-pre-tab-max-content"
                | "anonymous-pre-tab-leading"
                | "anonymous-pre-tab-exact-stop"
                | "anonymous-pre-tab-consecutive"
                | "anonymous-pre-tab-forced-break"
                | "anonymous-pre-tab-trailing"
                | "anonymous-pre-tab-min-content"
                | "anonymous-pre-tab-fixed-width-nowrap"
                | "anonymous-pre-tab-size-2-two-columns"
                | "anonymous-pre-tab-size-2-leading"
                | "anonymous-pre-tab-size-2-exact-stop"
                | "anonymous-pre-tab-size-2-consecutive"
                | "anonymous-pre-tab-size-2-reset"
                | "anonymous-pre-tab-size-2-crlf-reset"
                | "anonymous-pre-tab-size-2-trailing"
                | "anonymous-pre-tab-size-2-min-content"
                | "anonymous-pre-tab-size-2-fixed-overflow"
                | "anonymous-pre-tab-size-1-leading"
                | "anonymous-pre-tab-size-1-exact-stop"
                | "anonymous-pre-tab-size-4-leading"
                | "anonymous-pre-tab-size-4-exact-stop"
                | "anonymous-break-spaces-max-content"
                | "anonymous-break-spaces-comment-split-max-content"
                | "anonymous-break-spaces-min-content"
                | "anonymous-break-spaces-soft-wrap-25"
                | "anonymous-break-spaces-forced-break-max-content"
                | "anonymous-break-spaces-forced-break-min-content"
                | "anonymous-break-spaces-trailing-soft-wrap-25"
                | "anonymous-break-spaces-trailing-max-content"
                | "anonymous-break-spaces-whitespace-only"
        ) {
        vec![ComputedCssGridTrackSize::Auto]
    } else {
        vec![ComputedCssGridTrackSize::LengthPx(10.0)]
    };
    grid.columns == expected_columns
        && grid.rows == expected_rows
        && grid.columns_auto_repeat.is_none()
        && grid.rows_auto_repeat.is_none()
        && grid.template_areas.is_none()
        && grid.auto_flow == ComputedCssGridAutoFlow::Row
        && grid.auto_rows == ComputedCssGridAutoRows::Auto
        && grid.auto_columns == ComputedCssGridAutoColumns::Auto
        && grid.columns_line_names.len() == grid.columns.len() + 1
        && grid.rows_line_names.len() == grid.rows.len() + 1
        && grid.columns_line_names.iter().all(Vec::is_empty)
        && grid.rows_line_names.iter().all(Vec::is_empty)
}

fn validate_text_profile_properties(
    node_id: NodeId,
    properties: &BTreeMap<String, String>,
    expected_white_space_collapse: &str,
    expected_wrap_mode: &str,
    font_size: f32,
) -> Result<(), RuntimeLayoutFailure> {
    let font_size_css = font_size::font_size_css_value_for_px(font_size);
    for (name, expected) in [
        ("font-family", "Ahem"),
        ("font-weight", "400"),
        ("font-style", "normal"),
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
        let value = properties.get(name).map(String::as_str);
        if value != Some(expected) {
            return Err(layout_failure(
                "unsupported_c114_anonymous_text_style",
                Some(node_id.get()),
                Some(format!("{name}: expected {expected}, got {value:?}")),
            ));
        }
    }
    let actual_font_size = properties.get("font-size").map(String::as_str);
    if actual_font_size != Some(font_size_css) {
        return Err(layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some(format!(
                "font-size: expected {font_size_css}, got {actual_font_size:?}"
            )),
        ));
    }
    let actual_white_space_collapse = properties.get("white-space-collapse").map(String::as_str);
    if actual_white_space_collapse != Some(expected_white_space_collapse) {
        return Err(layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some(format!(
                "white-space-collapse: expected {expected_white_space_collapse}, got {actual_white_space_collapse:?}"
            )),
        ));
    }
    let actual_wrap_mode = properties.get("text-wrap-mode").map(String::as_str);
    if actual_wrap_mode != Some(expected_wrap_mode) {
        return Err(layout_failure(
            "unsupported_c114_anonymous_text_style",
            Some(node_id.get()),
            Some(format!(
                "text-wrap-mode: expected {expected_wrap_mode}, got {actual_wrap_mode:?}"
            )),
        ));
    }
    Ok(())
}

fn style_for(
    styles: &ComputedStyleSnapshot,
    node_id: NodeId,
) -> Result<&ComputedElementStyle, RuntimeLayoutFailure> {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == node_id)
        .ok_or_else(|| layout_failure("missing_computed_style", Some(node_id.get()), None))
}

fn element_identity(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
) -> Result<(String, String), RuntimeLayoutFailure> {
    let node = snapshot
        .node(handle)
        .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
    let HostNodeKind::Element(element) = node.kind() else {
        return Err(unsupported(handle.id(), "element expected"));
    };
    let id = element
        .attributes()
        .iter()
        .find_map(|(name, value)| {
            (name.namespace().is_none() && name.local_name() == "id")
                .then(|| value.to_string_lossy())
        })
        .unwrap_or_default();
    Ok((element.local_name().to_owned(), id))
}

fn anonymous_paragraph_style(
    scene_id: &str,
    tab_size: TabSize,
    line_height: ComputedLineHeight,
    font_size: f32,
) -> ParagraphStyle {
    let (line_height, line_height_is_normal) = match line_height {
        ComputedLineHeight::Normal => (10.0, true),
        ComputedLineHeight::AbsolutePx(value) => (value, false),
    };
    ParagraphStyle {
        writing_mode: spinon_core::WritingMode::HorizontalTb,
        text_orientation: spinon_core::TextOrientation::Mixed,
        font_family: "Ahem".to_owned(),
        font_size,
        line_height,
        line_height_is_normal,
        font_weight: 400.0,
        font_style: FontStyleMode::Normal,
        white_space: match scene_id {
            scene_id
                if LINE_HEIGHT_SCENE_IDS.contains(&scene_id)
                    && !scene_id.ends_with("soft-wrap") =>
            {
                WhiteSpaceMode::PreLine
            }
            "anonymous-nowrap-min-content" => WhiteSpaceMode::NoWrap,
            "anonymous-pre-line-max-content" => WhiteSpaceMode::PreLine,
            "anonymous-pre-wrap-max-content-spaces"
            | "anonymous-pre-wrap-comment-split-max-content"
            | "anonymous-pre-wrap-min-content-spaces"
            | "anonymous-pre-wrap-soft-wrap"
            | "anonymous-pre-wrap-forced-break"
            | "anonymous-pre-wrap-whitespace-before-block" => WhiteSpaceMode::PreWrap,
            "anonymous-pre-max-content-spaces"
            | "anonymous-pre-min-content-spaces"
            | "anonymous-pre-forced-break"
            | "anonymous-pre-fixed-width-nowrap" => WhiteSpaceMode::Pre,
            scene_id if is_pre_tab_scene(scene_id) => WhiteSpaceMode::PreWithTabSize(tab_size),
            "anonymous-break-spaces-max-content"
            | "anonymous-break-spaces-comment-split-max-content"
            | "anonymous-break-spaces-min-content"
            | "anonymous-break-spaces-soft-wrap-25"
            | "anonymous-break-spaces-forced-break-max-content"
            | "anonymous-break-spaces-forced-break-min-content"
            | "anonymous-break-spaces-trailing-soft-wrap-25"
            | "anonymous-break-spaces-trailing-max-content"
            | "anonymous-break-spaces-whitespace-only"
            | "anonymous-break-spaces-whitespace-before-block" => WhiteSpaceMode::BreakSpaces,
            _ => WhiteSpaceMode::Normal,
        },
        word_break: WordBreakMode::Normal,
        direction: TextParagraphDirection::Ltr,
        line_break: LineBreakMode::Strict,
        ligatures: LigatureMode::Normal,
    }
}

fn zero_edges(style: &ComputedElementStyle) -> bool {
    zero_spacing(style.layout_spacing.margin.top)
        && zero_spacing(style.layout_spacing.margin.right)
        && zero_spacing(style.layout_spacing.margin.bottom)
        && zero_spacing(style.layout_spacing.margin.left)
        && zero_spacing(style.layout_spacing.padding.top)
        && zero_spacing(style.layout_spacing.padding.right)
        && zero_spacing(style.layout_spacing.padding.bottom)
        && zero_spacing(style.layout_spacing.padding.left)
        && style.layout_border == Default::default()
}

fn zero_spacing(value: spinon_style::ComputedCssSpacingValue) -> bool {
    matches!(value, spinon_style::ComputedCssSpacingValue::LengthPx(value) if value.abs() <= f32::EPSILON)
}

fn unsupported(node: NodeId, reason: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_anonymous_text_fixture",
        Some(node.get()),
        Some(reason.to_owned()),
    )
}

enum ExpectedChild<'a> {
    Text(&'a str),
    Block(&'a str),
    Hidden { id: &'a str, text: &'a str },
}

#[cfg(test)]
mod tests {
    use super::super::c11_4_text::AHEM;
    use super::{
        ComputedLineHeight, FONT_SIZE_SCENE_IDS, FULL_TAB_CASE_IDS, FULL_TAB_COMPUTED_VALUES,
        FULL_TAB_CRLF_FIRST_LINE_WIDTHS, FULL_TAB_EXPECTED_WIDTHS, FULL_TAB_LF_SECOND_LINE_WIDTHS,
        FULL_TAB_VALUE_IDS, SCENE_IDS, anonymous_paragraph_style, full_tab_expected_width,
        full_tab_scene, full_tab_scene_id, grid_item_placement_is_auto, grid_tracks_match,
        parse_computed_tab_size, pre_tab_text, validate_computed_tab_size,
        validate_text_profile_properties,
    };
    use spinon_core::NodeId;
    use spinon_core::{TextOrientation, WritingMode};
    use spinon_layout::{
        FontStyleMode, LigatureMode, LineBreakMode, ParagraphInput, ParagraphStyle, TabSize,
        TextLayoutEngine, TextParagraphDirection, TextSource, WhiteSpaceMode, WordBreakMode,
    };
    use spinon_style::{
        ComputedCssGridAutoColumns, ComputedCssGridAutoFlow, ComputedCssGridAutoRepeat,
        ComputedCssGridAutoRepeatMode, ComputedCssGridAutoRows, ComputedCssGridItemPlacement,
        ComputedCssGridLayout, ComputedCssGridPlacement, ComputedCssGridTemplateAreas,
        ComputedCssGridTrackSize,
    };
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn anonymous_text_fixture_cases_are_unique() {
        let scene_ids = SCENE_IDS.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(SCENE_IDS.len(), 55);
        assert_eq!(scene_ids.len(), SCENE_IDS.len());
    }

    #[test]
    fn computed_font_size_fixture_cases_keep_expected_tracks_and_paragraph_sizes() {
        assert_eq!(FONT_SIZE_SCENE_IDS.len(), 10);
        for (scene_id, font_size, column) in [
            (
                "font-size-5-min-content",
                5.0,
                ComputedCssGridTrackSize::MinContent,
            ),
            (
                "font-size-5-max-content",
                5.0,
                ComputedCssGridTrackSize::MaxContent,
            ),
            (
                "font-size-7-5-min-content",
                7.5,
                ComputedCssGridTrackSize::MinContent,
            ),
            (
                "font-size-7-5-max-content",
                7.5,
                ComputedCssGridTrackSize::MaxContent,
            ),
            (
                "font-size-10-min-content",
                10.0,
                ComputedCssGridTrackSize::MinContent,
            ),
            (
                "font-size-10-max-content",
                10.0,
                ComputedCssGridTrackSize::MaxContent,
            ),
            (
                "font-size-12-5-min-content",
                12.5,
                ComputedCssGridTrackSize::MinContent,
            ),
            (
                "font-size-12-5-max-content",
                12.5,
                ComputedCssGridTrackSize::MaxContent,
            ),
            (
                "font-size-20-min-content",
                20.0,
                ComputedCssGridTrackSize::MinContent,
            ),
            (
                "font-size-20-max-content",
                20.0,
                ComputedCssGridTrackSize::MaxContent,
            ),
        ] {
            let expected_grid = grid(vec![column.clone()], vec![ComputedCssGridTrackSize::Auto]);
            assert!(
                grid_tracks_match(&expected_grid, scene_id),
                "{scene_id}: track mismatch"
            );
            let wrong_column = match &column {
                ComputedCssGridTrackSize::MinContent => ComputedCssGridTrackSize::MaxContent,
                ComputedCssGridTrackSize::MaxContent => ComputedCssGridTrackSize::MinContent,
                _ => unreachable!("font-size fixture column is intrinsic"),
            };
            assert!(
                !grid_tracks_match(
                    &grid(vec![wrong_column], vec![ComputedCssGridTrackSize::Auto]),
                    scene_id
                ),
                "{scene_id}: opposite intrinsic track must fail"
            );
            assert!(
                !grid_tracks_match(
                    &grid(
                        vec![column.clone()],
                        vec![ComputedCssGridTrackSize::LengthPx(10.0)],
                    ),
                    scene_id
                ),
                "{scene_id}: fixed row must fail"
            );
            let style = anonymous_paragraph_style(
                scene_id,
                TabSize::Number(8.0),
                ComputedLineHeight::AbsolutePx(10.0),
                font_size,
            );
            assert_eq!(
                style.font_size, font_size,
                "{scene_id}: paragraph font-size"
            );
            assert_eq!(style.line_height, 10.0, "{scene_id}: fixed line-height");
        }
    }

    #[test]
    fn computed_font_size_scales_anonymous_text_intrinsic_sizes_and_auto_height() {
        let node_id = NodeId::new(1).unwrap();
        let mut engine = TextLayoutEngine::new_without_system_fonts();
        engine.register_font("Ahem", AHEM).unwrap();

        for font_size in [5.0, 7.5, 10.0, 12.5, 20.0] {
            let paragraph = ParagraphInput {
                node: node_id,
                sources: vec![TextSource {
                    node: node_id,
                    inline_ancestors: Vec::new(),
                    text: "XX XX".to_owned(),
                }],
                style: anonymous_paragraph_style(
                    "font-size-matrix-test",
                    TabSize::Number(8.0),
                    ComputedLineHeight::AbsolutePx(10.0),
                    font_size,
                ),
                inline_styles: BTreeMap::new(),
                inline_run_styles: BTreeMap::new(),
            };
            let intrinsic = engine.intrinsic_inline_sizes(&paragraph).unwrap();
            assert!((intrinsic.min_content() - 2.0 * font_size).abs() < 0.02);
            assert!((intrinsic.max_content() - 5.0 * font_size).abs() < 0.02);
            assert!((intrinsic.min_content_height() - 20.0).abs() < 0.02);
            assert!((intrinsic.max_content_height() - 10.0).abs() < 0.02);

            let min_content = engine
                .layout_paragraph(&paragraph, 2.0 * font_size)
                .unwrap();
            assert_eq!(
                min_content.lines.len(),
                2,
                "font-size {font_size}px min-content"
            );
            assert!((min_content.width - 2.0 * font_size).abs() < 0.02);
            assert!((min_content.height - 20.0).abs() < 0.02);

            let max_content = engine
                .layout_paragraph(&paragraph, 5.0 * font_size)
                .unwrap();
            assert_eq!(
                max_content.lines.len(),
                1,
                "font-size {font_size}px max-content"
            );
            assert!((max_content.width - 5.0 * font_size).abs() < 0.02);
            assert!((max_content.height - 10.0).abs() < 0.02);
        }
    }

    #[test]
    fn anonymous_text_tab_size_comes_from_computed_style_and_rejects_other_values() {
        let node_id = NodeId::new(1).unwrap();
        for (scene_id, value, expected) in [
            (
                "anonymous-pre-tab-size-1-leading",
                "1",
                TabSize::Number(1.0),
            ),
            (
                "anonymous-pre-tab-size-2-two-columns",
                "2",
                TabSize::Number(2.0),
            ),
            (
                "anonymous-pre-tab-size-4-leading",
                "4",
                TabSize::Number(4.0),
            ),
            ("anonymous-pre-tab-max-content", "8", TabSize::Number(8.0)),
            (
                "anonymous-pre-tab-size-0-negative",
                "0",
                TabSize::Number(0.0),
            ),
            (
                "anonymous-pre-tab-size-fractional-negative",
                "1.5",
                TabSize::Number(1.5),
            ),
            (
                "anonymous-pre-tab-size-length-negative",
                "20px",
                TabSize::LengthPx(20.0),
            ),
        ] {
            let properties = BTreeMap::from([("tab-size".to_owned(), value.to_owned())]);
            assert_eq!(
                validate_computed_tab_size(node_id, &properties, scene_id).unwrap(),
                expected
            );
        }

        let mismatched = BTreeMap::from([("tab-size".to_owned(), "2".to_owned())]);
        assert!(
            validate_computed_tab_size(node_id, &mismatched, "anonymous-pre-tab-size-4-leading")
                .is_err()
        );
        for (scene_id, value) in [
            ("anonymous-pre-tab-size-0-negative", "-1"),
            ("anonymous-pre-tab-size-fractional-negative", "NaN"),
            ("anonymous-pre-tab-size-length-negative", "20em"),
        ] {
            let properties = BTreeMap::from([("tab-size".to_owned(), value.to_owned())]);
            assert!(validate_computed_tab_size(node_id, &properties, scene_id).is_err());
        }
        assert!(
            validate_computed_tab_size(
                node_id,
                &BTreeMap::new(),
                "anonymous-pre-tab-size-2-two-columns"
            )
            .is_err()
        );
    }

    #[test]
    fn anonymous_text_full_tab_size_matrix_matches_the_fixed_css_stop_oracle() {
        let node_id = NodeId::new(1).unwrap();
        let mut engine = TextLayoutEngine::new_without_system_fonts();
        engine.register_font("Ahem", AHEM).unwrap();
        let mut generated = BTreeSet::new();

        for (value_index, value_id) in FULL_TAB_VALUE_IDS.iter().enumerate() {
            let expected_style = parse_computed_tab_size(FULL_TAB_COMPUTED_VALUES[value_index])
                .expect("fixed computed tab-size value");
            for (case_index, case_id) in FULL_TAB_CASE_IDS.iter().enumerate() {
                let scene_id = full_tab_scene_id(value_id, case_id);
                assert!(generated.insert(scene_id.clone()));
                assert_eq!(full_tab_scene(&scene_id), Some((value_index, case_index)));
                assert_eq!(
                    full_tab_expected_width(&scene_id),
                    Some(FULL_TAB_EXPECTED_WIDTHS[value_index][case_index])
                );
                let expected_columns = match case_index {
                    8 => vec![ComputedCssGridTrackSize::MinContent],
                    9 => vec![ComputedCssGridTrackSize::LengthPx(25.0)],
                    _ => vec![ComputedCssGridTrackSize::MaxContent],
                };
                assert!(grid_tracks_match(
                    &grid(expected_columns, vec![ComputedCssGridTrackSize::Auto]),
                    &scene_id
                ));
                let computed = BTreeMap::from([(
                    "tab-size".to_owned(),
                    FULL_TAB_COMPUTED_VALUES[value_index].to_owned(),
                )]);
                assert_eq!(
                    validate_computed_tab_size(node_id, &computed, &scene_id).unwrap(),
                    expected_style,
                    "{scene_id} computed style"
                );

                let text = pre_tab_text(&scene_id);
                let style = ParagraphStyle {
                    writing_mode: WritingMode::HorizontalTb,
                    text_orientation: TextOrientation::Mixed,
                    font_family: "Ahem".to_owned(),
                    font_size: 10.0,
                    line_height: 10.0,
                    line_height_is_normal: false,
                    font_weight: 400.0,
                    font_style: FontStyleMode::Normal,
                    white_space: WhiteSpaceMode::PreWithTabSize(expected_style),
                    word_break: WordBreakMode::Normal,
                    direction: TextParagraphDirection::Ltr,
                    line_break: LineBreakMode::Strict,
                    ligatures: LigatureMode::Normal,
                };
                let paragraph = ParagraphInput {
                    node: node_id,
                    sources: vec![TextSource {
                        node: node_id,
                        inline_ancestors: Vec::new(),
                        text: text.to_owned(),
                    }],
                    style,
                    inline_styles: BTreeMap::new(),
                    inline_run_styles: BTreeMap::new(),
                };
                let intrinsic = engine.intrinsic_inline_sizes(&paragraph).unwrap();
                let expected_width = FULL_TAB_EXPECTED_WIDTHS[value_index][case_index];
                assert!(
                    (intrinsic.min_content() - expected_width).abs() < 0.02,
                    "{scene_id} min"
                );
                assert!(
                    (intrinsic.max_content() - expected_width).abs() < 0.02,
                    "{scene_id} max"
                );
                let layout = engine.layout_paragraph(&paragraph, 25.0).unwrap();
                assert_eq!(
                    layout.text,
                    text.replace('\t', "").replace("\r\n", "\n"),
                    "{scene_id} normalized text"
                );
                assert_eq!(
                    layout.lines.len(),
                    if case_index == 6 || case_index == 7 {
                        2
                    } else {
                        1
                    },
                    "{scene_id} line count"
                );
                assert!(
                    (layout.width - expected_width).abs() < 0.02,
                    "{scene_id} width"
                );
                if case_index == 6 {
                    assert!(
                        (layout.lines[1].advance - FULL_TAB_LF_SECOND_LINE_WIDTHS[value_index])
                            .abs()
                            < 0.02,
                        "{scene_id} LF reset"
                    );
                }
                if case_index == 7 {
                    assert!(
                        (layout.lines[0].advance - FULL_TAB_CRLF_FIRST_LINE_WIDTHS[value_index])
                            .abs()
                            < 0.02,
                        "{scene_id} CRLF first line"
                    );
                    assert!(
                        (layout.lines[1].advance - FULL_TAB_EXPECTED_WIDTHS[value_index][7]).abs()
                            < 0.02,
                        "{scene_id} CRLF reset"
                    );
                }
                let tab_sources = layout
                    .sources
                    .iter()
                    .filter(|source| source.paragraph_utf8.is_empty());
                assert_eq!(
                    tab_sources.count(),
                    text.matches('\t').count(),
                    "{scene_id} tab provenance"
                );
            }
        }
        assert_eq!(generated.len(), 140);
    }

    #[test]
    fn anonymous_text_block_separator_requires_auto_grid_placement() {
        let mut placement = ComputedCssGridItemPlacement {
            row_start: ComputedCssGridPlacement::Auto,
            row_end: ComputedCssGridPlacement::Auto,
            column_start: ComputedCssGridPlacement::Auto,
            column_end: ComputedCssGridPlacement::Auto,
        };
        assert!(grid_item_placement_is_auto(Some(&placement)));

        placement.column_start = ComputedCssGridPlacement::Line(1);
        assert!(!grid_item_placement_is_auto(Some(&placement)));
        assert!(!grid_item_placement_is_auto(None));
    }

    fn grid(
        columns: Vec<ComputedCssGridTrackSize>,
        rows: Vec<ComputedCssGridTrackSize>,
    ) -> ComputedCssGridLayout {
        ComputedCssGridLayout {
            columns_line_names: vec![Vec::new(); columns.len() + 1],
            rows_line_names: vec![Vec::new(); rows.len() + 1],
            columns,
            rows,
            columns_auto_repeat: None,
            rows_auto_repeat: None,
            template_areas: None,
            auto_flow: ComputedCssGridAutoFlow::Row,
            auto_rows: ComputedCssGridAutoRows::Auto,
            auto_columns: ComputedCssGridAutoColumns::Auto,
        }
    }

    #[test]
    fn anonymous_grid_fixture_rejects_typography_and_track_profile_mutations() {
        let properties = BTreeMap::from([
            ("font-family".to_owned(), "Ahem".to_owned()),
            ("font-size".to_owned(), "10px".to_owned()),
            ("font-weight".to_owned(), "400".to_owned()),
            ("font-style".to_owned(), "normal".to_owned()),
            ("line-height".to_owned(), "10px".to_owned()),
            ("tab-size".to_owned(), "8".to_owned()),
            ("white-space-collapse".to_owned(), "collapse".to_owned()),
            ("text-wrap-mode".to_owned(), "wrap".to_owned()),
            ("word-break".to_owned(), "normal".to_owned()),
            ("direction".to_owned(), "ltr".to_owned()),
            ("line-break".to_owned(), "auto".to_owned()),
            ("text-align".to_owned(), "start".to_owned()),
            ("overflow-wrap".to_owned(), "normal".to_owned()),
            ("text-transform".to_owned(), "none".to_owned()),
            ("writing-mode".to_owned(), "horizontal-tb".to_owned()),
            ("letter-spacing".to_owned(), "normal".to_owned()),
            ("word-spacing".to_owned(), "0px".to_owned()),
            ("text-indent".to_owned(), "0px".to_owned()),
        ]);
        assert!(
            validate_text_profile_properties(
                NodeId::new(1).unwrap(),
                &properties,
                "collapse",
                "wrap",
                10.0,
            )
            .is_ok()
        );
        let mut pre = properties.clone();
        pre.insert("white-space-collapse".to_owned(), "preserve".to_owned());
        pre.insert("text-wrap-mode".to_owned(), "nowrap".to_owned());
        assert!(
            validate_text_profile_properties(
                NodeId::new(1).unwrap(),
                &pre,
                "preserve",
                "nowrap",
                10.0,
            )
            .is_ok()
        );
        let mut nowrap = properties.clone();
        nowrap.insert("text-wrap-mode".to_owned(), "nowrap".to_owned());
        assert!(
            validate_text_profile_properties(
                NodeId::new(1).unwrap(),
                &nowrap,
                "collapse",
                "nowrap",
                10.0,
            )
            .is_ok()
        );
        let mut pre_line = properties.clone();
        pre_line.insert(
            "white-space-collapse".to_owned(),
            "preserve-breaks".to_owned(),
        );
        assert!(
            validate_text_profile_properties(
                NodeId::new(1).unwrap(),
                &pre_line,
                "preserve-breaks",
                "wrap",
                10.0,
            )
            .is_ok()
        );
        assert!(
            validate_text_profile_properties(
                NodeId::new(1).unwrap(),
                &pre_line,
                "collapse",
                "wrap",
                10.0,
            )
            .is_err()
        );
        let mut pre_wrap = properties.clone();
        pre_wrap.insert("white-space-collapse".to_owned(), "preserve".to_owned());
        assert!(
            validate_text_profile_properties(
                NodeId::new(1).unwrap(),
                &pre_wrap,
                "preserve",
                "wrap",
                10.0,
            )
            .is_ok()
        );
        for (name, changed) in [
            ("font-family", "sans-serif"),
            ("font-size", "11px"),
            ("font-weight", "700"),
            ("font-style", "italic"),
            ("white-space-collapse", "preserve"),
            ("text-wrap-mode", "nowrap"),
            ("word-break", "break-all"),
            ("direction", "rtl"),
            ("line-break", "strict"),
            ("text-align", "center"),
            ("overflow-wrap", "anywhere"),
            ("text-transform", "uppercase"),
            ("writing-mode", "vertical-rl"),
            ("letter-spacing", "1px"),
            ("word-spacing", "1px"),
            ("text-indent", "1px"),
        ] {
            let mut mutated = properties.clone();
            mutated.insert(name.to_owned(), changed.to_owned());
            assert!(
                validate_text_profile_properties(
                    NodeId::new(1).unwrap(),
                    &mutated,
                    "collapse",
                    "wrap",
                    10.0,
                )
                .is_err(),
                "the fixture must reject a changed {name}"
            );
        }

        let valid = grid(
            vec![
                ComputedCssGridTrackSize::MinContent,
                ComputedCssGridTrackSize::Fraction(1.0),
            ],
            vec![ComputedCssGridTrackSize::Auto],
        );
        assert!(grid_tracks_match(&valid, "anonymous-min-content-wrap"));
        let min_content_single_track = grid(
            vec![ComputedCssGridTrackSize::MinContent],
            vec![ComputedCssGridTrackSize::Auto],
        );
        assert!(grid_tracks_match(
            &min_content_single_track,
            "anonymous-normal-min-content"
        ));
        assert!(grid_tracks_match(
            &min_content_single_track,
            "anonymous-nowrap-min-content"
        ));
        let max_content_single_track = grid(
            vec![ComputedCssGridTrackSize::MaxContent],
            vec![ComputedCssGridTrackSize::Auto],
        );
        assert!(grid_tracks_match(
            &max_content_single_track,
            "anonymous-normal-newline-max-content"
        ));
        assert!(grid_tracks_match(
            &max_content_single_track,
            "anonymous-pre-line-max-content"
        ));
        for scene_id in [
            "anonymous-normal-max-content-spaces",
            "anonymous-pre-wrap-max-content-spaces",
            "anonymous-pre-wrap-comment-split-max-content",
            "anonymous-pre-wrap-forced-break",
        ] {
            assert!(grid_tracks_match(&max_content_single_track, scene_id));
        }
        assert!(grid_tracks_match(
            &min_content_single_track,
            "anonymous-pre-wrap-min-content-spaces"
        ));
        assert!(grid_tracks_match(
            &max_content_single_track,
            "anonymous-pre-max-content-spaces"
        ));
        assert!(grid_tracks_match(
            &max_content_single_track,
            "anonymous-pre-forced-break"
        ));
        for scene_id in [
            "anonymous-pre-tab-max-content",
            "anonymous-pre-tab-leading",
            "anonymous-pre-tab-exact-stop",
            "anonymous-pre-tab-consecutive",
            "anonymous-pre-tab-forced-break",
            "anonymous-pre-tab-trailing",
            "anonymous-pre-tab-size-2-two-columns",
            "anonymous-pre-tab-size-2-leading",
            "anonymous-pre-tab-size-2-exact-stop",
            "anonymous-pre-tab-size-2-consecutive",
            "anonymous-pre-tab-size-2-reset",
            "anonymous-pre-tab-size-2-crlf-reset",
            "anonymous-pre-tab-size-2-trailing",
            "anonymous-pre-tab-size-1-leading",
            "anonymous-pre-tab-size-1-exact-stop",
            "anonymous-pre-tab-size-4-leading",
            "anonymous-pre-tab-size-4-exact-stop",
        ] {
            assert!(grid_tracks_match(&max_content_single_track, scene_id));
        }
        assert!(grid_tracks_match(
            &min_content_single_track,
            "anonymous-pre-tab-min-content"
        ));
        assert!(grid_tracks_match(
            &min_content_single_track,
            "anonymous-pre-tab-size-2-min-content"
        ));
        assert!(grid_tracks_match(
            &min_content_single_track,
            "anonymous-pre-min-content-spaces"
        ));
        assert!(grid_tracks_match(
            &grid(
                vec![ComputedCssGridTrackSize::LengthPx(25.0)],
                vec![ComputedCssGridTrackSize::Auto],
            ),
            "anonymous-normal-fixed-width-control"
        ));
        assert!(grid_tracks_match(
            &grid(
                vec![ComputedCssGridTrackSize::LengthPx(25.0)],
                vec![ComputedCssGridTrackSize::Auto],
            ),
            "anonymous-pre-tab-size-2-fixed-overflow"
        ));
        assert!(grid_tracks_match(
            &grid(
                vec![ComputedCssGridTrackSize::LengthPx(25.0)],
                vec![ComputedCssGridTrackSize::Auto],
            ),
            "anonymous-pre-fixed-width-nowrap"
        ));
        assert!(grid_tracks_match(
            &grid(
                vec![ComputedCssGridTrackSize::LengthPx(25.0)],
                vec![ComputedCssGridTrackSize::Auto],
            ),
            "anonymous-pre-tab-fixed-width-nowrap"
        ));
        assert!(grid_tracks_match(
            &grid(
                vec![ComputedCssGridTrackSize::LengthPx(25.0)],
                vec![ComputedCssGridTrackSize::Auto],
            ),
            "anonymous-pre-wrap-soft-wrap"
        ));
        assert!(grid_tracks_match(
            &grid(
                vec![
                    ComputedCssGridTrackSize::LengthPx(20.0),
                    ComputedCssGridTrackSize::LengthPx(20.0),
                ],
                vec![ComputedCssGridTrackSize::LengthPx(10.0)],
            ),
            "anonymous-pre-wrap-whitespace-before-block"
        ));
        let break_spaces_max_track = grid(
            vec![ComputedCssGridTrackSize::MaxContent],
            vec![ComputedCssGridTrackSize::Auto],
        );
        for scene_id in [
            "anonymous-break-spaces-max-content",
            "anonymous-break-spaces-comment-split-max-content",
            "anonymous-break-spaces-forced-break-max-content",
            "anonymous-break-spaces-trailing-max-content",
        ] {
            assert!(grid_tracks_match(&break_spaces_max_track, scene_id));
        }
        let break_spaces_min_track = grid(
            vec![ComputedCssGridTrackSize::MinContent],
            vec![ComputedCssGridTrackSize::Auto],
        );
        for scene_id in [
            "anonymous-break-spaces-min-content",
            "anonymous-break-spaces-forced-break-min-content",
        ] {
            assert!(grid_tracks_match(&break_spaces_min_track, scene_id));
        }
        for scene_id in [
            "anonymous-break-spaces-soft-wrap-25",
            "anonymous-break-spaces-trailing-soft-wrap-25",
        ] {
            assert!(grid_tracks_match(
                &grid(
                    vec![ComputedCssGridTrackSize::LengthPx(25.0)],
                    vec![ComputedCssGridTrackSize::Auto],
                ),
                scene_id
            ));
        }

        macro_rules! rejects_grid_mutation {
            ($name:literal, $mutation:expr) => {{
                let mut mutated = valid.clone();
                $mutation(&mut mutated);
                assert!(
                    !grid_tracks_match(&mutated, "anonymous-min-content-wrap"),
                    "the fixture must reject {name}",
                    name = $name
                );
            }};
        }

        rejects_grid_mutation!("different columns", |grid: &mut ComputedCssGridLayout| {
            grid.columns[0] = ComputedCssGridTrackSize::LengthPx(20.0);
        });
        rejects_grid_mutation!("different rows", |grid: &mut ComputedCssGridLayout| {
            grid.rows = vec![ComputedCssGridTrackSize::LengthPx(10.0)];
        });
        rejects_grid_mutation!("column auto-repeat", |grid: &mut ComputedCssGridLayout| {
            grid.columns_auto_repeat = Some(ComputedCssGridAutoRepeat {
                mode: ComputedCssGridAutoRepeatMode::AutoFill,
                start: 0,
                end: 1,
                line_names: Vec::new(),
                fixed_repeat_minmax_track_indices: Vec::new(),
            });
        });
        rejects_grid_mutation!("row auto-repeat", |grid: &mut ComputedCssGridLayout| {
            grid.rows_auto_repeat = Some(ComputedCssGridAutoRepeat {
                mode: ComputedCssGridAutoRepeatMode::AutoFit,
                start: 0,
                end: 1,
                line_names: Vec::new(),
                fixed_repeat_minmax_track_indices: Vec::new(),
            });
        });
        rejects_grid_mutation!("template areas", |grid: &mut ComputedCssGridLayout| {
            grid.template_areas = Some(ComputedCssGridTemplateAreas {
                rows: 1,
                columns: 1,
                areas: Vec::new(),
            });
        });
        rejects_grid_mutation!("column auto-flow", |grid: &mut ComputedCssGridLayout| {
            grid.auto_flow = ComputedCssGridAutoFlow::Column;
        });
        rejects_grid_mutation!("dense auto-flow", |grid: &mut ComputedCssGridLayout| {
            grid.auto_flow = ComputedCssGridAutoFlow::RowDense;
        });
        rejects_grid_mutation!("auto rows", |grid: &mut ComputedCssGridLayout| {
            grid.auto_rows = ComputedCssGridAutoRows::LengthPx(10.0);
        });
        rejects_grid_mutation!("auto columns", |grid: &mut ComputedCssGridLayout| {
            grid.auto_columns = ComputedCssGridAutoColumns::LengthPx(10.0);
        });
        rejects_grid_mutation!("column line names", |grid: &mut ComputedCssGridLayout| {
            grid.columns_line_names[0].push("named".to_owned());
        });
        rejects_grid_mutation!("row line names", |grid: &mut ComputedCssGridLayout| {
            grid.rows_line_names[0].push("named".to_owned());
        });
    }
}

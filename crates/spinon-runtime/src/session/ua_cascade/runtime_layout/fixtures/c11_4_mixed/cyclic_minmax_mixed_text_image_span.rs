use std::collections::BTreeMap;

use spinon_core::{
    HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, IntrinsicSizeRevision,
};
use spinon_image::{ImageRequest, ImageResourceSnapshot};
use spinon_layout::TextLayoutEngine;
use spinon_style::{
    ComputedCssDimension, ComputedCssGridAutoColumns, ComputedCssGridAutoFlow,
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMath, ComputedCssMaxSize, ComputedCssPosition,
    ComputedCssSpacingValue, ComputedElementStyle, ComputedLayoutBorder, ComputedStyleProfile,
    ComputedStyleSnapshot,
};

use super::super::super::{RuntimeLayoutFailure, layout_failure};
use super::super::C11_4MixedInput;
use super::super::c11_4_nested_grid_depth_two::{
    children, element_identity, has_empty_attribute, style_for,
};
use super::{AHEM, IMAGE_URL, NATURAL_HEIGHT, NATURAL_WIDTH, PNG, ahem_paragraph_style};

#[derive(Clone, Copy)]
enum Width {
    MinContent,
    MaxContent,
    Px(f32),
}

#[derive(Clone, Copy)]
enum ItemKind {
    Text(&'static str),
    Image,
}

#[derive(Clone, Copy)]
struct Item {
    key: &'static str,
    kind: ItemKind,
    column_start: i32,
    column_end: i32,
}

#[derive(Clone, Copy)]
struct Scene {
    id: &'static str,
    width: Width,
    order: &'static [Item; 3],
}

const A: Item = Item {
    key: "a",
    kind: ItemKind::Text("AA A A"),
    column_start: 1,
    column_end: 3,
};
const B: Item = Item {
    key: "b",
    kind: ItemKind::Image,
    column_start: 2,
    column_end: 4,
};
const C: Item = Item {
    key: "c",
    kind: ItemKind::Text("AA AA AA AA"),
    column_start: 1,
    column_end: 4,
};
const FORWARD: [Item; 3] = [A, B, C];
const REVERSE: [Item; 3] = [C, B, A];

const SCENES: [Scene; 8] = [
    Scene {
        id: "cyclic-minmax-calc-mixed-text-image-span-intrinsic-forward",
        width: Width::MinContent,
        order: &FORWARD,
    },
    Scene {
        id: "cyclic-minmax-calc-mixed-text-image-span-intrinsic-reverse",
        width: Width::MinContent,
        order: &REVERSE,
    },
    Scene {
        id: "definite-minmax-calc-mixed-text-image-span-forward",
        width: Width::Px(40.0),
        order: &FORWARD,
    },
    Scene {
        id: "definite-minmax-calc-mixed-text-image-span-reverse",
        width: Width::Px(40.0),
        order: &REVERSE,
    },
    Scene {
        id: "max-content-forward",
        width: Width::MaxContent,
        order: &FORWARD,
    },
    Scene {
        id: "max-content-reverse",
        width: Width::MaxContent,
        order: &REVERSE,
    },
    Scene {
        id: "definite-forward",
        width: Width::Px(40.0),
        order: &FORWARD,
    },
    Scene {
        id: "definite-reverse",
        width: Width::Px(40.0),
        order: &REVERSE,
    },
];

pub(super) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<C11_4MixedInput>, RuntimeLayoutFailure> {
    let (tag, root_id) = element_identity(snapshot, root)?;
    let Some(scene) = SCENES
        .iter()
        .find(|scene| root_id == format!("c114-{}-grid", scene.id))
        .copied()
    else {
        return Ok(None);
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || tag != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
        || !has_empty_attribute(snapshot, root, "data-node")
    {
        return Err(unsupported(root, scene, "root identity, profile, or owner"));
    }

    validate_grid_style(style_for(styles, root.id())?, scene)?;
    let grid_children = children(snapshot, root)?;
    if grid_children.len() != scene.order.len() {
        return Err(unsupported(
            root,
            scene,
            "exactly three direct items required",
        ));
    }

    let mut paragraph_styles = BTreeMap::new();
    let mut image_completions = Vec::with_capacity(1);
    for (handle, expected) in grid_children.iter().zip(scene.order) {
        let (tag, id) = element_identity(snapshot, *handle)?;
        let expected_id = format!("c114-{}-item-{}", scene.id, expected.key);
        if id != expected_id
            || snapshot.parent(*handle) != Some(HostParent::Node(root))
            || !has_empty_attribute(snapshot, *handle, "data-node")
        {
            return Err(unsupported(
                *handle,
                scene,
                "direct child identity, order, or owner differs",
            ));
        }
        validate_item_style(style_for(styles, handle.id())?, scene, *expected)?;

        match (expected.kind, tag.as_str()) {
            (ItemKind::Text(text), "div") => {
                let text_children = children(snapshot, *handle)?;
                if text_children.len() != 1 {
                    return Err(unsupported(*handle, scene, "one direct text leaf required"));
                }
                let text_node = text_children[0];
                let actual = snapshot.node(text_node).and_then(|node| match node.kind() {
                    HostNodeKind::Text(value) => Some(value.to_string_lossy()),
                    HostNodeKind::Element(_) => None,
                });
                if actual.as_deref() != Some(text)
                    || snapshot.parent(text_node) != Some(HostParent::Node(*handle))
                {
                    return Err(unsupported(text_node, scene, "fixed Ahem text differs"));
                }
                paragraph_styles.insert(handle.id(), ahem_paragraph_style());
            }
            (ItemKind::Image, "img") => {
                if !children(snapshot, *handle)?.is_empty()
                    || super::image_src(snapshot, *handle)?.as_deref() != Some(IMAGE_URL)
                {
                    return Err(unsupported(*handle, scene, "fixed PNG image leaf required"));
                }
                let completion = ImageRequest::from_document(snapshot, *handle)
                    .map_err(|error| failure("c114_mixed_cyclic_image_request", *handle, error))?
                    .decode(PNG)
                    .map_err(|error| failure("c114_mixed_cyclic_image_decode", *handle, error))?;
                image_completions.push(completion);
            }
            _ => {
                return Err(unsupported(
                    *handle,
                    scene,
                    "item tag does not match source kind",
                ));
            }
        }
    }

    if image_completions.len() != 1 {
        return Err(unsupported(
            root,
            scene,
            "exactly one replaced image required",
        ));
    }
    let revision = IntrinsicSizeRevision::INITIAL
        .checked_next()
        .ok_or_else(|| failure("c114_mixed_cyclic_revision_exhausted", root, "revision"))?;
    let resources = ImageResourceSnapshot::new(snapshot, revision, image_completions)
        .map_err(|error| failure("c114_mixed_cyclic_image_snapshot", root, error))?;
    let image_handle = grid_children
        .iter()
        .find(|handle| element_identity(snapshot, **handle).is_ok_and(|(tag, _)| tag == "img"))
        .copied()
        .ok_or_else(|| failure("c114_mixed_cyclic_image_missing", root, "image item"))?;
    let image = resources.images().get(&image_handle.id()).ok_or_else(|| {
        failure(
            "c114_mixed_cyclic_image_metadata_missing",
            image_handle,
            "metadata",
        )
    })?;
    let dimensions = image.dimensions();
    if dimensions.width() != NATURAL_WIDTH || dimensions.height() != NATURAL_HEIGHT {
        return Err(failure(
            "c114_mixed_cyclic_image_metadata_mismatch",
            image_handle,
            format!("{}x{}", dimensions.width(), dimensions.height()),
        ));
    }
    let intrinsic_sizes = resources
        .to_layout_intrinsic_size_snapshot()
        .map_err(|error| failure("c114_mixed_cyclic_intrinsic_snapshot", root, error))?;

    let mut engine = TextLayoutEngine::new_without_system_fonts();
    engine
        .register_font("Ahem", AHEM)
        .map_err(|error| failure("c114_mixed_cyclic_font_registration", root, error))?;
    Ok(Some(C11_4MixedInput {
        paragraph_styles,
        engine,
        intrinsic_sizes,
        allow_two_axis_root_intrinsic: true,
    }))
}

fn validate_grid_style(
    style: &ComputedElementStyle,
    scene: Scene,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_width = match scene.width {
        Width::MinContent => ComputedCssDimension::MinContent,
        Width::MaxContent => ComputedCssDimension::MaxContent,
        Width::Px(value) => ComputedCssDimension::LengthPx(value),
    };
    let expected_border = ComputedLayoutBorder {
        top: 2.0,
        right: 2.0,
        bottom: 2.0,
        left: 2.0,
    };
    if property(style, "display") != Some("grid")
        || property(style, "box-sizing") != Some("content-box")
        || property(style, "align-content") != Some("start")
        || property(style, "justify-content") != Some("start")
        || property(style, "font-family") != Some("Ahem")
        || property(style, "font-size") != Some("10px")
        || property(style, "font-weight") != Some("400")
        || property(style, "font-style") != Some("normal")
        || property(style, "line-height") != Some("10px")
        || property(style, "color") != Some("rgb(229, 72, 77)")
        || style.layout_position != ComputedCssPosition::Static
        || style.layout_dimensions.width != expected_width
        || style.layout_dimensions.height != ComputedCssDimension::MaxContent
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || !spacing_is(style.layout_spacing.column_gap, 2.0)
        || !spacing_is(style.layout_spacing.row_gap, 2.0)
        || style.layout_border != expected_border
    {
        return Err(unsupported_node(
            style.node_id.get(),
            scene,
            "root Grid box differs",
        ));
    }
    let Some(grid) = style.grid_layout.as_ref() else {
        return Err(unsupported_node(
            style.node_id.get(),
            scene,
            "computed Grid is missing",
        ));
    };
    if !tracks_match(&grid.columns, 3)
        || !tracks_match(&grid.rows, 2)
        || grid.columns_auto_repeat.is_some()
        || grid.rows_auto_repeat.is_some()
        || grid.template_areas.is_some()
        || grid.auto_flow != ComputedCssGridAutoFlow::Row
        || grid.auto_columns != ComputedCssGridAutoColumns::Auto
        || grid.auto_rows != ComputedCssGridAutoRows::Auto
        || grid.columns_line_names.len() != 4
        || grid.rows_line_names.len() != 3
        || grid
            .columns_line_names
            .iter()
            .any(|names| !names.is_empty())
        || grid.rows_line_names.iter().any(|names| !names.is_empty())
    {
        return Err(unsupported_node(
            style.node_id.get(),
            scene,
            "typed track matrix differs",
        ));
    }
    Ok(())
}

fn validate_item_style(
    style: &ComputedElementStyle,
    scene: Scene,
    item: Item,
) -> Result<(), RuntimeLayoutFailure> {
    let placement_matches = style.grid_item_placement.as_ref().is_some_and(|placement| {
        placement.column_start == ComputedCssGridPlacement::Line(item.column_start)
            && placement.column_end == ComputedCssGridPlacement::Line(item.column_end)
            && placement.row_start == ComputedCssGridPlacement::Line(1)
            && placement.row_end == ComputedCssGridPlacement::Line(3)
    });
    let common_invalid = property(style, "display") != Some("block")
        || property(style, "box-sizing") != Some("border-box")
        || property(style, "color") != Some("rgb(229, 72, 77)")
        || style.layout_position != ComputedCssPosition::Static
        || !placement_matches
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != ComputedCssMaxSize::None
        || style.layout_dimensions.flex_basis != ComputedCssDimension::Auto
        || !zero_edges(style.layout_spacing.margin)
        || !zero_edges(style.layout_spacing.padding)
        || style.layout_border != Default::default();
    if common_invalid {
        return Err(unsupported_node(
            style.node_id.get(),
            scene,
            "item box or placement differs",
        ));
    }
    match item.kind {
        ItemKind::Text(_) => {
            if property(style, "align-self") != Some("auto")
                || property(style, "font-family") != Some("Ahem")
                || property(style, "font-size") != Some("10px")
                || property(style, "font-weight") != Some("400")
                || property(style, "font-style") != Some("normal")
                || property(style, "line-height") != Some("10px")
                || property(style, "white-space-collapse") != Some("collapse")
                || property(style, "text-wrap-mode") != Some("wrap")
                || property(style, "word-break") != Some("normal")
                || property(style, "direction") != Some("ltr")
                || property(style, "line-break") != Some("auto")
                || property(style, "text-align") != Some("start")
                || property(style, "overflow-wrap") != Some("normal")
                || property(style, "text-transform") != Some("none")
                || property(style, "writing-mode") != Some("horizontal-tb")
                || property(style, "letter-spacing") != Some("normal")
                || property(style, "word-spacing") != Some("0px")
                || property(style, "text-indent") != Some("0px")
                || (style.font_size_css_px - 10.0).abs() > f32::EPSILON
                || style.layout_dimensions.width != ComputedCssDimension::Auto
                || style.layout_dimensions.min_width != ComputedCssDimension::Auto
            {
                return Err(unsupported_node(
                    style.node_id.get(),
                    scene,
                    "Ahem text style differs",
                ));
            }
        }
        ItemKind::Image => {
            if property(style, "align-self") != Some("start")
                || property(style, "justify-self") != Some("auto")
                || style.layout_dimensions.width != ComputedCssDimension::LengthPx(40.0)
                || style.layout_dimensions.min_width != ComputedCssDimension::LengthPx(40.0)
                || style.layout_aspect_ratio.is_some()
            {
                return Err(unsupported_node(
                    style.node_id.get(),
                    scene,
                    "replaced image style differs",
                ));
            }
        }
    }
    Ok(())
}

fn tracks_match(actual: &[ComputedCssGridTrackSize], expected_len: usize) -> bool {
    actual.len() == expected_len
        && actual.iter().all(|track| match track {
            ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Calc(ComputedCssMath::Sum(terms)),
                max: ComputedCssGridTrackBreadth::MaxContent,
            } => {
                terms.len() == 2
                    && matches!(terms[0], ComputedCssMath::Percentage(value) if approximately(value, 0.1))
                    && matches!(terms[1], ComputedCssMath::LengthPx(value) if approximately(value, 2.0))
            }
            _ => false,
        })
}

fn property<'a>(style: &'a ComputedElementStyle, name: &str) -> Option<&'a str> {
    style.properties.get(name).map(String::as_str)
}

fn spacing_is(value: ComputedCssSpacingValue, expected: f32) -> bool {
    matches!(value, ComputedCssSpacingValue::LengthPx(actual) if approximately(actual, expected))
}

fn zero_edges(edges: spinon_style::ComputedCssEdges) -> bool {
    [edges.top, edges.right, edges.bottom, edges.left]
        .into_iter()
        .all(|value| spacing_is(value, 0.0))
}

fn approximately(actual: f32, expected: f32) -> bool {
    (actual - expected).abs() <= 1.0 / 64.0
}

fn unsupported(node: HostNodeHandle, scene: Scene, detail: &str) -> RuntimeLayoutFailure {
    unsupported_node(node.id().get(), scene, detail)
}

fn unsupported_node(node: u64, scene: Scene, detail: &str) -> RuntimeLayoutFailure {
    layout_failure(
        "unsupported_c114_cyclic_mixed_text_image_span_fixture",
        Some(node),
        Some(format!("{}: {detail}", scene.id)),
    )
}

fn failure(
    code: &'static str,
    node: HostNodeHandle,
    detail: impl ToString,
) -> RuntimeLayoutFailure {
    layout_failure(code, Some(node.id().get()), Some(detail.to_string()))
}

#[cfg(test)]
mod tests {
    use super::{FORWARD, REVERSE, SCENES, Width};

    #[test]
    fn exposes_only_eight_fixed_source_order_scenes() {
        assert_eq!(SCENES.len(), 8);
        assert_eq!(
            SCENES.map(|scene| scene.id),
            [
                "cyclic-minmax-calc-mixed-text-image-span-intrinsic-forward",
                "cyclic-minmax-calc-mixed-text-image-span-intrinsic-reverse",
                "definite-minmax-calc-mixed-text-image-span-forward",
                "definite-minmax-calc-mixed-text-image-span-reverse",
                "max-content-forward",
                "max-content-reverse",
                "definite-forward",
                "definite-reverse",
            ]
        );
        assert!(matches!(SCENES[0].width, Width::MinContent));
        assert!(matches!(SCENES[1].width, Width::MinContent));
        assert!(matches!(SCENES[2].width, Width::Px(value) if value == 40.0));
        assert!(matches!(SCENES[3].width, Width::Px(value) if value == 40.0));
        assert!(matches!(SCENES[4].width, Width::MaxContent));
        assert!(matches!(SCENES[5].width, Width::MaxContent));
        assert!(matches!(SCENES[6].width, Width::Px(value) if value == 40.0));
        assert!(matches!(SCENES[7].width, Width::Px(value) if value == 40.0));
        assert_eq!(FORWARD.map(|item| item.key), ["a", "b", "c"]);
        assert_eq!(REVERSE.map(|item| item.key), ["c", "b", "a"]);
        for scene in SCENES {
            assert_eq!(scene.order.len(), 3);
            assert_eq!(
                scene
                    .order
                    .iter()
                    .filter(|item| matches!(item.kind, super::ItemKind::Image))
                    .count(),
                1
            );
        }
    }
}

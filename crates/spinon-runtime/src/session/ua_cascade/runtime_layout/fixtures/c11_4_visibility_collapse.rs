use super::super::RuntimeLayoutCompleted;
use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, HostParent, NodeId};
use spinon_layout::LayoutOutput;
use spinon_style::{
    ComputedBackgroundPaint, ComputedCssDimension, ComputedCssGridItemPlacement,
    ComputedCssGridPlacement, ComputedCssGridTrackBreadth, ComputedCssGridTrackSize,
    ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot, OpaqueCssSrgb,
};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
const ROOT_PREFIX: &str = "c114-grid-visibility-collapse-";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Axis {
    Columns,
    Rows,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Collapse,
    Hidden,
    DisplayNone,
    CollapseVisibleDescendant,
    Visible,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SceneSpec {
    id: &'static str,
    axis: Axis,
    mode: Mode,
}

const SCENES: [SceneSpec; 10] = [
    SceneSpec {
        id: "collapse-column",
        axis: Axis::Columns,
        mode: Mode::Collapse,
    },
    SceneSpec {
        id: "hidden-column",
        axis: Axis::Columns,
        mode: Mode::Hidden,
    },
    SceneSpec {
        id: "display-none-column",
        axis: Axis::Columns,
        mode: Mode::DisplayNone,
    },
    SceneSpec {
        id: "collapse-visible-descendant-column",
        axis: Axis::Columns,
        mode: Mode::CollapseVisibleDescendant,
    },
    SceneSpec {
        id: "collapse-row",
        axis: Axis::Rows,
        mode: Mode::Collapse,
    },
    SceneSpec {
        id: "hidden-row",
        axis: Axis::Rows,
        mode: Mode::Hidden,
    },
    SceneSpec {
        id: "display-none-row",
        axis: Axis::Rows,
        mode: Mode::DisplayNone,
    },
    SceneSpec {
        id: "collapse-visible-descendant-row",
        axis: Axis::Rows,
        mode: Mode::CollapseVisibleDescendant,
    },
    SceneSpec {
        id: "visible-column",
        axis: Axis::Columns,
        mode: Mode::Visible,
    },
    SceneSpec {
        id: "visible-row",
        axis: Axis::Rows,
        mode: Mode::Visible,
    },
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Scene {
    spec: SceneSpec,
    root: NodeId,
    item: NodeId,
    visible_child: Option<NodeId>,
    content: NodeId,
}

pub(crate) fn is_root(snapshot: &HostDocumentSnapshot, root: HostNodeHandle) -> bool {
    scene_spec(snapshot, root).is_some()
}

pub(crate) fn prepare(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &ComputedStyleSnapshot,
) -> Result<Option<Scene>, super::super::RuntimeLayoutFailure> {
    let Some(spec) = scene_spec(snapshot, root) else {
        return Ok(None);
    };
    let reject = |node, detail: &str| {
        failure(
            node,
            "c114_visibility_collapse_fixture_rejected",
            Some(detail.to_owned()),
        )
    };
    if styles.profile != ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1
        || styles.generation != snapshot.generation()
        || styles.document_revision != snapshot.document_revision()
        || styles.render_tree_revision != snapshot.render_tree_revision()
    {
        return Err(reject(
            root.id(),
            "style snapshot profile or revision differs",
        ));
    }
    validate_element(snapshot, root, "div", &root_id(spec), HostParent::Root)?;
    let root_style = style_for(styles, root)?;
    require_property(root_style, "display", "grid")?;
    require_property(root_style, "visibility", "visible")?;
    require_background(root_style, [240, 240, 240])?;
    validate_root_size(root_style, spec.axis)?;
    validate_root_tracks(root_style, spec.axis)?;

    let children = snapshot
        .children(root)
        .ok_or_else(|| reject(root.id(), "root child list unavailable"))?
        .collect::<Vec<_>>();
    let [item] = children.as_slice() else {
        return Err(reject(root.id(), "exactly one direct Grid item required"));
    };
    let item = *item;
    let item_id = format!("{}-item", root_id(spec));
    validate_element(snapshot, item, "div", &item_id, HostParent::Node(root))?;
    let item_style = style_for(styles, item)?;
    let expected_display = if spec.mode == Mode::DisplayNone {
        "none"
    } else {
        "block"
    };
    let expected_visibility = match spec.mode {
        Mode::Collapse | Mode::CollapseVisibleDescendant => "collapse",
        Mode::Hidden => "hidden",
        Mode::DisplayNone | Mode::Visible => "visible",
    };
    require_property(item_style, "display", expected_display)?;
    require_property(item_style, "visibility", expected_visibility)?;
    validate_item_size(item_style)?;
    if spec.mode != Mode::DisplayNone {
        validate_item_placement(item_style, spec.axis)?;
    }

    let descendants = snapshot
        .children(item)
        .ok_or_else(|| reject(item.id(), "Grid item child list unavailable"))?
        .collect::<Vec<_>>();
    let has_visible_child = spec.mode == Mode::CollapseVisibleDescendant;
    if descendants.len() != if has_visible_child { 2 } else { 1 } {
        return Err(reject(item.id(), "Grid item child count differs"));
    }
    let (visible_child, content) = if has_visible_child {
        let [visible_child, content] = descendants.as_slice() else {
            unreachable!("exact child count was validated")
        };
        (Some(*visible_child), *content)
    } else {
        (None, descendants[0])
    };
    if let Some(visible_child) = visible_child {
        validate_element(
            snapshot,
            visible_child,
            "div",
            &format!("{}-visible-child", root_id(spec)),
            HostParent::Node(item),
        )?;
        let style = style_for(styles, visible_child)?;
        require_property(style, "display", "block")?;
        require_property(style, "visibility", "visible")?;
        validate_content_size(style, 8.0, 8.0)?;
        require_background(style, [0, 0, 255])?;
    }

    validate_element(
        snapshot,
        content,
        "div",
        &format!("{}-content", root_id(spec)),
        HostParent::Node(item),
    )?;
    let content_style = style_for(styles, content)?;
    require_property(content_style, "display", "block")?;
    let inherited_visibility = match spec.mode {
        Mode::Collapse | Mode::CollapseVisibleDescendant => "collapse",
        Mode::Hidden => "hidden",
        Mode::DisplayNone | Mode::Visible => "visible",
    };
    require_property(content_style, "visibility", inherited_visibility)?;
    let content_height = if spec.axis == Axis::Rows && spec.mode == Mode::CollapseVisibleDescendant
    {
        92.0
    } else if spec.axis == Axis::Rows {
        100.0
    } else {
        20.0
    };
    validate_content_size(
        content_style,
        if spec.axis == Axis::Columns {
            100.0
        } else {
            20.0
        },
        content_height,
    )?;
    require_background(content_style, [255, 0, 0])?;

    let expected_nodes = 3 + usize::from(has_visible_child);
    if styles.elements.len() != expected_nodes {
        return Err(reject(root.id(), "computed style node count differs"));
    }
    Ok(Some(Scene {
        spec,
        root: root.id(),
        item: item.id(),
        visible_child: visible_child.map(HostNodeHandle::id),
        content: content.id(),
    }))
}

pub(crate) fn validate_output(
    scene: Scene,
    layout: &LayoutOutput,
    completed: &RuntimeLayoutCompleted,
) -> Result<(), super::super::RuntimeLayoutFailure> {
    let reject = |node, detail: Option<String>| {
        failure(node, "c114_visibility_collapse_output_mismatch", detail)
    };
    let trace = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .and_then(|diagnostics| {
            diagnostics
                .traces
                .iter()
                .find(|trace| trace.node == scene.root)
        })
        .ok_or_else(|| {
            reject(
                scene.root,
                Some("Grid track-sizing trace missing".to_owned()),
            )
        })?;
    let (columns, rows) = match (scene.spec.axis, scene.spec.mode) {
        (Axis::Columns, Mode::DisplayNone) => ("0px 0px", "20px"),
        (Axis::Rows, Mode::DisplayNone) => ("20px", "0px 0px"),
        (Axis::Columns, _) => ("50px 50px", "20px"),
        (Axis::Rows, _) => ("20px", "50px 50px"),
    };
    if trace.used_columns.used_track_list != columns || trace.used_rows.used_track_list != rows {
        return Err(reject(
            scene.root,
            Some(format!(
                "resolved tracks are {:?}/{:?}, expected {columns}/{rows}",
                trace.used_columns.used_track_list, trace.used_rows.used_track_list
            )),
        ));
    }
    let root_size = if scene.spec.axis == Axis::Columns {
        [80.0, 20.0]
    } else {
        [20.0, 80.0]
    };
    let item_size = if scene.spec.mode == Mode::DisplayNone {
        [0.0, 0.0]
    } else if scene.spec.axis == Axis::Columns {
        [100.0, 20.0]
    } else {
        [20.0, 100.0]
    };
    assert_frame(layout, scene.root, [0.0, 0.0, root_size[0], root_size[1]])?;
    assert_optional_frame(layout, scene.item, [0.0, 0.0, item_size[0], item_size[1]])?;
    let content_size = if scene.spec.mode == Mode::DisplayNone {
        [0.0, 0.0]
    } else if scene.spec.axis == Axis::Columns {
        [100.0, 20.0]
    } else if scene.spec.mode == Mode::CollapseVisibleDescendant {
        [20.0, 92.0]
    } else {
        [20.0, 100.0]
    };
    let content_y = if scene.visible_child.is_some() {
        8.0
    } else {
        0.0
    };
    assert_optional_frame(
        layout,
        scene.content,
        [0.0, content_y, content_size[0], content_size[1]],
    )?;
    if let Some(visible_child) = scene.visible_child {
        let frame = layout.frames.get(&visible_child).copied().ok_or_else(|| {
            reject(
                visible_child,
                Some("visible descendant frame missing".to_owned()),
            )
        })?;
        if [frame.x, frame.y, frame.width, frame.height] != [0.0, 0.0, 8.0, 8.0] {
            return Err(reject(
                visible_child,
                Some("visible descendant frame differs".to_owned()),
            ));
        }
    }

    let render = completed
        .render_snapshot
        .as_ref()
        .ok_or_else(|| reject(scene.root, Some("render snapshot missing".to_owned())))?;
    let item_expected_in_render = scene.spec.mode == Mode::Visible;
    let item_box = render
        .boxes()
        .iter()
        .any(|render_box| render_box.node_id() == scene.item);
    if item_box != item_expected_in_render {
        return Err(reject(
            scene.item,
            Some("visibility paint suppression differs".to_owned()),
        ));
    }
    let visible_child_box = scene.visible_child.is_some_and(|node| {
        render
            .boxes()
            .iter()
            .any(|render_box| render_box.node_id() == node)
    });
    if visible_child_box != scene.visible_child.is_some() {
        return Err(reject(
            scene.item,
            Some("visible descendant paint differs".to_owned()),
        ));
    }
    let root_point = if scene.spec.axis == Axis::Columns {
        (40.0, 10.0)
    } else {
        (10.0, 40.0)
    };
    let expected_root_hit = matches!(
        scene.spec.mode,
        Mode::Collapse | Mode::Hidden | Mode::DisplayNone
    );
    if expected_root_hit
        && render
            .hit_test_css_point(root_point.0, root_point.1)
            .is_none_or(|render_box| render_box.node_id() != scene.root)
    {
        return Err(reject(
            scene.root,
            Some("hidden scene hit target is not the root".to_owned()),
        ));
    }
    if scene.spec.mode == Mode::CollapseVisibleDescendant
        && render
            .hit_test_css_point(4.0, 4.0)
            .is_none_or(|render_box| Some(render_box.node_id()) != scene.visible_child)
    {
        return Err(reject(
            scene.item,
            Some("visible descendant hit target differs".to_owned()),
        ));
    }
    let overflow_point = if scene.spec.axis == Axis::Columns {
        (81.0, 10.0)
    } else {
        (10.0, 81.0)
    };
    match scene.spec.mode {
        Mode::Visible => {
            if render
                .hit_test_css_point(overflow_point.0, overflow_point.1)
                .is_none_or(|render_box| render_box.node_id() != scene.content)
            {
                return Err(reject(
                    scene.content,
                    Some("visible overflow hit target differs".to_owned()),
                ));
            }
        }
        _ if scene.spec.mode != Mode::DisplayNone
            && render
                .hit_test_css_point(overflow_point.0, overflow_point.1)
                .is_some() =>
        {
            return Err(reject(
                scene.item,
                Some("invisible overflow remains hit-testable".to_owned()),
            ));
        }
        _ => {}
    }
    Ok(())
}

fn scene_spec(snapshot: &HostDocumentSnapshot, root: HostNodeHandle) -> Option<SceneSpec> {
    let node = snapshot.node(root)?;
    let HostNodeKind::Element(element) = node.kind() else {
        return None;
    };
    if element.namespace() != HTML_NAMESPACE
        || element.local_name() != "div"
        || snapshot.parent(root) != Some(HostParent::Root)
    {
        return None;
    }
    let id = element.attributes().iter().find_map(|(name, value)| {
        (name.namespace().is_none() && name.local_name() == "id").then(|| value.to_string_lossy())
    })?;
    let scene_id = id.strip_prefix(ROOT_PREFIX)?;
    SCENES.iter().find(|scene| scene.id == scene_id).copied()
}

fn root_id(scene: SceneSpec) -> String {
    format!("{ROOT_PREFIX}{}", scene.id)
}

fn validate_element(
    snapshot: &HostDocumentSnapshot,
    handle: HostNodeHandle,
    tag: &str,
    expected_id: &str,
    parent: HostParent,
) -> Result<(), super::super::RuntimeLayoutFailure> {
    let reject = |detail: &str| {
        failure(
            handle.id(),
            "c114_visibility_collapse_tree_rejected",
            Some(detail.to_owned()),
        )
    };
    let node = snapshot
        .node(handle)
        .ok_or_else(|| reject("node missing"))?;
    let HostNodeKind::Element(element) = node.kind() else {
        return Err(reject("expected HTML element"));
    };
    let id = element.attributes().iter().find_map(|(name, value)| {
        (name.namespace().is_none() && name.local_name() == "id").then(|| value.to_string_lossy())
    });
    if element.namespace() != HTML_NAMESPACE
        || element.local_name() != tag
        || id.as_deref() != Some(expected_id)
        || snapshot.parent(handle) != Some(parent)
    {
        return Err(reject("element identity or parent differs"));
    }
    Ok(())
}

fn style_for(
    styles: &ComputedStyleSnapshot,
    handle: HostNodeHandle,
) -> Result<&ComputedElementStyle, super::super::RuntimeLayoutFailure> {
    styles
        .elements
        .iter()
        .find(|style| style.node_id == handle.id())
        .ok_or_else(|| failure(handle.id(), "c114_visibility_collapse_missing_style", None))
}

fn require_property(
    style: &ComputedElementStyle,
    property: &str,
    expected: &str,
) -> Result<(), super::super::RuntimeLayoutFailure> {
    let actual = style.properties.get(property).map(String::as_str);
    if actual != Some(expected) {
        return Err(failure(
            style.node_id,
            "c114_visibility_collapse_computed_style_mismatch",
            Some(format!("{property}={actual:?}, expected {expected}")),
        ));
    }
    Ok(())
}

fn validate_root_size(
    style: &ComputedElementStyle,
    axis: Axis,
) -> Result<(), super::super::RuntimeLayoutFailure> {
    let expected = if axis == Axis::Columns {
        [80.0, 20.0]
    } else {
        [20.0, 80.0]
    };
    if style.layout_dimensions.width != ComputedCssDimension::LengthPx(expected[0])
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(expected[1])
    {
        return Err(failure(
            style.node_id,
            "c114_visibility_collapse_root_size",
            None,
        ));
    }
    Ok(())
}

fn validate_root_tracks(
    style: &ComputedElementStyle,
    axis: Axis,
) -> Result<(), super::super::RuntimeLayoutFailure> {
    let Some(grid) = style.grid_layout.as_ref() else {
        return Err(failure(
            style.node_id,
            "c114_visibility_collapse_grid_missing",
            None,
        ));
    };
    let intrinsic_track = ComputedCssGridTrackSize::MinMax {
        min: ComputedCssGridTrackBreadth::Auto,
        max: ComputedCssGridTrackBreadth::MaxContent,
    };
    let tracks = vec![intrinsic_track.clone(), intrinsic_track];
    let valid = if axis == Axis::Columns {
        grid.columns == tracks && grid.rows == [ComputedCssGridTrackSize::LengthPx(20.0)]
    } else {
        grid.columns == [ComputedCssGridTrackSize::LengthPx(20.0)] && grid.rows == tracks
    };
    if !valid {
        return Err(failure(
            style.node_id,
            "c114_visibility_collapse_grid_tracks",
            None,
        ));
    }
    Ok(())
}

fn validate_item_size(
    style: &ComputedElementStyle,
) -> Result<(), super::super::RuntimeLayoutFailure> {
    if style.layout_dimensions.width != ComputedCssDimension::Auto
        || style.layout_dimensions.height != ComputedCssDimension::Auto
    {
        return Err(failure(
            style.node_id,
            "c114_visibility_collapse_item_size",
            None,
        ));
    }
    Ok(())
}

fn validate_item_placement(
    style: &ComputedElementStyle,
    axis: Axis,
) -> Result<(), super::super::RuntimeLayoutFailure> {
    let Some(placement) = style.grid_item_placement.as_ref() else {
        return Err(failure(
            style.node_id,
            "c114_visibility_collapse_placement_missing",
            None,
        ));
    };
    let expected = if axis == Axis::Columns {
        ComputedCssGridItemPlacement {
            row_start: ComputedCssGridPlacement::Line(1),
            row_end: ComputedCssGridPlacement::Line(2),
            column_start: ComputedCssGridPlacement::Line(1),
            column_end: ComputedCssGridPlacement::Span(2),
        }
    } else {
        ComputedCssGridItemPlacement {
            row_start: ComputedCssGridPlacement::Line(1),
            row_end: ComputedCssGridPlacement::Span(2),
            column_start: ComputedCssGridPlacement::Line(1),
            column_end: ComputedCssGridPlacement::Line(2),
        }
    };
    if placement != &expected {
        return Err(failure(
            style.node_id,
            "c114_visibility_collapse_placement",
            None,
        ));
    }
    Ok(())
}

fn validate_content_size(
    style: &ComputedElementStyle,
    width: f32,
    height: f32,
) -> Result<(), super::super::RuntimeLayoutFailure> {
    if style.layout_dimensions.width != ComputedCssDimension::LengthPx(width)
        || style.layout_dimensions.height != ComputedCssDimension::LengthPx(height)
    {
        return Err(failure(
            style.node_id,
            "c114_visibility_collapse_content_size",
            None,
        ));
    }
    Ok(())
}

fn require_background(
    style: &ComputedElementStyle,
    expected: [u8; 3],
) -> Result<(), super::super::RuntimeLayoutFailure> {
    if style.background_paint
        != Some(ComputedBackgroundPaint::Opaque(OpaqueCssSrgb {
            red: expected[0],
            green: expected[1],
            blue: expected[2],
        }))
    {
        return Err(failure(
            style.node_id,
            "c114_visibility_collapse_background",
            None,
        ));
    }
    Ok(())
}

fn assert_frame(
    layout: &LayoutOutput,
    node: NodeId,
    expected: [f32; 4],
) -> Result<(), super::super::RuntimeLayoutFailure> {
    let frame = layout
        .frames
        .get(&node)
        .copied()
        .ok_or_else(|| failure(node, "c114_visibility_collapse_frame_missing", None))?;
    if [frame.x, frame.y, frame.width, frame.height] != expected {
        return Err(failure(
            node,
            "c114_visibility_collapse_frame_mismatch",
            Some(format!(
                "{:?}, expected {expected:?}",
                [frame.x, frame.y, frame.width, frame.height]
            )),
        ));
    }
    Ok(())
}

fn assert_optional_frame(
    layout: &LayoutOutput,
    node: NodeId,
    expected: [f32; 4],
) -> Result<(), super::super::RuntimeLayoutFailure> {
    if let Some(frame) = layout.frames.get(&node).copied() {
        if [frame.x, frame.y, frame.width, frame.height] != expected {
            return Err(failure(
                node,
                "c114_visibility_collapse_frame_mismatch",
                Some(format!(
                    "{:?}, expected {expected:?}",
                    [frame.x, frame.y, frame.width, frame.height]
                )),
            ));
        }
    } else if expected[2] != 0.0 || expected[3] != 0.0 {
        return Err(failure(
            node,
            "c114_visibility_collapse_frame_missing",
            None,
        ));
    }
    Ok(())
}

fn failure(
    node: NodeId,
    code: &'static str,
    detail: Option<String>,
) -> super::super::RuntimeLayoutFailure {
    super::super::layout_failure(code, Some(node.get()), detail)
}

use std::collections::{BTreeMap, BTreeSet};

use spinon_core::{HostDocumentSnapshot, HostNodeHandle, HostNodeKind, NodeId};
use spinon_style::{
    ComputedCssDimension, ComputedCssOverflow, ComputedCssSpacingValue, ComputedElementStyle,
    ComputedLayoutBorder,
};

use super::{FONT_SIZE, HTML_NAMESPACE, RuntimeLayoutFailure, layout_failure};

#[derive(Clone, Copy)]
pub(super) struct FixtureNodes {
    pub(super) body: NodeId,
    pub(super) main: NodeId,
    pub(super) visible: NodeId,
    pub(super) visible_text: NodeId,
    pub(super) hidden: NodeId,
    pub(super) hidden_text: NodeId,
    pub(super) not_rendered: NodeId,
    pub(super) not_rendered_text: NodeId,
}

pub(super) fn find_fixture_nodes(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &BTreeMap<NodeId, &ComputedElementStyle>,
) -> Result<FixtureNodes, RuntimeLayoutFailure> {
    let root_node = snapshot
        .node(root)
        .ok_or_else(|| layout_failure("invalid_root_html", Some(root.id().get()), None))?;
    let spinon_core::HostNodeKind::Element(root_element) = root_node.kind() else {
        return Err(layout_failure(
            "invalid_root_html",
            Some(root.id().get()),
            None,
        ));
    };
    if root_element.namespace() != HTML_NAMESPACE || root_element.local_name() != "html" {
        return Err(layout_failure(
            "invalid_root_html",
            Some(root.id().get()),
            None,
        ));
    }

    let mut by_id = BTreeMap::<String, HostNodeHandle>::new();
    let mut pending = vec![root];
    while let Some(handle) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
        if let HostNodeKind::Element(element) = node.kind()
            && let Some(id) = element.attributes().iter().find_map(|(name, value)| {
                (name.namespace().is_none() && name.local_name() == "id")
                    .then(|| value.to_string_lossy())
            })
            && by_id.insert(id.clone(), handle).is_some()
        {
            return Err(layout_failure(
                "duplicate_root_html_text_fixture_id",
                Some(handle.id().get()),
                Some(id),
            ));
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children);
        }
    }

    let by_named_id = |name: &str| {
        by_id.get(name).copied().ok_or_else(|| {
            layout_failure(
                "missing_root_html_text_fixture_id",
                None,
                Some(name.to_owned()),
            )
        })
    };
    let body = find_element_by_tag(snapshot, root, "body")?;
    let main = by_named_id("stage")?;
    let visible = by_named_id("visible")?;
    let hidden = by_named_id("hidden")?;
    let not_rendered = by_named_id("not-rendered")?;
    for handle in [body, main, visible, hidden, not_rendered] {
        if !styles.contains_key(&handle.id()) {
            return Err(layout_failure(
                "missing_computed_style",
                Some(handle.id().get()),
                None,
            ));
        }
    }
    if snapshot.parent(main) != Some(spinon_core::HostParent::Node(body))
        || snapshot.parent(visible) != Some(spinon_core::HostParent::Node(main))
        || snapshot.parent(hidden) != Some(spinon_core::HostParent::Node(main))
        || snapshot.parent(not_rendered) != Some(spinon_core::HostParent::Node(main))
    {
        return Err(layout_failure(
            "root_html_text_fixture_tree_mismatch",
            None,
            None,
        ));
    }
    validate_element_children(snapshot, body, &[main])?;
    validate_element_children(snapshot, main, &[visible, hidden, not_rendered])?;
    let visible_text = only_direct_text(snapshot, visible, "XX")?;
    let hidden_text = only_direct_text(snapshot, hidden, "XX")?;
    let not_rendered_text = only_direct_text(snapshot, not_rendered, "HIDDEN")?;
    Ok(FixtureNodes {
        body: body.id(),
        main: main.id(),
        visible: visible.id(),
        visible_text,
        hidden: hidden.id(),
        hidden_text,
        not_rendered: not_rendered.id(),
        not_rendered_text,
    })
}

fn find_element_by_tag(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    local_name: &str,
) -> Result<HostNodeHandle, RuntimeLayoutFailure> {
    let mut pending = vec![root];
    let mut matches = Vec::new();
    while let Some(handle) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
        if let HostNodeKind::Element(element) = node.kind()
            && element.namespace() == HTML_NAMESPACE
            && element.local_name() == local_name
        {
            matches.push(handle);
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(children);
        }
    }
    if matches.len() != 1 {
        return Err(layout_failure(
            "root_html_text_fixture_tree_mismatch",
            None,
            Some(format!(
                "expected one {local_name}, found {}",
                matches.len()
            )),
        ));
    }
    Ok(matches[0])
}

fn validate_element_children(
    snapshot: &HostDocumentSnapshot,
    parent: HostNodeHandle,
    expected: &[HostNodeHandle],
) -> Result<(), RuntimeLayoutFailure> {
    let mut elements = Vec::new();
    if let Some(children) = snapshot.children(parent) {
        for child in children {
            let node = snapshot
                .node(child)
                .ok_or_else(|| layout_failure("missing_node", Some(child.id().get()), None))?;
            match node.kind() {
                HostNodeKind::Element(_) => elements.push(child),
                HostNodeKind::Text(text) if text.is_only_collapsible_html_whitespace() => {}
                HostNodeKind::Text(_) => {
                    return Err(layout_failure(
                        "root_html_text_fixture_tree_mismatch",
                        Some(child.id().get()),
                        None,
                    ));
                }
            }
        }
    }
    if elements != expected {
        return Err(layout_failure(
            "root_html_text_fixture_tree_mismatch",
            Some(parent.id().get()),
            None,
        ));
    }
    Ok(())
}

fn only_direct_text(
    snapshot: &HostDocumentSnapshot,
    parent: HostNodeHandle,
    expected: &str,
) -> Result<NodeId, RuntimeLayoutFailure> {
    let children = snapshot
        .children(parent)
        .ok_or_else(|| layout_failure("missing_node", Some(parent.id().get()), None))?
        .collect::<Vec<_>>();
    if children.len() != 1 {
        return Err(layout_failure(
            "root_html_text_direct_child_count",
            Some(parent.id().get()),
            None,
        ));
    }
    let child = children[0];
    let node = snapshot
        .node(child)
        .ok_or_else(|| layout_failure("missing_node", Some(child.id().get()), None))?;
    let HostNodeKind::Text(text) = node.kind() else {
        return Err(layout_failure(
            "root_html_text_direct_child_mismatch",
            Some(child.id().get()),
            None,
        ));
    };
    if text.to_string_lossy() != expected {
        return Err(layout_failure(
            "root_html_text_source_mismatch",
            Some(child.id().get()),
            None,
        ));
    }
    Ok(child.id())
}

pub(super) fn validate_visible_tree(
    snapshot: &HostDocumentSnapshot,
    root: HostNodeHandle,
    styles: &BTreeMap<NodeId, &ComputedElementStyle>,
    nodes: &FixtureNodes,
) -> Result<(), RuntimeLayoutFailure> {
    let expected_visible = BTreeSet::from([
        root.id(),
        nodes.body,
        nodes.main,
        nodes.visible,
        nodes.hidden,
    ]);
    let mut visited_visible = BTreeSet::new();
    let mut pending = vec![(root, false)];
    while let Some((handle, ancestor_hidden)) = pending.pop() {
        let node = snapshot
            .node(handle)
            .ok_or_else(|| layout_failure("missing_node", Some(handle.id().get()), None))?;
        if let HostNodeKind::Text(text) = node.kind() {
            if !ancestor_hidden
                && !text.is_only_collapsible_html_whitespace()
                && ![nodes.visible_text, nodes.hidden_text].contains(&node.id())
            {
                return Err(layout_failure(
                    "unsupported_visible_root_html_text",
                    Some(node.id().get()),
                    None,
                ));
            }
            continue;
        }
        let style = styles
            .get(&node.id())
            .copied()
            .ok_or_else(|| layout_failure("missing_computed_style", Some(node.id().get()), None))?;
        let display = property(style, "display")?;
        let hidden = ancestor_hidden || display == "none";
        if !hidden {
            let HostNodeKind::Element(element) = node.kind() else {
                unreachable!("text nodes returned above")
            };
            if element.namespace() != HTML_NAMESPACE
                || !matches!(element.local_name(), "html" | "body" | "main" | "div")
                || display != "block"
            {
                return Err(layout_failure(
                    "unsupported_visible_root_html_element",
                    Some(node.id().get()),
                    Some(format!(
                        "{}:{}:{display}",
                        element.namespace(),
                        element.local_name()
                    )),
                ));
            }
            visited_visible.insert(node.id());
        }
        if let Some(children) = snapshot.children(handle) {
            pending.extend(
                children
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|child| (child, hidden)),
            );
        }
    }
    if visited_visible != expected_visible
        || nodes.not_rendered_text == nodes.visible_text
        || nodes.not_rendered_text == nodes.hidden_text
    {
        return Err(layout_failure(
            "root_html_text_fixture_tree_mismatch",
            None,
            None,
        ));
    }
    Ok(())
}

pub(super) fn validate_box_style(
    style: &ComputedElementStyle,
    label: &str,
    width: f32,
    height: f32,
) -> Result<(), RuntimeLayoutFailure> {
    let node = style.node_id;
    if property(style, "display")? != "block"
        || property(style, "box-sizing")? != "content-box"
        || !is_px(style.layout_dimensions.width, width)
        || !is_px(style.layout_dimensions.height, height)
        || style.layout_overflow_x != Some(ComputedCssOverflow::Visible)
        || style.layout_overflow_y != Some(ComputedCssOverflow::Visible)
    {
        return Err(layout_failure(
            "unsupported_root_html_text_box_style",
            Some(node.get()),
            Some(label.to_owned()),
        ));
    }
    validate_zero_edges(style, label)
}

pub(super) fn validate_text_owner_style(
    style: &ComputedElementStyle,
    overflow: ComputedCssOverflow,
) -> Result<(), RuntimeLayoutFailure> {
    let node = style.node_id;
    if property(style, "display")? != "block"
        || property(style, "box-sizing")? != "content-box"
        || !is_px(style.layout_dimensions.width, 15.0)
        || style.layout_dimensions.height != ComputedCssDimension::Auto
        || style.layout_dimensions.min_width != ComputedCssDimension::Auto
        || style.layout_dimensions.min_height != ComputedCssDimension::Auto
        || style.layout_dimensions.max_width != spinon_style::ComputedCssMaxSize::None
        || style.layout_dimensions.max_height != spinon_style::ComputedCssMaxSize::None
        || style.layout_overflow_x != Some(overflow)
        || style.layout_overflow_y != Some(overflow)
    {
        return Err(layout_failure(
            "unsupported_root_html_text_owner_style",
            Some(node.get()),
            None,
        ));
    }
    validate_zero_edges(style, "text owner")?;
    for (name, expected) in [
        ("font-family", "Ahem"),
        ("font-weight", "400"),
        ("font-style", "normal"),
        ("line-height", "10px"),
        ("white-space-collapse", "collapse"),
        ("text-wrap-mode", "wrap"),
        ("word-break", "normal"),
        ("overflow-wrap", "normal"),
        ("letter-spacing", "normal"),
        ("word-spacing", "0px"),
        ("text-indent", "0px"),
        ("text-transform", "none"),
        ("text-align", "start"),
        ("direction", "ltr"),
        ("writing-mode", "horizontal-tb"),
    ] {
        if property(style, name)? != expected {
            return Err(layout_failure(
                "unsupported_root_html_text_style",
                Some(node.get()),
                Some(name.to_owned()),
            ));
        }
    }
    if (style.font_size_css_px - FONT_SIZE).abs() > f32::EPSILON {
        return Err(layout_failure(
            "unsupported_root_html_text_style",
            Some(node.get()),
            Some("font-size".to_owned()),
        ));
    }
    let expected_color = [9.0 / 255.0, 33.0 / 255.0, 112.0 / 255.0, 1.0];
    let color = style.computed_color.ok_or_else(|| {
        layout_failure(
            "missing_root_html_text_color",
            Some(node.get()),
            Some("color".to_owned()),
        )
    })?;
    if !super::text::color_matches(color, expected_color) {
        return Err(layout_failure(
            "unsupported_root_html_text_color",
            Some(node.get()),
            Some("color".to_owned()),
        ));
    }
    Ok(())
}

pub(super) fn validate_hidden_style(
    style: &ComputedElementStyle,
) -> Result<(), RuntimeLayoutFailure> {
    if property(style, "display")? != "none" {
        return Err(layout_failure(
            "root_html_text_hidden_control_visible",
            Some(style.node_id.get()),
            Some("display".to_owned()),
        ));
    }
    Ok(())
}

fn validate_zero_edges(
    style: &ComputedElementStyle,
    label: &str,
) -> Result<(), RuntimeLayoutFailure> {
    let zero = ComputedCssSpacingValue::LengthPx(0.0);
    if [
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
    .any(|value| value != zero)
        || !border_is_zero(style.layout_border)
        || [
            "border-top-style",
            "border-right-style",
            "border-bottom-style",
            "border-left-style",
        ]
        .into_iter()
        .any(|property_name| !property(style, property_name).is_ok_and(|value| value == "none"))
    {
        return Err(layout_failure(
            "unsupported_root_html_text_box_edges",
            Some(style.node_id.get()),
            Some(label.to_owned()),
        ));
    }
    Ok(())
}

fn border_is_zero(border: ComputedLayoutBorder) -> bool {
    border.top == 0.0 && border.right == 0.0 && border.bottom == 0.0 && border.left == 0.0
}

fn is_px(value: ComputedCssDimension, expected: f32) -> bool {
    matches!(value, ComputedCssDimension::LengthPx(actual) if (actual - expected).abs() <= f32::EPSILON)
}

fn property<'a>(
    style: &'a ComputedElementStyle,
    name: &str,
) -> Result<&'a str, RuntimeLayoutFailure> {
    style
        .properties
        .get(name)
        .map(String::as_str)
        .ok_or_else(|| {
            layout_failure(
                "missing_root_html_text_computed_property",
                Some(style.node_id.get()),
                Some(name.to_owned()),
            )
        })
}

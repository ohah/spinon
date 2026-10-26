use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostDocumentSnapshot,
    HostNodeHandle, HostNodeKind, HostParent, OwnerId, StyleRevision,
};
use spinon_layout::{WhiteSpaceMode, WordBreakMode};
use spinon_style::{
    CssOrigin, StylesheetSource, StyloDocumentView,
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};

use super::super::super::super::RuntimeLayoutFailure;
use super::cases::{HTML, ItemExpectation, SceneExpectation, VIEWPORT};

#[derive(Clone)]
pub(super) enum NodeSpec {
    Element {
        namespace: &'static str,
        tag: &'static str,
        id: Option<String>,
        class: Option<String>,
        style: Option<String>,
        children: Vec<NodeSpec>,
    },
    Text(String),
}

pub(super) struct Fixture {
    pub(super) snapshot: HostDocumentSnapshot,
    pub(super) root: HostNodeHandle,
    pub(super) items: Vec<HostNodeHandle>,
    pub(super) view: StyloDocumentView,
}

pub(super) fn positive_fixture(scene: SceneExpectation) -> Fixture {
    custom_fixture(
        scene.root_id,
        scene.root_class,
        scene.items.iter().map(item_spec).collect(),
        None,
    )
}

pub(super) fn positive_fixture_with_inline_style(scene: SceneExpectation, style: &str) -> Fixture {
    custom_fixture(
        scene.root_id,
        scene.root_class,
        scene
            .items
            .iter()
            .enumerate()
            .map(|(index, expected)| {
                let mut spec = item_spec(expected);
                if let (0, NodeSpec::Element { style: inline, .. }) = (index, &mut spec) {
                    *inline = Some(style.to_owned());
                }
                spec
            })
            .collect(),
        None,
    )
}

pub(super) fn item_spec(expected: &ItemExpectation) -> NodeSpec {
    item(
        expected.id,
        expected.class,
        vec![NodeSpec::text(expected.text)],
    )
}

pub(super) fn item(id: &str, class: &str, children: Vec<NodeSpec>) -> NodeSpec {
    element(HTML, "div", Some(id), Some(class), None, children)
}

pub(super) fn element(
    namespace: &'static str,
    tag: &'static str,
    id: Option<&str>,
    class: Option<&str>,
    style: Option<&str>,
    children: Vec<NodeSpec>,
) -> NodeSpec {
    NodeSpec::Element {
        namespace,
        tag,
        id: id.map(str::to_owned),
        class: class.map(str::to_owned),
        style: style.map(str::to_owned),
        children,
    }
}

impl NodeSpec {
    pub(super) fn text(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

pub(super) fn custom_fixture(
    root_id: &str,
    root_class: &str,
    children: Vec<NodeSpec>,
    root_style: Option<&str>,
) -> Fixture {
    let root = NodeSpec::Element {
        namespace: HTML,
        tag: "div",
        id: Some(root_id.to_owned()),
        class: Some(root_class.to_owned()),
        style: root_style.map(str::to_owned),
        children,
    };
    let mut document = HostDocument::new().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_414).unwrap(), document.document_revision());
    let root_handle = append_spec(&mut document, &mut batch, HostParent::Root, root);
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root_handle)
            .unwrap();
    let items = snapshot
        .children(root_handle)
        .unwrap()
        .filter(|handle| {
            matches!(
                snapshot.node(*handle).map(|node| node.kind()),
                Some(HostNodeKind::Element(_))
            )
        })
        .collect();
    Fixture {
        snapshot,
        root: root_handle,
        items,
        view,
    }
}

fn append_spec(
    document: &mut HostDocument,
    batch: &mut DocumentChangeBatch,
    parent: HostParent,
    spec: NodeSpec,
) -> HostNodeHandle {
    let handle = document.reserve_node_handle().unwrap();
    let children = match spec {
        NodeSpec::Text(value) => {
            batch.push(DocumentOperation::CreateText {
                node: handle,
                data: value.into(),
            });
            Vec::new()
        }
        NodeSpec::Element {
            namespace,
            tag,
            id,
            class,
            style,
            children,
        } => {
            batch.push(DocumentOperation::CreateElement {
                node: handle,
                namespace: namespace.to_owned(),
                local_name: tag.to_owned(),
            });
            for (name, value) in [("id", id), ("class", class), ("style", style)] {
                if let Some(value) = value {
                    batch.push(DocumentOperation::SetAttribute {
                        node: handle,
                        name: AttributeName::new(None, name).unwrap(),
                        value: value.into(),
                    });
                }
            }
            children
        }
    };
    batch.push(DocumentOperation::InsertBefore {
        parent,
        node: handle,
        before: None,
    });
    for child in children {
        append_spec(document, batch, HostParent::Node(handle), child);
    }
    handle
}

pub(super) fn cascade(view: &StyloDocumentView, css: &str) -> spinon_style::ComputedStyleSnapshot {
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        view,
        &[stylesheet(css)],
        VIEWPORT,
        StyleRevision::INITIAL,
    )
    .unwrap()
}

pub(super) fn stylesheet(css: &str) -> StylesheetSource {
    StylesheetSource {
        id: "c11-4-generic-direct-text".to_owned(),
        base_url: "https://spinon.invalid/c11-4/generic-direct-text.css".to_owned(),
        origin: CssOrigin::Author,
        css: css.to_owned(),
    }
}

pub(super) fn direct_text(snapshot: &HostDocumentSnapshot, item: HostNodeHandle) -> HostNodeHandle {
    snapshot
        .children(item)
        .unwrap()
        .find(|handle| {
            matches!(
                snapshot.node(*handle).map(|node| node.kind()),
                Some(HostNodeKind::Text(text)) if !text.is_only_collapsible_html_whitespace()
            )
        })
        .unwrap()
}

pub(super) fn expected_white_space(scene: SceneExpectation) -> WhiteSpaceMode {
    if scene
        .root_class
        .split_whitespace()
        .any(|class| class == "nowrap")
    {
        WhiteSpaceMode::NoWrap
    } else {
        WhiteSpaceMode::Normal
    }
}

pub(super) fn expected_word_break(value: &str) -> WordBreakMode {
    match value {
        "normal" => WordBreakMode::Normal,
        "break-all" => WordBreakMode::BreakAll,
        _ => unreachable!(),
    }
}

pub(super) fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{label}: expected {expected}, got {actual}"
    );
}

pub(super) fn close_option(actual: Option<f32>, expected: f32, label: &str) {
    close(actual.unwrap_or(f32::NAN), expected, label);
}

pub(super) fn take_error<T>(
    result: Result<T, RuntimeLayoutFailure>,
    label: &str,
) -> RuntimeLayoutFailure {
    match result {
        Err(error) => error,
        Ok(_) => panic!("{label}: operation unexpectedly returned output"),
    }
}

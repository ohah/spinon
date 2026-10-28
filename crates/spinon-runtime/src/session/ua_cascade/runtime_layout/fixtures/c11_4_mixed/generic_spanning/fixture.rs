use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostDocumentSnapshot,
    HostNodeHandle, HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};

use super::super::HTML_NAMESPACE;

pub(super) const VIEWPORT: CssViewport = CssViewport {
    width_css_px: 1280.0,
    height_css_px: 633.0,
    device_scale_factor: 1.0,
    environment_revision: spinon_core::EnvironmentRevision::INITIAL,
    media_environment: CssMediaEnvironment::DESKTOP,
};

const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
const STYLESHEET: &str = r#"
.root {
  box-sizing: border-box;
  display: grid;
  width: 220px;
  height: 180px;
  margin: 0;
  padding: 0;
  row-gap: 0;
  column-gap: 0;
  align-items: start;
  align-content: start;
  justify-content: start;
  grid-template-columns: repeat(4, min-content);
  grid-template-rows: repeat(3, 60px);
}
.root.bad-track { grid-template-columns: repeat(3, min-content); }
.root.bad-edge { padding-left: 1px; }
.item {
  box-sizing: border-box;
  display: block;
  position: static;
  width: auto;
  height: auto;
  min-width: auto;
  min-height: auto;
  margin: 0;
  padding: 0;
  justify-self: auto;
  align-self: auto;
}
.text {
  font-family: Ahem;
  font-size: 10px;
  font-weight: 400;
  font-style: normal;
  line-height: 10px;
  white-space-collapse: collapse;
  text-wrap-mode: wrap;
  word-break: normal;
  direction: ltr;
  line-break: auto;
  text-align: start;
  overflow-wrap: normal;
  text-transform: none;
  letter-spacing: normal;
  word-spacing: 0px;
  text-indent: 0px;
}
.text-a { grid-column: 1 / span 2; grid-row: 1; }
.text-c { grid-column: 1 / span 3; grid-row: 3; }
.image { grid-column: 2 / span 2; grid-row: 2; height: 48px; }
.bad-height { height: 47px; }
.bad-placement { grid-column: 1 / span 1; }
"#;

#[derive(Clone, Copy)]
pub(super) enum Order {
    TwoForward,
    TwoReversed,
    ThreeForward,
    ThreeReversed,
}

pub(super) struct Options {
    pub(super) order: Order,
    pub(super) bad_text: bool,
    pub(super) bad_image_source: bool,
    pub(super) omit_image: bool,
    pub(super) extra_child: bool,
    pub(super) nested_text: bool,
    pub(super) bad_track: bool,
    pub(super) bad_edge: bool,
    pub(super) bad_a_placement: bool,
    pub(super) bad_image_height: bool,
}

impl Options {
    pub(super) fn valid(order: Order) -> Self {
        Self {
            order,
            bad_text: false,
            bad_image_source: false,
            omit_image: false,
            extra_child: false,
            nested_text: false,
            bad_track: false,
            bad_edge: false,
            bad_a_placement: false,
            bad_image_height: false,
        }
    }
}

pub(super) struct Fixture {
    pub(super) document: HostDocument,
    pub(super) snapshot: HostDocumentSnapshot,
    pub(super) root: HostNodeHandle,
    pub(super) view: StyloDocumentView,
}

enum NodeSpec {
    Element {
        tag: &'static str,
        id: &'static str,
        class: String,
        attributes: Vec<(&'static str, String)>,
        children: Vec<NodeSpec>,
    },
    Text(&'static str),
}

fn item_node(kind: &'static str, options: &Options) -> NodeSpec {
    let (tag, class, text, attributes) = match kind {
        "a" => (
            "div",
            if options.bad_a_placement {
                "item text text-a bad-placement"
            } else {
                "item text text-a"
            },
            Some(if options.bad_text {
                "XX XXXX"
            } else {
                "XX XXXXX"
            }),
            Vec::new(),
        ),
        "b" => (
            "img",
            if options.bad_image_height {
                "item image bad-height"
            } else {
                "item image"
            },
            None,
            vec![(
                "src",
                if options.bad_image_source {
                    "https://spinon.invalid/wrong.png".to_owned()
                } else {
                    IMAGE_URL.to_owned()
                },
            )],
        ),
        "c" => (
            "div",
            "item text text-c",
            Some("XXXXXXXXXXXXXXXXXX"),
            Vec::new(),
        ),
        _ => unreachable!("fixture item kind is fixed"),
    };
    let mut children = text.into_iter().map(NodeSpec::Text).collect::<Vec<_>>();
    if options.nested_text && kind == "a" {
        children = vec![NodeSpec::Element {
            tag: "span",
            id: "generic-host-nested-text",
            class: String::new(),
            attributes: Vec::new(),
            children,
        }];
    }
    NodeSpec::Element {
        tag,
        id: match kind {
            "a" => "generic-host-text-a",
            "b" => "generic-host-image-b",
            "c" => "generic-host-text-c",
            _ => unreachable!("fixture item kind is fixed"),
        },
        class: class.to_owned(),
        attributes,
        children,
    }
}

pub(super) fn make_fixture(options: Options) -> Fixture {
    let order: &[&str] = match options.order {
        Order::TwoForward => &["a", "b"],
        Order::TwoReversed => &["b", "a"],
        Order::ThreeForward => &["a", "b", "c"],
        Order::ThreeReversed => &["c", "b", "a"],
    };
    let mut children = order
        .iter()
        .filter(|kind| !(options.omit_image && **kind == "b"))
        .map(|kind| item_node(kind, &options))
        .collect::<Vec<_>>();
    if options.extra_child {
        children.push(NodeSpec::Element {
            tag: "div",
            id: "generic-host-extra",
            class: "item text text-a".to_owned(),
            attributes: Vec::new(),
            children: vec![NodeSpec::Text("X")],
        });
    }
    let root_id = match options.order {
        Order::TwoForward => "generic-host-root-span2-forward",
        Order::TwoReversed => "generic-host-root-span2-reversed",
        Order::ThreeForward => "generic-host-root-span2-before-span3",
        Order::ThreeReversed => "generic-host-root-span3-before-span2",
    };
    let root_class = if options.bad_track {
        "root bad-track"
    } else if options.bad_edge {
        "root bad-edge"
    } else {
        "root"
    };
    let root = NodeSpec::Element {
        tag: "div",
        id: root_id,
        class: root_class.to_owned(),
        attributes: Vec::new(),
        children,
    };
    let mut document = HostDocument::new().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_489).unwrap(), document.document_revision());
    let root_handle = append_spec(&mut document, &mut batch, HostParent::Root, root);
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root_handle)
            .unwrap();
    Fixture {
        document,
        snapshot,
        root: root_handle,
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
            tag,
            id,
            class,
            attributes,
            children,
        } => {
            batch.push(DocumentOperation::CreateElement {
                node: handle,
                namespace: HTML_NAMESPACE.to_owned(),
                local_name: tag.to_owned(),
            });
            for (name, value) in [("id", id.to_owned()), ("class", class)]
                .into_iter()
                .chain(attributes)
            {
                batch.push(DocumentOperation::SetAttribute {
                    node: handle,
                    name: AttributeName::new(None, name).unwrap(),
                    value: value.into(),
                });
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

pub(super) fn cascade(view: &StyloDocumentView) -> spinon_style::ComputedStyleSnapshot {
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets(
        view,
        &[StylesheetSource {
            id: "c11-4-generic-mixed-spanning".to_owned(),
            base_url: "https://spinon.invalid/c11-4/generic-mixed-spanning.css".to_owned(),
            origin: CssOrigin::Author,
            css: STYLESHEET.to_owned(),
        }],
        VIEWPORT,
        StyleRevision::INITIAL,
    )
    .unwrap()
}

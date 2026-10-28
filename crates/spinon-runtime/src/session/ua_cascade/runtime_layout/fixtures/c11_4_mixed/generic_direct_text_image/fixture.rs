use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostDocumentSnapshot,
    HostNodeHandle, HostNodeKind, HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets,
};

use super::super::HTML_NAMESPACE;
use super::element_identity;

const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";

pub(super) const VIEWPORT: CssViewport = CssViewport {
    width_css_px: 1280.0,
    height_css_px: 633.0,
    device_scale_factor: 1.0,
    environment_revision: spinon_core::EnvironmentRevision::INITIAL,
    media_environment: CssMediaEnvironment::DESKTOP,
};
const STYLESHEET: &str = r#"
.root {
  box-sizing: border-box;
  display: grid;
  width: 120px;
  height: 48px;
  margin: 0;
  padding: 0;
  row-gap: 0;
  column-gap: 0;
  align-items: start;
  align-content: start;
  justify-content: start;
  justify-items: start;
  direction: ltr;
  writing-mode: horizontal-tb;
  grid-template-rows: 24px 24px;
}
.root.min { grid-template-columns: min-content; }
.root.max { grid-template-columns: max-content; }
.root.bad-track { grid-template-columns: min-content max-content; }
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
  grid-column: 1 / span 1;
  grid-row: 1;
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
.image { grid-column: 1 / span 1; grid-row: 2; }
.bad-placement { grid-column: 2 / span 1; }
"#;

#[derive(Clone, Copy)]
pub(super) enum Track {
    MinContent,
    MaxContent,
}

pub(super) struct Options {
    pub(super) root_id: &'static str,
    pub(super) text_id: &'static str,
    pub(super) image_id: &'static str,
    pub(super) track: Track,
    pub(super) reversed: bool,
    pub(super) text: &'static str,
    pub(super) image_src: Option<&'static str>,
    pub(super) bad_track: bool,
    pub(super) bad_edge: bool,
    pub(super) bad_text_placement: bool,
    pub(super) extra_child: bool,
    pub(super) nested_text: bool,
    pub(super) include_text: bool,
    pub(super) include_image: bool,
}

impl Options {
    pub(super) fn novel(track: Track, reversed: bool) -> Self {
        Self {
            root_id: match (track, reversed) {
                (Track::MinContent, false) => "novel-mixed-min-forward-root",
                (Track::MinContent, true) => "novel-mixed-min-reversed-root",
                (Track::MaxContent, false) => "novel-mixed-max-forward-root",
                (Track::MaxContent, true) => "novel-mixed-max-reversed-root",
            },
            text_id: match (track, reversed) {
                (Track::MinContent, false) => "novel-mixed-min-forward-text",
                (Track::MinContent, true) => "novel-mixed-min-reversed-text",
                (Track::MaxContent, false) => "novel-mixed-max-forward-text",
                (Track::MaxContent, true) => "novel-mixed-max-reversed-text",
            },
            image_id: match (track, reversed) {
                (Track::MinContent, false) => "novel-mixed-min-forward-image",
                (Track::MinContent, true) => "novel-mixed-min-reversed-image",
                (Track::MaxContent, false) => "novel-mixed-max-forward-image",
                (Track::MaxContent, true) => "novel-mixed-max-reversed-image",
            },
            track,
            reversed,
            text: "XX XXXXX",
            image_src: Some(IMAGE_URL),
            bad_track: false,
            bad_edge: false,
            bad_text_placement: false,
            extra_child: false,
            nested_text: false,
            include_text: true,
            include_image: true,
        }
    }

    fn root_class(&self) -> String {
        let mut classes = vec![
            "root",
            match self.track {
                Track::MinContent => "min",
                Track::MaxContent => "max",
            },
        ];
        if self.bad_track {
            classes.push("bad-track");
        }
        if self.bad_edge {
            classes.push("bad-edge");
        }
        classes.join(" ")
    }
}

pub(super) struct Fixture {
    pub(super) document: HostDocument,
    pub(super) snapshot: HostDocumentSnapshot,
    pub(super) root: HostNodeHandle,
    pub(super) text_item: Option<HostNodeHandle>,
    pub(super) image_item: Option<HostNodeHandle>,
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

pub(super) fn make_fixture(options: Options) -> Fixture {
    let text_class = if options.bad_text_placement {
        "item text bad-placement".to_owned()
    } else {
        "item text".to_owned()
    };
    let text_child = if options.nested_text {
        NodeSpec::Element {
            tag: "span",
            id: "novel-nested-text-child",
            class: String::new(),
            attributes: Vec::new(),
            children: vec![NodeSpec::Text(options.text)],
        }
    } else {
        NodeSpec::Text(options.text)
    };
    let text = NodeSpec::Element {
        tag: "div",
        id: options.text_id,
        class: text_class,
        attributes: Vec::new(),
        children: vec![text_child],
    };
    let mut image_attributes = vec![("alt", String::new())];
    if let Some(source) = options.image_src {
        image_attributes.push(("src", source.to_owned()));
    }
    let image = NodeSpec::Element {
        tag: "img",
        id: options.image_id,
        class: "item image".to_owned(),
        attributes: image_attributes,
        children: Vec::new(),
    };
    let mut children = Vec::new();
    if options.reversed {
        if options.include_image {
            children.push(image);
        }
        if options.include_text {
            children.push(text);
        }
    } else {
        if options.include_text {
            children.push(text);
        }
        if options.include_image {
            children.push(image);
        }
    }
    if options.extra_child {
        children.push(NodeSpec::Element {
            tag: "div",
            id: "novel-extra-child",
            class: "item text".to_owned(),
            attributes: Vec::new(),
            children: vec![NodeSpec::Text("X")],
        });
    }
    let root = NodeSpec::Element {
        tag: "div",
        id: options.root_id,
        class: options.root_class(),
        attributes: Vec::new(),
        children,
    };

    let mut document = HostDocument::new().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_488).unwrap(), document.document_revision());
    let root_handle = append_spec(&mut document, &mut batch, HostParent::Root, root);
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root_handle)
            .unwrap();
    let direct_children = snapshot.children(root_handle).unwrap().collect::<Vec<_>>();
    let mut text_item = None;
    let mut image_item = None;
    for child in direct_children {
        let node = snapshot.node(child).unwrap();
        let HostNodeKind::Element(_) = node.kind() else {
            continue;
        };
        let (_, _, id) = element_identity(&snapshot, child).unwrap();
        if id == options.text_id {
            text_item = Some(child);
        }
        if id == options.image_id {
            image_item = Some(child);
        }
    }
    Fixture {
        document,
        snapshot,
        root: root_handle,
        text_item,
        image_item,
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
            id: "c11-4-generic-mixed-direct-text-image".to_owned(),
            base_url: "https://spinon.invalid/c11-4/generic-mixed.css".to_owned(),
            origin: CssOrigin::Author,
            css: STYLESHEET.to_owned(),
        }],
        VIEWPORT,
        StyleRevision::INITIAL,
    )
    .unwrap()
}

pub(super) fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{label}: expected {expected}, got {actual}"
    );
}

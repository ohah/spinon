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
use super::super::element_identity;

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
  display: grid;
  width: 74px;
  height: auto;
  margin: 0;
  padding: 0;
  column-gap: 0;
  row-gap: 0;
  writing-mode: horizontal-tb;
  direction: ltr;
  align-items: start;
  grid-template-columns: 50px 24px;
}
.root.min { grid-template-rows: min-content; }
.root.max { grid-template-rows: max-content; }
.root.column-control.min { grid-template-columns: min-content; grid-template-rows: 24px 24px; }
.root.column-control.max { grid-template-columns: max-content; grid-template-rows: 24px 24px; }
.root.bad-track { grid-template-rows: min-content 10px; }
.root.bad-columns { grid-template-columns: 51px 24px; }
.root.bad-root-edge { padding-left: 1px; }
.root.bad-align-content { align-content: start; }
.root.bad-justify-items { justify-items: start; }
.root.bad-box-sizing { box-sizing: border-box; }
.item {
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
.image { grid-column: 2 / span 1; grid-row: 1; }
.root.column-control .image { grid-column: 1 / span 1; grid-row: 2; }
.bad-text-placement { grid-column: 2 / span 1; }
.bad-image-placement { grid-row: 2; }
.bad-item-edge { padding-left: 1px; }
.bad-font { font-family: sans-serif; }
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
    pub(super) column_control: bool,
    pub(super) text: &'static str,
    pub(super) image_src: Option<&'static str>,
    pub(super) bad_track: bool,
    pub(super) bad_columns: bool,
    pub(super) bad_root_edge: bool,
    pub(super) bad_align_content: bool,
    pub(super) bad_justify_items: bool,
    pub(super) bad_box_sizing: bool,
    pub(super) bad_text_placement: bool,
    pub(super) bad_image_placement: bool,
    pub(super) bad_item_edge: bool,
    pub(super) bad_font: bool,
    pub(super) extra_child: bool,
    pub(super) nested_text: bool,
    pub(super) include_text: bool,
    pub(super) include_image: bool,
}

impl Options {
    pub(super) fn novel(track: Track, reversed: bool) -> Self {
        let suffix = match (track, reversed) {
            (Track::MinContent, false) => "min-forward",
            (Track::MinContent, true) => "min-reversed",
            (Track::MaxContent, false) => "max-forward",
            (Track::MaxContent, true) => "max-reversed",
        };
        Self {
            root_id: match suffix {
                "min-forward" => "novel-generic-row-min-forward-root",
                "min-reversed" => "novel-generic-row-min-reversed-root",
                "max-forward" => "novel-generic-row-max-forward-root",
                _ => "novel-generic-row-max-reversed-root",
            },
            text_id: match suffix {
                "min-forward" => "novel-generic-row-min-forward-text",
                "min-reversed" => "novel-generic-row-min-reversed-text",
                "max-forward" => "novel-generic-row-max-forward-text",
                _ => "novel-generic-row-max-reversed-text",
            },
            image_id: match suffix {
                "min-forward" => "novel-generic-row-min-forward-image",
                "min-reversed" => "novel-generic-row-min-reversed-image",
                "max-forward" => "novel-generic-row-max-forward-image",
                _ => "novel-generic-row-max-reversed-image",
            },
            track,
            reversed,
            column_control: false,
            text: "XX XXXXX",
            image_src: Some(IMAGE_URL),
            bad_track: false,
            bad_columns: false,
            bad_root_edge: false,
            bad_align_content: false,
            bad_justify_items: false,
            bad_box_sizing: false,
            bad_text_placement: false,
            bad_image_placement: false,
            bad_item_edge: false,
            bad_font: false,
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
        if self.column_control {
            classes.push("column-control");
        }
        for (condition, class) in [
            (self.bad_track, "bad-track"),
            (self.bad_columns, "bad-columns"),
            (self.bad_root_edge, "bad-root-edge"),
            (self.bad_align_content, "bad-align-content"),
            (self.bad_justify_items, "bad-justify-items"),
            (self.bad_box_sizing, "bad-box-sizing"),
        ] {
            if condition {
                classes.push(class);
            }
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
    let mut text_class = "item text".to_owned();
    if options.bad_text_placement {
        text_class.push_str(" bad-text-placement");
    }
    if options.bad_item_edge {
        text_class.push_str(" bad-item-edge");
    }
    if options.bad_font {
        text_class.push_str(" bad-font");
    }
    let text_child = if options.nested_text {
        NodeSpec::Element {
            tag: "span",
            id: "novel-generic-row-nested-text",
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
    let image_class = if options.bad_image_placement {
        "item image bad-image-placement"
    } else {
        "item image"
    };
    let image = NodeSpec::Element {
        tag: "img",
        id: options.image_id,
        class: image_class.to_owned(),
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
            id: "novel-generic-row-extra-child",
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
        DocumentChangeBatch::new(OwnerId::new(11_489).unwrap(), document.document_revision());
    let root_handle = append_spec(&mut document, &mut batch, HostParent::Root, root);
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root_handle)
            .unwrap();
    let mut text_item = None;
    let mut image_item = None;
    for child in snapshot.children(root_handle).unwrap() {
        if !matches!(
            snapshot.node(child).unwrap().kind(),
            HostNodeKind::Element(_)
        ) {
            continue;
        }
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
            id: "c11-4-generic-mixed-row-axis".to_owned(),
            base_url: "https://spinon.invalid/c11-4/generic-row.css".to_owned(),
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

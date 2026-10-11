use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    CssMediaEnvironment, CssOrigin, CssViewport, StylesheetSource, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets,
    compute_runtime_flex_layout_cascade,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_style_layout};

const HTML: &str = "http://www.w3.org/1999/xhtml";

struct Fixture {
    document: HostDocument,
    root: HostNodeHandle,
    nodes: BTreeMap<&'static str, HostNodeHandle>,
}

impl Fixture {
    fn new(target_style: &str) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let row = document.reserve_node_handle().unwrap();
        let target = document.reserve_node_handle().unwrap();
        let sibling = document.reserve_node_handle().unwrap();
        let nodes = BTreeMap::from([
            ("root", root),
            ("row", row),
            ("target", target),
            ("sibling", sibling),
        ]);
        let owner = OwnerId::new(12_101).unwrap();
        let mut batch = DocumentChangeBatch::new(owner, document.document_revision());
        for (name, handle, parent, style) in [
            (
                "root",
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:100vw;height:100vh;margin:0;padding:0",
            ),
            (
                "row",
                row,
                HostParent::Node(root),
                "display:flex;box-sizing:border-box;width:100px;height:40px;flex-direction:row;align-items:flex-start;justify-content:flex-start;column-gap:0;margin:0;padding:0",
            ),
            ("target", target, HostParent::Node(row), target_style),
            (
                "sibling",
                sibling,
                HostParent::Node(row),
                "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0",
            ),
        ] {
            batch.push(DocumentOperation::CreateElement {
                node: handle,
                namespace: HTML.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node: handle,
                name: AttributeName::new(None, "id").unwrap(),
                value: name.to_owned().into(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node: handle,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.to_owned().into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent,
                node: handle,
                before: None,
            });
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            nodes,
        }
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        self.compute_with_custom_properties(false)
    }

    fn compute_with_custom_properties(
        &self,
        custom_properties: bool,
    ) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let view = StyloDocumentView::new_with_base_url(
            snapshot.clone(),
            self.root,
            true,
            QuirksMode::NoQuirks,
            "https://spinon.invalid/c12-1/position.html",
        )
        .unwrap();
        let viewport = CssViewport {
            width_css_px: 100.0,
            height_css_px: 100.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let computed = if custom_properties {
            compute_runtime_flex_custom_properties_cascade(&view, viewport, StyleRevision::INITIAL)
                .unwrap()
        } else {
            compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::INITIAL).unwrap()
        };
        compute_runtime_style_layout(&snapshot, self.root, computed, viewport)
    }

    fn id(&self, name: &'static str) -> spinon_core::NodeId {
        self.nodes[name].id()
    }
}

#[test]
fn stylo_relative_inset_reaches_layout_without_changing_flow_or_flex_sibling() {
    let fixture = Fixture::new(
        "display:block;box-sizing:border-box;width:20px;height:10px;position:relative;left:12px;top:-3px;margin:0;padding:0",
    );
    let output = fixture.compute().unwrap();
    let target = fixture.id("target");
    let sibling = fixture.id("sibling");

    assert_eq!(
        output
            .computed_styles
            .elements
            .iter()
            .find(|style| style.node_id == target)
            .unwrap()
            .properties["position"],
        "relative"
    );
    assert_eq!(output.layout.flow_frames[&target].x, 0.0);
    assert_eq!(output.layout.flow_frames[&target].y, 0.0);
    assert_eq!(output.layout.frames[&target].x, 12.0);
    assert_eq!(output.layout.frames[&target].y, -3.0);
    assert_eq!(output.layout.flow_frames[&sibling].x, 20.0);
    assert_eq!(output.layout.frames[&sibling].x, 20.0);
}

#[test]
fn static_position_ignores_computed_insets_and_absolute_layout_root_is_rejected() {
    let static_fixture = Fixture::new(
        "display:block;box-sizing:border-box;width:20px;height:10px;position:static;left:12px;top:3px;margin:0;padding:0",
    );
    let static_output = static_fixture.compute().unwrap();
    assert_eq!(
        static_output.layout.frames,
        static_output.layout.flow_frames
    );

    let absolute_child = Fixture::new(
        "display:block;box-sizing:border-box;width:20px;height:10px;position:absolute;left:12px;top:3px;margin:0;padding:0",
    );
    let absolute_output = absolute_child.compute().unwrap();
    assert_eq!(
        absolute_output.layout.flow_frames[&absolute_child.id("target")].x,
        0.0
    );
    assert_eq!(
        absolute_output.layout.flow_frames[&absolute_child.id("target")].y,
        0.0
    );
    assert_eq!(
        absolute_output.layout.frames[&absolute_child.id("target")].x,
        12.0
    );
    assert_eq!(
        absolute_output.layout.frames[&absolute_child.id("target")].y,
        3.0
    );
    assert_eq!(
        absolute_output.layout.frames[&absolute_child.id("sibling")].x,
        0.0
    );

    for position in ["fixed", "sticky"] {
        let positioned_fixture = Fixture::new(&format!(
            "display:block;box-sizing:border-box;width:20px;height:10px;position:{position};left:12px;top:3px;margin:0;padding:0"
        ));
        assert!(
            matches!(
                positioned_fixture.compute(),
                Err(StyleLayoutError::UnsupportedComputedValue {
                    property: "position",
                    value,
                    ..
                }) if value == position
            ),
            "position:{position}는 layout 성공 경로에서 거부되어야 합니다"
        );
    }
}

#[test]
fn relative_inset_shorthand_and_percentage_reach_typed_layout_input() {
    let fixture = Fixture::new(
        "display:block;box-sizing:border-box;width:20px;height:10px;position:relative;inset:25% 10%;margin:0;padding:0",
    );
    let output = fixture.compute().unwrap();
    let target = fixture.id("target");

    assert_eq!(output.layout.frames[&target].x, 10.0);
    assert_eq!(output.layout.frames[&target].y, 10.0);
    assert_eq!(output.layout.flow_frames[&target].x, 0.0);
    assert_eq!(output.layout.flow_frames[&target].y, 0.0);
}

#[test]
fn relative_inset_calc_resolves_custom_properties_into_the_layout_result() {
    let fixture = Fixture::new(
        "display:block;box-sizing:border-box;width:20px;height:10px;position:relative;--move-x:4px;left:calc(10% + var(--move-x));top:calc(-1px + 3px);margin:0;padding:0",
    );
    let output = fixture.compute_with_custom_properties(true).unwrap();
    let target = fixture.id("target");

    assert_eq!(output.layout.frames[&target].x, 14.0);
    assert_eq!(output.layout.frames[&target].y, 2.0);
    assert_eq!(output.layout.flow_frames[&target].x, 0.0);
    assert_eq!(output.layout.flow_frames[&target].y, 0.0);
}

#[test]
fn runtime_author_layer_cascade_selects_position_and_inset_before_layout() {
    let fixture = Fixture::new(
        "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0",
    );
    let snapshot = fixture.document.snapshot();
    let view = StyloDocumentView::new_with_base_url(
        snapshot.clone(),
        fixture.root,
        true,
        QuirksMode::NoQuirks,
        "https://spinon.invalid/c12-1/layers.html",
    )
    .unwrap();
    let viewport = CssViewport {
        width_css_px: 100.0,
        height_css_px: 100.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: CssMediaEnvironment::MOBILE,
    };
    let stylesheet = StylesheetSource {
        id: "c12-position-layers".to_owned(),
        base_url: "https://spinon.invalid/c12-1/layers.css".to_owned(),
        origin: CssOrigin::Author,
        css: "@layer base, adjust; @layer base { #target { position:relative; left:4px; } } @layer adjust { #target { left:11px; } }".to_owned(),
    };
    let computed = compute_runtime_flex_custom_properties_paint_cascade_with_stylesheets(
        &view,
        &[stylesheet],
        viewport,
        StyleRevision::INITIAL,
    )
    .unwrap();
    assert_eq!(
        computed
            .elements
            .iter()
            .find(|style| style.node_id == fixture.id("target"))
            .unwrap()
            .properties["left"],
        "11px"
    );

    let output = compute_runtime_style_layout(&snapshot, fixture.root, computed, viewport).unwrap();
    let target = fixture.id("target");
    assert_eq!(output.layout.flow_frames[&target].x, 0.0);
    assert_eq!(output.layout.frames[&target].x, 11.0);
}

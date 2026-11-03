use std::collections::BTreeMap;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_style::{
    CssMediaEnvironment, CssViewport, StyloDocumentView,
    compute_runtime_root_html_text_paint_cascade_with_stylesheets,
};
use style::context::QuirksMode;

use crate::{StyleLayoutError, compute_runtime_style_layout_with_root_html_grid_text_profiles};

const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

struct GridFixture {
    document: HostDocument,
    root: HostNodeHandle,
    items: Vec<HostNodeHandle>,
}

struct GridSpec<'a> {
    width: f32,
    height: f32,
    columns: &'a str,
    rows: &'a str,
    alignment: &'a str,
}

struct ItemSpec<'a> {
    width: f32,
    height: f32,
    alignment: &'a str,
    count: usize,
}

impl GridFixture {
    fn new(grid_spec: GridSpec<'_>, item_spec: ItemSpec<'_>) -> Self {
        let mut document = HostDocument::new().unwrap();
        let root = document.reserve_node_handle().unwrap();
        let grid = document.reserve_node_handle().unwrap();
        let items = (0..item_spec.count)
            .map(|_| document.reserve_node_handle().unwrap())
            .collect::<Vec<_>>();
        let grid_style = format!(
            "display:grid;box-sizing:border-box;width:{}px;height:{}px;grid-template-columns:{};grid-template-rows:{};column-gap:10px;row-gap:10px;align-content:normal;justify-content:normal;overflow:visible;margin:0;padding:0;{}",
            grid_spec.width,
            grid_spec.height,
            grid_spec.columns,
            grid_spec.rows,
            grid_spec.alignment
        );
        let item_style = format!(
            "display:block;box-sizing:border-box;width:{}px;height:{}px;margin:0;padding:0;{}",
            item_spec.width, item_spec.height, item_spec.alignment
        );
        let mut batch =
            DocumentChangeBatch::new(OwnerId::new(11_310).unwrap(), document.document_revision());
        for (node, parent, style) in [
            (
                root,
                HostParent::Root,
                "display:block;box-sizing:border-box;width:300px;height:200px;margin:0;padding:0",
            ),
            (grid, HostParent::Node(root), grid_style.as_str()),
        ]
        .into_iter()
        .chain(
            items
                .iter()
                .copied()
                .map(|node| (node, HostParent::Node(grid), item_style.as_str())),
        ) {
            batch.push(DocumentOperation::CreateElement {
                node,
                namespace: HTML_NAMESPACE.to_owned(),
                local_name: "div".to_owned(),
            });
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, "style").unwrap(),
                value: style.to_owned().into(),
            });
            batch.push(DocumentOperation::InsertBefore {
                parent,
                node,
                before: None,
            });
        }
        document.commit(batch).unwrap();
        Self {
            document,
            root,
            items,
        }
    }

    fn compute(&self) -> Result<crate::StyleLayoutOutput, StyleLayoutError> {
        let snapshot = self.document.snapshot();
        let viewport = CssViewport {
            width_css_px: 300.0,
            height_css_px: 200.0,
            device_scale_factor: 1.0,
            environment_revision: Default::default(),
            media_environment: CssMediaEnvironment::MOBILE,
        };
        let view = StyloDocumentView::new(snapshot.clone(), self.root, true, QuirksMode::NoQuirks)
            .unwrap();
        let computed_styles = compute_runtime_root_html_text_paint_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .map_err(StyleLayoutError::Cascade)?;
        compute_runtime_style_layout_with_root_html_grid_text_profiles(
            &snapshot,
            self.root,
            computed_styles,
            viewport,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &spinon_layout::TextLayoutEngine::default(),
        )
    }

    fn item_frame(&self, output: &crate::StyleLayoutOutput, index: usize) -> [f32; 4] {
        let frame = output.layout.frames[&self.items[index].id()];
        [frame.x, frame.y, frame.width, frame.height]
    }

    fn set_root_style(&mut self, style: &str) {
        let mut batch = DocumentChangeBatch::new(
            OwnerId::new(11_310).unwrap(),
            self.document.document_revision(),
        );
        batch.push(DocumentOperation::SetAttribute {
            node: self.root,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        self.document.commit(batch).unwrap();
    }
}

fn assert_css_px(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "expected {expected}px ± 1/64px, got {actual}px"
    );
}

#[test]
fn root_html_grid_item_alignment_maps_positional_and_place_items_values() {
    let fixture = GridFixture::new(
        GridSpec {
            width: 120.0,
            height: 110.0,
            columns: "40px 60px",
            rows: "30px 50px",
            alignment: "align-items:end;justify-items:center",
        },
        ItemSpec {
            width: 10.0,
            height: 10.0,
            alignment: "",
            count: 4,
        },
    );
    let output = fixture.compute().unwrap();
    for (index, (x, y)) in [(15.0, 20.0), (75.0, 20.0), (15.0, 80.0), (75.0, 80.0)]
        .into_iter()
        .enumerate()
    {
        let frame = fixture.item_frame(&output, index);
        assert_css_px(frame[0], x);
        assert_css_px(frame[1], y);
        assert_eq!([frame[2], frame[3]], [10.0, 10.0]);
    }

    let place_items = GridFixture::new(
        GridSpec {
            width: 40.0,
            height: 30.0,
            columns: "40px",
            rows: "30px",
            alignment: "place-items:center",
        },
        ItemSpec {
            width: 10.0,
            height: 10.0,
            alignment: "",
            count: 1,
        },
    );
    let output = place_items.compute().unwrap();
    let frame = place_items.item_frame(&output, 0);
    assert_css_px(frame[0], 15.0);
    assert_css_px(frame[1], 10.0);
}

#[test]
fn root_html_grid_item_alignment_maps_physical_left_and_right_in_ltr() {
    for (value, expected_x) in [("left", 0.0), ("right", 30.0)] {
        let fixture = GridFixture::new(
            GridSpec {
                width: 40.0,
                height: 30.0,
                columns: "40px",
                rows: "30px",
                alignment: &format!("justify-items:{value}"),
            },
            ItemSpec {
                width: 10.0,
                height: 10.0,
                alignment: "",
                count: 1,
            },
        );
        let output = fixture.compute().unwrap();
        assert_css_px(fixture.item_frame(&output, 0)[0], expected_x);
    }
}

#[test]
fn root_html_grid_item_safe_alignment_falls_back_and_unsafe_alignment_overflows() {
    for (value, expected_x, expected_y) in
        [("safe center", 0.0, 0.0), ("unsafe center", -5.0, -5.0)]
    {
        let fixture = GridFixture::new(
            GridSpec {
                width: 40.0,
                height: 30.0,
                columns: "40px",
                rows: "30px",
                alignment: &format!("align-items:{value};justify-items:{value}"),
            },
            ItemSpec {
                width: 50.0,
                height: 40.0,
                alignment: "",
                count: 1,
            },
        );
        let output = fixture.compute().unwrap();
        let frame = fixture.item_frame(&output, 0);
        assert_css_px(frame[0], expected_x);
        assert_css_px(frame[1], expected_y);
    }
}

#[test]
fn root_html_grid_item_alignment_rejects_baseline_and_out_of_profile_values() {
    for (property, value) in [
        ("align-items", "baseline"),
        ("justify-items", "baseline"),
        ("justify-items", "safe start"),
    ] {
        let fixture = GridFixture::new(
            GridSpec {
                width: 40.0,
                height: 30.0,
                columns: "40px",
                rows: "30px",
                alignment: &format!("{property}:{value}"),
            },
            ItemSpec {
                width: 10.0,
                height: 10.0,
                alignment: "",
                count: 1,
            },
        );
        assert!(
            matches!(
                fixture.compute(),
                Err(StyleLayoutError::UnsupportedComputedValue {
                    property: actual_property,
                    value: actual_value,
                    ..
                }) if actual_property == property && actual_value == value
            ),
            "{property}:{value} should fail closed"
        );
    }

    for property in ["align-items", "justify-items"] {
        let nonfixed = GridFixture::new(
            GridSpec {
                width: 80.0,
                height: 30.0,
                columns: "40px 1fr",
                rows: "30px",
                alignment: &format!("{property}:center"),
            },
            ItemSpec {
                width: 10.0,
                height: 10.0,
                alignment: "",
                count: 1,
            },
        );
        let error = nonfixed.compute().unwrap_err();
        assert!(
            matches!(
                &error,
                StyleLayoutError::UnsupportedComputedValue {
                    property: actual_property,
                    value,
                    ..
                } if *actual_property == property && value == "center"
            ),
            "unexpected ineligible Grid error for {property}: {error:?}"
        );
    }
}

#[test]
fn root_html_grid_item_alignment_keeps_item_self_alignment_rejected() {
    for (property, value) in [("align-self", "end"), ("justify-self", "start")] {
        let fixture = GridFixture::new(
            GridSpec {
                width: 40.0,
                height: 30.0,
                columns: "40px",
                rows: "30px",
                alignment: "align-items:center;justify-items:center",
            },
            ItemSpec {
                width: 10.0,
                height: 10.0,
                alignment: &format!("{property}:{value}"),
                count: 1,
            },
        );
        assert!(
            matches!(
                fixture.compute(),
                Err(StyleLayoutError::UnsupportedComputedValue {
                    property: actual_property,
                    value: actual_value,
                    ..
                }) if actual_property == property && actual_value == value
            ),
            "{property}:{value} should remain rejected"
        );
    }
}

#[test]
fn root_html_grid_item_alignment_is_rejected_outside_an_eligible_grid() {
    for property in ["align-items", "justify-items"] {
        let mut fixture = GridFixture::new(
            GridSpec {
                width: 40.0,
                height: 30.0,
                columns: "40px",
                rows: "30px",
                alignment: "align-items:normal;justify-items:normal",
            },
            ItemSpec {
                width: 10.0,
                height: 10.0,
                alignment: "",
                count: 1,
            },
        );
        fixture.set_root_style(&format!(
            "display:block;box-sizing:border-box;width:300px;height:200px;margin:0;padding:0;{property}:center"
        ));
        assert!(
            matches!(
                fixture.compute(),
                Err(StyleLayoutError::UnsupportedComputedValue {
                    property: actual_property,
                    value,
                    ..
                }) if actual_property == property && value == "center"
            ),
            "{property} on a non-grid root must fail closed"
        );
    }
}

#[test]
fn root_html_grid_self_axis_values_reach_the_grid_item_layout() {
    let fixture = GridFixture::new(
        GridSpec {
            width: 40.0,
            height: 30.0,
            columns: "40px",
            rows: "30px",
            alignment: "align-items:self-end;justify-items:unsafe center",
        },
        ItemSpec {
            width: 10.0,
            height: 10.0,
            alignment: "",
            count: 1,
        },
    );
    let output = fixture.compute().unwrap();
    assert_css_px(fixture.item_frame(&output, 0)[0], 15.0);
    assert_css_px(fixture.item_frame(&output, 0)[1], 20.0);
}

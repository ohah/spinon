use std::collections::BTreeMap;
use std::sync::Arc;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId, StyleRevision,
};
use spinon_layout::PositionedContainingBlockOwner;
use spinon_style::{
    ComputedStyleSnapshot, CssViewport, StyloDocumentView,
    compute_runtime_flex_custom_properties_cascade,
    compute_runtime_flex_custom_properties_paint_cascade, compute_runtime_flex_layout_cascade,
    compute_runtime_flex_paint_cascade,
    compute_runtime_flex_registered_properties_cascade_with_stylesheets,
    compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets,
};

use crate::compute_runtime_style_layout;

const HTML: &str = "http://www.w3.org/1999/xhtml";

fn fixture(nodes: &[(&str, Option<usize>, &str)]) -> (HostDocument, Vec<HostNodeHandle>) {
    let mut document = HostDocument::new().unwrap();
    let handles = nodes
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(10_056).unwrap(), document.document_revision());
    for (index, ((id, parent, style), handle)) in nodes.iter().zip(&handles).enumerate() {
        batch.push(DocumentOperation::CreateElement {
            node: *handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: *handle,
            name: AttributeName::new(None, "id").unwrap(),
            value: (*id).to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: *handle,
            name: AttributeName::new(None, "style").unwrap(),
            value: (*style).to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: parent.map_or(HostParent::Root, |parent| HostParent::Node(handles[parent])),
            node: *handle,
            before: None,
        });
        assert_eq!(
            index,
            handles.iter().position(|item| item == handle).unwrap()
        );
    }
    document.commit(batch).unwrap();
    (document, handles)
}

fn layout(document: &HostDocument, root: HostNodeHandle) -> crate::StyleLayoutOutput {
    layout_result(document, root).unwrap()
}

fn layout_result(
    document: &HostDocument,
    root: HostNodeHandle,
) -> Result<crate::StyleLayoutOutput, crate::StyleLayoutError> {
    let snapshot = Arc::new(document.snapshot());
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::clone(&snapshot), root).unwrap();
    let viewport = CssViewport::C04_FIXTURE;
    let styles =
        compute_runtime_flex_paint_cascade(&view, viewport, StyleRevision::INITIAL).unwrap();
    compute_runtime_style_layout(&snapshot, root, styles, viewport)
}

fn chromium_fixture_layout(
    fixture_case: &serde_json::Value,
    owner_id: u64,
) -> (
    HostDocument,
    Vec<(String, HostNodeHandle)>,
    crate::StyleLayoutOutput,
) {
    let nodes = fixture_case["nodes"].as_array().unwrap();
    let mut document = HostDocument::new().unwrap();
    let handles = nodes
        .iter()
        .map(|_| document.reserve_node_handle().unwrap())
        .collect::<Vec<_>>();
    let by_id = nodes
        .iter()
        .zip(&handles)
        .map(|(node, handle)| (node["id"].as_str().unwrap().to_owned(), *handle))
        .collect::<BTreeMap<_, _>>();
    let mut parents = BTreeMap::new();
    for node in nodes {
        for child in node["children"].as_array().unwrap() {
            parents.insert(
                child.as_str().unwrap().to_owned(),
                node["id"].as_str().unwrap().to_owned(),
            );
        }
    }
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(owner_id).unwrap(),
        document.document_revision(),
    );
    for (node, handle) in nodes.iter().zip(&handles) {
        batch.push(DocumentOperation::CreateElement {
            node: *handle,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        for (name, value) in [
            ("id", node["id"].as_str().unwrap()),
            ("style", node["style"].as_str().unwrap()),
        ] {
            batch.push(DocumentOperation::SetAttribute {
                node: *handle,
                name: AttributeName::new(None, name).unwrap(),
                value: value.to_owned().into(),
            });
        }
        let parent = parents.get(node["id"].as_str().unwrap());
        batch.push(DocumentOperation::InsertBefore {
            parent: parent.map_or(HostParent::Root, |id| HostParent::Node(by_id[id])),
            node: *handle,
            before: None,
        });
    }
    document.commit(batch).unwrap();

    let root = handles[0];
    let snapshot = Arc::new(document.snapshot());
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::clone(&snapshot), root).unwrap();
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 240.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: spinon_style::CssMediaEnvironment::MOBILE,
    };
    let styles =
        compute_runtime_flex_paint_cascade(&view, viewport, StyleRevision::INITIAL).unwrap();
    let output = compute_runtime_style_layout(&snapshot, root, styles, viewport).unwrap();
    let named_handles = by_id.into_iter().collect::<Vec<_>>();
    (document, named_handles, output)
}

#[test]
fn c10_3_5_all_fixture_frames_and_owners_match_pinned_chromium() {
    let inventory: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/css/c10/positioned-flex-inventory.json"
    ))
    .unwrap();
    let reference: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/css/references/c10-3-5-positioned-flex-v1.json"
    ))
    .unwrap();
    assert_eq!(
        reference["oracle"]["cliVersion"],
        "Google Chrome 154.0.8037.98"
    );
    let expected_cases = inventory["cases"].as_array().unwrap();
    let observed_cases = reference["observations"][0]["cases"].as_array().unwrap();
    assert_eq!(observed_cases.len(), expected_cases.len());

    for (case_index, (fixture_case, observed_case)) in
        expected_cases.iter().zip(observed_cases).enumerate()
    {
        assert_eq!(fixture_case["id"], observed_case["id"]);
        let (_document, named_handles, output) =
            chromium_fixture_layout(fixture_case, 10_100 + case_index as u64);
        let handle_by_name = named_handles.into_iter().collect::<BTreeMap<_, _>>();
        let observed_nodes = observed_case["nodes"].as_array().unwrap();
        assert_eq!(
            observed_nodes.len(),
            fixture_case["nodes"].as_array().unwrap().len()
        );

        for node in observed_nodes {
            let name = node["id"].as_str().unwrap();
            let handle = handle_by_name[name];
            let frame = output.layout.frames[&handle.id()];
            for (field, actual) in [
                ("x", frame.x),
                ("y", frame.y),
                ("width", frame.width),
                ("height", frame.height),
            ] {
                let expected = node["rect"][field].as_f64().unwrap() as f32;
                assert!(
                    (actual - expected).abs() <= 0.5,
                    "{}/{} {} Chrome={} Spinon={}",
                    fixture_case["id"].as_str().unwrap(),
                    name,
                    field,
                    expected,
                    actual
                );
            }

            let expected_owner = match node["owner"].as_str().unwrap() {
                "viewport" => PositionedContainingBlockOwner::Viewport,
                "none" => PositionedContainingBlockOwner::NoBox,
                owner_id => PositionedContainingBlockOwner::Node(handle_by_name[owner_id].id()),
            };
            assert_eq!(
                output.layout.positioned_owners[&handle.id()],
                expected_owner,
                "{} owner for {}",
                fixture_case["id"].as_str().unwrap(),
                name
            );
        }

        if let Some(flow_nodes) = observed_case["flowInvariant"].as_array() {
            for flow_node in flow_nodes {
                let name = flow_node["id"].as_str().unwrap();
                let handle = handle_by_name[name];
                let frame = output.layout.frames[&handle.id()];
                for (field, actual) in [
                    ("x", frame.x),
                    ("y", frame.y),
                    ("width", frame.width),
                    ("height", frame.height),
                ] {
                    let expected = flow_node["rect"][field].as_f64().unwrap() as f32;
                    assert!(
                        (actual - expected).abs() <= 0.5,
                        "{}/{} in-flow {} Chrome={} Spinon={}",
                        fixture_case["id"].as_str().unwrap(),
                        name,
                        field,
                        expected,
                        actual
                    );
                }
            }
        }
    }
}

#[test]
fn direct_absolute_flex_child_is_supported_by_all_six_runtime_flex_profiles() {
    let (document, nodes) = fixture(&[
        (
            "root",
            None,
            "display:flex;position:relative;box-sizing:border-box;width:160px;height:80px;flex-direction:row;justify-content:center;align-items:flex-end;margin:0;padding:0",
        ),
        (
            "absolute",
            Some(0),
            "display:block;position:absolute;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0",
        ),
        (
            "flow",
            Some(0),
            "display:block;box-sizing:border-box;width:20px;height:10px;margin:0;padding:0",
        ),
    ]);
    let snapshot = Arc::new(document.snapshot());
    let view =
        StyloDocumentView::new_html_fragment_child_shared(Arc::clone(&snapshot), nodes[0]).unwrap();
    let viewport = CssViewport {
        width_css_px: 320.0,
        height_css_px: 240.0,
        device_scale_factor: 1.0,
        environment_revision: Default::default(),
        media_environment: spinon_style::CssMediaEnvironment::MOBILE,
    };
    let style_snapshots: [ComputedStyleSnapshot; 6] = [
        compute_runtime_flex_layout_cascade(&view, viewport, StyleRevision::INITIAL).unwrap(),
        compute_runtime_flex_custom_properties_cascade(&view, viewport, StyleRevision::INITIAL)
            .unwrap(),
        compute_runtime_flex_registered_properties_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_paint_cascade(&view, viewport, StyleRevision::INITIAL).unwrap(),
        compute_runtime_flex_custom_properties_paint_cascade(
            &view,
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap(),
        compute_runtime_flex_registered_properties_paint_cascade_with_stylesheets(
            &view,
            &[],
            viewport,
            StyleRevision::INITIAL,
        )
        .unwrap(),
    ];

    for styles in style_snapshots {
        let output = compute_runtime_style_layout(&snapshot, nodes[0], styles, viewport).unwrap();
        assert_eq!(
            (
                output.layout.frames[&nodes[1].id()].x,
                output.layout.frames[&nodes[1].id()].y
            ),
            (70.0, 70.0)
        );
        assert_eq!(output.layout.frames[&nodes[2].id()].x, 70.0);
        assert_eq!(
            output.layout.positioned_owners[&nodes[1].id()],
            PositionedContainingBlockOwner::Node(nodes[0].id())
        );
    }
}

#[test]
fn unsupported_flex_static_position_sizes_and_baselines_fail_without_a_partial_output() {
    for (child_style, expected_field) in [
        (
            "display:block;position:absolute;box-sizing:border-box;width:auto;height:10px;margin:0;padding:0",
            "fixed-size",
        ),
        (
            "display:block;position:absolute;box-sizing:border-box;width:20px;height:10px;align-self:first baseline;margin:0;padding:0",
            "baseline",
        ),
    ] {
        let (document, nodes) = fixture(&[
            (
                "root",
                None,
                "display:flex;position:relative;box-sizing:border-box;width:120px;height:80px;flex-direction:row;justify-content:center;align-items:flex-start;margin:0;padding:0",
            ),
            ("absolute", Some(0), child_style),
        ]);
        let result = layout_result(&document, nodes[0]);
        assert!(
            matches!(
                result,
                Err(crate::StyleLayoutError::Layout(
                    spinon_layout::LayoutError::UnsupportedPositioning { node, reason }
                )) if node == nodes[1].id() && reason.contains(expected_field)
            ),
            "unsupported C10.3.5 input {expected_field} must not publish partial frames"
        );
    }
}

#[test]
fn direct_flex_absolute_child_uses_single_item_static_position_without_changing_flow() {
    let (document, nodes) = fixture(&[
        (
            "root",
            None,
            "display:flex;position:relative;box-sizing:border-box;width:180px;height:100px;flex-direction:row;justify-content:center;align-items:flex-end;padding:10px;border:2px solid transparent",
        ),
        (
            "before",
            Some(0),
            "display:block;box-sizing:border-box;width:20px;height:20px;margin:0;padding:0",
        ),
        (
            "absolute",
            Some(0),
            "display:block;position:absolute;order:-7;box-sizing:border-box;width:30px;height:20px;margin:0;padding:0",
        ),
        (
            "after",
            Some(0),
            "display:block;box-sizing:border-box;width:20px;height:20px;margin:0;padding:0",
        ),
    ]);
    let output = layout(&document, nodes[0]);

    assert_eq!(output.layout.frames[&nodes[1].id()].x, 70.0);
    assert_eq!(output.layout.frames[&nodes[1].id()].y, 68.0);
    assert_eq!(output.layout.frames[&nodes[2].id()].x, 75.0);
    assert_eq!(output.layout.frames[&nodes[2].id()].y, 68.0);
    assert_eq!(output.layout.frames[&nodes[3].id()].x, 90.0);
    assert_eq!(output.layout.frames[&nodes[3].id()].y, 68.0);
    assert_eq!(
        output.layout.positioned_owners[&nodes[2].id()],
        PositionedContainingBlockOwner::Node(nodes[0].id())
    );
}

#[test]
fn reversed_and_column_flex_axes_use_their_own_static_alignment_axes() {
    let (row_document, row_nodes) = fixture(&[
        (
            "row-root",
            None,
            "display:flex;position:relative;box-sizing:border-box;width:160px;height:80px;flex-direction:row-reverse;justify-content:flex-end;align-items:center;padding:8px",
        ),
        (
            "row-absolute",
            Some(0),
            "display:block;position:absolute;box-sizing:border-box;width:24px;height:18px;align-self:flex-start;margin:0;padding:0",
        ),
    ]);
    let row = layout(&row_document, row_nodes[0]);
    assert_eq!(row.layout.frames[&row_nodes[1].id()].x, 8.0);
    assert_eq!(row.layout.frames[&row_nodes[1].id()].y, 8.0);

    let (column_document, column_nodes) = fixture(&[
        (
            "column-root",
            None,
            "display:flex;position:relative;box-sizing:border-box;width:120px;height:140px;flex-direction:column;justify-content:center;align-items:flex-end;padding:6px",
        ),
        (
            "before",
            Some(0),
            "display:block;box-sizing:border-box;width:20px;height:16px;margin:0;padding:0",
        ),
        (
            "column-absolute",
            Some(0),
            "display:block;position:absolute;box-sizing:border-box;width:30px;height:20px;align-self:center;margin:0;padding:0",
        ),
        (
            "after",
            Some(0),
            "display:block;box-sizing:border-box;width:20px;height:16px;margin:0;padding:0",
        ),
    ]);
    let column = layout(&column_document, column_nodes[0]);
    assert_eq!(column.layout.frames[&column_nodes[2].id()].x, 45.0);
    assert_eq!(column.layout.frames[&column_nodes[2].id()].y, 60.0);
    assert_eq!(column.layout.frames[&column_nodes[1].id()].y, 54.0);
    assert_eq!(column.layout.frames[&column_nodes[3].id()].y, 70.0);
}

#[path = "c10_3_5_positioned_owner_tests.rs"]
mod owner_tests;

use super::*;

#[derive(Clone, Copy)]
struct Case {
    id: &'static str,
    width: &'static str,
    columns: &'static str,
    placement: &'static str,
    expected_outer_width: f32,
    expected_outer_columns: &'static [f32],
    expected_nested_width: f32,
    expected_contribution_span: u16,
}

const CASES: [Case; 3] = [
    Case {
        id: "nested-grid-span-two",
        width: "max-content",
        columns: "min-content min-content",
        placement: "1 / span 2",
        expected_outer_width: 134.0,
        expected_outer_columns: &[65.0, 65.0],
        expected_nested_width: 134.0,
        expected_contribution_span: 2,
    },
    Case {
        id: "nested-grid-one-track",
        width: "max-content",
        columns: "min-content min-content",
        placement: "1 / 2",
        expected_outer_width: 138.0,
        expected_outer_columns: &[134.0, 0.0],
        expected_nested_width: 134.0,
        expected_contribution_span: 1,
    },
    Case {
        id: "nested-grid-definite-flex-control",
        width: "120px",
        columns: "minmax(auto,1fr) minmax(auto,1fr)",
        placement: "1 / span 2",
        expected_outer_width: 120.0,
        expected_outer_columns: &[58.0, 58.0],
        expected_nested_width: 120.0,
        expected_contribution_span: 2,
    },
];

#[test]
fn c114_nested_grid_items_keep_inner_text_sources_and_parent_contributions() {
    for (index, case) in CASES.iter().enumerate() {
        run_case(case, 11_500 + index as u64);
    }
}

fn run_case(case: &Case, owner: u64) {
    let (snapshot, [root, nested, item_a, item_b]) = fixture(case, owner);
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: if case.width == "120px" { 120.0 } else { 400.0 },
            height_css_px: 200.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };
    let calculation = compute_request_for_block_flex_grid_track_sizing(&request)
        .unwrap_or_else(|error| panic!("{}: calculation failed: {error:?}", case.id));
    let layout = calculation
        .layout
        .unwrap_or_else(|error| panic!("{}: layout failed: {error:?}", case.id));
    let frame = |node: HostNodeHandle| {
        layout
            .frames
            .iter()
            .find(|frame| frame.node_id == node.id().get())
            .unwrap_or_else(|| panic!("{}: missing frame for {}", case.id, node.id().get()))
    };
    assert_close(
        frame(root).width,
        case.expected_outer_width,
        case.id,
        "outer width",
    );
    assert_close(frame(root).height, 20.0, case.id, "outer height");
    assert_close(
        frame(nested).width,
        case.expected_nested_width,
        case.id,
        "nested width",
    );
    assert_close(frame(nested).height, 20.0, case.id, "nested height");
    for (node, expected_width) in [(item_a, 70.0), (item_b, 60.0)] {
        assert_close(
            frame(node).width,
            expected_width,
            case.id,
            "inner text width",
        );
        assert_close(frame(node).height, 20.0, case.id, "inner text height");
    }
    assert_close(
        frame(item_b).x - frame(nested).x,
        74.0,
        case.id,
        "second text item x",
    );

    let diagnostics = layout
        .grid_track_sizing_diagnostics
        .unwrap_or_else(|| panic!("{}: missing track sizing diagnostics", case.id));
    let outer_trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node.get() == root.id().get())
        .unwrap_or_else(|| panic!("{}: missing outer Grid trace", case.id));
    let nested_trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node.get() == nested.id().get())
        .unwrap_or_else(|| panic!("{}: missing nested Grid trace", case.id));
    assert_columns(outer_trace, case.expected_outer_columns, case.id);
    assert_columns(nested_trace, &[70.0, 60.0], case.id);

    let leaf_inputs = diagnostics
        .item_inputs
        .iter()
        .filter(|input| input.grid_container.get() == nested.id().get())
        .collect::<Vec<_>>();
    assert_eq!(leaf_inputs.len(), 2, "{}: inner text inputs", case.id);
    assert!(
        diagnostics
            .item_inputs
            .iter()
            .all(|input| input.node.get() != nested.id().get())
    );
    for (input, expected) in leaf_inputs.iter().zip([(70.0, 100.0), (60.0, 60.0)]) {
        let spinon_layout::LayoutGridItemIntrinsicSource::Text { sizes } = &input.source else {
            panic!("{}: nested leaf input must remain a text source", case.id);
        };
        assert_close(
            sizes.min_content(),
            expected.0,
            case.id,
            "min-content input",
        );
        assert_close(
            sizes.max_content(),
            expected.1,
            case.id,
            "max-content input",
        );
    }

    let columns = outer_trace
        .passes
        .iter()
        .find(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{}: missing outer column pass", case.id));
    if case.id != "nested-grid-definite-flex-control" {
        assert!(
            columns.contributions.iter().any(|contribution| {
                contribution.item.get() == nested.id().get()
                    && contribution.kind
                        == spinon_layout::LayoutGridTrackSizingContributionKind::MinContent
                    && contribution.span == case.expected_contribution_span
                    && (contribution.value - 134.0).abs() <= 1.0 / 64.0
            }),
            "{}: missing nested-item min-content contribution: {:#?}",
            case.id,
            columns.contributions
        );
    } else {
        assert!(
            columns.contributions.iter().any(|contribution| {
                contribution.item.get() == nested.id().get()
                    && contribution.kind
                        == spinon_layout::LayoutGridTrackSizingContributionKind::Minimum
                    && contribution.span == case.expected_contribution_span
                    && contribution.value.abs() <= 1.0 / 64.0
            }),
            "{}: missing nested-item zero minimum contribution: {:#?}",
            case.id,
            columns.contributions
        );
        assert!(
            columns.contributions.iter().all(|contribution| {
                contribution.item.get() != nested.id().get()
                    || (contribution.value >= 0.0 && contribution.value <= 120.0)
            }),
            "{}: nested overflow must not enlarge the parent",
            case.id
        );
    }
}

fn assert_columns(trace: &spinon_layout::LayoutGridTrackSizingTrace, expected: &[f32], case: &str) {
    let columns = trace
        .passes
        .iter()
        .find(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{case}: missing Columns pass"));
    let sizes = columns
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: missing final Columns stage"))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(sizes.len(), expected.len(), "{case}: track count");
    for (actual, expected) in sizes.iter().zip(expected) {
        assert_close(*actual, *expected, case, "column track");
    }
}

fn assert_close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{case}: {property} expected {expected}, got {actual}"
    );
}

fn fixture(case: &Case, owner: u64) -> (Arc<HostDocumentSnapshot>, [HostNodeHandle; 4]) {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let nested = document.reserve_node_handle().unwrap();
    let item_a = document.reserve_node_handle().unwrap();
    let item_b = document.reserve_node_handle().unwrap();
    let text_a = document.reserve_node_handle().unwrap();
    let text_b = document.reserve_node_handle().unwrap();
    let root_id = format!("c114-{}-grid", case.id);
    let nested_id = format!("c114-{}-nested", case.id);
    let item_a_id = format!("c114-{}-item-a", case.id);
    let item_b_id = format!("c114-{}-item-b", case.id);
    let root_style = format!(
        "box-sizing:border-box;display:grid;width:{};height:20px;margin:0;padding:0;border:0;column-gap:4px;row-gap:0;grid-template-columns:{};grid-template-rows:auto;background-color:#e9f1ff",
        case.width, case.columns
    );
    let nested_style = format!(
        "box-sizing:border-box;display:grid;width:auto;height:20px;margin:0;padding:0;border:0;column-gap:4px;row-gap:0;grid-column:{};grid-template-columns:min-content max-content;grid-template-rows:auto;background-color:#e7f6ed",
        case.placement
    );
    let item_style = |column| {
        format!(
            "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:{column};font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#e5484d"
        )
    };
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    for (node, id, style) in [
        (root, root_id, root_style),
        (nested, nested_id, nested_style),
        (item_a, item_a_id, item_style(1)),
        (item_b, item_b_id, item_style(2)),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.into(),
        });
    }
    for (node, data) in [(text_a, "XX XXXXXXX"), (text_b, "XXXXXX")] {
        batch.push(DocumentOperation::CreateText {
            node,
            data: data.into(),
        });
    }
    for (parent, node) in [
        (HostParent::Root, root),
        (HostParent::Node(root), nested),
        (HostParent::Node(nested), item_a),
        (HostParent::Node(nested), item_b),
        (HostParent::Node(item_a), text_a),
        (HostParent::Node(item_b), text_b),
    ] {
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (
        Arc::new(document.snapshot()),
        [root, nested, item_a, item_b],
    )
}

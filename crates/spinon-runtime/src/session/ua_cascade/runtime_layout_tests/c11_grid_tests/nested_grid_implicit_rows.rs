use super::*;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
};

#[derive(Clone, Copy)]
struct Case {
    id: &'static str,
    nested_row_end: &'static str,
    root_rows: [f32; 2],
    root_height: f32,
    outer_span: u16,
}

const CASES: [Case; 2] = [
    Case {
        id: "nested-grid-implicit-row-span-two",
        nested_row_end: "3",
        root_rows: [18.0, 18.0],
        root_height: 40.0,
        outer_span: 2,
    },
    Case {
        id: "nested-grid-implicit-row-one-row",
        nested_row_end: "2",
        root_rows: [40.0, 0.0],
        root_height: 44.0,
        outer_span: 1,
    },
];

#[test]
fn nested_grid_positive_implicit_rows_contribute_to_outer_rows_at_runtime() {
    for (index, case) in CASES.iter().enumerate() {
        run_case(case, 11_770 + index as u64);
    }
}

fn run_case(case: &Case, owner: u64) {
    let (snapshot, [root, nested, explicit, first_implicit, image]) = fixture(case, owner);
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 400.0,
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
    assert_close(frame(root).width, 24.0, case.id, "root width");
    assert_close(frame(root).height, case.root_height, case.id, "root height");
    assert_close(frame(nested).x - frame(root).x, 0.0, case.id, "nested x");
    assert_close(frame(nested).y - frame(root).y, 0.0, case.id, "nested y");
    assert_close(frame(nested).width, 24.0, case.id, "nested width");
    assert_close(
        frame(nested).height,
        40.0,
        case.id,
        "nested contribution height",
    );
    for (node, expected_y, expected_width, expected_height, label) in [
        (explicit, 0.0, 24.0, 8.0, "explicit row item"),
        (first_implicit, 12.0, 4.0, 8.0, "first implicit row item"),
        (image, 24.0, 24.0, 16.0, "second implicit row image"),
    ] {
        assert_close(frame(node).x - frame(nested).x, 0.0, case.id, label);
        assert_close(
            frame(node).y - frame(nested).y,
            expected_y,
            case.id,
            &format!("{label} y"),
        );
        assert_close(
            frame(node).width,
            expected_width,
            case.id,
            &format!("{label} width"),
        );
        assert_close(
            frame(node).height,
            expected_height,
            case.id,
            &format!("{label} height"),
        );
    }

    let diagnostics = layout
        .grid_track_sizing_diagnostics
        .unwrap_or_else(|| panic!("{}: missing track sizing diagnostics", case.id));
    assert_eq!(
        diagnostics.item_inputs.len(),
        1,
        "{}: image input count",
        case.id
    );
    let image_input = &diagnostics.item_inputs[0];
    assert_eq!(image_input.node, image.id());
    assert_eq!(image_input.grid_container, nested.id());
    let LayoutGridItemIntrinsicSource::Replaced { natural } = &image_input.source else {
        panic!("{}: implicit max-content image source", case.id);
    };
    assert_eq!((natural.width, natural.height), (Some(24.0), Some(16.0)));

    let outer_trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node == root.id())
        .unwrap_or_else(|| panic!("{}: missing outer Grid trace", case.id));
    let nested_trace = diagnostics
        .traces
        .iter()
        .find(|trace| trace.node == nested.id())
        .unwrap_or_else(|| panic!("{}: missing nested Grid trace", case.id));
    assert_axis_tracks(outer_trace, &case.root_rows, case.id);
    assert_axis_tracks(nested_trace, &[8.0, 8.0, 16.0], case.id);

    let outer_rows = outer_trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .unwrap_or_else(|| panic!("{}: missing outer Rows pass", case.id));
    assert!(outer_rows.contributions.iter().any(|contribution| {
        contribution.item == nested.id()
            && contribution.kind == LayoutGridTrackSizingContributionKind::MinContent
            && contribution.span == case.outer_span
            && (contribution.value - 40.0).abs() <= 1.0 / 64.0
    }));
}

fn assert_axis_tracks(
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
    expected: &[f32],
    case: &str,
) {
    let rows = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .unwrap_or_else(|| panic!("{case}: missing Rows pass"));
    let tracks = rows
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: missing final Rows stage"))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(tracks, expected, "{case}: Rows tracks");
}

fn fixture(case: &Case, owner: u64) -> (Arc<HostDocumentSnapshot>, [HostNodeHandle; 5]) {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let nested = document.reserve_node_handle().unwrap();
    let explicit = document.reserve_node_handle().unwrap();
    let first_implicit = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let root_style = "box-sizing:border-box;display:grid;width:24px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:4px;grid-template-columns:24px;grid-template-rows:min-content min-content;background-color:#e9f1ff";
    let nested_style = format!(
        "box-sizing:border-box;display:grid;width:24px;height:auto;margin:0;padding:0;border:0;column-gap:0;row-gap:4px;grid-row:1 / {};grid-template-columns:24px;grid-template-rows:min-content;grid-auto-rows:min-content max-content;align-items:start;justify-items:normal;background-color:#e7f6ed",
        case.nested_row_end
    );
    let explicit_style = "box-sizing:border-box;display:block;width:24px;height:8px;margin:0;padding:0;border:0;grid-column:1;grid-row:1 / 2;background-color:#2a6";
    let first_implicit_style = "box-sizing:border-box;display:block;width:4px;height:8px;margin:0;padding:0;border:0;grid-column:1;grid-row:2 / 3;background-color:#e90";
    let image_style = "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:1;grid-row:3 / 4;background-color:#44ddee";
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    for (node, id, tag, parent, style) in [
        (
            root,
            format!("c114-{}-grid", case.id),
            "div",
            HostParent::Root,
            root_style.to_owned(),
        ),
        (
            nested,
            format!("c114-{}-nested", case.id),
            "div",
            HostParent::Node(root),
            nested_style,
        ),
        (
            explicit,
            format!("c114-{}-explicit", case.id),
            "div",
            HostParent::Node(nested),
            explicit_style.to_owned(),
        ),
        (
            first_implicit,
            format!("c114-{}-implicit-min", case.id),
            "div",
            HostParent::Node(nested),
            first_implicit_style.to_owned(),
        ),
        (
            image,
            format!("c114-{}-implicit-max", case.id),
            "img",
            HostParent::Node(nested),
            image_style.to_owned(),
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: tag.to_owned(),
        });
        for (name, value) in [
            ("id", id),
            ("style", style),
            (
                "src",
                if node == image {
                    IMAGE_URL.to_owned()
                } else {
                    String::new()
                },
            ),
        ] {
            if name == "src" && node != image {
                continue;
            }
            batch.push(DocumentOperation::SetAttribute {
                node,
                name: AttributeName::new(None, name).unwrap(),
                value: value.into(),
            });
        }
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (
        Arc::new(document.snapshot()),
        [root, nested, explicit, first_implicit, image],
    )
}

fn assert_close(actual: f32, expected: f32, case: &str, property: &str) {
    assert!(
        (actual - expected).abs() <= 1.0 / 64.0,
        "{case}: {property} expected {expected}, got {actual}"
    );
}

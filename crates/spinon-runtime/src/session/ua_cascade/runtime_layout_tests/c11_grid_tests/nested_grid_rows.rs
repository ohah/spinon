use super::*;

use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument, HostNodeHandle,
    HostParent, OwnerId,
};
use spinon_layout::{LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind};

#[derive(Clone, Copy)]
struct Case {
    id: &'static str,
    height: &'static str,
    rows: &'static str,
    placement: &'static str,
    outer_rows: [f32; 2],
    root_height: f32,
    nested_height: f32,
    contribution_span: u16,
    flex_control: bool,
}

const CASES: [Case; 3] = [
    Case {
        id: "nested-grid-text-image-row-span-two",
        height: "auto",
        rows: "min-content min-content",
        placement: "1 / span 2",
        outer_rows: [18.0, 18.0],
        root_height: 40.0,
        nested_height: 40.0,
        contribution_span: 2,
        flex_control: false,
    },
    Case {
        id: "nested-grid-text-image-row-one-track",
        height: "auto",
        rows: "min-content min-content",
        placement: "1 / 2",
        outer_rows: [40.0, 0.0],
        root_height: 44.0,
        nested_height: 40.0,
        contribution_span: 1,
        flex_control: false,
    },
    Case {
        id: "nested-grid-text-image-row-definite-flex-control",
        height: "60px",
        rows: "minmax(auto,1fr) minmax(auto,1fr)",
        placement: "1 / span 2",
        outer_rows: [28.0, 28.0],
        root_height: 60.0,
        nested_height: 60.0,
        contribution_span: 2,
        flex_control: true,
    },
];

#[test]
fn c114_nested_grid_text_and_image_contribute_to_outer_rows() {
    for (index, case) in CASES.iter().enumerate() {
        run_case(case, 11_730 + index as u64);
    }
}

fn run_case(case: &Case, owner: u64) {
    let (snapshot, [root, nested, text_item, image]) = fixture(case, owner);
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
    assert_close(frame(root).width, 98.0, case.id, "outer width");
    assert_close(
        frame(root).height,
        case.root_height,
        case.id,
        "outer height",
    );
    assert_close(frame(nested).x - frame(root).x, 0.0, case.id, "nested x");
    assert_close(frame(nested).y - frame(root).y, 0.0, case.id, "nested y");
    assert_close(frame(nested).width, 98.0, case.id, "nested width");
    assert_close(
        frame(nested).height,
        case.nested_height,
        case.id,
        "nested height",
    );
    assert_close(frame(text_item).x - frame(nested).x, 0.0, case.id, "text x");
    assert_close(frame(text_item).y - frame(nested).y, 0.0, case.id, "text y");
    assert_close(frame(text_item).width, 70.0, case.id, "text width");
    assert_close(
        frame(text_item).height,
        20.0,
        case.id,
        "text wrapped height",
    );
    assert_close(frame(image).x - frame(nested).x, 74.0, case.id, "image x");
    assert_close(frame(image).y - frame(nested).y, 24.0, case.id, "image y");
    assert_close(frame(image).width, 24.0, case.id, "image natural width");
    assert_close(frame(image).height, 16.0, case.id, "image natural height");

    let diagnostics = layout
        .grid_track_sizing_diagnostics
        .unwrap_or_else(|| panic!("{}: missing track sizing diagnostics", case.id));
    assert_eq!(
        diagnostics.item_inputs.len(),
        2,
        "{}: item input count",
        case.id
    );
    assert!(
        diagnostics
            .item_inputs
            .iter()
            .all(|input| input.revision == diagnostics.revision),
        "{}: text and image revisions differ",
        case.id
    );
    assert_eq!(
        diagnostics
            .item_inputs
            .iter()
            .map(|input| input.node.get())
            .collect::<Vec<_>>(),
        [text_item.id().get(), image.id().get()],
        "{}: source order",
        case.id
    );
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
    assert_axis_tracks(
        outer_trace,
        LayoutGridTrackSizingAxis::Rows,
        &case.outer_rows,
        case.id,
    );
    assert_axis_tracks(
        nested_trace,
        LayoutGridTrackSizingAxis::Rows,
        &[20.0, 16.0],
        case.id,
    );

    let outer_rows = outer_trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .unwrap_or_else(|| panic!("{}: missing outer Rows pass", case.id));
    if case.flex_control {
        assert!(outer_rows.contributions.iter().any(|contribution| {
            contribution.item == nested.id()
                && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
                && contribution.span == case.contribution_span
                && contribution.value.abs() <= 1.0 / 64.0
        }));
    } else {
        assert!(outer_rows.contributions.iter().any(|contribution| {
            contribution.item == nested.id()
                && contribution.kind == LayoutGridTrackSizingContributionKind::MinContent
                && contribution.span == case.contribution_span
                && (contribution.value - 40.0).abs() <= 1.0 / 64.0
        }));
    }
}

fn assert_axis_tracks(
    trace: &spinon_layout::LayoutGridTrackSizingTrace,
    axis: LayoutGridTrackSizingAxis,
    expected: &[f32],
    case: &str,
) {
    let pass = trace
        .passes
        .iter()
        .find(|pass| pass.axis == axis)
        .unwrap_or_else(|| panic!("{case}: missing {axis:?} pass"));
    let tracks = pass
        .stages
        .last()
        .unwrap_or_else(|| panic!("{case}: missing final {axis:?} stage"))
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(tracks.len(), expected.len(), "{case}: {axis:?} track count");
    for (actual, expected) in tracks.iter().zip(expected) {
        assert_close(*actual, *expected, case, "row track");
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
    const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let nested = document.reserve_node_handle().unwrap();
    let text_item = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let root_style = format!(
        "box-sizing:border-box;display:grid;width:98px;height:{};margin:0;padding:0;border:0;column-gap:0;row-gap:4px;grid-template-columns:98px;grid-template-rows:{};background-color:#e9f1ff",
        case.height, case.rows
    );
    let nested_style = format!(
        "box-sizing:border-box;display:grid;width:auto;height:auto;margin:0;padding:0;border:0;column-gap:4px;row-gap:4px;grid-row:{};grid-template-columns:min-content max-content;grid-template-rows:min-content min-content;align-items:start;justify-items:normal;background-color:#e7f6ed",
        case.placement
    );
    const TEXT_STYLE: &str = "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:1;grid-row:1;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#fff0e1";
    const IMAGE_STYLE: &str = "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:2;grid-row:2;background-color:#44ddee";
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    for (node, id, tag, parent, style) in [
        (
            root,
            format!("c114-{}-grid", case.id),
            "div",
            HostParent::Root,
            root_style,
        ),
        (
            nested,
            format!("c114-{}-nested", case.id),
            "div",
            HostParent::Node(root),
            nested_style,
        ),
        (
            text_item,
            format!("c114-{}-text", case.id),
            "div",
            HostParent::Node(nested),
            TEXT_STYLE.to_owned(),
        ),
        (
            image,
            format!("c114-{}-image", case.id),
            "img",
            HostParent::Node(nested),
            IMAGE_STYLE.to_owned(),
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
    batch.push(DocumentOperation::CreateText {
        node: text,
        data: "XX XXXXXXX".into(),
    });
    batch.push(DocumentOperation::InsertBefore {
        parent: HostParent::Node(text_item),
        node: text,
        before: None,
    });
    document.commit(batch).unwrap();
    (
        Arc::new(document.snapshot()),
        [root, nested, text_item, image],
    )
}

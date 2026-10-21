use super::*;

use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
};

#[derive(Clone, Copy)]
struct Case {
    id: &'static str,
    width: &'static str,
    columns: &'static str,
    placement: &'static str,
    outer_width: f32,
    outer_columns: &'static [f32],
    nested_width: f32,
    contribution_span: u16,
    flex_control: bool,
}

const CASES: [Case; 3] = [
    Case {
        id: "nested-grid-text-image-span-two",
        width: "max-content",
        columns: "min-content min-content",
        placement: "1 / span 2",
        outer_width: 98.0,
        outer_columns: &[47.0, 47.0],
        nested_width: 98.0,
        contribution_span: 2,
        flex_control: false,
    },
    Case {
        id: "nested-grid-text-image-one-track",
        width: "max-content",
        columns: "min-content min-content",
        placement: "1 / 2",
        outer_width: 102.0,
        outer_columns: &[98.0, 0.0],
        nested_width: 98.0,
        contribution_span: 1,
        flex_control: false,
    },
    Case {
        id: "nested-grid-text-image-definite-flex-control",
        width: "120px",
        columns: "minmax(auto,1fr) minmax(auto,1fr)",
        placement: "1 / span 2",
        outer_width: 120.0,
        outer_columns: &[58.0, 58.0],
        nested_width: 120.0,
        contribution_span: 2,
        flex_control: true,
    },
];

#[test]
fn c114_nested_grid_text_and_replaced_item_share_intrinsic_layout_revision() {
    for (index, case) in CASES.iter().enumerate() {
        run_case(case, 11_700 + index as u64);
    }
}

fn run_case(case: &Case, owner: u64) {
    let (snapshot, [root, nested, text_item, image]) = fixture(case, owner);
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: if case.flex_control { 120.0 } else { 400.0 },
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
    assert_close(frame(root).width, case.outer_width, case.id, "outer width");
    assert_close(frame(root).height, 20.0, case.id, "outer height");
    assert_close(
        frame(nested).width,
        case.nested_width,
        case.id,
        "nested width",
    );
    assert_close(frame(nested).height, 20.0, case.id, "nested height");
    assert_close(frame(text_item).x - frame(nested).x, 0.0, case.id, "text x");
    assert_close(
        frame(text_item).width,
        70.0,
        case.id,
        "text min-content width",
    );
    assert_close(frame(text_item).height, 20.0, case.id, "text height");
    assert_close(frame(image).x - frame(nested).x, 74.0, case.id, "image x");
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
            .all(|input| input.revision == diagnostics.revision)
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
    assert!(
        diagnostics
            .item_inputs
            .iter()
            .all(|input| input.grid_container == nested.id())
    );
    assert!(
        diagnostics
            .item_inputs
            .iter()
            .all(|input| input.node != nested.id())
    );
    let text_input = &diagnostics.item_inputs[0];
    let LayoutGridItemIntrinsicSource::Text { sizes } = &text_input.source else {
        panic!(
            "{}: first nested item must be the Ahem text source",
            case.id
        );
    };
    assert_close(sizes.min_content(), 70.0, case.id, "text min-content input");
    assert_close(
        sizes.max_content(),
        100.0,
        case.id,
        "text max-content input",
    );
    let image_input = &diagnostics.item_inputs[1];
    let LayoutGridItemIntrinsicSource::Replaced { natural } = &image_input.source else {
        panic!(
            "{}: second nested item must be the replaced image source",
            case.id
        );
    };
    assert_eq!(
        (natural.width, natural.height),
        (Some(24.0), Some(16.0)),
        "{}: image natural size",
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
    assert_columns(outer_trace, case.outer_columns, case.id);
    assert_columns(nested_trace, &[70.0, 24.0], case.id);
    let outer_columns = outer_trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
        .unwrap_or_else(|| panic!("{}: missing outer Columns pass", case.id));
    if case.flex_control {
        assert!(outer_columns.contributions.iter().any(|contribution| {
            contribution.item == nested.id()
                && contribution.kind == LayoutGridTrackSizingContributionKind::Minimum
                && contribution.span == case.contribution_span
                && contribution.value.abs() <= 1.0 / 64.0
        }));
        assert!(outer_columns.contributions.iter().all(|contribution| {
            contribution.item != nested.id()
                || (contribution.value >= 0.0 && contribution.value <= 120.0)
        }));
    } else {
        assert!(outer_columns.contributions.iter().any(|contribution| {
            contribution.item == nested.id()
                && contribution.kind == LayoutGridTrackSizingContributionKind::MinContent
                && contribution.span == case.contribution_span
                && (contribution.value - 98.0).abs() <= 1.0 / 64.0
        }));
    }
}

fn assert_columns(trace: &spinon_layout::LayoutGridTrackSizingTrace, expected: &[f32], case: &str) {
    let columns = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
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
    const IMAGE_URL: &str = "https://spinon.invalid/c14-1-png-basic.png";
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let nested = document.reserve_node_handle().unwrap();
    let text_item = document.reserve_node_handle().unwrap();
    let image = document.reserve_node_handle().unwrap();
    let text = document.reserve_node_handle().unwrap();
    let root_style = format!(
        "box-sizing:border-box;display:grid;width:{};height:20px;margin:0;padding:0;border:0;column-gap:4px;row-gap:0;grid-template-columns:{};grid-template-rows:auto;background-color:#e9f1ff",
        case.width, case.columns
    );
    let nested_style = format!(
        "box-sizing:border-box;display:grid;width:auto;height:20px;margin:0;padding:0;border:0;column-gap:4px;row-gap:0;grid-column:{};grid-template-columns:min-content max-content;grid-template-rows:auto;align-items:start;justify-items:normal;background-color:#e7f6ed",
        case.placement
    );
    const TEXT_STYLE: &str = "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:1;font-family:Ahem;font-size:10px;font-weight:400;font-style:normal;line-height:10px;white-space-collapse:collapse;text-wrap-mode:wrap;word-break:normal;direction:ltr;line-break:auto;text-align:start;overflow-wrap:normal;text-transform:none;letter-spacing:normal;word-spacing:0px;text-indent:0px;background-color:#fff0e1";
    const IMAGE_STYLE: &str = "box-sizing:border-box;display:block;width:auto;height:auto;margin:0;padding:0;border:0;grid-column:2;background-color:#44ddee";
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

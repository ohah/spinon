use super::*;

use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, HostDocument};
use spinon_layout::{LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind};

const ROOT_STYLE: &str = "box-sizing:border-box;display:grid;width:60px;height:300px;margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:normal;grid-auto-flow:row;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content(40px);background-color:#17212b";
const ITEM_STYLE: &str = "box-sizing:border-box;display:block;width:auto;height:auto;min-height:0;grid-row:span 2;margin:0;padding:0;border:0;background-color:#e5484d";
const CHILD_STYLE: &str = "box-sizing:border-box;display:block;width:60px;height:200px;margin:0;padding:0;border:0;background-color:#ffd166";

#[test]
fn c114_fit_content_span_two_caps_rows_and_keeps_child_overflow_visible() {
    let (snapshot, [root, item, child]) = fixture(ITEM_STYLE, 11_444);
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 60.0,
            height_css_px: 300.0,
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
        .expect("exact fit-content spanning style is accepted");
    let layout = calculation
        .layout
        .expect("fit-content span layout completes");
    let frame = |node: HostNodeHandle| {
        layout
            .frames
            .iter()
            .find(|frame| frame.node_id == node.id().get())
            .unwrap_or_else(|| panic!("missing frame for node {}", node.id().get()))
    };
    assert_eq!(
        (
            frame(root).x,
            frame(root).y,
            frame(root).width,
            frame(root).height
        ),
        (0.0, 0.0, 60.0, 300.0)
    );
    assert_eq!(
        (
            frame(item).x,
            frame(item).y,
            frame(item).width,
            frame(item).height
        ),
        (0.0, 0.0, 60.0, 80.0)
    );
    assert_eq!(
        (
            frame(child).x,
            frame(child).y,
            frame(child).width,
            frame(child).height
        ),
        (0.0, 0.0, 60.0, 200.0)
    );

    let diagnostics = layout
        .grid_track_sizing_diagnostics
        .expect("opt-in track sizing diagnostics are present");
    let observation = diagnostics
        .fit_content_spanning
        .as_ref()
        .expect("scene observation is attached to the same layout revision");
    assert_eq!(observation.grid_root, root.id());
    assert_eq!(observation.node_ids, [root.id(), item.id(), child.id()]);
    assert_eq!(diagnostics.traces.len(), 1);
    let trace = &diagnostics.traces[0];
    let rows = trace
        .passes
        .iter()
        .find(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
        .expect("row sizing pass is present");
    assert!(rows.contributions.iter().any(|contribution| {
        contribution.item == item.id()
            && contribution.span == 2
            && contribution.kind == LayoutGridTrackSizingContributionKind::MaxContent
            && contribution.value == 200.0
    }));
    assert!(rows.stages.last().is_some_and(|stage| {
        stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>()
            == [40.0, 40.0]
    }));
}

#[test]
fn c114_fit_content_spanning_rejects_a_mutated_row_span() {
    let (snapshot, _) = fixture(
        "box-sizing:border-box;display:block;width:auto;height:auto;min-height:0;margin:0;padding:0;border:0;background-color:#e5484d",
        11_445,
    );
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 60.0,
            height_css_px: 300.0,
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
        .expect("mutation request reaches layout validation");
    assert!(calculation.layout.is_err());
}

#[test]
fn c114_fit_content_spanning_rejects_scene_mutations() {
    let mutations = [
        (
            "root width",
            ROOT_STYLE.replace("width:60px", "width:61px"),
            ITEM_STYLE.to_owned(),
            CHILD_STYLE.to_owned(),
            false,
        ),
        (
            "root height",
            ROOT_STYLE.replace("height:300px", "height:301px"),
            ITEM_STYLE.to_owned(),
            CHILD_STYLE.to_owned(),
            false,
        ),
        (
            "fit-content row limit",
            ROOT_STYLE.replace("fit-content(40px)", "fit-content(41px)"),
            ITEM_STYLE.to_owned(),
            CHILD_STYLE.to_owned(),
            false,
        ),
        (
            "row gap",
            ROOT_STYLE.replace("row-gap:0", "row-gap:1px"),
            ITEM_STYLE.to_owned(),
            CHILD_STYLE.to_owned(),
            false,
        ),
        (
            "item minimum height",
            ROOT_STYLE.to_owned(),
            ITEM_STYLE.replace("min-height:0", "min-height:1px"),
            CHILD_STYLE.to_owned(),
            false,
        ),
        (
            "explicit visible overflow declaration",
            ROOT_STYLE.to_owned(),
            ITEM_STYLE.replace("min-height:0", "min-height:0;overflow:visible"),
            CHILD_STYLE.to_owned(),
            false,
        ),
        (
            "child height",
            ROOT_STYLE.to_owned(),
            ITEM_STYLE.to_owned(),
            CHILD_STYLE.replace("height:200px", "height:201px"),
            false,
        ),
        (
            "vertical writing mode",
            ROOT_STYLE.replace("display:grid", "display:grid;writing-mode:vertical-rl"),
            ITEM_STYLE.to_owned(),
            CHILD_STYLE.to_owned(),
            false,
        ),
        (
            "extra direct item",
            ROOT_STYLE.to_owned(),
            ITEM_STYLE.to_owned(),
            CHILD_STYLE.to_owned(),
            true,
        ),
    ];

    for (index, (name, root_style, item_style, child_style, extra_item)) in
        mutations.into_iter().enumerate()
    {
        let (snapshot, _) = fixture_with_styles(
            &root_style,
            &item_style,
            &child_style,
            extra_item,
            11_446 + index as u64,
        );
        assert!(
            rejected(snapshot),
            "scene mutation must fail closed: {name}"
        );
    }
}

fn fixture(item_style: &str, owner: u64) -> (Arc<HostDocumentSnapshot>, [HostNodeHandle; 3]) {
    fixture_with_styles(ROOT_STYLE, item_style, CHILD_STYLE, false, owner)
}

fn fixture_with_styles(
    root_style: &str,
    item_style: &str,
    child_style: &str,
    extra_direct_item: bool,
    owner: u64,
) -> (Arc<HostDocumentSnapshot>, [HostNodeHandle; 3]) {
    const HTML: &str = "http://www.w3.org/1999/xhtml";
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(owner).unwrap(), document.document_revision());
    for (node, id, parent, style) in [
        (
            root,
            "c114-fit-content-spanning-grid",
            HostParent::Root,
            root_style,
        ),
        (
            item,
            "c114-fit-content-spanning-item",
            HostParent::Node(root),
            item_style,
        ),
        (
            child,
            "c114-fit-content-spanning-child",
            HostParent::Node(item),
            child_style,
        ),
    ] {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "id").unwrap(),
            value: id.to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "style").unwrap(),
            value: style.to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node,
            name: AttributeName::new(None, "data-node").unwrap(),
            value: "".to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    if extra_direct_item {
        let extra = document.reserve_node_handle().unwrap();
        batch.push(DocumentOperation::CreateElement {
            node: extra,
            namespace: HTML.to_owned(),
            local_name: "div".to_owned(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: extra,
            name: AttributeName::new(None, "id").unwrap(),
            value: "c114-fit-content-spanning-extra-item".to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: extra,
            name: AttributeName::new(None, "style").unwrap(),
            value: "display:block;width:60px;height:1px".to_owned().into(),
        });
        batch.push(DocumentOperation::SetAttribute {
            node: extra,
            name: AttributeName::new(None, "data-node").unwrap(),
            value: "".to_owned().into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(root),
            node: extra,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    (Arc::new(document.snapshot()), [root, item, child])
}

fn rejected(snapshot: Arc<HostDocumentSnapshot>) -> bool {
    let request = WorkRequest {
        key: key_for(&snapshot, EnvironmentRevision::INITIAL),
        snapshot,
        viewport: CssViewport {
            width_css_px: 60.0,
            height_css_px: 300.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::MOBILE,
        },
        previous_snapshot: None,
        force_full: true,
        invalidation: None,
        previous_styles: None,
    };
    match compute_request_for_block_flex_grid_track_sizing(&request) {
        Err(_) => true,
        Ok(calculation) => calculation.layout.is_err(),
    }
}

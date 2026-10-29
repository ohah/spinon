use super::{
    RuntimeUaCascadeKey, TRACK_LIMIT, prepare, revisions_match, validate_grid_input,
    validate_rows_pass,
};
use spinon_core::{
    AttributeName, DocumentChangeBatch, DocumentOperation, EnvironmentRevision, HostDocument,
    HostParent, NodeId, OwnerId, StyleRevision,
};
use spinon_layout::{
    GridAutoColumns, GridAutoFlow, GridAutoRows, GridContainerInput, GridPlacementMode,
    GridTrackLengthPercentage, GridTrackSize, LayoutGridTrackSizingAxis,
    LayoutGridTrackSizingContributionKind, LayoutGridTrackSizingContributionTrace,
    LayoutGridTrackSizingMinimumContributionSource, LayoutGridTrackSizingPassTrace,
    LayoutGridTrackSizingStageKind, LayoutGridTrackSizingStageTrace,
    LayoutGridTrackSizingTrackState, LayoutInputRevision, LayoutSourceRevision,
};
use spinon_style::{CssMediaEnvironment, CssViewport, StyloDocumentView};
use std::sync::Arc;

fn expected_grid_input() -> GridContainerInput {
    GridContainerInput {
        columns: vec![GridTrackSize::LengthPx(120.0)],
        columns_line_names: vec![vec![], vec![]],
        auto_columns: GridAutoColumns::Auto,
        auto_rows: GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
            GridTrackLengthPercentage::LengthPx(TRACK_LIMIT),
        )]),
        auto_flow: GridAutoFlow::Row,
        placement_mode: GridPlacementMode::TrackSizing,
        ..GridContainerInput::default()
    }
}

fn expected_rows_pass(item: NodeId) -> LayoutGridTrackSizingPassTrace {
    let stages = [
        (LayoutGridTrackSizingStageKind::Initialize, 0.0),
        (
            LayoutGridTrackSizingStageKind::IntrinsicItemBatch {
                span: 1,
                crosses_flexible_track: false,
            },
            0.0,
        ),
        (LayoutGridTrackSizingStageKind::IntrinsicSizingComplete, 0.0),
        (LayoutGridTrackSizingStageKind::MaximizeTracks, TRACK_LIMIT),
        (
            LayoutGridTrackSizingStageKind::ExpandFlexibleTracks,
            TRACK_LIMIT,
        ),
        (
            LayoutGridTrackSizingStageKind::StretchAutoTracks { applied: true },
            TRACK_LIMIT,
        ),
    ]
    .into_iter()
    .map(|(stage, base)| LayoutGridTrackSizingStageTrace {
        stage,
        tracks: vec![
            LayoutGridTrackSizingTrackState {
                internal_index: 0,
                is_gutter: true,
                is_collapsed: true,
                base_size: 0.0,
                growth_limit: Some(0.0),
                infinitely_growable: false,
            },
            LayoutGridTrackSizingTrackState {
                internal_index: 1,
                is_gutter: false,
                is_collapsed: false,
                base_size: base,
                growth_limit: None,
                infinitely_growable: false,
            },
            LayoutGridTrackSizingTrackState {
                internal_index: 2,
                is_gutter: true,
                is_collapsed: true,
                base_size: 0.0,
                growth_limit: Some(0.0),
                infinitely_growable: false,
            },
        ],
    })
    .collect();
    LayoutGridTrackSizingPassTrace {
        sequence: 1,
        axis: LayoutGridTrackSizingAxis::Rows,
        stages,
        contributions: vec![
            LayoutGridTrackSizingContributionTrace {
                item,
                kind: LayoutGridTrackSizingContributionKind::Minimum,
                span: 1,
                crosses_flexible_track: false,
                value: 0.0,
                minimum_source: Some(
                    LayoutGridTrackSizingMinimumContributionSource::SpecifiedMinimum,
                ),
            },
            LayoutGridTrackSizingContributionTrace {
                item,
                kind: LayoutGridTrackSizingContributionKind::MinContent,
                span: 1,
                crosses_flexible_track: false,
                value: 800.0,
                minimum_source: None,
            },
            LayoutGridTrackSizingContributionTrace {
                item,
                kind: LayoutGridTrackSizingContributionKind::MaxContent,
                span: 1,
                crosses_flexible_track: false,
                value: 800.0,
                minimum_source: None,
            },
        ],
        fixture_phase: None,
        fixture_auto_repeat_count: None,
        auto_repeat_count: None,
        fixture_percentage_basis: None,
    }
}

fn prepare_fixture_with_zooms(
    root_zoom: &str,
    item_zoom: &str,
    child_zoom: &str,
) -> Result<Option<[NodeId; 3]>, super::RuntimeLayoutFailure> {
    const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
    let mut document = HostDocument::new().unwrap();
    let root = document.reserve_node_handle().unwrap();
    let item = document.reserve_node_handle().unwrap();
    let child = document.reserve_node_handle().unwrap();
    let nodes = [
        (
            root,
            HostParent::Root,
            "c114-fit-content-zoomed-grid",
            format!(
                "box-sizing:border-box;display:grid;width:60px;height:300px;zoom:{root_zoom};margin:0;padding:0;border:0;column-gap:0;row-gap:0;align-content:normal;grid-auto-flow:row;grid-template-columns:60px;grid-template-rows:none;grid-auto-rows:fit-content(40px)"
            ),
        ),
        (
            item,
            HostParent::Node(root),
            "c114-fit-content-zoomed-grid-item",
            format!(
                "box-sizing:border-box;display:block;width:auto;height:auto;min-height:0;zoom:{item_zoom};margin:0;padding:0;border:0"
            ),
        ),
        (
            child,
            HostParent::Node(item),
            "c114-fit-content-zoomed-grid-child",
            format!(
                "box-sizing:border-box;display:block;width:auto;height:400px;zoom:{child_zoom};margin:0;padding:0;border:0"
            ),
        ),
    ];
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(6_114_002).unwrap(),
        document.document_revision(),
    );
    for (node, parent, id, style) in nodes {
        batch.push(DocumentOperation::CreateElement {
            node,
            namespace: HTML_NAMESPACE.to_owned(),
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
            value: style.into(),
        });
        batch.push(DocumentOperation::InsertBefore {
            parent,
            node,
            before: None,
        });
    }
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view =
        StyloDocumentView::new_c11_4_fit_content_zoom_fixture_test_html_fragment_child_shared(
            Arc::new(snapshot.clone()),
            root,
        )
        .unwrap();
    let styles = spinon_style::compute_runtime_block_flex_grid_track_sizing_zoom_fixture_test_cascade_with_stylesheets(
        &view,
        &[],
        CssViewport {
            width_css_px: 320.0,
            height_css_px: 800.0,
            device_scale_factor: 1.0,
            environment_revision: EnvironmentRevision::INITIAL,
            media_environment: CssMediaEnvironment::DESKTOP,
        },
        StyleRevision::INITIAL,
    )
    .unwrap();
    prepare(&snapshot, root, &styles)
}

#[test]
fn accepts_only_the_zoom_scaled_grid_input() {
    let root = NodeId::new(1).unwrap();
    assert!(validate_grid_input(root, &expected_grid_input()).is_ok());

    let mut unscaled_limit = expected_grid_input();
    unscaled_limit.auto_rows = GridAutoRows::TrackList(vec![GridTrackSize::FitContent(
        GridTrackLengthPercentage::LengthPx(40.0),
    )]);
    assert!(validate_grid_input(root, &unscaled_limit).is_err());

    let mut extra_explicit_row = expected_grid_input();
    extra_explicit_row.rows.push(GridTrackSize::LengthPx(40.0));
    assert!(validate_grid_input(root, &extra_explicit_row).is_err());

    let mut changed_width = expected_grid_input();
    changed_width.columns[0] = GridTrackSize::LengthPx(60.0);
    assert!(validate_grid_input(root, &changed_width).is_err());
}

#[test]
fn accepts_only_the_pinned_zoomed_row_trace_and_contributions() {
    let item = NodeId::new(2).unwrap();
    let expected = expected_rows_pass(item);
    assert!(validate_rows_pass(&expected, item).is_ok());

    let mut unscaled_track = expected.clone();
    unscaled_track.stages[3].tracks[1].base_size = 40.0;
    assert!(validate_rows_pass(&unscaled_track, item).is_err());

    let mut wrong_max_content = expected.clone();
    wrong_max_content.contributions[2].value = 400.0;
    assert!(validate_rows_pass(&wrong_max_content, item).is_err());

    let mut wrong_sequence = expected;
    wrong_sequence.sequence = 0;
    assert!(validate_rows_pass(&wrong_sequence, item).is_err());
}

#[test]
fn rejects_other_zoom_values_and_zoom_owners() {
    assert!(prepare_fixture_with_zooms("2", "1", "1").unwrap().is_some());

    for (root, item, child) in [
        ("1", "1", "1"),
        ("1.5", "1", "1"),
        ("3", "1", "1"),
        ("-1", "1", "1"),
        ("1", "2", "1"),
        ("2", "2", "1"),
        ("2", "1", "2"),
    ] {
        assert!(
            prepare_fixture_with_zooms(root, item, child).is_err(),
            "unexpectedly accepted root/item/child zoom {root}/{item}/{child}"
        );
    }
}

#[test]
fn rejects_stale_style_document_tree_and_environment_revisions() {
    let document = HostDocument::new().unwrap();
    let snapshot = document.snapshot();
    let style_revision = StyleRevision::INITIAL;
    let environment_revision = EnvironmentRevision::INITIAL;
    let key = RuntimeUaCascadeKey {
        generation: snapshot.generation().get(),
        document_revision: snapshot.document_revision().get(),
        render_tree_revision: snapshot.render_tree_revision().get(),
        style_revision,
        environment_revision: environment_revision.get(),
    };
    let source = LayoutSourceRevision::HostDocument {
        generation: snapshot.generation(),
        document: snapshot.document_revision(),
        render_tree: snapshot.render_tree_revision(),
    };
    let layout_revision = LayoutInputRevision::new(source, style_revision, environment_revision);
    assert!(revisions_match(
        style_revision,
        key.generation,
        key.document_revision,
        key.render_tree_revision,
        layout_revision,
        key,
    ));
    assert!(!revisions_match(
        style_revision,
        key.generation,
        key.document_revision + 1,
        key.render_tree_revision,
        layout_revision,
        key,
    ));
    assert!(!revisions_match(
        style_revision,
        key.generation,
        key.document_revision,
        key.render_tree_revision + 1,
        layout_revision,
        key,
    ));
    assert!(!revisions_match(
        style_revision.checked_next().unwrap(),
        key.generation,
        key.document_revision,
        key.render_tree_revision,
        layout_revision,
        key,
    ));
    let changed_environment_key = RuntimeUaCascadeKey {
        environment_revision: key.environment_revision + 1,
        ..key
    };
    assert!(!revisions_match(
        style_revision,
        key.generation,
        key.document_revision,
        key.render_tree_revision,
        layout_revision,
        changed_environment_key,
    ));
}

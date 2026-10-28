use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, OwnerId};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
};
use spinon_style::ComputedStyleProfile;

use super::fixture::{Options, Order, VIEWPORT, cascade, make_fixture};
use super::{GENERIC_SCENES, SceneGrid, prepare};

#[test]
fn generic_host_ids_route_all_four_mixed_spanning_scenes() {
    for (scene, order) in GENERIC_SCENES.iter().zip([
        Order::TwoForward,
        Order::TwoReversed,
        Order::ThreeForward,
        Order::ThreeReversed,
    ]) {
        let fixture = make_fixture(Options::valid(order));
        let styles = cascade(&fixture.view);
        let input = prepare(&fixture.snapshot, fixture.root, &styles)
            .unwrap_or_else(|error| panic!("{}: unexpected adapter error: {error:?}", scene.id))
            .expect("novel root/item IDs must use the generic spanning route");
        assert_eq!(input.paragraph_styles.len(), scene.items.len() - 1);

        let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
            &fixture.snapshot,
            fixture.root,
            styles,
            VIEWPORT,
            &input.paragraph_styles,
            &input.engine,
            input.intrinsic_sizes,
        )
        .unwrap_or_else(|error| panic!("{}: layout failed: {error:?}", scene.id));

        let root = output.layout.frames[&fixture.root.id()];
        assert_frame(root, [0.0, 0.0, 220.0, 180.0], "root");
        let children = fixture
            .snapshot
            .children(fixture.root)
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(children.len(), scene.items.len());
        for (source_order, (child, item)) in children.iter().zip(scene.items).enumerate() {
            let frame = output.layout.frames[&child.id()];
            let expected = match item.kind {
                super::super::ItemKind::Text("XX XXXXX") if scene.items.len() == 2 => {
                    [0.0, 0.0, 61.0, 20.0]
                }
                super::super::ItemKind::Text("XX XXXXX") => [0.0, 0.0, 116.0 + 1.0 / 3.0, 10.0],
                super::super::ItemKind::Text("XXXXXXXXXXXXXXXXXX") => [0.0, 120.0, 180.0, 10.0],
                super::super::ItemKind::Image { .. } if scene.items.len() == 2 => {
                    [25.0, 60.0, 72.0, 48.0]
                }
                super::super::ItemKind::Image { .. } => [52.0 + 2.0 / 3.0, 60.0, 72.0, 48.0],
                _ => panic!("unexpected fixed mixed-span item"),
            };
            assert_frame(frame, expected, "item");
            let intrinsic = output
                .layout
                .grid_item_intrinsic_inputs
                .iter()
                .find(|candidate| candidate.node == child.id())
                .expect("every direct mixed item has one intrinsic input");
            assert_eq!(intrinsic.source_order, source_order);
            assert_eq!(intrinsic.revision, output.layout.revision);
            match (item.kind, &intrinsic.source) {
                (
                    super::super::ItemKind::Text(text),
                    LayoutGridItemIntrinsicSource::Text { sizes },
                ) => {
                    let inline = if text == "XX XXXXX" { 50.0 } else { 180.0 };
                    assert_close(sizes.min_content(), inline, "text min-content");
                    assert_close(
                        sizes.max_content(),
                        if text == "XX XXXXX" { 80.0 } else { 180.0 },
                        "text max-content",
                    );
                }
                (
                    super::super::ItemKind::Image { .. },
                    LayoutGridItemIntrinsicSource::Replaced { natural },
                ) => {
                    assert_eq!(natural.width, Some(24.0));
                    assert_eq!(natural.height, Some(16.0));
                }
                (_, source) => panic!("unexpected intrinsic source: {source:?}"),
            }
        }
        assert_eq!(
            output.layout.grid_item_intrinsic_inputs.len(),
            scene.items.len()
        );
        assert_eq!(output.layout.revision.intrinsic_size().get(), 1);

        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .expect("generic root trace");
        assert_eq!(trace.passes.len(), 2);
        assert_eq!(trace.passes[0].axis, LayoutGridTrackSizingAxis::Columns);
        assert_eq!(trace.passes[1].axis, LayoutGridTrackSizingAxis::Rows);
        let columns = &trace.passes[0];
        let rows = &trace.passes[1];
        assert_track_sizes(columns, scene.columns, "columns");
        assert_track_sizes(rows, scene.rows, "rows");
        let expected_contributions = scene
            .items
            .iter()
            .map(|item| {
                let (span, value) = match item.kind {
                    super::super::ItemKind::Text("XX XXXXX") => (2, 50.0),
                    super::super::ItemKind::Text("XXXXXXXXXXXXXXXXXX") => (3, 180.0),
                    super::super::ItemKind::Image { .. } => (2, 72.0),
                    _ => unreachable!(),
                };
                (item.id, span, value)
            })
            .collect::<Vec<_>>();
        for (child, (expected_label, span, value)) in children.iter().zip(expected_contributions) {
            let contribution = columns
                .contributions
                .iter()
                .find(|candidate| {
                    candidate.item == child.id()
                        && candidate.kind == LayoutGridTrackSizingContributionKind::MinContent
                        && (candidate.value - value).abs() <= 1.0 / 64.0
                })
                .expect("one column contribution per item");
            assert_eq!(contribution.span, span, "{expected_label}");
            assert_eq!(
                contribution.kind,
                LayoutGridTrackSizingContributionKind::MinContent
            );
            assert!(!contribution.crosses_flexible_track);
            assert_close(contribution.value, value, "min-content contribution");
        }
    }
}

#[test]
fn malformed_spanning_mixed_inputs_never_publish_partial_layout() {
    let mut cases = Vec::new();
    let mut bad_text = Options::valid(Order::ThreeForward);
    bad_text.bad_text = true;
    cases.push(("changed text", bad_text));
    let mut bad_image = Options::valid(Order::TwoForward);
    bad_image.bad_image_source = true;
    cases.push(("wrong image source", bad_image));
    let mut extra = Options::valid(Order::ThreeForward);
    extra.extra_child = true;
    cases.push(("extra child", extra));
    let mut nested = Options::valid(Order::TwoForward);
    nested.nested_text = true;
    cases.push(("nested text", nested));
    let mut track = Options::valid(Order::TwoForward);
    track.bad_track = true;
    cases.push(("wrong track count", track));
    let mut edge = Options::valid(Order::TwoForward);
    edge.bad_edge = true;
    cases.push(("nonzero root edge", edge));
    let mut placement = Options::valid(Order::TwoForward);
    placement.bad_a_placement = true;
    cases.push(("wrong span", placement));
    let mut height = Options::valid(Order::TwoForward);
    height.bad_image_height = true;
    cases.push(("wrong image height", height));

    for (label, options) in cases {
        let fixture = make_fixture(options);
        let styles = cascade(&fixture.view);
        assert!(
            super::super::prepare(&fixture.snapshot, fixture.root, &styles).is_err(),
            "{label} must fail closed before producing a mixed layout"
        );
    }
}

#[test]
fn missing_image_cannot_fall_back_to_a_partial_mixed_layout() {
    let mut options = Options::valid(Order::ThreeForward);
    options.omit_image = true;
    let fixture = make_fixture(options);
    let styles = cascade(&fixture.view);
    assert!(
        !matches!(
            super::super::prepare(&fixture.snapshot, fixture.root, &styles),
            Ok(Some(_))
        ),
        "text-only degradation must not be returned as a mixed layout"
    );
}

#[test]
fn stale_revision_and_wrong_profile_fail_closed() {
    let fixture = make_fixture(Options::valid(Order::TwoForward));
    let mut wrong_profile = cascade(&fixture.view);
    wrong_profile.profile = ComputedStyleProfile::RuntimeRootHtmlTextPaintV1;
    assert!(prepare(&fixture.snapshot, fixture.root, &wrong_profile).is_err());

    let mut fixture = make_fixture(Options::valid(Order::TwoForward));
    let styles = cascade(&fixture.view);
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_489).unwrap(),
        fixture.document.document_revision(),
    );
    batch.push(DocumentOperation::SetAttribute {
        node: fixture.root,
        name: AttributeName::new(None, "data-revision-change").unwrap(),
        value: "changed".into(),
    });
    fixture.document.commit(batch).unwrap();
    let changed_snapshot = fixture.document.snapshot();
    assert!(prepare(&changed_snapshot, fixture.root, &styles).is_err());
}

#[test]
fn generic_spanning_scenes_use_novel_root_and_item_dom_ids() {
    for order in [
        Order::TwoForward,
        Order::TwoReversed,
        Order::ThreeForward,
        Order::ThreeReversed,
    ] {
        let fixture = make_fixture(Options::valid(order));
        let root_id = super::super::element_identity(&fixture.snapshot, fixture.root)
            .unwrap()
            .2;
        assert!(root_id.starts_with("generic-host-root-"));
        assert!(!root_id.starts_with("c114-"));
        for child in fixture.snapshot.children(fixture.root).unwrap() {
            let child_id = super::super::element_identity(&fixture.snapshot, child)
                .unwrap()
                .2;
            assert!(child_id.starts_with("generic-host-"));
            assert!(!child_id.starts_with("c114-"));
        }
    }
}

#[test]
fn generic_spanning_scene_contract_keeps_four_source_order_controls() {
    assert_eq!(GENERIC_SCENES.len(), 4);
    assert!(
        GENERIC_SCENES
            .iter()
            .all(|scene| scene.grid == SceneGrid::GenericSpanningMultiTrack)
    );
    assert_eq!(GENERIC_SCENES[0].columns, [25.0, 36.0, 36.0, 0.0]);
    assert_eq!(GENERIC_SCENES[1].columns, GENERIC_SCENES[0].columns);
    assert_eq!(
        GENERIC_SCENES[2].columns,
        [158.0 / 3.0, 191.0 / 3.0, 191.0 / 3.0, 0.0]
    );
    assert_eq!(GENERIC_SCENES[3].columns, GENERIC_SCENES[2].columns);
}

fn assert_track_sizes(
    pass: &spinon_layout::LayoutGridTrackSizingPassTrace,
    expected: &[f32],
    label: &str,
) {
    let tracks = pass
        .stages
        .last()
        .expect("final track sizing stage")
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(tracks.len(), expected.len(), "{label} track count");
    for (actual, expected) in tracks.into_iter().zip(expected) {
        assert_close(actual, *expected, label);
    }
}

fn assert_frame(frame: spinon_layout::LayoutFrame, expected: [f32; 4], label: &str) {
    for (actual, expected) in [frame.x, frame.y, frame.width, frame.height]
        .into_iter()
        .zip(expected)
    {
        assert_close(actual, expected, label);
    }
}

fn assert_close(actual: f32, expected: f32, label: &str) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= 1.0 / 64.0,
        "{label}: expected {expected}, got {actual}"
    );
}

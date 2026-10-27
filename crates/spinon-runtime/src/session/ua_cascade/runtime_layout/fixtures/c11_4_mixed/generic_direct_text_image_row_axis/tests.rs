use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, OwnerId};
use spinon_layout::{
    LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis, LayoutGridTrackSizingContributionKind,
};
use spinon_style::ComputedStyleProfile;

use super::fixture::{Options, Track, VIEWPORT, cascade, close, make_fixture};
use super::prepare;

#[test]
fn novel_host_ids_accept_mixed_row_scenes_in_both_source_orders() {
    for (track, reversed) in [
        (Track::MinContent, false),
        (Track::MinContent, true),
        (Track::MaxContent, false),
        (Track::MaxContent, true),
    ] {
        let fixture = make_fixture(Options::novel(track, reversed));
        let styles = cascade(&fixture.view);
        let input = prepare(&fixture.snapshot, fixture.root, &styles)
            .unwrap_or_else(|error| panic!("unexpected adapter error: {error:?}"))
            .expect("novel HostNode IDs must use the generic row adapter");
        let text_item = fixture.text_item.unwrap();
        let image_item = fixture.image_item.unwrap();
        assert_eq!(input.paragraph_styles.len(), 1);
        let paragraph = &input.paragraph_styles[&text_item.id()];
        assert_eq!(paragraph.font_family, "Ahem");
        assert_eq!(paragraph.font_size, 10.0);
        assert_eq!(paragraph.line_height, 10.0);
        assert_eq!(
            paragraph.direction,
            spinon_layout::TextParagraphDirection::Ltr
        );

        let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
            &fixture.snapshot,
            fixture.root,
            styles,
            VIEWPORT,
            &input.paragraph_styles,
            &input.engine,
            input.intrinsic_sizes,
        )
        .unwrap_or_else(|error| panic!("layout failed: {error:?}"));
        let root_frame = output.layout.frames[&fixture.root.id()];
        assert_eq!((root_frame.x, root_frame.y), (0.0, 0.0));
        close(root_frame.width, 74.0, "root width");
        close(root_frame.height, 20.0, "root height");
        let text_frame = output.layout.frames[&text_item.id()];
        close(text_frame.x, 0.0, "text x");
        close(text_frame.y, 0.0, "text y");
        close(text_frame.width, 50.0, "text width");
        close(text_frame.height, 20.0, "text height");
        let image_frame = output.layout.frames[&image_item.id()];
        close(image_frame.x, 50.0, "image x");
        close(image_frame.y, 0.0, "image y");
        close(image_frame.width, 24.0, "image width");
        close(image_frame.height, 16.0, "image height");

        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .expect("root grid trace");
        assert_eq!(trace.passes.len(), 2);
        assert_eq!(trace.passes[0].axis, LayoutGridTrackSizingAxis::Columns);
        assert_eq!(trace.passes[1].axis, LayoutGridTrackSizingAxis::Rows);
        let columns = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .max_by_key(|pass| pass.sequence)
            .expect("column pass");
        let rows = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Rows)
            .max_by_key(|pass| pass.sequence)
            .expect("row pass");
        assert!(columns.sequence < rows.sequence, "columns precede rows");
        assert_track_sizes(&columns.stages, &[50.0, 24.0], "columns");
        assert_track_sizes(&rows.stages, &[20.0], "rows");
        let expected_contribution_kind = match track {
            Track::MinContent => LayoutGridTrackSizingContributionKind::MinContent,
            Track::MaxContent => LayoutGridTrackSizingContributionKind::MaxContent,
        };
        for (item, expected_value) in [(text_item, 20.0), (image_item, 16.0)] {
            assert!(
                rows.contributions.iter().any(|contribution| {
                    contribution.item == item.id()
                        && contribution.kind == expected_contribution_kind
                        && contribution.span == 1
                        && !contribution.crosses_flexible_track
                        && (contribution.value - expected_value).abs() <= 0.05
                }),
                "row contribution for {item:?}"
            );
        }
        for (item, expected_source_order, expected_kind) in [
            (text_item, if reversed { 1 } else { 0 }, 0),
            (image_item, if reversed { 0 } else { 1 }, 1),
        ] {
            let intrinsic = output
                .layout
                .grid_item_intrinsic_inputs
                .iter()
                .find(|candidate| candidate.node == item.id())
                .expect("mixed intrinsic input");
            assert_eq!(intrinsic.source_order, expected_source_order);
            assert_eq!(intrinsic.revision, output.layout.revision);
            match (&intrinsic.source, expected_kind) {
                (LayoutGridItemIntrinsicSource::Text { .. }, 0) => {}
                (LayoutGridItemIntrinsicSource::Replaced { natural }, 1) => {
                    assert_eq!(natural.width, Some(24.0));
                    assert_eq!(natural.height, Some(16.0));
                }
                (source, _) => panic!("unexpected intrinsic source: {source:?}"),
            }
        }
        assert_eq!(output.layout.grid_item_intrinsic_inputs.len(), 2);
        assert_eq!(output.layout.revision.intrinsic_size().get(), 1);
    }
}

#[test]
fn malformed_row_mixed_inputs_fail_closed() {
    let mut cases = Vec::new();

    let mut changed_text = Options::novel(Track::MinContent, false);
    changed_text.text = "XX XXXX";
    cases.push(("changed text", changed_text));

    let mut wrong_image = Options::novel(Track::MinContent, false);
    wrong_image.image_src = Some("https://spinon.invalid/not-the-fixture.png");
    cases.push(("wrong image source", wrong_image));

    let mut missing_image_source = Options::novel(Track::MinContent, false);
    missing_image_source.image_src = None;
    cases.push(("missing image source", missing_image_source));

    let mut extra_child = Options::novel(Track::MinContent, false);
    extra_child.extra_child = true;
    cases.push(("extra direct child", extra_child));

    let mut nested_text = Options::novel(Track::MinContent, false);
    nested_text.nested_text = true;
    cases.push(("nested text source", nested_text));

    let mut wrong_track = Options::novel(Track::MinContent, false);
    wrong_track.bad_track = true;
    cases.push(("additional row track", wrong_track));

    let mut wrong_columns = Options::novel(Track::MinContent, false);
    wrong_columns.bad_columns = true;
    cases.push(("wrong fixed columns", wrong_columns));

    let mut wrong_root_edge = Options::novel(Track::MinContent, false);
    wrong_root_edge.bad_root_edge = true;
    cases.push(("nonzero root edge", wrong_root_edge));

    let mut wrong_align_content = Options::novel(Track::MinContent, false);
    wrong_align_content.bad_align_content = true;
    cases.push(("authored align-content", wrong_align_content));

    let mut wrong_justify_items = Options::novel(Track::MinContent, false);
    wrong_justify_items.bad_justify_items = true;
    cases.push(("authored justify-items", wrong_justify_items));

    let mut wrong_box_sizing = Options::novel(Track::MinContent, false);
    wrong_box_sizing.bad_box_sizing = true;
    cases.push(("root box-sizing differs", wrong_box_sizing));

    let mut wrong_text_placement = Options::novel(Track::MinContent, false);
    wrong_text_placement.bad_text_placement = true;
    cases.push(("wrong text column", wrong_text_placement));

    let mut wrong_image_placement = Options::novel(Track::MinContent, false);
    wrong_image_placement.bad_image_placement = true;
    cases.push(("wrong image row", wrong_image_placement));

    let mut wrong_item_edge = Options::novel(Track::MinContent, false);
    wrong_item_edge.bad_item_edge = true;
    cases.push(("nonzero item edge", wrong_item_edge));

    let mut wrong_font = Options::novel(Track::MinContent, false);
    wrong_font.bad_font = true;
    cases.push(("font differs", wrong_font));

    for (label, options) in cases {
        let fixture = make_fixture(options);
        let styles = cascade(&fixture.view);
        assert!(
            prepare(&fixture.snapshot, fixture.root, &styles).is_err(),
            "{label} must be rejected"
        );
    }
}

#[test]
fn profile_and_document_revision_mismatches_fail_closed() {
    let fixture = make_fixture(Options::novel(Track::MinContent, false));
    let mut styles = cascade(&fixture.view);
    styles.profile = ComputedStyleProfile::RuntimeRootHtmlTextPaintV1;
    assert!(prepare(&fixture.snapshot, fixture.root, &styles).is_err());

    let mut fixture = make_fixture(Options::novel(Track::MinContent, false));
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
fn non_row_and_non_mixed_inputs_are_not_claimed() {
    let mut text_only = Options::novel(Track::MinContent, false);
    text_only.include_image = false;
    let fixture = make_fixture(text_only);
    let styles = cascade(&fixture.view);
    assert!(
        prepare(&fixture.snapshot, fixture.root, &styles)
            .unwrap()
            .is_none()
    );

    let mut image_only = Options::novel(Track::MinContent, false);
    image_only.include_text = false;
    let fixture = make_fixture(image_only);
    let styles = cascade(&fixture.view);
    assert!(
        prepare(&fixture.snapshot, fixture.root, &styles)
            .unwrap()
            .is_none()
    );

    let mut column_control = Options::novel(Track::MinContent, false);
    column_control.column_control = true;
    let fixture = make_fixture(column_control);
    let styles = cascade(&fixture.view);
    assert!(
        prepare(&fixture.snapshot, fixture.root, &styles)
            .unwrap()
            .is_none()
    );
}

fn assert_track_sizes(
    stages: &[spinon_layout::LayoutGridTrackSizingStageTrace],
    expected: &[f32],
    label: &str,
) {
    let stage = stages.last().expect("final track sizing stage");
    let sizes = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter && !track.is_collapsed)
        .map(|track| track.base_size)
        .collect::<Vec<_>>();
    assert_eq!(sizes.len(), expected.len(), "{label} track count");
    for (actual, expected) in sizes.into_iter().zip(expected) {
        close(actual, *expected, label);
    }
}

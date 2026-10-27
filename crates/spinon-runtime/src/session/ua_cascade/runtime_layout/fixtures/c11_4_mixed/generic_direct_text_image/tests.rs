use spinon_core::{AttributeName, DocumentChangeBatch, DocumentOperation, OwnerId};
use spinon_layout::{LayoutGridItemIntrinsicSource, LayoutGridTrackSizingAxis};

use super::fixture::{Options, Track, VIEWPORT, cascade, close, make_fixture};
use super::prepare;

#[test]
fn unknown_host_ids_accept_all_four_mixed_intrinsic_scenes() {
    for (track, reversed, expected_track, expected_text_height) in [
        (Track::MinContent, false, 50.0, 20.0),
        (Track::MinContent, true, 50.0, 20.0),
        (Track::MaxContent, false, 80.0, 10.0),
        (Track::MaxContent, true, 80.0, 10.0),
    ] {
        let fixture = make_fixture(Options::novel(track, reversed));
        let styles = cascade(&fixture.view);
        let input = prepare(&fixture.snapshot, fixture.root, &styles)
            .unwrap_or_else(|error| panic!("unexpected adapter error: {error:?}"))
            .expect("novel HostNode IDs must use the generic mixed adapter");
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
        close(root_frame.width, 120.0, "root width");
        close(root_frame.height, 48.0, "root height");
        let text_frame = output.layout.frames[&text_item.id()];
        close(text_frame.x, 0.0, "text x");
        close(text_frame.y, 0.0, "text y");
        close(text_frame.width, expected_track, "text width");
        close(text_frame.height, expected_text_height, "text height");
        let image_frame = output.layout.frames[&image_item.id()];
        close(image_frame.x, 0.0, "image x");
        close(image_frame.y, 24.0, "image y");
        close(image_frame.width, 24.0, "image width");
        close(image_frame.height, 16.0, "image height");

        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .expect("root grid trace");
        let columns = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == LayoutGridTrackSizingAxis::Columns)
            .max_by_key(|pass| pass.sequence)
            .expect("column pass");
        let final_tracks = columns
            .stages
            .last()
            .expect("final stage")
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(final_tracks.len(), 1);
        close(final_tracks[0], expected_track, "used column track");

        assert_eq!(output.layout.grid_item_intrinsic_inputs.len(), 2);
        let text_input = output
            .layout
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == text_item.id())
            .expect("text intrinsic input");
        assert!(matches!(
            text_input.source,
            LayoutGridItemIntrinsicSource::Text { .. }
        ));
        assert_eq!(text_input.source_order, if reversed { 1 } else { 0 });
        assert_eq!(text_input.revision, output.layout.revision);
        let image_input = output
            .layout
            .grid_item_intrinsic_inputs
            .iter()
            .find(|input| input.node == image_item.id())
            .expect("image intrinsic input");
        match &image_input.source {
            LayoutGridItemIntrinsicSource::Replaced { natural } => {
                assert_eq!(natural.width, Some(24.0));
                assert_eq!(natural.height, Some(16.0));
            }
            ref source => panic!("unexpected image intrinsic source: {source:?}"),
        }
        assert_eq!(image_input.source_order, if reversed { 0 } else { 1 });
        assert_eq!(image_input.revision, output.layout.revision);
    }
}

#[test]
fn malformed_mixed_inputs_fail_closed() {
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
    cases.push(("wrong column profile", wrong_track));

    let mut wrong_edge = Options::novel(Track::MinContent, false);
    wrong_edge.bad_edge = true;
    cases.push(("nonzero root edge", wrong_edge));

    let mut wrong_placement = Options::novel(Track::MinContent, false);
    wrong_placement.bad_text_placement = true;
    cases.push(("wrong item placement", wrong_placement));

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
fn profile_and_revision_mismatches_fail_closed() {
    let fixture = make_fixture(Options::novel(Track::MinContent, false));
    let mut styles = cascade(&fixture.view);
    styles.profile = spinon_style::ComputedStyleProfile::RuntimeRootHtmlTextPaintV1;
    assert!(prepare(&fixture.snapshot, fixture.root, &styles).is_err());

    let mut fixture = make_fixture(Options::novel(Track::MinContent, false));
    let styles = cascade(&fixture.view);
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_488).unwrap(),
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
fn text_only_and_image_only_roots_do_not_claim_the_mixed_route() {
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
}

#[test]
fn existing_static_scene_still_uses_its_fixed_id_route() {
    let mut options = Options::novel(Track::MinContent, false);
    options.root_id = "c114-direct-min-content-forward-grid";
    options.text_id = "c114-direct-min-content-forward-text";
    options.image_id = "c114-direct-min-content-forward-image";
    let fixture = make_fixture(options);
    let styles = cascade(&fixture.view);
    assert!(
        prepare(&fixture.snapshot, fixture.root, &styles)
            .unwrap()
            .is_some()
    );
}

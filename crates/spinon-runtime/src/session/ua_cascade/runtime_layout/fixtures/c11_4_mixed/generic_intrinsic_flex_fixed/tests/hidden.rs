use spinon_core::{DocumentChangeBatch, HostNodeHandle, HostParent, OwnerId};
use spinon_layout::{LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind};
use spinon_style::StyloDocumentView;

use super::super::Axis;
use super::{Fixture, NodeSpec, VIEWPORT, append_spec, cascade};

struct HiddenFixture {
    fixture: Fixture,
    hidden: HostNodeHandle,
}

#[test]
fn hidden_text_is_absent_from_columns_and_rows_sizing_inputs() {
    for axis in [Axis::Columns, Axis::Rows] {
        let hidden = make_fixture(axis);
        let fixture = &hidden.fixture;
        let styles = cascade(fixture);
        let input = super::super::prepare(&fixture.snapshot, fixture.root, &styles)
            .unwrap_or_else(|error| panic!("hidden mixed scene rejected: {error:?}"))
            .expect("two fixed hidden-item scenes route through the existing mixed profile");
        assert!(!input.use_extended_fixture_projection);
        assert_eq!(input.paragraph_styles.len(), 2);
        assert!(!input.paragraph_styles.contains_key(&hidden.hidden.id()));
        assert!(input.paragraph_styles.contains_key(&fixture.a.id()));
        assert!(input.paragraph_styles.contains_key(&fixture.c.id()));

        let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
            &fixture.snapshot,
            fixture.root,
            styles,
            VIEWPORT,
            &input.paragraph_styles,
            &input.engine,
            input.intrinsic_sizes,
        )
        .unwrap_or_else(|error| panic!("hidden mixed Grid layout failed: {error:?}"));
        let root = output.layout.frames[&fixture.root.id()];
        let hidden_frame = output.layout.frames[&hidden.hidden.id()];
        let a = output.layout.frames[&fixture.a.id()];
        let image = output.layout.frames[&fixture.b.id()];
        let c = output.layout.frames[&fixture.c.id()];
        assert_eq!(
            [
                hidden_frame.x,
                hidden_frame.y,
                hidden_frame.width,
                hidden_frame.height
            ],
            [0.0; 4]
        );
        close(
            root.width,
            if axis == Axis::Rows { 120.0 } else { 300.0 },
            "root width",
        );
        close(
            root.height,
            if axis == Axis::Rows { 300.0 } else { 120.0 },
            "root height",
        );
        match axis {
            Axis::Columns => {
                close(a.width, 100.0, "visible first-track item");
                close(a.height, 120.0, "visible first item cross size");
                close(image.x, 110.0, "fixed image x");
                close(image.y, 0.0, "fixed image y");
                close(c.x, 160.0, "third-track item x");
                close(c.width, 140.0, "second flexible track");
            }
            Axis::Rows => {
                close(a.width, 120.0, "visible first item cross size");
                close(a.height, 100.0, "visible first-track item");
                close(image.x, 0.0, "fixed image x");
                close(image.y, 110.0, "fixed image y");
                close(c.y, 160.0, "third-track item y");
                close(c.height, 140.0, "second flexible track");
            }
        }
        close(image.width, 24.0, "image natural width");
        close(image.height, 16.0, "image natural height");

        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .expect("hidden scene track-sizing trace");
        let target = if axis == Axis::Rows {
            LayoutGridTrackSizingAxis::Rows
        } else {
            LayoutGridTrackSizingAxis::Columns
        };
        let pass = trace
            .passes
            .iter()
            .find(|pass| pass.axis == target)
            .expect("target-axis pass");
        assert_eq!(
            pass.stages
                .iter()
                .filter(|stage| stage.stage == LayoutGridTrackSizingStageKind::ExpandFlexibleTracks)
                .count(),
            1
        );
        assert!(
            pass.contributions
                .iter()
                .all(|contribution| contribution.item != hidden.hidden.id())
        );
        assert!(
            output
                .layout
                .grid_item_intrinsic_inputs
                .iter()
                .all(|item| item.node != hidden.hidden.id())
        );
        assert_eq!(
            output
                .layout
                .grid_item_intrinsic_inputs
                .iter()
                .map(|item| (item.node, item.source_order))
                .collect::<Vec<_>>(),
            vec![
                (fixture.a.id(), 1),
                (fixture.b.id(), 2),
                (fixture.c.id(), 3),
            ]
        );
        let used = pass
            .stages
            .iter()
            .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::ExpandFlexibleTracks)
            .unwrap()
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(used, [100.0, 40.0, 140.0]);
    }
}

#[test]
fn hidden_scene_shape_is_exact_and_fail_closed() {
    for axis in [Axis::Columns, Axis::Rows] {
        let hidden = make_fixture(axis);
        let fixture = &hidden.fixture;
        let mut styles = cascade(fixture);
        let item = element_mut(&mut styles, hidden.hidden);
        assert_eq!(item.properties["display"], "none");
        assert!(item.grid_item_placement.is_none());

        for (name, value) in [
            ("display", "block"),
            ("display", "contents"),
            ("visibility", "hidden"),
            ("font-family", "Arial"),
            ("grid-column-start", "2"),
        ] {
            let mut changed = styles.clone();
            element_mut(&mut changed, hidden.hidden)
                .properties
                .insert(name.to_owned(), value.to_owned());
            assert_rejected(fixture, &changed, "computed hidden style mutation");
        }

        let mut wrong_writing_mode = styles.clone();
        let writing = std::sync::Arc::make_mut(&mut wrong_writing_mode.writing_styles);
        let entry = writing
            .iter_mut()
            .find(|(node, _, _)| *node == hidden.hidden.id())
            .expect("hidden typed writing style");
        entry.1 = if axis == Axis::Rows {
            spinon_core::WritingMode::HorizontalTb
        } else {
            spinon_core::WritingMode::VerticalRl
        };
        assert_rejected(
            fixture,
            &wrong_writing_mode,
            "hidden typed writing-mode mutation",
        );

        let mut changed_tracks = styles.clone();
        let root_style = element_mut(&mut changed_tracks, fixture.root);
        let grid = root_style.grid_layout.as_mut().unwrap();
        let tracks = if axis == Axis::Rows {
            &mut grid.rows
        } else {
            &mut grid.columns
        };
        tracks[0] = spinon_style::ComputedCssGridTrackSize::MinMax {
            min: spinon_style::ComputedCssGridTrackBreadth::Auto,
            max: spinon_style::ComputedCssGridTrackBreadth::Fraction(1.0),
        };
        assert_rejected(fixture, &changed_tracks, "hidden scene track mutation");

        styles.document_revision = Default::default();
        assert_rejected(fixture, &styles, "hidden scene stale revision");
    }
}

fn make_fixture(axis: Axis) -> HiddenFixture {
    let scene_id = if axis == Axis::Rows {
        "row-hidden"
    } else {
        "column-hidden"
    };
    let prefix = format!("novel-mixed-intrinsic-flex-fixed-{scene_id}");
    let image = NodeSpec::Element {
        tag: "img",
        id: format!("{prefix}-b"),
        class: format!("item image b {}", axis.class_name()),
        attributes: vec![
            ("alt", String::new()),
            (
                "src",
                "https://spinon.invalid/c14-1-png-basic.png".to_owned(),
            ),
        ],
        children: Vec::new(),
    };
    let root = NodeSpec::Element {
        tag: "div",
        id: format!("{prefix}-root"),
        class: format!("root {}", axis.class_name()),
        attributes: Vec::new(),
        children: vec![
            NodeSpec::Element {
                tag: "span",
                id: format!("{prefix}-hidden"),
                class: "item text hidden".to_owned(),
                attributes: Vec::new(),
                children: vec![NodeSpec::Text("XXXXXXXXXXXXXXX".to_owned())],
            },
            NodeSpec::Element {
                tag: "div",
                id: format!("{prefix}-a"),
                class: "item text a".to_owned(),
                attributes: Vec::new(),
                children: vec![NodeSpec::Text("XXXXXXXXXX".to_owned())],
            },
            image,
            NodeSpec::Element {
                tag: "div",
                id: format!("{prefix}-c"),
                class: format!("item text c {}", axis.class_name()),
                attributes: Vec::new(),
                children: vec![NodeSpec::Text("XXXX".to_owned())],
            },
        ],
    };
    let mut document = spinon_core::HostDocument::new().unwrap();
    let mut batch =
        DocumentChangeBatch::new(OwnerId::new(11_489).unwrap(), document.document_revision());
    let root = append_spec(&mut document, &mut batch, HostParent::Root, root);
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_runtime_mount_shared(
        std::sync::Arc::new(snapshot.clone()),
        root,
    )
    .unwrap();
    let mut hidden = None;
    let mut a = None;
    let mut b = None;
    let mut c = None;
    for child in snapshot.children(root).unwrap() {
        let (_, tag, id) = super::super::super::element_identity(&snapshot, child).unwrap();
        match (tag.as_str(), id.as_str()) {
            ("span", id) if id.ends_with("-hidden") => hidden = Some(child),
            ("div", id) if id.ends_with("-a") => a = Some(child),
            ("img", id) if id.ends_with("-b") => b = Some(child),
            ("div", id) if id.ends_with("-c") => c = Some(child),
            other => panic!("unexpected fixture child: {other:?}"),
        }
    }
    let hidden = hidden.unwrap();
    HiddenFixture {
        fixture: Fixture {
            _document: document,
            snapshot,
            root,
            a: a.unwrap(),
            b: b.unwrap(),
            c: c.unwrap(),
            view,
        },
        hidden,
    }
}

fn element_mut(
    styles: &mut spinon_style::ComputedStyleSnapshot,
    handle: HostNodeHandle,
) -> &mut spinon_style::ComputedElementStyle {
    std::sync::Arc::make_mut(&mut styles.elements)
        .iter_mut()
        .find(|style| style.node_id == handle.id())
        .expect("computed style for fixture element")
}

fn assert_rejected(fixture: &Fixture, styles: &spinon_style::ComputedStyleSnapshot, label: &str) {
    match super::super::prepare(&fixture.snapshot, fixture.root, styles) {
        Err(_) => {}
        Ok(None) => panic!("{label}: malformed hidden item silently fell through"),
        Ok(Some(_)) => panic!("{label}: malformed hidden item produced a fixture input"),
    }
}

fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.05,
        "{label}: expected {expected}, got {actual}"
    );
}

use spinon_core::{DocumentChangeBatch, HostNodeHandle, HostParent, OwnerId};
use spinon_layout::{LayoutGridTrackSizingAxis, LayoutGridTrackSizingStageKind};
use spinon_style::StyloDocumentView;

use super::super::Axis;
use super::{Fixture, NodeSpec, VIEWPORT, append_spec, cascade};

#[path = "visibility_hidden/border_overflow.rs"]
mod border_overflow;

struct VisibilityFixture {
    fixture: Fixture,
    span: HostNodeHandle,
}

#[test]
fn visibility_hidden_contributes_to_columns_and_rows_without_a_visible_control_regression() {
    for (axis, visible_control) in [
        (Axis::Columns, false),
        (Axis::Columns, true),
        (Axis::Rows, false),
        (Axis::Rows, true),
    ] {
        let scene = make_fixture(axis, visible_control);
        let fixture = &scene.fixture;
        let styles = cascade(fixture);
        let input = super::super::prepare(&fixture.snapshot, fixture.root, &styles)
            .unwrap_or_else(|error| panic!("visibility mixed scene rejected: {error:?}"))
            .expect("four exact visibility scenes route through the mixed profile");
        assert!(!input.use_extended_fixture_projection);
        assert_eq!(input.paragraph_styles.len(), 3);
        for handle in [fixture.a, scene.span, fixture.c] {
            assert!(input.paragraph_styles.contains_key(&handle.id()));
        }

        let output = spinon_style_to_layout::compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_and_intrinsic_sizes(
            &fixture.snapshot,
            fixture.root,
            styles,
            VIEWPORT,
            &input.paragraph_styles,
            &input.engine,
            input.intrinsic_sizes,
        )
        .unwrap_or_else(|error| panic!("visibility mixed Grid layout failed: {error:?}"));

        let root = output.layout.frames[&fixture.root.id()];
        let a = output.layout.frames[&fixture.a.id()];
        let span = output.layout.frames[&scene.span.id()];
        let image = output.layout.frames[&fixture.b.id()];
        let c = output.layout.frames[&fixture.c.id()];
        match axis {
            Axis::Columns => {
                close(root.width, 300.0, "root width");
                close(root.height, 120.0, "root height");
                for frame in [a, span] {
                    close(frame.x, 0.0, "overlapping first item x");
                    close(frame.y, 0.0, "overlapping first item y");
                    close(frame.width, 150.0, "overlapping first item width");
                    close(frame.height, 120.0, "overlapping first item height");
                }
                close(image.x, 160.0, "fixed image x");
                close(image.y, 0.0, "fixed image y");
                close(c.x, 210.0, "third-track item x");
                close(c.width, 90.0, "third-track item width");
            }
            Axis::Rows => {
                close(root.width, 120.0, "root width");
                close(root.height, 300.0, "root height");
                for frame in [a, span] {
                    close(frame.x, 0.0, "overlapping first item x");
                    close(frame.y, 0.0, "overlapping first item y");
                    close(frame.width, 120.0, "overlapping first item width");
                    close(frame.height, 150.0, "overlapping first item height");
                }
                close(image.x, 0.0, "fixed image x");
                close(image.y, 160.0, "fixed image y");
                close(c.y, 210.0, "third-track item y");
                close(c.height, 90.0, "third-track item height");
            }
        }
        close(image.width, 24.0, "image natural width");
        close(image.height, 16.0, "image natural height");

        assert_eq!(
            output
                .layout
                .grid_item_intrinsic_inputs
                .iter()
                .map(|item| (item.node, item.source_order))
                .collect::<Vec<_>>(),
            vec![
                (fixture.a.id(), 0),
                (scene.span.id(), 1),
                (fixture.b.id(), 2),
                (fixture.c.id(), 3),
            ]
        );
        let trace = output
            .layout
            .grid_track_sizing_traces
            .iter()
            .find(|trace| trace.node == fixture.root.id())
            .expect("visibility scene track-sizing trace");
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
        assert!(
            pass.contributions
                .iter()
                .any(|item| item.item == scene.span.id())
        );
        let used = pass
            .stages
            .iter()
            .find(|stage| stage.stage == LayoutGridTrackSizingStageKind::ExpandFlexibleTracks)
            .expect("flex expansion stage")
            .tracks
            .iter()
            .filter(|track| !track.is_gutter && !track.is_collapsed)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        assert_eq!(used, [150.0, 40.0, 90.0]);
    }
}

#[test]
fn visibility_scene_contract_rejects_visibility_and_grid_input_mutations() {
    for (axis, visible_control) in [(Axis::Columns, false), (Axis::Rows, true)] {
        let scene = make_fixture(axis, visible_control);
        let fixture = &scene.fixture;
        let styles = cascade(fixture);
        let expected = if visible_control { "visible" } else { "hidden" };
        assert_eq!(
            styles
                .elements
                .iter()
                .find(|style| style.node_id == scene.span.id())
                .unwrap()
                .properties["visibility"],
            expected
        );

        let mut changed_span = styles.clone();
        element_mut(&mut changed_span, scene.span)
            .properties
            .insert(
                "visibility".to_owned(),
                if visible_control { "hidden" } else { "visible" }.to_owned(),
            );
        assert_rejected(fixture, &changed_span, "span visibility mutation");

        let mut changed_root = styles.clone();
        element_mut(&mut changed_root, fixture.root)
            .properties
            .insert("visibility".to_owned(), "hidden".to_owned());
        assert_rejected(fixture, &changed_root, "root inherited visibility mutation");

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
        assert_rejected(fixture, &changed_tracks, "root track mutation");
    }
}

fn make_fixture(axis: Axis, visible_control: bool) -> VisibilityFixture {
    make_fixture_with_border_overflow(axis, visible_control, false)
}

fn make_fixture_with_border_overflow(
    axis: Axis,
    visible_control: bool,
    border_overflow: bool,
) -> VisibilityFixture {
    let scene_id = match (axis, visible_control, border_overflow) {
        (Axis::Columns, false, false) => "column-visibility-hidden",
        (Axis::Columns, true, false) => "column-visibility-visible-control",
        (Axis::Rows, false, false) => "row-visibility-hidden",
        (Axis::Rows, true, false) => "row-visibility-visible-control",
        (Axis::Columns, false, true) => "column-visibility-hidden-border-overflow",
        (Axis::Columns, true, true) => "column-visibility-visible-border-overflow-control",
        (Axis::Rows, false, true) => "row-visibility-hidden-border-overflow",
        (Axis::Rows, true, true) => "row-visibility-visible-border-overflow-control",
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
        class: format!(
            "root {}{}",
            axis.class_name(),
            if border_overflow {
                " border-overflow"
            } else {
                ""
            }
        ),
        attributes: Vec::new(),
        children: vec![
            NodeSpec::Element {
                tag: "div",
                id: format!("{prefix}-a"),
                class: "item text a".to_owned(),
                attributes: Vec::new(),
                children: vec![NodeSpec::Text("XXXXXXXXXX".to_owned())],
            },
            NodeSpec::Element {
                tag: "span",
                id: format!("{prefix}-visibility"),
                class: if visible_control {
                    "item text visibility-visible-control".to_owned()
                } else {
                    "item text visibility-hidden".to_owned()
                },
                attributes: Vec::new(),
                children: vec![NodeSpec::Text("XXXXXXXXXXXXXXX".to_owned())],
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
        DocumentChangeBatch::new(OwnerId::new(11_490).unwrap(), document.document_revision());
    let root = append_spec(&mut document, &mut batch, HostParent::Root, root);
    document.commit(batch).unwrap();
    let snapshot = document.snapshot();
    let view = StyloDocumentView::new_html_runtime_mount_shared(
        std::sync::Arc::new(snapshot.clone()),
        root,
    )
    .unwrap();
    let mut a = None;
    let mut span = None;
    let mut b = None;
    let mut c = None;
    for child in snapshot.children(root).unwrap() {
        let (_, tag, id) = super::super::super::element_identity(&snapshot, child).unwrap();
        match (tag.as_str(), id.as_str()) {
            ("div", id) if id.ends_with("-a") => a = Some(child),
            ("span", id) if id.ends_with("-visibility") => span = Some(child),
            ("img", id) if id.ends_with("-b") => b = Some(child),
            ("div", id) if id.ends_with("-c") => c = Some(child),
            other => panic!("unexpected visibility fixture child: {other:?}"),
        }
    }
    VisibilityFixture {
        fixture: Fixture {
            _document: document,
            snapshot,
            root,
            a: a.unwrap(),
            b: b.unwrap(),
            c: c.unwrap(),
            view,
        },
        span: span.unwrap(),
    }
}

fn element_mut(
    styles: &mut spinon_style::ComputedStyleSnapshot,
    handle: HostNodeHandle,
) -> &mut spinon_style::ComputedElementStyle {
    std::sync::Arc::make_mut(&mut styles.elements)
        .iter_mut()
        .find(|style| style.node_id == handle.id())
        .expect("fixture computed style")
}

fn assert_rejected(fixture: &Fixture, styles: &spinon_style::ComputedStyleSnapshot, label: &str) {
    let result = super::super::prepare(&fixture.snapshot, fixture.root, styles);
    assert!(result.is_err(), "{label} must fail closed");
}

fn close(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() < 1.0e-4,
        "{label}: expected {expected}, got {actual}"
    );
}

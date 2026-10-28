use std::sync::Arc;

use spinon_core::{DocumentChangeBatch, DocumentOperation, HostNodeHandle, HostParent, OwnerId};
use spinon_style::{
    ComputedCssDimension, ComputedCssGridPlacement, ComputedCssGridTrackBreadth,
    ComputedCssGridTrackSize, ComputedCssMaxSize, ComputedCssPosition, ComputedCssSpacingValue,
    ComputedElementStyle, ComputedStyleProfile, ComputedStyleSnapshot, StyloDocumentView,
};

use super::{Axis, Fixture, NodeSpec, append_spec, cascade, make_fixture};

#[test]
fn changed_ahem_text_and_direct_item_shape_are_rejected() {
    for axis in [Axis::Columns, Axis::Rows] {
        for (a_text, c_text) in [
            ("XXXXXXXXXY", "XXXX"),
            ("XXXXXXXXXX", "XXXY"),
            ("XXXXXX", "XXXX"),
        ] {
            let mut fixture = make_fixture(axis, 10, false);
            let a = fixture.a;
            replace_text(&mut fixture, a, a_text);
            let c = fixture.c;
            replace_text(&mut fixture, c, c_text);
            let styles = cascade(&fixture);
            assert_dispatch_rejects(&fixture, &styles, "changed Ahem source");
        }
    }
}

#[test]
fn missing_extra_and_nonmonotonic_track_order_are_rejected() {
    for axis in [Axis::Columns, Axis::Rows] {
        let mut missing = make_fixture(axis, 10, false);
        let omitted = missing.c;
        remove_child(&mut missing, omitted);
        let styles = cascade(&missing);
        assert_dispatch_rejects(&missing, &styles, "missing direct Grid item");

        let mut extra = make_fixture(axis, 10, false);
        append_extra_child(&mut extra, axis);
        let styles = cascade(&extra);
        assert_dispatch_rejects(&extra, &styles, "extra direct Grid item");

        let mut reordered = make_fixture(axis, 10, false);
        let order = [reordered.a, reordered.c, reordered.b];
        reorder_children(&mut reordered, order);
        let styles = cascade(&reordered);
        assert_dispatch_rejects(&reordered, &styles, "nonmonotonic source track order");
    }
}

#[test]
fn root_dimensions_gaps_alignment_and_constraints_are_rejected() {
    for axis in [Axis::Columns, Axis::Rows] {
        assert_root_mutation_rejected(axis, "wrong definite dimension", |root| {
            let width = if axis == Axis::Columns { 299.0 } else { 119.0 };
            root.layout_dimensions.width = ComputedCssDimension::LengthPx(width);
        });
        assert_root_mutation_rejected(axis, "wrong target-axis gap", |root| match axis {
            Axis::Columns => {
                root.layout_spacing.column_gap = ComputedCssSpacingValue::LengthPx(9.0)
            }
            Axis::Rows => root.layout_spacing.row_gap = ComputedCssSpacingValue::LengthPx(9.0),
        });
        assert_root_mutation_rejected(axis, "wrong alignment declaration", |root| {
            root.properties
                .insert("align-content".to_owned(), "normal".to_owned());
        });
        assert_root_mutation_rejected(axis, "unexpected min-size", |root| {
            root.layout_dimensions.min_width = ComputedCssDimension::LengthPx(1.0);
        });
        assert_root_mutation_rejected(axis, "unexpected max-size", |root| {
            root.layout_dimensions.max_height = ComputedCssMaxSize::LengthPx(299.0);
        });
    }
}

#[test]
fn item_presentation_typography_and_placement_changes_are_rejected() {
    for axis in [Axis::Columns, Axis::Rows] {
        assert_item_mutation_rejected(axis, "changed display", |item| {
            item.properties
                .insert("display".to_owned(), "none".to_owned());
        });
        assert_item_mutation_rejected(axis, "changed position", |item| {
            item.layout_position = ComputedCssPosition::Relative;
        });
        assert_item_mutation_rejected(axis, "authored item width", |item| {
            item.layout_dimensions.width = ComputedCssDimension::LengthPx(10.0);
        });
        assert_item_mutation_rejected(axis, "changed font family", |item| {
            item.properties
                .insert("font-family".to_owned(), "Arial".to_owned());
        });
        assert_item_mutation_rejected(axis, "changed typed font size", |item| {
            item.font_size_css_px = 11.0;
        });
        assert_item_mutation_rejected(axis, "spanning item placement", |item| {
            let placement = item.grid_item_placement.as_mut().unwrap();
            match axis {
                Axis::Columns => placement.column_end = ComputedCssGridPlacement::Line(3),
                Axis::Rows => placement.row_end = ComputedCssGridPlacement::Line(3),
            }
        });
    }
}

#[test]
fn track_function_factor_neighbor_axis_and_nonfinite_changes_do_not_fall_through() {
    for axis in [Axis::Columns, Axis::Rows] {
        assert_target_tracks_rejected(axis, "changed intrinsic minimum", |tracks| {
            tracks[0] = ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::Auto,
                max: ComputedCssGridTrackBreadth::Fraction(1.0),
            };
        });
        assert_target_tracks_rejected(axis, "changed flex factor", |tracks| {
            tracks[2] = ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(0.0),
                max: ComputedCssGridTrackBreadth::Fraction(3.0),
            };
        });
        assert_target_tracks_rejected(axis, "changed fixed neighbor", |tracks| {
            tracks[1] = ComputedCssGridTrackSize::LengthPx(41.0);
        });
        assert_target_tracks_rejected(axis, "non-finite flex factor", |tracks| {
            tracks[2] = ComputedCssGridTrackSize::MinMax {
                min: ComputedCssGridTrackBreadth::LengthPx(0.0),
                max: ComputedCssGridTrackBreadth::Fraction(f32::INFINITY),
            };
        });

        let fixture = make_fixture(axis, 10, false);
        let mut styles = cascade(&fixture);
        let root = element_mut(&mut styles, fixture.root);
        let grid = root.grid_layout.as_mut().unwrap();
        match axis {
            Axis::Columns => {
                grid.columns = vec![ComputedCssGridTrackSize::LengthPx(120.0)];
                grid.rows = mixed_tracks();
            }
            Axis::Rows => {
                grid.columns = mixed_tracks();
                grid.rows = vec![ComputedCssGridTrackSize::LengthPx(120.0)];
            }
        }
        assert_dispatch_rejects(&fixture, &styles, "track list moved to the other axis");
    }
}

#[test]
fn stale_revision_and_wrong_computed_profile_are_rejected() {
    for axis in [Axis::Columns, Axis::Rows] {
        let fixture = make_fixture(axis, 10, false);
        assert_ne!(fixture.snapshot.document_revision().get(), 0);

        let mut stale = cascade(&fixture);
        stale.document_revision = Default::default();
        assert_dispatch_rejects(&fixture, &stale, "stale document revision");

        let mut wrong_profile = cascade(&fixture);
        wrong_profile.profile = ComputedStyleProfile::FlexLayoutV1;
        assert_dispatch_rejects(&fixture, &wrong_profile, "wrong computed-style profile");

        let mut wrong_writing_mode = cascade(&fixture);
        let writing = Arc::make_mut(&mut wrong_writing_mode.writing_styles);
        let entry = writing
            .iter_mut()
            .find(|(node, _, _)| *node == fixture.a.id())
            .expect("typed writing style for first text item");
        entry.1 = match axis {
            Axis::Columns => spinon_core::WritingMode::VerticalRl,
            Axis::Rows => spinon_core::WritingMode::HorizontalTb,
        };
        assert_dispatch_rejects(&fixture, &wrong_writing_mode, "wrong typed writing mode");
    }
}

fn assert_root_mutation_rejected(
    axis: Axis,
    label: &str,
    mutate: impl FnOnce(&mut ComputedElementStyle),
) {
    let fixture = make_fixture(axis, 10, false);
    let mut styles = cascade(&fixture);
    mutate(element_mut(&mut styles, fixture.root));
    assert_dispatch_rejects(&fixture, &styles, label);
}

fn assert_item_mutation_rejected(
    axis: Axis,
    label: &str,
    mutate: impl FnOnce(&mut ComputedElementStyle),
) {
    let fixture = make_fixture(axis, 10, false);
    let mut styles = cascade(&fixture);
    mutate(element_mut(&mut styles, fixture.a));
    assert_dispatch_rejects(&fixture, &styles, label);
}

fn assert_target_tracks_rejected(
    axis: Axis,
    label: &str,
    mutate: impl FnOnce(&mut Vec<ComputedCssGridTrackSize>),
) {
    let fixture = make_fixture(axis, 10, false);
    let mut styles = cascade(&fixture);
    let grid = element_mut(&mut styles, fixture.root)
        .grid_layout
        .as_mut()
        .unwrap();
    mutate(match axis {
        Axis::Columns => &mut grid.columns,
        Axis::Rows => &mut grid.rows,
    });
    assert_dispatch_rejects(&fixture, &styles, label);
}

fn assert_dispatch_rejects(fixture: &Fixture, styles: &ComputedStyleSnapshot, label: &str) {
    match super::super::super::prepare(&fixture.snapshot, fixture.root, styles) {
        Err(_) => {}
        Ok(None) => panic!("{label}: malformed mixed input silently fell through"),
        Ok(Some(_)) => panic!("{label}: malformed mixed input produced a fixture input"),
    }
}

fn element_mut(
    styles: &mut ComputedStyleSnapshot,
    handle: HostNodeHandle,
) -> &mut ComputedElementStyle {
    Arc::make_mut(&mut styles.elements)
        .iter_mut()
        .find(|style| style.node_id == handle.id())
        .expect("computed style for fixture element")
}

fn replace_text(fixture: &mut Fixture, item: HostNodeHandle, text: &str) {
    let text_node = fixture
        .snapshot
        .children(item)
        .and_then(|mut children| children.next())
        .expect("direct text child");
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_489).unwrap(),
        fixture._document.document_revision(),
    );
    batch.push(DocumentOperation::SetTextData {
        node: text_node,
        data: text.to_owned().into(),
    });
    fixture._document.commit(batch).unwrap();
    refresh(fixture);
}

fn remove_child(fixture: &mut Fixture, child: HostNodeHandle) {
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_489).unwrap(),
        fixture._document.document_revision(),
    );
    batch.push(DocumentOperation::RemoveChild {
        parent: HostParent::Node(fixture.root),
        node: child,
    });
    fixture._document.commit(batch).unwrap();
    refresh(fixture);
}

fn reorder_children(fixture: &mut Fixture, order: [HostNodeHandle; 3]) {
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_489).unwrap(),
        fixture._document.document_revision(),
    );
    for child in [fixture.a, fixture.b, fixture.c] {
        batch.push(DocumentOperation::RemoveChild {
            parent: HostParent::Node(fixture.root),
            node: child,
        });
    }
    for child in order {
        batch.push(DocumentOperation::InsertBefore {
            parent: HostParent::Node(fixture.root),
            node: child,
            before: None,
        });
    }
    fixture._document.commit(batch).unwrap();
    refresh(fixture);
}

fn append_extra_child(fixture: &mut Fixture, axis: Axis) {
    let mut batch = DocumentChangeBatch::new(
        OwnerId::new(11_489).unwrap(),
        fixture._document.document_revision(),
    );
    append_spec(
        &mut fixture._document,
        &mut batch,
        HostParent::Node(fixture.root),
        NodeSpec::Element {
            tag: "div",
            id: "novel-extra".to_owned(),
            class: format!("item text extra {}", axis.class_name()),
            attributes: Vec::new(),
            children: vec![NodeSpec::Text("XXXX".to_owned())],
        },
    );
    fixture._document.commit(batch).unwrap();
    refresh(fixture);
}

fn refresh(fixture: &mut Fixture) {
    fixture.snapshot = fixture._document.snapshot();
    fixture.view = StyloDocumentView::new_html_runtime_mount_shared(
        Arc::new(fixture.snapshot.clone()),
        fixture.root,
    )
    .unwrap();
}

fn mixed_tracks() -> Vec<ComputedCssGridTrackSize> {
    vec![
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::MinContent,
            max: ComputedCssGridTrackBreadth::Fraction(1.0),
        },
        ComputedCssGridTrackSize::LengthPx(40.0),
        ComputedCssGridTrackSize::MinMax {
            min: ComputedCssGridTrackBreadth::LengthPx(0.0),
            max: ComputedCssGridTrackBreadth::Fraction(2.0),
        },
    ]
}

use super::*;

fn names(entries: &[&[&str]]) -> Vec<Vec<String>> {
    entries
        .iter()
        .map(|group| group.iter().map(|name| (*name).to_owned()).collect())
        .collect()
}

fn named(name: &str, occurrence: i32) -> ComputedCssGridPlacement {
    ComputedCssGridPlacement::NamedLine {
        name: name.to_owned(),
        occurrence,
    }
}

fn span_named(name: &str, count: u32) -> ComputedCssGridPlacement {
    ComputedCssGridPlacement::SpanNamed {
        name: name.to_owned(),
        count,
    }
}

#[test]
fn positive_occurrence_uses_line_index_and_deduplicates_same_line_names() {
    let line_names = names(&[&["edge", "edge"], &[], &["edge"], &["edge"]]);
    assert_eq!(
        project_axis(
            NodeId::new(1).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &named("edge", 2),
            &ComputedCssGridPlacement::Line(4),
            &line_names,
            3,
        )
        .unwrap()
        .0,
        GridPlacementInput::Line(3)
    );
}

#[test]
fn negative_occurrence_counts_from_last_explicit_line() {
    let line_names = names(&[&["edge"], &[], &["edge"], &["edge"]]);
    assert_eq!(
        project_axis(
            NodeId::new(2).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &named("edge", -2),
            &ComputedCssGridPlacement::Line(4),
            &line_names,
            3,
        )
        .unwrap()
        .0,
        GridPlacementInput::Line(3)
    );
}

#[test]
fn named_span_searches_forward_from_definite_start() {
    let line_names = names(&[&[], &["edge"], &[], &["edge"], &["edge"]]);
    assert_eq!(
        project_axis(
            NodeId::new(3).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &ComputedCssGridPlacement::Line(1),
            &span_named("edge", 2),
            &line_names,
            4,
        )
        .unwrap()
        .1,
        GridPlacementInput::Line(4)
    );
}

#[test]
fn named_span_searches_backward_from_definite_end() {
    let line_names = names(&[&["edge"], &[], &["edge"], &[], &["edge"]]);
    assert_eq!(
        project_axis(
            NodeId::new(4).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &span_named("edge", 2),
            &ComputedCssGridPlacement::Line(5),
            &line_names,
            4,
        )
        .unwrap()
        .0,
        GridPlacementInput::Line(1)
    );
    assert_eq!(
        project_axis(
            NodeId::new(4).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &span_named("edge", 1),
            &ComputedCssGridPlacement::Line(-1),
            &line_names,
            4,
        )
        .unwrap()
        .0,
        GridPlacementInput::Line(3)
    );
}

#[test]
fn same_line_duplicate_name_is_one_span_occurrence() {
    let line_names = names(&[&[], &["edge", "edge"], &["edge"], &[]]);
    assert_eq!(
        project_axis(
            NodeId::new(5).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &ComputedCssGridPlacement::Line(1),
            &span_named("edge", 2),
            &line_names,
            3,
        )
        .unwrap()
        .1,
        GridPlacementInput::Line(3)
    );
}

#[test]
fn line_names_are_case_sensitive() {
    let line_names = names(&[&["Edge"], &[], &["edge"]]);
    assert_eq!(
        project_axis(
            NodeId::new(6).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &named("Edge", 1),
            &ComputedCssGridPlacement::Line(3),
            &line_names,
            2,
        )
        .unwrap()
        .0,
        GridPlacementInput::Line(1)
    );
    assert!(
        project_axis(
            NodeId::new(6).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &named("EDGE", 1),
            &ComputedCssGridPlacement::Line(3),
            &line_names,
            2,
        )
        .is_err()
    );
}

#[test]
fn missing_name_and_occurrence_shortage_fail_closed() {
    let line_names = names(&[&[], &["known"], &[]]);
    for placement in [named("missing", 1), named("known", 2), named("known", -2)] {
        assert!(
            project_axis(
                NodeId::new(7).unwrap(),
                "grid-column-start",
                "grid-column-end",
                &placement,
                &ComputedCssGridPlacement::Line(3),
                &line_names,
                2,
            )
            .is_err()
        );
    }
}

#[test]
fn zero_and_extreme_negative_occurrences_fail_closed_without_overflow() {
    let line_names = names(&[&["edge"], &[], &["edge"]]);
    for occurrence in [0, i32::MIN] {
        let error = project_axis(
            NodeId::new(12).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &named("edge", occurrence),
            &ComputedCssGridPlacement::Auto,
            &line_names,
            2,
        )
        .unwrap_err();
        assert!(matches!(
            error,
            StyleLayoutError::UnsupportedNamedGridLine { .. }
        ));
    }
}

#[test]
fn named_line_metadata_shape_and_zero_span_count_fail_closed() {
    let wrong_length = names(&[&["edge"], &[]]);
    let error = project_axis(
        NodeId::new(13).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &named("edge", 1),
        &ComputedCssGridPlacement::Auto,
        &wrong_length,
        2,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::UnsupportedNamedGridLine { .. }
    ));

    let complete = names(&[&[], &["edge"], &[]]);
    let error = project_axis(
        NodeId::new(14).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &ComputedCssGridPlacement::Line(1),
        &span_named("edge", 0),
        &complete,
        2,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::UnsupportedNamedGridLine { .. }
    ));

    let error = project_axis(
        NodeId::new(15).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &ComputedCssGridPlacement::Line(1),
        &span_named("edge", u32::MAX),
        &complete,
        2,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        StyleLayoutError::UnsupportedNamedGridLine { .. }
    ));
}

#[test]
fn named_span_shortage_fails_instead_of_creating_implicit_lines() {
    let line_names = names(&[&[], &["edge"], &[]]);
    assert!(
        project_axis(
            NodeId::new(8).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &ComputedCssGridPlacement::Line(2),
            &span_named("edge", 1),
            &line_names,
            2,
        )
        .is_err()
    );
}

#[test]
fn lone_named_span_uses_default_single_track_span() {
    let line_names = names(&[&[], &[], &[]]);
    assert_eq!(
        project_axis(
            NodeId::new(9).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &span_named("missing", 4),
            &ComputedCssGridPlacement::Auto,
            &line_names,
            2,
        )
        .unwrap()
        .0,
        GridPlacementInput::Span(1)
    );
}

#[test]
fn conflicting_named_spans_do_not_resolve_names_or_create_implicit_lines() {
    let line_names = names(&[&[], &[], &[]]);
    let (start, end) = project_axis(
        NodeId::new(10).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &span_named("missing-start", 7),
        &span_named("missing-end", 9),
        &line_names,
        2,
    )
    .unwrap();
    assert_eq!(start, GridPlacementInput::Span(1));
    assert_eq!(end, GridPlacementInput::Auto);
}

#[test]
fn conflicting_end_named_span_is_dropped_before_name_resolution() {
    let line_names = names(&[&[], &[], &[]]);
    let (start, end) = project_axis(
        NodeId::new(11).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &ComputedCssGridPlacement::Span(2),
        &span_named("missing-end", 9),
        &line_names,
        2,
    )
    .unwrap();
    assert_eq!(start, GridPlacementInput::Span(2));
    assert_eq!(end, GridPlacementInput::Auto);
}

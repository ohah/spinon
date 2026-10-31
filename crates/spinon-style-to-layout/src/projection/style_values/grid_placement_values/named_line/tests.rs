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
        occurrence: Some(occurrence),
    }
}

fn bare_named(name: &str) -> ComputedCssGridPlacement {
    ComputedCssGridPlacement::NamedLine {
        name: name.to_owned(),
        occurrence: None,
    }
}

fn span_named(name: &str, count: u32) -> ComputedCssGridPlacement {
    ComputedCssGridPlacement::SpanNamed {
        name: name.to_owned(),
        count,
    }
}

fn metadata<'a>(
    names: &'a [Vec<String>],
    opposite_names: &'a [Vec<String>],
    areas: Option<&'a ComputedCssGridTemplateAreas>,
    axis: GridAxis,
    track_count: usize,
) -> LineMetadata<'a> {
    LineMetadata {
        names,
        opposite_names,
        areas,
        axis,
        track_count,
        allow_area_dimension_mismatch: false,
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
fn missing_backward_named_span_creates_leading_line_and_selects_signed_extent() {
    let line_names = names(&[&[], &[]]);
    let (start, end, usage) = project_axis_with_areas_and_implicit_named_lines(
        NodeId::new(40).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &span_named("edge", 1),
        &ComputedCssGridPlacement::Line(4),
        LineMetadata {
            names: &line_names,
            opposite_names: &[],
            areas: None,
            axis: GridAxis::Columns,
            track_count: 1,
            allow_area_dimension_mismatch: false,
        },
        true,
    )
    .unwrap();

    assert_eq!(start, GridPlacementInput::Line(-3));
    assert_eq!(end, GridPlacementInput::Line(4));
    assert_eq!(
        usage,
        ImplicitNamedLineUsage {
            uses_implicit_tracks: true,
            requires_signed_extent: true,
            allows_positive_auto_track_sizing: false,
            allows_positive_named_line_pair_auto_track_sizing: false,
        }
    );
}

#[test]
fn only_two_resolved_positive_implicit_named_line_endpoints_select_auto_pair_sizing() {
    let line_names = names(&[&[], &[], &[]]);
    let (start, end, usage) = project_axis_with_areas_and_implicit_named_lines(
        NodeId::new(42).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &named("start-name", 1),
        &named("end-name", 2),
        metadata(&line_names, &[], None, GridAxis::Columns, 2),
        true,
    )
    .unwrap();

    assert_eq!(start, GridPlacementInput::Line(4));
    assert_eq!(end, GridPlacementInput::Line(5));
    assert!(usage.uses_implicit_tracks);
    assert!(usage.allows_positive_named_line_pair_auto_track_sizing);

    let explicit_start_name = names(&[&["known"], &[], &[]]);
    let (_, _, usage) = project_axis_with_areas_and_implicit_named_lines(
        NodeId::new(43).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &named("known", 1),
        &named("missing", 1),
        metadata(&explicit_start_name, &[], None, GridAxis::Columns, 2),
        true,
    )
    .unwrap();
    assert!(!usage.allows_positive_named_line_pair_auto_track_sizing);

    let (_, _, usage) = project_axis_with_areas_and_implicit_named_lines(
        NodeId::new(44).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &named("start-name", -1),
        &named("end-name", -2),
        metadata(&line_names, &[], None, GridAxis::Columns, 2),
        true,
    )
    .unwrap();
    assert!(!usage.allows_positive_named_line_pair_auto_track_sizing);

    let (_, _, usage) = project_axis_with_areas_and_implicit_named_lines(
        NodeId::new(45).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &span_named("start-name", 1),
        &span_named("end-name", 1),
        metadata(&line_names, &[], None, GridAxis::Columns, 2),
        true,
    )
    .unwrap();
    assert!(!usage.allows_positive_named_line_pair_auto_track_sizing);
}

#[test]
fn missing_negative_occurrences_create_distinct_leading_lines() {
    let line_names = names(&[&[], &[], &[]]);
    let (start, end, usage) = project_axis_with_areas_and_implicit_named_lines(
        NodeId::new(41).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &named("edge", -1),
        &named("edge", -2),
        LineMetadata {
            names: &line_names,
            opposite_names: &[],
            areas: None,
            axis: GridAxis::Columns,
            track_count: 2,
            allow_area_dimension_mismatch: false,
        },
        true,
    )
    .unwrap();

    assert_eq!(start, GridPlacementInput::Line(-4));
    assert_eq!(end, GridPlacementInput::Line(-5));
    assert!(usage.uses_implicit_tracks);
    assert!(usage.requires_signed_extent);
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
fn auto_repeat_named_placements_remain_symbolic_with_or_without_authored_names() {
    let projected = project_axis_for_auto_repeat(
        NodeId::new(15).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &bare_named("item"),
        &span_named("item", 2),
    )
    .unwrap();
    assert_eq!(
        projected,
        (
            GridPlacementInput::NamedLine {
                name: "item".to_owned(),
                occurrence: 0,
            },
            GridPlacementInput::SpanNamed {
                name: "item".to_owned(),
                count: 2,
            },
            false,
        )
    );

    let missing_name = project_axis_for_auto_repeat(
        NodeId::new(16).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &bare_named("not-authored"),
        &ComputedCssGridPlacement::Auto,
    )
    .unwrap();
    assert_eq!(
        missing_name,
        (
            GridPlacementInput::NamedLine {
                name: "not-authored".to_owned(),
                occurrence: 0,
            },
            GridPlacementInput::Auto,
            false,
        )
    );

    let missing_span = project_axis_for_auto_repeat(
        NodeId::new(17).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &ComputedCssGridPlacement::Line(4),
        &span_named("not-authored", 2),
    )
    .unwrap();
    assert_eq!(
        missing_span,
        (
            GridPlacementInput::Line(4),
            GridPlacementInput::SpanNamed {
                name: "not-authored".to_owned(),
                count: 2,
            },
            false,
        )
    );

    for placement in [named("item", 0), named("item", 40_000)] {
        let error = project_axis_for_auto_repeat(
            NodeId::new(15).unwrap(),
            "grid-column-start",
            "grid-column-end",
            &placement,
            &ComputedCssGridPlacement::Auto,
        )
        .unwrap_err();
        assert!(matches!(
            error,
            StyleLayoutError::UnsupportedNamedGridLine { .. }
        ));
    }

    let error = project_axis_for_auto_repeat(
        NodeId::new(18).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &ComputedCssGridPlacement::Line(1),
        &span_named("not-authored", 70_000),
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

#[test]
fn bare_ident_uses_area_edge_while_explicit_one_uses_the_authored_name() {
    let areas = ComputedCssGridTemplateAreas {
        rows: 1,
        columns: 3,
        areas: vec![spinon_style::ComputedCssGridArea {
            name: "hero".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 1,
            column_end: 2,
        }],
    };
    let column_names = names(&[&["hero-start"], &["hero"], &["hero-end"], &[]]);
    let row_names = names(&[&[], &[]]);
    let bare = project_axis_with_areas(
        NodeId::new(16).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &bare_named("hero"),
        &ComputedCssGridPlacement::Line(4),
        metadata(
            &column_names,
            &row_names,
            Some(&areas),
            GridAxis::Columns,
            3,
        ),
    )
    .unwrap();
    let explicit_one = project_axis_with_areas(
        NodeId::new(17).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &named("hero", 1),
        &ComputedCssGridPlacement::Line(4),
        metadata(
            &column_names,
            &row_names,
            Some(&areas),
            GridAxis::Columns,
            3,
        ),
    )
    .unwrap();

    assert_eq!(bare.0, GridPlacementInput::Line(1));
    assert_eq!(explicit_one.0, GridPlacementInput::Line(2));
}

#[test]
fn generated_area_aliases_resolve_as_named_lines_and_named_spans() {
    let areas = ComputedCssGridTemplateAreas {
        rows: 1,
        columns: 3,
        areas: vec![spinon_style::ComputedCssGridArea {
            name: "main".to_owned(),
            row_start: 0,
            row_end: 1,
            column_start: 1,
            column_end: 3,
        }],
    };
    let column_names = names(&[&[], &[], &[], &[]]);
    let row_names = names(&[&[], &[]]);
    let explicit_alias = project_axis_with_areas(
        NodeId::new(18).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &named("main-start", 1),
        &span_named("main-end", 1),
        metadata(
            &column_names,
            &row_names,
            Some(&areas),
            GridAxis::Columns,
            3,
        ),
    )
    .unwrap();

    assert_eq!(explicit_alias.0, GridPlacementInput::Line(2));
    assert_eq!(explicit_alias.1, GridPlacementInput::Line(4));
}

#[test]
fn line_only_implicit_area_requires_matching_edges_on_both_axes() {
    let columns = names(&[&[], &["box-start"], &[], &["box-end"]]);
    let rows = names(&[&[], &["box-start"], &["box-end"]]);
    let implicit = project_axis_with_areas(
        NodeId::new(19).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &bare_named("box"),
        &bare_named("box"),
        metadata(&columns, &rows, None, GridAxis::Columns, 3),
    )
    .unwrap();
    let not_an_area = project_axis_with_areas(
        NodeId::new(20).unwrap(),
        "grid-column-start",
        "grid-column-end",
        &bare_named("box"),
        &ComputedCssGridPlacement::Line(4),
        metadata(
            &columns,
            &names(&[&[], &[], &[]]),
            None,
            GridAxis::Columns,
            3,
        ),
    )
    .unwrap_err();

    assert_eq!(implicit.0, GridPlacementInput::Line(2));
    assert_eq!(implicit.1, GridPlacementInput::Line(4));
    assert!(matches!(
        not_an_area,
        StyleLayoutError::UnsupportedNamedGridLine { .. }
    ));
}

use super::{Constraints, UsedReplacedSize, clamp_replaced_ratio_size, normalize_max};

#[test]
fn css_22_replaced_ratio_min_max_table_covers_all_ten_violations() {
    let natural = UsedReplacedSize {
        content_width: 24.0,
        content_height: 16.0,
        width: 24.0,
        height: 16.0,
    };
    let cases = [
        (
            "width exceeds max",
            Constraints {
                max_width: Some(20.0),
                ..Constraints::default()
            },
            (20.0, 13.333333),
        ),
        (
            "width below min",
            Constraints {
                min_width: Some(30.0),
                ..Constraints::default()
            },
            (30.0, 20.0),
        ),
        (
            "height exceeds max",
            Constraints {
                max_height: Some(10.0),
                ..Constraints::default()
            },
            (15.0, 10.0),
        ),
        (
            "height below min",
            Constraints {
                min_height: Some(24.0),
                ..Constraints::default()
            },
            (36.0, 24.0),
        ),
        (
            "both above max, width ratio wins",
            Constraints {
                max_width: Some(20.0),
                max_height: Some(14.0),
                ..Constraints::default()
            },
            (20.0, 13.333333),
        ),
        (
            "both above max, height ratio wins",
            Constraints {
                max_width: Some(22.0),
                max_height: Some(12.0),
                ..Constraints::default()
            },
            (18.0, 12.0),
        ),
        (
            "both below min, height ratio wins",
            Constraints {
                min_width: Some(30.0),
                min_height: Some(22.0),
                ..Constraints::default()
            },
            (33.0, 22.0),
        ),
        (
            "both below min, width ratio wins",
            Constraints {
                min_width: Some(36.0),
                min_height: Some(20.0),
                ..Constraints::default()
            },
            (36.0, 24.0),
        ),
        (
            "width below min and height above max",
            Constraints {
                min_width: Some(30.0),
                max_height: Some(10.0),
                ..Constraints::default()
            },
            (30.0, 10.0),
        ),
        (
            "width above max and height below min",
            Constraints {
                max_width: Some(20.0),
                min_height: Some(20.0),
                ..Constraints::default()
            },
            (20.0, 20.0),
        ),
    ];

    for (name, constraints, expected) in cases {
        let actual = clamp_replaced_ratio_size(natural, 1.5, constraints);
        assert!(
            (actual.0 - expected.0).abs() < 0.0001 && (actual.1 - expected.1).abs() < 0.0001,
            "{name}: {actual:?} != {expected:?}"
        );
    }
}

#[test]
fn minimum_wins_when_maximum_is_smaller() {
    assert_eq!(normalize_max(Some(48.0), Some(30.0)), Some(48.0));
    assert_eq!(normalize_max(None, Some(30.0)), Some(30.0));
}

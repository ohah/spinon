use super::*;
use crate::{
    GridTrackSize, LayoutCalcId, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue,
    LayoutNode, LayoutStyle,
};

const FIRST_CALC: LayoutCalcId = LayoutCalcId(71_100);

fn calc_value(id: LayoutCalcId, expression: LayoutCssMath) -> LayoutCssMathValue {
    LayoutCssMathValue {
        id,
        node_id: id_fn(1),
        property: LayoutCssMathProperty::GridTemplateRows,
        expression,
    }
}

fn id_fn(value: u64) -> spinon_core::NodeId {
    spinon_core::NodeId::new(value).unwrap()
}

fn grid_with_row_pattern(
    item_count: usize,
    height: f32,
    row_gap: f32,
    auto_rows: Vec<GridTrackSize>,
) -> crate::LayoutInput {
    let mut input = sparse_grid(1, 2, item_count);
    input.nodes[0].style.width = LayoutDimension::Fixed(50.0);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.nodes[0].style.gap = crate::LayoutGap {
        column: LayoutLengthPercentage::LengthPx(0.0),
        row: LayoutLengthPercentage::LengthPx(row_gap),
    };
    input.viewport.width = 50.0;
    input.viewport.height = height.max(1.0);
    for node in input.nodes.iter_mut().skip(1) {
        node.style.display = LayoutDisplay::Block;
        node.style.width = LayoutDimension::Fixed(10.0);
        node.style.height = LayoutDimension::Fixed(10.0);
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(50.0)];
    container.rows = vec![GridTrackSize::LengthPx(10.0), GridTrackSize::LengthPx(20.0)];
    container.auto_rows = GridAutoRows::TrackList(auto_rows);
    container.placement_mode = GridPlacementMode::RootHtmlImplicitRowsV1;
    input
}

fn wrap_grid_with_fixed_parent(input: &mut crate::LayoutInput, width: f32, height: f32) {
    let grid = input.nodes.remove(0);
    let parent = LayoutNode {
        id: id(100),
        children: vec![grid.id],
        style: LayoutStyle {
            width: LayoutDimension::Fixed(width),
            height: LayoutDimension::Fixed(height),
            ..LayoutStyle::default()
        },
    };
    input.root = id(100);
    input.nodes.insert(0, parent);
    input.nodes.insert(1, grid);
    input.viewport.width = width;
    input.viewport.height = height.max(1.0);
}

fn y_positions(input: &crate::LayoutInput, nodes: &[u64]) -> Vec<f32> {
    let output = TaffyLayoutEngine.compute(input).unwrap();
    nodes
        .iter()
        .map(|node| output.frames[&id(*node)].y)
        .collect()
}

#[test]
fn root_html_calc_auto_row_list_resolves_pattern_from_first_implicit_row() {
    let mut input = grid_with_row_pattern(
        8,
        100.0,
        3.0,
        vec![
            GridTrackSize::Calc(FIRST_CALC),
            GridTrackSize::Percentage(0.25),
            GridTrackSize::LengthPx(15.0),
        ],
    );
    input.css_math = vec![calc_value(
        FIRST_CALC,
        LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(0.1),
            LayoutCssMath::LengthPx(2.0),
        ]),
    )];

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    let frames = (2..=9)
        .map(|node| {
            let frame = output.frames[&id(node)];
            [frame.x, frame.y, frame.width, frame.height]
        })
        .collect::<Vec<_>>();
    assert_eq!(
        frames,
        vec![
            [0.0, 0.0, 10.0, 10.0],
            [0.0, 13.0, 10.0, 10.0],
            [0.0, 36.0, 10.0, 10.0],
            [0.0, 51.0, 10.0, 10.0],
            [0.0, 79.0, 10.0, 10.0],
            [0.0, 97.0, 10.0, 10.0],
            [0.0, 112.0, 10.0, 10.0],
            [0.0, 140.0, 10.0, 10.0],
        ]
    );
    assert_eq!(
        output.frames[&id(1)],
        crate::LayoutFrame {
            x: 0.0,
            y: 0.0,
            width: 50.0,
            height: 100.0,
        }
    );
}

#[test]
fn root_html_calc_auto_rows_clamp_negative_and_preserve_large_tracks() {
    let negative_math = LayoutCssMath::Sum(vec![
        LayoutCssMath::LengthPx(1.0),
        LayoutCssMath::Negate(Box::new(LayoutCssMath::Percentage(0.1))),
    ]);
    let mut negative = grid_with_row_pattern(4, 100.0, 3.0, vec![GridTrackSize::Calc(FIRST_CALC)]);
    negative.css_math = vec![calc_value(FIRST_CALC, negative_math)];
    assert_eq!(y_positions(&negative, &[4, 5]), [36.0, 39.0]);

    let mut large = grid_with_row_pattern(4, 100.0, 3.0, vec![GridTrackSize::Calc(FIRST_CALC)]);
    large.css_math = vec![calc_value(
        FIRST_CALC,
        LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(1.2),
            LayoutCssMath::LengthPx(1.0),
        ]),
    )];
    assert_eq!(y_positions(&large, &[4, 5]), [36.0, 160.0]);
}

#[test]
fn root_html_calc_auto_rows_use_zero_height_basis_and_keep_row_gap_separate() {
    let mut zero = grid_with_row_pattern(3, 0.0, 0.0, vec![GridTrackSize::Calc(FIRST_CALC)]);
    zero.grid_containers.get_mut(&id(1)).unwrap().rows = vec![GridTrackSize::LengthPx(0.0)];
    zero.css_math = vec![calc_value(
        FIRST_CALC,
        LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(0.1),
            LayoutCssMath::LengthPx(5.0),
        ]),
    )];
    wrap_grid_with_fixed_parent(&mut zero, 50.0, 100.0);
    assert_eq!(y_positions(&zero, &[3, 4]), [0.0, 5.0]);

    let mut gap = grid_with_row_pattern(3, 100.0, 10.0, vec![GridTrackSize::Calc(FIRST_CALC)]);
    gap.grid_containers.get_mut(&id(1)).unwrap().rows = vec![GridTrackSize::LengthPx(0.0)];
    gap.css_math = vec![calc_value(
        FIRST_CALC,
        LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(0.1),
            LayoutCssMath::LengthPx(5.0),
        ]),
    )];
    assert_eq!(y_positions(&gap, &[3, 4]), [10.0, 35.0]);
}

#[test]
fn root_html_calc_auto_rows_compose_with_fraction_tracks() {
    let mut input = grid_with_row_pattern(
        5,
        100.0,
        5.0,
        vec![
            GridTrackSize::Calc(FIRST_CALC),
            GridTrackSize::Fraction(1.0),
        ],
    );
    input.css_math = vec![calc_value(
        FIRST_CALC,
        LayoutCssMath::Sum(vec![
            LayoutCssMath::Percentage(0.1),
            LayoutCssMath::LengthPx(5.0),
        ]),
    )];
    assert_eq!(
        y_positions(&input, &[2, 3, 4, 5, 6]),
        [0.0, 15.0, 40.0, 60.0, 85.0]
    );
}

#[test]
fn root_html_calc_auto_rows_reject_bad_bindings_and_recover_after_failure() {
    let valid = calc_value(
        FIRST_CALC,
        LayoutCssMath::Sum(vec![
            LayoutCssMath::LengthPx(2.0),
            LayoutCssMath::Percentage(0.1),
        ]),
    );
    let missing = grid_with_row_pattern(4, 100.0, 3.0, vec![GridTrackSize::Calc(FIRST_CALC)]);
    assert!(matches!(
        TaffyLayoutEngine.compute(&missing),
        Err(crate::LayoutError::MissingCssMath {
            property: "grid-auto-rows",
            ..
        })
    ));

    for (mut value, expect_duplicate) in [(valid.clone(), false), (valid.clone(), true)] {
        value.property = if expect_duplicate {
            LayoutCssMathProperty::GridTemplateRows
        } else {
            LayoutCssMathProperty::GridTemplateColumns
        };
        let mut input = grid_with_row_pattern(4, 100.0, 3.0, vec![GridTrackSize::Calc(FIRST_CALC)]);
        input.css_math = if expect_duplicate {
            vec![valid.clone(), value]
        } else {
            vec![value]
        };
        let result = TaffyLayoutEngine.compute(&input);
        if expect_duplicate {
            assert!(matches!(
                result,
                Err(crate::LayoutError::DuplicateCssMathId)
            ));
        } else {
            assert!(matches!(
                result,
                Err(crate::LayoutError::CssMathBindingMismatch {
                    property: "grid-auto-rows",
                    ..
                })
            ));
        }
    }

    let mut wrong_owner =
        grid_with_row_pattern(4, 100.0, 3.0, vec![GridTrackSize::Calc(FIRST_CALC)]);
    let mut wrong_owner_value = valid.clone();
    wrong_owner_value.node_id = id_fn(2);
    wrong_owner.css_math = vec![wrong_owner_value];
    assert!(matches!(
        TaffyLayoutEngine.compute(&wrong_owner),
        Err(crate::LayoutError::CssMathBindingMismatch {
            property: "grid-auto-rows",
            ..
        })
    ));

    let mut recovered = grid_with_row_pattern(4, 100.0, 3.0, vec![GridTrackSize::Calc(FIRST_CALC)]);
    recovered.css_math = vec![valid];
    assert_eq!(y_positions(&recovered, &[4]), [36.0]);
}

#[test]
fn root_html_calc_auto_rows_require_unconstrained_definite_height() {
    for (change, property) in [
        (0, "height"),
        (1, "min-height"),
        (2, "max-height"),
        (3, "aspect-ratio"),
    ] {
        let mut input = grid_with_row_pattern(4, 100.0, 3.0, vec![GridTrackSize::Calc(FIRST_CALC)]);
        input.css_math = vec![calc_value(
            FIRST_CALC,
            LayoutCssMath::Sum(vec![
                LayoutCssMath::LengthPx(2.0),
                LayoutCssMath::Percentage(0.1),
            ]),
        )];
        match change {
            0 => input.nodes[0].style.height = LayoutDimension::Auto,
            1 => input.nodes[0].style.min_height = LayoutDimension::Fixed(1.0),
            2 => input.nodes[0].style.max_height = LayoutDimension::Fixed(101.0),
            3 => input.nodes[0].style.aspect_ratio = Some(1.0),
            _ => unreachable!(),
        }
        wrap_grid_with_fixed_parent(&mut input, 50.0, 100.0);
        let result = TaffyLayoutEngine.compute(&input);
        assert!(
            matches!(
                result,
                Err(crate::LayoutError::UnsupportedGridContainerStyle {
                    property: actual,
                    ..
                }) if actual == property
            ),
            "{property} should reject calc percentage rows"
        );
    }
}

#[test]
fn root_html_calc_auto_rows_check_aggregate_gap_extent_before_layout() {
    let mut input =
        grid_with_row_pattern(4, 100.0, f32::MAX, vec![GridTrackSize::Calc(FIRST_CALC)]);
    input.css_math = vec![calc_value(
        FIRST_CALC,
        LayoutCssMath::Sum(vec![
            LayoutCssMath::LengthPx(2.0),
            LayoutCssMath::Percentage(0.1),
        ]),
    )];
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::InvalidGridTrack {
            property: "grid-auto-rows",
            ..
        })
    ));
}

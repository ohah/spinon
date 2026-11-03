use super::{id, root_html_column_flow_with_auto_columns};
use crate::{
    GridAutoColumns, GridTrackSize, LayoutCalcId, LayoutCssMath, LayoutCssMathProperty,
    LayoutCssMathValue, LayoutDimension, LayoutDisplay, LayoutEngine, LayoutNode, LayoutStyle,
    TaffyLayoutEngine,
};

fn calc_value(expression: LayoutCssMath) -> LayoutCssMathValue {
    LayoutCssMathValue {
        id: LayoutCalcId(7_100),
        node_id: id(1),
        property: LayoutCssMathProperty::GridTemplateColumns,
        expression,
    }
}

fn width_plus_percentage(width: f32, percentage: f32) -> LayoutCssMath {
    LayoutCssMath::Sum(vec![
        LayoutCssMath::LengthPx(width),
        LayoutCssMath::Percentage(percentage),
    ])
}

fn with_calc(expression: LayoutCssMath, item_count: usize) -> crate::LayoutInput {
    let mut input = root_html_column_flow_with_auto_columns(
        item_count,
        GridAutoColumns::TrackList(vec![GridTrackSize::Calc(LayoutCalcId(7_100))]),
    );
    input.css_math = vec![calc_value(expression)];
    input
}

fn x_positions(input: &crate::LayoutInput, nodes: &[u64]) -> Vec<f32> {
    let output = TaffyLayoutEngine.compute(input).unwrap();
    nodes
        .iter()
        .map(|node| output.frames[&id(*node)].x)
        .collect()
}

fn wrap_grid(input: &mut crate::LayoutInput, width: f32) {
    let grid = input.nodes.remove(0);
    let wrapper = LayoutNode {
        id: id(100),
        children: vec![id(1)],
        style: LayoutStyle {
            display: LayoutDisplay::Block,
            width: LayoutDimension::Fixed(width),
            height: LayoutDimension::Fixed(63.0),
            ..LayoutStyle::default()
        },
    };
    input.root = id(100);
    input.nodes.insert(0, wrapper);
    input.nodes.insert(1, grid);
    input.viewport.width = width;
}

#[test]
fn root_html_column_flow_resolves_calc_auto_columns_against_content_width() {
    let mut input = root_html_column_flow_with_auto_columns(
        12,
        GridAutoColumns::TrackList(vec![
            GridTrackSize::Calc(LayoutCalcId(7_100)),
            GridTrackSize::Calc(LayoutCalcId(7_101)),
        ]),
    );
    input.nodes[0].style.width = LayoutDimension::Fixed(240.0);
    input.viewport.width = 240.0;
    input.css_math = vec![
        calc_value(width_plus_percentage(5.0, 0.1)),
        LayoutCssMathValue {
            id: LayoutCalcId(7_101),
            node_id: id(1),
            property: LayoutCssMathProperty::GridTemplateColumns,
            expression: width_plus_percentage(2.0, 0.02),
        },
    ];
    assert_eq!(
        x_positions(&input, &[2, 4, 6, 8, 10, 12]),
        [0.0, 35.0, 80.0, 114.0, 125.8, 159.8]
    );
}

#[test]
fn root_html_column_flow_clamps_negative_calc_and_keeps_over_100_percent_extent() {
    let mut negative = with_calc(
        LayoutCssMath::Sum(vec![
            LayoutCssMath::LengthPx(1.0),
            LayoutCssMath::Negate(Box::new(LayoutCssMath::Percentage(0.1))),
        ]),
        12,
    );
    negative.nodes[0].style.width = LayoutDimension::Fixed(240.0);
    negative.viewport.width = 240.0;
    assert_eq!(
        x_positions(&negative, &[6, 8, 10, 12]),
        [80.0, 85.0, 90.0, 95.0]
    );

    let mut large = with_calc(width_plus_percentage(1.0, 1.2), 12);
    large.nodes[0].style.width = LayoutDimension::Fixed(240.0);
    large.viewport.width = 240.0;
    assert_eq!(
        x_positions(&large, &[6, 8, 10, 12]),
        [80.0, 374.0, 668.0, 962.0]
    );
}

#[test]
fn root_html_column_flow_calc_combines_with_fraction_and_accepts_zero_basis() {
    let mut mixed = root_html_column_flow_with_auto_columns(
        12,
        GridAutoColumns::TrackList(vec![
            GridTrackSize::Calc(LayoutCalcId(7_100)),
            GridTrackSize::Fraction(1.0),
        ]),
    );
    mixed.nodes[0].style.width = LayoutDimension::Fixed(240.0);
    mixed.viewport.width = 240.0;
    mixed.css_math = vec![calc_value(width_plus_percentage(5.0, 0.1))];
    assert_eq!(
        x_positions(&mixed, &[6, 8, 10, 12]),
        [80.0, 114.0, 162.5, 196.5]
    );

    let mut zero = with_calc(width_plus_percentage(5.0, 0.1), 12);
    zero.nodes[0].style.width = LayoutDimension::Fixed(0.0);
    wrap_grid(&mut zero, 175.0);
    assert_eq!(
        x_positions(&zero, &[6, 8, 10, 12]),
        [80.0, 90.0, 100.0, 110.0]
    );
}

#[test]
fn root_html_column_flow_calc_requires_definite_unconstrained_width_and_valid_math_binding() {
    let valid = calc_value(width_plus_percentage(5.0, 0.1));
    let mut input = with_calc(valid.expression.clone(), 8);
    input.nodes[0].style.width = LayoutDimension::Auto;
    wrap_grid(&mut input, 175.0);
    let result = TaffyLayoutEngine.compute(&input);
    assert!(
        matches!(
            &result,
            Err(crate::LayoutError::UnsupportedGridContainerStyle {
                property: "width",
                ..
            })
        ),
        "{result:?}"
    );

    for (property, mutate) in [
        (
            "min-width",
            Box::new(|style: &mut LayoutStyle| style.min_width = LayoutDimension::Fixed(1.0))
                as Box<dyn Fn(&mut LayoutStyle)>,
        ),
        (
            "max-width",
            Box::new(|style: &mut LayoutStyle| style.max_width = LayoutDimension::Fixed(200.0))
                as Box<dyn Fn(&mut LayoutStyle)>,
        ),
        (
            "aspect-ratio",
            Box::new(|style: &mut LayoutStyle| style.aspect_ratio = Some(1.0))
                as Box<dyn Fn(&mut LayoutStyle)>,
        ),
    ] {
        let mut constrained = with_calc(valid.expression.clone(), 8);
        mutate(&mut constrained.nodes[0].style);
        assert!(matches!(
            TaffyLayoutEngine.compute(&constrained),
            Err(crate::LayoutError::UnsupportedGridContainerStyle { property: found, .. })
                if found == property
        ));
    }

    let mut missing = with_calc(valid.expression.clone(), 8);
    missing.css_math.clear();
    assert!(matches!(
        TaffyLayoutEngine.compute(&missing),
        Err(crate::LayoutError::MissingCssMath {
            property: "grid-auto-columns",
            ..
        })
    ));

    let mut duplicate = with_calc(valid.expression.clone(), 8);
    duplicate.css_math.push(valid.clone());
    assert!(matches!(
        TaffyLayoutEngine.compute(&duplicate),
        Err(crate::LayoutError::DuplicateCssMathId)
    ));

    let mut wrong_owner = with_calc(valid.expression.clone(), 8);
    wrong_owner.css_math[0].node_id = id(2);
    assert!(matches!(
        TaffyLayoutEngine.compute(&wrong_owner),
        Err(crate::LayoutError::CssMathBindingMismatch {
            property: "grid-auto-columns",
            ..
        })
    ));

    let mut wrong_property = with_calc(valid.expression.clone(), 8);
    wrong_property.css_math[0].property = LayoutCssMathProperty::GridTemplateRows;
    assert!(matches!(
        TaffyLayoutEngine.compute(&wrong_property),
        Err(crate::LayoutError::CssMathBindingMismatch {
            property: "grid-auto-columns",
            ..
        })
    ));

    let mut unsupported = with_calc(
        LayoutCssMath::Sum(vec![
            LayoutCssMath::LengthPx(5.0),
            LayoutCssMath::Product(vec![LayoutCssMath::Percentage(0.1)]),
        ]),
        8,
    );
    assert!(matches!(
        TaffyLayoutEngine.compute(&unsupported),
        Err(crate::LayoutError::UnsupportedGridAutoColumns { .. })
    ));
    unsupported.css_math[0] = valid;
    assert!(TaffyLayoutEngine.compute(&unsupported).is_ok());
}

#[test]
fn root_html_column_flow_calc_rejects_cumulative_f32_extent_overflow() {
    let mut input = with_calc(width_plus_percentage(5.0, 0.1), 8);
    input.nodes[0].style.width = LayoutDimension::Fixed(240.0);
    input.viewport.width = 240.0;
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(f32::MAX); 2];
    assert!(matches!(
        TaffyLayoutEngine.compute(&input),
        Err(crate::LayoutError::InvalidGridTrack {
            property: "grid-template-columns",
            ..
        })
    ));
}

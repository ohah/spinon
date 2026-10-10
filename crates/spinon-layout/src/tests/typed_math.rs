use super::*;
use crate::{
    LayoutCalcId, LayoutCssMath, LayoutCssMathProperty, LayoutCssMathValue,
    calc_tree::CalcLayoutTree,
};
use spinon_core::{EnvironmentRevision, Revision, StyleRevision};

#[test]
fn non_finite_css_math_intermediate_is_censored_before_a_frame_is_returned() {
    let root = node_id(901);
    let child = node_id(902);
    let expression = LayoutCssMath::Product(vec![
        LayoutCssMath::LengthPx(10.0),
        LayoutCssMath::Invert(Box::new(LayoutCssMath::Number(0.0))),
    ]);
    let input = input_with_child(
        root,
        child,
        LayoutDimension::Calc(LayoutCalcId(1)),
        vec![math_value(child, LayoutCssMathProperty::Width, expression)],
    );

    let output = TaffyLayoutEngine.compute(&input).unwrap();
    assert_eq!(output.frames[&child].width, 33_554_428.0);
    assert!(output.frames.values().all(|frame| {
        [frame.x, frame.y, frame.width, frame.height]
            .into_iter()
            .all(f32::is_finite)
    }));
}

#[test]
fn dimension_error_discards_the_entire_layout_result() {
    let root = node_id(911);
    let child = node_id(912);
    let expression = LayoutCssMath::Sum(vec![
        LayoutCssMath::Number(1.0),
        LayoutCssMath::LengthPx(2.0),
    ]);
    let input = input_with_child(
        root,
        child,
        LayoutDimension::Calc(LayoutCalcId(2)),
        vec![math_value(child, LayoutCssMathProperty::Width, expression)],
    );

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::InvalidCssMath {
            node: child,
            property: "width",
            reason: "sum 연산자의 CSS 계산 차원이 서로 다릅니다",
        })
    );
}

#[test]
fn mismatched_math_owner_and_property_fail_before_taffy_callback() {
    let root = node_id(921);
    let child = node_id(922);
    let input = input_with_child(
        root,
        child,
        LayoutDimension::Calc(LayoutCalcId(3)),
        vec![math_value(
            root,
            LayoutCssMathProperty::Width,
            LayoutCssMath::LengthPx(4.0),
        )],
    );

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::CssMathBindingMismatch {
            node: child,
            property: "width",
            id: LayoutCalcId(3),
        })
    );
}

#[test]
fn duplicate_math_ids_fail_before_taffy_callback() {
    let root = node_id(931);
    let child = node_id(932);
    let values = vec![
        LayoutCssMathValue {
            id: LayoutCalcId(4),
            node_id: child,
            property: LayoutCssMathProperty::Width,
            expression: LayoutCssMath::LengthPx(4.0),
        },
        LayoutCssMathValue {
            id: LayoutCalcId(4),
            node_id: child,
            property: LayoutCssMathProperty::Height,
            expression: LayoutCssMath::LengthPx(5.0),
        },
    ];
    let input = input_with_child(root, child, LayoutDimension::Calc(LayoutCalcId(4)), values);

    assert_eq!(
        TaffyLayoutEngine.compute(&input),
        Err(LayoutError::DuplicateCssMathId)
    );
}

#[test]
fn every_min_max_math_dimension_requires_a_property_matched_binding() {
    let root = node_id(935);
    let child = node_id(936);
    for (property, id) in [
        (LayoutCssMathProperty::MinWidth, LayoutCalcId(41)),
        (LayoutCssMathProperty::MaxWidth, LayoutCalcId(42)),
        (LayoutCssMathProperty::MinHeight, LayoutCalcId(43)),
        (LayoutCssMathProperty::MaxHeight, LayoutCalcId(44)),
    ] {
        let mut input = input_with_child(root, child, LayoutDimension::Fixed(30.0), vec![]);
        match property {
            LayoutCssMathProperty::MinWidth => {
                input.nodes[1].style.min_width = LayoutDimension::Calc(id)
            }
            LayoutCssMathProperty::MaxWidth => {
                input.nodes[1].style.max_width = LayoutDimension::Calc(id)
            }
            LayoutCssMathProperty::MinHeight => {
                input.nodes[1].style.min_height = LayoutDimension::Calc(id)
            }
            LayoutCssMathProperty::MaxHeight => {
                input.nodes[1].style.max_height = LayoutDimension::Calc(id)
            }
            _ => unreachable!("최소·최대 속성만 나열했습니다"),
        }
        assert_eq!(
            TaffyLayoutEngine.compute(&input),
            Err(LayoutError::MissingCssMath {
                node: child,
                property: property.name(),
                id,
            })
        );
    }
}

#[test]
fn unknown_taffy_handle_is_recorded_instead_of_dereferenced() {
    let root = node_id(941);
    let input = LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 800.0,
        },
        root_sizing: RootSizingPolicy::Match,
        nodes: vec![LayoutNode {
            id: root,
            children: vec![],
            style: LayoutStyle {
                width: LayoutDimension::Fixed(320.0),
                height: LayoutDimension::Fixed(800.0),
                display: crate::LayoutDisplay::Block,
                ..LayoutStyle::default()
            },
        }],
        css_math: vec![],
        positioning: Default::default(),
    };
    let index = BTreeMap::from([(root, 0)]);
    let mut tree = CalcLayoutTree::new(&input, &index, &[root]).unwrap();

    assert_eq!(
        <CalcLayoutTree as taffy::LayoutPartialTree>::resolve_calc_value(
            &tree,
            std::ptr::null(),
            1.0,
        ),
        0.0
    );
    assert_eq!(
        tree.take_calc_error(),
        Some(LayoutError::UnknownCssMathHandle)
    );
}

fn math_value(
    node_id: NodeId,
    property: LayoutCssMathProperty,
    expression: LayoutCssMath,
) -> LayoutCssMathValue {
    LayoutCssMathValue {
        id: LayoutCalcId(match property {
            LayoutCssMathProperty::Width => 1,
            LayoutCssMathProperty::Height => 2,
            _ => 3,
        }),
        node_id,
        property,
        expression,
    }
}

fn input_with_child(
    root: NodeId,
    child: NodeId,
    child_width: LayoutDimension,
    mut css_math: Vec<LayoutCssMathValue>,
) -> LayoutInput {
    for value in &mut css_math {
        if value.property == LayoutCssMathProperty::Width {
            value.id = match child_width {
                LayoutDimension::Calc(id) => id,
                _ => unreachable!("테스트 입력은 계산식 width를 사용합니다"),
            };
        }
    }
    LayoutInput {
        root,
        revision: LayoutInputRevision::new(
            LayoutSourceRevision::Tree(Revision::default()),
            StyleRevision::default(),
            EnvironmentRevision::default(),
        ),
        viewport: Viewport {
            width: 320.0,
            height: 800.0,
        },
        root_sizing: RootSizingPolicy::Match,
        positioning: Default::default(),
        nodes: vec![
            LayoutNode {
                id: root,
                children: vec![child],
                style: LayoutStyle {
                    width: LayoutDimension::Fixed(320.0),
                    height: LayoutDimension::Fixed(800.0),
                    display: crate::LayoutDisplay::Block,
                    ..LayoutStyle::default()
                },
            },
            LayoutNode {
                id: child,
                children: vec![],
                style: LayoutStyle {
                    width: child_width,
                    height: LayoutDimension::Fixed(10.0),
                    display: crate::LayoutDisplay::Block,
                    ..LayoutStyle::default()
                },
            },
        ],
        css_math,
    }
}

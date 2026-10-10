use crate::{FlexDirection, LayoutDisplay, LayoutLengthPercentage, LayoutStyle};
use taffy::prelude::Layout;

/// Taffy 0.14는 음수 교차축 여유 공간을 `auto` 시작 여백에 배정합니다.
/// CSS Flexbox는 이때 시작 여백을 0으로 두고 넘치는 크기를 끝 방향으로 보냅니다.
pub(super) fn negative_cross_axis_auto_margin_offset_correction(
    parent: LayoutStyle,
    child: LayoutStyle,
    child_layout: Layout,
) -> (f32, f32) {
    if parent.display != LayoutDisplay::Flex {
        return (0.0, 0.0);
    }

    match parent.flex_direction {
        FlexDirection::Row | FlexDirection::RowReverse
            if child.margin.top == LayoutLengthPercentage::Auto
                && child_layout.margin.top < 0.0 =>
        {
            (0.0, -child_layout.margin.top)
        }
        FlexDirection::Column | FlexDirection::ColumnReverse
            if parent.direction == crate::TextDirection::Ltr
                && child.margin.left == LayoutLengthPercentage::Auto
                && child_layout.margin.left < 0.0 =>
        {
            (-child_layout.margin.left, 0.0)
        }
        _ => (0.0, 0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::negative_cross_axis_auto_margin_offset_correction;
    use crate::{
        FlexDirection, LayoutAlignItems, LayoutAlignSelf, LayoutDimension, LayoutDisplay,
        LayoutEdges, LayoutEngine, LayoutInput, LayoutInputRevision, LayoutLengthPercentage,
        LayoutNode, LayoutSourceRevision, LayoutStyle, RootSizingPolicy, TaffyLayoutEngine,
        TextDirection, Viewport,
    };
    use spinon_core::{EnvironmentRevision, NodeId, Revision, StyleRevision};
    use taffy::prelude::Layout;

    fn flex_parent(direction: FlexDirection) -> LayoutStyle {
        LayoutStyle {
            display: LayoutDisplay::Flex,
            flex_direction: direction,
            direction: TextDirection::Ltr,
            ..LayoutStyle::default()
        }
    }

    #[test]
    fn corrects_negative_row_cross_start_margin() {
        let child = LayoutStyle {
            margin: LayoutEdges {
                top: LayoutLengthPercentage::Auto,
                ..LayoutEdges::default()
            },
            ..LayoutStyle::default()
        };
        let layout = Layout {
            margin: taffy::geometry::Rect {
                top: -10.0,
                ..taffy::geometry::Rect::zero()
            },
            ..Layout::new()
        };

        assert_eq!(
            negative_cross_axis_auto_margin_offset_correction(
                flex_parent(FlexDirection::Row),
                child,
                layout,
            ),
            (0.0, 10.0)
        );
    }

    #[test]
    fn corrects_negative_column_cross_start_margin() {
        let child = LayoutStyle {
            margin: LayoutEdges {
                left: LayoutLengthPercentage::Auto,
                ..LayoutEdges::default()
            },
            ..LayoutStyle::default()
        };
        let layout = Layout {
            margin: taffy::geometry::Rect {
                left: -7.0,
                ..taffy::geometry::Rect::zero()
            },
            ..Layout::new()
        };

        assert_eq!(
            negative_cross_axis_auto_margin_offset_correction(
                flex_parent(FlexDirection::Column),
                child,
                layout,
            ),
            (7.0, 0.0)
        );
    }

    #[test]
    fn does_not_correct_end_main_axis_or_nonnegative_margins() {
        let parent = flex_parent(FlexDirection::Row);
        let bottom_auto = LayoutStyle {
            margin: LayoutEdges {
                bottom: LayoutLengthPercentage::Auto,
                ..LayoutEdges::default()
            },
            ..LayoutStyle::default()
        };
        let negative_bottom = Layout {
            margin: taffy::geometry::Rect {
                bottom: -10.0,
                ..taffy::geometry::Rect::zero()
            },
            ..Layout::new()
        };
        let main_axis_auto = LayoutStyle {
            margin: LayoutEdges {
                left: LayoutLengthPercentage::Auto,
                ..LayoutEdges::default()
            },
            ..LayoutStyle::default()
        };
        let positive_top = Layout {
            margin: taffy::geometry::Rect {
                top: 10.0,
                ..taffy::geometry::Rect::zero()
            },
            ..Layout::new()
        };

        assert_eq!(
            negative_cross_axis_auto_margin_offset_correction(parent, bottom_auto, negative_bottom),
            (0.0, 0.0)
        );
        assert_eq!(
            negative_cross_axis_auto_margin_offset_correction(parent, main_axis_auto, positive_top),
            (0.0, 0.0)
        );
        assert_eq!(
            negative_cross_axis_auto_margin_offset_correction(
                LayoutStyle {
                    display: LayoutDisplay::Block,
                    ..parent
                },
                bottom_auto,
                negative_bottom,
            ),
            (0.0, 0.0)
        );
    }

    #[test]
    fn full_layout_corrects_overflowing_cross_axis_auto_margins() {
        for (flex_direction, parent_width, parent_height, child_width, child_height, margin) in [
            (
                FlexDirection::Row,
                60.0,
                10.0,
                20.0,
                20.0,
                LayoutEdges {
                    top: LayoutLengthPercentage::Auto,
                    ..LayoutEdges::default()
                },
            ),
            (
                FlexDirection::Row,
                60.0,
                10.0,
                20.0,
                20.0,
                LayoutEdges {
                    top: LayoutLengthPercentage::Auto,
                    bottom: LayoutLengthPercentage::Auto,
                    ..LayoutEdges::default()
                },
            ),
            (
                FlexDirection::Column,
                10.0,
                60.0,
                20.0,
                20.0,
                LayoutEdges {
                    left: LayoutLengthPercentage::Auto,
                    ..LayoutEdges::default()
                },
            ),
        ] {
            let root = NodeId::new(1).unwrap();
            let item = NodeId::new(2).unwrap();
            let input = LayoutInput {
                root,
                revision: LayoutInputRevision::new(
                    LayoutSourceRevision::Tree(Revision::default()),
                    StyleRevision::default(),
                    EnvironmentRevision::default(),
                ),
                viewport: Viewport {
                    width: 100.0,
                    height: 100.0,
                },
                root_sizing: RootSizingPolicy::ResolveWithin,
                css_math: vec![],
                nodes: vec![
                    LayoutNode {
                        id: root,
                        children: vec![item],
                        style: LayoutStyle {
                            display: LayoutDisplay::Flex,
                            width: LayoutDimension::Fixed(parent_width),
                            height: LayoutDimension::Fixed(parent_height),
                            flex_direction,
                            align_items: LayoutAlignItems::Center,
                            ..LayoutStyle::default()
                        },
                    },
                    LayoutNode {
                        id: item,
                        children: vec![],
                        style: LayoutStyle {
                            width: LayoutDimension::Fixed(child_width),
                            height: LayoutDimension::Fixed(child_height),
                            margin,
                            align_self: LayoutAlignSelf::FlexEnd,
                            ..LayoutStyle::default()
                        },
                    },
                ],
            };

            let frame = TaffyLayoutEngine.compute(&input).unwrap().frames[&item];
            assert_eq!(
                (frame.x, frame.y, frame.width, frame.height),
                (0.0, 0.0, child_width, child_height),
                "{flex_direction:?} {margin:?}"
            );
        }
    }
}

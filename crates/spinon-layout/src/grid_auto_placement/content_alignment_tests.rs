use super::*;
use crate::{
    AlignmentSafety, ContentAlignmentPosition, JustifyContentPosition, LayoutAlignContent,
    LayoutJustifyContent,
};

const CHROME_CSS_PIXEL_TOLERANCE: f32 = 1.0 / 64.0;

fn fixed_root_html_grid(width: f32, height: f32) -> LayoutInput {
    let mut input = sparse_grid(2, 2, 4);
    input.nodes[0].style.width = LayoutDimension::Fixed(width);
    input.nodes[0].style.height = LayoutDimension::Fixed(height);
    input.nodes[0].style.gap = LayoutGap {
        column: LayoutLengthPercentage::length(10.0),
        row: LayoutLengthPercentage::length(10.0),
    };
    input.viewport.width = width;
    input.viewport.height = height;
    for child in input.nodes.iter_mut().skip(1) {
        child.style.width = LayoutDimension::Fixed(10.0);
        child.style.height = LayoutDimension::Fixed(10.0);
    }
    let container = input.grid_containers.get_mut(&id(1)).unwrap();
    container.columns = vec![GridTrackSize::LengthPx(40.0), GridTrackSize::LengthPx(60.0)];
    container.rows = vec![GridTrackSize::LengthPx(30.0), GridTrackSize::LengthPx(50.0)];
    container.auto_rows = GridAutoRows::Auto;
    container.auto_columns = GridAutoColumns::Auto;
    container.placement_mode = GridPlacementMode::Existing;
    input
}

fn assert_css_px(actual: f32, expected: f32, context: &str) {
    assert!(
        (actual - expected).abs() <= CHROME_CSS_PIXEL_TOLERANCE,
        "{context}: expected {expected}px ± {CHROME_CSS_PIXEL_TOLERANCE}px, got {actual}px"
    );
}

#[test]
fn fixed_root_html_grid_justify_content_positions_tracks_on_positive_space() {
    for (value, expected) in [
        (LayoutJustifyContent::Normal, [0.0, 50.0]),
        (LayoutJustifyContent::Stretch, [0.0, 50.0]),
        (
            LayoutJustifyContent::Position {
                position: JustifyContentPosition::Start,
                safety: None,
            },
            [0.0, 50.0],
        ),
        (LayoutJustifyContent::FlexStart, [0.0, 50.0]),
        (
            LayoutJustifyContent::Position {
                position: JustifyContentPosition::End,
                safety: None,
            },
            [130.0, 180.0],
        ),
        (LayoutJustifyContent::FlexEnd, [130.0, 180.0]),
        (LayoutJustifyContent::Center, [65.0, 115.0]),
        (LayoutJustifyContent::SpaceBetween, [0.0, 180.0]),
        (LayoutJustifyContent::SpaceAround, [32.5, 147.5]),
        (
            LayoutJustifyContent::SpaceEvenly,
            [130.0 / 3.0, 410.0 / 3.0],
        ),
    ] {
        let mut input = fixed_root_html_grid(240.0, 180.0);
        input.nodes[0].style.justify_content = value;
        let result = TaffyLayoutEngine.compute(&input).unwrap();
        assert_css_px(
            result.frames[&id(2)].x,
            expected[0],
            &format!("{value:?} first"),
        );
        assert_css_px(
            result.frames[&id(3)].x,
            expected[1],
            &format!("{value:?} second"),
        );
    }
}

#[test]
fn fixed_root_html_grid_align_content_positions_tracks_on_positive_space() {
    for (value, expected) in [
        (LayoutAlignContent::Normal, [0.0, 40.0]),
        (LayoutAlignContent::Stretch, [0.0, 40.0]),
        (
            LayoutAlignContent::Position {
                position: ContentAlignmentPosition::Start,
                safety: None,
            },
            [0.0, 40.0],
        ),
        (LayoutAlignContent::FlexStart, [0.0, 40.0]),
        (
            LayoutAlignContent::Position {
                position: ContentAlignmentPosition::End,
                safety: None,
            },
            [90.0, 130.0],
        ),
        (LayoutAlignContent::FlexEnd, [90.0, 130.0]),
        (LayoutAlignContent::Center, [45.0, 85.0]),
        (LayoutAlignContent::SpaceBetween, [0.0, 130.0]),
        (LayoutAlignContent::SpaceAround, [22.5, 107.5]),
        (LayoutAlignContent::SpaceEvenly, [30.0, 100.0]),
    ] {
        let mut input = fixed_root_html_grid(240.0, 180.0);
        input.nodes[0].style.align_content = Some(value);
        let result = TaffyLayoutEngine.compute(&input).unwrap();
        assert_css_px(
            result.frames[&id(2)].y,
            expected[0],
            &format!("{value:?} first"),
        );
        assert_css_px(
            result.frames[&id(4)].y,
            expected[1],
            &format!("{value:?} second"),
        );
    }
}

#[test]
fn safe_and_unsafe_content_alignment_handle_negative_free_space() {
    for (value, expected) in [
        (LayoutJustifyContent::Center, -15.0),
        (
            LayoutJustifyContent::Position {
                position: JustifyContentPosition::Center,
                safety: Some(AlignmentSafety::Safe),
            },
            0.0,
        ),
        (
            LayoutJustifyContent::Position {
                position: JustifyContentPosition::Center,
                safety: Some(AlignmentSafety::Unsafe),
            },
            -15.0,
        ),
        (
            LayoutJustifyContent::Position {
                position: JustifyContentPosition::End,
                safety: None,
            },
            -30.0,
        ),
        (
            LayoutJustifyContent::Position {
                position: JustifyContentPosition::End,
                safety: Some(AlignmentSafety::Safe),
            },
            0.0,
        ),
    ] {
        let mut input = fixed_root_html_grid(80.0, 70.0);
        input.nodes[0].style.justify_content = value;
        let result = TaffyLayoutEngine.compute(&input).unwrap();
        assert_css_px(result.frames[&id(2)].x, expected, &format!("{value:?}"));
    }

    for (value, expected) in [
        (LayoutAlignContent::Center, -10.0),
        (
            LayoutAlignContent::Position {
                position: ContentAlignmentPosition::Center,
                safety: Some(AlignmentSafety::Safe),
            },
            0.0,
        ),
        (
            LayoutAlignContent::Position {
                position: ContentAlignmentPosition::Center,
                safety: Some(AlignmentSafety::Unsafe),
            },
            -10.0,
        ),
        (
            LayoutAlignContent::Position {
                position: ContentAlignmentPosition::End,
                safety: None,
            },
            -20.0,
        ),
        (
            LayoutAlignContent::Position {
                position: ContentAlignmentPosition::End,
                safety: Some(AlignmentSafety::Safe),
            },
            0.0,
        ),
    ] {
        let mut input = fixed_root_html_grid(80.0, 70.0);
        input.nodes[0].style.align_content = Some(value);
        let result = TaffyLayoutEngine.compute(&input).unwrap();
        assert_css_px(result.frames[&id(2)].y, expected, &format!("{value:?}"));
    }
}

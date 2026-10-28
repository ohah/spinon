use spinon_layout::LayoutGridTrackSizingAxis;
use spinon_style::ComputedCssOverflow;

use super::COLUMNS_GRID_HEIGHT;

#[derive(Clone, Copy)]
pub(super) enum MinimumSource {
    ContentBased,
    Overflow,
    Specified,
    TrackMinimum,
}

#[derive(Clone, Copy)]
pub(super) enum ItemKind {
    Text,
    Image,
}

#[derive(Clone, Copy)]
pub(in crate::session::ua_cascade::runtime_layout) struct Case {
    pub(super) scene_id: &'static str,
    pub(super) root_id: &'static str,
    pub(super) axis: LayoutGridTrackSizingAxis,
    pub(super) item_kind: ItemKind,
    pub(super) authored_overflow_x: &'static str,
    pub(super) authored_overflow_y: &'static str,
    pub(super) computed_overflow_x: ComputedCssOverflow,
    pub(super) computed_overflow_y: ComputedCssOverflow,
    pub(super) zero_item_minimum: bool,
    pub(super) zero_track_minimum: bool,
    pub(super) expected_track_base_size: f32,
    pub(super) expected_item_width: f32,
    pub(super) expected_item_height: f32,
    pub(super) expected_minimum_value: Option<f32>,
    pub(super) minimum_source: MinimumSource,
}

macro_rules! column_case {
    ($suffix:literal, $overflow:literal, $zero_item:literal, $zero_track:literal,
     $track:literal, $minimum:expr, $source:ident, $computed_x:expr, $computed_y:expr) => {
        Case {
            scene_id: concat!("column-", $suffix),
            root_id: concat!("c114-auto-min-scrollable-overflow-column-", $suffix),
            axis: LayoutGridTrackSizingAxis::Columns,
            item_kind: ItemKind::Text,
            authored_overflow_x: $overflow,
            authored_overflow_y: "visible",
            computed_overflow_x: $computed_x,
            computed_overflow_y: $computed_y,
            zero_item_minimum: $zero_item,
            zero_track_minimum: $zero_track,
            expected_track_base_size: $track,
            expected_item_width: $track,
            expected_item_height: COLUMNS_GRID_HEIGHT,
            expected_minimum_value: $minimum,
            minimum_source: MinimumSource::$source,
        }
    };
}

macro_rules! row_case {
    ($suffix:literal, $overflow:literal, $zero_item:literal, $zero_track:literal,
     $track:literal, $minimum:expr, $source:ident, $computed_x:expr, $computed_y:expr) => {
        Case {
            scene_id: concat!("row-", $suffix),
            root_id: concat!("c114-auto-min-scrollable-overflow-row-", $suffix),
            axis: LayoutGridTrackSizingAxis::Rows,
            item_kind: ItemKind::Image,
            authored_overflow_x: "visible",
            authored_overflow_y: $overflow,
            computed_overflow_x: $computed_x,
            computed_overflow_y: $computed_y,
            zero_item_minimum: $zero_item,
            zero_track_minimum: $zero_track,
            expected_track_base_size: $track,
            expected_item_width: 10.0,
            expected_item_height: $track,
            expected_minimum_value: $minimum,
            minimum_source: MinimumSource::$source,
        }
    };
}

pub(super) const CASES: [Case; 14] = [
    column_case!(
        "visible-content-minimum",
        "visible",
        false,
        false,
        100.0,
        Some(100.0),
        ContentBased,
        ComputedCssOverflow::Visible,
        ComputedCssOverflow::Visible
    ),
    column_case!(
        "clip-content-minimum",
        "clip",
        false,
        false,
        100.0,
        Some(100.0),
        ContentBased,
        ComputedCssOverflow::Clip,
        ComputedCssOverflow::Visible
    ),
    column_case!(
        "hidden-zero-minimum",
        "hidden",
        false,
        false,
        60.0,
        Some(0.0),
        Overflow,
        ComputedCssOverflow::Hidden,
        ComputedCssOverflow::Auto
    ),
    column_case!(
        "auto-zero-minimum",
        "auto",
        false,
        false,
        60.0,
        Some(0.0),
        Overflow,
        ComputedCssOverflow::Auto,
        ComputedCssOverflow::Auto
    ),
    column_case!(
        "scroll-zero-minimum",
        "scroll",
        false,
        false,
        60.0,
        Some(0.0),
        Overflow,
        ComputedCssOverflow::Scroll,
        ComputedCssOverflow::Auto
    ),
    column_case!(
        "specified-minimum-control",
        "visible",
        true,
        false,
        60.0,
        Some(0.0),
        Specified,
        ComputedCssOverflow::Visible,
        ComputedCssOverflow::Visible
    ),
    column_case!(
        "zero-track-control",
        "visible",
        false,
        true,
        60.0,
        None,
        TrackMinimum,
        ComputedCssOverflow::Visible,
        ComputedCssOverflow::Visible
    ),
    row_case!(
        "visible-content-minimum",
        "visible",
        false,
        false,
        10.0,
        Some(10.0),
        ContentBased,
        ComputedCssOverflow::Visible,
        ComputedCssOverflow::Visible
    ),
    row_case!(
        "clip-content-minimum",
        "clip",
        false,
        false,
        10.0,
        Some(10.0),
        ContentBased,
        ComputedCssOverflow::Visible,
        ComputedCssOverflow::Clip
    ),
    row_case!(
        "hidden-zero-minimum",
        "hidden",
        false,
        false,
        5.0,
        Some(0.0),
        Overflow,
        ComputedCssOverflow::Auto,
        ComputedCssOverflow::Hidden
    ),
    row_case!(
        "auto-zero-minimum",
        "auto",
        false,
        false,
        5.0,
        Some(0.0),
        Overflow,
        ComputedCssOverflow::Auto,
        ComputedCssOverflow::Auto
    ),
    row_case!(
        "scroll-zero-minimum",
        "scroll",
        false,
        false,
        5.0,
        Some(0.0),
        Overflow,
        ComputedCssOverflow::Auto,
        ComputedCssOverflow::Scroll
    ),
    row_case!(
        "specified-minimum-control",
        "visible",
        true,
        false,
        5.0,
        Some(0.0),
        Specified,
        ComputedCssOverflow::Visible,
        ComputedCssOverflow::Visible
    ),
    row_case!(
        "zero-track-control",
        "visible",
        false,
        true,
        5.0,
        None,
        TrackMinimum,
        ComputedCssOverflow::Visible,
        ComputedCssOverflow::Visible
    ),
];

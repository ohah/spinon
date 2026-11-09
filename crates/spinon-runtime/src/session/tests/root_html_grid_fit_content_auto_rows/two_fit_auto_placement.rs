use super::*;
use serde_json::Value;
use spinon_render::OpaqueCssSrgb;
use spinon_style::{ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize};

#[path = "two_fit_auto_placement/controls.rs"]
mod controls;
#[path = "two_fit_auto_placement/matrix.rs"]
mod matrix;
#[path = "two_fit_auto_placement/rejections.rs"]
mod rejections;

pub(super) const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-two-fit-content-runtime.html"
);
pub(super) const OBSERVATIONS: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-two-fit-content-precomparison-2026-11-10/chrome-matrix-observation.ndjson"
);
pub(super) const BASE_TRACK_LIST: &str =
    "fit-content(10px) min-content fit-content(20%) max-content auto min-content";
pub(super) const ITEM_COLORS: [OpaqueCssSrgb; 9] = [
    OpaqueCssSrgb::new(229, 72, 77),
    OpaqueCssSrgb::new(242, 102, 94),
    OpaqueCssSrgb::new(91, 168, 245),
    OpaqueCssSrgb::new(85, 199, 145),
    OpaqueCssSrgb::new(168, 85, 247),
    OpaqueCssSrgb::new(250, 204, 21),
    OpaqueCssSrgb::new(14, 165, 233),
    OpaqueCssSrgb::new(251, 146, 60),
    OpaqueCssSrgb::new(45, 212, 191),
];
pub(super) const PAYLOAD_TEXTS: [&str; 8] = [
    "A A A A A A",
    "AAAA AAAA A",
    "AA AA AA AA",
    "AAA AAA AAA",
    "AAA AAA",
    "A A A A A A",
    "A A A A A A A A",
    "AA",
];

pub(super) fn scenes() -> Vec<Value> {
    let records = OBSERVATIONS
        .lines()
        .map(|line| {
            serde_json::from_str::<Value>(line).expect("Chrome record should be valid JSON")
        })
        .collect::<Vec<_>>();
    assert_eq!(records.len(), 2_881, "one metadata record and 2,880 scenes");
    assert_eq!(records[0]["record"].as_str(), Some("metadata"));
    assert_eq!(records[0]["sceneCount"].as_u64(), Some(2_880));
    records.into_iter().skip(1).collect()
}

pub(super) fn source_for(scene: &Value) -> String {
    let track_list = scene["scene"]["trackList"]
        .as_str()
        .expect("Chrome scene track list should be a string");
    let align = scene["scene"]["alignContent"]
        .as_str()
        .expect("Chrome scene alignment should be a string");
    let source = replace_once(SOURCE, BASE_TRACK_LIST, track_list);
    replace_once(
        &source,
        "align-content: start;",
        &format!("align-content: {align};"),
    )
}

pub(super) fn computed_tracks(scene: &Value) -> Vec<ComputedCssGridTrackSize> {
    scene["scene"]["trackMembers"]
        .as_array()
        .expect("Chrome track members should be an array")
        .iter()
        .map(|member| {
            let member = member.as_str().expect("track member should be a string");
            match member {
                "min-content" => ComputedCssGridTrackSize::MinContent,
                "max-content" => ComputedCssGridTrackSize::MaxContent,
                "auto" => ComputedCssGridTrackSize::Auto,
                value if value.starts_with("fit-content(") && value.ends_with(')') => {
                    let cap = &value[12..value.len() - 1];
                    let cap = if let Some(percent) = cap.strip_suffix('%') {
                        ComputedCssGridTrackLengthPercentage::Percentage(
                            percent.parse::<f32>().expect("percentage cap should parse") / 100.0,
                        )
                    } else {
                        ComputedCssGridTrackLengthPercentage::LengthPx(
                            cap.strip_suffix("px")
                                .expect("length cap should use px")
                                .parse::<f32>()
                                .expect("px cap should parse"),
                        )
                    };
                    ComputedCssGridTrackSize::FitContent(cap)
                }
                other => panic!("unsupported Chrome track member {other:?}"),
            }
        })
        .collect()
}

pub(super) fn frame(value: &Value) -> [f32; 4] {
    let values = value.as_array().expect("expected frame should be an array");
    assert_eq!(values.len(), 4);
    std::array::from_fn(|index| {
        values[index]
            .as_f64()
            .expect("frame component should be numeric") as f32
    })
}

pub(super) fn runtime_frame(frame: &crate::session::RuntimeLayoutFrame) -> [f32; 4] {
    [frame.x, frame.y, frame.width, frame.height]
}

pub(super) fn frame_for_background(
    completed: &crate::session::RuntimeLayoutCompleted,
    styles: &[spinon_style::ComputedElementStyle],
    expected: OpaqueCssSrgb,
    label: &str,
) -> [f32; 4] {
    let matches = styles
        .iter()
        .filter(|style| {
            style.background_color.is_some_and(|color| {
                (color.red, color.green, color.blue)
                    == (expected.red(), expected.green(), expected.blue())
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "{label}: unique computed background");
    let node_id = matches[0].node_id.get();
    completed
        .frames
        .iter()
        .find(|frame| frame.node_id == node_id)
        .map(runtime_frame)
        .unwrap_or_else(|| panic!("{label}: computed style node {node_id} has a frame"))
}

pub(super) fn assert_frame(actual: [f32; 4], expected: [f32; 4], label: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.01,
            "{label} component {component}: actual={actual:?}, expected={expected:?} ± 0.01 CSS px"
        );
    }
}

pub(super) fn frame_for_payload(
    completed: &crate::session::RuntimeLayoutCompleted,
    node_id: u64,
    label: &str,
) -> [f32; 4] {
    completed
        .frames
        .iter()
        .find(|frame| frame.node_id == node_id)
        .map(runtime_frame)
        .unwrap_or_else(|| panic!("{label}: payload node {node_id} has a frame"))
}

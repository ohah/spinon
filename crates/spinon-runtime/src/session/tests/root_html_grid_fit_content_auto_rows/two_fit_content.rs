use super::*;
use serde_json::Value;
use spinon_render::OpaqueCssSrgb;
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize,
};
use std::collections::{BTreeMap, BTreeSet};

const OBSERVATIONS: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-content-precomparison-2026-11-09/chrome-observation.ndjson"
);
const VERIFICATION: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-content-precomparison-2026-11-09/verification.json"
);
const MIN_CONTENT_FLOOR_CONTROL: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-two-fit-content-runtime-2026-11-09/chrome157-min-content-floor-control.json"
);
const EMPTY_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(17, 23, 31);
const A_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(31, 41, 51);
const B_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(61, 71, 81);
const C_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(91, 101, 111);
const D_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(101, 111, 121);
const E_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(111, 121, 131);
const A_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(121, 131, 141);
const B_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(151, 161, 171);
const C_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(181, 191, 201);
const D_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(191, 201, 211);
const E_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(201, 211, 221);

type FramesByItem = BTreeMap<String, [f32; 4]>;

fn frame_for_computed_background(
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
    let frame = completed
        .frames
        .iter()
        .find(|frame| frame.node_id == node_id)
        .unwrap_or_else(|| panic!("{label}: computed style node {node_id} has a frame"));
    [frame.x, frame.y, frame.width, frame.height]
}

fn observations() -> Vec<Value> {
    OBSERVATIONS
        .lines()
        .map(|line| serde_json::from_str(line).expect("Chrome observation is valid JSON"))
        .collect()
}

fn frame(value: &Value) -> [f32; 4] {
    let values = value.as_array().expect("frame is an array");
    assert_eq!(values.len(), 4);
    std::array::from_fn(|index| values[index].as_f64().unwrap() as f32)
}

fn text_for(id: &str) -> &'static str {
    match id {
        "A" => "AAAA BBBB CCCC DDDD EEEE FFFF",
        "B" => "AAAAAA BBBBB CCCC DDD",
        "C" => "AAAAA BBBB CCC DDDD",
        "D" => "AAAA BBB CCCCC DDDD",
        "E" => "AAA BBBB CCCCCC",
        _ => panic!("unexpected text item {id:?}"),
    }
}

fn item_color(id: &str) -> (OpaqueCssSrgb, &'static str) {
    match id {
        "empty" => (EMPTY_ITEM, "rgb(17, 23, 31)"),
        "A" => (A_ITEM, "rgb(31, 41, 51)"),
        "B" => (B_ITEM, "rgb(61, 71, 81)"),
        "C" => (C_ITEM, "rgb(91, 101, 111)"),
        "D" => (D_ITEM, "rgb(101, 111, 121)"),
        "E" => (E_ITEM, "rgb(111, 121, 131)"),
        _ => panic!("unexpected item id {id:?}"),
    }
}

fn payload_color(id: &str) -> (OpaqueCssSrgb, &'static str) {
    match id {
        "A" => (A_PAYLOAD, "rgb(121, 131, 141)"),
        "B" => (B_PAYLOAD, "rgb(151, 161, 171)"),
        "C" => (C_PAYLOAD, "rgb(181, 191, 201)"),
        "D" => (D_PAYLOAD, "rgb(191, 201, 211)"),
        "E" => (E_PAYLOAD, "rgb(201, 211, 221)"),
        _ => panic!("unexpected payload id {id:?}"),
    }
}

fn source_for(scene: &Value) -> String {
    let track_list = scene["trackList"].as_str().expect("trackList is a string");
    let align = scene["alignContent"]
        .as_str()
        .expect("alignContent is a string");
    let mut stylesheet = String::from(
        "* { box-sizing: border-box; } html, body { margin: 0; padding: 0; }\
         .grid { display: grid; width: 40px; height: 220px; grid-template-columns: 40px; grid-template-rows: 10px;\
         grid-auto-flow: row; align-items: normal; justify-content: start; justify-items: normal; direction: ltr;\
         overflow: visible; margin: 0; padding: 0; row-gap: 0; column-gap: 0; }\
         .item { display: block; width: auto; height: auto; min-height: 0; margin: 0; padding: 0;\
         overflow: visible; grid-column: 1 / 2; }\
         .empty { height: 10px; }\
         .payload { display: block; width: auto; height: auto; margin: 0; padding: 0; overflow: visible;\
         font-family: monospace; font-size: 10px; font-weight: 400; font-style: normal; line-height: 10px;\
         letter-spacing: 0; word-spacing: 0; white-space: normal; direction: ltr; }",
    );
    stylesheet.push_str(&format!(
        ".item.empty {{ background-color: {}; }}",
        item_color("empty").1
    ));
    for id in ["A", "B", "C", "D", "E"] {
        let (_, item_background) = item_color(id);
        let (_, payload_background) = payload_color(id);
        stylesheet.push_str(&format!(
            ".item[data-item=\"{id}\"] {{ background-color: {item_background}; }}\
             .item[data-item=\"{id}\"] .payload {{ background-color: {payload_background}; }}"
        ));
    }
    let mut source = format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><style>{stylesheet}</style></head><body><div class=\"grid\" style=\"grid-auto-rows:{track_list};align-content:{align}\">\
         <div class=\"item empty\" data-item=\"empty\" style=\"grid-row:1 / 2\"></div>"
    );
    for entry in scene["items"].as_array().expect("span items is an array") {
        let values = entry.as_array().expect("span item is a tuple");
        let id = values[0].as_str().expect("item id is a string");
        let row_start = values[1].as_u64().expect("row start is an integer");
        let span = values[2].as_u64().expect("row span is an integer");
        source.push_str(&format!(
            "<div class=\"item\" data-item=\"{id}\" style=\"grid-row:{row_start} / span {span}\"><div class=\"payload\" data-payload=\"{id}\">{}</div></div>",
            text_for(id)
        ));
    }
    source.push_str("</div></body></html>");
    source
}

fn computed_track(value: &str) -> ComputedCssGridTrackSize {
    if let Some(cap) = value
        .strip_prefix("fit-content(")
        .and_then(|value| value.strip_suffix(')'))
    {
        let cap = if let Some(value) = cap.strip_suffix('%') {
            ComputedCssGridTrackLengthPercentage::Percentage(
                value.parse::<f32>().expect("percentage cap is numeric") / 100.0,
            )
        } else {
            ComputedCssGridTrackLengthPercentage::LengthPx(
                cap.strip_suffix("px")
                    .expect("length cap is px")
                    .parse::<f32>()
                    .expect("length cap is numeric"),
            )
        };
        return ComputedCssGridTrackSize::FitContent(cap);
    }
    match value {
        "min-content" => ComputedCssGridTrackSize::MinContent,
        "max-content" => ComputedCssGridTrackSize::MaxContent,
        "auto" => ComputedCssGridTrackSize::Auto,
        other => panic!("unexpected track member {other:?}"),
    }
}

fn payload_text_descriptors(
    render: &spinon_render::RuntimeRenderSnapshot,
) -> Vec<(usize, f32, usize)> {
    render
        .texts()
        .iter()
        .map(|text| {
            let baselines = text
                .glyphs()
                .iter()
                .map(spinon_render::RuntimeGlyphBitmap::baseline_y_px)
                .collect::<BTreeSet<_>>();
            (text.glyphs().len(), text.origin_y_css_px(), baselines.len())
        })
        .collect()
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_track_trace(
    completed: &crate::session::RuntimeLayoutCompleted,
    expected: &Value,
    label: &str,
) -> Vec<f32> {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("fixture feature exposes bounded track diagnostics");
    assert_eq!(diagnostics.traces.len(), 1, "{label}: one Grid trace");
    let trace = &diagnostics.traces[0];
    assert_eq!(trace.used_rows.explicit_track_count, 1, "{label}");
    assert_eq!(trace.used_rows.negative_implicit_track_count, 0, "{label}");
    assert_eq!(trace.used_rows.positive_implicit_track_count, 6, "{label}");
    assert!(
        trace.used_rows.collapsed_track_indices.is_empty(),
        "{label}"
    );
    let pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("final rows sizing pass exists");
    let stage = pass.stages.last().expect("final rows stage exists");
    let actual = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .collect::<Vec<_>>();
    let expected_rows = expected["rowValues"]
        .as_array()
        .expect("Chrome rowValues is an array");
    assert_eq!(actual.len(), 7, "{label}: explicit plus six implicit rows");
    assert_eq!(
        actual.len(),
        expected_rows.len(),
        "{label}: trace row count"
    );
    for (index, (actual, expected)) in actual.iter().zip(expected_rows).enumerate() {
        assert_eq!(usize::from(actual.internal_index), index * 2 + 1, "{label}");
        assert!(
            !actual.is_collapsed,
            "{label}: row {index} is not collapsed"
        );
        let expected = expected.as_f64().unwrap() as f32;
        // This diagnostic is the raw Taffy f32 trace; item frames below verify the normalized
        // Blink 1/64px output, so the pre-snap trace may differ by at most one LayoutUnit.
        assert!(
            actual.base_size.is_finite()
                && (actual.base_size - expected).abs() <= (1.0 / 64.0 + 0.0001),
            "{label}: row {index}: raw Taffy={}, expected Chrome={expected} ± one 1/64px LayoutUnit",
            actual.base_size
        );
    }
    actual.iter().map(|track| track.base_size).collect()
}

#[path = "two_fit_content/controls.rs"]
mod controls;
#[path = "two_fit_content/matrix.rs"]
mod matrix;
#[path = "two_fit_content/rejections.rs"]
mod rejections;

use super::*;
use serde_json::Value;
use spinon_render::OpaqueCssSrgb;
use spinon_style::{ComputedCssGridTrackLengthPercentage, ComputedCssGridTrackSize};

const MATRIX: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-later-fit-content-row-span-precomparison-2026-11-09/chrome157-matrix-observation.ndjson"
);
const REPEATED_AUTO_PLACEMENT_SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-auto-rows-runtime.html"
);

const EMPTY_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(11, 17, 23);
const A_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(31, 41, 51);
const B_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(61, 71, 81);
const C_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(91, 101, 111);
const A_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(121, 131, 141);
const B_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(151, 161, 171);
const C_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(181, 191, 201);

fn scenes() -> (Value, Vec<Value>) {
    let mut lines = MATRIX.lines();
    let metadata = serde_json::from_str(lines.next().expect("matrix metadata is present"))
        .expect("matrix metadata is valid JSON");
    let scenes = lines
        .map(|line| serde_json::from_str(line).expect("matrix scene is valid JSON"))
        .collect();
    (metadata, scenes)
}

fn tail_css(scene: &Value) -> Vec<String> {
    scene["tail"]
        .as_array()
        .expect("tail is an array")
        .iter()
        .map(|track| track.as_str().expect("tail member is a string").to_owned())
        .collect()
}

fn frame(value: &Value) -> [f32; 4] {
    let values = value.as_array().expect("Chrome frame should be an array");
    assert_eq!(values.len(), 4);
    std::array::from_fn(|index| values[index].as_f64().unwrap() as f32)
}

fn track_list_for_position(scene: &Value, fit_position: usize) -> String {
    let mut tracks = tail_css(scene);
    let cap = scene["cap"].as_str().expect("cap is a string");
    tracks.insert(fit_position - 1, format!("fit-content({cap})"));
    tracks.join(" ")
}

fn payload_text(id: &str) -> &'static str {
    match id {
        "A" => "A A A A A A",
        "B" => "A A A A A A A A",
        "C" => "A A A A A A A A A A A A",
        _ => panic!("unexpected text payload {id:?}"),
    }
}

fn source_for(scene: &Value) -> String {
    source_with_track_list(scene, &scene_track_list(scene), None)
}

fn scene_track_list(scene: &Value) -> String {
    scene["trackList"]
        .as_array()
        .expect("track list is an array")
        .iter()
        .map(|track| track.as_str().expect("track is a string"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn source_with_track_list(
    scene: &Value,
    track_list: &str,
    dark_track_list: Option<&str>,
) -> String {
    let align = scene["align"].as_str().expect("alignment is a string");
    let mut children = String::new();
    for item in scene["items"].as_array().expect("items is an array") {
        let id = item["id"].as_str().expect("item id is a string");
        if id == "empty-0" {
            children
                .push_str("<div class=\"empty\" style=\"grid-row:1 / 2;grid-column:1 / 2\"></div>");
            continue;
        }
        let row_start = item["rowStart"].as_str().expect("row start is a string");
        let row_end = item["rowEnd"].as_str().expect("row end is a string");
        children.push_str(&format!(
            "<div class=\"item\" data-item=\"{id}\" style=\"grid-row:{row_start} / {row_end}\"><div class=\"payload\">{}</div></div>",
            payload_text(id)
        ));
    }
    let dark_rule = dark_track_list.map_or_else(String::new, |tracks| {
        format!("@media (prefers-color-scheme: dark) {{ #grid {{ grid-auto-rows: {tracks}; }} }}")
    });
    let style = format!(
        "* {{ box-sizing: border-box; }} html,body {{ margin: 0; padding: 0; }} #grid {{ display: grid; width: 40px; height: 220px; grid-template-columns: 40px; grid-template-rows: 10px; grid-auto-rows: {track_list}; grid-auto-flow: row; align-content: {align}; align-items: normal; justify-content: start; justify-items: normal; direction: ltr; overflow: visible; margin: 0; padding: 0; row-gap: 0; column-gap: 0; }} .item {{ display: block; width: auto; height: auto; min-height: 0; margin: 0; padding: 0; overflow: visible; grid-column: 1 / 2; }} .empty {{ height: 10px; background-color: rgb(11, 17, 23); }} .payload {{ display: block; width: auto; height: auto; margin: 0; padding: 0; font-family: monospace; font-size: 10px; font-weight: 400; font-style: normal; line-height: 10px; letter-spacing: 0; word-spacing: 0; white-space: normal; direction: ltr; }} .item[data-item=\"A\"] {{ background-color: rgb(31, 41, 51); }} .item[data-item=\"B\"] {{ background-color: rgb(61, 71, 81); }} .item[data-item=\"C\"] {{ background-color: rgb(91, 101, 111); }} .item[data-item=\"A\"] .payload {{ background-color: rgb(121, 131, 141); }} .item[data-item=\"B\"] .payload {{ background-color: rgb(151, 161, 171); }} .item[data-item=\"C\"] .payload {{ background-color: rgb(181, 191, 201); }} {dark_rule}"
    );
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><style>{style}</style></head><body><div id=\"grid\">{children}</div></body></html>"
    )
}

fn color_for_item(id: &str) -> OpaqueCssSrgb {
    match id {
        "empty-0" => EMPTY_ITEM,
        "A" => A_ITEM,
        "B" => B_ITEM,
        "C" => C_ITEM,
        _ => panic!("unexpected item id {id:?}"),
    }
}

fn color_for_payload(id: &str) -> OpaqueCssSrgb {
    match id {
        "A" => A_PAYLOAD,
        "B" => B_PAYLOAD,
        "C" => C_PAYLOAD,
        _ => panic!("unexpected payload id {id:?}"),
    }
}

fn computed_track(value: &str) -> ComputedCssGridTrackSize {
    if let Some(cap) = value
        .strip_prefix("fit-content(")
        .and_then(|value| value.strip_suffix(')'))
    {
        let length = if let Some(value) = cap.strip_suffix('%') {
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
        return ComputedCssGridTrackSize::FitContent(length);
    }
    match value {
        "min-content" => ComputedCssGridTrackSize::MinContent,
        "max-content" => ComputedCssGridTrackSize::MaxContent,
        "auto" => ComputedCssGridTrackSize::Auto,
        _ => panic!("unexpected track value {value:?}"),
    }
}

#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
fn assert_track_trace(
    completed: &crate::session::RuntimeLayoutCompleted,
    expected: &Value,
    label: &str,
) {
    let diagnostics = completed
        .grid_track_sizing_diagnostics
        .as_ref()
        .expect("track trace feature exposes bounded diagnostics");
    assert_eq!(diagnostics.traces.len(), 1, "{label}: one Grid trace");
    let trace = &diagnostics.traces[0];
    assert_eq!(trace.used_rows.explicit_track_count, 1, "{label}");
    assert_eq!(trace.used_rows.negative_implicit_track_count, 0, "{label}");
    assert_eq!(trace.used_rows.positive_implicit_track_count, 3, "{label}");
    assert!(
        trace.used_rows.collapsed_track_indices.is_empty(),
        "{label}"
    );
    let pass = trace
        .passes
        .iter()
        .filter(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
        .max_by_key(|pass| pass.sequence)
        .expect("row sizing pass exists");
    let stage = pass.stages.last().expect("final row stage exists");
    let actual = stage
        .tracks
        .iter()
        .filter(|track| !track.is_gutter)
        .collect::<Vec<_>>();
    let expected_rows = expected["rows"].as_array().expect("rows is an array");
    assert_eq!(actual.len(), 4, "{label}: explicit plus implicit rows");
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
        let expected = expected.as_f64().expect("row size is numeric") as f32;
        assert!(
            actual.base_size.is_finite() && (actual.base_size - expected).abs() <= 0.01,
            "{label}: trace row {index}: actual={}, expected={expected} ± 0.01px",
            actual.base_size
        );
    }
}

#[path = "later_fit_content_row_span/matrix.rs"]
mod matrix;
#[path = "later_fit_content_row_span/rejections.rs"]
mod rejections;

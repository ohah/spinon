use super::*;
use serde_json::Value;
use spinon_render::OpaqueCssSrgb;
use std::collections::BTreeMap;

const CAPTURE: &str = include_str!(
    "../../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6-precomparison-2026-11-09/chrome157-capture.json"
);

const EMPTY_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(11, 17, 23);
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

type SceneKey = (usize, String, String, String, String);
type FramesByItem = BTreeMap<String, [f32; 4]>;

fn capture() -> (Value, Vec<Value>) {
    let outer: Value = serde_json::from_str(CAPTURE).expect("capture wrapper is valid JSON");
    let parsed: Value = match outer {
        Value::String(serialized) => {
            serde_json::from_str(&serialized).expect("captured observation is valid JSON")
        }
        value => value,
    };
    let cases = parsed["cases"]
        .as_array()
        .expect("Chrome capture includes the scene matrix")
        .clone();
    (parsed, cases)
}

fn text_for(id: &str) -> &'static str {
    match id {
        "A" => "A A A A A A A A A A A A A A A A",
        "B" => "A A A A A A A A A A A A A A",
        "C" => "A A A A A A A A A A A A",
        "D" => "A A A A A A A A A A",
        "E" => "A A A A A A A A",
        _ => panic!("unexpected repeated text item {id:?}"),
    }
}

fn source_for_scene(scene: &Value, replacement_tracks: Option<&str>) -> String {
    let generated_tracks = scene["trackList"]
        .as_array()
        .expect("trackList is an array")
        .iter()
        .map(|track| track.as_str().expect("track member is a string"))
        .collect::<Vec<_>>()
        .join(" ");
    let tracks = replacement_tracks.unwrap_or(&generated_tracks);
    let align = scene["align"].as_str().expect("align is a string");
    let mut children = String::new();
    for item in scene["items"].as_array().expect("items is an array") {
        let id = item["id"].as_str().expect("item id is a string");
        if id == "empty-0" {
            children.push_str(
                "<div class=\"item empty\" style=\"grid-row:1 / 2;grid-column:1 / 2\"></div>",
            );
            continue;
        }
        let row_start = item["rowStart"].as_str().expect("rowStart is a string");
        let row_end = item["rowEnd"].as_str().expect("rowEnd is a string");
        children.push_str(&format!(
            "<div class=\"item\" data-item=\"{id}\" style=\"grid-row:{row_start} / {row_end}\"><div class=\"payload\">{}</div></div>",
            text_for(id)
        ));
    }
    let style = r#"
* { box-sizing: border-box; }
html, body { margin: 0; padding: 0; }
#grid {
  display: grid; width: 40px; height: 220px;
  grid-template-columns: 40px; grid-template-rows: 10px;
  grid-auto-rows: __TRACKS__; grid-auto-flow: row;
  row-gap: 0; column-gap: 0; align-content: __ALIGN__;
  align-items: normal; justify-content: start; justify-items: normal;
  direction: ltr; overflow: visible; margin: 0; padding: 0;
}
.item { display: block; width: auto; height: auto; min-height: 0; margin: 0; padding: 0; overflow: visible; grid-column: 1 / 2; }
.empty { height: 10px; background-color: rgb(11, 17, 23); }
.payload { display: block; width: auto; height: auto; margin: 0; padding: 0; font-family: monospace; font-size: 10px; font-weight: 400; font-style: normal; line-height: 10px; letter-spacing: 0; word-spacing: 0; white-space: normal; direction: ltr; }
.item[data-item="A"] { background-color: rgb(31, 41, 51); }
.item[data-item="B"] { background-color: rgb(61, 71, 81); }
.item[data-item="C"] { background-color: rgb(91, 101, 111); }
.item[data-item="D"] { background-color: rgb(101, 111, 121); }
.item[data-item="E"] { background-color: rgb(111, 121, 131); }
.item[data-item="A"] .payload { background-color: rgb(121, 131, 141); }
.item[data-item="B"] .payload { background-color: rgb(151, 161, 171); }
.item[data-item="C"] .payload { background-color: rgb(181, 191, 201); }
.item[data-item="D"] .payload { background-color: rgb(191, 201, 211); }
.item[data-item="E"] .payload { background-color: rgb(201, 211, 221); }
"#
    .replace("__TRACKS__", tracks)
    .replace("__ALIGN__", align);
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
        "D" => D_ITEM,
        "E" => E_ITEM,
        _ => panic!("unexpected item id {id:?}"),
    }
}

fn color_for_payload(id: &str) -> OpaqueCssSrgb {
    match id {
        "A" => A_PAYLOAD,
        "B" => B_PAYLOAD,
        "C" => C_PAYLOAD,
        "D" => D_PAYLOAD,
        "E" => E_PAYLOAD,
        _ => panic!("unexpected payload id {id:?}"),
    }
}

fn assert_frame_001(actual: [f32; 4], expected: [f32; 4], name: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.01,
            "{name} frame component {component}: actual={actual:?}, expected={expected:?} ± 0.01 CSS px"
        );
    }
}

fn frame(value: &Value) -> [f32; 4] {
    let values = value.as_array().expect("Chrome frame is an array");
    assert_eq!(values.len(), 4);
    std::array::from_fn(|index| values[index].as_f64().unwrap() as f32)
}

fn row_sizes(completed: &crate::session::RuntimeLayoutCompleted, expected: &Value) -> Vec<f32> {
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    {
        let diagnostics = completed
            .grid_track_sizing_diagnostics
            .as_ref()
            .expect("track trace feature exposes bounded diagnostics");
        assert_eq!(diagnostics.traces.len(), 1, "one Grid track trace");
        let trace = &diagnostics.traces[0];
        assert_eq!(trace.used_rows.explicit_track_count, 1);
        assert_eq!(trace.used_rows.negative_implicit_track_count, 0);
        assert_eq!(trace.used_rows.positive_implicit_track_count, 6);
        assert!(trace.used_rows.collapsed_track_indices.is_empty());
        let pass = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
            .max_by_key(|pass| pass.sequence)
            .expect("row sizing pass exists");
        let stage = pass.stages.last().expect("final row stage exists");
        let stage_summary = pass
            .stages
            .iter()
            .map(|stage| {
                (
                    stage.stage,
                    stage
                        .tracks
                        .iter()
                        .filter(|track| !track.is_gutter)
                        .map(|track| (track.base_size, track.growth_limit))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        let actual = stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .collect::<Vec<_>>();
        assert_eq!(actual.len(), 7, "explicit row plus six implicit rows");
        let expected_rows = expected["rows"].as_array().expect("rows is an array");
        assert_eq!(actual.len(), expected_rows.len());
        let actual_sizes = actual
            .iter()
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        let expected_sizes = expected_rows
            .iter()
            .map(|value| value.as_f64().unwrap() as f32)
            .collect::<Vec<_>>();
        let topology = expected["topology"].as_str().unwrap();
        let track_list = &expected["trackList"];
        for (index, (actual, expected)) in actual.iter().zip(expected_rows).enumerate() {
            assert_eq!(usize::from(actual.internal_index), index * 2 + 1);
            assert!(!actual.is_collapsed, "row {index} is not collapsed");
            let expected = expected.as_f64().unwrap() as f32;
            assert!(
                actual.base_size.is_finite() && (actual.base_size - expected).abs() <= 0.01,
                "topology={topology} tracks={track_list} row {index}: actual sizes={actual_sizes:?}, expected={expected_sizes:?}; stages={stage_summary:?}; contributions={:?}",
                pass.contributions
            );
        }
        actual.into_iter().map(|track| track.base_size).collect()
    }
    #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
    {
        let _ = completed;
        expected["rows"]
            .as_array()
            .expect("rows is an array")
            .iter()
            .map(|value| value.as_f64().unwrap() as f32)
            .collect()
    }
}

fn scene_key(scene: &Value) -> SceneKey {
    (
        scene["fitPosition"].as_u64().unwrap() as usize,
        serde_json::to_string(&scene["tail"]).unwrap(),
        scene["cap"].as_str().unwrap().to_owned(),
        scene["align"].as_str().unwrap().to_owned(),
        scene["topology"].as_str().unwrap().to_owned(),
    )
}

#[path = "positions_4_6/matrix.rs"]
mod matrix;
#[path = "positions_4_6/rejections.rs"]
mod rejections;

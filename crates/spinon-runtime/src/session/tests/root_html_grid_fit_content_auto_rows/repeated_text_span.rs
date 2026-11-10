use super::*;
use serde_json::Value;
use spinon_render::OpaqueCssSrgb;
use spinon_style::{
    ComputedCssGridAutoRows, ComputedCssGridPlacement, ComputedCssGridTrackLengthPercentage,
    ComputedCssGridTrackSize,
};

#[cfg(all(test, feature = "c11-4-grid-track-sizing-fixture-test"))]
#[path = "repeated_text_span/automatic_minimum.rs"]
mod automatic_minimum;
#[cfg(test)]
#[path = "repeated_text_span/rejections.rs"]
mod rejections;
#[cfg(all(test, feature = "c11-4-grid-track-sizing-fixture-test"))]
#[path = "repeated_text_span/three_fit_auto_minimum.rs"]
mod three_fit_auto_minimum;

const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-precomparison.html"
);
const SIX_ROW_SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-positions-4-6-auto-minimum-precomparison.html"
);
const MATRIX: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-precomparison-2026-11-09/chrome157-matrix-observation.ndjson"
);
const APP_SENTINEL_SOURCE: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-implementation-2026-11-09/root-html-span-sentinel.html"
);

const EMPTY_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(11, 17, 23);
const A_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(31, 41, 51);
const B_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(61, 71, 81);
const C_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(91, 101, 111);
const D_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(211, 81, 111);
const E_ITEM: OpaqueCssSrgb = OpaqueCssSrgb::new(241, 121, 151);
const A_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(121, 131, 141);
const B_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(151, 161, 171);
const C_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(181, 191, 201);
const D_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(21, 181, 211);
const E_PAYLOAD: OpaqueCssSrgb = OpaqueCssSrgb::new(91, 221, 111);

fn frame(value: &Value) -> [f32; 4] {
    let values = value.as_array().expect("Chrome frame should be an array");
    assert_eq!(values.len(), 4);
    std::array::from_fn(|index| values[index].as_f64().unwrap() as f32)
}

fn assert_frame(actual: [f32; 4], expected: [f32; 4], label: &str) {
    for (component, (actual, expected)) in actual.into_iter().zip(expected).enumerate() {
        assert!(
            actual.is_finite() && (actual - expected).abs() <= 0.01,
            "{label} component {component}: actual={actual:?}, expected={expected:?} ± 0.01 CSS px"
        );
    }
}

fn color_for_item(id: &str) -> OpaqueCssSrgb {
    match id {
        "empty-0" | "empty" => EMPTY_ITEM,
        "A" => A_ITEM,
        "B" => B_ITEM,
        "C" => C_ITEM,
        "D" => D_ITEM,
        "E" => E_ITEM,
        _ => panic!("unexpected Chrome item id {id:?}"),
    }
}

fn color_for_payload(id: &str) -> OpaqueCssSrgb {
    match id {
        "A" => A_PAYLOAD,
        "B" => B_PAYLOAD,
        "C" => C_PAYLOAD,
        "D" => D_PAYLOAD,
        "E" => E_PAYLOAD,
        _ => panic!("unexpected Chrome payload id {id:?}"),
    }
}

fn source_for(scene: &Value) -> String {
    let scene = scene
        .get("scene")
        .filter(|scene| scene.is_object())
        .unwrap_or(scene);
    let topology = scene["topology"]
        .as_str()
        .expect("topology should be a string");
    let section = if topology.contains("-six-") {
        assert!(
            SIX_ROW_SOURCE.contains(&format!("\"{topology}\": [")),
            "six-row fixture topology not found: {topology}"
        );
        let empty = "<div class=\"item empty\" data-item=\"empty\" style=\"grid-row:1 / 2\"></div>";
        let items = scene["items"]
            .as_array()
            .expect("six-row scene should record item placements")
            .iter()
            .map(|item| {
                let id = item[0].as_str().expect("item id should be a string");
                let start = item[1].as_u64().expect("row start should be positive");
                let span = item[2].as_u64().expect("row span should be positive");
                let word_count = match id {
                    "A" => 16,
                    "B" => 14,
                    "C" => 12,
                    "D" => 10,
                    "E" => 8,
                    other => panic!("unsupported six-row text item {other:?}"),
                };
                let text = vec!["A"; word_count].join(" ");
                format!(
                    "<div class=\"item\" data-item=\"{id}\" style=\"grid-row:{start} / span {span}\"><div class=\"payload\">{text}</div></div>"
                )
            })
            .collect::<Vec<_>>()
            .join("");
        format!(
            "<div class=\"scene\" data-scene=\"{topology}\"><div class=\"grid\">{empty}{items}</div></div>"
        )
    } else {
        let marker = format!("<section class=\"scene\" data-scene=\"{topology}\">");
        let section_start = SOURCE
            .find(&marker)
            .unwrap_or_else(|| panic!("fixture topology not found: {topology}"));
        let section_source = &SOURCE[section_start..];
        let section_end = section_source
            .find("</section>")
            .expect("fixture section should close")
            + "</section>".len();
        section_source[..section_end]
            .replace("<section", "<div")
            .replace("</section>", "</div>")
    };

    let tail = scene["tail"]
        .as_array()
        .expect("tail order should be an array")
        .iter()
        .map(|track| track.as_str().unwrap())
        .collect::<Vec<_>>();
    let cap = scene["cap"].as_str().expect("cap should be a string");
    let fit_position = match scene.get("fitPosition") {
        Some(value) => usize::try_from(
            value
                .as_u64()
                .expect("fit position should be an unsigned integer"),
        )
        .expect("fit position should fit usize"),
        None => 1,
    };
    assert!(
        (1..=6).contains(&fit_position),
        "fit position should be supported"
    );
    let mut auto_rows = tail.into_iter().map(str::to_owned).collect::<Vec<_>>();
    auto_rows.insert(fit_position - 1, format!("fit-content({cap})"));
    let auto_rows = auto_rows.join(" ");
    let align = scene["align"]
        .as_str()
        .or_else(|| scene["alignContent"].as_str())
        .expect("alignment should be a string");
    let min_height = scene["minHeight"].as_str().unwrap_or("0");
    let explicit_minimum = if min_height == "auto" {
        "min-height:0; "
    } else {
        ""
    };
    let style = format!(
        "* {{ box-sizing: border-box; }}\n\
         html, body {{ margin: 0; padding: 0; }}\n\
         .scene {{ margin: 0; }}\n\
         .grid {{ display: grid; width: 40px; height: 220px; grid-template-columns: 40px; grid-template-rows: 10px; grid-auto-rows: {auto_rows}; grid-auto-flow: row; align-content: {align}; align-items: normal; justify-content: start; justify-items: normal; overflow: visible; margin: 0; padding: 0; row-gap: 0; column-gap: 0; }}\n\
         .item {{ display: block; width: auto; height: auto; min-height: {min_height}; margin: 0; padding: 0; overflow: visible; grid-column: 1 / 2; }}\n\
         .empty {{ {explicit_minimum}height: 10px; grid-row: 1 / 2; background-color: rgb(11, 17, 23); }}\n\
         .payload {{ display: block; width: auto; height: auto; margin: 0; padding: 0; font-family: monospace; font-size: 10px; font-weight: 400; font-style: normal; line-height: 10px; letter-spacing: 0; word-spacing: 0; white-space: normal; direction: ltr; }}\n\
         .item[data-item=\"A\"] {{ background-color: rgb(31, 41, 51); }}\n\
         .item[data-item=\"B\"] {{ background-color: rgb(61, 71, 81); }}\n\
         .item[data-item=\"C\"] {{ background-color: rgb(91, 101, 111); }}\n\
         .item[data-item=\"D\"] {{ background-color: rgb(211, 81, 111); }}\n\
         .item[data-item=\"E\"] {{ background-color: rgb(241, 121, 151); }}\n\
         .item[data-item=\"A\"] .payload {{ background-color: rgb(121, 131, 141); }}\n\
         .item[data-item=\"B\"] .payload {{ background-color: rgb(151, 161, 171); }}\n\
         .item[data-item=\"C\"] .payload {{ background-color: rgb(181, 191, 201); }}\n\
         .item[data-item=\"D\"] .payload {{ background-color: rgb(21, 181, 211); }}\n\
         .item[data-item=\"E\"] .payload {{ background-color: rgb(91, 221, 111); }}"
    );
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><style>{style}</style></head><body>{section}</body></html>"
    )
}

fn without_unused_extended_item_color_rules(source: &str) -> String {
    [
        ".item[data-item=\"D\"] { background-color: rgb(211, 81, 111); }\n",
        ".item[data-item=\"E\"] { background-color: rgb(241, 121, 151); }\n",
        ".item[data-item=\"D\"] .payload { background-color: rgb(21, 181, 211); }\n",
        ".item[data-item=\"E\"] .payload { background-color: rgb(91, 221, 111); }",
    ]
    .into_iter()
    .fold(source.to_owned(), |normalized, rule| {
        normalized.replace(rule, "")
    })
    .replace("\n</style>", "</style>")
}

fn assert_optional_track_trace(
    completed: &crate::session::RuntimeLayoutCompleted,
    expected: &Value,
    label: &str,
) {
    #[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]
    {
        let diagnostics = completed
            .grid_track_sizing_diagnostics
            .as_ref()
            .expect("track trace feature should expose bounded diagnostics");
        assert_eq!(diagnostics.traces.len(), 1, "{label}: one Grid track trace");
        let trace = &diagnostics.traces[0];
        assert_eq!(trace.used_rows.explicit_track_count, 1, "{label}");
        assert_eq!(trace.used_rows.negative_implicit_track_count, 0, "{label}");
        let expected_implicit_rows = u16::try_from(
            expected
                .get("rows")
                .or_else(|| expected.get("rowValues"))
                .expect("expected rows should be recorded")
                .as_array()
                .expect("expected rows should be an array")
                .len()
                - 1,
        )
        .expect("bounded implicit row count should fit u16");
        assert_eq!(
            trace.used_rows.positive_implicit_track_count, expected_implicit_rows,
            "{label}"
        );
        let rows_pass = trace
            .passes
            .iter()
            .filter(|pass| pass.axis == spinon_layout::LayoutGridTrackSizingAxis::Rows)
            .max_by_key(|pass| pass.sequence)
            .expect("row track trace should exist");
        let final_stage = rows_pass
            .stages
            .last()
            .expect("final row stage should exist");
        let actual = final_stage
            .tracks
            .iter()
            .filter(|track| !track.is_gutter)
            .map(|track| track.base_size)
            .collect::<Vec<_>>();
        let expected_rows = expected
            .get("rows")
            .or_else(|| expected.get("rowValues"))
            .expect("expected rows should be recorded")
            .as_array()
            .expect("expected rows should be an array");
        assert_eq!(actual.len(), expected_rows.len(), "{label}: track count");
        for (index, (actual, expected)) in actual.iter().zip(expected_rows).enumerate() {
            let expected = expected.as_f64().unwrap() as f32;
            assert!(
                (actual - expected).abs() <= 1.0 / 64.0,
                "{label}: raw row {index}: actual={actual:?}, expected={expected:?} ± 1/64 CSS px"
            );
        }
    }
    #[cfg(not(feature = "c11-4-grid-track-sizing-fixture-test"))]
    {
        let _ = (completed, expected, label);
    }
}

#[test]
fn parsed_root_html_repeated_text_row_spans_match_all_960_chrome_scenes() {
    let expected = MATRIX
        .lines()
        .skip(1)
        .map(|line| serde_json::from_str::<Value>(line).expect("matrix scene should be valid JSON"))
        .collect::<Vec<_>>();
    assert_eq!(expected.len(), 960);
    assert_eq!(
        without_unused_extended_item_color_rules(&source_for(&expected[0])),
        APP_SENTINEL_SOURCE,
        "first scene matches the platform sentinel after dropping unused D/E paint rules"
    );
    let topologies = expected
        .iter()
        .map(|scene| scene["topology"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(topologies.len(), 4);
    let tails = expected
        .iter()
        .map(|scene| serde_json::to_string(&scene["tail"]).unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(tails.len(), 30);

    for (scene_index, expected_scene) in expected.iter().enumerate() {
        let source = source_for(expected_scene);
        let (session, layout) = run(&source);
        let cascade_snapshot = session.ua_cascade_snapshot();
        assert_eq!(
            layout.state,
            crate::session::RuntimeLayoutState::Ready,
            "scene {scene_index} {:?}: layout={:?}; diagnostics={:?}; cascade={:?}; styles={:?}",
            expected_scene["topology"],
            layout.error,
            layout.diagnostics,
            cascade_snapshot.error,
            cascade_snapshot.completed.as_ref().map(|cascade| {
                cascade.roots[0]
                    .styles
                    .elements
                    .iter()
                    .map(|style| {
                        (
                            style.node_id.get(),
                            style.properties.get("display"),
                            style.grid_layout.clone(),
                            style.grid_item_placement.clone(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
        );
        let completed = layout
            .completed
            .as_ref()
            .expect("scene should publish layout");
        let render = completed
            .render_snapshot
            .as_ref()
            .expect("scene should publish render output");
        let cascade = cascade_snapshot
            .completed
            .expect("same revision should publish computed styles");
        assert_eq!(
            completed.key, cascade.key,
            "scene {scene_index}: revision key"
        );

        let grid_style = cascade.roots[0]
            .styles
            .elements
            .iter()
            .find(|style| style.grid_layout.is_some())
            .expect("scene should have one parsed Grid owner");
        let tail = expected_scene["tail"]
            .as_array()
            .unwrap()
            .iter()
            .map(|track| match track.as_str().unwrap() {
                "min-content" => ComputedCssGridTrackSize::MinContent,
                "max-content" => ComputedCssGridTrackSize::MaxContent,
                "auto" => ComputedCssGridTrackSize::Auto,
                other => panic!("unsupported matrix track {other:?}"),
            })
            .collect::<Vec<_>>();
        let cap = match expected_scene["cap"].as_str().unwrap() {
            value if value.ends_with('%') => ComputedCssGridTrackLengthPercentage::Percentage(
                value.trim_end_matches('%').parse::<f32>().unwrap() / 100.0,
            ),
            value => ComputedCssGridTrackLengthPercentage::LengthPx(
                value.trim_end_matches("px").parse::<f32>().unwrap(),
            ),
        };
        let mut auto_rows = vec![ComputedCssGridTrackSize::FitContent(cap)];
        auto_rows.extend(tail);
        assert_eq!(
            grid_style.grid_layout.as_ref().unwrap().auto_rows,
            ComputedCssGridAutoRows::TrackList(auto_rows),
            "scene {scene_index}: exact computed auto-row order"
        );
        assert_eq!(
            grid_style
                .properties
                .get("align-content")
                .map(String::as_str),
            expected_scene["align"].as_str(),
            "scene {scene_index}: computed content alignment"
        );

        let grid_frame = completed
            .frames
            .iter()
            .find(|frame| frame.node_id == grid_style.node_id.get())
            .expect("Grid frame should publish");
        assert_frame(
            [
                grid_frame.x,
                grid_frame.y,
                grid_frame.width,
                grid_frame.height,
            ],
            [0.0, 0.0, 40.0, 220.0],
            &format!("scene {scene_index} Grid"),
        );

        let expected_items = expected_scene["items"].as_array().unwrap();
        for expected_item in expected_items {
            let id = expected_item["id"].as_str().unwrap();
            let item_frame = frame_for_paint(render, color_for_item(id));
            assert_frame(
                item_frame,
                frame(&expected_item["frame"]),
                &format!("scene {scene_index} item {id}"),
            );
            if expected_item["payload"].is_array() {
                assert_frame(
                    frame_for_paint(render, color_for_payload(id)),
                    frame(&expected_item["payload"]),
                    &format!("scene {scene_index} payload {id}"),
                );
            }
        }
        assert_optional_track_trace(completed, expected_scene, &format!("scene {scene_index}"));

        let placements = cascade.roots[0]
            .styles
            .elements
            .iter()
            .filter_map(|style| style.grid_item_placement.as_ref())
            .filter(|placement| {
                placement.row_start != ComputedCssGridPlacement::Auto
                    || placement.row_end != ComputedCssGridPlacement::Auto
            })
            .collect::<Vec<_>>();
        let expected_item_count = expected_items.len();
        assert_eq!(
            placements.len(),
            expected_item_count,
            "scene {scene_index}: items"
        );
        assert!(placements.iter().all(|placement| {
            placement.column_start == ComputedCssGridPlacement::Line(1)
                && placement.column_end == ComputedCssGridPlacement::Line(2)
        }));
        assert_eq!(
            placements
                .iter()
                .filter(|placement| {
                    placement.row_start == ComputedCssGridPlacement::Line(1)
                        && placement.row_end == ComputedCssGridPlacement::Line(2)
                })
                .count(),
            1,
            "scene {scene_index}: explicit empty row"
        );
        assert_eq!(
            placements
                .iter()
                .filter(|placement| {
                    placement.row_start == ComputedCssGridPlacement::Line(2)
                        && placement.row_end == ComputedCssGridPlacement::Span(2)
                })
                .count(),
            1,
            "scene {scene_index}: row 2 span 2"
        );
        assert_eq!(
            placements
                .iter()
                .filter(|placement| {
                    placement.row_start == ComputedCssGridPlacement::Line(3)
                        && placement.row_end == ComputedCssGridPlacement::Span(2)
                })
                .count(),
            1,
            "scene {scene_index}: row 3 span 2"
        );
        assert_eq!(
            placements
                .iter()
                .filter(|placement| {
                    placement.row_start == ComputedCssGridPlacement::Line(2)
                        && placement.row_end == ComputedCssGridPlacement::Span(3)
                })
                .count(),
            usize::from(expected_items.len() == 4),
            "scene {scene_index}: optional row 2 span 3"
        );
    }
}

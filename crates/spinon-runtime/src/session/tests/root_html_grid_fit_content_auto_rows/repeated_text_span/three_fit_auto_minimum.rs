use super::*;
use std::collections::BTreeMap;

#[path = "three_fit_auto_minimum/dynamic.rs"]
mod dynamic;
pub(super) use dynamic::{
    configure_scene, configure_six_fit_mixed_minimum_scene, observe_current,
    source_for_six_fit_mixed_minimum_track_list, source_for_track_list,
};
#[path = "three_fit_auto_minimum/five_fit.rs"]
mod five_fit;
#[path = "three_fit_auto_minimum/four_fit.rs"]
mod four_fit;
#[path = "three_fit_auto_minimum/matrix.rs"]
mod matrix;
#[path = "three_fit_auto_minimum/rejections.rs"]
mod rejections;
#[path = "three_fit_auto_minimum/trace.rs"]
mod trace;

const OBSERVATIONS: &str = include_str!(
    "../../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-precomparison-2026-11-10/chrome157-observation.ndjson"
);

#[derive(Clone)]
pub(super) struct SceneResult {
    /// Chrome oracle의 used-row 값. 런타임 행 추적은 별도 trace assertion이 검증한다.
    pub(super) rows: Vec<f32>,
    pub(super) item_frames: BTreeMap<String, [f32; 4]>,
    pub(super) payload_frames: BTreeMap<String, [f32; 4]>,
}

fn records() -> Vec<Value> {
    OBSERVATIONS
        .lines()
        .map(|line| serde_json::from_str(line).expect("three-fit Chrome observation is JSON"))
        .collect()
}

fn candidate_record() -> Value {
    records()
        .into_iter()
        .find(|record| {
            record["scene"]["minimum"] == "auto"
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Six-forward"
        })
        .expect("three-fit matrix has a row-span auto-minimum candidate")
}

fn computed_tracks(scene: &Value) -> Vec<ComputedCssGridTrackSize> {
    scene["trackMembers"]
        .as_array()
        .expect("Chrome records authored track members")
        .iter()
        .map(|member| {
            let member = member.as_str().expect("track member is a string");
            match member {
                "min-content" => ComputedCssGridTrackSize::MinContent,
                "max-content" => ComputedCssGridTrackSize::MaxContent,
                "auto" => ComputedCssGridTrackSize::Auto,
                value if value.starts_with("fit-content(") && value.ends_with(')') => {
                    let cap = &value[12..value.len() - 1];
                    let cap = if let Some(percent) = cap.strip_suffix('%') {
                        ComputedCssGridTrackLengthPercentage::Percentage(
                            percent.parse::<f32>().expect("percentage cap parses") / 100.0,
                        )
                    } else {
                        ComputedCssGridTrackLengthPercentage::LengthPx(
                            cap.strip_suffix("px")
                                .expect("length cap uses px")
                                .parse::<f32>()
                                .expect("px cap parses"),
                        )
                    };
                    ComputedCssGridTrackSize::FitContent(cap)
                }
                other => panic!("unsupported Chrome track member {other:?}"),
            }
        })
        .collect()
}

fn source_for(record: &Value) -> String {
    let scene = &record["scene"];
    let item_text = record["items"]
        .as_array()
        .expect("Chrome records text items")
        .iter()
        .map(|item| {
            let id = item["id"].as_str().expect("item id is a string");
            let text = item["text"].as_str().expect("item text is a string");
            assert!(
                text.bytes()
                    .all(|byte| byte.is_ascii_uppercase() || byte == b' '),
                "fixed payload is limited to uppercase ASCII words"
            );
            (id.to_owned(), text.to_owned())
        })
        .collect::<BTreeMap<_, _>>();
    let mut children = String::from(
        "<div class=\"item empty\" data-item=\"empty\" style=\"grid-row:1 / 2\"></div>",
    );
    for item in scene["items"].as_array().expect("scene records placements") {
        let id = item[0].as_str().expect("placement id is a string");
        let start = item[1].as_u64().expect("row start is positive");
        let span = item[2].as_u64().expect("row span is positive");
        let text = item_text
            .get(id)
            .unwrap_or_else(|| panic!("Chrome text payload exists for {id}"));
        children.push_str(&format!(
            "<div class=\"item\" data-item=\"{id}\" style=\"grid-row:{start} / span {span}\"><div class=\"payload\">{text}</div></div>"
        ));
    }
    let minimum = if scene["minimum"] == "auto" {
        "auto"
    } else {
        "0"
    };
    let style = format!(
        "* {{ box-sizing: border-box; }} html,body {{ margin:0; padding:0; }} \
         #grid {{ display:grid; width:40px; height:220px; grid-template-columns:40px; grid-template-rows:10px; grid-auto-rows:{}; grid-auto-flow:row; align-content:{}; align-items:normal; justify-content:start; justify-items:normal; direction:ltr; overflow:visible; margin:0; padding:0; row-gap:0; column-gap:0; }} \
         .item {{ display:block; width:auto; height:auto; min-height:{minimum}; margin:0; padding:0; overflow:visible; grid-column:1 / 2; }} \
         .empty {{ min-height:0; height:10px; background-color:rgb(11,17,23); }} \
         .payload {{ display:block; width:auto; height:auto; margin:0; padding:0; overflow:visible; font-family:monospace; font-size:10px; font-style:normal; font-weight:400; line-height:10px; letter-spacing:0; word-spacing:0; white-space:normal; word-break:normal; direction:ltr; }} \
         .item[data-item=\"A\"] {{ background-color:rgb(31,41,51); }} \
         .item[data-item=\"B\"] {{ background-color:rgb(61,71,81); }} \
         .item[data-item=\"C\"] {{ background-color:rgb(91,101,111); }} \
         .item[data-item=\"D\"] {{ background-color:rgb(211,81,111); }} \
         .item[data-item=\"E\"] {{ background-color:rgb(241,121,151); }} \
         .item[data-item=\"A\"] .payload {{ background-color:rgb(121,131,141); }} \
         .item[data-item=\"B\"] .payload {{ background-color:rgb(151,161,171); }} \
         .item[data-item=\"C\"] .payload {{ background-color:rgb(181,191,201); }} \
         .item[data-item=\"D\"] .payload {{ background-color:rgb(21,181,211); }} \
         .item[data-item=\"E\"] .payload {{ background-color:rgb(91,221,111); }}",
        scene["trackList"].as_str().expect("track list is a string"),
        scene["alignContent"]
            .as_str()
            .expect("alignment is a string")
    );
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><style>{style}</style></head><body><div id=\"grid\">{children}</div></body></html>"
    )
}

pub(super) fn style_for_color<'a>(
    styles: &'a [spinon_style::ComputedElementStyle],
    color: OpaqueCssSrgb,
    label: &str,
) -> &'a spinon_style::ComputedElementStyle {
    let matches = styles
        .iter()
        .filter(|style| {
            style.background_color.is_some_and(|actual| {
                (actual.red, actual.green, actual.blue)
                    == (color.red(), color.green(), color.blue())
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1, "{label}: color identifies one element");
    matches[0]
}

pub(super) fn row_values(record: &Value) -> Vec<f32> {
    record["rowValues"]
        .as_array()
        .expect("Chrome records used row sizes")
        .iter()
        .map(|value| value.as_f64().expect("row size is numeric") as f32)
        .collect()
}

fn item_span_height(record: &Value, item: &Value, rows: &[f32], label: &str) -> f32 {
    let id = item["id"].as_str().expect("item id is a string");
    let placement = scene_item_placement(record, id, label);
    let first_row = placement.0 - 1;
    let end_row = first_row + placement.1;
    rows.get(first_row..end_row)
        .unwrap_or_else(|| panic!("{label}: placement exceeds used rows {rows:?}"))
        .iter()
        .sum()
}

fn scene_item_placement(record: &Value, id: &str, label: &str) -> (usize, usize) {
    let placement = record["scene"]["items"]
        .as_array()
        .expect("scene records item placement")
        .iter()
        .find(|item| item[0] == id)
        .unwrap_or_else(|| panic!("{label}: expected scene placement for {id}"));
    (
        usize::try_from(placement[1].as_u64().expect("row start is unsigned"))
            .expect("row start fits usize"),
        usize::try_from(placement[2].as_u64().expect("row span is unsigned"))
            .expect("row span fits usize"),
    )
}

pub(super) fn eligible_for_auto_minimum(record: &Value, id: &str) -> bool {
    let scene = &record["scene"];
    let placement = scene["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item[0] == id)
        .expect("item has a placement");
    let start = placement[1].as_u64().unwrap() as usize;
    let span = placement[2].as_u64().unwrap() as usize;
    scene["trackMembers"]
        .as_array()
        .unwrap()
        .get(start - 2..start - 2 + span)
        .expect("placement stays within the six implicit rows")
        .iter()
        .any(|member| {
            let member = member.as_str().unwrap();
            member == "auto" || member.starts_with("fit-content(")
        })
}

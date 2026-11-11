use super::three_fit_auto_minimum::{
    configure_scene, eligible_for_auto_minimum, observe_current, row_values, source_for_track_list,
    style_for_color,
};
use super::*;
use flate2::read::GzDecoder;
use std::collections::BTreeMap;
use std::io::Read;
use std::sync::OnceLock;

#[path = "six_fit_auto_minimum/matrix.rs"]
mod matrix;
#[path = "six_fit_auto_minimum/rejections.rs"]
mod rejections;
#[path = "six_fit_auto_minimum/trace.rs"]
mod trace;

const OBSERVATIONS: &[u8] = include_bytes!(
    "../../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-six-fit-auto-minimum-precomparison-2026-11-12/chrome157-observation.ndjson.gz"
);

fn records() -> &'static [Value] {
    static RECORDS: OnceLock<Vec<Value>> = OnceLock::new();
    RECORDS.get_or_init(|| {
        let mut decoder = GzDecoder::new(OBSERVATIONS);
        let mut contents = String::new();
        decoder
            .read_to_string(&mut contents)
            .expect("six-fit Chrome observations decompress as UTF-8");
        contents
            .lines()
            .map(|line| serde_json::from_str(line).expect("six-fit Chrome observation is JSON"))
            .filter(|record: &Value| record["record"] == "scene")
            .collect()
    })
}

fn candidate_record() -> Value {
    records()
        .iter()
        .find(|record| {
            record["scene"]["minimum"] == "auto"
                && record["scene"]["fitPositions"]
                    .as_array()
                    .is_some_and(|items| items.len() == 6)
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Six-forward"
        })
        .expect("six-fit matrix contains a row-span automatic-minimum candidate")
        .clone()
}

fn css_color(color: OpaqueCssSrgb) -> String {
    format!("rgb({},{},{})", color.red(), color.green(), color.blue())
}

fn source_for_record(record: &Value) -> String {
    let scene = &record["scene"];
    let item_text = record["items"]
        .as_array()
        .expect("Chrome records text items")
        .iter()
        .map(|item| {
            (
                item["id"].as_str().expect("item id is a string").to_owned(),
                item["text"]
                    .as_str()
                    .expect("item text is a string")
                    .to_owned(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut children = String::from(
        "<div class=\"item empty\" data-item=\"empty\" style=\"grid-row:1 / 2\"></div>",
    );
    for placement in scene["items"]
        .as_array()
        .expect("scene records item placements")
    {
        let id = placement[0].as_str().expect("item id is a string");
        let start = placement[1].as_u64().expect("row start is positive");
        let span = placement[2].as_u64().expect("row span is positive");
        children.push_str(&format!(
            "<div class=\"item\" data-item=\"{id}\" style=\"grid-row:{start} / span {span}\"><div class=\"payload\">{}</div></div>",
            item_text
                .get(id)
                .unwrap_or_else(|| panic!("Chrome payload exists for {id}"))
        ));
    }

    let minimum = if scene["minimum"] == "auto" {
        "auto"
    } else {
        "0"
    };
    let mut style = format!(
        "* {{ box-sizing:border-box; }} html,body {{ margin:0; padding:0; }} \
         #grid {{ display:grid; width:40px; height:220px; grid-template-columns:40px; grid-template-rows:10px; grid-auto-rows:{}; grid-auto-flow:row; align-content:{}; align-items:normal; justify-content:start; justify-items:normal; direction:ltr; overflow:visible; margin:0; padding:0; row-gap:0; column-gap:0; }} \
         #grid .item {{ display:block; width:auto; height:auto; min-height:{minimum}; margin:0; padding:0; overflow:visible; grid-column:1 / 2; }} \
         #grid .empty {{ min-height:0; height:10px; background-color:{}; }} \
         #grid .payload {{ display:block; width:auto; height:auto; margin:0; padding:0; overflow:visible; font-family:monospace; font-size:10px; font-style:normal; font-weight:400; line-height:10px; letter-spacing:0; word-spacing:0; white-space:normal; word-break:normal; direction:ltr; }}",
        scene["trackList"].as_str().expect("track list is a string"),
        scene["alignContent"]
            .as_str()
            .expect("alignment is a string"),
        css_color(color_for_item("empty")),
    );
    for id in ["A", "B", "C", "D", "E"] {
        style.push_str(&format!(
            "#grid .item[data-item=\"{id}\"] {{ background-color:{}; }} \
             #grid .item[data-item=\"{id}\"] .payload {{ background-color:{}; }}",
            css_color(color_for_item(id)),
            css_color(color_for_payload(id)),
        ));
    }
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><style>{style}</style></head><body><div id=\"grid\">{children}</div></body></html>"
    )
}

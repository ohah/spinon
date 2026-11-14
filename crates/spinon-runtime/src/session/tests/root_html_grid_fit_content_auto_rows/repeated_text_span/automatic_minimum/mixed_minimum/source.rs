use super::super::*;
use flate2::read::GzDecoder;
use std::io::{BufRead, BufReader};

pub(super) const MIXED_LAYOUT_WIDTH_BASE: usize = 1280;
pub(super) const MIXED_MINIMUM_HEIGHT_BASE: usize = 400;

const MIXED_OBSERVATIONS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-mixed-auto-minimum-precomparison-2026-11-14/chrome157-one-fit-mixed-observation.ndjson.gz"
));

pub(super) fn observation_lines() -> std::io::Lines<BufReader<GzDecoder<&'static [u8]>>> {
    BufReader::new(GzDecoder::new(MIXED_OBSERVATIONS)).lines()
}

pub(super) fn read_metadata(
    lines: &mut std::io::Lines<BufReader<GzDecoder<&'static [u8]>>>,
) -> Value {
    let line = lines
        .next()
        .expect("one-fit mixed metadata line exists")
        .expect("one-fit mixed metadata decompresses");
    let metadata =
        serde_json::from_str::<Value>(&line).expect("one-fit mixed metadata is valid JSON");
    assert_eq!(metadata["record"], "metadata");
    assert_eq!(
        metadata["schema"],
        "spinon.c11-4.repeated-text-row-span-one-fit-mixed-auto-min-precomparison/v1"
    );
    assert_eq!(metadata["sceneCount"], 120_960);
    assert_eq!(metadata["uniqueTrackListCount"], 900);
    metadata
}

fn replace_once(source: &str, from: &str, to: &str) -> String {
    assert!(source.contains(from), "source fragment missing: {from:?}");
    source.replacen(from, to, 1)
}

pub(super) fn source_for_mixed(record: &Value) -> String {
    let mut source = super::super::source_for(record);
    let scene = &record["scene"];
    for id in scene["autoItems"]
        .as_array()
        .expect("mixed scene records auto item IDs")
        .iter()
        .map(|item| item.as_str().expect("auto item ID is a string"))
    {
        let placement = scene["items"]
            .as_array()
            .expect("mixed scene records row placements")
            .iter()
            .find(|item| item[0] == id)
            .unwrap_or_else(|| panic!("mixed scene places automatic item {id}"));
        let start = placement[1].as_u64().expect("row start is unsigned");
        let span = placement[2].as_u64().expect("row span is unsigned");
        let from = format!("data-item=\"{id}\" style=\"grid-row:{start} / span {span}\"");
        let to =
            format!("data-item=\"{id}\" style=\"grid-row:{start} / span {span}; min-height:auto\"");
        source = replace_once(&source, &from, &to);
    }
    source
}

pub(super) fn topology_index(row_profile: &str, topology: &str) -> usize {
    match (row_profile, topology) {
        ("three-row", "span2-forward") | ("six-row", "span2-six-forward") => 0,
        ("three-row", "span2-reversed") | ("six-row", "span2-six-reversed") => 1,
        ("three-row", "span2-before-span3") | ("six-row", "span2-span3-six-forward") => 2,
        ("three-row", "span3-before-span2") | ("six-row", "span2-span3-six-reversed") => 3,
        _ => panic!("unsupported one-fit mixed topology {row_profile:?}/{topology:?}"),
    }
}

pub(super) fn item_ids_for_variant(records: &[Value], variant: usize) -> Vec<String> {
    let sample = records
        .iter()
        .find(|record| {
            topology_index(
                record["scene"]["rowProfile"].as_str().unwrap(),
                record["scene"]["topology"].as_str().unwrap(),
            ) == variant
        })
        .unwrap_or_else(|| panic!("one-fit mixed batch has topology variant {variant}"));
    sample["scene"]["maskItemIds"]
        .as_array()
        .expect("scene records semantic minimum-mask item IDs")
        .iter()
        .map(|id| id.as_str().expect("mask item ID is a string").to_owned())
        .collect()
}

pub(super) fn item_color(variant: usize, id: &str, payload: bool) -> OpaqueCssSrgb {
    let item = match id {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "D" => 3,
        "E" => 4,
        other => panic!("unsupported one-fit mixed item {other:?}"),
    };
    let variant = u8::try_from(variant).expect("four topology variants fit in u8");
    let item = u8::try_from(item).expect("five text items fit in u8");
    if payload {
        OpaqueCssSrgb::new(
            150 + variant * 20 + item * 4,
            110 + variant * 25 + item * 6,
            80 + variant * 30 + item * 3,
        )
    } else {
        OpaqueCssSrgb::new(
            20 + variant * 30 + item * 5,
            30 + variant * 20 + item * 7,
            40 + variant * 15 + item * 11,
        )
    }
}

fn css_color(color: OpaqueCssSrgb) -> String {
    format!("rgb({},{},{})", color.red(), color.green(), color.blue())
}

fn text_payload(row_profile: &str, id: &str) -> String {
    let word_count = match (row_profile, id) {
        ("three-row", "A") => 6,
        ("three-row", "B") => 8,
        ("three-row", "C") => 12,
        ("six-row", "A") => 16,
        ("six-row", "B") => 14,
        ("six-row", "C") => 12,
        ("six-row", "D") => 10,
        ("six-row", "E") => 8,
        _ => panic!("unsupported one-fit mixed payload {row_profile:?}/{id:?}"),
    };
    vec!["A"; word_count].join(" ")
}

pub(super) fn source_for_track_list(
    track_list: &str,
    row_profile: &str,
    records: &[Value],
) -> String {
    let variants = (0..4)
        .map(|variant| {
            records
                .iter()
                .find(|record| {
                    topology_index(
                        record["scene"]["rowProfile"].as_str().unwrap(),
                        record["scene"]["topology"].as_str().unwrap(),
                    ) == variant
                })
                .unwrap_or_else(|| panic!("missing topology variant {variant}"))
        })
        .collect::<Vec<_>>();
    assert!(
        variants
            .iter()
            .all(|record| record["scene"]["rowProfile"] == row_profile),
        "one track list has one row topology profile"
    );

    let mut style = String::from(
        "* { box-sizing:border-box; } html,body { margin:0; padding:0; } \
         #grid { display:grid; box-sizing:border-box; width:40px; height:220px; grid-template-columns:40px; grid-template-rows:10px; grid-auto-flow:row; align-content:start; align-items:normal; justify-content:start; justify-items:normal; direction:ltr; overflow:visible; margin:0; padding:0; row-gap:0; column-gap:0; grid-auto-rows:",
    );
    style.push_str(track_list);
    style.push_str(
        "; } \
         #grid .item { display:block; box-sizing:border-box; width:auto; height:auto; min-height:0; margin:0; padding:0; overflow:visible; grid-column:1 / 2; } \
         #grid .empty { min-height:0; height:10px; background-color:rgb(11,17,23); } \
         #grid .payload { display:block; box-sizing:border-box; width:auto; height:auto; margin:0; padding:0; overflow:visible; font-family:monospace; font-size:10px; font-style:normal; font-weight:400; line-height:10px; letter-spacing:0; word-spacing:0; white-space:normal; word-break:normal; direction:ltr; } \
         #grid .variant { display:none; }",
    );

    for (variant, record) in variants.iter().enumerate() {
        let scene = &record["scene"];
        for placement in scene["items"].as_array().unwrap() {
            let id = placement[0].as_str().unwrap();
            let item = css_color(item_color(variant, id, false));
            let payload = css_color(item_color(variant, id, true));
            style.push_str(&format!(
                "#grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] {{ background-color:{item}; }} \
                 #grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] .payload[data-payload=\"{id}\"] {{ background-color:{payload}; }}"
            ));
        }
    }

    for mask in 0..32 {
        style.push_str(&format!(
            "@media (min-height:{}px) {{",
            MIXED_MINIMUM_HEIGHT_BASE + mask
        ));
        for variant in 0..4 {
            let ids = item_ids_for_variant(records, variant);
            for id in &ids {
                style.push_str(&format!(
                    "#grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] {{ min-height:0; }}"
                ));
            }
            for (bit, id) in ids.iter().enumerate() {
                if mask & (1 << bit) != 0 {
                    style.push_str(&format!(
                        "#grid .item[data-variant=\"{variant}\"][data-item=\"{id}\"] {{ min-height:auto; }}"
                    ));
                }
            }
        }
        style.push('}');
    }

    for profile in 0..8 {
        let width = MIXED_LAYOUT_WIDTH_BASE + profile;
        let variant = profile / 2;
        let alignment = if profile % 2 == 0 { "start" } else { "stretch" };
        style.push_str(&format!(
            "@media (min-width:{width}px) {{#grid {{ align-content:{alignment}; }}"
        ));
        for reset_variant in 0..4 {
            let display = if reset_variant == variant {
                "block"
            } else {
                "none"
            };
            style.push_str(&format!(
                "#grid .variant[data-variant=\"{reset_variant}\"] {{ display:{display}; }}"
            ));
        }
        style.push('}');
    }

    let mut children = String::from(
        "<div id=\"grid\"><div class=\"item empty\" data-item=\"empty\" style=\"grid-row:1 / 2; min-height:0\"></div>",
    );
    for (variant, record) in variants.iter().enumerate() {
        for placement in record["scene"]["items"].as_array().unwrap() {
            let id = placement[0].as_str().unwrap();
            let start = placement[1].as_u64().unwrap();
            let span = placement[2].as_u64().unwrap();
            children.push_str(&format!(
                "<div class=\"item variant\" data-variant=\"{variant}\" data-item=\"{id}\" style=\"grid-row:{start} / span {span}\"><div class=\"payload\" data-payload=\"{id}\">{}</div></div>",
                text_payload(row_profile, id)
            ));
        }
    }
    children.push_str("</div>");
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><style>{style}</style></head><body>{children}</body></html>"
    )
}

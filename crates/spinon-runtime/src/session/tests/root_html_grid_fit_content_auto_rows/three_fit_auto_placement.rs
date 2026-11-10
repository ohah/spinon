use serde_json::Value;
use spinon_style::ComputedCssGridTrackSize;

#[path = "three_fit_auto_placement/controls.rs"]
mod controls;
#[path = "three_fit_auto_placement/matrix.rs"]
mod matrix;
#[path = "three_fit_auto_placement/rejections.rs"]
mod rejections;

const OBSERVATIONS: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-three-fit-auto-min-precomparison-2026-11-11/chrome157-observation.ndjson"
);
const SOURCE: &str = include_str!(
    "../../../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-autoplacement-three-fit-content-runtime.html"
);
const BASE_TRACK_LIST: &str =
    "fit-content(0px) fit-content(10px) fit-content(20%) min-content max-content auto";
const DARK_TRACK_LIST: &str =
    "fit-content(10px) min-content fit-content(20%) max-content fit-content(30px) auto";
const DARK_FOUR_FIT_TRACK_LIST: &str = "fit-content(10px) fit-content(20px) fit-content(30px) fit-content(40px) min-content max-content";
const ITEMS_PLACEHOLDER: &str = "<!-- C11_4_REPEATED_TEXT_ITEMS -->";
const MINIMUM_RULES_PLACEHOLDER: &str = "/* C11_4_ITEM_MINIMUM_RULES */";

pub(super) fn scenes() -> Vec<Value> {
    let records = OBSERVATIONS
        .lines()
        .map(|line| {
            serde_json::from_str::<Value>(line).expect("Chrome record should be valid JSON")
        })
        .collect::<Vec<_>>();
    assert_eq!(records.len(), 5_761, "one metadata record and 5,760 scenes");
    assert_eq!(records[0]["record"].as_str(), Some("metadata"));
    assert_eq!(records[0]["sceneCount"].as_u64(), Some(5_760));
    records.into_iter().skip(1).collect()
}

pub(super) fn source_for(scene: &Value) -> String {
    let track_list = scene["scene"]["trackList"]
        .as_str()
        .expect("Chrome scene track list should be a string");
    let alignment = scene["scene"]["alignContent"]
        .as_str()
        .expect("Chrome scene alignment should be a string");
    let minimum = scene["scene"]["minHeight"]
        .as_str()
        .expect("Chrome scene minimum should be a string");
    let items = scene["items"]
        .as_array()
        .expect("Chrome scene items should be an array")
        .iter()
        .skip(1)
        .map(|item| {
            let id = item["id"]
                .as_str()
                .expect("item id should be a string")
                .parse::<usize>()
                .expect("repeated item id should be its one-based index");
            let payload = super::two_fit_auto_placement::PAYLOAD_TEXTS[id - 1];
            format!(
                "      <div id=\"item-{id}\" class=\"item\"><div id=\"text-{id}\" class=\"payload\">{payload}</div></div>"
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let minimum_rules = if minimum == "auto" {
        "#grid .item { min-height: auto; }\n      #grid #item-0 { min-height: 0; }"
    } else {
        "#grid .item { min-height: 0; }\n      #grid #item-0 { min-height: 0; }"
    };
    let source = super::replace_once(SOURCE, ITEMS_PLACEHOLDER, &items);
    let source = super::replace_once(&source, MINIMUM_RULES_PLACEHOLDER, minimum_rules);
    let source = super::replace_once(&source, BASE_TRACK_LIST, track_list);
    super::replace_once(
        &source,
        "align-content: start;",
        &format!("align-content: {alignment};"),
    )
}

pub(super) fn computed_tracks(scene: &Value) -> Vec<ComputedCssGridTrackSize> {
    super::two_fit_auto_placement::computed_tracks(scene)
}

pub(super) fn is_automatic_minimum(scene: &Value) -> bool {
    scene["scene"]["minHeight"].as_str() == Some("auto")
}

pub(super) fn item_id(item: &Value) -> usize {
    item["id"]
        .as_str()
        .expect("item id should be a string")
        .parse::<usize>()
        .expect("repeated item id should be its one-based index")
}

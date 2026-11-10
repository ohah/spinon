use serde_json::Value;
use spinon_style::{
    ComputedCssGridTrackLengthPercentage as TrackLength, ComputedCssGridTrackSize as TrackSize,
};
use std::collections::BTreeMap;

#[path = "one_fit_auto_placement/controls.rs"]
mod controls;
#[path = "one_fit_auto_placement/matrix.rs"]
mod matrix;
#[path = "one_fit_auto_placement/rejections.rs"]
mod rejections;

const AUTO_MINIMUM_OBSERVATIONS: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-one-fit-auto-min-precomparison-2026-11-10/chrome-matrix-observation.ndjson"
);
const BASELINE_OBSERVATIONS: &str = include_str!(
    "../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-fit-content-positions-precomparison-2026-11-09/chrome157-matrix-observation.ndjson"
);

pub(super) fn auto_minimum_scenes() -> Vec<Value> {
    scenes_from(AUTO_MINIMUM_OBSERVATIONS, "sceneCount", 1_440)
}

pub(super) fn baseline_scenes() -> Vec<Value> {
    scenes_from(BASELINE_OBSERVATIONS, "caseCount", 1_440)
}

fn scenes_from(observations: &str, count_key: &str, expected_count: u64) -> Vec<Value> {
    let records = observations
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("Chrome record is JSON"))
        .collect::<Vec<_>>();
    let metadata = records.first().expect("matrix metadata exists");
    assert_eq!(metadata["record"].as_str(), Some("metadata"));
    assert_eq!(metadata[count_key].as_u64(), Some(expected_count));
    let scenes = records.into_iter().skip(1).collect::<Vec<_>>();
    assert_eq!(scenes.len(), expected_count as usize);
    scenes
}

pub(super) fn scene_key(scene: &Value) -> String {
    let tail = serde_json::to_string(&scene["tail"]).expect("scene tail serializes");
    format!(
        "{}|{}|{}|{}",
        scene["fitPosition"].as_u64().expect("fit position"),
        tail,
        scene["cap"].as_str().expect("cap"),
        scene["align"].as_str().expect("alignment")
    )
}

pub(super) fn source_for(scene: &Value, auto_minimum: bool) -> String {
    let track_list = scene["computedAutoRows"]
        .as_str()
        .expect("computed track list is present");
    let source = super::replace_once(
        super::two_fit_auto_placement::SOURCE,
        super::two_fit_auto_placement::BASE_TRACK_LIST,
        track_list,
    );
    let source = super::replace_once(
        &source,
        "align-content: start;",
        &format!("align-content: {};", scene["align"].as_str().unwrap()),
    );
    if !auto_minimum {
        return source;
    }
    let source = super::replace_once(&source, "min-height: 0;", "min-height: auto;");
    super::replace_once(
        &source,
        "#item-0 { height: 10px;",
        "#item-0 { min-height: 0; height: 10px;",
    )
}

pub(super) fn computed_tracks(scene: &Value) -> Vec<TrackSize> {
    scene["authoredAutoRows"]
        .as_array()
        .expect("authored track list exists")
        .iter()
        .map(|value| {
            let value = value.as_str().expect("track member is a string");
            match value {
                "min-content" => TrackSize::MinContent,
                "max-content" => TrackSize::MaxContent,
                "auto" => TrackSize::Auto,
                value if value.starts_with("fit-content(") && value.ends_with(')') => {
                    let cap = &value[12..value.len() - 1];
                    let cap = if let Some(percent) = cap.strip_suffix('%') {
                        TrackLength::Percentage(
                            percent.parse::<f32>().expect("percent cap parses") / 100.0,
                        )
                    } else {
                        TrackLength::LengthPx(
                            cap.strip_suffix("px")
                                .expect("px cap suffix")
                                .parse::<f32>()
                                .expect("px cap parses"),
                        )
                    };
                    TrackSize::FitContent(cap)
                }
                other => panic!("unsupported track member {other:?}"),
            }
        })
        .collect()
}

pub(super) fn expected_frame(value: &Value) -> [f32; 4] {
    let values = value.as_array().expect("frame is an array");
    assert_eq!(values.len(), 4);
    std::array::from_fn(|index| values[index].as_f64().expect("frame value") as f32)
}

pub(super) fn rows(scene: &Value) -> Vec<f32> {
    scene["rows"]
        .as_array()
        .expect("row sizes are present")
        .iter()
        .map(|row| row.as_f64().expect("row size is numeric") as f32)
        .collect()
}

pub(super) fn track_list_key(scene: &Value) -> String {
    scene["authoredAutoRows"]
        .as_array()
        .expect("authored track list exists")
        .iter()
        .map(|member| member.as_str().expect("track is text"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn assert_scene_metadata(scenes: &[Value]) {
    assert_eq!(scenes.len(), 1_440);
    let positions = scenes
        .iter()
        .map(|scene| scene["fitPosition"].as_u64().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(positions.len(), 6);
    let tails = scenes
        .iter()
        .map(|scene| serde_json::to_string(&scene["tail"]).unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(tails.len(), 30);
    let caps = scenes
        .iter()
        .map(|scene| scene["cap"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        caps,
        std::collections::BTreeSet::from(["0%", "120%", "20%", "20px"])
    );
    let alignments = scenes
        .iter()
        .map(|scene| scene["align"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        alignments,
        std::collections::BTreeSet::from(["start", "stretch"])
    );
}

pub(super) fn assert_scene_key_sets_match() -> BTreeMap<String, Value> {
    let candidate = auto_minimum_scenes();
    let baseline = baseline_scenes();
    assert_scene_metadata(&candidate);
    assert_scene_metadata(&baseline);
    let baseline_by_key = baseline
        .into_iter()
        .map(|scene| (scene_key(&scene), scene))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        baseline_by_key.len(),
        1_440,
        "baseline scene keys are unique"
    );
    let candidate_keys = candidate
        .iter()
        .map(scene_key)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        candidate_keys.len(),
        1_440,
        "candidate scene keys are unique"
    );
    assert_eq!(candidate_keys, baseline_by_key.keys().cloned().collect());
    baseline_by_key
}

use super::dynamic::{configure_scene, observe_current, source_for_track_list};
use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[path = "four_fit/matrix.rs"]
mod matrix;
#[path = "four_fit/rejections.rs"]
mod rejections;
#[path = "four_fit/trace.rs"]
mod trace;

const FOUR_FIT_OBSERVATIONS: &str = include_str!(
    "../../../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum-precomparison-2026-11-11/chrome157-observation.ndjson"
);

fn four_fit_records() -> Vec<Value> {
    FOUR_FIT_OBSERVATIONS
        .lines()
        .map(|line| serde_json::from_str(line).expect("four-fit Chrome observation is JSON"))
        .collect()
}

fn four_fit_candidate_record() -> Value {
    four_fit_records()
        .into_iter()
        .find(|record| {
            record["scene"]["minimum"] == "auto"
                && record["scene"]["fitPositions"]
                    .as_array()
                    .is_some_and(|positions| positions.len() == 4)
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Six-forward"
        })
        .expect("four-fit matrix contains a row-span automatic-minimum candidate")
}

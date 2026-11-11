use super::dynamic::{configure_scene, observe_current, source_for_track_list};
use super::*;
use flate2::read::GzDecoder;
use std::io::Read;
use std::sync::OnceLock;

#[path = "five_fit/matrix.rs"]
mod matrix;
#[path = "five_fit/rejections.rs"]
mod rejections;
#[path = "five_fit/trace.rs"]
mod trace;

const OBSERVATIONS: &[u8] = include_bytes!(
    "../../../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-precomparison-2026-11-11/chrome157-observation.ndjson.gz"
);

fn records() -> &'static [Value] {
    static RECORDS: OnceLock<Vec<Value>> = OnceLock::new();
    RECORDS.get_or_init(|| {
        let mut decoder = GzDecoder::new(OBSERVATIONS);
        let mut contents = String::new();
        decoder
            .read_to_string(&mut contents)
            .expect("five-fit Chrome observations decompress as UTF-8");
        contents
            .lines()
            .map(|line| serde_json::from_str(line).expect("five-fit Chrome observation is JSON"))
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
                    .is_some_and(|positions| positions.len() == 5)
                && record["scene"]["alignContent"] == "start"
                && record["scene"]["topology"] == "span2Six-forward"
        })
        .expect("five-fit matrix contains a row-span automatic-minimum candidate")
        .clone()
}

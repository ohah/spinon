use super::three_fit_auto_minimum::{
    SceneResult, configure_six_fit_mixed_minimum_scene, eligible_for_auto_minimum, observe_current,
    row_values, source_for_six_fit_mixed_minimum_track_list,
};
use super::*;
use flate2::read::GzDecoder;
use std::collections::BTreeMap;
use std::io::Read;
use std::sync::OnceLock;

#[path = "five_fit_mixed_auto_minimum/rejections.rs"]
mod rejections;

const OBSERVATIONS: &[u8] = include_bytes!(
    "../../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-mixed-auto-minimum-precomparison-2026-11-12/runtime-matrix-3984.ndjson.gz"
);

fn records() -> &'static [Value] {
    static RECORDS: OnceLock<Vec<Value>> = OnceLock::new();
    RECORDS.get_or_init(|| {
        let mut decoder = GzDecoder::new(OBSERVATIONS);
        let mut contents = String::new();
        decoder
            .read_to_string(&mut contents)
            .expect("five-fit mixed RuntimeSession sample decompresses as UTF-8");
        let mut parsed = contents.lines().map(|line| {
            serde_json::from_str::<Value>(line)
                .expect("five-fit mixed RuntimeSession sample is JSON")
        });
        let metadata = parsed.next().expect("runtime sample metadata is present");
        assert_eq!(metadata["record"], "metadata");
        assert_eq!(metadata["sceneCount"], 3_984);
        assert_eq!(metadata["fullOracleSceneCount"], 414_720);
        assert_eq!(metadata["selectedTrackListCount"], 120);
        assert_eq!(
            metadata["fullOracleSha256"],
            "fa9e2d0194c8f2739df8eb9dd1bf7576bd6aa7c92bb927701d4f757bc876f35a"
        );
        parsed
            .filter(|record| record["record"] == "scene")
            .collect()
    })
}

fn batches() -> &'static BTreeMap<String, Vec<&'static Value>> {
    static BATCHES: OnceLock<BTreeMap<String, Vec<&'static Value>>> = OnceLock::new();
    BATCHES.get_or_init(|| {
        let mut batches = BTreeMap::<String, Vec<&'static Value>>::new();
        for record in records() {
            let track_list = record["scene"]["trackList"]
                .as_str()
                .expect("track list is a string")
                .to_owned();
            batches.entry(track_list).or_default().push(record);
        }
        batches
    })
}

fn topology_item_ids(topology: &str) -> &'static [&'static str] {
    if topology.starts_with("span2Six-") {
        &["A", "B", "C", "D", "E"]
    } else if topology.starts_with("span2Span3Six-") {
        &["A", "B", "C", "D"]
    } else {
        panic!("unknown five-fit mixed topology {topology:?}")
    }
}

fn encoded_minimum_mask(scene: &Value) -> usize {
    let topology = scene["topology"].as_str().expect("topology is a string");
    let ids = topology_item_ids(topology);
    let auto_items = scene["autoItems"]
        .as_array()
        .expect("scene records automatic-minimum item IDs");
    let mut mask = 0;
    for (bit, id) in ids.iter().enumerate() {
        if auto_items.iter().any(|item| item.as_str() == Some(id)) {
            mask |= 1 << bit;
        }
    }
    assert_eq!(scene["minimumMask"].as_u64(), Some(mask as u64));
    assert!(
        auto_items
            .iter()
            .all(|item| item.as_str().is_some_and(|id| ids.contains(&id)))
    );
    mask
}

pub(super) fn assert_frame_maps_equal(
    actual: &BTreeMap<String, [f32; 4]>,
    expected: &BTreeMap<String, [f32; 4]>,
    label: &str,
) {
    assert_eq!(
        actual.keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>(),
        "{label}: semantic frame owners"
    );
    for (id, frame) in actual {
        assert_frame(*frame, expected[id], &format!("{label}: {id}"));
    }
}

#[path = "five_fit_mixed_auto_minimum/matrix.rs"]
mod matrix;
#[path = "five_fit_mixed_auto_minimum/rounding_cases.rs"]
mod rounding_cases;

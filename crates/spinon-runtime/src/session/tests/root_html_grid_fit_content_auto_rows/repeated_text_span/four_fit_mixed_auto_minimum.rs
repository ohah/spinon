use super::*;
use flate2::read::GzDecoder;
use std::collections::BTreeMap;
use std::io::Read;
use std::sync::OnceLock;

#[path = "four_fit_mixed_auto_minimum/matrix.rs"]
mod matrix;
#[path = "four_fit_mixed_auto_minimum/rejections.rs"]
mod rejections;

const OBSERVATIONS: &[u8] = include_bytes!(
    "../../../../../../../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison-2026-11-12/runtime-matrix-3840.ndjson.gz"
);

pub(super) fn records() -> &'static [Value] {
    static RECORDS: OnceLock<Vec<Value>> = OnceLock::new();
    RECORDS.get_or_init(|| {
        let mut decoder = GzDecoder::new(OBSERVATIONS);
        let mut contents = String::new();
        decoder
            .read_to_string(&mut contents)
            .expect("four-fit mixed RuntimeSession sample decompresses as UTF-8");
        let mut parsed = contents.lines().map(|line| {
            serde_json::from_str::<Value>(line)
                .expect("four-fit mixed RuntimeSession sample is JSON")
        });
        let metadata = parsed.next().expect("sample metadata is present");
        assert_eq!(metadata["record"], "metadata");
        assert_eq!(metadata["sceneCount"], 3_840);
        assert_eq!(metadata["fullOracleSceneCount"], 138_240);
        assert_eq!(metadata["selectedTrackListCount"], 20);
        assert_eq!(
            metadata["fullOracleSha256"],
            "312497ced752c543ba367b57995c49bf00fe51bb5bcdd22f5302af0d7fc22570"
        );
        parsed
            .filter(|record| record["record"] == "scene")
            .collect()
    })
}

pub(super) fn batches() -> &'static BTreeMap<String, Vec<&'static Value>> {
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

pub(super) fn topology_item_ids(topology: &str) -> &'static [&'static str] {
    if topology.starts_with("span2Six-") {
        &["A", "B", "C", "D", "E"]
    } else if topology.starts_with("span2Span3Six-") {
        &["A", "B", "C", "D"]
    } else {
        panic!("unknown four-fit mixed topology {topology:?}")
    }
}

pub(super) fn encoded_minimum_mask(scene: &Value) -> usize {
    let ids = topology_item_ids(scene["topology"].as_str().unwrap());
    let auto_items = scene["autoItems"].as_array().unwrap();
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

pub(super) fn source_order_key(scene: &Value) -> String {
    let topology = scene["topology"].as_str().unwrap();
    let family = topology
        .strip_suffix("-forward")
        .or_else(|| topology.strip_suffix("-reverse"))
        .expect("topology records source order");
    format!(
        "{}|{}|{}|{}",
        scene["trackList"].as_str().unwrap(),
        scene["alignContent"].as_str().unwrap(),
        scene["minimumMask"].as_u64().unwrap(),
        family
    )
}

pub(super) fn alignment_key(scene: &Value) -> String {
    format!(
        "{}|{}|{}",
        scene["trackList"].as_str().unwrap(),
        scene["topology"].as_str().unwrap(),
        scene["minimumMask"].as_u64().unwrap()
    )
}

pub(super) fn grid_end_overflow(frame: [f32; 4], grid_frame: [f32; 4]) -> bool {
    frame[1] + frame[3] > grid_frame[1] + grid_frame[3] + 0.01
}

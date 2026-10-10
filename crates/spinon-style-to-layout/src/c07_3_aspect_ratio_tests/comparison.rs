use super::*;

#[test]
fn aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs() {
    let reference: Value = serde_json::from_slice(&fs::read(REFERENCE).unwrap()).unwrap();
    assert_eq!(reference["oracle"]["product"], "Chrome/154.0.8037.98");
    assert_eq!(
        reference["oracle"]["revision"],
        "@b859317bf11f6be47f9b7799ec690a0a42a1fb33"
    );
    assert_eq!(reference["fixture"]["id"], "C07.3-aspect-ratio-v1");

    let dpr_one = compare_dpr(1.0, &reference);
    let dpr_two = compare_dpr(2.0, &reference);
    assert_eq!(dpr_one, dpr_two, "CSS px 결과가 DPR에 따라 달라졌습니다");
}

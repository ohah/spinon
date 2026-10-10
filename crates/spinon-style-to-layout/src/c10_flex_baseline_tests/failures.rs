use super::fixture::{
    assert_author_stylesheet_last_baseline_falls_back, assert_legacy_profile_rejects_baseline,
    assert_reference_matches_all_runtime_profiles,
};

#[test]
fn baseline_frames_match_all_six_runtime_flex_profiles_and_both_dprs() {
    assert_reference_matches_all_runtime_profiles();
}

#[test]
fn baseline_values_do_not_leak_into_the_legacy_flex_profile() {
    assert_legacy_profile_rejects_baseline();
}

#[test]
fn invalid_last_baseline_declarations_preserve_stylesheet_cascade_fallback() {
    assert_author_stylesheet_last_baseline_falls_back();
}

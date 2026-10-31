# Existing workspace test failure observed during C11.2.2 verification

Command:

```sh
cargo test --workspace --locked
```

One test in `spinon-style-to-layout` failed before the workspace run stopped:

```text
c07_3_aspect_ratio_tests::comparison::aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs
1x ratio-calc-number.aspect-ratio computed style
actual:   Some("1.7777778 / 1")
expected: Some("1.77778 / 1")
```

The exact test was rerun alone and reproduced the same serialization mismatch. No C07.3 test or aspect-ratio implementation file is changed in this candidate. The rest of the workspace passed in a second run with only this exact test excluded: 1,431 passed, 4 ignored, 0 failed. That filtered run is logged in [workspace-test-excluding-c07-3-baseline.log](./workspace-test-excluding-c07-3-baseline.log).

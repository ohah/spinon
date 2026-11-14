# One-fit mixed RuntimeSession matrix result

## Command

```sh
mise exec -- env CARGO_PROFILE_TEST_OPT_LEVEL=2 cargo test -p spinon-runtime \
  --features c11-4-grid-track-sizing-fixture-test --lib \
  parsed_root_html_one_fit_row_span_itemwise_mixed_minimum_matches_all_120960_chrome_scenes \
  -- --nocapture
```

## Result

- RuntimeSession scenes: **120,960 / 120,960**
- Unique track-list batches: **900 / 900**
- Eligible automatic-minimum item occurrences: **169,344**
- Result: **passed**
- Test duration: **2,633.00 s**

Each scene checked computed track order and alignment, per-item computed minimum, owner/revision-scoped row trace and automatic-minimum contribution, row/item/payload frames, text line count, content floor, and Grid content-edge overflow.

The first full run stopped at scene 99449 because the 5-second layout wait returned `Pending` with no layout error. That scene passed alone in 1.78 s. The test wait was raised to 30 seconds, and the complete matrix then passed. Details are in [runtime-matrix-timeout-regression.md](runtime-matrix-timeout-regression.md).

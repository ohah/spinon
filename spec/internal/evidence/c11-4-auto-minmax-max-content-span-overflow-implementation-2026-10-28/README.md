# C11.4 `minmax(auto, max-content)` two-track span · 구현 근거

## 구현 범위

새 `#[cfg(test)]` submodule에 five fixed Columns cases를 추가했다. One Ahem text item spans exactly two `minmax(auto,max-content)` columns. Tests assert the definite root frame, item frame, both per-track final base sizes, one `Minimum` contribution source/value, and one `MaxContent` contribution value. Existing `minmax(auto,1fr)` and one-track `minmax(auto,max-content)` tests are unchanged. Production layout, CSS admission, app route, FFI/API and Taffy source did not change.

## 기대값과 결과

The independent values were fixed before implementation in the [precomparison](../c11-4-auto-minmax-max-content-span-overflow-precomparison-2026-10-28/README.md). `visible` and `clip` require content-based automatic minimum 100px, two final tracks `[50,50]px`, and item frame `100×20px`. `hidden`, `auto`, and `scroll` require overflow automatic minimum 0px, tracks `[40,40]px`, item frame `80×20px`, and preserve the 100px max-content contribution. Content-based trace requires `spans_auto_min_track=true`, `spans_multiple_tracks=true`, `spans_flexible_max_track=false`, and `content_based=true`.

## 실행 게이트

| Gate | Result |
| --- | --- |
| focused `text_grid_auto_minmax_max_content_two_track_span_follows_overflow_scrollability` | 1 passed |
| `mise exec -- cargo test --locked -p spinon-layout` | 298 passed, 0 failed; doc-tests 0 |
| `mise exec -- cargo fmt --all -- --check` | passed after applying `cargo fmt --all`; first check found formatting-only differences in the new file |
| `mise exec -- cargo clippy --locked -p spinon-layout --all-targets -- -D warnings` | passed |

This is pure Rust layout unit coverage, so Android/iOS app and Simulator execution are not applicable. The independent Chrome 156 browser baseline is recorded separately and is not a Spinon result. The change does not exercise general CSS overflow admission, clipping pixels, scrolling or scrollbars, WPT, or the full C11.4 contract. The new test uses exact `assert_eq!` comparisons because the shared helper's tolerance is `1/64px`, wider than this plan's zero-pixel tolerance.

## Source hashes

| File | SHA-256 |
| --- | --- |
| `crates/spinon-layout/src/grid_auto_min_scrollable_overflow_tests.rs` | `8420c791997c662f58166674c18ae549483766e59a817c8cca106f64dd371889` |
| `crates/spinon-layout/src/grid_auto_min_scrollable_overflow_tests/two_track_span_max_content.rs` | `9434b0bf46a8939c3fbf6c1b94b5cd4f94c19ceb9feb35e0cd8bf38c4f555373` |

The changed Rust source passed the distinct [implementation/runtime review in 20 perspectives](./implementation-runtime-review-20.md) and its separate [PR diff review in 30 perspectives](./pr-diff-review-30.md). C11.4 stays incomplete and the official completion count remains `72/213`.

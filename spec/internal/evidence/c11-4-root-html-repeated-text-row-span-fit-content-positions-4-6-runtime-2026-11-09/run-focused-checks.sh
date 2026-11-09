#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$(cd "$(dirname "$0")" && pwd)"
checks_dir="$evidence_dir/checks"
mkdir -p "$checks_dir"
cd "$repo_root"

run_check() {
  local name="$1"
  shift
  {
    printf '+'
    printf ' %q' "$@"
    printf '\n'
    "$@"
  } 2>&1 | tee "$checks_dir/$name.log"
}

run_check matrix-positions-4-6 \
  cargo test -p spinon-runtime --lib \
  --features c11-4-grid-track-sizing-fixture-test \
  parsed_root_html_later_fit_content_positions_four_through_six_match_all_4320_chrome_scenes \
  -- --nocapture
run_check row-span-writing-context-rejections \
  cargo test -p spinon-runtime --lib \
  --features c11-4-grid-track-sizing-fixture-test \
  parsed_root_html_six_row_profile_rejects_non_ltr_or_non_horizontal_writing_contexts_atomically \
  -- --nocapture
run_check matrix-positions-1-3 \
  cargo test -p spinon-runtime --lib \
  --features c11-4-grid-track-sizing-fixture-test \
  parsed_root_html_later_fit_content_positions_match_all_1920_chrome_scenes \
  -- --nocapture
run_check matrix-repeated-text-810 \
  cargo test -p spinon-runtime --lib \
  --features c11-4-grid-track-sizing-fixture-test \
  parsed_root_html_repeated_intrinsic_text_matches_all_810_chrome_scenes \
  -- --nocapture
run_check matrix-row-span-960 \
  cargo test -p spinon-runtime --lib \
  --features c11-4-grid-track-sizing-fixture-test \
  parsed_root_html_repeated_text_row_spans_match_all_960_chrome_scenes \
  -- --nocapture
run_check matrix-auto-placement-fit-content \
  cargo test -p spinon-runtime --lib \
  --features c11-4-grid-track-sizing-fixture-test \
  parsed_root_html_repeated_text_auto_placement_matches_all_fit_content_positions \
  -- --nocapture
run_check grid-track-sizing-127 \
  cargo test -p spinon-layout --lib track_sizing
run_check focused-clippy \
  cargo clippy -p spinon-runtime --lib \
  --features c11-4-grid-track-sizing-fixture-test -- -D warnings
run_check rustfmt-workspace cargo fmt --all -- --check
run_check rustfmt-vendored \
  rustfmt --edition 2021 --check \
  vendor/taffy/src/compute/grid/track_sizing.rs \
  vendor/taffy/src/compute/grid/types/grid_track.rs
run_check git-diff-check git diff --check

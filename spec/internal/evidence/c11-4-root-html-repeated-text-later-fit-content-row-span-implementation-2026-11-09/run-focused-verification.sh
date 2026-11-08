#!/usr/bin/env bash
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
repo_root="$(git -C "$script_directory" rev-parse --show-toplevel)"
logs="$script_directory/rust-verification"
mkdir -p "$logs"

run() {
  label=$1
  shift
  log="$logs/$label.log"
  printf 'START %s\n' "$label"
  if (cd "$repo_root" && "$@") > "$log" 2>&1; then
    printf 'exit_code=0 command=' >> "$log"
    printf '%q ' "$@" >> "$log"
    printf '\n' >> "$log"
    printf 'PASS %s\n' "$label"
  else
    result=$?
    cat "$log" >&2
    printf 'FAIL %s (exit %s)\n' "$label" "$result" >&2
    exit "$result"
  fi
}

run cargo-fmt cargo fmt --all -- --check
run scoped-clippy cargo clippy --locked -p spinon-layout -p spinon-runtime -p spinon-style-to-layout --all-targets -- -D warnings
run style-projection cargo test --locked -p spinon-style-to-layout --lib root_html_repeated_intrinsic_auto_row_list -- --nocapture
run layout-fit-content cargo test --locked -p spinon-layout --lib grid_root_html_fit_content_auto_rows_tests -- --nocapture
run grid-track-sizing cargo test --locked -p spinon-layout --lib track_sizing_tests:: -- --nocapture
run runtime-new-matrix cargo test --locked -p spinon-runtime --lib parsed_root_html_later_fit_content_positions_match_all_1920_chrome_scenes -- --nocapture
run runtime-new-rejections cargo test --locked -p spinon-runtime --lib later_fit_content_row_span::rejections -- --nocapture
run runtime-position1-span-regression cargo test --locked -p spinon-runtime --lib parsed_root_html_repeated_text_row_spans_match_all_960_chrome_scenes -- --nocapture
run runtime-position1-text-regression cargo test --locked -p spinon-runtime --lib parsed_root_html_repeated_intrinsic_text_matches_all_810_chrome_scenes -- --nocapture
run runtime-trace-matrix cargo test --locked -p spinon-runtime --lib --features c11-4-grid-track-sizing-fixture-test parsed_root_html_later_fit_content_positions_match_all_1920_chrome_scenes -- --nocapture
run diff-check git diff --check
run staged-diff-check git diff --cached --check

printf 'All focused implementation verification commands passed.\n'

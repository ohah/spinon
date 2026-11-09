#!/usr/bin/env bash
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
repo_root="$(git -C "$script_directory" rev-parse --show-toplevel)"
raw_log_hashes="$script_directory/raw-log-sha256.txt"
: > "$raw_log_hashes"

archive_log() {
  log_path=$1
  log_name="${log_path##*/}"
  log_hash="$(shasum -a 256 "$log_path" | awk '{print $1}')"
  printf '%s  %s.gz\n' "$log_hash" "$log_name" >> "$raw_log_hashes"
  gzip -n -f "$log_path"
}

run() {
  name=$1
  shift
  log_path="$script_directory/$name.log"
  if "$@" > "$log_path" 2>&1; then
    tail -n 5 "$log_path"
    archive_log "$log_path"
  else
    tail -n 120 "$log_path" >&2
    archive_log "$log_path"
    return 1
  fi
}

cd "$repo_root"
run style-projection cargo test --locked -p spinon-style-to-layout root_html_repeated_intrinsic_auto_row_list
run layout-matrix cargo test --locked -p spinon-layout six_member_repeated_intrinsic
run runtime-existing-regressions cargo test --locked -p spinon-runtime parsed_root_html_repeated_intrinsic -- --nocapture
run runtime-span-comparison cargo test --locked -p spinon-runtime parsed_root_html_later_fit_content_positions_match_all_1920_chrome_scenes -- --nocapture
run runtime-span-rejection cargo test --locked -p spinon-runtime parsed_root_html_later_fit_content_positions_4_through_6_reject_atomically -- --nocapture
run runtime-later-position-rejection cargo test --locked -p spinon-runtime parsed_root_html_later_fit_content_position_requires_the_repeated_text_profile
run runtime-matrix cargo test --locked -p spinon-runtime parsed_root_html_repeated_text_auto_placement_matches_all_fit_content_positions -- --nocapture
run runtime-trace-matrix cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_repeated_text_auto_placement_matches_all_fit_content_positions -- --nocapture
run clippy cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings
run rustfmt cargo fmt --all -- --check
run diff-check git diff --check

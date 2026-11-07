#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-root-html-fit-content-percentage-auto-rows-runtime-2026-11-08"
cd "$repo_root"

cargo test --locked -p spinon-style-to-layout \
  root_html_fit_content_percentage_auto_rows_require_the_bounded_definite_grid_shape \
  -- --nocapture > "$evidence_dir/checks/style-percentage-auto-rows.log" 2>&1
cargo test --locked -p spinon-style-to-layout \
  root_html_explicit_fit_content -- --nocapture \
  > "$evidence_dir/checks/style-explicit-fit-content-regression.log" 2>&1
cargo test --locked -p spinon-layout \
  root_html_fit_content_implicit_row -- --nocapture \
  > "$evidence_dir/checks/layout-fit-content-auto-rows.log" 2>&1
cargo test --locked -p spinon-runtime \
  parsed_root_html_fit_content_percentage_auto_rows -- --nocapture \
  > "$evidence_dir/checks/runtime-percentage-auto-rows.log" 2>&1
cargo test --locked -p spinon-runtime \
  parsed_root_html_fit_content_auto_rows -- --nocapture \
  > "$evidence_dir/checks/runtime-px-regression.log" 2>&1
cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime \
  --all-targets -- -D warnings > "$evidence_dir/checks/clippy.log" 2>&1
cargo fmt --all -- --check > "$evidence_dir/checks/fmt.log" 2>&1
git diff --check > "$evidence_dir/checks/diff-check.log" 2>&1

printf 'Focused projection, layout, runtime, Clippy, rustfmt, and diff checks passed.\n'

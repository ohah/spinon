#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
validation_dir="$repo_root/spec/internal/evidence/c11-4-generic-mixed-row-axis-runtime-2026-10-28/validation/archive-boundary"
archive="$repo_root/target/debug/libspinon_ffi.a"
routes=(
  direct-row-min-content-generic-forward
  direct-row-min-content-generic-reversed
  direct-row-max-content-generic-forward
  direct-row-max-content-generic-reversed
)

mkdir -p "$validation_dir"
mise exec -- cargo build --locked -p spinon-ffi \
  > "$validation_dir/default-build.log" 2>&1
shasum -a 256 "$archive" > "$validation_dir/default-archive.sha256"
for route in "${routes[@]}"; do
  set +e
  strings "$archive" | rg -F "$route" > "$validation_dir/default-$route.txt"
  probe_status=("${PIPESTATUS[@]}")
  set -e
  if [[ "${probe_status[0]}" -ne 0 || "${probe_status[1]}" -ne 1 ]]; then
    echo "FAIL: $route must be absent from the default spinon-ffi archive" >&2
    exit 1
  fi
  printf 'ABSENT: %s\n' "$route" > "$validation_dir/default-$route.txt"
done

mise exec -- cargo build --locked -p spinon-ffi \
  --features c04-runtime-gpu-fixture-test \
  > "$validation_dir/fixture-build.log" 2>&1
shasum -a 256 "$archive" > "$validation_dir/fixture-archive.sha256"
for route in "${routes[@]}"; do
  set +e
  strings "$archive" | rg -F "$route" > "$validation_dir/fixture-$route.txt"
  probe_status=("${PIPESTATUS[@]}")
  set -e
  if [[ "${probe_status[0]}" -ne 0 || "${probe_status[1]}" -ne 0 ]]; then
    echo "FAIL: $route must be present in the opt-in fixture archive" >&2
    exit 1
  fi
done

cat > "$validation_dir/result.txt" <<'RESULT'
PASS: all four generic mixed row-axis scene IDs are absent from the default spinon-ffi archive
PASS: all four generic mixed row-axis scene IDs are present in the opt-in c04-runtime-gpu-fixture-test archive
Scope: Rust static archive only; this does not establish fixture exclusion from release APK or IPA bundles.
RESULT

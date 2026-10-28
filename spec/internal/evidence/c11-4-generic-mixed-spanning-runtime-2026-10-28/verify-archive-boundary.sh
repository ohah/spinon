#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-generic-mixed-spanning-runtime-2026-10-28/archive-boundary"
archive="$repo_root/target/debug/libspinon_ffi.a"
markers=(
  mixed-text-image-span2-forward
  mixed-text-image-span2-reversed
  mixed-text-image-span2-before-span3
  mixed-text-image-span3-before-span2
  generic-host-root-
)

mkdir -p "$evidence_dir"
probe_archive() {
  local mode="$1"
  local expected="$2"
  : > "$evidence_dir/$mode-markers.txt"
  for marker in "${markers[@]}"; do
    local marker_file="$evidence_dir/$mode-${marker//[^a-zA-Z0-9-]/_}.txt"
    set +e
    strings "$archive" | rg -F "$marker" > "$marker_file"
    local probe_status=("${PIPESTATUS[@]}")
    set -e
    if [[ "${probe_status[0]}" -ne 0 ]]; then
      echo "FAIL: strings could not read the $mode spinon-ffi archive" >&2
      exit 1
    fi
    if [[ "$expected" == "present" && "${probe_status[1]}" -ne 0 ]]; then
      echo "FAIL: $marker must be present in the $mode spinon-ffi archive" >&2
      exit 1
    fi
    if [[ "$expected" == "absent" && "${probe_status[1]}" -ne 1 ]]; then
      echo "FAIL: $marker must be absent from the $mode spinon-ffi archive" >&2
      exit 1
    fi
    printf '%s: %s\n' "$expected" "$marker" >> "$evidence_dir/$mode-markers.txt"
  done
}

mise exec -- cargo build --locked -p spinon-ffi > "$evidence_dir/default-build.log" 2>&1
shasum -a 256 "$archive" > "$evidence_dir/default-archive.sha256"
probe_archive default absent

mise exec -- cargo build --locked -p spinon-ffi --features c04-runtime-gpu \
  > "$evidence_dir/product-build.log" 2>&1
shasum -a 256 "$archive" > "$evidence_dir/product-archive.sha256"
probe_archive product absent

mise exec -- cargo build --locked -p spinon-ffi \
  --features c04-runtime-gpu-fixture-test \
  > "$evidence_dir/fixture-build.log" 2>&1
shasum -a 256 "$archive" > "$evidence_dir/fixture-archive.sha256"
probe_archive fixture present

cat > "$evidence_dir/result.txt" <<'RESULT'
PASS: all four mixed text/image spanning scene IDs and the generic root ID marker are absent from default and c04-runtime-gpu product archives
PASS: all four mixed text/image spanning scene IDs and the generic root ID marker are present in the opt-in c04-runtime-gpu-fixture-test archive
Scope: Rust static archive only; this does not establish fixture exclusion from release APK or IPA bundles.
RESULT

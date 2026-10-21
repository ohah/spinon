#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-anonymous-grid-border-overflow-2026-10-21/feature-boundary"
artifact_dir="$repo_root/build/c11-4-anonymous-grid-border-overflow-artifacts"
default_archive="$artifact_dir/libspinon_ffi-default.a"
fixture_archive="$artifact_dir/libspinon_ffi-fixture.a"
default_symbols="$artifact_dir/default-symbols.txt"
fixture_symbols="$artifact_dir/fixture-symbols.txt"
symbol='spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture'
payload="$repo_root/tests/fixtures/resources/c14-1-png-basic.png"
rust_llvm_nm="$(mise exec -- rustc --print sysroot)/lib/rustlib/aarch64-apple-darwin/bin/llvm-nm"

mkdir -p "$evidence_dir" "$artifact_dir"
mise exec -- cargo build --locked -p spinon-ffi --lib --release --no-default-features \
  > "$evidence_dir/default-build.log" 2>&1
cp "$repo_root/target/release/libspinon_ffi.a" "$default_archive"
mise exec -- cargo build --locked -p spinon-ffi --lib --release \
  --features c04-runtime-gpu-fixture-test \
  > "$evidence_dir/fixture-build.log" 2>&1
cp "$repo_root/target/release/libspinon_ffi.a" "$fixture_archive"

"$rust_llvm_nm" --defined-only "$default_archive" > "$default_symbols" 2>/dev/null
"$rust_llvm_nm" --defined-only "$fixture_archive" > "$fixture_symbols" 2>/dev/null
if rg -F "$symbol" "$default_symbols" > /dev/null; then
  echo "fixture FFI symbol leaked into default staticlib" >&2
  exit 1
fi
rg -F "$symbol" "$fixture_symbols" > "$evidence_dir/fixture-symbol.txt"

python3 - "$default_archive" "$fixture_archive" "$payload" \
  > "$evidence_dir/byte-scan.json" <<'PY'
from hashlib import sha256
from pathlib import Path
import json
import sys

default_path, fixture_path, payload_path = map(Path, sys.argv[1:])
default = default_path.read_bytes()
fixture = fixture_path.read_bytes()
payload = payload_path.read_bytes()
markers = [
    "anonymous-runs-two-images-forward",
    "two-images-anonymous-runs-reversed",
    "c114-overflow-image-a-forward",
    "border:2px solid #f0c674",
    "https://spinon.invalid/c14-1-png-basic.png",
]
marker_results = {
    marker: {"default": marker.encode() in default, "fixture": marker.encode() in fixture}
    for marker in markers
}
if any(result["default"] or not result["fixture"] for result in marker_results.values()):
    raise SystemExit("fixture marker leaked into default or is missing from positive control")
if payload in default or payload not in fixture:
    raise SystemExit("exact PNG payload leaked into default or is missing from positive control")
print(json.dumps({
    "scope": "macOS ARM64 release spinon-ffi static archives",
    "archives": {
        "default": {"bytes": len(default), "sha256": sha256(default).hexdigest()},
        "fixture": {"bytes": len(fixture), "sha256": sha256(fixture).hexdigest()},
    },
    "symbol": "spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture",
    "symbolPresent": {"default": False, "fixture": True},
    "markers": marker_results,
    "exactPngPayload": {
        "bytes": len(payload),
        "default": payload in default,
        "fixture": payload in fixture,
    },
}, ensure_ascii=False, indent=2))
PY

echo "PASS: default/opt-in fixture artifact boundary"

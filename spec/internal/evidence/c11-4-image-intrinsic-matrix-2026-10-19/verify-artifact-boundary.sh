#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-image-intrinsic-matrix-2026-10-19/artifact-boundary"
artifact_dir="$repo_root/build/c11-4-artifact-boundary"
default_archive="$artifact_dir/libspinon_ffi-default.a"
fixture_archive="$artifact_dir/libspinon_ffi-fixture.a"
symbol='spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture'
payload="$repo_root/tests/fixtures/resources/c14-1-png-basic.png"
rust_llvm_nm="$(mise exec -- rustc --print sysroot)/lib/rustlib/aarch64-apple-darwin/bin/llvm-nm"

mkdir -p "$evidence_dir" "$artifact_dir"
mise exec -- cargo build --locked -p spinon-ffi --lib --release --no-default-features \
  > "$evidence_dir/default-build.log" 2>&1
cp "$repo_root/target/release/libspinon_ffi.a" "$default_archive"
mise exec -- cargo build --locked -p spinon-ffi --lib --release \
  --features c11-grid-metal-surface-pixels-test \
  > "$evidence_dir/fixture-build.log" 2>&1
cp "$repo_root/target/release/libspinon_ffi.a" "$fixture_archive"

if "$rust_llvm_nm" --defined-only "$default_archive" 2>/dev/null | rg -q "$symbol"; then
  echo "fixture FFI symbol leaked into default staticlib" >&2
  exit 1
fi
"$rust_llvm_nm" --defined-only "$fixture_archive" 2>/dev/null | rg "$symbol" \
  > "$evidence_dir/fixture-symbol.txt"

: > "$evidence_dir/fixture-strings.txt"
for marker in 'auto-image-definite' 'c114-auto-image-grid' \
    'https://spinon.invalid/c14-1-png-basic.png'; do
  if strings "$default_archive" | rg -oF -q "$marker"; then
    echo "fixture string leaked into default staticlib: $marker" >&2
    exit 1
  fi
  strings "$fixture_archive" | rg -oF "$marker" \
    >> "$evidence_dir/fixture-strings.txt"
done
LC_ALL=C sort -u "$evidence_dir/fixture-strings.txt" \
  -o "$evidence_dir/fixture-strings.txt"

python3 - "$payload" "$default_archive" "$fixture_archive" \
  > "$evidence_dir/png-payload-check.txt" <<'PY'
from pathlib import Path
import sys

payload_path, default_path, fixture_path = map(Path, sys.argv[1:])
payload = payload_path.read_bytes()
default_archive = default_path.read_bytes()
fixture_archive = fixture_path.read_bytes()
if payload in default_archive:
    raise SystemExit("fixture PNG payload leaked into default staticlib")
if payload not in fixture_archive:
    raise SystemExit("fixture PNG payload missing from positive-control staticlib")
print(f"PASS: exact {len(payload)}-byte PNG payload absent from default and present in fixture staticlib")
PY

{
  echo "default: $(stat -f '%z bytes' "$default_archive")"
  shasum -a 256 "$default_archive"
  echo "fixture: $(stat -f '%z bytes' "$fixture_archive")"
  shasum -a 256 "$fixture_archive"
  cat "$evidence_dir/png-payload-check.txt"
} > "$evidence_dir/archive-hashes.txt"

echo "PASS: default/positive-control staticlib feature boundary"

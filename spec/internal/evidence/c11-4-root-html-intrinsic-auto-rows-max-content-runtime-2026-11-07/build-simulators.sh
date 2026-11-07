#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-root-html-intrinsic-auto-rows-max-content-runtime-2026-11-07"
backup_dir="$repo_root/build/spinon/c11-4-maxcontent-bootstrap-backup-20261107"
android_log="$evidence_dir/android/build.log"
ios_log="$evidence_dir/ios/build.log"

test ! -e "$backup_dir"
test ! -e "$evidence_dir/android"
test ! -e "$evidence_dir/ios"
mkdir -p "$backup_dir" "$evidence_dir/android" "$evidence_dir/ios"
cp "$repo_root/examples/bootstrap/index.html" "$backup_dir/index.html"
cp "$repo_root/examples/bootstrap/root-app.js" "$backup_dir/root-app.js"

restore_bootstrap() {
  cp "$backup_dir/index.html" "$repo_root/examples/bootstrap/index.html"
  cp "$backup_dir/root-app.js" "$repo_root/examples/bootstrap/root-app.js"
}
trap restore_bootstrap EXIT HUP INT TERM

cp "$repo_root/tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-max-content-precomparison.html" "$repo_root/examples/bootstrap/index.html"
cat > "$repo_root/examples/bootstrap/root-app.js" <<'EOF'
globalThis.__spinonRootHtmlDefaultRouteSmoke = "max-content";
EOF

env SPINON_ENABLE_C04_RUNTIME_GPU=1 bash "$repo_root/tools/build-android-app.sh" > "$android_log" 2>&1
env SPINON_ENABLE_C04_RUNTIME_GPU=1 bash "$repo_root/tools/build-ios-sim.sh" > "$ios_log" 2>&1

restore_bootstrap
trap - EXIT HUP INT TERM
shasum -a 256 "$repo_root/examples/bootstrap/index.html" "$repo_root/examples/bootstrap/root-app.js"
printf 'Android and iOS simulator builds completed with c04_runtime_gpu=1 and fixture-test flags at defaults.\n'

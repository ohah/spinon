#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
evidence_dir="$(cd "$(dirname "$0")" && pwd)"
index_path="$repo_root/examples/bootstrap/index.html"
app_path="$repo_root/examples/bootstrap/root-app.js"

restore_sources() {
  local status=$?
  trap - EXIT
  cp "$backup_dir/index.html" "$index_path"
  cp "$backup_dir/root-app.js" "$app_path"
  (
    cd "$repo_root"
    shasum -a 256 examples/bootstrap/index.html examples/bootstrap/root-app.js
  ) > "$evidence_dir/source-restoration.log"
  rm -rf "$backup_dir"
  exit "$status"
}

cd "$repo_root"
test "$(shasum -a 256 examples/bootstrap/index.html | awk '{print $1}')" = \
  "e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9"
test "$(shasum -a 256 examples/bootstrap/root-app.js | awk '{print $1}')" = \
  "b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32"
backup_dir="$(mktemp -d /private/tmp/spinon-c114-five-fit-build.XXXXXX)"
cp "$index_path" "$backup_dir/index.html"
cp "$app_path" "$backup_dir/root-app.js"
trap restore_sources EXIT
cp "$evidence_dir/root-html-five-fit-auto-minimum-runtime.html" "$index_path"
cp "$evidence_dir/root-app-five-fit-auto-minimum-runtime.js" "$app_path"

printf '%s\n' '$ SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:ios-sim' \
  > "$evidence_dir/ios-build.log"
SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:ios-sim \
  >> "$evidence_dir/ios-build.log" 2>&1

printf '%s\n' '$ SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:android' \
  > "$evidence_dir/android-build.log"
SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:android \
  >> "$evidence_dir/android-build.log" 2>&1

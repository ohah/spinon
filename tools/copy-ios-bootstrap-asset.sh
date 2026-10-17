#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
resource_dir="${1:?Xcode 리소스 디렉터리가 필요합니다}"
source="$repo_root/build/spinon/bootstrap/app.js"
root_document_source="$repo_root/build/spinon/bootstrap/index.html"
root_app_source="$repo_root/build/spinon/bootstrap/root-app.js"
fixture_source="$repo_root/build/spinon/bootstrap/s03-lifecycle-probe.js"
if [[ ! -f "$source" || ! -f "$root_document_source" || ! -f "$root_app_source" ]]; then
  echo "Bun 번들이 없습니다. bun run bundle:bootstrap을 먼저 실행하세요." >&2
  exit 1
fi
mkdir -p "$resource_dir"
cp "$source" "$resource_dir/app.js"
cp "$root_document_source" "$resource_dir/index.html"
cp "$root_app_source" "$resource_dir/root-app.js"
if [[ -f "$fixture_source" ]]; then
  cp "$fixture_source" "$resource_dir/s03-lifecycle-probe.js"
else
  rm -f "$resource_dir/s03-lifecycle-probe.js"
fi
root_html_asset_fixture="$repo_root/build/spinon/bootstrap/c04-12-root-html-asset.html"
if [[ "${SPINON_ENABLE_C04_12_ROOT_HTML_FIXTURE_TEST:-0}" == "1" && -f "$root_html_asset_fixture" ]]; then
  cp "$root_html_asset_fixture" "$resource_dir/c04-12-root-html-asset.html"
else
  rm -f "$resource_dir/c04-12-root-html-asset.html"
fi

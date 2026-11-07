#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-track-list-runtime-2026-11-08"
ios_out="$evidence_dir/ios/final"
ios_pid="$(awk -F': ' '/dev\.spinon\.bootstrap:/ { print $2; exit }' "$ios_out/launch.log")"
test -n "$ios_pid"
rg -F "SpinonBootstrap[$ios_pid:" "$ios_out/app.log" > "$ios_out/app-current-process.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_SOURCE html_bytes=2183 ' "$ios_out/app-current-process.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=12' "$ios_out/app-current-process.log"
rg -q 'SPINON_C0410_ENVIRONMENT status=0 .*layout=ready boxes=12' "$ios_out/app-current-process.log"
rg -q 'page_scale=1([.]0+)?([[:space:]]|$)' "$ios_out/app-current-process.log"
test -s "$ios_out/screenshot.png"
printf 'Current iOS Simulator PID %s reported fixture source, layout=ready boxes=12 and page_scale=1; screenshot is present for visual review.\n' "$ios_pid"

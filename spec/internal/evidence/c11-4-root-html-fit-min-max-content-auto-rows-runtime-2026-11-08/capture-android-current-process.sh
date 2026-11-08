#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-runtime-2026-11-08"
android_out="$evidence_dir/android/final"
android_serial="emulator-5554"
android_package="dev.spinon.bootstrap"

android_pid="$(adb -s "$android_serial" shell pidof "$android_package" | tr -d '\r')"
test -n "$android_pid"
printf '%s\n' "$android_pid" > "$android_out/current-app-pid.txt"
adb -s "$android_serial" logcat -d -v threadtime \
  | awk -v app_pid="$android_pid" '$3 == app_pid' \
  > "$android_out/app-current-process.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_ASSET html_bytes=2375 ' "$android_out/app-current-process.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_SOURCE bytes=2375' "$android_out/app-current-process.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=14' "$android_out/app-current-process.log"
rg -q 'SPINON_C0410_DRAW status=0 presented boxes=14' "$android_out/app-current-process.log"
rg -q 'page_scale=1([.]0+)?([[:space:]]|$)' "$android_out/app-current-process.log"
adb -s "$android_serial" exec-out screencap -p > "$android_out/screenshot-final.png"
shasum -a 256 "$android_out/screenshot-final.png" > "$android_out/screenshot-final.sha256"
printf 'Current Android PID %s reported the fixture HTML hash, layout=ready boxes=14, draw-presented and page_scale=1.\n' "$android_pid"

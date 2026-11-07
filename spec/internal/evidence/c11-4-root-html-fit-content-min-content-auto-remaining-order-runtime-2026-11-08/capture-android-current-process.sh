#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08"
android_out="$evidence_dir/android/final"
android_serial="emulator-5554"
android_package="dev.spinon.bootstrap"
apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
ios_out="$evidence_dir/ios/final"
fixture="$repo_root/tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-remaining-order-runtime.html"
fixture_bytes="$(wc -c < "$fixture" | tr -d '[:space:]')"

android_pid="$(adb -s "$android_serial" shell pidof "$android_package" | tr -d '\r')"
test -n "$android_pid"
printf '%s\n' "$android_pid" > "$android_out/current-app-pid.txt"
for attempt in $(seq 1 45); do
  adb -s "$android_serial" logcat -d -v threadtime \
    | awk -v app_pid="$android_pid" '$3 == app_pid' > "$android_out/app-current-process.log"
  if rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=12' "$android_out/app-current-process.log" \
    && rg -q 'SPINON_C0410_DRAW status=0 presented boxes=12' "$android_out/app-current-process.log"; then break; fi
  sleep 1
done
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_ASSET html_bytes=$fixture_bytes " "$android_out/app-current-process.log"
rg -q "SPINON_ROOT_HTML_DEFAULT_APP_SOURCE bytes=$fixture_bytes" "$android_out/app-current-process.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready boxes=12' "$android_out/app-current-process.log"
rg -q 'SPINON_C0410_DRAW status=0 presented boxes=12' "$android_out/app-current-process.log"
rg -q 'page_scale=1([.]0+)?([[:space:]]|$)' "$android_out/app-current-process.log"
sleep 1
adb -s "$android_serial" exec-out screencap -p > "$android_out/screenshot.png"
shasum -a 256 "$android_out/screenshot.png" > "$android_out/screenshot.sha256"
shasum -a 256 \
  "$apk" "$ios_app/SpinonBootstrap" "$ios_app/index.html" \
  "$android_out/screenshot.png" "$ios_out/screenshot.png" \
  > "$evidence_dir/final-capture.sha256"
printf 'Current Android process %s presented the 12-box scene before screenshot capture.\n' "$android_pid"

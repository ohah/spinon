#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_dir="$(cd "$(dirname "$0")" && pwd)"
simulator_udid="ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D"
app_path="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
bundle_id="dev.spinon.bootstrap"

if xcrun simctl list devices booted | rg -q "$simulator_udid"; then
  echo "Refusing to take over an already booted simulator: $simulator_udid" >&2
  exit 2
fi

xcrun simctl boot "$simulator_udid"
cleanup() {
  xcrun simctl terminate "$simulator_udid" "$bundle_id" >/dev/null 2>&1 || true
  if [[ -n "${original_appearance:-}" ]]; then
    xcrun simctl ui "$simulator_udid" appearance "$original_appearance" >/dev/null 2>&1 || true
  fi
  xcrun simctl shutdown "$simulator_udid" >/dev/null 2>&1 || true
}
trap cleanup EXIT

xcrun simctl bootstatus "$simulator_udid" -b
original_style="$(xcrun simctl spawn "$simulator_udid" defaults read -g AppleInterfaceStyle 2>/dev/null || true)"
if [[ "$original_style" == "Dark" ]]; then
  original_appearance="dark"
else
  original_appearance="light"
fi
xcrun simctl install "$simulator_udid" "$app_path"

for appearance in light dark; do
  xcrun simctl ui "$simulator_udid" appearance "$appearance"
  xcrun simctl launch --terminate-running-process "$simulator_udid" "$bundle_id"
  sleep 3
  xcrun simctl spawn "$simulator_udid" log show --last 2m --style compact \
    --predicate 'process == "SpinonBootstrap"' \
    > "$evidence_dir/ios-$appearance.log"
  xcrun simctl io "$simulator_udid" screenshot "$evidence_dir/ios-default-$appearance.png" >/dev/null
  xcrun simctl terminate "$simulator_udid" "$bundle_id" >/dev/null 2>&1 || true
done

printf 'simulator=%s\noriginal_appearance=%s\nresult=light-dark captured; simulator shutdown by trap\n' \
  "$simulator_udid" "$original_appearance"

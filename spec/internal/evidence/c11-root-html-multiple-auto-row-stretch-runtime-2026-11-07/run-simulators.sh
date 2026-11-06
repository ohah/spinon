#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../../../.." && pwd)"
evidence_dir="$repo_root/spec/internal/evidence/c11-root-html-multiple-auto-row-stretch-runtime-2026-11-07"
pass_name="${1:-pass01}"
if [[ ! "$pass_name" =~ ^pass[0-9]{2}$ ]]; then
  echo "Pass name must use passNN format" >&2
  exit 1
fi
run_dir="$evidence_dir/$pass_name"
bootstrap_dir="$repo_root/build/spinon/bootstrap"
ios_app="$repo_root/build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app"
android_apk="$repo_root/platforms/android/app/build/outputs/apk/debug/app-debug.apk"
fixture="$repo_root/tests/fixtures/css/c11/root-html-multiple-auto-row-stretch-runtime.html"
root_app="$evidence_dir/root-app.js"
android_serial="emulator-5554"
ios_udid="9DCCE836-7659-4914-B6C1-13F54BFFA9DB"
bundle_id="dev.spinon.bootstrap"
android_sdk="/Users/yoonhb/Library/Android/sdk"
export ANDROID_HOME="$android_sdk"
export ANDROID_SDK_ROOT="$android_sdk"

if [[ -e "$run_dir" ]]; then
  echo "Refusing to overwrite existing simulator evidence: $run_dir" >&2
  exit 1
fi
for source in \
  "$fixture" \
  "$root_app" \
  "$repo_root/examples/bootstrap/index.html" \
  "$repo_root/examples/bootstrap/root-app.js" \
  "$android_apk" \
  "$ios_app/index.html" \
  "$ios_app/root-app.js"; do
  if [[ ! -s "$source" ]]; then
    echo "Missing required simulator input: $source" >&2
    exit 1
  fi
done
if [[ "$(adb -s "$android_serial" shell getprop ro.build.version.sdk | tr -d '\r')" != "37" ]]; then
  echo "Android target is not API 37: $android_serial" >&2
  exit 1
fi
if [[ "$(adb -s "$android_serial" shell getprop ro.product.cpu.abi | tr -d '\r')" != "arm64-v8a" ]]; then
  echo "Android target is not ARM64: $android_serial" >&2
  exit 1
fi
if [[ -n "$(adb -s "$android_serial" shell pidof "$bundle_id" | tr -d '\r' || true)" ]]; then
  echo "Refusing to replace a running Android app on $android_serial" >&2
  exit 1
fi
if ! xcrun simctl list devices available | rg -q "$ios_udid.*Booted"; then
  echo "Target iPhone 17 Pro simulator is not booted: $ios_udid" >&2
  exit 1
fi
if xcrun simctl spawn "$ios_udid" launchctl list | rg -q 'dev\.spinon\.bootstrap'; then
  echo "Refusing to replace a running iOS app on $ios_udid" >&2
  exit 1
fi

mkdir -p "$run_dir/android" "$run_dir/ios" "$run_dir/original/bootstrap"
cp -p "$repo_root/examples/bootstrap/index.html" "$run_dir/original/bootstrap/index.html"
cp -p "$repo_root/examples/bootstrap/root-app.js" "$run_dir/original/bootstrap/root-app.js"
printf 'candidate_head=%s\n' "$(git -C "$repo_root" rev-parse HEAD)" > "$run_dir/environment.txt"
printf 'fixture_sha256=%s\n' "$(shasum -a 256 "$fixture" | awk '{print $1}')" >> "$run_dir/environment.txt"
printf 'root_app_sha256=%s\n' "$(shasum -a 256 "$root_app" | awk '{print $1}')" >> "$run_dir/environment.txt"
printf 'android_serial=%s\nandroid_api=37\nandroid_abi=arm64-v8a\n' "$android_serial" >> "$run_dir/environment.txt"
printf 'ios_udid=%s\nios_device=iPhone 17 Pro\nios_runtime=iOS 26.2\n' "$ios_udid" >> "$run_dir/environment.txt"
printf 'fixture_feature_flags=all C11 fixture flags disabled; C04 runtime GPU enabled\n' >> "$run_dir/environment.txt"

backup_dir="$(mktemp -d "${TMPDIR:-/tmp}/spinon-c11-multi-auto-stretch.XXXXXX")"
: > "$backup_dir/.spinon-owned-temporary-backup"
had_android_install=0
android_install_checked=0
had_ios_install=0
ios_install_checked=0
had_old_ios_build=0
had_old_android_apk=0
had_old_bootstrap_assets=0
old_android_installed_sha=""
old_ios_installed_executable_sha=""
old_ios_build_executable_sha=""
old_android_build_apk_sha=""

restore_state() {
  local original_status=$?
  local restore_status=0
  trap - EXIT INT TERM

  adb -s "$android_serial" shell am force-stop "$bundle_id" >/dev/null 2>&1 || true
  xcrun simctl terminate "$ios_udid" "$bundle_id" >/dev/null 2>&1 || true

  cp -p "$run_dir/original/bootstrap/index.html" "$repo_root/examples/bootstrap/index.html" || restore_status=1
  cp -p "$run_dir/original/bootstrap/root-app.js" "$repo_root/examples/bootstrap/root-app.js" || restore_status=1
  cmp -s "$run_dir/original/bootstrap/index.html" "$repo_root/examples/bootstrap/index.html" || restore_status=1
  cmp -s "$run_dir/original/bootstrap/root-app.js" "$repo_root/examples/bootstrap/root-app.js" || restore_status=1

  if [[ "$had_old_bootstrap_assets" == 1 ]]; then
    mkdir -p "$bootstrap_dir"
    cp -p "$backup_dir/old-bootstrap-index.html" "$bootstrap_dir/index.html" || restore_status=1
    cp -p "$backup_dir/old-bootstrap-root-app.js" "$bootstrap_dir/root-app.js" || restore_status=1
    cmp -s "$backup_dir/old-bootstrap-index.html" "$bootstrap_dir/index.html" || restore_status=1
    cmp -s "$backup_dir/old-bootstrap-root-app.js" "$bootstrap_dir/root-app.js" || restore_status=1
  fi
  if [[ -d "$ios_app" ]]; then
    mv "$ios_app" "$backup_dir/candidate-ios.app" || restore_status=1
  fi
  if [[ "$had_old_ios_build" == 1 ]]; then
    mkdir -p "$(dirname "$ios_app")"
    mv "$backup_dir/old-build-ios.app" "$ios_app" || restore_status=1
    test "$(shasum -a 256 "$ios_app/SpinonBootstrap" | awk '{print $1}')" = \
      "$old_ios_build_executable_sha" || restore_status=1
  fi
  if [[ "$had_old_android_apk" == 1 ]]; then
    if [[ -f "$android_apk" ]]; then
      mv "$android_apk" "$backup_dir/candidate-app-debug.apk" || restore_status=1
    fi
    mkdir -p "$(dirname "$android_apk")"
    mv "$backup_dir/original-app-debug.apk" "$android_apk" || restore_status=1
    test "$(shasum -a 256 "$android_apk" | awk '{print $1}')" = \
      "$old_android_build_apk_sha" || restore_status=1
  fi

  if [[ "$android_install_checked" == 1 ]]; then
    if [[ "$had_android_install" == 1 ]]; then
      if adb -s "$android_serial" install -r "$backup_dir/old-installed.apk" \
        > "$run_dir/android/restore-installed-app.log" 2>&1; then
        printf 'Android previous installed APK reinstall exited 0.\n' \
          >> "$run_dir/android/restore-installed-app.log"
      else
        restore_status=1
      fi
      restored_android_path="$(adb -s "$android_serial" shell pm path "$bundle_id" \
        | tr -d '\r' | sed -n 's/^package://p' | head -1)"
      adb -s "$android_serial" pull "$restored_android_path" "$backup_dir/restored-installed.apk" \
        >> "$run_dir/android/restore-installed-app.log" || restore_status=1
      test "$(shasum -a 256 "$backup_dir/restored-installed.apk" | awk '{print $1}')" = \
        "$old_android_installed_sha" || restore_status=1
      printf 'android_restored_installed_apk_sha256=%s\n' \
        "$(shasum -a 256 "$backup_dir/restored-installed.apk" | awk '{print $1}')" \
        >> "$run_dir/environment.txt"
    else
      adb -s "$android_serial" uninstall "$bundle_id" \
        > "$run_dir/android/restore-installed-app.log" 2>&1 || true
    fi
  fi
  if [[ "$ios_install_checked" == 1 ]]; then
    if [[ "$had_ios_install" == 1 ]]; then
      if xcrun simctl install "$ios_udid" "$backup_dir/old-installed-ios.app" \
        > "$run_dir/ios/restore-installed-app.log" 2>&1; then
        printf 'iOS previous installed app reinstall exited 0.\n' \
          >> "$run_dir/ios/restore-installed-app.log"
      else
        restore_status=1
      fi
      restored_ios_app="$(xcrun simctl get_app_container "$ios_udid" "$bundle_id" app)"
      restored_ios_sha="$(shasum -a 256 "$restored_ios_app/SpinonBootstrap" | awk '{print $1}')"
      test "$restored_ios_sha" = "$old_ios_installed_executable_sha" || restore_status=1
      printf 'ios_restored_installed_executable_sha256=%s\n' "$restored_ios_sha" \
        >> "$run_dir/environment.txt"
    else
      xcrun simctl uninstall "$ios_udid" "$bundle_id" \
        > "$run_dir/ios/restore-installed-app.log" 2>&1 || true
    fi
  fi

  if [[ "$restore_status" == 0 ]]; then
    printf 'asset_and_simulator_restoration=PASS\n' >> "$run_dir/environment.txt"
    if [[ -f "$backup_dir/.spinon-owned-temporary-backup" ]]; then
      rm -rf "$backup_dir"
    else
      restore_status=1
    fi
  else
    printf 'asset_and_simulator_restoration=FAIL backup=%s\n' "$backup_dir" >> "$run_dir/environment.txt"
    echo "Restoration failed; preserved recovery data at $backup_dir" >&2
  fi
  if [[ "$original_status" != 0 ]]; then exit "$original_status"; fi
  exit "$restore_status"
}
trap restore_state EXIT INT TERM

if adb -s "$android_serial" shell pm path "$bundle_id" 2>/dev/null | rg -q 'package:'; then
  old_android_paths="$(adb -s "$android_serial" shell pm path "$bundle_id" \
    | tr -d '\r' | sed -n 's/^package://p')"
  if [[ "$(printf '%s\n' "$old_android_paths" | wc -l | tr -d ' ')" != 1 ]]; then
    echo "Refusing to replace a split-installed Android app without a complete backup" >&2
    exit 1
  fi
  old_android_installed_path="$old_android_paths"
  adb -s "$android_serial" pull "$old_android_installed_path" "$backup_dir/old-installed.apk" \
    > "$run_dir/android/preserve-installed-app.log" 2>&1
  old_android_installed_sha="$(shasum -a 256 "$backup_dir/old-installed.apk" | awk '{print $1}')"
  printf 'android_installed_base_apk_sha256=%s\n' \
    "$old_android_installed_sha" \
    >> "$run_dir/environment.txt"
  had_android_install=1
fi
android_install_checked=1
if ios_installed_app="$(xcrun simctl get_app_container "$ios_udid" "$bundle_id" app 2>/dev/null)"; then
  ditto "$ios_installed_app" "$backup_dir/old-installed-ios.app"
  had_ios_install=1
  old_ios_installed_executable_sha="$(shasum -a 256 "$backup_dir/old-installed-ios.app/SpinonBootstrap" | awk '{print $1}')"
  printf 'ios_installed_executable_sha256=%s\n' "$old_ios_installed_executable_sha" \
    >> "$run_dir/environment.txt"
fi
ios_install_checked=1
if [[ -d "$ios_app" ]]; then
  old_ios_build_executable_sha="$(shasum -a 256 "$ios_app/SpinonBootstrap" | awk '{print $1}')"
  printf 'ios_previous_build_executable_sha256=%s\n' "$old_ios_build_executable_sha" \
    >> "$run_dir/environment.txt"
  mv "$ios_app" "$backup_dir/old-build-ios.app"
  had_old_ios_build=1
fi
if [[ -f "$android_apk" ]]; then
  old_android_build_apk_sha="$(shasum -a 256 "$android_apk" | awk '{print $1}')"
  printf 'android_previous_build_apk_sha256=%s\n' "$old_android_build_apk_sha" \
    >> "$run_dir/environment.txt"
  mv "$android_apk" "$backup_dir/original-app-debug.apk"
  had_old_android_apk=1
fi
if [[ -s "$bootstrap_dir/index.html" && -s "$bootstrap_dir/root-app.js" ]]; then
  cp -p "$bootstrap_dir/index.html" "$backup_dir/old-bootstrap-index.html"
  cp -p "$bootstrap_dir/root-app.js" "$backup_dir/old-bootstrap-root-app.js"
  had_old_bootstrap_assets=1
fi

cp "$fixture" "$repo_root/examples/bootstrap/index.html"
cp "$root_app" "$repo_root/examples/bootstrap/root-app.js"
SPINON_ENABLE_C04_RUNTIME_GPU=1 bash "$repo_root/tools/build-android-app.sh" \
  > "$run_dir/android/build.log" 2>&1
SPINON_ENABLE_C04_RUNTIME_GPU=1 bash "$repo_root/tools/build-ios-sim.sh" \
  > "$run_dir/ios/build.log" 2>&1

fixture_hash="$(shasum -a 256 "$fixture" | awk '{print $1}')"
test "$(unzip -p "$android_apk" assets/index.html | shasum -a 256 | awk '{print $1}')" = "$fixture_hash"
test "$(shasum -a 256 "$ios_app/index.html" | awk '{print $1}')" = "$fixture_hash"
printf 'android_apk_sha256=%s\n' "$(shasum -a 256 "$android_apk" | awk '{print $1}')" > "$run_dir/android/artifact-sha256.txt"
printf 'android_fixture_sha256=%s\n' "$(unzip -p "$android_apk" assets/index.html | shasum -a 256 | awk '{print $1}')" >> "$run_dir/android/artifact-sha256.txt"
printf 'ios_executable_sha256=%s\n' "$(shasum -a 256 "$ios_app/SpinonBootstrap" | awk '{print $1}')" > "$run_dir/ios/artifact-sha256.txt"
printf 'ios_fixture_sha256=%s\n' "$(shasum -a 256 "$ios_app/index.html" | awk '{print $1}')" >> "$run_dir/ios/artifact-sha256.txt"

adb -s "$android_serial" install -r "$android_apk" > "$run_dir/android/install.log" 2>&1
adb -s "$android_serial" shell am start -W -n "$bundle_id/.MainActivity" > "$run_dir/android/launch.log" 2>&1
android_pid=""
for attempt in $(seq 1 100); do
  android_pid="$(adb -s "$android_serial" shell pidof "$bundle_id" | tr -d '\r' || true)"
  if [[ "$android_pid" =~ ^[0-9]+$ ]] \
      && adb -s "$android_serial" logcat -d -s SpinonBootstrap \
        | awk -v pid="$android_pid" '$3 == pid' \
        | rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass' \
      && adb -s "$android_serial" logcat -d -s SpinonBootstrap \
        | awk -v pid="$android_pid" '$3 == pid' \
        | rg -q 'SPINON_C0410_DRAW .*presented .*runtime_text_count=2'; then
    break
  fi
  sleep 0.2
done
if [[ ! "$android_pid" =~ ^[0-9]+$ ]]; then
  echo "Android app process PID was not observed" >&2
  exit 1
fi
adb -s "$android_serial" logcat -d -s SpinonBootstrap \
  | awk -v pid="$android_pid" '$3 == pid' > "$run_dir/android/runtime.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready' "$run_dir/android/runtime.log"
rg -q 'SPINON_C0410_DRAW .*presented .*runtime_text_count=2' "$run_dir/android/runtime.log"
adb -s "$android_serial" exec-out screencap -p > "$run_dir/android/simulator.png"
shasum -a 256 "$run_dir/android/simulator.png" > "$run_dir/android/screenshot-sha256.txt"
for android_property in \
  ro.product.manufacturer \
  ro.product.model \
  ro.product.device \
  ro.build.version.release \
  ro.build.version.sdk \
  ro.product.cpu.abi; do
  printf '%s=%s\n' "$android_property" \
    "$(adb -s "$android_serial" shell getprop "$android_property" | tr -d '\r')"
done > "$run_dir/android/device-properties.txt"

xcrun simctl install "$ios_udid" "$ios_app" > "$run_dir/ios/install.log" 2>&1
printf 'iOS candidate app install exited 0.\n' >> "$run_dir/ios/install.log"
xcrun simctl launch "$ios_udid" "$bundle_id" > "$run_dir/ios/launch.log"
ios_pid="$(awk -F': ' '/^dev\.spinon\.bootstrap: [0-9]+$/ { print $2 }' "$run_dir/ios/launch.log")"
if [[ ! "$ios_pid" =~ ^[0-9]+$ ]]; then
  echo "iOS app PID was not reported by simctl launch" >&2
  exit 1
fi
sleep 3
xcrun simctl spawn "$ios_udid" log show --last 3m --style compact \
  --predicate 'process == "SpinonBootstrap"' \
  | awk -v pid="$ios_pid" 'index($0, "SpinonBootstrap[" pid ":") > 0' \
  > "$run_dir/ios/runtime.log"
rg -q 'SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass .*layout=ready' "$run_dir/ios/runtime.log"
rg -q 'SPINON_C0410_DRAW .*presented .*runtime_text_count=2' "$run_dir/ios/runtime.log"
xcrun simctl io "$ios_udid" screenshot "$run_dir/ios/simulator.png" \
  > "$run_dir/ios/screenshot.log" 2>&1
shasum -a 256 "$run_dir/ios/simulator.png" > "$run_dir/ios/screenshot-sha256.txt"
xcrun simctl list devices available \
  | rg "$ios_udid" \
  | sed -E 's/[[:space:]]+$//' > "$run_dir/ios/device-list.txt"

printf 'android_result=PASS default Root HTML route, layout ready, two runtime text records\n' >> "$run_dir/environment.txt"
printf 'ios_result=PASS default Root HTML route, layout ready, two runtime text records\n' >> "$run_dir/environment.txt"

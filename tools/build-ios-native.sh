#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
v8_dir="${SPINON_V8_DIR:-$repo_root/build/v8-source/v8}"
v8_revision="$(cat "$repo_root/tools/v8/v8-revision.txt")"
platform_name="${PLATFORM_NAME:-iphonesimulator}"

case "${SPINON_ENABLE_R10_EXPERIMENT:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_R10_EXPERIMENT은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac

case "${SPINON_ENABLE_S04_IOS_FIXTURE:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_S04_IOS_FIXTURE은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac

case "${SPINON_ENABLE_S03_DOM_GC_FIXTURE:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_S03_DOM_GC_FIXTURE은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C04_RUNTIME_GPU은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_C14_1_APP_FIXTURE_TEST:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C14_1_APP_FIXTURE_TEST은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
if [[ "${SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE:-0}" == "1" \
      && "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" != "1" ]]; then
  echo "draw 실패 fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다." >&2
  exit 2
fi
if [[ "${SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST:-0}" == "1" \
      && "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" != "1" ]]; then
  echo "C13.1 overflow clip fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다." >&2
  exit 2
fi
if [[ "${SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST:-0}" == "1" \
      && "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" != "1" ]]; then
  echo "C13.2 scroll fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다." >&2
  exit 2
fi
if [[ "${SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST:-0}" == "1" \
      && "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" != "1" ]]; then
  echo "C13.3 gesture fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다." >&2
  exit 2
fi
if [[ "${SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST:-0}" == "1" \
      && "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" != "1" ]]; then
  echo "C12.4 worker stack 검증은 C04.10 runtime GPU fixture와 함께 켜야 합니다." >&2
  exit 2
fi
if [[ "${SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST:-0}" == "1" \
      && "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" != "1" ]]; then
  echo "C12.4 Metal surface readback은 C04.10 runtime GPU fixture와 함께 켜야 합니다." >&2
  exit 2
fi
if [[ "${SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST:-0}" == "1" \
      && "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" != "1" ]]; then
  echo "C11.1 Grid Metal pixel 표본은 C04.10 runtime GPU fixture와 함께 켜야 합니다." >&2
  exit 2
fi
s03_dom_gc_cpp_flag="-DSPINON_ENABLE_S03_DOM_GC_FIXTURE=${SPINON_ENABLE_S03_DOM_GC_FIXTURE:-0}"
runtime_gpu_cpp_flag="-DSPINON_ENABLE_C04_RUNTIME_GPU=${SPINON_ENABLE_C04_RUNTIME_GPU:-0}"
runtime_gpu_fixture_test_cpp_flag="-DSPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=${SPINON_ENABLE_C04_RUNTIME_GPU:-0}"
c1123_grid_fixture_cpp_flag="-DSPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=${SPINON_ENABLE_C04_RUNTIME_GPU:-0}"
ua_cascade_probe_cpp_flag="-DSPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE=${SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE:-0}"
runtime_gpu_failure_cpp_flag="-DSPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE=${SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE:-0}"
c124_surface_readback_cpp_flag="-DSPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST=${SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST:-0}"
c11_grid_surface_pixel_cpp_flag="-DSPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=${SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST:-0}"
c133_gesture_fixture_cpp_flag="-DSPINON_ENABLE_C133_GESTURE_FIXTURE_TEST=${SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST:-0}"
ffi_features=""
append_ffi_feature() {
  if [[ -n "$ffi_features" ]]; then
    ffi_features+=','
  fi
  ffi_features+="$1"
}
if [[ "${SPINON_ENABLE_R10_EXPERIMENT:-0}" == "1" ]]; then
  append_ffi_feature r10-experiment
fi
if [[ "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" == "1" ]]; then
  append_ffi_feature c04-runtime-gpu-fixture-test
  append_ffi_feature c11-2-3-grid-fixture-test
fi
if [[ "${SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST:-0}" == "1" ]]; then
  append_ffi_feature c13-1-overflow-clip-fixture-test
fi
if [[ "${SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST:-0}" == "1" ]]; then
  append_ffi_feature c13-2-scroll-fixture-test
fi
if [[ "${SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST:-0}" == "1" ]]; then
  append_ffi_feature c13-3-gesture-fixture-test
fi
if [[ "${SPINON_ENABLE_C14_1_APP_FIXTURE_TEST:-0}" == "1" ]]; then
  append_ffi_feature c14-1-app-fixture-test
fi
if [[ "${SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE:-0}" == "1" ]]; then
  append_ffi_feature c04-runtime-ua-cascade-fixture-test
fi
if [[ "${SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE:-0}" == "1" ]]; then
  append_ffi_feature c04-runtime-gpu-test-hooks
fi
if [[ "${SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST:-0}" == "1" ]]; then
  append_ffi_feature c12-4-metal-surface-readback-test
fi
if [[ "${SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST:-0}" == "1" ]]; then
  append_ffi_feature c11-grid-metal-surface-pixels-test
fi
if [[ "${SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST:-0}" == "1" ]]; then
  append_ffi_feature c12-4-worker-stack-residency-test
fi
echo "SPINON_IOS_NATIVE_FEATURES ffi=${ffi_features:-none} c11_grid_cpp=${c11_grid_surface_pixel_cpp_flag} c13_1_fixture=${SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST:-0} c13_2_fixture=${SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST:-0} c13_3_fixture=${SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST:-0} c14_1_app_fixture=${SPINON_ENABLE_C14_1_APP_FIXTURE_TEST:-0}"

case "$platform_name" in
  iphonesimulator)
    v8_out="out/boson-ios-sim"
    rust_target="aarch64-apple-ios-sim"
    rustup_target="aarch64-apple-ios-sim"
    sdk="iphonesimulator"
    target="arm64-apple-ios18.0-simulator"
    ;;
  iphoneos)
    v8_out="out/boson-ios-device"
    rust_target="aarch64-apple-ios"
    rustup_target="aarch64-apple-ios"
    sdk="iphoneos"
    target="arm64-apple-ios18.0"
    ;;
  *)
    echo "지원하지 않는 iOS 플랫폼입니다: $platform_name" >&2
    exit 1
    ;;
esac

if [[ ! -d "$v8_dir" ]] || [[ "$(git -C "$v8_dir" rev-parse HEAD 2>/dev/null || true)" != "$v8_revision" ]]; then
  echo "V8 소스가 고정 커밋과 다릅니다. 먼저 bash tools/v8/checkout.sh를 실행하세요." >&2
  exit 1
fi
v8_archive="$v8_dir/$v8_out/obj/libv8_monolith.a"
if [[ ! -f "$v8_archive" ]]; then
  echo "iOS용 V8 산출물이 없습니다: $v8_archive" >&2
  echo "먼저 bash tools/v8/build-ios-${sdk}.sh를 실행하세요." >&2
  exit 1
fi

if command -v mise >/dev/null 2>&1; then
  mise exec -- bun run bundle:bootstrap
  mise exec -- rustup target add "$rustup_target"
  if [[ -n "$ffi_features" ]]; then
    mise exec -- env CARGO_PROFILE_RELEASE_PANIC=abort cargo build --locked --release --target "$rust_target" -p spinon-ffi --features "$ffi_features"
  else
    mise exec -- env CARGO_PROFILE_RELEASE_PANIC=abort cargo build --locked --release --target "$rust_target" -p spinon-ffi
  fi
  if [[ "${SPINON_ENABLE_S04_IOS_FIXTURE:-0}" == "1" ]]; then
    mise exec -- cargo build --manifest-path "$repo_root/spikes/wgpu-backend/Cargo.toml" --locked --release --target "$rust_target" --features s04-ios-fixture
  else
    mise exec -- cargo build --manifest-path "$repo_root/spikes/wgpu-backend/Cargo.toml" --locked --release --target "$rust_target"
  fi
else
  bun run bundle:bootstrap
  rustup target add "$rustup_target"
  if [[ -n "$ffi_features" ]]; then
    CARGO_PROFILE_RELEASE_PANIC=abort cargo build --locked --release --target "$rust_target" -p spinon-ffi --features "$ffi_features"
  else
    CARGO_PROFILE_RELEASE_PANIC=abort cargo build --locked --release --target "$rust_target" -p spinon-ffi
  fi
  if [[ "${SPINON_ENABLE_S04_IOS_FIXTURE:-0}" == "1" ]]; then
    cargo build --manifest-path "$repo_root/spikes/wgpu-backend/Cargo.toml" --locked --release --target "$rust_target" --features s04-ios-fixture
  else
    cargo build --manifest-path "$repo_root/spikes/wgpu-backend/Cargo.toml" --locked --release --target "$rust_target"
  fi
fi

bootstrap_assets="$repo_root/build/spinon/bootstrap"
lifecycle_probe="$repo_root/tests/fixtures/dom/s03/runtime-lifecycle-probe-v1.js"
mkdir -p "$bootstrap_assets"
if [[ "${SPINON_ENABLE_S03_DOM_GC_FIXTURE:-0}" == "1" ]]; then
  cp "$lifecycle_probe" "$bootstrap_assets/s03-lifecycle-probe.js"
else
  rm -f "$bootstrap_assets/s03-lifecycle-probe.js"
fi

output_dir="$repo_root/build/spinon/$platform_name"
mkdir -p "$output_dir"
sdk_path="$(xcrun --sdk "$sdk" --show-sdk-path)"
xcrun --sdk "$sdk" clang++ -std=c++20 -O2 -fPIC -target "$target" \
  -isysroot "$sdk_path" -I"$v8_dir/include" "$s03_dom_gc_cpp_flag" "$runtime_gpu_cpp_flag" \
  "$runtime_gpu_fixture_test_cpp_flag" \
  "$c1123_grid_fixture_cpp_flag" \
  "$ua_cascade_probe_cpp_flag" \
  "$runtime_gpu_failure_cpp_flag" "$c124_surface_readback_cpp_flag" \
  "$c11_grid_surface_pixel_cpp_flag" "$c133_gesture_fixture_cpp_flag" \
  -I"$repo_root/native/v8/include" \
  -c "$repo_root/native/v8/src/spinon_v8.cc" \
  -o "$output_dir/spinon_v8.o"
cp "$repo_root/target/$rust_target/release/libspinon_ffi.a" \
  "$output_dir/libspinon_ffi.a"
cp "$repo_root/spikes/wgpu-backend/target/$rust_target/release/libspinon_wgpu_r08_spike.a" \
  "$output_dir/libspinon_wgpu_r08_spike.a"
echo "iOS $platform_name 네이티브 입력 준비 완료: $output_dir"

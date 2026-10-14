#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
v8_dir="${SPINON_V8_DIR:-$repo_root/build/v8-source/v8}"
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
if [[ "${SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST:-0}" == "1" \
      && "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" != "1" ]]; then
  echo "C12.4 worker stack 검증은 C04.10 runtime GPU fixture와 함께 켜야 합니다." >&2
  exit 2
fi
if [[ "${SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST:-0}" == "1" \
      && "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" != "1" ]]; then
  echo "C13.1 overflow clip fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다." >&2
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
echo "SPINON_IOS_SIM_BUILD_FLAGS c04_runtime_gpu=${SPINON_ENABLE_C04_RUNTIME_GPU:-0} c04_runtime_gpu_fixture_test=${SPINON_ENABLE_C04_RUNTIME_GPU:-0} c04_ua_cascade_fixture_probe=${SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE:-0} c11_2_3_grid_fixture=${SPINON_ENABLE_C04_RUNTIME_GPU:-0} c11_grid_metal_surface_pixels=${SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST:-0} c13_1_overflow_clip_fixture=${SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST:-0} c14_1_app_fixture_test=${SPINON_ENABLE_C14_1_APP_FIXTURE_TEST:-0}"

if command -v mise >/dev/null 2>&1; then
  mise exec -- xcodebuild \
    -project "$repo_root/platforms/ios/SpinonBootstrap.xcodeproj" \
    -scheme SpinonBootstrap \
    -sdk iphonesimulator \
    -destination 'generic/platform=iOS Simulator' \
    -derivedDataPath "$repo_root/build/spinon/DerivedData" \
    "SPINON_V8_ROOT=$v8_dir" \
    "SPINON_ENABLE_S04_IOS_FIXTURE=${SPINON_ENABLE_S04_IOS_FIXTURE:-0}" \
    "SPINON_ENABLE_S03_DOM_GC_FIXTURE=${SPINON_ENABLE_S03_DOM_GC_FIXTURE:-0}" \
    "SPINON_ENABLE_C04_RUNTIME_GPU=${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" \
    "SPINON_ENABLE_C14_1_APP_FIXTURE_TEST=${SPINON_ENABLE_C14_1_APP_FIXTURE_TEST:-0}" \
    "SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=${SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST:-0}" \
    "SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" \
    "SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" \
    "SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE=${SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE:-0}" \
    "SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE=${SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE:-0}" \
    "SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST=${SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST:-0}" \
    "SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=${SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST:-0}" \
    "SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST=${SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST:-0}" \
    CODE_SIGNING_ALLOWED=NO build
else
  xcodebuild \
    -project "$repo_root/platforms/ios/SpinonBootstrap.xcodeproj" \
    -scheme SpinonBootstrap \
    -sdk iphonesimulator \
    -destination 'generic/platform=iOS Simulator' \
    -derivedDataPath "$repo_root/build/spinon/DerivedData" \
    "SPINON_V8_ROOT=$v8_dir" \
    "SPINON_ENABLE_S04_IOS_FIXTURE=${SPINON_ENABLE_S04_IOS_FIXTURE:-0}" \
    "SPINON_ENABLE_S03_DOM_GC_FIXTURE=${SPINON_ENABLE_S03_DOM_GC_FIXTURE:-0}" \
    "SPINON_ENABLE_C04_RUNTIME_GPU=${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" \
    "SPINON_ENABLE_C14_1_APP_FIXTURE_TEST=${SPINON_ENABLE_C14_1_APP_FIXTURE_TEST:-0}" \
    "SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=${SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST:-0}" \
    "SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" \
    "SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" \
    "SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE=${SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE:-0}" \
    "SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE=${SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE:-0}" \
    "SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST=${SPINON_ENABLE_C124_METAL_SURFACE_READBACK_TEST:-0}" \
    "SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=${SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST:-0}" \
    "SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST=${SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST:-0}" \
    CODE_SIGNING_ALLOWED=NO build
fi

#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
source "$repo_root/tools/android-ndk.sh"
v8_dir="${SPINON_V8_DIR:-$repo_root/build/v8-source/v8}"
v8_revision="$(cat "$repo_root/tools/v8/v8-revision.txt")"
v8_out="out/boson-android-mac"
v8_archive="$v8_dir/$v8_out/obj/libv8_monolith.a"
v8_libcxx="$v8_dir/$v8_out/obj/buildtools/third_party/libc++/libc++.a"
v8_libcxxabi="$v8_dir/$v8_out/obj/buildtools/third_party/libc++abi/libc++abi.a"
ndk_dir="$(spinon_android_ndk_dir "$repo_root")"

case "${SPINON_ENABLE_R10_EXPERIMENT:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_R10_EXPERIMENT은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
case "${SPINON_ENABLE_S04_ANDROID_FIXTURE:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_S04_ANDROID_FIXTURE은 0 또는 1이어야 합니다." >&2
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
case "${SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE:-0}" in
  0|1) ;;
  *)
    echo "SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE은 0 또는 1이어야 합니다." >&2
    exit 2
    ;;
esac
if [[ "${SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE:-0}" == "1" \
      && "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" != "1" ]]; then
  echo "draw 실패 fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다." >&2
  exit 2
fi
s03_dom_gc_cpp_flag="-DSPINON_ENABLE_S03_DOM_GC_FIXTURE=${SPINON_ENABLE_S03_DOM_GC_FIXTURE:-0}"
s04_runtime_gpu_cpp_flag="-DSPINON_ENABLE_C04_RUNTIME_GPU=${SPINON_ENABLE_C04_RUNTIME_GPU:-0}"
s04_runtime_gpu_failure_cpp_flag="-DSPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE=${SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE:-0}"
s04_android_cpp_flags=()
if [[ "${SPINON_ENABLE_S04_ANDROID_FIXTURE:-0}" == "1" ]]; then
  s04_android_cpp_flags=(-DSPINON_ENABLE_S04_ANDROID_FIXTURE=1)
fi
ffi_features=()
if [[ "${SPINON_ENABLE_R10_EXPERIMENT:-0}" == "1" ]]; then
  ffi_features+=(r10-experiment)
fi
if [[ "${SPINON_ENABLE_C04_RUNTIME_GPU:-0}" == "1" ]]; then
  ffi_features+=(c04-runtime-gpu)
fi
if [[ "${SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE:-0}" == "1" ]]; then
  ffi_features+=(c04-runtime-gpu-test-hooks)
fi
ffi_feature_csv=""
if ((${#ffi_features[@]} > 0)); then
  ffi_feature_csv="$(IFS=,; printf '%s' "${ffi_features[*]}")"
fi

build_spinon_ffi() {
  if [[ -n "$ffi_feature_csv" ]]; then
    "$@" --features "$ffi_feature_csv"
  else
    "$@"
  fi
}

if [[ ! -d "$v8_dir" ]] || [[ "$(git -C "$v8_dir" rev-parse HEAD 2>/dev/null || true)" != "$v8_revision" ]]; then
  echo "V8 소스가 고정 커밋과 다릅니다. 먼저 bash tools/v8/checkout.sh를 실행하세요." >&2
  exit 1
fi
for required in "$v8_archive" "$v8_libcxx" "$v8_libcxxabi"; do
  if [[ ! -f "$required" ]]; then
    echo "Android용 V8 산출물이 없습니다: $required" >&2
    echo "먼저 bash tools/v8/build-android-macos.sh를 실행하세요." >&2
    exit 1
  fi
done
ndk_root="$(find "$ndk_dir/toolchains/llvm/prebuilt" -mindepth 1 -maxdepth 1 -type d | head -n 1)"
unwind_candidates=("$ndk_root"/lib/clang/*/lib/linux/aarch64/libunwind.a)
if [[ ${#unwind_candidates[@]} -ne 1 || ! -f "${unwind_candidates[0]}" ]]; then
  echo "고정 Android NDK에서 AArch64 libunwind.a를 하나만 찾을 수 없습니다: $ndk_root" >&2
  exit 1
fi
unwind_archive="${unwind_candidates[0]}"
ndk_llvm_nm="$ndk_root/bin/llvm-nm"
if [[ ! -x "$ndk_llvm_nm" ]] || ! "$ndk_llvm_nm" -g "$unwind_archive" | grep -Eq '[[:space:]]+[TtWw][[:space:]]+_Unwind_Resume$'; then
  echo "Android AArch64 libunwind.a에 필요한 _Unwind_Resume 정의가 없습니다: $unwind_archive" >&2
  exit 1
fi
v8_cxx="$v8_dir/third_party/llvm-build/Release+Asserts/bin/clang++"
v8_lld="$v8_dir/third_party/llvm-build/Release+Asserts/bin/ld.lld"
if [[ ! -x "$v8_cxx" || ! -x "$v8_lld" ]]; then
  echo "V8 도구 체인이 없습니다. tools/v8/build-android-macos.sh 결과를 확인하세요." >&2
  exit 1
fi

if command -v mise >/dev/null 2>&1; then
  mise exec -- bun run bundle:bootstrap
  build_spinon_ffi mise exec -- env CARGO_PROFILE_RELEASE_PANIC=abort cargo build --locked --release --target aarch64-linux-android -p spinon-ffi
  if [[ "${SPINON_ENABLE_S04_ANDROID_FIXTURE:-0}" == "1" ]]; then
    mise exec -- cargo build --manifest-path "$repo_root/spikes/wgpu-backend/Cargo.toml" --locked --release --target aarch64-linux-android --features s04-android-fixture
  else
    mise exec -- cargo build --manifest-path "$repo_root/spikes/wgpu-backend/Cargo.toml" --locked --release --target aarch64-linux-android
  fi
else
  bun run bundle:bootstrap
  build_spinon_ffi env CARGO_PROFILE_RELEASE_PANIC=abort cargo build --locked --release --target aarch64-linux-android -p spinon-ffi
  if [[ "${SPINON_ENABLE_S04_ANDROID_FIXTURE:-0}" == "1" ]]; then
    cargo build --manifest-path "$repo_root/spikes/wgpu-backend/Cargo.toml" --locked --release --target aarch64-linux-android --features s04-android-fixture
  else
    cargo build --manifest-path "$repo_root/spikes/wgpu-backend/Cargo.toml" --locked --release --target aarch64-linux-android
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

output_dir="$repo_root/build/spinon/android"
mkdir -p "$output_dir/obj" "$output_dir/jniLibs/arm64-v8a"
target="--target=aarch64-linux-android29"
sysroot="--sysroot=$ndk_root/sysroot"
common=("$target" "$sysroot" -std=c++20 -O2 -fPIC
  -fexperimental-relative-c++-abi-vtables -nostdinc++
  "$s04_runtime_gpu_cpp_flag"
  "$s04_runtime_gpu_failure_cpp_flag"
  -D_LIBCPP_HARDENING_MODE=_LIBCPP_HARDENING_MODE_EXTENSIVE
  -I"$v8_dir/buildtools/third_party/libc++"
  -isystem "$v8_dir/third_party/libc++/src/include"
  -isystem "$v8_dir/third_party/libc++abi/src/include"
  -I"$v8_dir/include"
  -I"$repo_root/native/v8/include"
  -I"$repo_root/crates/spinon-ffi/include"
  -I"$repo_root/spikes/wgpu-backend/include")

"$v8_cxx" "${common[@]}" "$s03_dom_gc_cpp_flag" \
  -c "$repo_root/native/v8/src/spinon_v8.cc" \
  -o "$output_dir/obj/spinon_v8.o"
if [[ "${SPINON_ENABLE_S04_ANDROID_FIXTURE:-0}" == "1" ]]; then
  "$v8_cxx" "${common[@]}" "$s04_runtime_gpu_cpp_flag" -I"$repo_root/platforms/android/app/src/main/cpp" \
    "${s04_android_cpp_flags[@]}" \
    -c "$repo_root/platforms/android/app/src/main/cpp/spinon_jni.cc" \
    -o "$output_dir/obj/spinon_jni.o"
else
  "$v8_cxx" "${common[@]}" -I"$repo_root/platforms/android/app/src/main/cpp" \
    -c "$repo_root/platforms/android/app/src/main/cpp/spinon_jni.cc" \
    -o "$output_dir/obj/spinon_jni.o"
fi

# 고정 NDK의 unwind ABI를 정적으로 포함하고 DSO 외부 심볼로는 내보내지 않는다.
"$v8_cxx" "${common[@]}" -shared \
  "$output_dir/obj/spinon_jni.o" \
  "$output_dir/obj/spinon_v8.o" \
  "$repo_root/target/aarch64-linux-android/release/libspinon_ffi.a" \
  "$repo_root/spikes/wgpu-backend/target/aarch64-linux-android/release/libspinon_wgpu_r08_spike.a" \
  "$v8_archive" "$v8_libcxx" "$v8_libcxxabi" "$unwind_archive" \
  "-fuse-ld=$v8_lld" -Wl,--gc-sections -Wl,--exclude-libs,libunwind.a \
  -Wl,-z,max-page-size=16384 -Wl,-z,common-page-size=16384 \
  -nostdlib++ --unwindlib=none -landroid -llog -ldl \
  -o "$output_dir/jniLibs/arm64-v8a/libspinon_bootstrap.so"

echo "Android ARM64 unwind runtime: $unwind_archive"
echo "Android ARM64 V8 smoke 라이브러리 준비 완료: $output_dir/jniLibs/arm64-v8a/libspinon_bootstrap.so"

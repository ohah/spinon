# C13.1 fixture archive 경계 확인 · 2026-10-18

두 빌드는 분리된 `CARGO_TARGET_DIR`에서 수행했다.

```sh
CARGO_TARGET_DIR=/tmp/spinon-c13-1-feature-boundary-default cargo build --locked -p spinon-ffi --lib
CARGO_TARGET_DIR=/tmp/spinon-c13-1-feature-boundary-opt-in cargo build --locked -p spinon-ffi --lib --features c13-1-overflow-clip-fixture-test
```

이 검사는 Rust host `aarch64-apple-darwin`, rustc `1.96.1`의 debug static archive를 대상으로 한다. 최종 Android APK·iOS app bundle 또는 release binary 검사로 해석하지 않는다. Archive SHA-256은 default `cff4f80e4a3293d1ac4989451d6415d8c110e9c6ac79cb88f288c8ca1377f341`, opt-in `0d1ed06083f7780622b5fa61e5d26142c6b69d7e5f3ba788d3f58e604e88e9ac`다.

`nm -g`에서 default archive의 C13.1 host export는 0개였고 opt-in positive control은 8개였다.

- `_spinon_runtime_gpu_host_eval_c13_1_compact_surface_matrix_fixture`
- `_spinon_runtime_gpu_host_eval_c13_1_overflow_clip_border_fixture`
- `_spinon_runtime_gpu_host_eval_c13_1_overflow_clip_fixture`
- `_spinon_runtime_gpu_host_new_c13_1_compact_surface_matrix_fixture`
- `_spinon_runtime_gpu_host_new_c13_1_compact_surface_pixel_readback_fixture`
- `_spinon_runtime_gpu_host_new_c13_1_overflow_clip_border_fixture`
- `_spinon_runtime_gpu_host_new_c13_1_overflow_clip_fixture`
- `_spinon_runtime_gpu_host_request_c13_1_surface_pixels_for_test`

`strings`를 `C13\.1|c13_1|c13-1-overflow`로 찾은 결과는 default archive 0건, opt-in archive 1,416건이었다. 따라서 이 컴파일 조합에서 C13.1 FFI symbol 및 고정 fixture payload는 명시적 opt-in에서만 포함됐다.

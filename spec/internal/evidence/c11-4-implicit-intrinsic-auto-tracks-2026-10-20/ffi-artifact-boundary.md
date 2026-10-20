# C11.4 fixture artifact boundary

이 검사는 기본 `spinon-ffi` 정적 라이브러리에서 C11.4 fixture ABI와 20개 고정 scene ID가 빠지고, opt-in fixture-test 조합에서만 들어가는지 실제 `.a` 산출물의 심볼과 문자열을 확인한다. 이 결과는 debug 정적 라이브러리 범위이며 Android/iOS 앱 번들이나 release 크기를 증명하지 않는다.

## 빌드

```sh
mise exec -- cargo build --locked --no-default-features -p spinon-ffi --lib
mise exec -- cargo build --locked --no-default-features -p spinon-ffi --lib --features c11-grid-metal-surface-pixels-test
```

`c11-grid-metal-surface-pixels-test`는 `c04-runtime-gpu-fixture-test`를 포함한다. 따라서 두 번째 산출물은 fixture ABI와 pixel-readback test hook을 함께 opt-in 한다.

## 산출물 검사

| 조합 | 정적 라이브러리 | 크기 | SHA-256 | C11.4 scene ID | C11.4 FFI 심볼 |
| --- | --- | ---: | --- | ---: | --- |
| 기본 (`features: []`) | `target/debug/deps/libspinon_ffi-0098a99528f2b0e6.a` | 508,223,384 bytes | `ec0ad7e42176fff6499c947109138862310e9c69755269d46e2a3b622052f881` | 0/20 | 0 |
| opt-in (`c04-runtime-gpu`, `c04-runtime-gpu-fixture-test`, `c11-grid-metal-surface-pixels-test`) | `target/debug/deps/libspinon_ffi-47a89cbf773ebb1a.a` | 676,619,888 bytes | `96b3261b56a1ee2d83d3227cc93fd1543fa15016088393a00d43d8bd29355795` | 20/20 | 3 |

scene ID 검사는 `implicit_intrinsic_tracks.rs`에서 추출한 20개 ID를 각 archive의 raw bytes에서 직접 검색했다. 기본 archive에서는 하나도 발견되지 않았고 opt-in archive에서는 전부 발견됐다.

Rust 1.96/LLVM 22가 만든 archive는 Xcode의 `nm`이 읽지 못해 `Unknown attribute kind` 오류를 냈다. 동일 Rust toolchain에 포함된 `llvm-nm`으로 확인했다. 기본 산출물에는 아래 C11.4 외부 심볼이 없고 opt-in 산출물에는 세 심볼이 모두 있다.

```text
_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture
_spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture
_spinon_runtime_gpu_host_request_c11_4_grid_surface_pixels_for_test
```

두 파일은 ignored `target/` 아래의 빌드 산출물이며 저장소에 추가하지 않았다.

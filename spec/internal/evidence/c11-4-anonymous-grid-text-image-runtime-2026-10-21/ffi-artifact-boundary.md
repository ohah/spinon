# C11.4 anonymous text/image FFI artifact 경계

검사는 macOS ARM64에서 만든 `spinon-ffi` 정적 라이브러리 두 개의 raw strings와 외부 심볼을 비교한다. 앱 번들 크기, release archive, Android/iOS 실행 파일의 전체 binary 크기를 대변하지 않는다.

## 빌드

```sh
mise exec -- cargo build --locked --no-default-features -p spinon-ffi --lib
mise exec -- cargo build --locked --no-default-features -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test
```

## 결과

| 구성 | archive | bytes | SHA-256 | 새 scene ID | 새 C fixture ABI |
| --- | --- | ---: | --- | ---: | ---: |
| 기본, feature 없음 | `target/debug/deps/libspinon_ffi-0098a99528f2b0e6.a` | 511,455,552 | `de2f87fb4429acf06428413b53e3ebd06a645f87f6fc77c4695694adad133808` | 0/5 | 0 |
| opt-in `c04-runtime-gpu-fixture-test` | `target/debug/deps/libspinon_ffi-311310be74a475a6.a` | 683,061,704 | `53f87324ca36d797968dff4401d34d5d4c987a2eacf24ffad56cccbb978c7f64` | 5/5 | 2 |

양 archive의 strings를 `strings | rg`로 대조했고 심볼은 Rust 1.96 toolchain `llvm-nm -g`로 확인했다. 기본 archive에는 아래 다섯 scene ID와 신규 심볼이 없었다. opt-in archive에서는 다섯 ID와 두 ABI 심볼이 모두 나왔다.

```text
_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture
_spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture
```

`c04-runtime-gpu-fixture-test`는 C11.4 runtime fixture feature를 포함한다. 결과는 fixture 경로가 기본 Rust FFI artifact에 노출되지 않는다는 증거이며, 다른 fixture ABI의 전체 기본/opt-in 격리 결과는 아니다.

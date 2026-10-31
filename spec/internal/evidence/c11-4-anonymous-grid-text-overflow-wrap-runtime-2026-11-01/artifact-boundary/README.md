# C11.4 anonymous Grid text `overflow-wrap` FFI archive boundary

두 archive는 동일한 macOS ARM64 checkout과 Rust 1.96.1에서 별도 `CARGO_TARGET_DIR`로 `--locked --release -p spinon-ffi --lib` 빌드했다. 기본 빌드에는 feature를 지정하지 않았다. fixture positive control은 `c04-runtime-gpu-fixture-test`를 켰으며 이 feature가 `spinon-runtime/c11-4-grid-track-sizing-fixture-test`를 포함한다.

| 빌드 | Archive SHA-256 | 신규 scene marker | C11.4 fixture host export |
|---|---|---:|---:|
| 기본 | `d8d75e4b9e204ddd120bc5c9136dd47fd460e9d9d5db66fbccd44a97745355f5` | 0/9 | 0 |
| `c04-runtime-gpu-fixture-test` | `8ff98d9de182b43ca204db7171efbd6781a641e134d41ca2e2d3d1c3ead1545b` | 9/9 | 2 |

검사 명령은 archive의 `strings -a`에서 신규 scene ID를 추출하고 Rust 1.96.1 sysroot의 `llvm-nm -g`에서 `spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`와 `spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture` export를 확인했다. [fixture marker 목록](./fixture-scene-markers.txt)과 [fixture host symbol 목록](./fixture-host-symbols.txt)은 positive control 결과다. [기본 빌드 로그](./default-build.log)와 [fixture build 로그](./fixture-build.log)를 보존한다.

이는 macOS static FFI archive의 feature boundary만 증명한다. Android APK나 iOS IPA 전체 bundle에서 모든 fixture code/payload가 제외되는지 증명하지 않는다. Simulator APK/executable hashes와 scene 실행 근거는 상위 [runtime evidence](../README.md)를 따른다.

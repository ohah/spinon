# C11.4 fit-content / auto fixture artifact 경계

2026-10-27 기준 macOS ARM64 `spinon-ffi` 정적 artifact 세 구성을 별도 `CARGO_TARGET_DIR`에서 빌드하고, `.rlib`와 `.a`의 payload 및 unmangled fixture export 이름을 확인했다.

| Cargo 구성 | 결과 | 근거 |
|---|---|---|
| 기본 `spinon-ffi` | scene ID, 고유 DOM prefix, C11.4 FFI export 모두 없음 | `default-build.log`, `default-artifact.sha256`, `default-symbol-string-scan.txt` |
| `c04-runtime-gpu` | scene ID, 고유 DOM prefix, C11.4 FFI export 모두 없음 | `core-build.log`, `core-artifacts.sha256`, `core-symbol-string-scan.txt` |
| `c04-runtime-gpu-fixture-test` | scene ID, 고유 DOM prefix, C11.4 FFI export 모두 있음 | `fixture-test-build.log`, `fixture-test-artifacts.sha256`, `fixture-test-symbol-string-scan.txt` |

재현 명령은 저장소 루트에서 다음 feature 조합을 각각 실행한다.

```sh
cargo build --locked -p spinon-ffi --lib
cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu
cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test
```

각 artifact에서 `fit-content-auto-neighbor-wpt`, `c114-fit-content-auto-neighbor-grid`, `spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture` 바이트를 검사했다. 세 입력은 모두 opt-in fixture-test artifact에서만 나타났다. 이 macOS 정적 archive 결과는 Android APK/iOS IPA 전체, release app bundle 또는 일반 CSS 지원 범위를 입증하지 않는다.

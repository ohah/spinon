# C11.4 `text-indent` flags static archive 경계

macOS ARM64, Rust 1.96.1에서 제품 feature와 fixture-test feature로 `spinon-ffi` release static archive를 각각 만들고 바로 검사했다. 두 빌드는 `target/release/libspinon_ffi.a` 경로를 공유한다. scanner는 고정 flags scene ID 20개와 C11.4 host FFI export 두 개를 확인한다. [scanner](./scan-artifact.mjs), [제품 build log](./default-release-build.log), [fixture-test build log](./fixture-release-build.log), JSON receipt([제품](./default-artifact.json), [fixture positive control](./fixture-artifact.json))를 함께 보관한다.

| Build | Cargo feature | 크기(bytes) | SHA-256 | flags scene ID | C11.4 host FFI export |
| --- | --- | ---: | --- | ---: | ---: |
| 제품 | `c04-runtime-gpu` | 75,283,624 | `1ac526d027717702b7d6c269b3565a43696a858d5221d40896e5ca48bf2ea821` | 0/20 | 0/2 |
| fixture positive control | `c04-runtime-gpu-fixture-test` | 92,019,656 | `9530144012da4b4326f27e9b4db5ea3bf99b974a796a22a2d4534291317946ea` | 20/20 | 2/2 |

Positive control의 export는 `_spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture`와 `_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`다. 재현 명령:

```sh
mise exec -- cargo build --locked --release -p spinon-ffi --features c04-runtime-gpu --lib
node spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/artifact-boundary/scan-artifact.mjs target/release/libspinon_ffi.a default
mise exec -- cargo build --locked --release -p spinon-ffi --features c04-runtime-gpu-fixture-test --lib
node spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/artifact-boundary/scan-artifact.mjs target/release/libspinon_ffi.a fixture
```

이 결과는 flags scene payload와 해당 fixture host export가 제품 feature static archive에는 없고 fixture-test positive control에는 있음을 보인다. Android APK/iOS app 전체 bundle이나 다른 내부 fixture symbol의 부재를 증명하지 않는다.

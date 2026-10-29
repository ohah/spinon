# C11.4 `visibility:collapse` static archive 경계

2026-10-29 현재 작업 트리에서 `spinon-ffi` static archive를 두 feature 조합으로 빌드해 `strings` 출력의 C11.4 visibility marker를 비교했다.

```sh
cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu
cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test
```

첫 결과를 `/tmp/spinon-c114-artifact-boundary/default.a`, 두 번째를 `fixture.a`로 복사해 확인했다.

| 조합 | archive SHA-256 | `c114-grid-visibility-collapse`, `collapse-visible-descendant-column`, C11.4 fixture FFI symbol |
| --- | --- | --- |
| 기본 `c04-runtime-gpu` | `10f6cb6d5024c2c8a29e1f0dd9c99ca7744a2afa466206adc9d7ad4a80b232f2` | 0 match |
| opt-in `c04-runtime-gpu-fixture-test` | `a52aeffe71a987c15d75f82d7ee2d2c97b2918841311d70ae5975c66b5e58ff9` | 10 matching strings |

Opt-in archive에는 고정 route와 `spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture` symbol이 존재한다. 이 결과는 Rust static archive에 한정하며 Android APK/iOS app 최종 산출물 전체의 fixture 경계나 크기를 주장하지 않는다.

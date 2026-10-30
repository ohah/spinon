# `spinon-ffi` release archive fixture 경계

같은 macOS ARM64/Rust 1.96.1 checkout에서 product와 fixture-test feature를 각각 release build하고 archive를 검사했다. `target/release/libspinon_ffi.a`는 두 빌드가 공유하므로 매번 build 직후 검사했다. 원본 [product build log](./default-build.log), [fixture-test build log](./fixture-build.log), [재현 scanner](./scan-artifact.mjs), [product receipt](./default-artifact.json), [fixture receipt](./fixture-artifact.json)를 둔다.

| 구성 | Cargo feature | Bytes | SHA-256 | 신규 scene marker | C11.4 host export |
| --- | --- | ---: | --- | ---: | ---: |
| Product | `c04-runtime-gpu` | 75,242,120 | `68aec6b378914d7a70640b20fcdd32bf41c606fca46ebd6acff1be1c40f4d2ec` | 0/2 | 0/2 |
| Fixture positive control | `c04-runtime-gpu-fixture-test` | 80,929,104 | `6cc04bf48e26b54b80f4ff28e44f8c1f8d597a6dc351f03ab1342a0b9a45271a` | 2/2 | 2/2 |

Scanner는 Rust sysroot의 같은 host triple용 `llvm-nm --defined-only --extern-only`와 archive bytes의 ASCII 문자열을 확인한다. Marker는 `anonymous-normal-newline-max-content`, `anonymous-pre-line-max-content`; host export는 `_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`, `_spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture`다.

제품 archive에는 변경 전부터 존재한 `RuntimeSession::new_runtime_gpu_block_flex_grid_track_sizing_fixture` 같은 내부 Rust symbol이 남을 수 있다. 이 대조는 신규 marker와 두 C host export의 feature 분리만 입증한다. 전체 C11.4 내부 symbol, Android APK, iOS app bundle 또는 release 앱에서 fixture가 완전히 빠진다고 확대 해석하지 않는다.

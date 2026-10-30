# FFI static archive fixture 경계

macOS ARM64, Rust 1.96.1에서 같은 checkout/source revision으로 `spinon-ffi` release static archive를 기본 product feature와 fixture-test opt-in feature로 각각 빌드했다. 두 archive는 검사가 끝난 뒤 저장하지 않았다.

| 조합 | 명령 | 크기 | SHA-256 | 새 scene ID | C11.4 host FFI exports |
| --- | --- | ---: | --- | ---: | --- |
| product | `mise exec -- cargo build --locked --release -p spinon-ffi --lib --features c04-runtime-gpu` | 75,242,088 bytes | `52106e681508612d929fb8d71d89ea8a04b21d3b05e7aa715bc6500044171d21` | 0/2 | 0/2 |
| fixture-test positive control | `mise exec -- cargo build --locked --release -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test` | 80,934,888 bytes | `a1b0f16a862feb8f8d99bfdaf0678aabf76c6fd87d940afc605f6ad0662571be` | 2/2 | 2/2 |

두 scene 문자열은 `anonymous-normal-min-content`, `anonymous-nowrap-min-content`다. FFI symbol scan은 Rust 1.96.1 toolchain의 `llvm-nm --defined-only --extern-only target/release/libspinon_ffi.a`로 `_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture` 및 `_spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture`를 찾았다. 문자열 scan은 `/usr/bin/strings -a target/release/libspinon_ffi.a`를 썼다. product archive에서는 두 scene marker와 두 export 모두 없었고 positive control에서는 모두 있었다.

제품 archive에는 기존 `RuntimeSession::new_runtime_gpu_block_flex_grid_track_sizing_fixture` Rust symbol이 보였다. 이 제한 구현 변경은 해당 기존 내부 경로를 수정하지 않았다. 이 검사는 새 fixture payload와 C host export의 feature 분리를 확인하며 전체 C11.4 내부 코드 격리, 최종 Android APK, iOS archive 또는 앱 번들 경계를 증명하지 않는다.

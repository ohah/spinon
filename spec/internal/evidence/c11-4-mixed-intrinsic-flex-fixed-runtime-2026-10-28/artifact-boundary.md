# 기본·opt-in fixture 아티팩트 경계

2026-10-28, PR #729 병합 이후 source `3e1add7414d534c0b5ca830a9c772ba65a3a3ace`에서 macOS ARM64 `spinon-ffi` release static library를 두 feature 구성으로 순차 빌드했다.

| Cargo 구성 | 명령 | `libspinon_ffi.a` SHA-256 | C11.4 fixture 검사 |
|---|---|---|---|
| 제품 runtime | `cargo build --locked -p spinon-ffi --lib --release --no-default-features --features c04-runtime-gpu` | `dd90e406139ae6d8ee9f1a2fe14ecb5d5fa74a081022d9e20e7ed8d85927e4a1` | C11.4 fixture FFI의 `new`·`eval` symbol 없음, generic fixture `runtime_source`·`prepare` Rust symbol 없음, `novel-mixed-intrinsic-flex-fixed` marker 0회 |
| fixture positive control | `cargo build --locked -p spinon-ffi --lib --release --no-default-features --features c04-runtime-gpu-fixture-test,c11-grid-metal-surface-pixels-test` | `00d62c2d11d8b8ceed9f537f0268a6134612b52716279c6fbde1b419fd968d54` | C11.4 fixture `new`·`eval` FFI symbol, generic fixture `runtime_source`·`prepare` Rust symbol, `novel-mixed-intrinsic-flex-fixed` marker 포함 |

검사는 `nm -gU target/release/libspinon_ffi.a`와 `strings target/release/libspinon_ffi.a`에서 `c11_4_grid_track_sizing_fixture`, `generic_intrinsic_flex_fixed`, `novel-mixed-intrinsic-flex-fixed`를 검색했다. Rust 앱 fixture 모듈은 `c04-runtime-gpu-fixture-test`에 gated되어 있으며, opt-in positive control에서만 위 symbol과 payload marker가 나타났다.

이 결과는 macOS ARM64 release **static FFI archive**의 이 fixture 경계에 한정한다. Android APK, iOS IPA, 전체 앱 bundle, 다른 fixture의 제외 여부는 검사하지 않았다.

## hidden-item follow-up

2026-10-28 구현 PR [#734](https://github.com/ohah/spinon/pull/734)의 hidden-item code를 macOS ARM64에서 두 feature 구성으로 순서대로 빌드했다. 작업 tree의 base commit은 `d73f71c950ae4ba64d27ebfd0c70f6e81475ec8c`이고, 빌드된 code changes는 PR head `61be0e3ec930398270f0929dadbc677f10d7a7f4`에 포함되어 merge commit `50e6286cf7b1c86a8c0398221751a42db105cb05`로 병합됐다. 아래 hash는 앞 절의 PR #729 archive와 별개인 이 hidden-item fixture 경계 결과다.

| Cargo 구성 | 명령 | `libspinon_ffi.a` SHA-256 | hidden fixture 경계 |
|---|---|---|---|
| 제품 runtime | `cargo build --locked -p spinon-ffi --lib --release --no-default-features --features c04-runtime-gpu` | `cb7bad54e87708a38d51d8e4a7fa86154258b3831fb0fef1effec3248f3c5400` | `spinon_runtime_gpu_host_new/eval_c11_4_grid_track_sizing_fixture` symbol 없음, `novel-mixed-intrinsic-flex-fixed` marker 없음 |
| fixture positive control | `cargo build --locked -p spinon-ffi --lib --release --no-default-features --features c04-runtime-gpu-fixture-test` | `222980b63bb717c2ea62faf5307cc2b2e5fc656f6e1389da054da6f391f8a50f` | `new`·`eval` C11.4 fixture FFI symbol 확인, marker 10회 포함 |

Symbol은 Rust 1.96.1 sysroot의 `llvm-nm -g target/release/libspinon_ffi.a`, payload는 `strings -a target/release/libspinon_ffi.a`로 검사했다. 이 정적 archive 결과는 Android APK, iOS IPA 또는 전체 앱 bundle의 fixture 제외 여부를 증명하지 않는다.

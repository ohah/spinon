# 기본·opt-in fixture 아티팩트 경계

2026-10-28, PR #729 병합 이후 source `3e1add7414d534c0b5ca830a9c772ba65a3a3ace`에서 macOS ARM64 `spinon-ffi` release static library를 두 feature 구성으로 순차 빌드했다.

| Cargo 구성 | 명령 | `libspinon_ffi.a` SHA-256 | C11.4 fixture 검사 |
|---|---|---|---|
| 제품 runtime | `cargo build --locked -p spinon-ffi --lib --release --no-default-features --features c04-runtime-gpu` | `dd90e406139ae6d8ee9f1a2fe14ecb5d5fa74a081022d9e20e7ed8d85927e4a1` | C11.4 fixture FFI의 `new`·`eval` symbol 없음, generic fixture `runtime_source`·`prepare` Rust symbol 없음, `novel-mixed-intrinsic-flex-fixed` marker 0회 |
| fixture positive control | `cargo build --locked -p spinon-ffi --lib --release --no-default-features --features c04-runtime-gpu-fixture-test,c11-grid-metal-surface-pixels-test` | `00d62c2d11d8b8ceed9f537f0268a6134612b52716279c6fbde1b419fd968d54` | C11.4 fixture `new`·`eval` FFI symbol, generic fixture `runtime_source`·`prepare` Rust symbol, `novel-mixed-intrinsic-flex-fixed` marker 포함 |

검사는 `nm -gU target/release/libspinon_ffi.a`와 `strings target/release/libspinon_ffi.a`에서 `c11_4_grid_track_sizing_fixture`, `generic_intrinsic_flex_fixed`, `novel-mixed-intrinsic-flex-fixed`를 검색했다. Rust 앱 fixture 모듈은 `c04-runtime-gpu-fixture-test`에 gated되어 있으며, opt-in positive control에서만 위 symbol과 payload marker가 나타났다.

이 결과는 macOS ARM64 release **static FFI archive**의 이 fixture 경계에 한정한다. Android APK, iOS IPA, 전체 앱 bundle, 다른 fixture의 제외 여부는 검사하지 않았다.

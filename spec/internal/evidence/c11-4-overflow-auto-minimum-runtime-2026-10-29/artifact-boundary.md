# C11.4 single-track overflow automatic minimum FFI archive 경계

같은 checkout에서 product `c04-runtime-gpu` archive와 `c04-runtime-gpu-fixture-test` positive-control archive를 순서대로 빌드했다. `target/debug/libspinon_ffi.a`에서 C11.4 single-track route marker와 SHA-256을 검사했다.

| build | route marker | SHA-256 | bytes |
|---|---|---|---:|
| `cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu` | `c114-auto-min-scrollable-overflow`, `c11-4-overflow-auto-minimum-10x10.png`, row/column scene IDs 없음 | `b16faa4bacfea39a40b8b6b14b1921d1e99f28495a3123be353bf56927ca758a` | 661,847,224 |
| `cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test` | route prefix, PNG URL, 14 scene IDs 있음 | `3cdb534e02f8bcaa3cfdcfb71584af8a575148d76a496af21d89e1d5b10d8c74` | 718,085,992 |

확인은 FFI static archive 내부 strings에 한정한다. Android APK, iOS app bundle 전체나 release artifact에서 다른 fixture가 배제됨을 증명하지 않는다.

# C11.4 overflow automatic minimum FFI artifact 경계

같은 최종 checkout에서 feature-off 기본 archive와 fixture-test positive-control archive를 순서대로 빌드했다. 검사는 `target/debug/libspinon_ffi.a`의 strings와 SHA-256을 대상으로 한다.

| build | archive | marker | SHA-256 | bytes |
|---|---|---|---|---:|
| `cargo build --locked -p spinon-ffi --lib` | feature-off | `c114-auto-minmax-span-overflow` 및 `columns-span2-visible-content-minimum` 없음 | `5327db6bd4632d61acfb0f6ac0b04d68fed4fdc143a595961f01279910a0ecbc` | 514,502,560 |
| `cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test` | fixture-test positive control | marker 있음 | `58f55bdbee16809a0a05e683019ce499444e4b66a5785337c3327a8a3dab87ce` | 715,495,824 |

이 검사는 FFI static archive의 기본·opt-in fixture 경계를 확인한다. Android APK나 iOS app bundle 전체에서 다른 feature의 fixture가 빠졌음을 뜻하지 않으며 release 산출물 검사도 아니다.

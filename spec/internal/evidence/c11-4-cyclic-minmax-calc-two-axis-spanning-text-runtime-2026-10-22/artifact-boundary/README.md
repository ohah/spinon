# Fixture static archive 경계

비교 대상은 commit `514a2ee73b807fdd2b19d57016adb976dae34d17`의 `spinon-ffi` macOS debug static archive다. 기본 feature set과 opt-in fixture-test feature set을 따로 빌드하고 `strings -a` 결과에서 scene ID·error·font registration marker를 검사했다.

| 빌드 | 명령 | 크기 | SHA-256 | 관찰 |
| --- | --- | ---: | --- | --- |
| 기본 | `mise exec -- cargo build --locked -p spinon-ffi --lib` | 511,480,400 bytes | `3fe95e39094f96fc35cebbc18a6ae417e3a2a1746f4d70c29afd49311b111215` | 네 scene ID, `unsupported_c114_two_axis_span_text_fixture`, `c114_two_axis_span_text_font_registration` 모두 없음 |
| fixture positive control | `mise exec -- cargo build --locked -p spinon-ffi --lib --features c11-grid-metal-surface-pixels-test` | 692,426,984 bytes | `8ec996d336c420f574cd8e95e85e2f40a78faa76070b7ee75a6bf089c89ad682` | 네 scene ID와 error/font marker가 포함됨 |

원본 build·scan 결과: [`default-build-post-commit.log`](./default-build-post-commit.log), [`default-scan-post-commit.log`](./default-scan-post-commit.log), [`fixture-build-post-commit.log`](./fixture-build-post-commit.log), [`fixture-scan-post-commit.log`](./fixture-scan-post-commit.log). 이 검사는 정적 archive만 비교하며 Android APK/iOS app 전체 또는 release artifact에서 fixture가 빠졌다는 증거로 확대하지 않는다.

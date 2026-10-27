# 기본/opt-in fixture artifact 경계

2026-10-27, checkout `feat/c11-4-grid-auto-repeat-intrinsic-wpt-20261027`에서 macOS ARM64 `spinon-ffi` library를 각 feature 구성으로 빌드하고 `.rlib` 심볼을 확인했다. 중간 산출물이나 Cargo cache를 삭제하지 않았다.

```sh
SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 mise exec -- cargo build --locked -p spinon-ffi --lib
SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 mise exec -- cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test
nm -C target/debug/deps/libspinon_ffi-0098a99528f2b0e6.rlib
nm -C target/debug/deps/libspinon_ffi-00c27cb82a2080a4.rlib
```

| Cargo 구성 | artifact | SHA-256 | `grid_auto_repeat_intrinsic_wpt::runtime_source` |
|---|---|---|---|
| 기본 | `libspinon_ffi-0098a99528f2b0e6.rlib` | `c5d87d92aba46e61bc52babb1c4b7cf5989a2c419d8b2b4879fb86245e354a10` | 없음 · `strings`의 `auto-repeat-intrinsic-fill-fixed-max` marker 0회 |
| `c04-runtime-gpu-fixture-test` | `libspinon_ffi-00c27cb82a2080a4.rlib` | `aa2f8f83a5bf9ad99c93353ec4f91a9205bd6f2edb89a758826d9da00d305475` | 있음 · marker 2회 |

`nm -C`에서 feature module의 `runtime_source`·`scene_source` 심볼은 opt-in archive에만 나타났다. `strings`에서도 대표 scene marker가 opt-in archive에만 나타났다. 이것은 Rust archive feature isolation에 대한 양성·음성 대조다. 최종 앱 패키지의 모든 fixture 문자열·payload 제외, release archive 크기 또는 APK/IPA 검사는 확인하지 않았다.

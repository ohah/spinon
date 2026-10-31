# C11.4 computed `word-spacing` fixture archive 분리

## 실행

같은 checkout과 release profile에서 Rust FFI static archive를 각각 빌드했다.

```sh
cargo build --locked --release -p spinon-ffi --no-default-features --features c04-runtime-gpu
cargo build --locked --release -p spinon-ffi --no-default-features --features c04-runtime-gpu-fixture-test
```

검사는 `target/release/libspinon_ffi.a`에서 신규 FFI/runtime symbol 정의와 24개 고정 scene ID payload를 찾는다. scene payload는 fixture expectation JSON에 있는 ID 각각을 archive bytes에서 검사한다.

## 결과

| 빌드 | archive SHA-256 | 크기 | 신규 candidate symbol 정의 | 24 scene payload |
|---|---|---:|---:|---:|
| `c04-runtime-gpu` 제품 runtime | `3ec313a807897e618a63dc3c53b7dfc3ae2663714fd48ccf49a0007bd5d8de38` | 75,265,408 bytes | 0 | 0/24 |
| `c04-runtime-gpu-fixture-test` positive control | `8f994cb6ba4e7a7254551b74bfe3198d8a505b792e9bb0bec378d503f14cb9e8` | 81,287,616 bytes | 2 | 24/24 |

Positive-control symbol 정의는 `computed_word_spacing::runtime_source` 및 `validate_computed_word_spacing`이다. 기본 archive에는 둘 다 없고 각 고정 scene ID도 없다. 이는 macOS ARM64 FFI static archive에 대한 검사다. Android/iOS app bundle이나 release package 전체에서 fixture payload가 빠진다는 뜻은 아니다.

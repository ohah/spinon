# C18.4 기본·opt-in artifact 경계

2026-10-16에 같은 worktree에서 기본 산출물과 C18.4 전용 opt-in 산출물을 순서대로 빌드하고 hash·symbol·DEX class·fixture 문자열·원본 PNG bytes를 비교했다. 앱 바이너리는 저장하지 않고 SHA-256과 검색 결과만 보존한다.

## Cargo staticlib

| 빌드 | SHA-256 | 확인 |
| --- | --- | --- |
| `cargo build --locked -p spinon-ffi --lib` | `6c3d3e57683e65252213c40dca55c8e0d1323a03a82c91ddc3ffc06f4f4f5341` | C18.4 FFI symbol·`spinon-fixture:c18-4/` 문자열 없음 |
| `cargo build --locked -p spinon-ffi --lib --features c18-4-image-fixture-test` | `156735b28a92d407491907599a0d78ad3c521c286d5808ba43f70ac18c5dc083` | 생성·eval·surface-sample C ABI symbol 3개와 fixture 문자열 존재 |

symbol inspection은 Rust 1.96/LLVM 22 object attribute를 읽지 못하는 Apple `nm` 대신 LLVM 24 `llvm-nm`으로 수행했다. 원본 encoded PNG payload는 Cargo staticlib 비교의 주 판정이 아니며, Android·iOS app executable에서 직접 byte scan했다.

## Android APK

| 빌드 | APK SHA-256 | 확인 |
| --- | --- | --- |
| C04=0, C18.4=0 `:app:assembleDebug` | `4b0783c71ac2200cd4ac7d693fb457b6371ac3727120f8c8d584c89c80ff1e65` | `lib/arm64-v8a/libspinon_bootstrap.so` SHA-256 `d078610414dd795826fb7ca96fffb8247c60648f47975b5c2a811e7c74a09d0a`; C18.4 fixture 문자열·PNG 4개 없음; DEX에 `AsyncImageFixtureCaptureAdapter` 및 `AsyncImageFixtureNativeAdapter` class definition 없음 |
| C04=1, C18.4=1, C18.1–3=0 `:app:assembleDebug` | `59103dbab2c653d738356406adc79e9ff24a0358b3ec0a6a60597326efc13092` | `lib/arm64-v8a/libspinon_bootstrap.so` SHA-256 `743d1e3e265d55f460be431fe4a327fd491f54d07bb1e25fffd3d04e5d9598f6`; DEX에 두 opt-in adapter class definition, native library에 C18.4 fixture 문자열과 PNG 4개 전부 존재 |

기본 DEX에는 생성된 `BuildConfig.SPINON_C18_4_ASYNC_IMAGE_FIXTURE_TEST` field 이름이 남지만 값은 `false`다. 이 compile switch 이름을 fixture implementation이나 payload 포함 증거로 세지 않았다. Android APK 단계의 native library는 stripped라 exported C ABI를 직접 열거할 symbol table이 없다. C ABI symbol의 양·음성 대조는 위 unstripped Cargo staticlib에서 확인했다. extracted opt-in native library SHA-256은 `743d1e3e265d55f460be431fe4a327fd491f54d07bb1e25fffd3d04e5d9598f6`다.

## iOS Simulator app executable

| 빌드 | `SpinonBootstrap` SHA-256 | 확인 |
| --- | --- | --- |
| C04=0, C18.4=0 `build:ios-sim` | `54c07021aca2c65fb74bda6c4d19b0feefd6d012cc6281be3fe0dc0ba85026b9` | C18.4 FFI symbol·fixture 문자열·PNG 4개 없음 |
| C04=1, C18.4=1, C18.1–3=0 `build:ios-sim` | `0e5ccad6f2915481b75a477222bdce30b56e79c37565f4d4a5a1636a01421973` | 생성·eval·surface-sample C ABI symbol 3개, fixture 문자열, PNG 4개 전부 존재 |

## 원본 검사 결과

- [`cargo-default.sha256`](cargo-default.sha256), [`cargo-opt-in.sha256`](cargo-opt-in.sha256), symbol/문자열 목록은 이 폴더의 `cargo-*.txt`다. LLVM 문자열 추출의 원본 줄 끝 공백은 `cargo-opt-in-c18-4-strings.txt.raw.gz`에 보존했다.
- [`android-default.sha256`](android-default.sha256), [`android-opt-in.sha256`](android-opt-in.sha256)는 APK hash를, `android-png-payload-check.txt`는 네 PNG 각각의 default `False`·opt-in `True`를 기록한다.
- `android-optin-c18-4-symbols.txt`는 stripped APK `.so`에서 가져온 symbol 목록이 아니라 Cargo staticlib symbol과 구분되는 runtime marker 목록이다.
- [`ios-default.sha256`](ios-default.sha256), [`ios-opt-in.sha256`](ios-opt-in.sha256), `ios-png-payload-check.txt` 및 `ios-*-c18-4-{symbols,strings}.txt`는 Simulator executable 직접 검사를 담는다.

Android와 iOS compile switch·fixture class/module이 기본 산출물에 없다는 결과는 일반 제품 URL loader나 제품 image 지원 완료를 뜻하지 않는다.

# C18.3 기본·opt-in artifact 경계

2026-10-16에 같은 checkout으로 fixture 기본값과 C18.3 opt-in을 각각 빌드했다. Android는 ARM64 Cargo release staticlib, APK native library 및 APK DEX를 검사했고, iOS는 iOS Simulator 앱 executable을 검사했다. 기본 빌드는 `SPINON_ENABLE_C04_RUNTIME_GPU=0`, C18.1/2/3 fixture flag `0`; opt-in은 C04 runtime GPU와 C18.3 fixture flag `1`이다.

## 바이너리 검사

검사 항목은 C18.3 FFI 심볼, fixture JavaScript 식별자(`c18-object-fit-root` / `c183Node`), 원본 PNG byte 전체, Android DEX adapter 및 route 문자열이다. 원본 PNG는 24×12 RGBA8, SHA-256 `cc621c7a4be9345a8124b91ebcd5a6ff9bc3a3206ec59fe1228b24ba09cae454`다.

| 산출물 | 기본: FFI / JS / PNG | opt-in: FFI / JS / PNG |
| --- | --- | --- |
| Android `libspinon_ffi.a` | 없음 / 없음 / 없음 | 있음 / 있음 / 있음 |
| Android APK의 `libspinon_bootstrap.so` | 없음 / 없음 / 없음 | 있음 / 있음 / 있음 |
| iOS Simulator `SpinonBootstrap` executable | 없음 / 없음 / 없음 | 있음 / 있음 / 있음 |
| Android APK의 DEX adapter 및 C18.3 route | 없음 / 없음 | 있음 / 있음 |

모든 opt-in positive control에서 C18.3 FFI·JS marker와 exact PNG bytes를 찾았다. 기본 control에서는 세 항목이 모두 없었다. Android APK의 `classes*.dex`에서도 default에는 `ObjectFitPositionFixtureCaptureAdapter`와 `spinon_c183_object_fit_position`이 없고 opt-in에는 둘 다 있다.

## SHA-256

| 산출물 | SHA-256 |
| --- | --- |
| Android default Cargo staticlib | `3f5de3a9a5b6635760dd4a26da745488647f77b80227d00adc292a3a95def3d5` |
| Android opt-in Cargo staticlib | `4e2541974750bea1e170710c670c32e2efe75ecb7f8ac4285d61b8ce3d5d6824` |
| Android default APK | `75fe31b6a55cadfebc7d80e7fbdf4d51f697ef06813eaa6d931533cd154dbd6f` |
| Android default native library | `449b0851029b9489776fb13b509eb7b185100a72a82eee381924df4480d062c4` |
| Android opt-in APK | `c3911985eb0986ef02c096bc3bafec96b750a796832850f75e7cce8a58c34853` |
| Android opt-in native library | `e4dfab0c80ed07a166a442ef3cc078d216b9990efecbb324896873d82e52f94c` |
| iOS default Simulator executable | `2d55d47794aefc7ce544fd72f0bc686b6da6c3349f3a89175a41d454e3d00ca7` |
| iOS opt-in Simulator executable | `b753bb44ff2adcd3d35234f986a4d56337689c861f14324936ea712e4bdc0e72` |

정적 library와 앱 executable 검사 결과를 최종 APK/앱 전체 크기 또는 release artifact 보증으로 확대 해석하지 않는다. 이 대조는 debug Android APK와 iOS Simulator debug app에 한정한다.

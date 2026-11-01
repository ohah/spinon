# C11.4 anonymous Grid text `text-transform:capitalize` runtime evidence

## 범위

이 근거는 내부 계약 0059의 anonymous direct-text Grid fixture-test 경계에서 고정 `lang="en"`, ASCII letters와 U+0020만 허용하는 제한 profile을 확인한다. 각 U+0020을 word boundary로 삼고 segment의 첫 lowercase ASCII letter만 uppercase로 바꾸며 원문 Text와 UTF-16 source range는 보존한다. punctuation, digits, non-ASCII, tab/newline, inline run, 상속 언어와 일반 CSS author admission은 거부한다. C11.4 또는 C11 전체 완료 근거가 아니다.

후보 source commit은 `9b124aeab080711028f1198686b8818f26aed122`다. 모든 최종 앱 캡처의 `sourceHead`가 이 SHA와 일치한다. Android APK 및 iOS executable 설치본 hash도 각각 빌드 산출물과 일치한다.

## 기준 및 판정

Chrome 156 Agent Browser의 고정 Noto Sans KR 10px/400 oracle, 독립 glyph advance 합산, fixture와 허용 오차는 [사전 비교](../c11-4-anonymous-grid-text-transform-capitalize-precomparison-2026-11-01/README.md)에 있다. six-scene matrix는 unchanged source, transformed layout text, computed mode, intrinsic width, final Grid column/row, line count와 source trace를 확인한다. 1/64 CSS px layout rounding에 대한 허용 오차는 0.03125 CSS px다.

| 장면 | Chrome 기준 column | AVD column | iOS column | 행 높이 |
| --- | ---: | ---: | ---: | ---: |
| 원문 `iiii ii`, min-content | 11 | 11 | 11 | 20 |
| 원문 `iiii ii`, max-content | 18.71875 | 18.74 | 18.74 | 10 |
| `capitalize`, min-content (`Iiii Ii`) | 11.1875 | 11.18 | 11.18 | 20 |
| `capitalize`, max-content (`Iiii Ii`) | 19.09375 | 19.10 | 19.10 | 10 |
| mixed case `iIII ii` → `IIII Ii` | 19.65625 | 19.64 | 19.64 | 10 |
| repeated spaces `ii   ii` → `Ii Ii` | 13.59375 | 13.60 | 13.60 | 10 |

Android API 37 ARM64 AVD `emulator-5556`의 `android-avd-pass02`와 iPhone 17 Pro / iOS 26.2 Simulator `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`의 `ios-simulator-pass04`가 각각 6/6 통과했다. 상세 실행 matrix, 장면별 raw log, simulator receipt와 representative screen은 각 폴더에 있다. 대표 화면은 `layout=ready` 앱 상태를 보여 주지만 glyph paint가 비활성화되어 글자 픽셀이나 화면 표면의 glyph 결과를 증명하지 않는다.

## iOS 실패 실행 분리

최초 iOS 실행 `ios-simulator-pass01`은 source commit 전 `status=-12`를 반환했다. commit-bound 실행 `ios-simulator-pass03`도 첫 장면에서 `status=-12`를 반환했고, failure detail은 새 화면 요청이 와서 이전 장면을 거부했다고 기록한다. 이 실패 log와 각 source/artifact receipt를 보존한다. 이후 같은 commit의 fresh-app `ios-simulator-pass04`는 6/6 통과했으나, 그 성공은 앞선 실패를 수정한 것으로 세지 않는다. 추가 시작 대조는 기존 `text-transform-none-min-content` 5/5, 새 `text-transform-capitalize-none-min-content` 5/5, 새 `text-transform-capitalize-min-content` 5/5 summary를 통과했다. 이 15회에는 `status=-12`가 없었지만 원인을 특정하거나 앞선 실패를 무효로 하지 않으며 iOS 실패 cohort를 계속 따로 집계한다. [시작 경합 대조 기록](./ios-startup-race-diagnostic-2026-11-01/README.md)를 참고한다. run IDs와 raw logs는 각 `ios-startup-race-diagnostic-2026-11-01/` 파일에 기록했다.

## 빌드와 경계

- Android fixture 앱: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-android-app.sh` — `BUILD SUCCESSFUL`.
- iOS Simulator fixture 앱: 같은 두 환경 변수를 지정해 `tools/build-ios-sim.sh` 실행 — `BUILD SUCCEEDED`.
- Android build는 compile SDK 37.2와 Android Gradle Plugin 호환성 권고 및 고정 NDK 환경 변수 불일치 안내를 출력했고 SDK NDK 27.1.12297006을 사용했다. iOS build는 V8 archive의 duplicate debug-map 경고를 출력했으나 성공했다.
- macOS ARM64 `spinon-ffi` default static archive raw byte scan: candidate scene marker 6종 각각 0, C11.4 fixture entrypoint marker 0; SHA-256 `882ed5b1fa1eaf72763121373295014151793312f4a159dc9c77bd8efb7b8a3b`.
- `c04-runtime-gpu-fixture-test` opt-in archive raw byte scan: candidate scene marker 6종 각각 4, C11.4 fixture entrypoint marker 10; SHA-256 `f0a989baf3a99bd854555931dfc17ea25a319541e47f596b9a53cf7aaff0363f`. 이 검사는 static archive 문자열 경계만 확인하며 최종 앱 번들 전체의 제외 여부를 뜻하지 않는다.
- `cargo fmt --all -- --check`, focused text-layout/runtime/FFI tests, JS reference tests 및 fixture-module boundary test는 이 commit을 만든 source tree에서 통과했다. Runtime Clippy는 통과했지만 다른 기존 `anonymous_text_hyphens.rs`의 `too_many_arguments` 경고 1건이 남아 `-D warnings` gate는 통과하지 않는다. FFI fixture Clippy는 경고 없이 통과했다. `swift-format`은 설치되지 않아 실행하지 못했다.

## 재현 명령

```sh
SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-android-app.sh
SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-ios-sim.sh
node tools/css-reference/capture-c11-4-computed-text-transform-simulators.mjs android-avd emulator-5556 spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-precomparison-2026-11-01/expected.json spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-runtime-2026-11-01/android-avd-pass02
node tools/css-reference/capture-c11-4-computed-text-transform-simulators.mjs ios-simulator 9DCCE836-7659-4914-B6C1-13F54BFFA9DB spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-precomparison-2026-11-01/expected.json spec/internal/evidence/c11-4-anonymous-grid-text-transform-capitalize-runtime-2026-11-01/ios-simulator-pass04
```

`SHA256SUMS.txt` records the final app binaries, Chrome inputs, commit-bound captures, retained iOS failures, and prior exploratory receipts. C11.4/C11 remain unchecked and the official completion count remains `72/213`.

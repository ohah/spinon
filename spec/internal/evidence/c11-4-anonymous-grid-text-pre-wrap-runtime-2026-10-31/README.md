# C11.4 anonymous Grid text `white-space:pre-wrap` runtime 후보

## 판정

최종 후보 소스 snapshot `ac0fe3995bee3c7780f28ca0252c23c7ee745a2ccb18460a321ef25f598922e1`의 opt-in fixture 앱에서 고정 16-scene 행렬을 실행했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator가 각각 16/16 장면을 통과했다. 신규 범위 장면은 두 플랫폼에서 기대 root/item frame과 Columns/Rows track을 만들었고, capture runner가 draw/present 및 superseded retry 0을 확인했다. 앱 해시도 각 설치본과 빌드 산출물이 일치한다.

이 결과는 구현 후보의 제한된 runtime 검증이다. C11.4/C11 완료, 일반 CSS 지원 또는 공식 `72/213` 집계 변경을 뜻하지 않는다.

## 신규 장면 결과

| 장면 | Columns | Rows | 추가 확인 |
| --- | --- | --- | --- |
| `anonymous-normal-max-content-spaces` | `[50]` | `[10]` | Normal의 반복 공백 collapse 대조 |
| `anonymous-pre-wrap-max-content-spaces` | `[60]` | `[10]` | 반복 공백 보존 |
| `anonymous-pre-wrap-comment-split-max-content` | `[60]` | `[10]` | HTML comment split oracle에 대응하는 두 direct Text source가 anonymous item 하나로 이어짐; 앱 입력에는 Comment node 자체가 없음 |
| `anonymous-pre-wrap-min-content-spaces` | `[20]` | `[20]` | 보존 공백 사이에서 두 줄로 wrap |
| `anonymous-pre-wrap-soft-wrap` | `[25]` | `[20]` | 25px track에서 20px 두 줄; 첫 줄 끝 보존 공백의 hanging advance는 10px이며 track contribution에서 제외 |
| `anonymous-pre-wrap-forced-break` | `[20]` | `[20]` | LF forced break |
| `anonymous-pre-wrap-whitespace-before-block` | `[20,20]` | `[10]` | block marker frame `[0,0,10,10]`; 공백 텍스트는 별도 item/cell을 만들지 않음 |

나머지 9개 기존 장면도 같은 최종 실행에서 재검증했다. 모든 장면에서 runner가 layout-ready 및 draw/present를 확인했고 재시도는 0이었다. 원본 전체 JSON과 scene별 로그·PNG는 각 실행 폴더에 보존했다.

## 실행 환경과 원본

- Android: `emulator-5554`, API 37, ARM64 AVD. 설치 APK와 빌드 APK SHA-256은 모두 `cbe11e1febfc1244d7a7821fddfb5d8d924ef8fbedbca9c4486ea34e9dd14274`다. [최종 16-scene 실행](android-avd-c114-anonymous-grid-text-android-avd-20261030T182836149Z-c7a75e1a/), [capture log](android-capture.log), [빌드 log](android-build.log).
- iOS: iPhone 17 Pro, iOS 26.2 Simulator, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`. 설치 앱과 빌드 앱 SHA-256은 모두 `18b661c42bf44ebd6c9725bf28ea954ed359944a349f3100c656eb4ff251e67b`다. [최종 16-scene 실행](ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T182908683Z-2a6cafcf/), [capture log](ios-capture.log), [빌드 log](ios-build.log).
- 두 실행은 commit `47ffeb1216a1085cb40c505f312108dea66c600c`, tree `48c14b578cd2883ec706f0eac84855b4024eedbc`를 기준으로 했다. tracked diff SHA-256은 `41dc52158c8742c8a2612bdb117a17d6eba3a53a6d7a7130a56f950c448396f8`이다. 정확한 tracked/untracked 입력 목록, 각 플랫폼 환경과 snapshot SHA-256은 두 `simulator.json`에 기록되어 있다.
- Android는 AVD, iOS는 Simulator 결과다. 실기기, Android hardware GPU, iOS compositor 이후/scanout 픽셀, glyph ink pixel 정합성을 주장하지 않는다. PNG는 앱 fixture 화면 확인용이며 text glyph paint의 정확한 pixel oracle이 아니다.
- tab expansion은 정확히 `XX\tXX`인 Rust unit-test profile에서만 검사했다. 다른 tab source나 inline ancestor는 fail-closed한다. 앱 16-scene matrix에는 tab 입력이 없으며 runtime `tab-size` projection이나 일반 authored `tab-size` 지원을 주장하지 않는다.

## 코드·검증 기록

- `PreWrap` normalization과 source provenance, min-/max-content, LF/CRLF, soft-wrap hanging advance는 `spinon-text-layout` 및 `spinon-layout` 테스트에서 확인했다.
- opt-in runtime 전체 suite에서 기존 empty/non-text explicit Grid가 ratio/text adapter로 잘못 들어가는 회귀가 드러났다. adapter를 보강하고 empty explicit Grid가 `None`으로 빠지는 회귀 테스트를 추가했다. 수정 뒤 `spinon-runtime --features c11-4-grid-track-sizing-fixture-test --lib`은 327 passed, 0 failed, 3 ignored였다.
- `cargo fmt --all -- --check`, 변경 crate와 opt-in feature의 `Clippy --all-targets -- -D warnings`, `spinon-ffi` fixture feature tests 및 CSS fixture module boundary Node tests는 통과했다. 전체 workspace `cargo test --workspace`는 변경 밖 `spinon-style-to-layout`의 `c07_3_aspect_ratio_tests::comparison::aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs`에서 실패했다. 기대 문자열은 `1.77778 / 1`, 실제는 `1.7777778 / 1`이었다. 이 실패를 전체 workspace 통과로 기록하지 않는다.
- 제품/fixture static archive positive-control 비교는 [artifact boundary 기록](artifact-boundary/README.md)에서 다룬다. 정적 archive 검사 결과를 APK/IPA 제외 검증으로 확대하지 않는다.

기준 관찰과 독립 계산은 [PreWrap 사전 비교](../c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31/README.md), 범위는 [계획](../../../../plan/c11-4-anonymous-grid-text-pre-wrap.md)에 있다. 이번 실행은 opt-in C11.4 fixture route의 anonymous text track sizing만 확인한다. 일반 author CSS cascade, arbitrary HTML/root-HTML parity, WPT, 임의 font/script, `white-space:pre`, `break-spaces`, 실기기·hardware renderer 및 전체 C11.4는 범위 밖이다.

계획 PR #869와 구현 PR [#870](https://github.com/ohah/spinon/pull/870)은 병합됐다. 구현/runtime 실패 관점 20개, source 후보 diff 30관점 검토, product/fixture static archive 경계와 최종 simulator/build 검증은 끝났다. PR #870 최종 head `bf817aff25c5d39dcec45a744102cac9a0d9d0d7`의 변경 patch가 로컬 검토본과 일치했다. PR checks 결과는 보고되지 않았고 main required checks도 설정되지 않아 CI 통과로 기록하지 않는다. 공식 C11 완료 수 `72/213` 및 C11.4 checkbox는 유지한다.

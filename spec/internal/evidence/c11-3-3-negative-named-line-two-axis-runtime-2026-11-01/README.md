# C11.3.3 양 축 negative named-line pair · unmerged candidate evidence

이 candidate는 PR #967의 one-axis 제한 mode를 넓히지 않고 새 two-axis typed placement mode를 추가한다. 양 축에 같은 custom-ident의 negative named-line pair가 동시에 leading implicit tracks를 만들 때만 동작한다. C11.3.3/C11 공식 완료 표시는 변경하지 않는다.

## 결과

- Chrome 156 Agent Browser 및 CSS Grid 2025 CRD 기반 8-scene precomparison: [입력·독립 기대값·oracle](../c11-3-3-negative-named-line-two-axis-precomparison-2026-11-01/README.md)
- 양 축 auto 기준: Columns `[6,6,6,0,20]px`, Rows `[4,4,4,0,15]px`, item `(0,0,18,12)`; fixed patterns, equal/reversed endpoints, 각 축 reversal도 Chrome와 일치
- 수정된 최종 candidate `0954c9cef855237510563a7ad3585e190249ee49`의 Android API 37 ARM64 AVD/SwiftShader 재실행: [실행](./android-avd-c1133-negative-pair-two-axis-android-avd-20261101T055432072Z-227c6067/README.md), 13 element frames, surface sample 6/6 exact, 최대 채널 오차 0, 잘못된 좌표는 readback 전 거부, 미등록 scene은 draw 전 거부
- 같은 수정 소스의 iPhone 17 Pro/iOS 26.2 Simulator 재실행: [실행](./ios-simulator-c1133-negative-pair-two-axis-ios-simulator-20261101T055554422Z-f9734042/README.md), 13 element frames, Metal drawable sample 6/6 exact, 최대 채널 오차 0, 잘못된 좌표는 readback 전 거부. OS compositor 이후 픽셀은 확인하지 않음
- 첫 PR 검토에서 auto-size item이 generic signed mode로 빠지던 수용 경로를 재현했다. 두 축 pair shape면 item 크기와 무관하게 전용 mode를 선택하고 layout에서 `UnsupportedGridItemSize`로 닫도록 고쳤다. 수정본 focused style 회귀와 양 플랫폼 runtime을 재실행했다.
- 최종 소스 revision과 Android/iOS app SHA-256 연결: [source-revision.json](./source-revision.json)
- 기존 one-axis negative pair regression: [Android](../c11-3-3-negative-named-line-pair-runtime-2026-11-01/android-avd-c1133-negative-pair-android-avd-20261101T054233146Z-70fec10c/README.md) 및 [iOS](../c11-3-3-negative-named-line-pair-runtime-2026-11-01/ios-simulator-c1133-negative-pair-ios-simulator-20261101T054243443Z-904c5203/README.md), 각각 8/8 exact
- 기존 C11.3.3 regression: [Android 39 scene·126 node](../c11-3-3-grid-template-areas-runtime-2026-11-01/android-avd-c1133-matrix-20261101T053617328Z-9647e9fd/README.md) 및 [iOS 39 scene·126 node](../c11-3-3-grid-template-areas-runtime-2026-11-01/ios-simulator-c1133-ios-matrix-20261101T053737770Z-36a56a45/README.md), geometry 오차 0 CSS px

## 실패 경계와 빌드 격리

0개/2개 participating item, 고정되지 않은 item 크기, 양 축 중 한 축만 leading pair인 입력, mixed/definite line, template areas, auto 외 unsupported track kind, non-finite breadth 및 10,001 tracks를 닫는 layout tests가 있다. Android/iOS strict scene route와 surface coordinate allowlist를 확인했다. [구현/runtime adversarial review 20개](./implementation-runtime-review-20.md)에 각 항목과 근거를 기록했다.

`artifact-boundary.json`은 기본 macOS `spinon-ffi` archive에서 세 candidate marker가 없고 `c11-grid-metal-surface-pixels-test` positive control archive에는 모두 존재함을 기록한다. 이는 static archive의 feature 경계다. APK/IPA 전체 bundle 제외를 증명하지 않는다.

검증 명령 결과: `spinon-layout` 330/330, focused style projection 및 FFI tests 통과, CSS reference 19/19, fixture module boundary 30/30, scoped Clippy 및 rustfmt 통과. 새 양 축 shape boundary test와 auto-size rejection 회귀도 focused로 통과했다. 전체 `spinon-style-to-layout`는 276 passed·1 failed이며, 실패는 C07.3 `aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs` float string assertion에서 실제 `1.7777778`, 기대 `1.77778`이다. 이를 candidate 통과로 계산하지 않는다. PR #970 diff 30개 독립 관점 검토를 마쳤다. GitHub check rollup은 비어 있어 hosted CI 성공으로 기록하지 않는다.

Android AVD는 SwiftShader software renderer다. iOS sample은 compositor 이전 Metal drawable이다. 실기기/hardware GPU, Spinon WPT, 일반 CSS admission, 전체 C11.3.3/C11 지원은 범위 밖이다. PR #970은 아직 merge 전이다.

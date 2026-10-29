# C11.3.3 unknown-area implicit `auto` track 구현/runtime 검토 · 20개

**검토 범위:** 기존 unknown-area fixture의 `TemplateGridImplicitTracks` 기본 auto 처리, Android/iOS fixture route, v2 runtime matrix. 계획 검토 20개 및 PR diff 검토와 별도로 진행했다.

**결과:** PASS 20 · PARTIAL 0 · FAIL 0.

| # | 실패 가능성 | 결과 |
|---:|---|---|
| 1 | template area가 없는 implicit grid에서도 기본 auto track을 허용함 | PASS — `implicit_auto_tracks_still_require_template_areas_and_signed_mode_rejects_them` layout test가 no-area 거부를 유지한다. |
| 2 | signed/negative leading extent가 새 auto 경로에 잘못 들어옴 | PASS — 같은 layout test가 signed mode의 기본 auto 거부를 확인한다. |
| 3 | 행만 확장하고 열은 기존 거부 경계에 남김 | PASS — 양축 unknown-area geometry가 두 플랫폼에서 Chrome과 일치한다. |
| 4 | 열만 확장하고 행은 미지원으로 남김 | PASS — 동일 양축 scene과 전용 layout input으로 확인했다. |
| 5 | unknown bare ident가 선언된 area 또는 area line alias보다 우선함 | PASS — C11.3.3 style test 19/19이며 기존 area/named-line precedence 사례가 통과한다. |
| 6 | area 기반 auto sizing이 일반 repeat/named-line profile로 새어 나감 | PASS — mode 선택은 `template_areas.is_some()`로 제한되고 no-area/signed control은 거부된다. |
| 7 | 기본 auto track sizing 대신 임의 item 크기를 만들어냄 | PASS — `GridTrackSize::Auto`가 Taffy auto track sizing에 전달되고 빈 scene의 item은 oracle에 따라 0 크기다. |
| 8 | unknown-area root의 definite size가 변함 | PASS — Android/iOS에서 root `(0,0,160,120)`을 최대 오차 0 CSS px로 비교했다. |
| 9 | unknown item이 잘못된 trailing line 또는 축에 배치됨 | PASS — 두 플랫폼에서 item `(160,120,0,0)`이 정확히 일치했다. |
| 10 | contribution과 관계없이 모든 implicit auto track을 0으로 축소함 | PASS — 0 대입을 추가하지 않았고 기존 C11.4 implicit auto item sizing 회귀가 13/13 통과했다. |
| 11 | 기존 C11.4 text contribution 동작이 회귀함 | PASS — auto sizing 및 feature-gated mixed text/image span 회귀가 각각 13/13, 3/3 통과했다. |
| 12 | 기존 C11.4 image contribution 경계가 회귀함 | PASS — feature-gated mixed text/image test에서 replaced-image 배치와 contribution geometry를 확인했다. |
| 13 | fixed-px auto-track list가 임의 intrinsic list를 허용함 | PASS — `TemplateAreaImplicit`는 `LengthPx` 고정 list guard를 유지하고 관련 회귀가 통과했다. |
| 14 | area-created track-list cycle 순서가 변함 | PASS — 6개 cycle scene이 37-scene matrix에 남고 Android/iOS 표면 표본 28/28이 각각 exact다. |
| 15 | 잘못된 template-area 선언이 cascade fallback 대신 성공함 | PASS — 두 simulator matrix에서 5개 fallback scene을 모두 cascade fallback으로 보고했다. |
| 16 | 과거 거부 기록을 덮어써 이전 근거를 잃음 | PASS — runtime matrix v1은 byte-identical하게 보존하고 v2가 별도 성공을 기록한다. |
| 17 | Android 기본 scene probe가 성공을 실패로 판정함 | PASS — 중복 obsolete rejection probe를 제거했고 전체 AVD matrix 37/37이 통과했다. |
| 18 | iOS 기본 scene probe가 성공을 실패로 판정함 | PASS — 동일한 Swift probe를 제거했고 전체 iOS matrix 37/37이 통과했다. |
| 19 | geometry 성공으로 surface pixel 불일치 또는 readback 실패를 숨김 | PASS — 기본 sample, cycle sample, 기존 fixed-track overflow control을 별도로 확인했고 기록된 표본은 exact다. |
| 20 | simulator evidence를 hardware, compositor 또는 WPT 증거로 과장함 | PASS — API 37 SwiftShader, compositor 전 iOS Metal drawable, Spinon WPT 미검증을 명시했다. |

## 근거

- [Android API 37 AVD 결과](../c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T201752342Z-ba38b087/result.json).
- [iOS 26.2 Simulator 결과](../c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T202005378Z-3b0cf3b6/result.json).
- [Rust, Node, matrix 및 archive 경계 요약](./README.md).

# C11.3.3 unknown-area implicit `auto` track 구현 PR diff 검토 · 30개

**검토 대상:** 구현 diff, source-bound runtime artifact, test, 내부 계약, status ledger, 공개 roadmap 기록. 구현/runtime 검토와 별도로 진행했다.

**결과:** PASS 30 · PARTIAL 0 · FAIL 0.

| # | diff 실패 가능성 | 결과 |
|---:|---|---|
| 1 | `TemplateGridImplicitTracks`의 모든 사용처로 변경 범위를 넓힘 | PASS — 기본 auto는 기존 template-area metadata가 있을 때만 허용한다. |
| 2 | signed leading implicit mode까지 잘못 확장함 | PASS — signed mode는 fixed-only 거부 arm에 남아 있다. |
| 3 | area 없는 no-name 경로가 허용적으로 변함 | PASS — 전용 negative layout control이 거부를 유지한다. |
| 4 | 행/열 검증에서 서로 다른 area guard를 사용함 | PASS — 두 validator가 같은 area-backed 조건을 쓴다. |
| 5 | 새 expansion mode가 임의 typed track function을 허용함 | PASS — track list는 비어 있지 않은 fixed `LengthPx`만 허용한다. |
| 6 | `GridTrackSize::Auto`를 가짜 `0px`로 바꿈 | PASS — Taffy로 `GridTrackSize::Auto`를 전달한다. |
| 7 | 새 public CSS/API 또는 FFI surface를 추가함 | PASS — 기존 opt-in fixture/profile 경로만 수정했다. |
| 8 | fixture adapter source가 `runtime_gpu/fixtures` 밖으로 이동함 | PASS — `fixture-module-boundary.test.mjs` 23/23 통과. |
| 9 | 기본 fixture-free source에 matrix payload를 포함함 | PASS — adapter는 기존 fixture feature 뒤에 있으며 boundary test가 통과한다. |
| 10 | 기존 runtime matrix를 재생성하거나 덮어씀 | PASS — v1 거부는 유지하고 v2는 별도 파일이다. |
| 11 | 신규 matrix가 과거 matrix와 분리됨 | PASS — v2가 historical v1 path와 SHA-256을 기록한다. |
| 12 | 현재 inventory를 덮어 expected outcome을 변경함 | PASS — inventory v1을 유지하고 v2가 대체 source를 선언한다. |
| 13 | v2 builder가 고정되지 않은 브라우저 실행에 의존함 | PASS — 기존 Chrome 154/156 근거에 연결되고 `--check`가 통과한다. |
| 14 | v2 summary 수치가 오래됨 | PASS — builder/test가 37 scene, 118 node, 성공 32, fallback 5, 거부 0을 검사한다. |
| 15 | FFI 수치 assertion이 과거 거부 개수를 유지함 | PASS — C11.3.3 FFI suite 7/7 통과. |
| 16 | iOS matrix 검사가 31 scene/90 node를 계속 기대함 | PASS — 37 scene/118 node와 32/5/0 결과를 검사한다. |
| 17 | Android AVD와 physical-device runner 모두 v2를 읽음 | PASS — AVD는 v2를 선택하고 physical runner는 기존 v1 호환을 유지한다. |
| 18 | pixel fixture의 v1 hash를 v2에 잘못 귀속함 | PASS — 양 플랫폼 runner가 legacy pixel fixture hash를 historical v1 기준으로 확인한다. |
| 19 | Android 앱에 중복된 과거 거부 probe가 남음 | PASS — Java probe를 제거하고 전체 AVD matrix가 scene을 직접 검증한다. |
| 20 | iOS 앱에 중복된 과거 거부 probe가 남음 | PASS — Swift probe를 제거하고 전체 iOS matrix가 scene을 직접 검증한다. |
| 21 | single-scene capture가 과거 거부 marker를 계속 요구함 | PASS — 선택한 base scene의 runtime, geometry, surface 결과만 검사한다. |
| 22 | runtime case가 바뀌며 거부 parser test도 사라짐 | PASS — 합성 rejection-contract probe가 error/property/node 및 no-partial-output을 계속 검사한다. |
| 23 | 신규 성공 경로에 직접 layout test가 없음 | PASS — `trailing_template_area_named_lines_use_default_auto_tracks`가 통과한다. |
| 24 | area-free 및 signed guard의 직접 negative test가 없음 | PASS — `implicit_auto_tracks_still_require_template_areas_and_signed_mode_rejects_them`이 통과한다. |
| 25 | style adapter test에서 known-area sibling을 놓침 | PASS — 새 style-to-layout test가 known/unknown node 둘 다 검사한다. |
| 26 | screenshot만으로 platform geometry를 추론함 | PASS — frame/paint/draw log가 Chrome과 비교되고 screenshot과 PixelCopy/Metal은 별도 근거다. |
| 27 | Android evidence가 연결된 실기기를 잘못 선택함 | PASS — command는 `emulator-5554`를 지정하고 API 37, QEMU, SwiftShader metadata가 확인됐다. |
| 28 | iOS evidence를 검증하지 않은 simulator에 귀속함 | PASS — run metadata가 iPhone 17 Pro, iOS 26.2, 선택 UDID를 식별한다. |
| 29 | 이 제한 변경으로 C11.3.3 또는 C11을 완료 표시함 | PASS — C11 checkbox와 공식 집계 `72/213`을 유지한다. |
| 30 | 문서에서 WPT, hardware GPU 또는 compositor 이후 검증을 주장함 | PASS — 계획, 계약, status, evidence에 각 한계를 기록했다. |

## 근거

- [구현/runtime 근거](./README.md).
- [Android API 37 AVD](../c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T201752342Z-ba38b087/README.md).
- [iOS 26.2 Simulator](../c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T202005378Z-3b0cf3b6/README.md).

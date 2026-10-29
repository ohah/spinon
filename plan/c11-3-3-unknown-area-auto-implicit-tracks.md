# C11.3.3 · 알 수 없는 `grid-area` 이름의 기본 auto implicit track

**상태:** 계획 PR [#818](https://github.com/ohah/spinon/pull/818)은 2026-10-29에 병합됐다. 제한 구현은 현재 브랜치에서 Rust·FFI·Android API 37 AVD·iOS 26.2 Simulator 검증을 통과했으며 구현 PR 병합은 대기 중이다. 기존 `unknown-area-implicit-tracks`의 과거 거부는 runtime matrix v1에 보존하고, v2 source-bound matrix에서 Chrome 154 geometry 기준 성공으로 갱신했다. 이 작업은 C11.3.3 candidate·C11 및 공식 완료 `72/213`을 완료로 바꾸지 않는다.

**상위:** [C11.3 계획](./c11-3-template-repeat-named-lines-areas.md) · [unknown named line 후속](./c11-3-3-unknown-named-area-implicit-lines.md) · [area implicit-track 목록 순환](./c11-3-3-template-area-track-list-cycling.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**규범 기준:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) §7.3.2, §7.5–7.6, §8.3–8.4, §11.4–11.8

**고정 oracle:** Chrome for Testing `154.0.8037.92` (macOS arm64), 기존 [C11.3.3 reference](../tests/fixtures/css/references/c11-3-3-grid-template-areas-precomparison-v1.json); source fixture [grid-template-areas-c1133.html](../tests/fixtures/css/c11/grid-template-areas-c1133.html)의 `unknown-area-implicit-tracks` case. 이 custom fixture는 WPT testharness 실행 결과가 아니다. 기존 거부는 [immutable runtime matrix v1](../tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v1.json)에 남고 구현 결과는 [inventory v2](../tests/fixtures/css/c11/grid-template-areas-c1133-inventory-v2.json)와 [runtime matrix v2](../tests/fixtures/css/c11/grid-template-areas-c1133-runtime-matrix-v2.json)에 기록된다.

**계획 검토:** [계획 실패 관점 20개](../spec/internal/evidence/c11-3-3-unknown-area-auto-implicit-tracks-plan-review-20.md)

## 목표

`grid-template-areas`에 선언되지 않은 이름을 `grid-area`로 지정하면 CSS Grid의 line-placement 규칙에 따라 implicit named line을 찾고, explicit grid 뒤에 필요한 implicit track을 만든다. 해당 track의 초기 크기인 `auto`는 `grid-auto-rows`와 `grid-auto-columns`의 기본값이다. 현재 C11.3.3 adapter는 이 입력을 거부한다. 이 계획은 그 거부를 제거하되 기존 named-line/area precedence와 fixed-px track-list 경로를 보존하고, `auto` track을 값만 보고 0px로 바꾸지 않고 Grid track-sizing 계산에 전달한다.

## 고정 입력과 사전 판정

기존 fixture의 scene은 viewport `320×240 CSS px`, root `160×120 CSS px`, LTR·`horizontal-tb`, zero gap이다. Root는 columns `80px 80px`, rows `60px 60px`, areas `"known ." ". ."`를 가진다. Child는 빈 box이며 `grid-area:not-authored`를 지정한다. 두 `grid-auto-*` longhand는 초기값 `auto`다. 별도의 fixed-track visible-overflow control은 [unknown-named-area inventory](../tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133-inventory.json)와 [pixel samples](../tests/fixtures/css/c11/unknown-area-fixed-implicit-tracks-mobile-pixels-v1.json)에 있다.

고정 Chrome 154 결과는 DPR 1·2에서 같다.

| 관측 | 기대값 |
| --- | --- |
| resolved columns | `80px 80px 0px 0px` |
| resolved rows | `60px 60px 0px 0px` |
| root frame | `(0, 0, 160, 120)` |
| unknown-area item frame | `(160, 120, 0, 0)` |

기준 자료 출처: source fixture SHA-256 `953bb95b881ae4cd41a31f7424fc0bb6db16fe75d498429268005a43f26c4bac`; reference JSON SHA-256 `e92cb8fc33c9c55a20871d4455f4d67f6179aae239b94d0ac12afdfc337e3913`; current expected-rejection runtime matrix SHA-256 `62baa58bab5af9fd9cfb498ed4a5a979202a5e43bba5438952358cfc204358ee`. Chrome executable SHA-256은 `ce0c09ea98a7592e7799e54c1867797ccf99d4d6e821a11067c332e454732349`이며 DPR 1 screenshot SHA-256은 `e04880c594c81aede81f694f2b9608ddc46047610eed7cbb636570cf83278db2`다. 고정 환경은 `en-US`, UTC, light color scheme, forced colors none, coarse pointer, hover none, LTR·`horizontal-tb`, viewport `320×240`, DPR 1·2다.

독립 기대값은 CSS Grid §8.3의 custom-ident fallback/implicit-line 검색, §7.5–7.6의 implicit track 생성과 기본 auto track size, §11.4–11.8의 empty contribution 및 available free-space 처리 순서로 유도한다. Explicit rows와 columns가 definite root 축을 이미 전부 차지하므로 빈 item의 두 trailing auto track은 0px이다. Chrome reference는 이 계산을 검산하는 oracle로만 사용하고 Spinon WPT pass로 부르지 않는다.

## 구현 계약

1. 기존 `unknown-area-implicit-tracks` fixture와 node identity를 유지한다. 새 scene ID를 만들거나 과거 expected rejection을 성공 기록으로 덮어쓰지 않고, source-bound 새 runtime result로 기존 case의 기대 결과를 갱신한다.
2. `grid-area` bare-ident 해석은 explicit named area 및 area-derived line alias를 먼저 조회한다. 이 둘이 없을 때만 CSS §8.3 규칙의 implicit named-line placement를 적용한다. 각 axis의 start/end, implicit extent와 `grid-area` shorthand 확장을 개별 검사한다.
3. 새 placement가 필요로 하는 trailing row·column implicit track은 C11.4 `TrackSizing` 계산에 쓰는 동일한 `auto` sizing 경로에 전달한다. 계산 결과의 track count/order와 `NodeId` frame은 Chrome reference 및 독립 기대값과 비교한다. `auto`를 고정 `0px`로 투영해 다른 item contribution을 숨기지 않는다. C11.4 경로가 이미 지원하는 item contribution은 계속 sizing에 포함하고, 그 경로에서 지원하지 않는 contribution은 명시 오류로 남긴다.
4. `TemplateGridImplicitTracks`와 기존 signed/area track-list mode의 경계를 확인한다. `grid-auto-*`의 유한 fixed-px list 순환, explicit area sizing, C11.3.1–3.2 profile은 그대로 동작해야 한다. 이 변경만으로 arbitrary author CSS나 일반 `auto` track sizing 전체를 완료로 표시하지 않는다.
5. 동일 template-area scene의 known-area sibling은 frame·source order·paint ordinal이 유지된다. 직접 `grid-row`/`grid-column` numeric placement, 명시 named line, fixed-px `grid-auto-*`, malformed computed input과 default auto input을 분리한 positive/negative control을 둔다. Existing C11.4 empty, fixed-size, text/image contribution cases는 새 mode 또는 shared sizing path에서 regression으로 실행해 empty-only `0px` special case가 들어가지 않았는지 확인한다.
6. Track/output allocation과 index arithmetic은 기존 Grid quota 및 fallible reservation/error atomicity 계약을 따른다. 지원 밖 intrinsic contribution, negative/leading extent, track limit 초과 또는 stale revision은 frame/draw/readback을 부분 게시하지 않는다.
7. Rust projection/layout 검사는 computed `grid-auto-*`, resolved track arrays, item/root frame, error와 revision을 확인한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서는 같은 fixture input을 최신 source로 실행해 scene, track, frame, draw/present와 해당 고정 sample을 검사한다. AVD는 software renderer이며 iOS Metal drawable은 compositor 제출 전이다.
8. 기본/product FFI archive에서 fixture symbol·payload가 빠지고 opt-in fixture-test positive control에만 나타나는지 확인한다. Android/iOS simulator build 결과를 앱 실행 근거와 별도로 기록한다.
9. 구현/runtime evidence는 구현 실패 관점 20개로, 각 PR diff는 독립 실패 관점 30개로 별도 검토한다. 발견된 문제는 수정 후 영향받는 관점을 재검토한다.
10. `spec/internal/0059-c11-grid-layout.md`, `spec/STATUS.md`, 이 계획, CSS reference inventory 및 public roadmap은 실제 결과에 맞춰 갱신한다. Tailnet preview는 반영 후 HTTP status·hash·Agent Browser 화면으로 검증하고 브라우저 프로세스를 닫는다. GitHub Pages를 배포하지 않는다.
11. 구현은 현재 체크아웃의 일반 feature branch에서 순차로 진행한다. 새 worktree나 동시 subagent를 만들지 않는다.

## 완료 기준

- Chrome 154 baseline, CSS Grid 독립 기대값, Rust computed/projection/layout output이 track sizes와 node frames에서 일치한다.
- 새 opt-in runtime path가 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최신 source로 통과하고, rejection/no-partial-output controls가 의도한 경계에서 실패한다.
- 기존 C11.3.3 area, named implicit line, fixed-px track-list, C11.3.1–3.2 회귀와 default/product FFI boundary가 확인된다.
- 구현/runtime 20관점과 각 PR의 독립 diff 검토 30관점을 별도 문서로 마치고 PR을 Rebase and merge한다.
- 공개/Tailnet roadmap은 해당 bounded behavior와 남은 미지원 범위를 보여준다. C11.3.3/C11 checkbox와 공식 완료 `72/213`은 그대로다.

## 구현 결과 · 2026-10-30

`TemplateGridImplicitTracks`에서 template areas가 있는 trailing implicit track에만 기본 `auto` track을 허용한다. `GridTrackSize::Auto`를 Taffy의 auto track sizing에 전달하며, area가 없거나 signed/leading mode에서는 거부를 유지한다. Auto track-list grammar는 fixed-px list만 허용한다. Android/iOS 앱의 기본 scene 시작 시 `unknown-area-implicit-tracks`를 예전 거부 조건으로 중복 실행하던 probe를 제거해, 버전 있는 전체 matrix가 해당 scene을 직접 검증한다.

새 v2 scene의 Chrome 기대 geometry는 root `(0,0,160,120)`, 빈 unknown-area item `(160,120,0,0)`이다. Android API 37 ARM64 AVD/SwiftShader와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 37 scene·118 node, 성공 32·cascade fallback 5·기대 거부 0, 최대 geometry 오차 0 CSS px를 확인했다. `unknown-area-implicit-tracks` 두 node 모두 Chrome geometry와 exact match했다. 기본 surface 표본은 각 2/2, fixed-track overflow control은 PixelCopy/Metal 3/3, track-list cycle 표본은 각 28/28 exact다. Metal readback은 compositor 제출 전이다.

로컬 검증은 `spinon-layout` grid tests 19/19, style-to-layout C11.3.3 19/19, feature-gated FFI C11.3.3 7/7, C11.4 auto sizing regression 13/13, C11.4 text/image regression 3/3, iOS matrix + CSS reference tests 35/35, fixture-module boundary 23/23, matrix builder `--check`, workspace Clippy `-D warnings`, rustfmt를 통과했다. macOS arm64 Release FFI archive에서 기본 product build의 fixture symbol/payload 부재와 opt-in build의 포함을 확인했다. 최종 implementation PR diff review 30개와 구현/runtime review 20개는 별도 evidence에 둔다. 초기 AVD 시도에서 구형 앱 probe가 새 성공 결과를 실패 처리해 멈춘 점과, 첫 iOS 시도에서 legacy pixel fixture hash를 v2 matrix와 직접 비교한 오류를 각각 수정한 뒤 양쪽 전체 행렬을 완료했다.

재현 근거: [구현/runtime 검토와 실행 요약](../spec/internal/evidence/c11-3-3-unknown-area-auto-implicit-tracks-implementation-2026-10-30/README.md) · [Android API 37 AVD 결과](../spec/internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-30/android-avd-c1133-matrix-20261029T201752342Z-ba38b087/README.md) · [iPhone 17 Pro Simulator 결과](../spec/internal/evidence/c11-3-3-grid-template-areas-runtime-2026-10-30/ios-simulator-c1133-ios-matrix-20261029T202005378Z-3b0cf3b6/README.md). 이 결과는 WPT 적합성, 실기기/hardware GPU, iOS compositor 이후 pixels, arbitrary author CSS 또는 전체 CSS Grid 지원을 뜻하지 않는다.

## 제외 범위

이 계획 단독으로는 새로운 text/image intrinsic contribution 알고리즘, 임의 `grid-auto-*` grammar, negative/leading implicit placement, 일반 named-area 오류 복구, C11.4에서 아직 지원하지 않는 sizing 조합, Spinon WPT runner/적합성, Android 실기기·hardware GPU, iOS compositor 이후 픽셀 또는 전체 CSS Grid 지원을 주장하지 않는다. 이미 구현된 C11.4 contribution 동작은 가능한 한 동일한 sizing path에서 보존한다. 미지원 입력은 명시 거부해야 하며 새 성공 fallback으로 숨기지 않는다.

# C11.4 direct-text `auto <ratio>` content-box 계획

**상태:** 계획 PR [#845](https://github.com/ohah/spinon/pull/845)와 projection-gate 보완 PR [#846](https://github.com/ohah/spinon/pull/846)은 병합됐다. 구현은 기존 16개 회귀 장면과 신규 7개 장면을 Android·iOS Simulator에서 각각 23/23, 표면 표본 72/72 exact로 통과했고 Rust·Clippy 및 FFI archive 경계를 확인했다. 별도 구현/runtime 20관점 및 PR diff 30관점 검토도 기록했다. 인접 C07.3 computed-style 문자열 비교 1건(4/5)은 이 변경의 serializer 수정 없이 남아 있어 PR 전 별도 판정이 필요하다. 구현 PR 검토·병합은 아직 완료되지 않았다. C11.4/C11 완료 표시는 아니다.

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [finite-positive ratio 선행 slice](./c11-4-direct-text-aspect-ratio-finite.md) · [C11 Grid 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md)

## 문제와 목표

PR [#842](https://github.com/ohah/spinon/pull/842)은 Stylo의 유한 양수 숫자 ratio를 Ahem direct-text intrinsic track fixture에 연결했다. 그 구현은 `aspect-ratio: auto <ratio>`를 cascade에서 거부한다. 현재 `ComputedElementStyle.layout_aspect_ratio: Option<f32>`는 숫자 ratio만 나타내므로, 숫자의 값을 유지하면서 ratio가 어느 box의 폭/높이를 가리키는지도 보존할 수 없다.

[CSS Box Sizing Level 4 §4.1](https://www.w3.org/TR/2026/WD-css-sizing-4-20260904/#aspect-ratio)은 `auto && <ratio>`의 크기 계산이 항상 content box 기준이라고 설명한다. 이는 `box-sizing:border-box`인 non-replaced box에서 일반 숫자 `<ratio>`가 `box-sizing`으로 선택된 box 기준인 것과 다르다. 이 문서는 탐색 단계 Working Draft이므로 단독 지원 계약으로 승격하지 않는다. [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-auto-precomparison-2026-10-30/README.md)가 고정 Ahem/Grid 입력에서 이 차이를 관측했다.

목표는 기존 C11.4 opt-in fixture-test profile에서 **HTML non-replaced direct-text Grid item 하나**에만 `auto <ratio>`의 content-box basis를 보존해 row contribution과 column max-content track에 연결하는 것이다. 일반 CSS admission을 열지 않고 숫자 ratio의 현재 `box-sizing` 의미와 다른 profile의 fail-closed 동작을 유지한다.

## 사전 기준

고정 입력, 독립 계산, Agent Browser HeadlessChrome 156 관측, screenshot 및 hashes는 [사전 비교 evidence](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-auto-precomparison-2026-10-30/README.md)에 있다. Ahem `AAAA` content advance는 40px, `A`는 10px이다. Item은 `padding:2px 4px`, 1px border라서 horizontal padding+border는 10px, vertical padding+border는 6px다.

| 장면 | 독립 기대값 | Chrome 156 관측 |
|---|---|---|
| auto row, border-box `auto 3/2` | content `40×26⅔`; outer height `32⅔px`; columns `[20,50]` | row/item height `32.65625px`; columns `20px 50px` |
| auto row, border-box numeric `3/2` | border-box height `50/1.5 = 33⅓px`; columns `[20,50]` | row/item height `33.328125px`; columns `20px 50px` |
| auto row, border-box `auto` control | line-height 10 + vertical edges 6 = `16px` | row/item height `16px` |
| auto row, content-box `auto 3/2` | 같은 content ratio, outer height `32⅔px` | row/item height `32.65625px` |
| definite 60px row, border-box `auto 3/2` | content height 54, content width 81, outer width `91px` | columns `[91,20]`; item `91×60px` |
| definite 60px row, border-box numeric `3/2` | border-box width `60×1.5 = 90px` | columns `[90,20]`; item `90×60px` |
| definite 60px row, border-box `auto` control | `AAAA` max-content outer width `40+10 = 50px` | columns `[50,20]`; item `50×60px` |

Chrome used dimensions은 1/64 CSS px 단위로 quantize되어 있으므로 32⅔·33⅓에 대한 구현 비교는 독립 rational과 별도 Chrome tolerance를 둔다. 두 수치의 기준은 서로 바꿔 쓰지 않는다.

## 제한 입력·typed contract

- 기존 `RuntimeBlockFlexGridTrackSizingV1` / `c11-4-grid-track-sizing-fixture-test` route만 사용한다. 사용자 CSS나 다른 product profile은 열지 않는다.
- Root는 HTML namespace의 단일 in-flow Grid다. 너비 100px, 높이 auto, Columns `max-content max-content`, Rows `auto` 또는 `60px`, gap 0, `align-items:normal`, normal self/content alignment, zero root margin/padding/border를 요구한다.
- 정확히 두 in-flow HTML block direct child를 가진다. 각각 direct text node 하나만 소유하고 내용은 `A` 또는 `AAAA`다. Font는 저장소 Ahem 10px/10px, horizontal-tb/LTR, nowrap다. Anonymous text, nested/extra item, replaced image, explicit placement, nonzero order, wrapping은 거부한다.
- At most 한 `AAAA` item에 ratio를 둔다. Auto-ratio candidate는 computed `aspect-ratio` serialization이 `auto <finite-positive ratio>`이고 Stylo typed ratio/basis와 일치해야 한다. `A`에 ratio, 복수 ratio item, 불일치·stale snapshot은 fail-closed다. Ratio 값을 scene ID, label, authored property string에서 추론하지 않는다.
- Item은 `width/height/min-width/min-height:auto`, `max-width/max-height:none`, margin 0이어야 한다. Border-box positive case는 네 면의 `padding:2px 4px`, 1px solid border다. Content-box positive control은 같은 edges를 쓴다. Percentage, `calc()`, 음수·비유한 edge 및 다른 border shape는 제외한다.
- Cascade snapshot은 ratio 수치뿐 아니라 **sizing basis**를 typed 값으로 보존한다. 제안 타입은 `ComputedLayoutAspectRatio { ratio: f32, sizing_box: ComputedAspectRatioSizingBox }`와 `BoxSizing`/`ContentBox` 두 variant다. 숫자 `<ratio>`는 CSS `box-sizing` 기준, `auto <ratio>`는 `ContentBox` variant를 갖는다. `layout_aspect_ratio == None`은 computed serialization이 정확히 `auto`인 경우에만 no-ratio control이다.
- `auto <ratio>`를 typed로 노출하는 cascade 허용은 `RuntimeBlockFlexGridTrackSizingV1`에서만 한다. C07.3, flex, block 등 현재 profile의 rejection은 계속 유지한다. Degenerate ratio, `auto 0/0`, malformed ratio 및 이 fixture에서 파생 geometry가 f32 범위를 벗어나는 값은 accepted positive로 축약하지 않고 거부한다.
- C11.4 cascade는 `RuntimeBlockFlexGridTrackSizingV1`을 여러 fixture가 공유하므로 프로필 하나만으로 Taffy sizing basis를 바꾸지 않는다. Direct-text collector는 accepted `auto <ratio>` owner의 정확한 `NodeId`를 `C11_4TextInput` projection marker에 담고, runtime dispatcher는 이 marker가 있을 때만 전용 style-to-layout fixture entry point를 호출한다. 그 entry point는 `StyleLayoutProjectionOptions`에 같은 `NodeId`를 전달한다. Projection은 해당 노드의 typed basis가 `ContentBox`이고 computed serialization과 일치할 때만 Taffy item의 `box_sizing`을 `ContentBox`로 바꾼다. 다른 owner, duplicate/missing marker, 그 외 ContentBox ratio는 layout 전에 fail-closed한다. 원래 computed `box-sizing` property는 snapshot에 유지한다.

## 구현·검증 판정

1. Stylo cascade가 ratio와 sizing basis를 동일 computed-value snapshot에서 추출하도록 typed representation을 수정한다. `auto <ratio>` admission은 C11.4 profile에 한정하고, computed serialization과 typed values의 ratio numerator/denominator 및 basis가 다르면 오류로 거부한다.
2. Style-to-layout projection 및 C11.4 direct-text collector를 확장한다. collector가 유일한 ratio owner `NodeId`를 내고 전용 projection entry point가 그 정확한 ID만 override할 수 있게 한다. Taffy adapter에는 content-box basis만 전달하고 CSS computed declarations를 그대로 보존한다. Taffy `box-sizing` 조정이 padding/border, auto dimensions 및 intrinsic measurement를 어떻게 반영하는지 unit-level test와 runtime trace로 판별한다. 예상과 다르면 임의로 edge를 더하거나 빼지 말고 profile을 거부한 뒤 알고리즘/contract를 재설계한다.
3. 신규 고정 scene 7개(표의 모든 행)를 fixture registry에 넣는다. 기존 PR #842의 16-scene finite-positive/2/1 회귀와 no-ratio control을 계속 통과시킨다. Track contribution, used Columns/Rows, root/item frame, overflow와 computed typed basis를 각각 검사한다.
4. Negative controls는 numeric `3/2`를 content-box로 잘못 분류, `auto 3/2`를 border-box로 잘못 분류, basis/string mismatch, missing/wrong/duplicate `NodeId` projection marker, non-C11 projection call, `auto 0/0`, invalid declaration이 만든 computed `auto`, A item ratio, two ratio owners, extra/replaced/nested item, nonzero margin, wrong edge, percent/calc edge, explicit dimensions/min/max, wrong root/track/row, wrong font, stale revision, extreme/nonfinite synthetic ratio, allocation failure atomicity를 포함한다.
5. Android API 37 ARM64 QEMU AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 신규 7개와 기존 16 controls를 각각 실행한다. Scene별 Source revision, artifact hash, track/contribution trace, frame, draw/present 및 표면 표본을 기록한다. Android software renderer, iOS compositor 제출 전 drawable 한계를 명시하고, test body에는 device screenshot을 연결한다.
6. Default/product static archive에 fixture route/symbol/string/payload가 없고 fixture-test positive control에는 있는지 재검사한다. Focused Rust/style/projection/runtime/FFI tests, 관련 crate gates, `cargo fmt`, feature Clippy 및 Android/iOS simulator build를 기록한다. 미실행 결과는 통과로 쓰지 않는다.
7. 내부 계약 [0059](../spec/internal/0059-c11-grid-layout.md)에 제한 입력, basis 전달, 오류·fallback과 미지원 범위를 기록하고 숫자 계약 `0.1.0`은 유지한다. [공식 상태 대장](../spec/STATUS.md)의 C11/C11.4 checkbox와 `72/213`은 그대로 둔다. 상위 계획과 Tailnet preview를 PR 병합 후 동기화한다.
8. 이 계획 자체는 별도 실패 관점 20개로 검토하고 계획 PR을 먼저 병합한다. 이후 구현/runtime 20관점과 별도 구현 PR diff 20관점을 새로 검토한다. PR #842와 이 계획의 테스트/리뷰 결과를 한 구현 slice로 합쳐 세지 않는다.

## 구현/runtime 결과 · 2026-10-30

- 수치 ratio와 sizing basis를 `ComputedLayoutAspectRatio { ratio, sizing_box }`로 보존한다. `auto <ratio>`는 `ContentBox`, 일반 숫자 ratio는 `BoxSizing`이다. content-box ratio projection은 일반 API에서 거부하고, C11.4 opt-in 경로에서 collector가 지정한 단일 `NodeId`에만 허용한다.
- Taffy Grid ratio transfer는 content-box dimensions에 ratio를 적용하고 padding/border를 결과에 더한다. numeric ratio의 기존 box-sizing basis는 유지한다.
- 기존 finite-positive 행렬 16개와 신규 `auto <ratio>`·numeric·no-ratio 장면 7개를 합친 23개를 실행했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator가 각각 23/23 장면, 72/72 표면 표본을 통과했다. 결과·입력 hash·화면·한계는 [runtime evidence](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-auto-runtime-2026-10-30/README.md)에 있다.
- `cargo fmt --all -- --check`, 기본 및 fixture-feature Clippy, runtime/style/layout focused tests와 기본/opt-in FFI archive marker 검사를 기록했다. 인접 `c07_3_aspect_ratio_tests`는 4/5 통과하고, 기존 computed-value 문자열 `1.7777778 / 1` 대 Chromium `1.77778 / 1` 비교 1개가 남아 있다. 해당 serializer는 이 변경에서 수정하지 않았고, 실패를 통과로 세지 않는다.
- 공식 C11.4/C11 체크와 완료 `72/213`은 유지한다. 일반 CSS author admission·WPT·실기기·hardware GPU 범위는 열지 않는다.

## 제외 범위

일반 CSS author admission, replaced/natural ratio, 다른 font/script/language, multiline/wrapping, multiple ratio owners, min/max transfer, non-auto dimensions, percentage/calc edges, transforms, writing-mode variants, baseline, full WPT runner, physical Android device, hardware GPU, iOS compositor 이후 pixels, 전체 Grid/C11.4 지원 완료는 포함하지 않는다. Chrome 결과는 이 7 fixed scene의 oracle일 뿐 플랫폼 일반 일치 보장이 아니다.

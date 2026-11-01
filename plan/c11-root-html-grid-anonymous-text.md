# C11.1 parsed Root HTML 기본 Grid anonymous text 제한 구현 계획

**상태:** 제한 구현 PR [#1011](https://github.com/ohah/spinon/pull/1011)은 2026-11-01T23:22:20Z Rebase and merge로 병합됐다(merge SHA `40725c44d5cfe7ea187fe7731c47fed439c3b98f`). 기본 parsed Root HTML runtime 경로에서 direct text `AB` 하나를 anonymous Grid item으로 배치·paint하는 제한 지원과 Android API 37 ARM64 AVD/iOS Simulator 근거가 반영됐다. 이 slice는 C11.1·C11.4·C11 완료 요건을 충족하지 않으므로 공식 완료 `72/213`은 유지한다. 다른 Grid direct text 조합과 지원 밖 입력은 계속 fail-closed한다.

**기준:** CSS Grid Layout Module Level 1 2025-03-26 CRD [§6 Grid Items](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-items), [§8.5 Grid Item Placement Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo). 독립 입력, 예상값, Chrome 156 관측, 화면 및 hash는 [사전 비교 자료](../spec/internal/evidence/c11-root-html-grid-anonymous-text-precomparison-2026-11-02/README.md)에 고정한다.

## 현재 결손

기본 parsed Root HTML 경로는 고정된 단일 direct text run `AB`만 지원하며, 다른 visible Grid direct child text 조합은 scene publication 전에 거부한다. C11.4 layout collector는 anonymous Grid text paragraph/item을 만들 수 있지만 `allow_c11_4_anonymous_grid_text_items`라는 제한 profile gate로 호출된다. 이 계획은 C11.4 fixture gate를 임의로 켜서 우회하지 않았다. Root HTML 입력의 computed style, document identity와 revision을 유지하면서 renderer가 이해하는 text paragraph와 layout item을 연결하는 경계를 이번 제한 slice에 반영했다.

CSS Grid §6에 따르면 in-flow direct text sequence는 anonymous block-container grid item이 되고, 해당 sequence가 white-space 처리 후 whitespace만이면 렌더링하지 않는다. §8.5의 anonymous item은 항상 자동 배치된다. 기준 입력은 한 개 item과 definite fixed-px tracks뿐이므로 첫 row-major cell의 위치·크기를 CSS Grid cell geometry와 normal stretch 규칙으로 독립 계산할 수 있다.

## 지원 후보

- `RuntimeSession::new_runtime_gpu_with_root_html()`의 기본 parsed Root HTML route를 확장한다. C11.4 전용 fixture 진입점이나 opt-in FFI/JNI/Swift payload를 추가하지 않는다.
- 하나의 visible HTML Grid root, definite `120×70px`, columns `40px 80px`, row `70px`, gap `0`, 기본 row flow, LTR·horizontal writing mode, zero edge, visible overflow만 사용한다.
- visible Grid direct children은 하나의 plain text run뿐이다. 값은 `AB`, computed `white-space: normal`, 기본 text alignment다. adjacent text node source는 같은 anonymous run으로 모을 수 있지만 NodeId 및 source-node provenance는 모두 유지한다.
- inherited computed text tuple은 `white-space: normal`, `font-family: monospace`, `font-size: 16px`, `line-height: 20px`, `color: #e5484d`, LTR와 기본 text alignment로 고정한다. 값 전달은 같은 Grid owner style snapshot에서 검증한다.
- 빈 run·CSS whitespace 처리 후 whitespace만인 run은 anonymous item을 만들지 않고 기존 explicit tracks만 유지한다.
- anonymous item은 원본 text node의 안정된 anchor NodeId와 동일한 document/style/layout revision에 결합한다. DOM에 synthetic element를 삽입하거나 HostDocument child vector를 재정렬하지 않는다.
- 기준의 text item frame은 `(0,0,40,70)` CSS px, text 시작은 `(0,0)`, 한 줄이다. Chrome의 monospace Range 폭 `19.265625px`은 관측값이며 플랫폼 고정 glyph metric이나 pixel equality 기준이 아니다.

## 제한 및 거부 입력

- direct text와 element grid item 혼합, 서로 다른 text sequence를 만드는 in-flow element separator, 두 개 이상의 visible text run은 이 slice에서 거부한다. 일반 Grid source-order auto-placement 전체를 구현했다고 주장하지 않는다.
- `white-space: pre|pre-wrap|pre-line|break-spaces`, line wrapping, non-default `text-align`, writing mode·direction 변형, text decorations/transforms, inline descendants, nested Grid, implicit tracks, non-px/indefinite tracks, unsupported font/style values는 성공 scene 전에 구체 진단으로 거부한다.
- whitespace-only control은 `white-space: normal`에 한정한다. 보존 공백이 시각 item이 되는 동작은 후속 범위다.
- stale/mismatched document·style/environment revision, duplicate/missing source NodeId, malformed UTF-16, allocation failure, unsupported layout/style, partial paragraph collection 또는 partial draw는 성공으로 내보내지 않는다. 기존 성공 scene이 남아 실패 입력의 결과처럼 보이지 않도록 atomic publication 경계를 확인한다.
- C11.4 collector를 generic API로 전환해야 할 경우 별도 feature-off/opt-in control을 둔다. 일반 경로와 C11.4 fixture 경로의 동작을 서로 바꾸지 않고 각각 회귀 검사한다.

## 고정 비교와 판정

고정 fixture [root-html-grid-anonymous-text-precomparison.html](../tests/fixtures/css/c11/root-html-grid-anonymous-text-precomparison.html)은 target run과 whitespace-only control을 포함한다. 독립 계산은 tracks 합과 grid size를 확인하고 item의 첫 cell frame을 산출한다. Agent Browser HeadlessChrome 156은 computed tracks, root DOMRect, child text node와 Range rect를 관찰했다. W3C 규칙이 판정 기준이고 Chrome은 독립 구현 관찰이다.

| 장면 | 기준 기대값 |
|---|---|
| `AB` direct text | root `(0,0,120,70)`; columns `[40,80]`; rows `[70]`; item `(0,0,40,70)`; text range origin `(0,0)`·한 줄 |
| whitespace-only direct text | anonymous item 0개; range rect 0개; root와 explicit tracks 유지 |
| direct text + block sibling | 현재 기본 route의 fail-closed 동작 유지; item 순서 혼동·부분 scene 없음 |
| `white-space:pre` | 성공으로 바꾸지 않고 fail-closed; 보존 공백 semantics를 normal whitespace와 혼동하지 않음 |

텍스트 색·배경이 보이는 Android/iOS simulator 캡처는 PR 보조 화면으로 첨부할 수 있다. 화면은 layout geometry·draw/present 경로의 보조 근거다. AVD screenshot은 hardware GPU 성능이나 Android 실기기 결과가 아니고, iOS Simulator screenshot은 compositor 이후 표면 pixel equality를 뜻하지 않는다. 고정 monospace glyph를 기준으로 simulator별 pixel equality를 주장하지 않는다.

## 구현 후 검증 계획

1. 사전 비교 fixture, JSON, verifier, screenshot과 SHA-256 manifest를 다시 확인하고 변경 전 기준을 보존한다.
2. Root HTML default renderer·text paragraph collector·HostDocument layout adapter 경계를 추적한다. C11.4 flag를 단순히 `true`로 바꾸지 말고 Root HTML 전용 admission과 source/revision provenance를 검증한다.
3. 기존 direct Grid text rejection 회귀를 성공 기준 fixture로 세분화한다. `AB` positive, normal whitespace-only, text+block sibling reject, preserved whitespace reject, text run 중복·separator reject와 stale revision/failure atomicity를 고정한다.
4. Rust focused Root HTML/Grid/text tests에서 root/track/item frames, text origin/line count, source NodeId set, document/style/layout revision identity 및 diagnostic을 확인한다. C11.4 anonymous Grid text suite와 기본 Grid named-area/fixed-gap/overflow 회귀를 별도로 실행한다.
5. Chrome 156 reference에 대응하는 기본 parsed Root HTML source를 Android API 37 ARM64 AVD 및 iOS Simulator에서 실행한다. runtime frame, draw/present, 성공·기대 거부, root 경계 밖 표본과 source/app hash를 기록한다. 가능하면 각 플랫폼 screenshot을 PR 본문에 첨부한다. AVD를 실기기·hardware GPU로, iOS drawable을 compositor 이후 픽셀로 표현하지 않는다.
6. default product archive에서 C11.4 fixture marker·payload가 없는지 확인하고 fixture-test positive control에서 기존 C11.4 entry가 유지되는지 확인한다.
7. focused tests, 관련 package tests, `cargo fmt --all -- --check`, scoped Clippy, Android/iOS simulator builds 및 runtime 행렬을 실행한다. 실패와 미실행 gate를 구분해 기록하고 전체 workspace가 실패하면 원인과 baseline을 분리한다.
8. 실제 구현/runtime을 대상으로 이 계획과 다른 독립 실패 관점 20개를 새로 검토하고, 실제 PR diff에는 별도의 30개 관점을 적용한다. 계획 검토를 구현 검토로 재사용하지 않는다.
9. 병합 후 `spec/STATUS.md`, C11 plan, public roadmap와 Tailnet preview를 실제 merge HEAD에 맞춘다. C11.1·C11.4·C11 완료 체크와 `72/213` 집계는 이 slice만으로 바꾸지 않는다.

## 완료 주장에서 제외

이 제한 slice는 모든 anonymous text sequence 조합, mixed element/text order, inline formatting, preserved whitespace, line wrapping·intrinsic contribution, text style 전체, Grid 일반 auto-placement, WPT/Spinon conformance, 공개 CSS 지원, C11.1 전체, C11.4 전체, C11 상위, Android 실기기·hardware GPU 또는 iOS compositor 이후 pixel correctness를 완료로 표시하지 않는다.

## 계획 실패 관점 검토

[구현 전 계획 실패 관점 20개](./c11-root-html-grid-anonymous-text-plan-review-20.md)를 별도로 수행했다. 계획 PR 변경 전체의 독립 30관점 대조는 [계획 PR diff 검토 30개](../spec/internal/evidence/c11-root-html-grid-anonymous-text-precomparison-2026-11-02/plan-pr-diff-review-30.md)에 기록한다.

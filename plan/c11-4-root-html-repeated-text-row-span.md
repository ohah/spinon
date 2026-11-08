# C11.4 · 반복 intrinsic row의 텍스트 행 span 기여

**상태:** 제한 구현 PR [#1295](https://github.com/ohah/spinon/pull/1295) Rebase and merge · C11.4/C11은 진행 중이며 공식 완료 `78/214` 유지

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**직전 구현:** [반복 intrinsic row의 제한 텍스트 content grammar](./c11-4-root-html-repeated-intrinsic-text-content.md) · PR [#1292](https://github.com/ohah/spinon/pull/1292)

**사전 비교:** [Chrome 관측·960 scene 검증](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-precomparison-2026-11-09/README.md) · [HTML fixture](../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-precomparison.html) · [matrix](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-precomparison-2026-11-09/chrome157-matrix-observation.ndjson)

**계획 실패 관점 검토:** [서로 다른 관점 20개](./c11-4-root-html-repeated-text-row-span-plan-review-20.md) · **계획 PR diff 검토:** [실제 변경 실패 관점 30개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-precomparison-2026-11-09/plan-pr-diff-review-30.md)

## 결손과 목표

기본 parsed Root HTML의 `RootHtmlFitContentAutoRowsV1` 반복 텍스트 profile은 여덟 direct text item을 측정하지만 모든 item을 auto-placement에 두고 한 implicit row 안에 제한한다. 일반 Grid에서 여러 행을 점유하는 text item은 그 item의 min-/max-content contribution을 span 전체의 track sizing 단계에 전달해야 한다. 기존 C11.4 multi-span oracle는 별도 opt-in fixture profile이며 이 기본 HTML 경로를 증명하지 않는다.

이번 slice는 현재 반복 track list 및 텍스트 grammar를 유지하면서 direct text item의 2행·3행 span을 제한적으로 허용한다. CSS Grid track sizing과 auto-placement가 열 축의 빈 implicit track을 만들지 않도록 모든 해당 item은 열 1 하나에 명시적으로 둔다. 실제 Root HTML parsing → cascade/style projection → 기본 RuntimeSession → layout path를 사용한다. 새 공개 API, public CSS support 선언, 별도 fixture 성공 경로는 추가하지 않는다.

## 고정 입력과 독립 비교 모델

- Grid는 40×220 CSS px이며 explicit columns는 `[40px]`, explicit rows는 `[10px]`다. Gap은 0, direction은 LTR, overflow는 visible, content alignment는 `start|stretch`, item alignment는 기존 `normal`이다.
- `grid-auto-rows`는 기존 six-member list만 허용한다. 첫 member는 유한하고 0 이상인 `fit-content(px|direct percentage)`이며 뒤 다섯 member는 `min-content` 2개, `max-content` 2개, `auto` 1개의 기존 30개 고유 순서다. Percentage basis는 definite 220px block size다.
- 첫 direct child는 비어 있고 row 1, column 1에 고정된 10px explicit-row item이다. 뒤의 direct child 수는 2 또는 3개다. Text item은 각자 독립된 direct wrapper와 text paragraph 하나를 가지며 기존 #1292의 1–32 byte ASCII 대문자 grammar와 10px monospace/10px line-height 조건을 그대로 쓴다.
- span placement는 `A: grid-row 2 / span 2`, `B: 3 / span 2`, 선택적으로 `C: 2 / span 3`이다. 모든 항목은 `grid-column: 1 / 2`다. DOM order는 span-2 forward/reversed와 span-2-before-span-3/span-3-before-span-2 두 쌍을 둔다.
- 텍스트는 A 2줄, B 3줄, C 4줄이다. 고정 fixture에서 네 topology × 30 track tail order × 4 cap (`20px`, `20%`, `0%`, `120%`) × 2 alignment (`start`, `stretch`) = 960 scene을 Chrome 157에서 관측했다.
- 독립 기대는 [CSS Grid 2025 CRD §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)의 intrinsic track 계산과 §10.5의 sizing 후 `align-content` 적용 순서다. Text source order를 바꿔도 semantic placement·used row tracks·frame은 같아야 한다. 각 item block frame 높이는 CSS row line 범위가 차지하는 used row size 합과 일치해야 한다. Chrome observed rows와 모든 item/payload frame은 fixed-input 교차 기준으로 별도 보존한다.
- 기준 sentinel은 `fit-content(20px) min-content max-content auto min-content max-content`, `align-content:start`다. 두 span-2 scene의 rows는 `[10,0,20,15]px`, A/B item heights는 `[20,35]px`다. span-2/3 pair는 rows `[10,0,22.5,17.5]px`, A/B/C heights `[22.5,40,40]px`다. 비교 허용치는 각 CSS geometry 수치 0.01px, line count/track count/span mapping은 exact다.

## 지원 및 거부 경계

지원 입력은 위 topology, text grammar, computed style, owner/revision, track-list multiset/order, cap, alignment뿐이다. 중복 child·누락 payload·anonymous text·inline/nested markup·replaced content·다른 writing mode·다른 font/text style·border/padding/gap·다른 track count 또는 container dimension은 그대로 거부한다.

Grid placement는 resolved numeric line과 span이 계획된 값과 정확히 맞아야 한다. `grid-column:auto`는 겹치는 row item을 다른 implicit column으로 자동 배치할 수 있으므로 거부한다. 미지원·모호한 `grid-row` shorthand, implicit-line 요구, 양·음수 line 변형, 1이 아닌 row span, span 4 이상, dense/column flow, 추가 implicit column, 숨김 item, 다중 Grid owner, stale style/layout revision도 publish 전에 fail-closed한다.

기존 810-scene repeated text corpus와 singleton/four-/six-member non-spanning tests는 회귀 대상이다. 이번 slice는 CSS Grid WPT 전체, 일반 author CSS admission, Unicode/font fallback, dynamic mutation, nested Grid, auto-repeat 결합, scroll/clip, C11.4/C11 전체 완료를 뜻하지 않는다.

## 구현 및 검증 관문

1. 이 계획 자체의 서로 다른 실패 관점 20개와 계획 PR 실제 diff 30개 검토가 완료되기 전에는 제품 코드를 수정하지 않는다.
2. `RootHtmlFitContentAutoRowsV1`의 새 topology 판별은 Grid owner, direct child 수/순서, explicit row, resolved row/column line 및 span, payload ancestry, computed text style, track grammar와 revision을 한 snapshot으로 확인해야 한다. 기존 auto-placement profile 순서를 변경하지 않는다.
3. Layout validator는 span 전체에 실제 text measure와 intrinsic row track algorithm을 사용한다. 각 scene의 row track sizing contribution order, track base/growth limit 및 final rows를 확인하고 text item frame이 정확히 span row들의 합을 차지하는지 비교한다. 고정 height나 line count를 구현 안에 주입해 통과시키지 않는다.
4. 반복 텍스트의 Blink 1/64 CSS px 정규화는 item 순서나 item y 차이로 row 수/크기를 추측하지 않는다. Span item이 겹치면 이 추정은 잘못된 track을 만든다. 필요한 경우 final row track trace와 resolved row span에서 track 경계를 계산하고, trace·span mapping이 없거나 모순이면 부분 frame을 publish하지 않는다.
5. Root HTML RuntimeSession matrix는 기존 Chrome 960-scene JSON과 비교한다. `gridTemplateRows`, semantic item/payload frame, line count, row-span/column placement 및 source-order invariance를 검사한다. 기존 810씬 repeated-text corpus, 127개 Grid track-sizing tests, text grammar boundary, unsupported input atomicity와 같은 session의 valid recovery도 regression으로 확인한다.
6. Negative matrix에는 auto column으로 인한 implicit column, wrong row start/end, reversed line pair, 1/4/oversized span, span count가 다른 topology, duplicate/missing item, inline/nested child, text/style 변형, track order/cap/alignment 변형, stale owner/revision, trace omission/mismatch를 넣는다. 각 거부 전후 output revision, frame/paint publication, 다음 유효 input recovery를 확인한다.
7. 영향을 받는 Layout/Style/Runtime Rust tests, 변경 crate Clippy `-D warnings`, `cargo fmt --all -- --check`를 실행한다. 전체 Style suite의 기존 C07.3 aspect-ratio 문자열 차이는 baseline branch에서 재현하기 전까지 미해결로 별도 보고한다. 검증하지 않은 suite·CI는 통과로 쓰지 않는다.
8. 동일 HTML source hash와 앱 hash를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 Root HTML route에서 실행한다. `layout=ready`, exact scene/item ids, draw/present와 frame/track evidence를 기록한다. OS screenshots는 visual smoke이며 PixelCopy/Metal readback이라 부르지 않는다. 실행 전에 boot 상태를 저장하고 이 작업에서만 만든 simulator·AVD와 Agent Browser를 확인 후 종료·제거하며 다른 데이터와 공유 시스템 image는 보존한다.
9. default product artifact와 기존 fixture-test 경계에 신규 test-only FFI·scene marker·payload가 들어가지 않는지 확인한다. 이 작업은 default Root HTML 동작이므로 opt-in fixture 성공 경로로 우회하지 않는다.
10. 새 구현/runtime 동작은 서로 다른 실패 관점 20개로, 구현 PR 전체 diff는 별도 30개로 검토한다. 구현 PR 본문에는 가능한 Android/iOS 실행 화면을 포함한다. Rebase and merge 후 C11.4/C11 체크 및 공식 `78/214`는 올리지 않고 STATUS·roadmap·Tailnet 미리보기를 merge main과 동기화한다.

## 완료 판정

이 bounded slice는 독립 CSS Grid 알고리즘 비교, 960 Chrome scene, runtime/layout track-stage 검증, 거부 atomicity와 same-session recovery, Android/iOS 기본 route, 변경 crate 검증, artifact 경계, 20개 구현 검토 및 PR 변경 검토가 모두 확인되면 완료다. 이는 일반 text Grid spanning, 전체 C11.4/C11, public CSS conformance 또는 공식 roadmap 완료 수 변경이 아니다.

## 구현 결과 · PR #1295

제한 구현은 2026-11-08T20:35:50Z Rebase and merge됐다. 최종 main merge SHA는 `40b416b4de4942c9251db0f4862626443310d092`이다. 구현은 이 계획의 여섯-member repeated `grid-auto-rows` list, 고정 direct text grammar, 지정된 row-span topology 및 40×220px Grid 경계만 지원한다.

Chrome 157의 960 scene 비교, RuntimeSession 960 scene·35 atomic rejection·same-session recovery, vertical-writing cascade 거부, 기존 810-scene 회귀와 Layout/Style/trace/Grid track sizing focused 검증을 마쳤다. 변경 crate Clippy와 rustfmt도 통과했다. Android API 37.2 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator는 같은 sentinel HTML SHA로 기본 Root HTML route의 layout-ready 화면 smoke를 기록했다. 이는 PixelCopy/Metal drawable readback이나 native 960-scene parity가 아니다. GitHub PR check은 보고되지 않았다. 전체 `spinon-layout --lib`은 506 passed와 기존 C15 HostDocument 수집 테스트 1 failed였고, clean baseline 비교는 하지 않았다.

실행 근거와 제한은 [implementation README](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-implementation-2026-11-09/README.md), [구현/runtime 검토 20개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-implementation-2026-11-09/implementation-adversarial-review-20.md), [PR diff 검토 30개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-implementation-2026-11-09/implementation-pr-diff-review-30.md)에 기록했다. 이 결과는 일반 CSS admission, WPT conformance, 일반 row-span 지원 또는 C11.4/C11 완료를 뜻하지 않으며 공식 `78/214`와 내부 계약 `0.1.0`은 그대로다.

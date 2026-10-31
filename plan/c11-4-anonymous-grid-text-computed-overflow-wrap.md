# C11.4 anonymous Grid text computed `overflow-wrap` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [CSS Text 3 §5.4](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#overflow-wrap-property)

**상태:** 제한 구현 후보. Chrome 156 Agent Browser 관찰과 Ahem font table 기반 독립 기대값에 맞춰 구현하고 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 9/9 scene을 통과했다. 기본/fixture archive 경계 및 새 구현/runtime 20관점 검토를 마쳤다. 구현 PR diff 30관점 검토는 PR 생성 후 수행한다. PR은 아직 생성하지 않았으며 C11.4/C11 및 공식 완료 `72/213`은 미완료 상태다.

## 문제와 목표

현재 anonymous direct-text Grid intrinsic adapter는 `overflow-wrap`을 `normal`로 고정해 Parley 0.12.0에 전달한다. Root HTML style adapter가 computed CSS property를 알고 있어도 이 fixture 경계는 `anywhere`가 제공하는 min-content break opportunity와 `break-word`의 min-content 예외를 보존하지 않는다.

목표는 같은 computed Grid-root style snapshot에서 `overflow-wrap:normal|anywhere|break-word`를 direct anonymous paragraph에 전달하고 CSS Text 3 §5.4 의미에 맞게 intrinsic min/max sizing과 제한된 definite-width wrapping을 확인하는 것이다. 구현은 기존 `c11-4-grid-track-sizing-fixture-test` 경로와 정확히 열거한 Ahem scenes에 한정한다.

## 독립 기대값과 Chrome 관찰

[사전 비교 evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-precomparison-2026-11-01/README.md)는 HTML fixture, Agent Browser Chrome 156 observation, screenshot, Ahem SHA와 독립 기대값을 묶는다. Ahem의 `X` advance는 UPEM 1000 중 1000 units이므로 10px 글자 네 개 `XXXX`의 max-content 폭은 40px이다.

CSS Text 3 §5.4에 따라 `anywhere`의 emergency break opportunities는 min-content에 반영되지만 `break-word`의 기회는 반영되지 않는다. 따라서 min-content column/auto row는 normal=`40/10px`, anywhere=`10/40px`, break-word=`40/10px`이다. max-content 세 값은 모두 `40/10px`이다. 별도의 fixed `10px` column은 normal의 한 줄 overflow와 anywhere/break-word의 네 줄 wrapping을 구분한다. 전체 아홉 장면의 expected columns, rows, line count는 [JSON](../spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-precomparison-2026-11-01/expected.json)에 고정했다.

## 고정 입력과 구현 경계

- 한 `display:grid` root와 anonymous direct-text source `XXXX`, Ahem 10px/400, LTR `horizontal-tb`, `white-space:normal`, `word-break:normal`, `line-height:10px`, gap/edge 0을 사용한다.
- computed `overflow-wrap` 값은 `normal|anywhere|break-word` 세 가지만 scene allowlist에서 허용한다. 한 값마다 `min-content`, `max-content`, fixed `10px` column을 적용해 아홉 scene을 만든다.
- 검증은 정확한 root NodeId, scene/text/track/property, computed serialization, typed style value 및 HostDocument/style revision을 함께 묶는다. Scene ID만 보고 computed value를 추정하지 않는다.
- `OverflowWrapMode`를 `ParagraphStyle`와 text layout snapshot에 보존하고 pinned Parley 0.12.0 `OverflowWrap`으로 매핑한다. `anywhere`는 min-content 측정의 break opportunities를 제공한다. `break-word`는 min-content 측정 때 `normal`과 같은 기회를 사용하되 일반 definite-width wrapping에는 `BreakWord`를 유지한다. 이 두 경로를 별도 regression으로 고정한다.
- app fixture validator는 property가 누락되거나 scene과 typed/computed 값이 다르거나 revision/owner/topology가 다르면 layout, trace, frame 또는 draw/present publication 전에 거부한다. 다른 white-space/word-break 및 미등록 scene 조합은 지원하지 않는다.
- 이 변경은 일반 author CSS admission, 임의 font/script, 언어별 line-breaking 확장, inline child/run, arbitrary `overflow-wrap` 상호작용, WPT 적합성, glyph/surface pixel correctness 또는 전체 C11.4/C11 완료를 의미하지 않는다. 내부 계약 0059 버전 `0.1.0`과 공식 완료 `72/213`은 유지한다.

## 구현 및 검증 순서

1. 계획 자체를 구현과 겹치지 않는 실패 관점 20개로 검토한다. 계획 PR diff는 별도의 독립 검토 30개를 거친다.
2. 계획 PR을 병합했고 같은 checkout에서 구현 branch를 진행한다. 이번 구현은 현재 checkout에서 순차적으로 수행하며 병렬 구현/worktree는 사용하지 않는다.
3–6. `OverflowWrapMode::{Normal, Anywhere, BreakWord}`를 `ParagraphStyle`과 snapshot에 연결하고, BreakWord min-content와 definite-width mapping을 분리했다. text-layout 단위 검사, nine-scene FFI/runtime validator 및 불일치/owner/revision/미등록 값 거부 경로를 구현했다.
7. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 신규 9 scene 각각 9/9를 통과했다. 최종 APK/설치본 및 iOS executable/설치본 hash는 [실행 evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-runtime-2026-11-01/README.md)에 연결했다. AVD는 software renderer이고 iOS는 Simulator이며 glyph paint나 compositor 이후 pixels의 증거가 아니다.
8. Android/iOS fixture app build, focused Rust/Node 검사 및 default/fixture FFI release archive 경계를 통과했다. Archive 검사의 hash와 한계는 [artifact evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-overflow-wrap-runtime-2026-11-01/artifact-boundary/README.md)에 기록했다. APK/IPA 전체 bundle exclusion으로 확대하지 않는다.
9. 구현 뒤 새 실패 관점 20개 검토와 구현 PR diff 30관점 검토를 각각 수행하고, 발견 사항은 수정 후 영향을 받은 검증을 반복한다.
10. PR 생성 후 화면 이미지를 첨부하고 Agent Browser로 PR의 이미지 링크와 내용이 표시되는지 확인한다. required checks와 최종 PR state를 조회해 Rebase and merge한 뒤 source/merge SHA를 기록한다. 실행하지 않은 check은 통과로 보고하지 않는다.
11. 병합 후 내부 계약 0059, C11.4 plan, `spec/STATUS.md`와 public roadmap을 동기화한다. C11.4/C11 checkboxes와 `72/213`은 계속 유지한다.

## 계획 실패 판정

- `anywhere` min-content가 10px이 아니거나 `break-word` min-content가 normal의 40px과 다르면 실패다.
- `break-word` definite 10px layout이 emergency wrap하지 않거나 normal control도 wrap하면 실패다.
- CSSOM track sizes, auto rows, text line count, typed `ParagraphStyle`, revision, item frame 또는 track trace 중 하나라도 독립값과 다르면 실패다.
- invalid style/owner/revision이 partial result를 남기거나 default/product artifact에 fixture code/payload를 넣으면 실패다.
- 이 slice의 통과를 general CSS admission, WPT, glyph paint, hardware device 또는 C11.4/C11 완료로 표현하면 실패다.

## 사전 검토

별도 [계획 실패 관점 20개](./c11-4-anonymous-grid-text-computed-overflow-wrap-plan-review-20.md)와 계획 변경분 [PR diff 검토 30개](./c11-4-anonymous-grid-text-computed-overflow-wrap-plan-pr-diff-review-30.md)를 구현 검토와 분리한다.

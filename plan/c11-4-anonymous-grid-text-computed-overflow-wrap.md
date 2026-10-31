# C11.4 anonymous Grid text computed `overflow-wrap` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [CSS Text 3 §5.4](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#overflow-wrap-property)

**상태:** 구현 전 계획 후보. Chrome 156 Agent Browser 관찰과 Ahem font table 기반 독립 기대값을 아홉 장면에 고정했다. C11.4/C11과 공식 완료 `72/213`은 미완료 상태다.

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
2. 계획 PR을 Rebase and merge한 뒤 같은 checkout에서 최신 `origin/main` 기준 구현 branch를 만든다. worktree와 병렬 구현은 사용하지 않는다.
3. `spinon-text-layout`에 `OverflowWrapMode::{Normal, Anywhere, BreakWord}`를 추가하고 validate/default/layout snapshot에 보존한다. `BreakWord`의 min-content pass는 CSS Text 3의 예외대로 normal soft-wrap opportunities를 사용하고, max-content/definite-width layout은 computed break-word를 보존한다.
4. fixed Ahem `XXXX` focused text-layout test에서 세 mode 각각의 intrinsic inline/block sizes와 fixed 10px definite-width line layout을 독립 기대값으로 확인한다. Snapshot이 computed mode와 intrinsic result의 소유 paragraph를 잃지 않는지 검사한다.
5. anonymous Grid runtime validator와 FFI source generator에 9개의 고유 scene을 추가한다. root property와 paragraph typed value는 동일한 current revision에서 읽고, `break-word` min-content는 `normal`과 같지만 fixed 10px layout은 four-line wrap임을 확인한다.
6. `overflow-wrap` serialization mismatch, missing property, scene/value mismatch, wrong owner, stale revision, wrong font/text/track, 다른 `word-break`/`white-space`, unknown scene을 각각 negative control로 거부한다. 실패 시 부분 frame/trace/draw가 없어야 한다.
7. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 신규 9 scene과 기존 anonymous-text regression을 실행한다. computed styles, text snapshot, intrinsic source, row/column tracks, frames, draw/present 및 installed/source artifact hashes를 기록한다. AVD software renderer와 iOS compositor 제출 전 drawable을 각각 분리해 적는다.
8. default/product와 `c11-4-grid-track-sizing-fixture-test` release archive를 비교해 새 scene marker/payload와 fixture ABI 경계를 검사한다. archive 결과를 APK/IPA 전체 제외로 확대하지 않는다. fmt, 관련 text-layout/style-to-layout/runtime/FFI focused tests, fixture boundary, scoped Clippy와 두 Simulator app builds를 실행한다.
9. 구현 뒤 계획 검토와 겹치지 않는 새 실패 관점 20개, 구현 PR diff의 별도 실패 관점 30개를 수행한다. 결함 수정은 영향받은 검토와 실행을 다시 확인한다.
10. 구현 PR에는 가능한 Chrome, Android, iOS screenshots를 첨부하고 raw image URL을 Agent Browser로 확인한다. required checks와 PR state를 조회해 Rebase and merge한 뒤 최종 source SHA를 확인한다. 실행하지 않은 check은 통과로 보고하지 않는다.
11. 병합 후 내부 계약 0059, C11.4 plan, `spec/STATUS.md`와 public roadmap에 제한 범위·증거·미완료 상태를 반영한다. C11.4/C11 checkboxes와 `72/213`을 유지한다.

## 계획 실패 판정

- `anywhere` min-content가 10px이 아니거나 `break-word` min-content가 normal의 40px과 다르면 실패다.
- `break-word` definite 10px layout이 emergency wrap하지 않거나 normal control도 wrap하면 실패다.
- CSSOM track sizes, auto rows, text line count, typed `ParagraphStyle`, revision, item frame 또는 track trace 중 하나라도 독립값과 다르면 실패다.
- invalid style/owner/revision이 partial result를 남기거나 default/product artifact에 fixture code/payload를 넣으면 실패다.
- 이 slice의 통과를 general CSS admission, WPT, glyph paint, hardware device 또는 C11.4/C11 완료로 표현하면 실패다.

## 사전 검토

별도 [계획 실패 관점 20개](./c11-4-anonymous-grid-text-computed-overflow-wrap-plan-review-20.md)와 계획 변경분 [PR diff 검토 30개](./c11-4-anonymous-grid-text-computed-overflow-wrap-plan-pr-diff-review-30.md)를 구현 검토와 분리한다.

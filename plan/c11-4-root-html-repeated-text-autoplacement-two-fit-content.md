# C11.4 · 반복 텍스트 자동 배치의 두 `fit-content()` 행

- **상태:** 계획 실패 관점 20개·계획 PR diff 30개 검토 완료 · Chrome 157 사전 비교 2,880/2,880 · 구현 전 · C11.4/C11 공식 체크 미완료 · `78/214` 유지
- **직전 자동 배치 slice:** [fit-content 위치 1–6](./c11-4-root-html-repeated-text-autoplacement-fit-content-positions.md) · 계획 PR #1301 · 구현 PR #1302
- **직전 두-fit row-span slice:** [row-span 두 `fit-content()`](./c11-4-root-html-repeated-text-row-span-two-fit-content.md) · 구현 PR #1311
- **Chrome 사전 비교:** [2,880 scene·독립 관계·범위](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-two-fit-content-precomparison-2026-11-10/README.md)
- **계획 실패 관점:** [독립 20개](./c11-4-root-html-repeated-text-autoplacement-two-fit-content-plan-review-20.md)
- **계획 PR 변경 검토:** [독립 30개](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-two-fit-content-precomparison-2026-11-10/plan-pr-diff-review-30.md)

## 결손과 목표

기본 parsed Root HTML의 `RootHtmlFitContentAutoRowsV1` repeated-text auto-placement 경로는 여섯-member `grid-auto-rows` list에서 finite non-negative direct px/% `fit-content()` 하나를 허용한다. 기존 위치 1–6 slice는 남은 member를 `min-content ×2`, `max-content ×2`, `auto ×1`로 제한한다. 구현 PR #1311은 두 fit track을 별도 row-span profile에 추가했지만 auto-placement 경로는 계속 single-fit profile에만 진입한다.

이번 bounded slice는 여덟 implicit text row를 만드는 기존 9-item auto-placement profile에서 fit member를 두 개로 늘린다. 여섯 authored 위치 중 서로 다른 두 위치 15쌍, 나머지 member `min-content ×2`, `max-content ×1`, `auto ×1`의 12개 고유 순서를 모두 다룬다. 두 cap은 각자 유한한 0 이상 direct px 또는 direct percentage만 허용한다. 대표 관측 cap pair는 독립 fixture를 구별하기 위한 입력이며 지원값 allowlist가 아니다.

Row-span topology와 `min-height:auto` floor 규칙은 변경하지 않는다. 일반 Grid track list, 일반 CSS admission 또는 C11.4/C11 지원 완료 선언을 추가하지 않는다.

## 고정 입력과 독립 비교 모델

- 기본 parsed Root HTML route를 사용한다. Grid는 definite `40×220px`, 한 개 `40px` explicit column, 한 개 `10px` explicit row, row/column gap 0, `grid-auto-flow:row`, LTR, visible overflow다.
- 직접 자식은 explicit row를 차지하는 빈 control item 하나와 auto-placement text item 여덟 개다. 각 text item은 제한 ASCII 대문자 text payload 하나를 갖는다. Item `min-height:0`, text는 `10px monospace`/`10px` line-height다. 고정 payload와 line 수 `[2,2,2,3,2,2,3,1]`는 fixture에 기록했다.
- `grid-auto-rows`는 정확히 여섯 member다: `fit-content(cap) ×2`, `min-content ×2`, `max-content ×1`, `auto ×1`. 두 fit 위치는 1–6에서 서로 다른 15쌍이다. 나머지 네 member의 12개 고유 순서를 전부 허용한다.
- Chrome 157 사전 비교는 cap assignment 8종, alignment `start|stretch`를 추가해 `15 × 8 × 12 × 2 = 2,880` scene을 실행했다. 이는 1,440개 authored list와 각각의 start/stretch pair다.
- Representative cap assignment는 `(0px,0%)`, `(10px,20px)`, `(10px,20%)`, `(20%,120%)` 및 각 pair의 역순이다. definite 220px block basis에서 직접 percentage는 각각 `0% = 0px`, `20% = 44px`, `120% = 264px`다. 유효한 다른 finite px/% cap도 implementation positive control로 쓴다.
- CSS Grid 기준은 [2025 CSS Grid CRD §7.2 track sizing functions](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing), [§7.6 implicit tracks](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-tracks), [§8 auto-placement algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo)다. 독립 관계는 explicit row 이후 implicit list가 반복되는 순서, 각 fit track의 authored 위치별 cap, item/track frame 대응, `stretch`의 auto-track-only 배분, visible overflow다.

사전 비교 결과 2,880 scene 모두에서 explicit row와 여덟 implicit row가 생성됐고 여섯-member CSSOM order가 유지됐다. Independent verifier는 fit row occurrence 7,680개와 alignment pair 1,440개를 확인했다. 모든 360 cap group에서 대표 cap 변경이 track geometry에 반영됐다. `stretch`는 auto occurrence에만 남는 positive space를 배분했고 non-auto 최대 변화는 0px였다. 288 scene은 Grid 아래로 text가 overflow됐고 최대 30px였다. 이 결과는 Chrome 기준 관측이며 Spinon runtime 근거가 아니다.

## 허용 및 거부 경계

- 여섯-member list에 유한하고 0 이상인 direct `fit-content(px|%)` 두 개를 서로 다른 위치 1–6에 허용한다. 나머지는 정확히 `min-content ×2`, `max-content ×1`, `auto ×1`이며 12개 후행 순서는 모두 허용한다.
- 기존 Root HTML grid geometry, 빈 explicit-row item 하나, auto-placement item 여덟 개, 고정 ASCII text grammar, font/line-height, 단일 column, `align-content:start|stretch` 경계를 유지한다.
- 목록 길이 또는 multiplicity 변형, fit track 1개 또는 3개 이상, 같은 authored 위치 중복, `calc()`·음수·비유한 cap, indefinite percentage basis, 잘못된 text/child 수, implicit column, dense/column auto-flow, nested/replaced/inline child, 추가 Grid owner, unsupported alignment, 누락·모순 track trace, stale owner/revision은 거부한다.
- 두-fit row-span 허용 조건과 topology는 그대로 유지하며 auto-placement과 혼합하지 않는다. 실패는 layout/render revision과 published frames를 부분 변경하지 않고 같은 RuntimeSession에서 유효 입력을 다시 처리한다.
- 일반 CSS admission, CSS Grid 전체 적합성, 임의 shaping, WPT, native matrix parity, surface pixel parity, 실기기/hardware GPU 또는 C11.4/C11 전체 완료를 뜻하지 않는다. 공개 API/FFI와 internal contract version 변경은 없다.

## 구현 및 검증 관문

1. 이 계획의 독립 실패 관점 20개와 계획 PR 전체 diff의 별도 30개 관점 검토를 통과한 뒤에만 제품 코드를 변경한다.
2. Style projection은 여섯 typed track, authored order, 두 typed cap과 position을 보존한다. Existing one-fit auto-placement와 two-fit row-span profile을 각각 독립적으로 허용하고 변형 list를 거부한다.
3. Layout admission은 기존 9-item direct-text auto-placement topology에서만 two-fit list를 허용한다. Explicit row를 auto-row cycle에 넣지 않고 여덟 implicit row의 track index 1–6, 1–2 반복을 보존한다. Trace·owner·revision 불일치는 fail-closed다.
4. RuntimeSession 기본 Root HTML 경로에서 Chrome 관측 2,880 scene을 2,880/2,880 비교한다. Authored/computed order, cap assignment, 9 item frame, 8 text line count와 payload frame, row cycle, overflow, 360 cap groups와 1,440 alignment pairs를 각각 검증한다.
5. 독립 positive control은 사전 비교 pair 밖의 유효 px/% cap, `0px`/`0%`의 typed distinction, 그리고 auto occurrence가 두 번 반복되는 위치 1·2를 포함한다. 이 profile은 `min-height:0`을 유지하므로 min-content floor 사례를 새로 허용하지 않는다. Negative matrix는 one/three fit, wrong tail multiplicity/order, invalid caps/basis, changed child/column/flow topology, missing/duplicate/out-of-order trace, stale owner/revision을 포함한다. Rejection atomicity와 same-session recovery를 확인한다.
6. 기존 single-fit auto-placement 위치 1–6 1,440 scene, two-fit row-span 3,840 scene, single-fit row-span 1–3 1,920 scene, row-span 위치 4–6 4,320 scene 및 810 text corpus를 회귀한다. 영향받은 Rust tests, scoped Clippy `-D warnings`, `cargo fmt --all -- --check`를 실행하고 기존 baseline failure와 변경 실패를 분리한다.
7. 동일 source hash의 Android API 37.2 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 기본 Root HTML route에서 대표 sentinel을 실행한다. Early `[1,2]`, adjacent `[2,5]`, separated `[1,6]` fit pair를 `start|stretch`와 zero/mixed/over-container cap assignment로 나눠 layout-ready와 draw/present를 확인한다. Screenshot은 visual smoke이며 PixelCopy/Metal drawable readback이 아니다.
8. Simulator, Agent Browser, app process의 시작 전 상태를 기록한다. 이번 검증에서 생성한 app install artifact와 실행 프로세스·simulator·Agent Browser session만 종료/삭제한다. 다음 반복 작업에 쓸 shared simulator profile은 보존한다.
9. 구현/runtime은 계획 리뷰와 다른 실패 관점 20개, 실제 implementation PR diff는 별도 30개로 검토한다. PR에는 검증된 Android/iOS screenshot을 immutable URL로 제공하고 merge 전에 Agent Browser로 로드를 확인한다.
10. Merge 뒤 `spec/STATUS.md`, 내부 계약 0059, 이 계획과 `spec/public/roadmap.html`을 동기화한다. Official `78/214`, C11.4/C11 미완료와 internal interface `0.1.0`은 유지한다. 사용자 지정 Tailnet preview는 게시한 source/hash를 재확인한 뒤 갱신한다.

## 완료 판정

2,880-scene RuntimeSession 비교, 독립 track/cap/cycle/alignment/frame 관계, cap floor와 percentage controls, atomic rejection/recovery, 기존 auto/span/text regression, focused Rust/Clippy/rustfmt, Android/iOS default-route sentinels, 별도 implementation/runtime 20-point review와 PR diff 30-point review, merge 후 공식 상태·명세·public roadmap 동기화를 마친 뒤에만 이 bounded slice를 구현 완료로 기록한다. Chrome 사전 비교만으로 구현을 완료 처리하지 않는다.

# C11.4 parsed Root HTML `align-content:safe|unsafe flex-start`

**상태:** 계획과 20개 실패 관점 검토 완료, 구현 전. Chrome 157 사전 비교는 6/6 scene·72/72 item-frame coordinates에서 독립 기대값과 정확히 일치했고 최대 오차 0 CSS px다. [사전 비교 근거](../spec/internal/evidence/c11-root-html-auto-row-safe-flex-start-precomparison-2026-11-07/README.md) · [계획 검토 20개](./c11-root-html-auto-row-safe-flex-start-plan-review-20.md). C11.4/C11과 공식 `78/214`는 미완료·변경 없음.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [선행 plain/safe/unsafe start](./c11-root-html-auto-row-safe-start.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 목적

기본 parsed Root HTML `RootHtmlAutoRowStretchV1` fixed profile에서 exact computed `align-content:safe flex-start`와 `unsafe flex-start`를 제한적으로 허용한다. CSS Box Alignment는 flex formatting context 밖의 `flex-start`를 `start`와 동등하게 정의한다. 이 후보는 해당 Grid profile에만 적용하며 일반 CSS 지원을 선언하지 않는다.

## 기준과 기대값

기준은 [CSS Box Alignment Level 3 §4.1 positional values](https://www.w3.org/TR/css-align-3/#positional-values), [§4.4 overflow alignment](https://www.w3.org/TR/css-align-3/#overflow-values) 및 고정 [Chrome 157 사전 비교](../spec/internal/evidence/c11-root-html-auto-row-safe-flex-start-precomparison-2026-11-07/README.md)다. 해당 W3C 문서는 Working Draft이며 `<overflow-position>`은 at-risk다.

고정 입력은 definite 40×40px content box, 40px explicit column, 10px explicit row, 두 direct-text implicit `auto` rows, row auto-flow, `column-gap:0`, `align-items:normal`, 40×10px fixed empty item 하나와 10px line-height direct-text `A`/`B`다. Existing track sizing과 row-gap profile은 유지한다.

| `row-gap` | `align-content` | Used rows (px) | Item y offsets (px) |
|---|---|---|---|
| `0px` | `start`, `safe flex-start`, `unsafe flex-start` | `[10,10,10]` | `[0,10,20]` |
| `calc(120% - 2px)` | `start`, `safe flex-start`, `unsafe flex-start` | `[10,10,10]` | `[0,56,112]` |

Positive scene에서는 rows가 container보다 10px 작다. Overflow scene의 두 46px gutters와 세 10px rows는 122px alignment subject를 만들며, start-side placement로 마지막 row는 root 아래로 넘친다. 세 alignment의 computed value는 각각 유지돼야 하고 item source order, rows와 gutter는 값에 따라 달라지면 안 된다.

## Admission과 실패 동작

- 해당 fixed profile에서 exact computed `safe flex-start`와 `unsafe flex-start`만 기존 `align-content` allowlist에 추가한다.
- computed string에 safety modifier와 `flex-start` keyword를 모두 보존한다.
- `start`, plain `flex-start`, safe/unsafe `start`, `center`, `end` 및 기존 row-gap 처리를 회귀시킨다.
- `safe flex-end`, baseline, distributed alignment에 붙인 safety modifier, unknown serialization 및 다른 item/track shape는 계속 거부한다.
- Style-to-layout과 layout allowlist는 일치해야 한다. rejection은 property `align-content`에 귀속되고 partial layout/snapshot을 publish하지 않아야 하며, 같은 RuntimeSession은 이어지는 valid request로 복구해야 한다.
- CSS parser/general admission, public API, FFI, row sizing, row-gap resolution, fixture ABI와 내부 계약 버전 `0.1.0`은 바꾸지 않는다.

## 검증 게이트

1. 고정 [Chrome 157 사전 비교](../spec/internal/evidence/c11-root-html-auto-row-safe-flex-start-precomparison-2026-11-07/README.md) 및 independent expectations를 사용한다. 6 scene과 1/64 CSS px tolerance를 유지한다.
2. Layout 및 style projection focused tests에서 positive/overflow geometry, computed string 보존, unchanged rows/gutters, rejection controls를 검증한다.
3. RuntimeSession에서 accepted values, exact property attribution, atomic rejection 및 same-session recovery를 확인한다.
4. Safe/unsafe start·center·end와 fixed/percentage/`calc()` row-gap regressions, scoped Clippy, `cargo fmt --all -- --check`를 실행한다.
5. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route에서 positive-space·overflow 장면을 실행한다. OS screenshot은 visual bounds evidence로만 분류하고 PixelCopy/Metal drawable readback이라 부르지 않는다.
6. App/build restoration hashes와 PR status rollup을 기록한다. Empty check rollup은 CI 성공이 아니다.
7. 구현/runtime 변경은 새로운 failure-oriented 20개로, 실제 PR diff는 별도 30개 관점으로 검토한다. PR 본문에는 플랫폼 화면을 올릴 수 있으면 함께 첨부한다.

## 남은 범위

Scroll-container safety, 다른 writing mode/direction, `safe|unsafe flex-end`, arbitrary track/item topology, distributed/baseline alignment, general CSS admission, WPT conformance, physical devices, hardware GPU, iOS compositor 이후 픽셀은 이 slice에서 다루지 않는다. C11.4/C11 완료 체크와 공식 `78/214`는 유지한다.

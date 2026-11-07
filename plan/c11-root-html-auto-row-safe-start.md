# C11.4 parsed Root HTML `align-content:safe|unsafe start`

**상태:** bounded 구현과 Android/iOS Simulator 검증을 마쳤고 [PR #1254](https://github.com/ohah/spinon/pull/1254)가 열려 있다. Chrome 157 Agent Browser 비교는 6/6 scene·72/72 item-frame coordinates가 독립 기대값과 정확히 일치하고 최대 오차 0 CSS px이다. Layout/style projection/RuntimeSession focused tests, safe-center/end와 row-gap regressions, scoped Clippy·rustfmt가 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route 세 장면 모두 layout-ready·draw/present와 screenshot bounds를 확인했다. GitHub checks/reviews는 보고되지 않아 CI green으로 기록하지 않는다. C11.4/C11 체크 및 공식 `78/214`는 유지한다. [사전 비교 근거](../spec/internal/evidence/c11-root-html-auto-row-safe-start-precomparison-2026-11-07/README.md) · [실행·화면 근거](../spec/internal/evidence/c11-root-html-auto-row-safe-start-runtime-2026-11-07/README.md) · [계획 실패 관점 검토 20개](./c11-root-html-auto-row-safe-start-plan-review-20.md) · [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-root-html-auto-row-safe-start-runtime-2026-11-07/implementation-review-20.md).

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [선행 `safe|unsafe center`](./c11-root-html-auto-row-safe-center.md) · [선행 `safe|unsafe end`](./c11-root-html-auto-row-safe-end.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 목적

기존 기본 parsed Root HTML `RootHtmlAutoRowStretchV1` fixed profile에서 exact computed `align-content:safe start`와 `unsafe start`를 제한적으로 허용한다. CSS Box Alignment에서 두 값은 start placement를 유지하며, 현재 지원된 plain `start`와 같은 Grid geometry를 낸다. 각 computed value는 style snapshot에서 보존되어야 한다.

규범 기대값은 [CSS Box Alignment Level 3 §4.4](https://www.w3.org/TR/css-align-3/#overflow-values) 및 [§5.1.4](https://www.w3.org/TR/css-align-3/#align-content-property)에 따른다. 참조는 Working Draft이고 `<overflow-position>`은 at-risk다. 따라서 이 작업은 선택한 Root HTML profile의 bounded implementation이며 공개 CSS 전반을 지원한다고 뜻하지 않는다.

## 고정 profile과 기대값

기존 profile을 유지한다: definite 40×40px content box, 40px explicit column, 10px explicit row, 두 implicit `auto` rows, row auto-flow, `column-gap:0`, `align-items:normal`, 40×10px fixed empty item 하나와 10px line-height direct-text item `A` 및 `B`.

| `row-gap` | `align-content` | Used rows (px) | Item y offsets (px) |
|---|---|---|---|
| `0px` | `start`, `safe start`, `unsafe start` | `[10,10,10]` | `[0,10,20]` |
| `calc(120% - 2px)` | `start`, `safe start`, `unsafe start` | `[10,10,10]` | `[0,56,112]` |

Overflow 입력에서 두 46px gutter가 세 10px tracks 사이에 있으므로 alignment subject는 122px이고 40px container보다 82px 크다. Start 계열은 subject 시작점을 container 시작점에 두며, item geometry는 safe/unsafe flag에 따라 달라지지 않아야 한다. Row sizes와 gutter sizes 역시 alignment 때문에 달라지면 안 된다.

## Admission and failure behavior

- 이 fixed profile에서 exact computed `safe start` 및 `unsafe start`만 기존 content-alignment allowlist에 추가한다.
- style projection과 layout profile allowlist가 일치하고 computed value를 정확히 보존하도록 한다.
- `start`, 기존 `safe|unsafe center`, 기존 `safe|unsafe end` 동작을 유지한다.
- 다른 item/track shape, baseline, distributed safety modifier, unknown serialization 및 현재 profile 바깥의 CSS admission은 계속 거부한다.
- unsupported alignment 오류는 `align-content`로 귀속되어야 한다. 실패 시 partial layout/snapshot을 publish하지 않고 같은 RuntimeSession에서 다음 valid value가 성공해야 한다.
- parser, 일반 CSS admission, public API, FFI, row sizing, row-gap 계산, fixture ABI와 내부 계약 버전을 바꾸지 않는다.

## 검증 게이트

1. 고정 [Chrome 157 사전 비교](../spec/internal/evidence/c11-root-html-auto-row-safe-start-precomparison-2026-11-07/README.md)와 1/64 CSS px tolerance를 사용한다.
2. Layout 및 style projection focused tests로 positive free-space, overflow, exact computed value, unchanged rows/gutters, rejected controls를 검증한다.
3. RuntimeSession accepted values, unsupported atomic rejection, error-property attribution과 same-session recovery를 검증한다.
4. 기존 safe/unsafe center·end와 fixed/percentage/`calc()` row-gap regressions를 유지하고 changed-crate Clippy와 rustfmt를 실행한다.
5. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 Root HTML route를 실행한다. OS screenshot은 visual bounds evidence로만 분류하며 PixelCopy/Metal drawable readback이라고 부르지 않는다.
6. App/build restoration hashes를 기록하고 PR status rollup을 있는 그대로 분류한다. Empty rollup은 CI 통과가 아니다.
7. 구현/runtime 변경을 fresh failure-oriented 20개로 검토하고, PR 변경분은 별도 30개 관점으로 검토한다.

## 남은 범위

Scroll-container safety, writing mode·direction, arbitrary track/item topology, distributed/baseline alignment, general CSS admission, WPT conformance, physical devices, hardware GPU 및 iOS compositor 이후 pixels는 제외한다. C11.4/C11 체크는 미완료로 유지하고 공식 전체 `78/214`를 바꾸지 않는다.

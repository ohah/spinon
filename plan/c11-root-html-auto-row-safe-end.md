# C11.4 parsed Root HTML `align-content:safe|unsafe end`

**상태:** bounded 구현 및 Android/iOS Simulator 검증 완료, PR 등록 전. Agent Browser HeadlessChrome 157 사전 비교 8/8 scene·96/96 item-frame 좌표가 독립 기대값과 정확히 일치했다. Layout, style projection, RuntimeSession focused tests, 기존 safe/unsafe center·calc row-gap 회귀, changed-crate Clippy와 rustfmt가 통과했다. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 기본 Root HTML 경로에서 positive·safe overflow·unsafe overflow 장면을 확인했다. Screenshot은 시각 경계 확인이며 PixelCopy/Metal drawable readback이 아니다. [사전 비교](../spec/internal/evidence/c11-root-html-auto-row-safe-end-precomparison-2026-11-07/README.md) · [실행 근거](../spec/internal/evidence/c11-root-html-auto-row-safe-end-runtime-2026-11-07/README.md) · [구현/runtime 실패 검토 20개](../spec/internal/evidence/c11-root-html-auto-row-safe-end-runtime-2026-11-07/implementation-review-20.md).

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 intrinsic track sizing](./c11-4-grid-track-sizing.md) · [선행 safe/unsafe center 제한 구현](./c11-root-html-auto-row-safe-center.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 목적

앞 단계에서 병합한 fixed `RootHtmlAutoRowStretchV1`의 `align-content:safe|unsafe center`에 `safe end`와 `unsafe end`를 다음 순차 제한 값으로 추가한다. Positive free-space에서는 두 값 모두 end alignment를 적용한다. Alignment subject가 container를 넘으면 `safe end`는 start로 fallback하고 `unsafe end`는 요청된 end alignment를 유지한다.

규범 기대값은 [CSS Box Alignment Level 3 §4.4](https://www.w3.org/TR/css-align-3/#overflow-values) 및 [§5.1.4](https://www.w3.org/TR/css-align-3/#align-content-property)를 따른다. 참조 문서는 Working Draft이며 `<overflow-position>`을 at-risk로 표시한다. 따라서 제한 profile 구현은 전체 CSS 지원 또는 안정된 공개 계약이 아니다.

## 고정 profile과 기대값

기존 profile을 유지한다: definite 40×40px content box, 40px explicit column, 10px explicit row, 두 implicit `auto` rows, row auto-flow, `column-gap:0`, `align-items:normal`, 40×10px fixed empty item 하나와 10px line-height direct-text item `A` 및 `B`. Track sizing과 row-gap 해석은 이전 `#1249` profile 그대로다.

| Row gap | `align-content` | Rows | Item y offsets (px) |
|---|---|---|---|
| `0px` | `safe end`, `unsafe end`, `end` | `[10,10,10]` | `[10,20,30]` |
| `calc(120% - 2px)` | `safe end` | `[10,10,10]` | `[0,56,112]` |
| `calc(120% - 2px)` | `unsafe end`, `end` | `[10,10,10]` | `[-82,-26,30]` |
| `calc(120% - 2px)` | `safe center`, `unsafe center` regression controls | `[10,10,10]` | `[0,56,112]`, `[-41,15,71]` |

Positive 정렬 subject는 30px라 40px container에 10px free space가 남는다. Overflow subject는 10px track 세 개와 46px gutter 두 개를 포함해 122px이고 free space는 -82px다. Safe 정렬은 start로 fallback하고 unsafe end는 subject 끝을 container 끝에 맞춘다. 정렬값에 따라 track과 gutter 크기는 바뀌지 않아야 한다.

## Admission and failure behavior

- Add only exact computed `safe end` and `unsafe end` to the existing content-alignment allowlist for this fixed profile.
- Keep existing `end`, `safe center` and `unsafe center` behavior intact.
- Continue rejecting `safe start`, `unsafe start`, safety modifiers on distributed values, baseline values, unknown serialization and other track/item shapes.
- Style projection and layout profile must agree on the exact allowlist. Unsupported input fails before partial scene publication; the same RuntimeSession must recover when a supported value is submitted next.
- Do not change general CSS parsing/admission, public APIs, WPT status, layout contract version, track sizing, row-gap resolution or fixture ABI.

## 검증 게이트

1. Use the pinned [Chrome 157 oracle and independent expected values](../spec/internal/evidence/c11-root-html-auto-row-safe-end-precomparison-2026-11-07/README.md); keep the 8-scene matrix and 1/64 CSS px comparison tolerance.
2. Add focused style projection and layout tests for positive free-space, safe overflow fallback, unsafe overflow end placement, unchanged track/gutter geometry and unsupported-value rejection.
3. Add RuntimeSession success/error/same-session recovery assertions and preserve existing `safe|unsafe center`, fixed/percentage/`calc()` row-gap regressions.
4. Run scoped Clippy and rustfmt for changed crates.
5. Run the default Root HTML app route on Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator. Record layout trace and screenshot bounds; classify screenshots as visual evidence only, not PixelCopy/Metal drawable readback.
6. Record app/build restoration hashes and retain the PR check rollup as reported; an empty rollup is not CI green.

## 남은 범위

다른 writing mode·direction, scroll-container safety/default behavior, content positions beyond the selected values, distributed and baseline alignment, arbitrary track/item topology, general CSS admission, WPT conformance, physical devices, hardware GPU and iOS compositor-after pixels remain outside this slice. C11.4/C11 stays incomplete and the official overall `78/214` counter is unchanged.

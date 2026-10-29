# C11.4 · `fit-content()` 성장 한계 경계 WPT 계획

**상태:** 고정 WPT의 세 제한 경계 assertion과 Chrome baseline을 확인했다. opt-in app runtime 구현은 시작하지 않았다. C11.4와 공식 완료 `72/213`은 미완료로 유지한다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목표

Pinned WPT `css/css-grid/grid-definition/fit-content-track-sizing.html`의 `contentLargerThanArgument`, `smallArgument`, `largeArgument` assertions를 같은 C11.4 intrinsic track-sizing runtime 경로에서 검증한다. 세 장면 모두 direct grid item에 `min-height:0`, 100px child를 두고 `fit-content()` argument만 40px, 30px, 200px으로 바꾼다. 이로써 argument가 max-content보다 작은 경우, 더 작은 cap, max-content보다 큰 경우를 연속해서 확인한다.

이번 작업은 세 고정 assertion의 test-only runtime fixture다. 일반 CSS author admission, arbitrary Grid tree, zoom, percentage/calc argument, automatic minimum, span, text/image shaping, 전체 WPT harness와 C11.4 완료를 추가하지 않는다. 규범값과 구현 결과가 다르면 oracle 숫자를 바꾸지 않고 computed style·typed track·contribution trace·Taffy sizing stage에서 원인을 찾는다.

## 고정 upstream과 Chrome baseline

- WPT revision: `c999c58338ee1d223df5ad62e48be1202ced0d37`.
- Source: [`fit-content-track-sizing.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html), SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`, 5,060 bytes.
- WPT assertions are the 4th–6th of the 13-test source file. The live `wpt.live` source returned the same SHA-256; Agent Browser HeadlessChrome 156 showed `13 Pass`, 0 failed.
- Original WPT `.grid` has auto width. At a 1280px browser viewport its used width was 1264px. A temporary width-60px normalization preserved each target row (`40px`, `30px`, `100px`) while making the runtime root and single explicit column deterministic at 60px. It changed no source assertion or content.
- The before/after observations, source hash, expected values and Agent Browser screenshot are in [precomparison evidence](../spec/internal/evidence/c11-4-fit-content-limit-boundaries-precomparison-2026-10-29/README.md). Chrome results are an upstream oracle, not a Spinon WPT pass.

## 독립 기대값

CSS Grid Level 1 2025 CRD §11.2 defines a `fit-content(limit)` track using `max(minimum, min(limit, max-content))`; §11.5.1 clamps the max-content growth limit by the fit-content argument. The target items specify `min-height:0`, removing the automatic minimum floor for these cases. Their 100px block children yield a 100px max-content contribution. Thus `minimum=0px`, `max-content=100px` in all three cases:

| WPT id | `grid-auto-rows` | 계산 | expected row / item | child | expected overflow beyond item |
| --- | --- | --- | ---: | ---: | ---: |
| `contentLargerThanArgument` | `fit-content(40px)` | `max(0, min(40, 100))` | 40px | 100px | 60px |
| `smallArgument` | `fit-content(30px)` | `max(0, min(30, 100))` | 30px | 100px | 70px |
| `largeArgument` | `fit-content(200px)` | `max(0, min(200, 100))` | 100px | 100px | 0px |

The root remains 60×300px, with one explicit 60px column, `grid-template-rows:none`, one implicit auto row, zero gap, `align-content:normal`, and visible overflow. The direct item remains auto-sized except for `min-height:0`; its only child is 100px high. Runtime row sizing and item/child frames must match the expected table in the same layout revision.

## 구현 경계와 검증

1. Use three exact opt-in scene IDs: `fit-content-limit-40-wpt`, `fit-content-limit-30-wpt`, and `fit-content-limit-200-wpt`. The fixture accepts only its exact root ID, tree, values and `c11-4-grid-track-sizing-fixture-test` feature. No product CSS allowlist is widened.
2. Assert the projected `GridTemplateRows:none`, `GridAutoRows:fit-content(<exact px>)`, item `min-height:0`, computed overflow, typed Taffy input, minimum contribution 0px, max-content contribution 100px, used row and all node frames. Compare layout/document/style/environment revision identity through draw/present.
3. Capture root background after the child, yellow child paint within the row, and yellow visible overflow past the 40px and 30px item edges. For the 200px argument, verify the child fits the 100px row. Preserve root-relative sample coordinates and reject samples outside the viewport or stale revision before readback.
4. Run each of the three cases on Android API 37 ARM64 software AVD PixelCopy and iPhone 17 Pro/iOS 26.2 Simulator Metal drawable. Record compositor boundary; simulator results do not imply physical-device, hardware-GPU or post-compositor parity.
5. Check default-vs-opt-in static archive fixture isolation, source-format/lint gates and focused C11.4 regressions. A baseline artifact omission claim is limited to the inspected static archive.
6. After implementation, perform a new 20-view implementation/runtime failure review and a separate 20-view PR diff review. Update internal contract 0059, the parent plan, STATUS evidence narrative and public/Tailnet roadmap while keeping C11.4 unchecked and `72/213` unchanged. Verify the Tailnet rendered hashes and close the Agent Browser session/process.

## 남은 범위

This slice does not run the WPT harness inside Spinon, implement all 13 source assertions as one route, open general CSS `fit-content()` or `zoom`, close all track sizing combinations, or complete C11.4/C11. Android physical hardware, hardware GPU, iOS compositor-after pixels and full matrix remain separate gates.

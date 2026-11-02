# C11.1 parsed Root HTML Grid percentage gaps

## 목표

기본 parsed Root HTML Grid 경로에서 computed `row-gap`과 `column-gap`의 유한한 비음수 percentage 값을 실제 Grid sizing에 연결한다. 기존 `normal`과 non-negative px gap을 유지하고, 0% 및 100% 초과도 clamp하지 않는다. 이 제한 구현은 C11.1/C11 완료, 일반 CSS author admission 또는 WPT 적합성으로 승격하지 않는다.

## 현재 구현 경계

- Cascade snapshot과 `LayoutGap`은 percentage spacing을 표현할 수 있고 Taffy adapter도 percentage gap을 전달하지만 Root HTML Grid admission은 `normal` 및 finite non-negative px만 허용한다.
- Root HTML percentage track의 extent 검사는 gap을 fixed px로만 계산한다. Sparse column-flow의 relative implicit column 계산도 fixed-px gap만 받는다.
- Fixed 2×2 named-area route는 gap 0만 허용한다. 이 계획에서는 기존 named-area track map·placement를 보존하며 px/percentage gap을 추가한다.
- 기존 typed math, indefinite basis, negative/non-finite computed value, `auto` 및 unsupported spacing은 계속 fail-closed한다.

## 기준 구현과 독립 기대값

- CSS Gaps Level 1 §2.3은 gap percentage가 해당 축 요소 content box의 corresponding size를 기준으로 해석된다고 규정한다. Grid의 cyclic percentage는 intrinsic contribution 계산 중 0으로 해석되지만 contents layout에서는 content box 기준으로 해석된다. 이 slice는 cyclic/indefinite 입력을 받지 않고 definite px basis에서의 final layout만 지원한다.
- Agent Browser HeadlessChrome 156.0.0.0의 고정 입력과 원 관측값은 [사전 비교 evidence](../spec/internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/README.md)에 보존한다. 입력 HTML SHA-256은 evidence에 기록한다.
- Independent expected 값은 CSS Gaps 규칙 및 CSS Grid track sizing으로 산출한다. Chrome 관측은 별도 값으로 기록하고 계산값을 대체하지 않는다.

| 장면 | 입력 | 독립 기대값 |
|---|---|---|
| `basis` | 200×100px, columns `40px 1fr`, rows `20px 1fr`, 두 gap 10% | column gap 20px, row gap 10px, columns `[40,140]`, rows `[20,70]`; item 원점 `(0,0)`, `(60,0)`, `(0,30)`, `(60,30)` |
| `zero` | 120×60px, `gap:0% 0%` | 두 gap 0px, 기존 track/frame 유지 |
| `zero-basis` | width 0px, two 0px columns, `column-gap:75%` | definite zero content basis resolves the gap to 0px; both zero-width item origins remain x=0 |
| `shorthand` | 200×100px, `gap:5% 10%`, columns `30px 1fr`, rows `10px 1fr` | row gap 5px, column gap 20px, columns `[30,150]`, rows `[10,85]` |
| `legacy-alias` | Same geometry via `grid-gap:5% 10%` | Legacy alias computes to the same row/column gaps and item frames as `shorthand` |
| `over-100` | 100px width, two 10px columns, `column-gap:120%` | gap 120px, second item x=130px, overflow retained |
| `implicit-row` | 100px height, 20px explicit row, `grid-auto-rows:25%`, `row-gap:10%` | implicit row 25px, gap 10px, second row y=30px |
| `column-flow` | 200px width, one 40px explicit column, two 20px rows, `grid-auto-columns:50%`, `column-gap:10%` | implicit column 100px, gap 20px, next column x=60px |
| `named-area` | 120×70px fixed 2×2 named-area map, both gaps 10% | column/row gaps 12/7px; lower row y=27px, second column x=52px; spanning header includes the column gutter |
| `named-area-px` | Same fixed area map with 6px column / 4px row gap | Existing px syntax is admitted consistently; lower row y=24px, second column x=46px, header width 126px |
| `invalid-negative` | valid 5px declaration followed by invalid `gap:-10px` | CSS cascade keeps 5px; second item x=15px |
| `math-control` | 200px width, `column-gap:calc(5% + 2px)` | Chrome computes used gap 12px; Spinon Root HTML Grid still rejects typed math with the exact gap property and publishes no partial layout |

All positive geometry uses content-box dimensions with no padding/border, LTR horizontal-tb, finite definite root width and height (including a zero basis), supported Root HTML child types, and the existing supported track/placement grammar. The legacy `grid-gap` alias must flow through computed `row-gap`/`column-gap` without a separate runtime code path. Percent values are not capped at 100%. A resolved gap or aggregate track extent outside finite f32 CSS-pixel range is rejected before output publication.

## 구현과 검증

1. Root HTML gap admission accepts `Normal`, finite non-negative px, and finite non-negative percentage only when the corresponding grid content dimension is definite finite non-negative CSS px.
2. Percentage gap extent validation uses the axis content-size basis, multiplies by the number of gutters, combines it with supported explicit/implicit track extents where the current adapter already validates those extents, and rejects overflow before publishing frames.
3. Existing Root HTML base Grid, sparse row/column placement, relative implicit tracks, and fixed named-area route preserve their current value and ownership gates. Percentage gap support must not leak into Flex or unrelated profiles.
4. Rust tests assert computed spacing, gap axis/basis, shorthand order, exact track/item frames, >100% overflow, finite-range rejection, named-area spans, implicit-row/column behavior, and no partial `StyleLayoutOutput` on rejection. Existing px, `normal`, track, stale revision, placement, and area regression tests remain active.
5. Run the existing app's parsed Root HTML product route on Android and iOS simulators using the same fixture hash. Record build/artifact hashes, `layout=ready`, node frames/draw submission, screenshots, and any visible overflow. Do not call screenshots compositor pixel equality or physical-device evidence.
6. No fixture FFI, task-specific scene ID, or new Cargo feature is added. Compare the default product artifacts for new task-specific symbols/payload (expected absent) only if the existing runtime capture path uses fixture adapters.
7. Keep internal contract `0059` at numeric version `0.1.0`. Update `spec/STATUS.md`, `plan/c11-css-grid.md`, public roadmap copy, and the Tailnet preview only with verified scope and merge state. C11.1/C11 remain unchecked.

## 제외 범위

- Cyclic/indefinite percentage gap resolution, intrinsic contributions, arbitrary min/max sizing, percentage padding/border, vertical writing modes, RTL, fragmentation, alignment-added spacing, arbitrary Grid grammar, nested Grid and general CSS admission.
- `calc()`/typed math gaps, invalid computed values, Flex percentage-gap behavior, app CSS fetch/loader changes, WPT runner/conformance, hardware GPU, physical-device results and iOS post-compositor pixel equality.
- C11.1, C11.3, C11.4 or C11 completion and any change to official completion count.

## 완료 기준

- 계획은 구현 전에 별도 20개 실패 관점으로 검토하고, 구현/runtime 검토와 PR delta 검토에 재사용하지 않는다. 현재 결과: [계획 적대 검토 20개](./c11-root-html-grid-percentage-gap-plan-review-20.md).
- Chrome 156 observations and independent expected values are hash-bound before implementation.
- Focused cascade, projection, layout, runtime, rejection/overflow tests pass; scoped Clippy, rustfmt and `git diff --check` pass.
- Android and iOS simulator runs use the same fixture and implementation commit, and both show the positive and overflow cases without claiming surface readback. [Simulator result, artifact hashes, logs and screenshots](../spec/internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/README.md).
- Implementation/runtime failure review uses 20 distinct failure paths; [the review is complete](../spec/internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/implementation-runtime-review-20.md). The final PR delta has a separate [30-point change-specific review](../spec/internal/evidence/c11-root-html-grid-percent-gap-2026-11-03/pr-diff-review-30.md).
- The bounded behavior is documented in [internal Grid contract 0059](../spec/internal/0059-c11-grid-layout.md). The official status ledger remains unchecked and the count remains `72/213`; C11.1/C11 are incomplete.

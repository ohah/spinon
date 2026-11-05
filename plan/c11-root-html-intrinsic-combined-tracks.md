# C11.4 parsed Root HTML intrinsic explicit tracks + direct-text implicit rows

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML explicit intrinsic tracks](./c11-root-html-intrinsic-grid-tracks.md) · [Root HTML direct-text `grid-auto-rows:min-content`](./c11-root-html-intrinsic-auto-rows-text.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 기준:** [Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-root-html-intrinsic-combined-tracks-precomparison-2026-11-06/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-combined-tracks-precomparison.html) · [독립 기대값](../spec/internal/evidence/c11-root-html-intrinsic-combined-tracks-precomparison-2026-11-06/expected.json) · [Chrome 관측](../spec/internal/evidence/c11-root-html-intrinsic-combined-tracks-precomparison-2026-11-06/chrome156-observation.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-intrinsic-combined-tracks-plan-review-20.md). **계획 PR 변경분 적대 검토:** 구현 PR과 분리해 30개 관점으로 기록한다. **구현/runtime 검토:** 구현 전 검토와 겹치지 않는 20개 관점으로 기록한다. **구현 PR 변경 검토:** 별도 30개 관점으로 기록한다.

## 결손과 목표

기본 parsed Root HTML route는 `RootHtmlIntrinsicTrackSizingV1`의 explicit intrinsic tracks와 direct text가 기여하는 `grid-auto-rows:min-content`를 각각 처리한다. 현재 `crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs`와 `crates/spinon-layout/src/grid.rs`의 admission gate는 두 축의 조합에서 하나를 거부한다. 그 결과 열의 intrinsic sizing 결과가 text item의 inline size를 정하고, 그 inline size가 다음 implicit row의 min-content block contribution을 바꾸는 CSS Grid sizing 순서를 같은 기본 runtime scene에서 검증할 수 없다.

이번 slice는 기존 explicit intrinsic track admission과 기존 direct-text implicit min-content row profile을 같은 `RuntimeRootHtmlTextPaintV1` / `RootHtmlIntrinsicTrackSizingV1` 계산에 결합한다. 새로운 track grammar, runtime profile, FFI, CSS parser, public API 또는 fixture-only 성공 route를 추가하지 않는다. C11.4와 C11 parent 상태 및 공식 완료 집계는 올리지 않는다.

## 비교 기준과 독립 기대값

기준은 [CSS Grid Layout Level 1, 2025-03-26 CRD §7.6, §11.1, §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)와 [Chrome 156 Agent Browser observation](../spec/internal/evidence/c11-root-html-intrinsic-combined-tracks-precomparison-2026-11-06/chrome156-observation.json)이다. §11.1은 먼저 열을 계산하고, 그 결과를 사용해 행 sizing 시 inline-axis available space를 구하도록 한다. §11.5는 `min-content` track의 base를 해당 track에 놓인 item들의 min-content contribution으로 정한다. §7.6의 `grid-auto-rows`는 explicit row 이후의 implicit rows를 정한다.

Fixture는 definite `40×40px` Grid, `10px` explicit row, direct text `A A A A A A`, 10px/10px `monospace` line metrics 및 고정 높이 빈 sibling을 사용한다. Chrome에서 별도로 잰 text min-content/max-content width는 각각 `6.03125px`/`66.234375px`다. Min-content column에서는 여섯 줄이 되어 implicit row contribution이 `60px`; max-content column에서는 한 줄 `10px`다. Independent expected JSON은 모든 target frame과 rows를 기록한다. Chrome 값은 교차 관측이며 OS monospace font metric이 Android/iOS와 같다고 가정하지 않는다.

| Scene | explicit columns | implicit rows | text line count | expected columns / rows |
| --- | --- | --- | ---: | --- |
| `min-content` | `min-content` | `min-content` | 6 | `[6.03125]` / `[10,60,34]px` |
| `max-content` | `max-content` | `min-content` | 1 | `[66.234375]` / `[10,10,34]px` |
| `auto-track` | `auto` | `min-content` | 2 | `[40]` / `[10,20,34]px` |
| `minmax-flex` | `minmax(min-content,1fr)` | `min-content` | 2 | `[40]` / `[10,20,34]px` |
| `gap` | `min-content` | `min-content`, row gap `5%` | 6 | `[6.03125]` / `[10,60,34]px`; used gaps `2px` |

`rows-control`은 fixed `40px` column에 기존 min-content implicit rows만 쓰는 대조군이다. 이 matrix는 intrinsic column 결과와 implicit text row의 상호 의존성을 검증하며 WPT 적합성이나 글리프 픽셀 동등성을 주장하지 않는다.

## Admission 경계

- 기본 Root HTML runtime의 하나의 visible Grid owner, LTR `horizontal-tb`, `grid-auto-flow:row`, sparse auto placement, positive definite CSS-px width/height와 static positioning만 허용한다.
- `grid-template-rows`는 유한한 양수 fixed-px track 하나다. `grid-auto-rows`는 반복되는 단일 `min-content` track이다. `grid-auto-columns`는 초기값 `auto`다.
- `grid-template-columns`는 기존 `RootHtmlIntrinsicTrackSizingV1` admission이 이미 허용하는 explicit track list만 사용한다. 이번 변경은 그 grammar를 확장하지 않는다. `fit-content()` 및 해당 profile에서 거부하는 unsupported/invalid values는 계속 거부한다.
- Gap은 기존 Root HTML intrinsic explicit-track route가 이미 허용하는 유효한 direct px/percentage 값만 쓴다. Percentage basis는 definite container에서 검증한다. math-backed gap을 새로 허용하지 않는다.
- Visible direct HTML `<div>`만 둔다. Empty fixed-size sibling과 text child 하나만 가진 direct block `<div>` 한 개를 사용한다. 기존 text item의 computed font, line-height, white-space, text transform, writing mode, direction 및 revision 검증을 유지한다.
- Root/item box edges, border, min/max, aspect ratio, named areas/lines, auto-repeat, implicit columns, dense/column flow, explicit placement, hidden/detached/stale node, inline descendant, anonymous Grid text, replaced item 및 multiple text-bearing items는 이번 scope 밖이다.
- Owner, node identity, source order, style/layout revision이나 text measurement가 어긋나면 layout/frame/paint 전체를 거부한다. Text contribution을 fixed estimate로 대체하거나 기존 default path의 다른 mode 선택 순서를 바꾸지 않는다.

## 구현과 검증 요구

1. 구현 전에 계획 실패 관점 20개를 이 문서와 별도 파일에 기록하고 계획 PR diff를 별도 30개 관점으로 검토한다.
2. Projection admission은 기존 explicit-intrinsic validator와 intrinsic-auto-row validator의 조합만 추가로 허용한다. 현재 track validation, computed typed values, owner/root/revision identity 및 unsupported diagnostic을 유지한다. `style_values.rs`의 fixed-only auto-row guard가 허용 범위를 정확히 교차시킨다.
3. `spinon-layout` preflight는 기존 intrinsic explicit track list, 단일 fixed explicit row, min-content auto-row pattern, gap subset을 일치시킨다. Column/row sizing trace의 최종 stage와 gutter를 모두 검사한다. Taffy intrinsic sizing 값을 fixed estimate로 바꾸지 않는다.
4. Rust layout tests는 pinned Ahem text measurer로 min-content/max-content/auto/flexible representative columns와 implicit rows의 row/column tracks, item frames, line counts 및 visible overflow를 독립 기대값과 비교한다. Unsupported fit-content, implicit column, track-list mismatch, gap outside the current typed allowlist, resource cap, invalid revision과 partial output을 negative controls로 둔다.
5. Default Root HTML `RuntimeSession` tests는 고정 fixture hash에서 Chrome matrix의 scene geometry를 검사한다. OS system `monospace` advance는 Chrome과 동일하다고 단정하지 않으며 실제 text-line count, track trace, element frames를 같은 runtime font path로 확인한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 default app route를 동일 source SHA로 실행한다. `layout=ready`, root/item node frames 또는 trace, draw/present 및 screenshots를 기록한다. PixelCopy/Metal readback, compositor timing, 실기기와 hardware GPU는 실제 수행했을 때만 별도로 표시한다.
7. Focused tests, `cargo fmt --all -- --check`, changed crate scoped Clippy, default artifact/fixture boundary, Android/iOS simulator build/runtime을 각각 기록한다. 기존 broad failures와 absent GitHub check rollup은 기준과 함께 보존한다.
8. 구현/runtime 변경 검토 20개는 계획 검토와 겹치지 않게 작성한다. Implementation PR의 actual diff review는 별도 30개 관점으로 수행하고, 발견 결함은 수정 뒤 영향을 받는 test/evidence/review를 다시 확인한다.
9. Merge 후 내부 계약 0059와 `spec/STATUS.md`에 제한 slice와 hash/validation limits만 기록한다. Tailnet preview는 main merge SHA와 served SHA가 일치하도록 갱신하고 Agent Browser desktop/mobile 상태를 확인한다. C11.4/C11 checkboxes와 공식 완료 집계 `78/214`는 유지한다. Agent Browser와 그 browser process는 작업 후 종료한다.

## 구현 병합 기록 · 2026-11-06

제한 구현 PR [#1195](https://github.com/ohah/spinon/pull/1195)이 2026-11-05T21:25:28Z Rebase and merge됐고 merge SHA는 `3641212a08a9d5174880dac07f4e5aa146ae52e7`이다. Fixture SHA-256 `fc9ff608f630e7a6e6e2f185f8e49da93b696f5ea1b167efca4996082403380e`의 Chrome matrix, focused Rust checks 및 Android/iOS Simulator 결과는 [실행 근거](../spec/internal/evidence/c11-root-html-intrinsic-combined-tracks-runtime-2026-11-06/README.md)에 기록했다. 이 merge는 이 bounded combined-track slice만 완료하며 C11.4/C11 체크와 공식 완료 `78/214`를 바꾸지 않는다. 후속 공식 문서와 공개 roadmap 동기화 및 Tailnet 제공본 확인은 순차 문서 작업으로 처리한다. 이 상태·로드맵 문서 변경분은 [30개 관점 검토](../spec/internal/evidence/c11-root-html-intrinsic-combined-tracks-runtime-2026-11-06/docs-pr-diff-review-30.md)에 기록한다.

## 제외와 완료 판정

이 bounded integration은 explicit/implicit intrinsic sizing의 모든 grammar를 새로 지원하거나 public CSS, 전체 C11.4/C11, WPT, 일반 HTML, 다중 text item, arbitrary fonts/writing modes, iOS compositor 또는 hardware GPU parity를 완료하는 것이 아니다. 단독 기능 matrix만 통과하고 combined cross-axis case를 검증하지 않은 경우 완료가 아니다.

계획 및 구현은 현재 main checkout 한 곳에서 순차적으로 진행한다. 별도 worktree와 병렬 implementation PR을 만들지 않는다.

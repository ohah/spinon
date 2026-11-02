# C11.1 · parsed Root HTML Grid `minmax()` bounded explicit tracks

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid 기본 경로](./c11-root-html-grid-runtime.md) · [percentage explicit tracks](./c11-root-html-grid-percentage-tracks.md) · [`fr` explicit tracks](./c11-root-html-grid-fr-track.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [고정 HTML·독립 기대값·Chrome 156 관측·화면](../spec/internal/evidence/c11-root-html-grid-minmax-precomparison-2026-11-03/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-minmax-precomparison.html) · [expected.json](../spec/internal/evidence/c11-root-html-grid-minmax-precomparison-2026-11-03/expected.json) · [Chrome observation](../spec/internal/evidence/c11-root-html-grid-minmax-precomparison-2026-11-03/chrome156-observation.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-minmax-plan-review-20.md) · **계획 PR diff 적대 검토:** [30개](../spec/internal/evidence/c11-root-html-grid-minmax-precomparison-2026-11-03/plan-pr-diff-review-30.md) · **구현/runtime 검토:** 구현 후 새 20개 · **구현 PR diff:** 별도 30개.

**상태:** 구현 전 계획. 기본 Root HTML projection은 모든 `ComputedCssGridTrackSize::MinMax`를 거부하고 일반 typed converter는 이미 `GridTrackSize::MinMax`를 만든다. 구현 PR은 계획 PR 병합 뒤 이 체크아웃에서 순차 진행한다. 이 bounded slice는 C11.1/C11 전체 완료나 일반 CSS Grid 지원을 뜻하지 않으며 공식 완료 `72/213`은 유지한다.

## 목표와 현행 결손

기존 `RuntimeSession::new_runtime_gpu_with_root_html()`의 parsed Root HTML `RuntimeRootHtmlTextPaintV1` 경로에서 명시 Grid track의 제한 `minmax()`를 기존 Stylo computed value → Root HTML validation → typed Grid projection → Taffy sizing → runtime scene pipeline으로 연결한다. CSS parser, 새 runtime profile, scene-ID route, FFI ABI, public API를 추가하지 않는다.

`crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs::validate_root_html_grid_tracks()`는 현재 direct finite non-negative length/percentage/fraction만 허용한다. `ComputedCssGridTrackSize::MinMax`는 일반 converter에서 이미 min/max breadth를 변환하지만 기본 Root HTML profile gate 전에 거부된다. [axis/range validation](../crates/spinon-style-to-layout/src/projection/style_values/root_html_grid_tracks.rs)은 direct percentage와 fraction만 탐지하므로 nested breadth도 별도로 검사해야 한다.

## 규범 해석과 기준 구현

[CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions)는 `minmax()`의 결과를 min 이상·max 이하의 범위로 정의하며 max<min일 때 max를 min에 바닥 처리한다. `<flex>` 최대값은 flex factor로 작동하고 최소값으로는 유효하지 않다. §11.6의 track maximization과 §11.7.1의 flex track 계산은 definite available space, growth limit, gaps를 포함해 used track sizes를 계산한다. Percentage track breadth는 definite axis에서 해당 Grid container의 inner size를 기준으로 resolve한다.

기대값은 규범과 고정 root size·gap을 이용해 [`expected.json`](../spec/internal/evidence/c11-root-html-grid-minmax-precomparison-2026-11-03/expected.json)에서 독립 계산했다. Agent Browser HeadlessChrome 156.0.0.0 observation은 [`chrome156-observation.json`](../spec/internal/evidence/c11-root-html-grid-minmax-precomparison-2026-11-03/chrome156-observation.json)에 고정했다. Browser output은 cross-check이며 규범상 기대값을 대체하지 않는다. 비교 허용치는 used sizes와 item origins 모두 0.02 CSS px다.

## 허용 범위

- 기본 parsed Root HTML 경로의 `grid-template-columns`와 `grid-template-rows`에서 explicit track list 안의 `minmax()`를 허용한다.
- Min breadth는 finite non-negative `LengthPx` 또는 `Percentage`만 허용한다. Max breadth는 finite non-negative `LengthPx`, `Percentage` 또는 `Fraction`만 허용한다. CSS grammar에서 flex minimum은 무효이며, 수동 typed-value injection도 명시 거부한다.
- 각 축의 percentage breadth와 fraction maximum은 같은 computed-style snapshot의 해당 축 definite finite non-negative CSS-pixel width/height를 요구한다. computed min-size는 `auto`, max-size는 `none`, aspect-ratio는 미설정이어야 한다. math-backed 또는 cyclic/indefinite basis는 기존 direct percentage/`fr` 경계와 같이 fail-closed한다.
- 두 fixed/percentage breadth가 definite basis에서 resolve되면 max<min을 min까지 바닥 처리한다. Fraction maximum은 existing flexible-track algorithm을 사용하며 fixed/percentage sizes와 gap을 제외한 available space를 기준으로 계산한다.
- 기존 integer `repeat()` expansion이 만든 minmax track은 line-name metadata와 auto-repeat 없이 일반 explicit track과 동일하게 처리한다. 기존 track cap과 line metadata quota를 바꾸지 않는다.
- 다른 explicit direct `px`/`%`/`fr` track 및 기존 fixed/percentage gap과 섞을 수 있다. Axis basis, gap, overflow 및 style/layout revision은 기존 계약을 유지한다.

## 이번 범위 밖

`minmax(auto, …)`, `minmax(min-content, …)`, `minmax(max-content, …)`, max의 `auto`/intrinsic breadth, `fit-content()`, math breadth, flex minimum, auto-repeat, line-name expansion, implicit `grid-auto-*` minmax, `grid-auto-flow:column` 조합, named-area track relaxation, nested Grid, auto/intrinsic item contribution 확장, alignment/placement 확장, arbitrary CSS admission, WPT conformance는 포함하지 않는다. 기존 named-area 경로는 현재 exact fixed-px tracks만 유지한다. 이 입력은 property와 node가 특정된 `unsupported_computed_value` 진단으로 scene publish 전에 거부한다.

일반 CSS-valid intrinsic/auto controls와 CSS-invalid negative-breadth control은 browser fixture에 남겨, 유효한 미지원 값을 syntax-invalid cascade fallback과 구분한다. WPT oracle나 Chrome exact pixel equality를 주장하지 않는다.

## 비교 행렬과 expected behavior

| 장면 | 독립 기대값 | 실패 경계 |
|---|---|---|
| `single-max` | width 160, `minmax(20,80)` → columns `[80]` | growth limit에 도달한 뒤 trailing space 보존 |
| `bounded-space` | width 60 → `[60]` | available size가 max보다 작음 |
| `minimum-overflow` | width 30, columns `minmax(40,80) 10` → `[40,10]`; 두 번째 8px item x=40..48 | 최소 track 유지 및 root 오른쪽 밖 visible marker |
| `max-under-min` | `minmax(40,20)` → `[40]` | max를 min으로 바닥 처리 |
| `two-bounds` | width 180, gap 10, tracks `[60,80]`; second item x=70 | 서로 다른 growth limit, gap 산입, 남은 30px |
| `percentage-bounds` / `percent-max` | width 200에서 `[80]` / `[60]` | definite column percentage basis |
| `percent-min-floor` / `px-min-percent-floor` | `[120]` / `[60]` | percentage-to-percentage 및 px-to-percentage max<min |
| `fraction-max` / `percent-flex-max` | width 130, gap 10 → `[60,60]`; width 200 → `[100,100]` | px/% minimum 아래의 flex max 분배 |
| `zero-fr-max` | width 130, gap 10 → `[20,100]` | 0fr maximum은 20px minimum보다 작아지지 않음 |
| `zero-percentage-basis` | width 0 → `[0]` | definite zero basis도 유효 |
| `row-max` / `row-percentage-bounds` | height 100에서 `[80]` / `[40]` | fixed·percentage block-axis basis |
| `repeat-bounds` | width 100, repeat count 2, gap 5 → `[40,40]`; second item x=45 | fixed integer repeat expansion |

Browser-only `intrinsic-max`, `auto-max`, invalid negative-max controls는 규범 관찰값은 보존하되 product positive matrix와 분리한다. 독립 expected와 모든 관측값은 evidence JSON을 따른다.

## 구현 수락 기준

1. 기본 Root HTML route에서 기존 parsed source, selector, computed typed value와 document/style/layout revision identity를 유지하고 새 profile 또는 host ID/class/child-index 기반 성공 분기를 만들지 않는다.
2. Columns/rows에서 MinMax breadth 허용 목록을 정확하게 검사하고 값·basis·resolved result의 finite/non-negative 및 f32-safe 조건을 layout 전에 확인한다. mixed-unit fixed/percentage max 비교는 definite axis에서 사용값으로 정규화한 뒤 min floor를 일관되게 적용한다.
3. Axis gate가 direct value와 동일하게 nested percentage/fraction maximum을 발견해야 한다. min/max override, aspect-ratio, indefinite/zero/positive definite axis 경계와 overflow/underflow를 positive/rejection matrix로 고정한다.
4. 기존 integer repeat, px/%/fr direct tracks, px/% gaps, named-area guard, implicit row/column, column-flow guard, hidden/overflow handling, allocation failure atomicity가 회귀하지 않는다. 지원 밖 computed values는 부분 frame 없이 구체 diagnostic으로 실패한다.
5. Rust tests는 projection acceptance, min/max mapping, invalid breadths, percentage/fraction axis constraints, resolved range, max<min, zero basis, zero-fr, row/column, repeat 및 regressions를 검증한다. Integration은 same root/source/style revision에서 full frame·track data를 expected.json과 비교한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 parsed Root HTML positive source를 기존 app entry로 실행한다. Layout geometry, draw/present, screenshot을 보존한다. Overflow scene에서는 AVD PixelCopy와 iOS Metal drawable sample로 root 끝 바깥 marker가 surface에 보이는지 확인한다. AVD software renderer, iOS compositor 제출 시점, 실기기/hardware GPU 한계를 분리 기록한다.
7. Default/product artifact에 opt-in scene/fixture ABI를 추가하지 않는다. `cargo fmt --all -- --check`, focused Rust suites, changed crate scoped Clippy, Android/iOS Simulator app builds를 실행하고 실행하지 않은 항목을 통과로 보고하지 않는다. Empty GitHub check rollup은 CI 통과로 부르지 않는다.
8. 구현/runtime 변경을 이 계획 검토와 다른 20개 실패 관점으로 검토하고, 구현 PR diff를 별도 30개 관점으로 검토한다. 결함 수정 뒤 관련 runtime/platform evidence와 검토 항목을 재확인한다.
9. Implementation PR은 한글 본문과 `enhancement`, `area: layout`, `area: testing` 라벨, 지원·미지원·실행근거를 포함한다. PR 화면 링크를 Agent Browser로 확인한다. Rebase and merge 뒤 내부 계약 0059, C11 plan, `spec/STATUS.md`, 공개 roadmap을 실제 제한 구현에 맞게 동기화하되 C11/C11.1 체크와 `72/213`은 유지한다. Tailnet preview는 기존 승인 범위대로 갱신하고 served hash와 desktop/mobile overflow를 확인한다.

## 진행 순서

1. 고정 precomparison, 독립 expected, 이 계획, 계획 실패 관점 20개와 계획 PR diff 30개를 먼저 검토해 계획 PR을 Rebase and merge한다.
2. 최신 `main`에서 기본 Root HTML validator, axis/range gate, projection/runtime tests와 simulator integration을 구현한다. 이 단계에서만 제품 코드에 손댄다.
3. 새 implementation/runtime hostile review 20개와 별도 PR diff review 30개를 고친 뒤 실행·화면·한계를 묶어 구현 PR을 Rebase and merge한다.
4. Merge SHA 기준 상태 문서와 Tailnet roadmap을 갱신하고, served content/hash와 overflow surface artifact를 확인한다.

이 계획 PR은 API를 추가하거나 기능 구현을 완료 표시하지 않는다. C11 상위와 C11.1/C11.3/C11.4 체크 및 공식 완료 `72/213`은 구현 PR에서도 유지한다.

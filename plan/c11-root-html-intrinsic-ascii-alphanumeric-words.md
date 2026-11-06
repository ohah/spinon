# C11.4 parsed Root HTML ASCII 영숫자 단어 intrinsic sizing

**상태:** 계획 및 Chrome 157 사전 비교 완료. 구현 전이다. 공식 C11.4/C11 checkbox와 완료 `78/214`를 바꾸지 않는다.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [기존 ASCII letter-word 구현](./c11-root-html-intrinsic-letter-words.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**사전 비교:** [Chrome observation과 독립 기대값](../spec/internal/evidence/c11-root-html-intrinsic-ascii-alphanumeric-precomparison-2026-11-06/README.md) · [10-scene fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-ascii-alphanumeric-precomparison.html) · [Agent Browser screenshot](../spec/internal/evidence/c11-root-html-intrinsic-ascii-alphanumeric-precomparison-2026-11-06/chrome-agent-browser.png)

**계획 실패 관점 검토:** [20개 항목](./c11-root-html-intrinsic-ascii-alphanumeric-words-plan-review-20.md)

**계획 PR diff 검토:** [30개 항목](../spec/internal/evidence/c11-root-html-intrinsic-ascii-alphanumeric-precomparison-2026-11-06/plan-pr-diff-review-30.md)

## 결손과 목표

PR [#1208](https://github.com/ohah/spinon/pull/1208)은 parsed Root HTML 기본 경로에서 `white-space:normal` direct text를 `grid-auto-rows:min-content`에 연결하고 ASCII letter words를 처리한다. 숫자만 있는 `12`, 영문·숫자 혼합 `A1`, 숫자로 시작하는 `1A`는 현재 allowlist가 거부한다. 이 계획은 동일한 제한 경로에서 token grammar를 `[A-Za-z0-9]+`로 확장해 ASCII 숫자가 포함된 unbreakable word의 min-content contribution 및 row sizing을 처리한다.

새 line-breaking·shaping·Grid algorithm을 추가하지 않는다. 기존 HTML text source, identity 보존, normal-whitespace 처리, intrinsic sizing과 paint 경로를 재사용한다.

## 규범 기준과 독립 oracle

- [CSS Grid Layout Module Level 1, 2025 CRD §11.4–11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)의 min-/max-content contribution 및 track sizing 절차를 따른다.
- [CSS Text Module Level 4 §3–6](https://drafts.csswg.org/css-text-4/#white-space-property)의 `white-space:normal`, wrapping 및 `word-break:normal` 경계를 따른다. Editor's Draft는 작업 중 명세이며 WPT 적합성 증거로 취급하지 않는다.
- [Chrome 157 사전 비교](../spec/internal/evidence/c11-root-html-intrinsic-ascii-alphanumeric-precomparison-2026-11-06/README.md)는 10 scenes의 used track/row/line을 독립 계산과 비교한다. 최종 Chrome track을 expected 계산에 넣지 않는다.
- 독립 expected는 고정 source token과 Canvas advance를 1/64 CSS px 단위로 올림해 계산한다. `min-content` column은 가장 넓은 unbreakable token이고, definite 40px `auto`는 `max(min-content, min(40px, max-content))`다. fixture line 수는 이 폭과 token 사이의 collapsible whitespace를 이용해 독립적으로 greedy 계산한다.

## 제한 grammar와 fail-closed 동작

1. Text token은 ASCII 영문 또는 숫자로 이루어진 비어 있지 않은 run `[A-Za-z0-9]+`만 허용한다. token 사이와 source 앞뒤에는 SPACE, TAB, LF, CR을 허용하며 source text를 trim·normalize·rewrite하지 않는다.
2. 기존 parsed Root HTML visible direct-child Grid profile만 대상으로 한다: definite 40×40px container, 10px explicit row, `grid-auto-rows:min-content`, `font-family:monospace`, `font-size:10px`, `line-height:10px`, `lang="en"`, `white-space:normal`, `word-break:normal`, `overflow-wrap:normal`.
3. Unicode, punctuation, NBSP, U+000B, U+000C, empty/whitespace-only source, 다른 font/style/owner, hidden/detached node 및 다른 track grammar는 계속 거부한다.
4. text source와 exact Text node identity를 보존한다. 구조·revision·style·track 검증 실패 때 이전 revision frame이나 partial draw를 새 결과로 publish하지 않는다.
5. 일반 author CSS admission, 임의 font shaping, C15 text layout, WPT runner, placement/named-area 조합 및 전체 C11.4는 범위 밖이다.

## Acceptance

- [고정 Chrome 비교](../spec/internal/evidence/c11-root-html-intrinsic-ascii-alphanumeric-precomparison-2026-11-06/expected.json)의 10개 scene을 Style-to-layout과 RuntimeSession에서 재생한다. exact source·node identity, revision, Grid track trace, rows, item frame, line count, reverse DOM order 및 rejection atomicity를 확인한다.
- Default Root HTML route에서 ASCII 숫자 source `12345678`을 parse해 layout-ready, draw/present, node source identity와 40px root 바깥 overflow item paint를 확인한다. Chrome 비교의 `(42px,19px)` sample을 같은 item background 지점으로 사용하되 실제 기기·hardware GPU·iOS compositor 이후 pixel equality로 확대 주장하지 않는다.
- Style-to-layout 및 RuntimeSession Rust 행렬에서 10개 scene 전부의 geometry·trace·source identity를 확인한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 앱 경로에서는 고정 numeric source `12345678`의 DOM·layout ready·draw/present와 지정 overflow sample을 확인한다. PixelCopy/Metal drawable은 이 고정 sample만 비교하고, 두 시뮬레이터 결과를 실기기 결과로 설명하지 않는다.
- 변경 crate Rust suite, `cargo fmt --all -- --check`, scoped Clippy, 기본/fixture-test artifact boundary를 기록한다. 기존 unrelated failure와 변경 유발 실패를 구분한다.
- 구현 후 별도 구현/runtime 실패 관점 20개와 실제 구현 PR diff 검토 30개를 수행한다. 계획 PR의 실제 diff도 별도로 30개 관점에서 검토한다.

이 slice를 C11.4 또는 C11 완료로 표시하지 않는다. 내부 계약 버전은 `0.1.0`을 유지한다.

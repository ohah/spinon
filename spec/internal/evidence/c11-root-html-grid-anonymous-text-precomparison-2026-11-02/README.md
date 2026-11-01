# C11.1 parsed Root HTML Grid anonymous text 사전 비교

이 자료는 기본 parsed Root HTML runtime에서 직접 text child가 CSS Grid의 anonymous item으로 배치되고 그 text가 실제 root renderer에 전달되어야 한다는 구현 전 기준이다. 고정 입력은 [`root-html-grid-anonymous-text-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-anonymous-text-precomparison.html), 독립 기대값은 [`expected.json`](./expected.json), Agent Browser Chrome 관측은 [`chrome-observation.json`](./chrome-observation.json), 보조 화면은 [`chrome-156-anonymous-grid-text.png`](./chrome-156-anonymous-grid-text.png)다.

구현 범위와 순서는 [전용 C11.1 계획](../../../../plan/c11-root-html-grid-anonymous-text.md)에, 계획 실패 관점 20개는 [계획 검토](../../../../plan/c11-root-html-grid-anonymous-text-plan-review-20.md)에, 실제 계획 PR diff 검토 30개는 [검토 기록](./plan-pr-diff-review-30.md)에 기록한다.

## 기준과 독립 계산

- 기준 문서는 [CSS Grid Layout Module Level 1, 2025-03-26 CRD §6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-items)의 anonymous grid item 규칙과 [§8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo)의 row-major auto-placement다.
- Grid root는 `(0,0)`, `120×70px`, Columns `[40,80]px`, Rows `[70]px`, gap 0이다. Track 합이 root 크기와 같다.
- Text는 computed `white-space: normal`, `font: 16px / 20px monospace`, `color: rgb(229, 72, 77)`로 고정했다. verifier가 이 computed tuple을 검사한다.
- 유일한 비공백 direct text run `AB`는 CSS Grid에서 anonymous item 하나를 만든다. 기본 `grid-auto-flow:row`의 첫 빈 cell은 row 1 / column 1이고, 기본 `justify-self`·`align-self:normal`은 이 auto-size item을 해당 `40×70px` cell에 stretch한다. Item 자체는 DOM element가 아니므로 DOMRect로 직접 조회하지 않는다.
- LTR/기본 text alignment에서 text Range의 시작은 cell 원점 `(0,0)`이다. Chrome의 monospace advance는 관측 자료에만 둔다. 플랫폼 간 glyph metric·pixel equality 기대값으로 쓰지 않는다.
- 공백만 있는 direct text는 anonymous item을 만들지 않는다. Chrome 대조에서 Range rect 0개, 동일한 root/tracks를 관찰했다.

## Agent Browser 관측

Agent Browser의 HeadlessChrome `156.0.0.0`에서 local fixture를 열고 viewport `1280×633 CSS px`, DPR 1, computed track list, root DOMRect, child node 종류, text Range rect를 읽었다. `AB`는 한 Range rect `(0,0,19.265625,19)`로 관찰됐고 첫 40px column 안에 놓였다. whitespace-only control은 Range rect 0개였다. 화면은 이 Chrome 기준 문서의 보조 증거이며 Spinon 결과가 아니다.

현 main은 `root_html_default_render/tree.rs`에서 non-whitespace direct text under Grid를 `unsupported_root_html_grid_text_item`으로 scene publication 전에 거부한다. 기존 regression [`root_html_grid.rs`](../../../../crates/spinon-runtime/src/session/tests/root_html_grid.rs)의 direct/preserved text cases도 이 diagnostic을 고정한다. 이 baseline은 현재 구현의 의도된 제한을 기록하며 성공 동작으로 잘못 세지 않는다.

[`verify-precomparison.mjs`](./verify-precomparison.mjs)는 fixture source, expected geometry, Chrome observation과 whitespace control을 대조한다. `SHA256SUMS.txt`는 fixture, expected/observation, screenshot 및 verifier hash를 고정한다. Agent Browser session을 닫고 Agent Browser/Chrome headless process가 남지 않은 것을 확인했다.

재현 명령은 `node spec/internal/evidence/c11-root-html-grid-anonymous-text-precomparison-2026-11-02/verify-precomparison.mjs`다. 이 실행은 fixture ID, HeadlessChrome 156, root/tracks, text Range, whitespace control을 검사해 PASS했다. 결과와 artifact hashes는 [verification.json](./verification.json), manifest는 [SHA256SUMS.txt](./SHA256SUMS.txt)에 보존한다. 이 검사는 사전 비교 fixture만 확인하며 Spinon runtime을 실행하지 않는다.

## 범위 한계

이 기준은 단일 non-whitespace anonymous text run과 fixed px tracks에 한정한다. 여러 run 사이의 element separator, implicit track, intrinsic track contribution, line wrapping, inline element style, arbitrary font shaping, `white-space:pre`, nested Grid, general CSS admission, WPT, Android/iOS runtime 및 glyph pixel parity를 검증하지 않는다.

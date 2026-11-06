# C11.4 parsed Root HTML normal whitespace for intrinsic implicit rows

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [intrinsic implicit rows with multiple direct text items](./c11-root-html-intrinsic-combined-multiple-text.md) · [internal contract 0059](../spec/internal/0059-c11-grid-layout.md)

**Precomparison:** [Chrome 156 and independent input matrix](../spec/internal/evidence/c11-root-html-intrinsic-normal-whitespace-precomparison-2026-11-06/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-normal-whitespace-multiple-text-precomparison.html) · [expected values](https://raw.githubusercontent.com/ohah/spinon/6af70e0e54d52705f7be908f7e2257795ad361e5/spec/internal/evidence/c11-root-html-intrinsic-normal-whitespace-precomparison-2026-11-06/expected.json) · [raw Chrome observation](https://raw.githubusercontent.com/ohah/spinon/6af70e0e54d52705f7be908f7e2257795ad361e5/spec/internal/evidence/c11-root-html-intrinsic-normal-whitespace-precomparison-2026-11-06/chrome/chrome156-observation.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-intrinsic-normal-whitespace-multiple-text-plan-review-20.md). **계획 PR 변경 검토:** [30개](../spec/internal/evidence/c11-root-html-intrinsic-normal-whitespace-precomparison-2026-11-06/plan-pr-diff-review-30.md). **구현 검토:** 구현 후 별도 실패 관점 20개와 실제 구현 PR diff 30개를 새로 검토한다.

## 문제와 목표

병합된 Root HTML C11.4 intrinsic-row 경로는 여러 direct text item, `grid-auto-rows: min-content`, text owner/source order 및 row contribution을 처리한다. 다만 text source 선택기가 `A` token 사이의 구분자를 단일 U+0020으로 제한한다. 같은 computed `white-space: normal` text가 HTML 들여쓰기, 탭, 여러 공백, line ending을 포함하면 Grid 전체가 거부된다.

기존 기본 `RuntimeRootHtmlTextPaintV1` 경로와 기존 min-content row 알고리즘에서 CSS Text의 normal whitespace 의미를 보존한다. 비어 있지 않은 `A` token들 사이와 앞뒤에 U+0020 SPACE, U+0009 TAB, U+000A LINE FEED, U+000D CARRIAGE RETURN이 올 수 있도록 source-selector를 확장한다. source 원문·text node identity·문서 순서·revision은 유지하고 기존 text-layout의 whitespace collapse/wrap에 맡긴다. 새 parser, runtime profile, fixture-only route, FFI 또는 public API는 추가하지 않는다.

## 기준값과 판정

[CSS Text Module Level 4 §4.1, §4.3.1, §4.3.3](https://drafts.csswg.org/css-text-4/#white-space-processing)는 collapse 모드에서 공백·탭·segment break 처리를 정의하고 carriage return을 space처럼 취급한다. Chrome 156 Agent Browser 사전 비교의 24개 scene은 세 가지 row/grid 불변 조건을 고정한다.

- `min-content` column에서 사용 열 `6.03125px`, 긴 text 6줄, 정방향 Rows `[10,60,10,34]px`, 역순 Rows `[10,10,60,34]px`.
- `auto` column에서 사용 열 `40px`, 긴 text 2줄, 정방향 Rows `[10,20,10,34]px`, 역순 Rows `[10,10,20,34]px`.
- 반복 공백·앞뒤 공백·tab·line feed·HTML `&#x0d;` carriage return·혼합 입력은 같은 열과 순서의 단일 공백 기준과 모든 item frame 및 line position이 같다.

표준의 segment-break transformation은 인접 문자와 언어에 따라 UA 결정이 남는다. 이 계획의 fixture는 `lang="en"`으로 고정하며 한글/중국어, 다른 script, 임의 author language 판정을 넓히지 않는다. 모든 scene의 정확한 Chrome 결과는 [precomparison evidence](../spec/internal/evidence/c11-root-html-intrinsic-normal-whitespace-precomparison-2026-11-06/README.md)를 기준으로 삼는다.

## 허용 범위와 실패 동작

- 같은 visible Grid owner, fixed definite container, 기존 explicit-column grammar, 단일 fixed explicit row, `grid-auto-rows:min-content`, normal text style, 여러 direct `<div>` text item을 유지한다.
- 각 text source는 하나 이상의 단독 ASCII `A` token만 포함한다. token 사이는 하나 이상의 위 네 collapsible ASCII whitespace 문자로 구분할 수 있고, source 앞뒤에도 이 문자가 올 수 있다. 단일 공백 profile의 기존 입력 결과는 그대로 보존한다.
- original source를 문자열 normalize/trim/rewrite 하지 않는다. whitespace-only text source는 text item으로 새로 인정하지 않는다. owner 아래 formatting whitespace 처리는 현재 동작을 바꾸지 않는다.
- `AA`, `B`, Unicode token, NBSP, vertical tab, form feed, 다른 control, empty/whitespace-only source, unsupported item/style/tree/owner/revision, hidden/detached item과 10,000-track 초과는 거부한다. partial frame/paint snapshot은 publish하지 않는다.
- columns `min-content`/`auto`, 6개 source variant, forward/reverse DOM 순서에서 each text node의 원문, style/layout revision, per-item min-content contribution, row trace, frame, line count, draw order와 Grid 아래 visible overflow를 검사한다.

## 구현과 검증 순서

1. 먼저 계획 PR을 병합한다. 변경은 계획·Chrome 사전 비교 fixture/evidence 및 review 자료에 한정한다.
2. `root_html_intrinsic_auto_rows::supports_text_source`를 ASCII collapsible whitespace tokenization으로 확장하고 공백만 있는 source가 성공하지 않는지 검사한다. NBSP, form feed, vertical tab과 non-`A` token은 reject한다.
3. style-to-layout tests에서 단일 공백 입력의 기존 결과를 회귀 확인하고 24개 source/columns/order 조합의 geometry·text-layout raw source·line count·row contribution identity를 검증한다. 잘못된 두 번째 item과 다른 invalid source는 전체 output을 원자 거부한다.
4. Default `RuntimeSession` Root HTML route에서 같은 24개 조합을 검사한다. render snapshot이 두 text source의 raw bytes/NodeId/order와 revision을 보존하고 row sizing/paint/draw order를 유지하는지 확인한다. Text raw source를 사전 normalize하지 않는다.
5. 실패 관점 20개 구현/runtime 검토를 작성하고 변경한 crate의 focused suites, `cargo fmt --all -- --check`, scoped Clippy와 기존 관련 regression을 실행한다. 영향 범위에 맞는 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 default Root HTML route를 실행해 frame/text/track/draw/present를 기록한다. OS screenshot은 visual smoke로만 분류하며 readback을 하지 않으면 pixel equality로 주장하지 않는다.
6. 실제 구현 PR diff를 30개 독립 실패 관점으로 검토하고 발견을 수정한 뒤 영향받은 검증을 반복한다. CI 상태는 GitHub에서 실제 확인한 값만 기록한다.
7. 구현 PR 병합 후 `spec/STATUS.md`, public roadmap과 Tailnet preview를 실제 merge hash 기준으로 동기화한다. Agent Browser session/process를 종료한다.

## 완료 경계

이 단계는 Root HTML `grid-auto-rows:min-content`의 고정 ASCII `A` token text profile에 normal collapsible whitespace 구분자를 추가한다. arbitrary text/Unicode/font/language, non-normal whitespace, nested/anonymous text, 일반 CSS Grid admission, WPT 적합성, C11.4 또는 C11 완료를 뜻하지 않는다. 공식 상태 집계와 C11.4/C11 checkbox는 바꾸지 않는다. 계획과 구현은 현재 main checkout에서 한 PR씩 순차 진행한다.

## 구현 기록 · 2026-11-06

제한 구현 PR [#1205](https://github.com/ohah/spinon/pull/1205)는 `supports_text_source`에 ASCII SPACE/TAB/LF/CR 구분자 처리를 추가하고, 기존 단일 SPACE 결과와 24개 source/column/order 조합을 style-to-layout 및 기본 `RuntimeSession`에서 검사한다. Empty/whitespace-only source, 비허용 control/Unicode whitespace, `AA`, non-`A` token은 atomic rejection으로 유지한다. 원문, text node/owner identity, order, revisions, per-item contribution과 draw order를 검증한다.

Focused style-to-layout 9/9, RuntimeSession 13/13, opt-in raw-source/trace 2/2, scoped Clippy와 rustfmt가 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기본 Root HTML route는 layout-ready, 8 boxes, app DOM assertion, 2 text draw/present에 도달했다. [실행 근거와 다섯 번의 harness 시도](../spec/internal/evidence/c11-root-html-intrinsic-normal-whitespace-runtime-2026-11-06/README.md) · [구현 검토 20개](../spec/internal/evidence/c11-root-html-intrinsic-normal-whitespace-runtime-2026-11-06/implementation-review-20.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-root-html-intrinsic-normal-whitespace-runtime-2026-11-06/pr-diff-review-30.md). Feature-enabled runtime Clippy의 두 unchanged baseline errors와 simulator/pixel verification limits는 실행 근거에 기록했다. PR merge 후 `spec/STATUS.md`, public roadmap과 Tailnet preview를 merge SHA 기준으로 갱신한다. C11.4/C11 checkbox 및 공식 `78/214`는 유지한다.

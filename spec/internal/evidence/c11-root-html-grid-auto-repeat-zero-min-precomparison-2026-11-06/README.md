# C11.1 Root HTML auto-repeat 0px minimum 사전 비교

기준 checkout은 PR #1214 병합 뒤 `main` HEAD `fb7faf314dcacf5cefab8439f8520ca9bcb1978b`다. 입력과 기대값은 구현 코드와 분리해 고정했다.

## 질문과 판정 기준

기존 parsed Root HTML auto-repeat profile은 `minmax(positive-px, 1fr)`만 받는다. CSS Grid auto-repeat 문법에서 definite `0px` minimum도 fixed-size가 될 수 있으므로 `minmax(0px, 1fr)`를 기존 auto-fill/auto-fit 제한 route에 연결할 때의 repetition count, track sizing과 auto-fit collapse를 조사했다.

CSS Grid 2025 CRD §7.2.3.1–7.2.3.2는 자동 반복을 definite track sizing function에 한정하고, 반복 개수를 셀 때 track size가 0에 가까우면 UA가 정한 floor를 적용하도록 한다. 명세는 1 CSS px을 권장한다. 이번 profile은 1 CSS px을 선택하고 이 값을 **count 계산에만** 적용한다. 실제 track minimum은 선언대로 0px로 유지한다. 다중 track pattern의 독립 count 식은 `max(1, floor((available + gap) / (sum(max(track minimum, 1px)) + pattern track count × gap)))`이다.

근거: [CSS Grid §7.2.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat) · [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat).

## Agent Browser 관찰

Agent Browser의 HeadlessChrome 157에서 `CSS.supports('grid-template-columns', 'repeat(auto-fill, minmax(0px, 1fr))')`는 true였다. 10개 fixture를 독립 count 식으로 계산하고 Chrome의 resolved tracks와 첫 item size를 대조했다. 10/10이 일치했다. 재현 입력은 [fixture](../../../../tests/fixtures/css/c11/root-html-grid-auto-repeat-zero-min-precomparison.html), 원본 DOM 관찰은 [Chrome JSON](./chrome157-observation.json), 독립 계산 대조는 [comparison.json](./comparison.json)이다.

주요 장면:

- 11px, gap 0, 한 개 zero-min track: auto-fill 11개, 각 1px.
- 11px, gap 1px: 6개, 각 1px.
- 12.5px, gap 0.5px: 8개, 각 1.125px.
- 두 track pattern `[0px, 0px]`, 19px/gap 1px: 5 repetitions/10 tracks.
- mixed pattern `[0px, 2px]`, 19px/gap 1px: 4 repetitions/8 tracks `[1,2]px`.
- auto-fit: 빈 tracks는 0px로 collapse하고, 점유된 첫 track은 사용 가능한 폭을 채운다. 빈 auto-fit은 11개 track 모두 0px가 된다.
- rows 축의 7px/gap 1px 및 mixed 19px/gap 1px도 동일 count 규칙을 보였다.

Chrome 157은 이 비교의 browser oracle이며, WPT 통과나 CSS Grid 전체 지원을 뜻하지 않는다. 구현 acceptance는 별도 고정 expected 및 기존 Taffy/runtime 제한 profile 검증을 사용한다. Chrome 화면은 [Agent Browser screenshot](./chrome157-agent-browser.png)이다.

## 산출물 무결성

fixture, independent expected, browser 관찰, 비교기 및 screenshot hash는 [SHA256SUMS.txt](./SHA256SUMS.txt)에 기록한다. Independent expected는 [독립 JSON](./independent-expected.json)에, 계획 검토 20개는 [계획 파일](../../../../plan/c11-root-html-grid-auto-repeat-zero-min-plan-review-20.md)에, 계획 PR diff 검토는 [이 폴더의 검토 기록](./plan-pr-diff-review-30.md)에 둔다.

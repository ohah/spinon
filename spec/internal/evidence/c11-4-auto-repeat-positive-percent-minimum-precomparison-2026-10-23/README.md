# C11.4 definite positive percentage auto-repeat minimum 계획 수정 사전 비교

CSS Grid 2025 CRD의 fixed percentage minimum을 Chrome 156과 독립 계산으로 비교했다. `calc(0px + 10%)`와 직접 `10%`는 Stylo computed cascade에서 둘 다 `Percentage(0.1)`이므로 이 계획은 authored calc만 구분하지 않고 양수 percentage minimum subset을 다룬다. 이는 Spinon runtime 결과나 WPT pass가 아니다.

## 기준과 입력

- 규범 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-syntax), [repeat-to-fill](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat).
- 브라우저: Agent Browser가 구동한 Google Chrome `156.0.8078.48`, DPR 1, viewport `700×800`.
- CSS fixture는 [고정 HTML](../../../../tests/fixtures/css/c11/grid-auto-repeat-positive-percent-minimum-c114-precomparison.html)에서 calc, direct percentage, numeric control을 열·행 auto-fill/auto-fit 각각 비교하고, 150% calc와 180px control의 실제 border overflow도 비교한다.
- 독립 계산은 [spec-derived expectations](./spec-derived-expectations.json), 관측은 [Chrome observation JSON](./chrome-observation.json), 화면은 [Agent Browser 캡처](./chrome156-agent-browser.png)에 둔다.
- `calc(0px + 10%)`가 `Percentage(0.1)`로 정규화된 첫 cascade probe와 assertion log는 [normalization log](./stylo-cascade-normalization.log)다. 이 탐색 assertion은 예상 AST를 잘못 고정해 실패했고, 제품 회귀 실패가 아니다. probe test는 제거했다.
- 수정 계획은 [C11.4 positive percentage minimum 계획](../../../../plan/c11-4-auto-repeat-positive-percent-minimum.md), [수정 계획 실패 관점 20개](../../../../plan/c11-4-auto-repeat-positive-percent-minimum-revision-review-20.md), [revision PR 변경 검토 20개](./plan-revision-pr-diff-review-20.md)다.

## 독립 기대값과 관측

모든 grid는 content-box이고 네 변에 2px border, padding/gap은 0이다. CSS 지정 content axis 120px가 percentage basis이며, outer border box는 124px이다. §7.2.1의 nonnegative `<length-percentage>`에 따라 10% minimum은 12px이고 `floor(120/12)=10`회 반복한다. auto-fill tracks는 `[12]×10`; auto-fit은 placement 뒤 `[120,0×9]`로 접힌다. calc, direct 10%, 고정 12px의 track list와 item frames가 모든 네 축/mode case에서 같았다.

150%는 180px minimum이므로 auto-repeat은 한 track만 만든다. 단일 item은 content border edge에서 시작해 180px를 차지하므로 120px content axis보다 60px, 124px outer border box보다 58px 넘는다. Chrome의 columns와 rows 모두 calc/180px control에서 같은 overflow frame을 보였다. 캡처에는 실제 2px border와 바깥으로 이어지는 item fill이 함께 보인다.

indefinite/cyclic/zero-size basis, nonzero gap/padding, 다른 track maximum, layout/runtime 실행 및 platform pixel equality는 이 사전 비교에 포함하지 않았다.

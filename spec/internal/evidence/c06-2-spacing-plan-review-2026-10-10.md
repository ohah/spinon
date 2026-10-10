# C06.2 spacing percentage 계획 공격 검토

검토 대상은 [`c06-value-unit-conversion.md`](../../../plan/c06-value-unit-conversion.md)의 C06.2다. 검토를 통과하기 전에 Chromium을 기준으로 spacing percentage fixture를 새로 고정하고, 계획 범위·타입·오류 경계를 다시 썼다. 이 기록은 C06.2 구현 코드 검증과 합산하지 않는다.

## 고정 비교 근거

- Chromium: Google Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`.
- 환경: `320×800 CSS px`, DPR 1, `en-US`, UTC, light, coarse pointer, no hover.
- 입력: [`spacing-percentages.html`](../../../tests/fixtures/css/c06/spacing-percentages.html), [`spacing-percentages-inventory.json`](../../../tests/fixtures/css/c06/spacing-percentages-inventory.json).
- 관찰: [`c06-spacing-percentages-v1.json`](../../../tests/fixtures/css/references/c06-spacing-percentages-v1.json), 78개 노드. inventory·HTML·capture tool·Chromium helper와 실행 파일의 SHA-256이 reference에 기록됐다.
- 회귀 확인: [`c06-spacing-percentages.test.mjs`](../../../tools/css-reference/c06-spacing-percentages.test.mjs), 4개 검사 통과.
- 기준 문서: [CSS Box Model Level 3 Recommendation](https://www.w3.org/TR/css-box-3/)은 margin/padding의 containing-block logical width를 정의한다. [CSS Gaps Module Level 1](https://www.w3.org/TR/css-gaps-1/)은 Working Draft이며 gap percentage 대응 content-box 축과 Flex cyclic percentage 규칙을 제안한다. [CSSWG의 Flex margin/padding 상호운용 설명](https://www.w3.org/blog/CSS/2017/12/20/how-should-we-resolve-percentage-margins-and-padding-on-grid-and-flex-items/)은 엔진 간 과거 차이를 기록한다. 표준 문서 상태만으로 Chromium 동작을 추정하지 않고 고정 reference를 별도로 쓴다.
- 구현 위험: Cargo.lock의 Taffy `0.14.0` 소스 `src/compute/flexbox.rs`는 auto main size 결정 후 main-axis percentage gap을 재해석한다. Chrome fixture의 auto-height column Flex는 `row-gap:10%` computed value를 유지하면서 두 10px child 사이에 0px를 둔다. 이 조합은 Taffy에 percentage를 그대로 전달하면 엇갈릴 위험이 있다.

## 서로 다른 실패 관점

| # | 공격 관점 | 계획에서 확인한 실패 가능성 | 반영 및 판정 |
|---:|---|---|---|
| 1 | 표준 문서 최신성·성숙도 | 2025 Box Alignment 초안만 기준으로 삼으면 이후 CSS Gaps 문서와 표준 성숙도를 놓칠 수 있다. | Box Model Recommendation과 현재 CSS Gaps Working Draft를 구분해 링크하고, 브라우저 동작은 별도 pinned Chrome reference로 고정했다. |
| 2 | logical width와 물리 width 혼동 | vertical edge를 부모 height 기준으로 넘기면 CSS 결과와 다르다. | width 200px·height 120px fixture에서 네 margin/padding edge를 따로 기록한다. |
| 3 | margin top/bottom percentage | 일부 layout code가 top/bottom을 height 기준으로 취급할 수 있다. | Chrome에서 네 edge가 모두 20px임을 관찰하고, Stylo typed 값에서 Taffy 전환까지 축 검증을 요구한다. |
| 4 | padding이 있는 containing block | 부모의 border-box width를 basis로 쓰면 padding 30px가 오차에 섞인다. | border-box 260px와 padding 30px의 content width 200px를 fixture에 추가했다. child margin/padding 결과는 20px다. |
| 5 | Flex 엔진 간 상호운용 차이 | Flex item의 세로 margin/padding에 대해 과거 브라우저가 다른 기준을 사용했다. | 일반적인 CSS 규칙을 Chrome 호환 증거로 단정하지 않고, 제품 비교 기준을 고정한 Chromium 154 profile로 한정했다. |
| 6 | 음수 margin | 음수 margin을 padding/gap과 같이 거부하거나 0으로 보정할 수 있다. | signed margin variant를 계획하고 `-5%` → `-10px` 관찰을 fixture로 고정했다. |
| 7 | `auto` margin CSSOM | Chrome used-value CSSOM의 `90px`만 보고 Stylo의 `auto` 타입을 알 수 있다고 오판할 수 있다. | `auto`를 지원 범위에서 제외하고 computed string 재파싱 대신 typed Stylo 값으로 명시 거부한다. |
| 8 | shorthand 확장 | shorthand 1–4개 값 중 일부 방향만 연결되거나 edge 순서가 뒤집힐 수 있다. | margin 1/2/3/4값, padding 1/4값과 각 edge를 fixture에 둔다. |
| 9 | 선언 원천 간 cascade | stylesheet shorthand 뒤 inline longhand 우선순위가 반영되지 않을 수 있다. | margin/gap의 stylesheet→inline longhand 덮어쓰기를 포함하고 computed physical edges를 검사한다. |
| 10 | LTR/RTL logical side | logical inline start/end를 고정 left/right로 해석할 수 있다. | `horizontal-tb` LTR/RTL에서 margin과 padding의 physical side 매핑을 각각 고정했다. percentage 기준 축은 계속 containing width다. |
| 11 | literal 음수 padding/gap | 잘못된 declaration을 runtime adapter가 새 오류로 바꾸거나 이전 valid declaration을 지울 수 있다. | invalid-at-parse 값은 Stylo cascade가 보존한 이전 valid value를 사용하며 `-10%` padding과 negative gap 사례를 둔다. |
| 12 | `var()` 치환 후 invalid 값 | invalid-at-computed-value가 parse-time invalid처럼 이전 선언을 복원한다고 착각할 수 있다. | negative padding/gap `var()` 사례에서 padding initial `0px`, gap `normal`로 돌아가는 관찰을 별도로 고정했다. |
| 13 | `var()`의 percentage 타입 | CSSOM 문자열을 다시 파싱하거나 변수를 무시하면 퍼센트 semantics가 손실될 수 있다. | 최종 computed typed `LengthPercentage`를 사용하며 margin/padding/gap 변수 사용 사례를 넣었다. |
| 14 | 100% 초과 | 퍼센트 fraction을 비율 1로 clamp할 수 있다. | `125%` margin/padding과 `150%` gap을 그대로 보존한다. |
| 15 | fraction·overflow·유한성 | 소수 값을 정수 반올림하거나 finite 입력의 최종 곱셈 overflow를 놓칠 수 있다. | `33.333%`를 넣고 typed 입력 및 최종 geometry를 모두 finite 검사하도록 명시했다. |
| 16 | gap 축과 Flex 방향 혼동 | `row-gap`을 무조건 main axis 또는 `column-gap`을 무조건 height로 처리할 수 있다. | 대응 content-box 차원(width/height)과 Flex 방향을 독립적으로 캡처한다. definite row/column 경로만 geometry 지원 대상으로 둔다. |
| 17 | gap shorthand 및 override | one-value gap을 두 축에 복제하지 못하거나 second value/longhand override를 놓칠 수 있다. | 1값·2값 shorthand, inline column-gap override를 검사한다. |
| 18 | `normal` 및 nowrap 의미 | `normal`을 em으로 계산하거나 row `row-gap`을 한 줄의 child 간격에 더할 수 있다. | Flex `normal` used gap 0을 관찰하고 nowrap row의 row-gap은 줄 간격이 없어 시각 효과가 없다고 명시한다. |
| 19 | definite로 정해지는 auto size와 cyclic auto size 구분 | 모든 computed `auto`를 indefinite라고 하면 block-flow auto width와 definite Flex cross-axis stretch를 잘못 막고, 모두 definite라고 하면 auto-height column의 cyclic gap을 잘못 계산한다. | 78-node reference에 block-flow auto-width row Flex(32px gap), definite row parent의 stretch column(12px gap), intrinsic auto-height column(0px gap)을 나란히 둔다. tree에서 증명한 basis만 허용하고, cyclic/불명확 경로는 `node + property + axis` 오류로 닫는다. |
| 20 | oracle-only·root 범위 누수 | inline-flex 관찰값이나 document root CSS 특례를 일반 `<div>`/HostDocument root 지원으로 잘못 확장할 수 있다. | inline-flex는 oracle-only, root percentage spacing은 미지원으로 분리했다. 일반 element fixture만 runtime 지원 subset에 포함한다. |

## 계획 검토 후 남은 구현 전제

- `LayoutEdges`와 `LayoutGap`은 px와 percentage를 잃지 않는 value type을 사용해야 한다. 기존 C04 px 좌표가 그대로 유지되어야 한다.
- definite basis는 tree 관계에서 증명해 원래 축을 전달한다. auto width block-flow와 flex stretch는 가능한 definite 경로로 인정하고, cyclic intrinsic main size는 0으로 추측하지 말고 진단 가능한 오류로 반환해야 한다.
- 이 계획은 C06.2 범위만 결정한다. Grid, wrap, inline-flex, document-root spacing, auto margin, CSS math는 여기서 구현한다고 주장하지 않는다.

## 구현 전 재검토

첫 hostile review 뒤 `auto`를 전부 indefinite로 간주하면 두 유효한 Chrome 경로까지 막는다는 반례를 확인했다. Chromium 비교에 block-flow auto-width row와 definite Flex parent에 stretch된 auto-height column을 추가했다. 영향을 받는 definite basis·cyclic sizing·지원 경계 관점을 다시 확인해 tree 기반 basis 판정으로 계획을 수정했다. 구현에서는 `auto` 문자열 하나만 보고 오류를 내지 말고 parent display·축·stretch·부모 definite size를 검사해야 한다.

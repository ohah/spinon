# C11.4 자동 반복 고정 최대값 계획 · 독립 실패 관점 20개

검토 대상은 [계획](./c11-4-auto-repeat-inflexible-fixed-maximum.md), [Agent Browser 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-inflexible-fixed-maximum-precomparison-2026-10-29/README.md), 고정 HTML fixture와 Chrome DOM observation이다. 구현 전 계획 자체의 별도 검토다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | `<fixed-size>`의 두 번째 문법 분기를 잘못 해석할 수 있음 | CSS Grid 2025 CRD §7.2.3.1의 `minmax(<inflexible-breadth>, <fixed-breadth>)`와 계획의 세 minimum keyword를 대조했다. `auto`, `min-content`, `max-content`만 정확히 고정됐다. 통과. |
| 2 | maximum을 일반 `<track-breadth>`로 넓혀 `1fr`까지 허용할 수 있음 | 계획은 최대값을 `LengthPx`, `Percentage`, 제한된 `Calc`로 열거하고 Fraction/Auto/Intrinsic 최대값을 거부한다. 통과. |
| 3 | fixed minimum과 fixed maximum의 두 문법 분기를 혼동할 수 있음 | 기존 fixed-minimum 경로는 유지하고 새 예외는 intrinsic minimum만 대상으로 분리했다. 통과. |
| 4 | 반복 개수에서 maximum 대신 intrinsic minimum을 사용할 수 있음 | §7.2.3.2의 definite 축 계산 규칙과 CSSOM 결과를 대조했다. `minmax(auto,40px)`는 40px breadth를 반복 수 계산에 사용한다. 통과. |
| 5 | 트랙 사이 gap 수를 반복 트랙 수와 같다고 셀 수 있음 | 단일 트랙은 `(n−1)`개 gap, 2-track fragment는 `2n−1`개 gap으로 식과 fixture를 분리했다. 135px 두 트랙 경계는 반복 2개에 정확히 맞는다. 통과. |
| 6 | 129/130px 반복 경계를 반대로 계산할 수 있음 | 40px breadth와 5px gap에서 129px는 트랙 2개, 130px는 3개라고 독립 계산했고 Chrome CSSOM도 같은 값을 냈다. 통과. |
| 7 | percentage maximum의 기준을 border box 또는 device pixel로 잘못 둘 수 있음 | 고정 CSS px 크기, padding/border 0, DPR 1을 사용해 CSS content 기준을 130px로 고정했다. 통과. |
| 8 | percentage `30%`를 30px로 잘못 읽을 수 있음 | definite 기준 크기 130px에서 39px를 독립 계산하고 관측된 확장 트랙과 대조했다. 통과. |
| 9 | `calc(15px + 25%)`를 선언 문자열이나 임의 float로만 추정할 수 있음 | definite 기준 130px의 독립 기대값 47.5px와 Agent Browser의 사용 트랙 결과를 비교했다. CSS math 범위는 정확한 2항 subset으로 명시했다. 통과. |
| 10 | 임의 calc 표현식을 제한 subset에서 허용할 수 있음 | 계획은 임의 AST를 지원하지 않고 정확한 owner/property identity 재검사와 fail-closed 조건을 둔다. 통과. |
| 11 | `auto-fit`의 빈 트랙을 repeat count 계산 전에 제거할 수 있음 | 반복 수 3/2개를 유지한 뒤 item 하나를 배치했을 때 빈 트랙이 0px로 접히는 결과를 별도로 관측했다. 통과. |
| 12 | `auto-fill`의 빈 트랙을 `auto-fit`처럼 접을 수 있음 | 같은 입력의 `auto-fill` 확장 CSSOM과 `auto-fit` 접힘 CSSOM을 쌍으로 비교했다. 통과. |
| 13 | 실제 트랙 최소값 계산 없이 `min-content`/`max-content` 문법만 허용할 수 있음 | item 20px를 유지하고 최대 10px, 축 10px인 대조군에서 사용 트랙이 20px인지 확인하는 6개 scene guard를 뒀다. 통과. |
| 14 | `auto` minimum의 automatic minimum 바닥값이 item 크기와 무관하게 사라질 수 있음 | 20×20px item과 minimum보다 작은 최대값 대조군을 계획의 수용 기준에 포함했다. 통과. |
| 15 | 반복 조각의 두 번째 트랙을 첫 트랙 breadth로 잘못 대체할 수 있음 | 2-track `minmax(auto,40px) 20px` CSSOM 대조군에서 확장 트랙 4개와 `auto-fit` 빈 트랙 접힘을 직접 비교한다. 통과. |
| 16 | Columns는 통과하지만 Rows의 basis/property mapping이 실패할 수 있음 | 핵심 조합 36개와 overflow/경계 대조군에 양축 사례가 있으며 root 크기와 축별 CSSOM을 각각 검증한다. 통과. |
| 17 | definite하지 않은 축의 Percentage/calc를 fixed로 통과시킬 수 있음 | definite한 양의 기준 크기 검사를 명시하고 definite하지 않은 양성/음성 대조군을 둔다. 통과. |
| 18 | 음수·NaN·Infinity가 UA floor 또는 float 변환에서 정상값으로 바뀔 수 있음 | 사전 검사에서 유한하지 않거나 음수인 결과를 거부하고 Taffy count floor와 따로 확인하도록 계획했다. 통과. |
| 19 | fixture feature가 제품 artifact에 포함될 수 있음 | 기본/제품 archive와 opt-in fixture archive를 positive/negative symbol/payload 대조군으로 각각 검사한다. 통과. |
| 20 | 49-scene Chrome 비교를 Spinon WPT 통과나 C11.4 완료로 부풀릴 수 있음 | README, 계획, 완료 조건은 Chrome oracle와 runtime fixture 결과를 구분하며 C11.4/C11 checkbox와 `72/213`을 유지한다. 통과. |

**결과:** 통과 20/20, 미해결 사항 0개. 브라우저 관측 49개는 계획용 사전 oracle이며 구현 실행 결과가 아니다. 코드 구현 전에 계획 PR을 병합해야 한다.

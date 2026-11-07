# C11.4 implicit percentage fit-content 계획 실패 관점 검토 20개

대상: [계획](./c11-4-root-html-fit-content-auto-rows-percentage.md), 고정 입력 및 Chrome 관측. 구현 코드는 아직 시작하지 않았다.

| # | 실패 관점 | 계획의 판정·방지 기준 |
|---:|---|---|
| 1 | percentage를 viewport 높이에 적용 | Grid inner block size 기준을 명시하고 definite 100px Grid content box만 허용한다. |
| 2 | border-box 또는 padding box를 percentage basis로 사용 | profile에서 padding과 border 0을 강제하고 border/padding 확장은 범위 밖으로 둔다. |
| 3 | Taffy가 row percentage를 width basis로 해석 | Rows에만 적용되는 100px block-size 장면과 computed/used track을 확인한다. |
| 4 | `20%`를 `20` 대신 ratio `0.2`로 전달 | style projection과 RuntimeSession computed typed value를 ratio로 assert한다. |
| 5 | `0%`가 auto minimum 0을 넘는 track 생성 | zero cap scene과 0 minimum item으로 used row 0px을 확인한다. |
| 6 | 0px used item frame이 누락되거나 publication에서 버려짐 | zero-height item frame을 completed layout frames에서 별도로 검사한다. |
| 7 | cap 아래에서 max-content contribution이 잘리지 않음 | 20%/24% caps와 34px nested content에서 20/24px track을 검사한다. |
| 8 | cap과 max-content가 같은 경계에서 부동소수 오차 | 34% exact-boundary scene을 독립 기대값과 0.02px tolerance로 비교한다. |
| 9 | cap이 max-content보다 높아 track을 불필요하게 확장 | 40% scene에서 34px contribution을 유지한다. |
| 10 | `>100%` percentage를 100%로 임의 clamp | 120% cap scene을 포함하고 used size는 34px contribution으로만 제한됨을 확인한다. |
| 11 | fixed-size probe에 이전 row cap이 누적됨 | 반복되는 auto-row의 세 번째 probe frame/track 6px을 별도 assert한다. |
| 12 | item `min-height`가 cap보다 높을 때 잘못 clamp | 20% cap과 28px minimum floor scene을 고정한다. |
| 13 | `fit-content(calc(...))` AST가 direct percentage로 오인 | Typed `Calc`는 candidate에서 배제하고 runtime negative control로 확인한다. |
| 14 | mixed auto-row list의 일부 track만 허용 | singleton list shape만 허용하고 mixed/multiple list를 거부한다. |
| 15 | indefinite Grid height의 cyclic percentage가 조용히 확정 | layout admission에서 정확한 definite height profile을 요구하고 auto-height를 거부한다. |
| 16 | zero gap 조건에서 percentage basis와 gutter가 혼합 | row/column gap 0을 고정하고 nonzero gap은 거부한다. |
| 17 | existing explicit fit-content percentage admission 변경 | 별도 `RootHtmlFitContentV1` profile 회귀를 실행하고 변경 대상은 implicit auto row에만 둔다. |
| 18 | auto-repeat, areas, dense/column flow가 동일 mode로 유입 | candidate와 layout contract에서 이 결합을 거부하며 negative controls를 추가한다. |
| 19 | 새 gate가 오류 후 stale/부분 snapshot을 남김 | unsupported revision에서 no-partial publication을 확인한 뒤 같은 세션 복구를 검사한다. |
| 20 | fixture evidence가 제품/CSS 지원을 과장 | opt-in oracle/runtime 범위를 분리하고 official C11/C11.4 checkbox와 `78/214`를 유지한다. |

결과: 각 관점의 입력 basis, sizing boundary, profile admission, failure publication 및 지원 범위를 계획에 고정했다. 구현과 구현 PR 검토에서는 이 항목을 재사용하지 않고 새로운 관점을 작성한다.

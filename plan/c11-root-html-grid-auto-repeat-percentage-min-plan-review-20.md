# C11.1 auto-repeat percentage minimum 계획 검토 20개

대상은 [계획](./c11-root-html-grid-auto-repeat-percentage-min.md), 고정 fixture, 독립 기대값과 Chrome 157 observation이다. 구현 전 계획만 검토했으며 이 기록은 runtime 구현 검토나 PR diff 검토를 대신하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | Default Root HTML route와 기존 C11.4 opt-in percentage auto-repeat의 범위 혼동 | **통과.** 계획은 `RuntimeRootHtmlTextPaintV1`/`RootHtmlAutoRepeatV1`만 대상으로 분리한다. |
| 2 | `minmax()` minimum percentage가 max sizing function처럼 취급되어 반복 횟수가 달라짐 | **통과.** flex maximum은 indefinite로 보고 definite percentage minimum을 count 기준으로 삼는다. |
| 3 | percentage basis가 border box, viewport 또는 parent와 혼동 | **통과.** definite repeated-axis content-box를 basis로 고정하고 box edge는 0이다. |
| 4 | 1-track column count에서 gutter 한쪽을 누락 | **통과.** 산식에 numerator `A+G`, denominator `m+G`를 고정한다. |
| 5 | 2-track pattern의 내부 gutter와 repeat 경계 gutter 수 계산 오류 | **통과.** denominator의 `P×G`, numerator의 마지막 gutter 보정을 분리하고 5-repeat scene을 둔다. |
| 6 | 1fr flex distribution을 단순 `min + equal free space`로 계산 | **통과.** min-constrained track freeze 후 잔여 flex tracks에 공간을 분배하는 독립 기대값으로 보완했다. |
| 7 | Chrome 1/64 CSS px serialization 차이를 제품 semantics와 혼동 | **통과.** repeat/track count는 exact, ideal used frame은 0.02px 허용치를 둔다. |
| 8 | 1px UA floor가 실제 0% minimum으로 누출 | **통과.** 0% positive/regression case에서 count와 typed actual track을 따로 단언한다. |
| 9 | 100% 초과 percentage를 count에서 clamp하거나 실제 track을 clamp | **통과.** 120% scene은 count 1과 240px visible overflow를 기대한다. |
| 10 | `auto-fit`에서 empty track collapse 이전/이후에 count를 다시 계산 | **통과.** count 8을 보존하고 placement 이후 7개 empty track collapse를 검사한다. |
| 11 | gap이 percentage로 허용되어 이번 minimum 변경 범위를 넓힘 | **통과.** 기존 non-negative fixed-px gap 계약을 유지한다. |
| 12 | columns만 추가하고 rows의 definite percentage basis를 빠뜨림 | **통과.** 독립 160px definite-height row scene을 계획에 포함했다. |
| 13 | 다른 축의 fixed track requirement 또는 named line metadata가 우회됨 | **통과.** 한 축 repeat·반대 축 unnamed fixed-px 단일 track 조건과 metadata rejection을 유지한다. |
| 14 | CSS source 문자열이나 fixture ID에서 지원 여부를 추정 | **통과.** cascade computed typed value와 기존 owner/revision gate만 사용한다. |
| 15 | 지원 밖 `calc()`, percentage maximum, non-flex max 또는 intrinsic min이 열림 | **통과.** typed grammar를 percentage minimum + `1fr` maximum으로 고정한다. |
| 16 | definite basis가 없거나 0/음수/비유한 값을 count에 사용 | **통과.** positive definite axis, finite/non-negative minimum, checked finite math를 gate로 둔다. |
| 17 | track expansion 10,000 상한이나 integer overflow 후 allocate | **통과.** expansion 전에 checked repeat count와 quota를 검증한다. |
| 18 | item capacity 초과 또는 stale revision이 partial frames를 게시 | **통과.** 기존 auto-repeat atomic publication contract와 source revision 검증을 수용 조건에 포함했다. |
| 19 | independent expected가 Chrome output을 복사하거나 matrix를 검증하지 않음 | **통과.** 산식·source input은 별도 작성했고 `verify-observation.mjs`가 scene, track count, frame을 검사한다. |
| 20 | simulator screenshot을 hardware/WPT/C11 완료 증거로 과장 | **통과.** AVD/Simulator, readback 시점, WPT/general CSS 및 상위 C11 한계를 명시한다. |

## 계획 관문 결과

서로 다른 oracle, percentage basis, repeat-count/gutter 계산, flex minimum freeze, overflow, atomicity, profile 및 검증 범위를 확인했다. 계획은 구현 검토와 PR diff 검토가 별도라는 점을 유지한다. 계획 PR의 실제 변경분은 별도의 30관점 기록으로 검토한다.

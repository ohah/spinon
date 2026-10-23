# C11.4 definite percentage auto-repeat neighbor 계획 검토 20개

계획 구현 전에 별도 실패 관점 20개를 검토했다. 각 항목은 다른 입력 경계 또는 invariant를 다룬다. 구현 후 runtime 검토와 PR 변경 검토는 이 표를 재사용하지 않는다.

| # | 실패 관점 | 계획에서 고정한 대응 |
|---:|---|---|
| 1 | percentage neighbor를 outer border-box basis로 계산 | border/padding을 뺀 116px content-basis controls를 둔다. |
| 2 | columns basis를 rows에도 재사용 | row/column scene을 독립 oracle로 둔다. |
| 3 | prefix와 suffix 합계를 repeat count에서 누락 | 양쪽 neighbor scene을 만들고 full track list를 비교한다. |
| 4 | percent neighbor는 count 산정에 들어가지만 gutter를 빠뜨림 | gap 4px를 고정하고 W3C formula에서 매 gutter를 포함한다. |
| 5 | auto-fill 대신 auto-fit의 collapsed tracks로 count를 계산 | auto-fit count와 post-placement collapse를 별도로 검사한다. |
| 6 | 빈 auto-fit에서 반복 수가 1로 줄어듦 | 빈 column/row 장면에서도 expanded count 4와 0px used tracks를 검사한다. |
| 7 | item 하나가 남은 auto-fit gutter를 잘못 보존 | first repeated track item과 collapsed neighbor tracks를 함께 고정한다. |
| 8 | percentage neighbor 합이 content axis보다 커서 count가 0이 됨 | 80%+30% overfill controls에서 규범상 최소 1회를 요구한다. |
| 9 | zero percentage neighbor를 UA 1px repeat-floor에 포함 | 0% neighbor와 40px repeated minimum을 분리해서 확인한다. |
| 10 | prefix-only와 suffix-only 위치가 다르게 계산됨 | 두 one-sided scene을 서로 다른 axes에서 비교한다. |
| 11 | percent size가 indefinite/cyclic인 상태에서 지원으로 열림 | 해당 축 definite gate를 필수로 하고 invalid basis를 negative control로 둔다. |
| 12 | negative, NaN 또는 infinite percent가 허용됨 | finite nonnegative preflight를 유지하고 constructed invalid layout inputs를 거부한다. |
| 13 | unsupported intrinsic/fraction/calc neighbors가 함께 허용됨 | 이 값들을 명시적으로 제외하고 기존 typed failure를 회귀 검증한다. |
| 14 | Taffy가 percentage neighbor를 같은 basis로 resolve하지 않음 | 의존성 source를 먼저 추적하고 13-scene count/track 결과를 검사한다. |
| 15 | floating-point quantization 차이를 수치 정확성으로 오판 | count/order는 exact, fractional tracks/frame은 1/64px 기준을 쓴다. |
| 16 | placement/style가 flex used-size를 바꿔 count 관찰을 흐림 | auto-fill/auto-fit 장면의 item placement를 고정해 count와 sizing을 각각 비교한다. |
| 17 | border-box scene에서 136px를 percentage basis로 잘못 사용 | expected basis 116px와 column·row controls를 둔다. |
| 18 | simulator geometry가 browser/runtime parity를 과장 | Chrome oracle, Rust layout 결과, Android/iOS runtime trace와 surface sample을 분리한다. |
| 19 | fixture 심볼이 제품 artifact에 들어감 | 기본 artifact negative와 opt-in fixture positive control을 유지한다. |
| 20 | 고정 13 scene을 전체 percentage track/auto-repeat 지원으로 확대 해석 | calc, indefinite/cyclic, intrinsic tracks, WPT와 일반 CSS admission을 제외 범위에 쓴다. |

기대값은 [사전 비교 기록](../spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-precomparison-2026-10-24/README.md), 기준은 [W3C CSS Grid 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat)다.

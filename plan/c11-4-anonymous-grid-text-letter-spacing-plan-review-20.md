# C11.4 anonymous Grid text `letter-spacing` 계획 적대 검토 20개

이 표는 코드를 바꾸기 전 계획의 범위, oracle, line-edge 계산, runtime 경계에서 서로 다른 실패 경로를 검토한다. 구현/runtime이나 PR diff 검토를 대신하지 않는다.

| # | 실패 관점 | 계획의 방어 또는 처리 | 판정 |
| ---: | --- | --- | --- |
| 1 | Chrome 결과를 CSS 규범 결과로 혼동 | CSS Text Level 3의 adjacent-unit 및 line-edge 규칙을 expected value로 고정하고 Chrome 156은 별도 관측으로 보존 | 통과 |
| 2 | 한 글자당 spacing을 양 끝에 모두 포함 | `XX`의 규범 기대값에서 한 내부 경계만 계산하고 Chrome의 18/22 차이를 differential로 남김 | 통과 |
| 3 | max-content에서 공백 TCU와 양쪽 인접 경계를 누락 | 5개 TCU의 4개 내부 경계에 적용해 `50 + 4s`를 독립 산출 | 통과 |
| 4 | min-content에서 두 단어를 한 폭으로 계산 | normal wrapping의 soft opportunity에서 나눈 두 `XX` line을 독립 계산 | 통과 |
| 5 | 음수 spacing으로 min-content width가 음수가 됨 | 지원 범위를 finite -2px까지 제한하고 nonnegative intrinsic size invariant를 회귀 검증 | 통과 |
| 6 | fractional px가 정수로 반올림됨 | 0.5px 기대값을 min/max 및 simulator scene에서 유지 | 통과 |
| 7 | `normal`과 computed `0px`가 다르게 처리 | 두 입력을 동일한 zero scalar로 normalize하고 기존 getComputedStyle 표현은 별도 유지 | 통과 |
| 8 | Stylo computed CSS px serialization을 잘못 parse | `normal` 또는 허용된 finite px grammar만 받고 `%`, em, calc와 잔여 문자열은 거부 | 통과 |
| 9 | source node나 style revision이 달라진 값이 적용 | 정확한 Grid root identity와 같은 computed snapshot revision을 연결하고 stale input negative control 추가 | 통과 |
| 10 | Parley 0.12 atom-after spacing이 줄 끝을 남김 | 한 줄 폭과 줄별 min-content 폭에서 trailing spacing 보정을 명시하고 1px 대조를 사용 | 통과 |
| 11 | negative spacing edge 보정 부호가 반대 | -1px 입력을 통해 줄 끝 spacing 제거 시 폭이 18에서 19로 회복되는지 확인 | 통과 |
| 12 | 빈 forced-break line에서 spacing을 빼거나 line count를 합침 | 보정은 비어 있지 않은 line에만 적용하고 block-size/line-count controls를 유지 | 통과 |
| 13 | collapsed whitespace를 hanging whitespace와 혼동 | 현재 fixture는 `XX XX`만 사용하며 inline/hanging contribution은 spacing correction에 포함하지 않음 | 통과 |
| 14 | inline box width를 문자 spacing처럼 다시 보정 | 범위에서 inline spans/replaced items를 제외하고 기존 inline-box accounting 회귀 실행 | 통과 |
| 15 | bidi visual edges에서 ASCII LTR 결과를 일반화 | 방향은 LTR로 고정하고 RTL·bidi를 명시적으로 제외 | 통과 |
| 16 | 새 style field의 implicit 값이 과거 callers에서 바뀜 | 기존 모든 ParagraphStyle initializer를 0px로 명시하고 normal/기존 C11.4 regression 유지 | 통과 |
| 17 | spacing 지원이 C15 또는 다른 Grid fixture로 새어 나감 | computed style adapter는 지정 anonymous-text scene만 허용하고 다른 profiles는 normal-only gate 유지 | 통과 |
| 18 | parse overflow/non-finite 값 뒤 partial runtime state 노출 | finite/range 검증을 layout publication 전에 수행하고 no-frame negative control 추가 | 통과 |
| 19 | opt-in fixture ABI가 기본 제품 산출물에 포함 | default artifact negative와 fixture-test positive symbol/payload control을 각각 빌드 | 통과 |
| 20 | simulator 및 제한 matrix를 전체 CSS/C11 완료로 승격 | AVD·Simulator 및 미검증 범위를 문서화하고 STATUS checkbox/count를 변경하지 않음 | 통과 |

계획에서 browser observation만 통과해야 한다고 쓰는 초안을 CSS Text Level 3의 줄 끝 규칙과 대조했다. 이를 고쳐 규범 expected values를 판정 기준으로 두고, Chrome 156의 trailing-spacing 차이를 알려진 상호운용성 차이로 남겼다. 각 관점은 구현 후 별도 코드/runtime 검토에서 새 입력으로 다시 검토한다.

# C11.4 safe/unsafe center 계획 실패 관점 검토 20개

검토 대상은 [후속 계획](./c11-root-html-auto-row-safe-center.md)과 그 Chrome 사전 비교다. 각 항목은 서로 다른 입력·계약 경계로 검토했다. Blocking finding은 없고, 아래 제한을 계획의 수용 기준으로 고정했다.

| # | 별도 실패 관점 | 검토 결과 / 고정한 예방 기준 |
|---:|---|---|
| 1 | W3C draft를 최종 표준이나 제품 지원 선언으로 오인 | 2026-10-08 Working Draft와 at-risk 상태를 계획에 명시했다. |
| 2 | `safe` overflow fallback의 방향을 반대로 해석 | 명세상 flex-start, Grid에서는 start로 고정했다. |
| 3 | `unsafe`도 overflow에서 start로 이동시키는 구현 | `unsafe center`의 negative free-space center 좌표를 별도 expected로 고정했다. |
| 4 | positive-space에서 안전성 modifier가 불필요하게 geometry 변경 | 세 값의 positive control `[5,15,25]`를 유지한다. |
| 5 | `center` 기존 결과를 바꾸거나 safe/unsafe로 치환 | plain `center` overflow 결과를 기존 동작 regression으로 보존한다. |
| 6 | `<length-percentage>`에서 percentage basis를 inline axis로 계산 | definite 40px block-size basis로 46px을 계산한다. |
| 7 | `calc(120% - 2px)` 항 순서·단위 변환 혼동 | 고정 source와 Chrome computed serialization을 기록한다. |
| 8 | gap을 포함하지 않아 overflow 여부를 잘못 판단 | alignment subject 3 tracks + 2 gutters = 122px로 계산한다. |
| 9 | `safe` fallback 때 track sizing 자체를 줄임 | tracks `[10,10,10]`와 46px gutters 유지, alignment offset만 바꾼다. |
| 10 | overflow group centering의 부호 오류 | negative free-space `-82/2 = -41px`를 명시했다. |
| 11 | child positions 대신 grid border box를 비교 | 세 child y offsets를 수용 조건으로 정의한다. |
| 12 | fixture가 item count/order를 달리해 기존 profile 밖으로 확장 | empty fixed item 후 text A/B의 세 direct child를 고정한다. |
| 13 | hidden/whitespace text 처리 차이로 implicit rows 증가 | canonical fixture topology와 visible direct-item 조건을 유지한다. |
| 14 | writing-mode 또는 direction 별도 규칙을 우연히 포함 | 기본 horizontal writing mode만 대상으로 제한한다. |
| 15 | scroll safety default와 explicit `safe` 의미 혼합 | scroll container/default behavior는 제외하고 명시 modifier만 비교한다. |
| 16 | computed serialization에 공백·토큰 순서 차이 | `safe center`와 `unsafe center` 정확 문자열 외에는 닫아 둔다. |
| 17 | generic parser 지원을 Root HTML profile 지원으로 오인 | profile allowlist와 runtime route의 독립 검증을 요구한다. |
| 18 | parser가 safety는 전달하지만 Taffy conversion에서 유실 | `ContentAlignmentPosition` 및 `AlignmentSafety` 전파를 focused layout test로 증명한다. |
| 19 | 브라우저 관측 자체를 기대값으로 재사용 | 자유 공간과 alignment offset의 독립 계산을 별도 JSON에 기록했다. |
| 20 | screenshot 또는 simulator 실행을 pixel parity/WPT로 과장 | 화면은 visual evidence로 한정하고 surface readback·WPT·실기기 주장을 금지했다. |

## 판정

계획은 `safe center` overflow fallback과 `unsafe center` 유지 동작을 구별하고 기존 C11.4 profile·오류 경계를 보존하도록 한정됐다. 구현 전 Chrome/독립 기대값 자료가 준비되어 있다. 계획 검토만 통과했으며 구현/runtime 검토는 아직 수행하지 않았다.

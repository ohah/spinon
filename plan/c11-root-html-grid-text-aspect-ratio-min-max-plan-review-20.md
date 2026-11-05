# C11.1 direct-text `aspect-ratio` min/max 계획 적대 검토 20개

**대상:** [계획](./c11-root-html-grid-text-aspect-ratio-min-max.md), [Chrome 156 precomparison](../spec/internal/evidence/c11-root-html-grid-text-aspect-ratio-min-max-precomparison-2026-11-05/README.md), 저장소 fixture와 독립 expected JSON.

| # | 실패 관점 | 검토와 판정 |
|---:|---|---|
| 1 | 기본 Root HTML과 C11.4 opt-in 구현을 같은 지원 증거로 취급 | 계획은 두 route와 증거를 분리한다. 통과. |
| 2 | browser-only screenshot을 geometry 기준으로 사용 | JSON은 DOMRect, computed tracks/properties를 기록하고 독립 expected와 비교한다. 통과. |
| 3 | definite width에서 height ratio 전이의 방향을 뒤집음 | `60px / 2 = 30px`과 `min-height:40`·`max-height:20` 장면이 있다. 통과. |
| 4 | definite height에서 width ratio 전이의 방향을 뒤집음 | `30px × 2 = 60px`과 `min-width:70`·`max-width:50` 장면이 있다. 통과. |
| 5 | min과 max의 동시 조건에서 max를 우선 처리 | 양 축에 `min > max` 장면을 두고 Chrome 결과 `min` 우선을 독립 계산으로 고정한다. 통과. |
| 6 | no-ratio item에도 ratio 전이를 강제로 적용 | width/height 별 no-ratio control이 ratio scenes와 별도다. 통과. |
| 7 | system font 차이를 ratio 지원 차이로 오판 | ratio 없는 `height-control-max-width`의 intrinsic 폭은 Chrome 기준 자료로만 두고 simulator equality gate에서 제외한다. 통과. |
| 8 | Grid owner의 dimensions나 min/max까지 조용히 허용 | owner min/max는 기존 제한을 유지하고 projection negative control을 계획했다. 통과. |
| 9 | ratio가 없는 `A` sibling을 ratio owner로 취급 | item 개수·DOM 순서·문자 내용을 모두 고정하고 ratio 대상은 `AAAA` NodeId만 허용한다. 통과. |
| 10 | 다른 ratio 값에 계획 범위를 넓힘 | min/max와 함께 허용하는 ratio는 typed serialization 모두 `2/1`로 제한한다. 기존 unconstrained ratio subset은 회귀로 유지한다. 통과. |
| 11 | min/max CSS serialization과 typed value가 서로 다른 snapshot | NodeId·owner·style revision coherence와 속성별 오류를 계획에 포함한다. 통과. |
| 12 | `auto`/`none` 초기값을 0px과 혼동 | min/max typed defaults와 computed longhands를 scene별로 검사한다. 통과. |
| 13 | 음수/NaN/무한 값이나 70px 초과가 제한 범위로 통과 | finite `0..=70px` 외 typed value를 layout 전 원자 거부하도록 한다. 통과. |
| 14 | 유효한 percentage 또는 calc CSS를 문법 오류로 취급 | valid-but-out-of-profile computed value를 property-bound rejection으로 구분하며 syntax-invalid fallback은 별도 control이다. 통과. |
| 15 | track 내부 60px 경계에서 item overflow를 잘라냄 | `min-width:70px`의 x=10, width=70, Grid x=0,width=70을 고정해 10px overflow를 검증한다. 통과. |
| 16 | row auto sizing이 sibling `A` height를 누락 | 10개 scenes 모두 auto row와 두 direct items를 포함하고 root/row heights를 expected에 둔다. 통과. |
| 17 | row/column track 또는 x/y 좌표를 교환 | 10개 관측 모두 `[10px,60px]` columns 및 cumulative y, item x=10을 비교한다. 통과. |
| 18 | unsupported writing mode, direction, alignment, edge, gap이 positive로 유입 | 기존 horizontal-tb/LTR, start alignment, zero edge/gap gate를 계속 요구한다. 통과. |
| 19 | layout failure 뒤 이전 revision의 partial frame을 재사용 | stale revision 및 다음 valid revision recovery/atomic rejection을 수용 기준에 포함한다. 통과. |
| 20 | simulator 또는 limited slice 결과를 public CSS/WPT 완료로 확대 | simulator/readback 층을 구분하고 C11.1/C11 및 `78/214` 상위 상태를 유지한다. 통과. |

**결과:** 이 20개 관점은 계획 검토다. 구현 code review와 PR diff review에 재사용하지 않는다.

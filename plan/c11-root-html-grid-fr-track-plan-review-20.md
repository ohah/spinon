# C11.1 parsed Root HTML Grid `fr` 계획 실패 관점 검토 20회

검토 대상은 [`c11-root-html-grid-fr-track.md`](./c11-root-html-grid-fr-track.md), 고정 HTML fixture, Chrome 156 관측과 독립 기대값이다. 각 항목은 서로 다른 실패 원인과 구현 gate를 확인했다. 이 문서는 코드 구현 검토나 PR diff 검토를 대신하지 않는다.

| 회차 | 적대 관점 | 판단과 계획 반영 |
|---:|---|---|
| 1 | fraction 허용이 Root HTML 이외의 computed-style profile까지 새지 않는가 | `RuntimeRootHtmlTextPaintV1`만 허용한다. Generic projection과 다른 profile은 그대로 둔다. 구현 test에서 profile별 실패를 확인한다. |
| 2 | 일반 Grid converter가 허용하는 `minmax()`·intrinsic track이 direct `fr` 경로를 우회하지 않는가 | direct `LengthPx`/`Fraction` enum만 허용하고, `minmax()`·intrinsic track은 실패시킨다. |
| 3 | 고정 named-area slice가 fraction track과 조합되어 기존보다 넓어지지 않는가 | named-area validator의 현재 exact fixed-px 조건을 유지하고 areas+fraction negative를 둔다. |
| 4 | negative, NaN, infinity factor가 계산기로 전달되지 않는가 | 모든 direct factor를 finite non-negative로 검사하고 각 비정상 값을 별도 negative로 둔다. |
| 5 | `0fr`가 0 나눗셈이나 다음 track의 잘못된 offset을 만들지 않는가 | `0fr 1fr`에서 `[0,120]`과 겹치는 zero-width item frame을 기대값으로 고정한다. |
| 6 | flex factor 합이 1보다 작은 경우 남는 공간이 잘못 재분배되지 않는가 | `.25fr .25fr`의 `[25,25]` 및 50px 잔여를 specification과 독립 계산으로 검사한다. |
| 7 | f32 flex-factor 누산 경계를 실제로 넘는 회귀 입력인가 | 초기 `1e38fr` 두 개는 합이 `2e38`이라 f32 overflow가 아니었다. 이를 제거하고 세 개의 유한 positive control과 네 개의 overflow divergence 입력으로 수정했다. |
| 8 | 세 factor 입력의 개별 값이 parser에서 이미 바뀌어 네 factor 합만의 차이라는 해석이 잘못되지 않는가 | 세 factor Chrome control `[40,40,40]`과 네 factor observation을 나란히 고정한다. 제품 test는 직접 typed factor도 확인한다. |
| 9 | f64 누산 뒤 f32 flex fraction cast 또는 factor 곱셈에서 0·infinity가 다시 생기지 않는가 | Taffy 변경은 누산·나눗셈만 넓은 정밀도로 처리하고, 결과 및 최종 track size의 finite·non-negative 변환 검사를 구현 gate로 둔다. |
| 10 | row 계산에서 width를, column 계산에서 height를 잘못 기준으로 쓰지 않는가 | column/row positive fixture를 각각 두고 동일 computed element snapshot의 해당 axis를 확인한다. |
| 11 | `min/max-size` clamp 또는 `aspect-ratio`가 definite basis를 바꾸지 않는가 | flex 축은 computed explicit LengthPx, min-size auto, max-size none, aspect ratio absent만 수용하고 각 다른 입력을 거부한다. |
| 12 | padding, border, margin 또는 `box-sizing` 차이가 flex free space를 바꾸지 않는가 | 현재 Root HTML box subset의 zero edge 조건을 유지하고 직접 width/height 기준을 사용한다. |
| 13 | gap을 두 번 빼거나 track/item 좌표에 사후 보정하지 않는가 | mixed px/fr와 row gap 기대식에서 gap을 한 번만 차감하고 item frame도 같이 비교한다. |
| 14 | 이름·selector·semantic child 순서를 성공 조건으로 하드코딩하지 않는가 | rename/reverse control에서 track 비율과 source-order auto-placement를 별개로 검증한다. |
| 15 | explicit capacity 바깥 item이 implicit track을 조용히 생성하지 않는가 | implicit item count 초과는 layout projection 전 거부하며 partial frame이 발행되지 않는지 검사한다. |
| 16 | CSS 문법 오류가 이전 computed style fallback처럼 보여 positive로 집계되지 않는가 | `minmax()`와 named area controls는 CSS-valid인지 브라우저에서 확인하고 typed runtime rejection을 별도 검증한다. |
| 17 | stale style/layout revision이나 다른 root의 size가 definite basis로 섞이지 않는가 | owner/root identity와 style/layout revision을 같은 snapshot에서 대조하고 mismatch diagnostics를 확인한다. |
| 18 | allocation/layout error 뒤 일부 scene 또는 이전 cache가 publish되지 않는가 | allocation failpoint와 layout 오류에서 scene·frame 발행이 원자적으로 실패하는지 검증한다. |
| 19 | 고정 scene·FFI adapter가 제품 artifact에 들어가지 않는가 | 기본/제품 artifact와 opt-in fixture-test artifact를 양쪽 빌드로 검사하며 scene ID·symbol·payload의 경계를 기록한다. |
| 20 | Chrome 156의 관측 오류가 명세 기대값으로 둔갑하거나 반대로 browser와 달라 구현이 잘못됐다고 판단하지 않는가 | W3C §11.7.1의 `[30,30,30,30]`과 Chrome 관측 `[0,0,0,0]`을 별도 기록한다. Chrome/Chromium source revision 불일치는 명시하고 구현 기대는 규범 알고리즘으로 고정한다. |

## 검토 결과

초기 큰-factor oracle의 두-track 입력이 f32 누산 overflow를 유발하지 않는 결함을 발견해, 세-track positive control과 네-track Chrome/spec divergence로 바꿨다. 계획은 이 차이를 분리하고 Taffy 산술 변환 gate를 추가한 뒤 구현 입력으로 사용할 수 있다. 아직 실행하지 않은 구현 tests, Android/iOS simulator 검증, 내부 계약/상태 문서 갱신 및 구현 PR diff 검토는 완료로 간주하지 않는다.

# C11.4 four-member intrinsic auto-row 계획 · 실패 관점 20개

검토 대상은 [전용 계획](./c11-4-root-html-fit-min-max-content-auto-rows.md)과 96-scene Chrome 157 oracle이다. 이 검토는 구현 전에 했으며 구현/runtime 또는 PR diff 검토에 재사용하지 않는다.

## 구현 후 계획 수정

11번의 최초 요구는 모든 authored `calc()` cap을 거부하는 것이었지만 computed-value 경계와 맞지 않았다. RuntimeSession 관측에서 Stylo가 `fit-content(calc(12px + 8px))`를 typed `FitContent(LengthPx(20))`로 정규화하고 이 profile에 도달하기 전에 `fit-content(20px)`로 직렬화하는 점을 확인했다. 따라서 제한 계약은 absolute normalization 뒤 computed finite non-negative px cap을 허용하고 unresolved mixed-percentage `calc()`를 거부한다. 이 수정은 [별도 계획 수정 관점 20개](./c11-4-root-html-fit-min-max-content-auto-rows-cap-normalization-review-20.md)로 검토했다. 아래 11번은 구현 전 예상이며 cap 관련 부분은 이 수정으로 대체한다.

| # | 적대적 실패 경로 | 계획에서 요구한 방어 | 결과 |
|---:|---|---|---|
| 1 | 네 값 조합 지원을 임의 track-list 지원으로 설명한다 | fit-content·min-content·max-content·auto 하나씩의 24개 순서만 admission한다. | PASS |
| 2 | 필요한 member 하나가 빠지고 다른 값이 중복된다 | 중복·누락·추가 member를 거부하고 정확한 순열 수를 확인한다. | PASS |
| 3 | fixed payload geometry가 같다는 이유로 max-content를 min-content로 투영한다 | frame과 별개로 computed/typed enum identity를 검사한다. | PASS |
| 4 | min/max geometry가 같다는 관측을 두 intrinsic algorithm의 증거로 주장한다 | 이 fixture의 min/max contribution이 같다고 기록하고 알고리즘 동등성은 주장하지 않는다. | PASS |
| 5 | style projection이 track 순서를 정렬하거나 canonicalize한다 | authored/computed order 문자열 24개와 typed list order를 비교한다. | PASS |
| 6 | 반복 목록이 첫 implicit row가 아니라 explicit row 뒤의 다른 index에서 시작한다 | explicit row 하나를 두고 다섯 번째 implicit row가 첫 member를 반복하는지 확인한다. | PASS |
| 7 | 두 같은 track의 content가 달라 cycle 결과 비교가 무효다 | implicit row 1과 5에 같은 20px payload를 사용하고 수정된 fixture를 별도로 고정한다. | PASS |
| 8 | 초기 잘못된 cycle fixture를 최종 근거로 혼동한다 | pass01은 보존하되 superseded로 표시하고 pass02 fixture hash만 oracle로 인정한다. | PASS |
| 9 | percentage cap basis가 indefinite이거나 viewport height에 잘못 연결된다 | definite 100px Grid block-size를 고정하고 `20%`와 `20px`를 대조한다. | PASS |
| 10 | percentage 정규화가 computed authored type을 지운다 | computed `fit-content(20%)` identity와 px/% geometry 동등성을 각각 확인한다. | PASS |
| 11 | 모든 authored `calc()`가 computed-style 경계에서 구별 가능하다고 가정한다 | 실제 computed typed 경계를 따라 absolute calc의 px normalization은 허용하고 unresolved mixed percentage calc는 거부한다. | 수정 후 PASS |
| 12 | 음수·NaN·무한대·표현 불가능 cap이 layout에 들어간다 | finite non-negative validation과 property-attributed fail-closed 입력을 둔다. | PASS |
| 13 | stretch가 max-content·min-content·fit-content track에 공간을 배분한다 | `start`와 `stretch`를 대조해 computed auto max-sizing 위치만 달라지는지 확인한다. | PASS |
| 14 | 반복 목록의 선두 auto track이 잘못된 공간을 받는다 | 선두 auto 순서를 포함해 두 auto track이 각각 22px free space 중 11px을 받는지 검사한다. | PASS |
| 15 | stretch에서 positive free space가 두 번 소진되거나 남는다 | stretch 총 row는 100px, start 총 row는 78px이어야 한다. | PASS |
| 16 | item source order나 explicit-row identity가 밀려 frame 기대값이 바뀐다 | explicit frame, 다섯 item/payload frame과 row position을 기록한다. | PASS |
| 17 | computed track list와 resolved `grid-template-rows`를 혼동한다 | `computedRows`와 used `gridTemplateRows`를 별도로 저장하고 검증한다. | PASS |
| 18 | 다른 Grid owner·nested/unsupported topology가 이 profile을 상속한다 | 단일 root owner, direct-child topology, fixed explicit tracks와 revision gate를 유지한다. | PASS |
| 19 | 거부 revision이 partial layout을 publish하거나 세션을 망가뜨린다 | 구현 검사에서 partial layout/draw 부재와 same-session recovery를 요구한다. | PASS |
| 20 | Chrome/static fixture 결과를 제품 지원이나 C11 완료로 확대한다 | Android/iOS 기본 route 근거와 제외 범위를 유지하고 C11.4/C11 및 `78/214`를 그대로 둔다. | PASS |

## 검토 수정 기록

pass01은 implicit row 5에 6px payload를 썼고 row 1 payload는 20px이었다. 서로 다른 contribution인데도 첫 cycle row의 used size가 같아야 한다고 assertion해 계획의 첫 cycle 검사가 잘못됐다. 수정 후 두 반복 위치 모두 20px payload를 사용하며 pass02에서 같은 첫 track의 두 row size를 독립 비교한다. pass01 artifact는 superseded 표식을 유지하고 96/96 결과에 포함하지 않는다.

## 판정

계획은 지원 경계, oracle, 독립 기대값, cycle 불변식과 platform/runtime 관문을 구현 전에 고정했다. 계획 단계의 미해결 blocker는 없다.

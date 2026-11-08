# C11.4 repeated intrinsic implicit-row list 계획 실패 관점 검토 20개

검토 대상은 [`c11-4-root-html-repeated-intrinsic-auto-rows.md`](./c11-4-root-html-repeated-intrinsic-auto-rows.md)와 독립 Chrome 사전 비교다. 구현 전 검토이며 코드 검토나 runtime 검증으로 재사용하지 않는다.

| # | 독립 실패 관점 | 판정과 반영 |
| --- | --- | --- |
| 1 | CSS 문법이 목록 안에서 track-size 반복을 허용하지 않는 경우 | 통과. CSS Grid 2 §7.6은 `grid-auto-rows` 값으로 `<track-size>+`를 정의한다. 개별 track size를 같은 목록에서 두 번 쓰는 것은 유효하다. |
| 2 | implicit row의 첫 track이 explicit row 다음에 잘못된 목록 member로 시작하는 경우 | 통과. 기대 모델은 10px explicit row를 제외하고 첫 implicit row에 첫 `fit-content()` member를 둔다. Chrome computed/used output이 이를 확인한다. |
| 3 | 두 번째 여섯-member cycle의 시작 index가 한 칸 밀리는 경우 | 통과. 여덟 implicit row에서 row 7은 `fit-content()`, row 8은 반복 `min-content`다. 네 scene 모두 동일 순서를 반환한다. |
| 4 | 3·4-member 기존 목록이 새 6-member exact matcher 때문에 거부되는 경우 | 계획에 기존 3-member·4-member 긍정 회귀와 singleton profile 회귀를 명시했다. 새 matcher를 별도 exact predicate로 유지한다. |
| 5 | 다른 여섯-member 순서나 누락·추가 member가 새 경로에 우연히 승인되는 경우 | 통과. 지원 값은 `[fit-content, min-content, max-content, auto, min-content, max-content]` 하나뿐이며 다른 순서는 fail-closed다. |
| 6 | 중복 `min-content` 두 track이 하나의 contribution/frame으로 합쳐지는 경우 | 통과. row 2와 row 5의 고정 payload는 각각 10px·6px로 서로 다른 used size를 기대한다. item index별 track/frame을 확인한다. |
| 7 | 중복 `max-content` 두 track이 같은 track state를 공유하는 경우 | 통과. row 3과 row 6은 12px·6px payload를 사용하며 각각 12px·6px여야 한다. |
| 8 | percentage cap이 definite block-size가 아닌 다른 basis를 쓰는 경우 | 통과. root height는 100px definite이고 box sizing은 border-box다. 20% cap 기대값은 20px로 고정했다. |
| 9 | px cap과 percentage cap이 같은 조건에서 다른 track geometry를 만드는 경우 | 통과. `20px`와 `20%` 네 scene의 used rows를 독립 기대값과 비교하며 Chrome 4/4 exact다. |
| 10 | fit-content cap이 direct px/percentage가 아닌 unresolved math까지 암묵적으로 넓어지는 경우 | 통과. finite non-negative direct px/percentage만 허용하고 unresolved `calc()`는 명시적으로 범위 밖이다. |
| 11 | 60px payload 때문에 반복 fit-content track이 60px로 커지는 경우 | 통과. row 7은 20px cap을 유지하고 payload만 overflow한다. Chrome item/payload frame에서 row20px·payload60px를 확인했다. |
| 12 | overflow가 grid border에서 잘리거나 계산된 scroll/clip 의미로 오해되는 경우 | 통과. root `overflow:visible`이며 payload bottom은 root bottom 바깥 30px(start)·34px(stretch)다. clipping/scrolling 지원은 주장하지 않는다. |
| 13 | `start`에서 남은 4px가 intrinsic track에 배분되는 경우 | 통과. start tracks 합계는 96px이고 4px free space를 남긴다. intrinsic/fit-content row의 값은 Chrome 관측과 일치한다. |
| 14 | `stretch`가 모든 track에 남은 공간을 배분하는 경우 | 통과. stretch는 `auto` row 하나만 6px에서 10px로 늘린다. 나머지 row sizes는 start와 동일하다. |
| 15 | 자연 track 합계가 컨테이너보다 커 음수 free-space 경로가 섞이는 경우 | 통과. payload `[20,10,12,6,6,6,60,6]`과 cap으로 track 합계96px을 유지하며 4px의 양의 여유를 명시한다. |
| 16 | 첫 explicit row item이 implicit list 항목 수나 cycle index에 포함되는 경우 | 통과. explicit row는 빈 fixed 10px item 하나이며 8개의 intrinsic item과 분리한다. expected rows는 explicit track을 첫 값으로 고정한다. |
| 17 | 8 implicit items가 모두 실제 grid auto-placement row를 생성하지 않는 경우 | 통과. fixture 관측은 explicit+8 implicit item을 모두 기록하며 used gridTemplateRows가 9개인지 검증한다. |
| 18 | sibling/추가 owner, text·image·nested Grid가 제한 topology 안에 섞이는 경우 | 통과. single owner, direct HTML div, 8개 payload topology를 요구하고 나머지는 fail-closed 및 negative test 대상으로 둔다. |
| 19 | computed style과 typed layout input 사이에서 member order/type identity가 바뀌는 경우 | 통과. 계획은 computed string과 typed `GridAutoRows::TrackList` 양쪽에서 exact order를 확인하고 layout geometry를 별도로 검증한다. |
| 20 | 이 slice 결과가 일반 CSS 지원, WPT 또는 C11.4 완료로 과대 표시되는 경우 | 통과. public admission/WPT/전체 intrinsic contribution은 범위 밖이며 C11.4/C11 체크와 공식 `78/214`를 유지한다. |

## 검토 결론

20개 계획 실패 관점에서 사전 비교의 계산·owner·cascade·layout·overflow·status 경계를 구분했다. 계획에서 수정해야 할 누락은 발견되지 않았다. 이 기록은 구현/runtime 리뷰나 구현 PR diff 검토를 대신하지 않는다.

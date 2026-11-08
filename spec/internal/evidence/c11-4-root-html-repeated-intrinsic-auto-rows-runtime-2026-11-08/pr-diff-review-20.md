# 반복 intrinsic implicit-row 목록 PR 변경분 실패 관점 검토 20개

검토 대상은 candidate branch의 code, regression tests, runtime fixture, plan, status, internal contract, public roadmap, simulator scripts와 evidence다. 계획 검토 및 구현/runtime 검토와 별도로 검사했다.

| # | 독립 실패 경로 | 판정과 근거 |
| --- | --- | --- |
| 1 | Style projection은 여섯 member를 허용하지만 Layout matcher가 다른 typed order를 받는 계약 불일치 | 통과. 양 계층이 동일한 exact sequence를 각각 비교하고 Style projection/Layout tests 모두 exact order와 type을 확인한다. |
| 2 | 두 번째 `min-content`나 `max-content`를 덜/더 허용하는 matcher 오타 | 통과. 각 인덱스의 enum을 고정하며 duplicate·missing negative controls가 있다. |
| 3 | list length가 5/7이거나 arbitrary trailing track을 받는 경계 누락 | 통과. positive six-length only predicate와 short/long Style rejection이 확인된다. |
| 4 | Style projection에서 9개 child, Layout에서 다른 개수를 허용하는 topology 분기 차이 | 통과. 두 계층이 six-member profile의 9 visible direct children을 별도 검증한다. |
| 5 | `min-height:0` 예외가 다른 C11.4 profile이나 explicit item에 새어 나감 | 통과. Layout guard는 9-child, item positions 1..=8, zero fixed dimension, exact six-member tracks를 함께 요구한다. |
| 6 | 숨김 또는 다른 owner의 Grid item이 direct child count에 섞임 | 통과. 기존 visible-child collection과 single-owner/revision validation을 재사용하고 실패는 node/property에 귀속한다. |
| 7 | non-finite/negative fit-content argument가 new list path에서 허용됨 | 통과. cap predicate 재사용과 음수/NaN/Infinity projection negative tests가 이를 막는다. |
| 8 | percentage cap이 indefinite axis 또는 다른 basis로 해석됨 | 통과. 기존 definite 100px Grid block-size profile을 요구하고 20% positive control이 projected percentage identity를 보존한다. |
| 9 | `0%`·`120%`가 parse/projection 과정에서 clamp되어 authored computed value와 달라짐 | 통과. projection test에서 두 값을 그대로 typed layout cap으로 확인한다. |
| 10 | six-member payload vector에서 position index가 explicit row 기준 한 칸 어긋남 | 통과. Layout test는 8개 item/payload의 각 y/height를 비교하고 RuntimeSession은 색상별 paint frame을 대조한다. |
| 11 | 7번째 item의 60px payload가 첫 fit-content cycle에도 허용됨 | 통과. 첫 fit-content position의 60px height negative control이 실패를 요구한다. |
| 12 | fit-content가 두 번째 cycle에서는 60px content에 따라 확장됨 | 통과. 7번째 implicit row는 20px로 유지되고 payload의 30/34px visible overflow가 start/stretch에서 확인된다. |
| 13 | `stretch`가 `min-content`·`max-content`까지 늘리거나 전체 합계가 잘못됨 | 통과. independent `start` 96px 및 `stretch` 100px rows와 auto row의 +4px만 exact compare한다. |
| 14 | 네-member·세-member·singleton 기존 profile의 validation/payload semantics가 바뀜 | 통과. 새 predicate 분리와 기존 경로의 회귀 test 79개를 확인했다. |
| 15 | RuntimeSession failure가 partial scene을 발행하거나 same-session recovery를 막음 | 통과. 잘못된 dark media order가 `grid-auto-rows`/Grid node로 거부되고 completion이 없으며 valid light revision이 같은 session에서 복구된다. |
| 16 | fixture input이 CSS collector가 지원하지 않는 shorthand 또는 payload shape에 의존 | 통과. `border:0` shorthand를 제거했고 payload width는 `auto`; explicit item에는 `min-height:0` class를 재사용하지 않는다. |
| 17 | Android/iOS에서 다른 `index.html` 또는 fixture build artifact를 실행 | 통과. fixture bytes 3007와 SHA `77874483…d231f`를 APK asset·Simulator app bundle과 runtime source log에 대조한다. |
| 18 | simulator capture의 검증기가 잘못된 기대 box 수를 다시 기록 | 통과. 최초 18-box harness 오류를 보존하고 pass02 expected count를 20으로 수정했으며 실제 layout과 Android draw log가 20이다. |
| 19 | 캡처가 기존 iOS data/app을 덮고 복원 실패가 숨겨짐 | 통과. 종료 handler로 앱을 재설치하고 iOS data를 복원했으며 11개 경로별 hash manifest가 설치 전후 일치한다. |
| 20 | 제한 fixture/screenshot을 공식 Grid support 또는 surface pixel parity로 과장 | 통과. C11.4/C11 체크와 `78/214`는 유지하고 README·status·roadmap은 screenshots를 visual smoke로 한정한다. 공개 API나 contract version은 바뀌지 않는다. |

검토에서 수정이 필요한 미해결 결함은 남지 않았다. 최초 18-vs-20 capture assertion 수정과 six-profile item/payload boundary 수정 내용은 [구현/runtime 검토](./implementation-runtime-review-20.md)와 [실행 근거](./README.md)에 연결했다. 이 표는 PR에 실제로 올라갈 변경분에 대한 최종 검토이므로, PR 생성 전 staged diff와도 재대조한다.

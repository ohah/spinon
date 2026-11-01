# C11.3.3 negative named-line pair 계획 검토 20개

검토 대상은 `plan/c11-3-3-negative-named-line-auto-implicit-tracks.md`와 연결된 사전 비교 입력·기대값이다. 각 행은 다른 실패 경로 또는 계약 경계를 확인한다. 1번에서 발견한 equal endpoint 설명 혼동은 §8.3.1의 end-line 제거와 기본 span 1 규칙을 분리해 계획에 반영했다. 이 검토는 계획 검토이며 구현 검토나 PR diff 검토를 대신하지 않는다.

| # | 실패 관점 | 확인·조치 |
| ---: | --- | --- |
| 1 | Equal endpoint를 broad auto-placement로 잘못 처리 | §8.3.1 end-line 제거 후 남은 definite line의 기본 1-track span으로 설명을 수정했다. 양 축 모두 automatic인 배치만 범위 밖으로 둔다. |
| 2 | Reversed endpoint를 implicit track sizing 전에 swap하지 않음 | Chrome control과 독립 기대값 모두 정상 순서 case와 같은 track order/frame을 요구한다. 구현 단계에서 conflict normalization을 sizing보다 먼저 실행한다. |
| 3 | 음수 occurrence 방향을 explicit-grid 시작점에서 셈 | `-4/-1`과 `-5/-2`는 explicit grid edge에서 negative occurrence를 찾고 필요한 leading track 수를 각각 4와 5로 고정한다. |
| 4 | 부족한 named occurrence에서 implicit-line fallback을 적용하지 않음 | `ghost`가 없는 authored grid에서 필요한 leading lines를 생성하는 규범 계산과 Chrome CSSOM을 기록했다. |
| 5 | Authored named line보다 implicit fallback을 먼저 선택 | `[ghost]` authored control에서 `-1 ghost`가 기존 line을 사용하고 `-2 ghost`만 implicit fallback을 사용한다. 새 mode는 두 끝이 모두 leading implicit으로 확인될 때만 선택한다. |
| 6 | Authored/implicit mixed endpoint를 새 제한 mode로 잘못 승격 | 기존 route가 precedence 조합을 표현하지 못하면 새 mode로 바꾸지 않고 fail-closed하도록 계획에 명시했다. |
| 7 | Leading `grid-auto-*` pattern 위상을 반대로 확장 | CSSOM start-to-end의 leading prefix `[5,7,5,7]` 및 `[7,5,7,5,7]`로 implicit pattern phase를 독립 고정했다. |
| 8 | `auto` track sizing 기여를 implicit extent 전체에 적용 | 18px item span의 세 track은 6px씩, span 밖 leading track은 0px로 별도 기대값을 기록했다. |
| 9 | Columns 결과를 Rows에도 그대로 가정해 축별 버그를 놓침 | fixed-px 및 `auto` row cases를 columns 결과와 대칭인 독립 scene으로 둔다. |
| 10 | Non-target axis가 implicit이거나 indefinite여서 범위가 확대됨 | 대상 한 축만 implicit이고 반대 축은 기존 definite profile을 사용하도록 입력과 plan gate에 고정했다. |
| 11 | 양 축 implicit extent를 한 축 정책으로 잘못 허용 | 두 축 동시 확장은 새 mode에서 거부하도록 scope와 failure cases에 명시했다. |
| 12 | Item intrinsic sizing이 fixed-size 검증을 우회 | item width/height 모두 definite fixed size로 제한하고 auto-sized item은 거부하도록 한다. |
| 13 | 추가 in-flow child가 implicit sizing 또는 auto-placement에 끼어듦 | direct grid item 하나만 허용하고 추가 participating child는 frame 생성 전에 거부한다. |
| 14 | CSS Grid area-backed 경로와 named-line 경로가 섞임 | `grid-template-areas`가 없는 container로 제한해 area-derived line names와 별도 route를 격리한다. |
| 15 | 지원하지 않는 grammar가 typed input으로 손실됨 | 대상축 `auto` 단일 값 또는 유한 non-negative px 목록만 수용하고 named span·혼합 sign·비지원 track을 거부한다. |
| 16 | overflow/quota 초과에서 부분 frame이 노출됨 | 산술 overflow와 10,000-track 초과를 frame 생성 전에 실패시키고 atomicity를 focused test gate에 둔다. |
| 17 | 잘못된 revision/snapshot이 현재 placement로 재사용됨 | revision 검증과 stale snapshot rejection을 projection/layout-core test 계획에 명시했다. |
| 18 | Chrome CSSOM 또는 screenshot만 제품 layout 증거로 오인 | 사전 비교는 Chrome oracle에 한정하며, 앱 구현은 resolved track, frame, draw/present와 surface readback 계층을 분리해 재검증하도록 한다. |
| 19 | Android AVD/iOS Simulator 결과를 실기기·compositor 완료로 확대 | AVD software renderer와 iOS drawable 제출 전 경계를 기록하고 실기기 검증이 아님을 명시했다. |
| 20 | 제한 fixture를 공개 CSS/WPT/C11 전체 완료로 표시 | public CSS admission, WPT conformance, 전체 C11.3.3 및 상위 C11을 제외하고 공식 `72/213`과 체크 상태를 미완료로 유지한다. |

**결과:** 20개의 서로 다른 실패 관점을 검토했다. 1번의 문구 혼동을 수정했으며, 나머지 경계는 계획의 명시적 reject gate 또는 이후 Android/iOS runtime gate로 추적한다. 계획 검토 통과는 구현 승인이나 제품 기능 완료를 뜻하지 않는다.

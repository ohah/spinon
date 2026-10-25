# C13.4 stable scrollbar gutter 계획 실패 관점 검토 20개 · 2026-10-26

**대상:** [C13.4 첫 조각 계획](../../../plan/c13-4-scrollbar-gutter.md), [내부 계약 초안 0083](../0083-c13-4-scrollbar-gutter.md), [Chrome 156 사전 관찰](c13-4-scrollbar-precomparison-2026-10-26/README.md)
**결론:** 새로 계획할 수 있는 구현 범위는 정의했지만 platform mode 확인 전에는 구현을 시작할 수 없다. 아래 20개 관점에서 그 gate가 빠지지 않는지 다시 확인했다.

| # | 실패 관점 | 계획상 방어·남은 판단 |
| --- | --- | --- |
| 1 | desktop Chrome 156의 15px를 iOS/Android 폭으로 하드코딩 | 폭은 UA/OS used value로 취급하고 버전별 관찰을 분리한다. |
| 2 | Chrome 156 observation을 계획 pinned Chrome 155로 표시 | evidence와 계획에서 version deviation을 명시하고 155 oracle로 승격하지 않는다. |
| 3 | overlay scrollbar에 `stable` gutter를 예약 | CSS Overflow 기대에 따라 overlay는 0px; simulator 관찰이 필요하다. |
| 4 | gutter 지원을 scrollbar 자체 표시 지원으로 오해 | stable은 scrollbar visibility를 바꾸지 않는다는 명세 경계를 적었다. |
| 5 | `auto`와 `stable`을 같은 overflow condition으로 구현 | overflowing과 non-overflowing control을 분리한다. |
| 6 | `both-edges`가 기존 시작 gutter만 유지 | classic mode에서 반대 inline edge도 같은 폭을 예약하는 표본을 고정한다. |
| 7 | `overflow:clip`이나 visible box를 scroll owner로 취급 | owner·gutter 없는 negative control을 둔다. |
| 8 | `hidden` programmatic scroll 의미를 잃음 | stable gutter로 scroll owner semantics를 바꾸지 않고 C13.2 lifecycle/range를 유지한다. |
| 9 | `overflow:auto` 축 정규화 뒤 cross-axis overflow가 달라짐 | 첫 구현 slice는 관찰된 단순 profile로 한정하고 auto cyclic selection은 제외한다. |
| 10 | scrollbar 폭만큼 outer border-box가 바뀜 | outer frame 유지, client/scrollport 내부 geometry만 변경한다고 명시한다. |
| 11 | gutter와 descendant layout이 다른 revision에 게시되거나 C13.3 gesture가 구 geometry를 소비 | scrollport·descendant·range·clip·paint/hit을 하나의 candidate snapshot으로 계산하고 C13.3 gesture session consumer를 integration regression으로 둔다. |
| 12 | 기존 offset이 재계산 뒤 잘못 유지되거나 초기화 | C13.2 보존/clamp/reset 규칙을 그대로 적용한다. |
| 13 | scrollbar만 바뀌고 clip/hit 영역은 이전 크기 사용 | clip/hit identity를 positive/negative matrix에서 함께 비교한다. |
| 14 | nested scroll owner에서 부모와 자식 gutter가 혼합됨 | nested owner control을 두되 구현은 한 profile로 제한한다. C13.3 전체 완료는 선행 조건이 아니지만 공유 snapshot 소비자는 회귀 gate다. |
| 15 | root gutter를 `body`로 잘못 전파 | root viewport/body propagation은 첫 slice에서 제외하고 C13.4 후속으로 남긴다. |
| 16 | RTL/vertical writing mode에서 inline edge가 반대 | 첫 profile을 horizontal-tb/LTR로 제한하고 edge 판정 불능은 거부한다. |
| 17 | Taffy `Overflow::Scroll` 예약이 flex/grid auto-minimum에 회귀 | 초기 Block-only profile 이후 다른 formatting context를 별도 plan/review로 둔다. |
| 18 | width/color의 style 입력을 gutter geometry와 섞어 대량 변경됨 | `scrollbar-width/color`와 scrollbar renderer/interaction은 후속으로 명시 제외한다. |
| 19 | mobile overlay bar가 안 보이자 테스트가 bar가 있는 것처럼 pass 처리하거나 Chrome first-run을 mode evidence로 과장 | Android Chrome은 first-run에서 중단했다. iOS Safari는 stable/both-edges zero-gutter geometry를 보였지만 indicator mode는 직접 보이지 않아 구현 gate는 닫혀 있다. |
| 20 | Simulator output이 CSS 의미가 아니라 host compositor 차이를 보여줌 | iOS screenshot과 DOM metrics를 함께 남기되 compositor 이후/실기기 및 gesture-time indicator는 미검증으로 구분한다. |

**검토 후 상태:** plan review 20/20 작성. 구현 review 및 PR delta review는 구현 코드/PR이 없으므로 아직 실행하지 않았다. AVD/iOS 실행 슬롯이 확보되면 mode 판정을 끝낸다. Overlay-only 결론이면 runtime code 없이 이 결과를 종료 근거로 남긴다.

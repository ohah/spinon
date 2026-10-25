# C12.5 sticky 계획 실패 관점 검토 · 20개 · 2026-10-26

대상은 [C12.5 plan](../../../../plan/c12-5-sticky-positioning.md), pinned fixture, Chrome observation JSON이다. 이 검토는 계획을 대상으로 하며 구현 코드 검토나 PR 변경 검토를 대체하지 않는다.

| # | 실패 관점 | 검토 결과 및 반영 |
|---:|---|---|
| 1 | 두 번째 browser eval이 앞 run의 scrollTop을 유지해 initial sample을 오염시켰는데 기준으로 재사용하는가 | 오염된 stdout은 사용하지 않고 첫 eval tool output의 값을 JSON으로 정리했다. 이를 README에 stdout 전사본으로 공개했고 screenshot도 상태가 불확실해 제거했다. |
| 2 | 큰 outer scroll이 sticky를 outer viewport에 고정해 nearest inner scrollport를 무시하는가 | `outer/inner=24/0`에서 Chrome이 sticky와 section을 함께 24px 이동한 관찰을 포함한다. runtime test는 inner scroll offset만 sticky threshold에 쓰는지 확인한다. |
| 3 | scrollport 시작과 sticky inset 계산에서 border·padding이 이중 계산되거나 빠지는가 | Chrome top 90과 C13.2 scrollport y 76의 차이를 확인했다. plan에 `scrollport_y + padding_top + top` 식을 명시하고 6px 누락을 실패로 고정했다. |
| 4 | section 끝 제한을 scrollport bottom 제한으로 잘못 구현하는가 | inner offset 212에서 section bottom 82, sticky height 28, sticky top 54의 관찰을 고정했다. lower bound의 geometry source를 containing block으로 명시했다. |
| 5 | 직전/직후 threshold를 건너뛴 단일 sample 때문에 고정 시작점을 잘못 추정하는가 | offsets 20, 36, 48을 따로 두어 자연 이동·경계 도달·고정 상태를 구분했다. |
| 6 | reverse scroll 결과를 실제 Chrome이 확인한 값으로 과장하는가 | Chrome observation은 증가 offset만 포함한다. reverse test는 runtime에서 동일 final offset state가 복원되는 invariant로 분리했고, 차이가 생기면 추가 Agent Browser oracle 전까지 비교를 중지하도록 plan을 보완했다. |
| 7 | scroll owner가 없을 때 sticky를 viewport 기준으로 성공 처리하는가 | 첫 profile은 양의 scroll range를 가진 nearest inner owner를 요구한다. owner 없는 장면은 property/node가 있는 fail-closed control로 둔다. |
| 8 | `overflow:clip`이나 `hidden` ancestor를 scroll container로 오인하는가 | 지원은 nearest y-axis `auto|scroll`만으로 제한하고 clip/hit clip은 C13.2 기존 계산 결과에 맡긴다. hidden/clip nearest owner는 제외 및 rejection 대상으로 명시했다. |
| 9 | outer와 inner offset을 모두 sticky threshold에 더해 위치를 두 번 보정하는가 | nearest owner와 outer scroll ancestry를 별도 보존하고, outer 이동은 scrollport와 sticky box 양쪽의 좌표 변환으로만 반영한다. |
| 10 | sticky를 Taffy out-of-flow box로 넣어 normal-flow 공간에서 제거하는가 | sticky box는 layout flow에 남고 C12.5 visual adjustment는 immutable render snapshot에서 적용한다고 명시했다. |
| 11 | unsupported sticky profile이 static/relative frame으로 성공 fallback하는가 | computed `Sticky`를 preserved enum으로 유지하고 unsupported property/node를 반환하며 fallback·0 offset·old snapshot 혼합을 금지한다. |
| 12 | `bottom`, logical inset, percentage나 negative top 값을 이 profile에서 누락된 상태로 허용하는가 | `top`의 finite nonnegative absolute length만 허용하고 다른 inset은 `auto`로 제한했다. 나머지 입력은 제외·거부 목록에 포함했다. |
| 13 | text/replaced/inline child가 있는 sticky subtree의 displacement를 root box에만 적용해 child frame이 분리되는가, 또는 RTL/writing mode의 containing block을 Block profile로 오인하는가 | sticky element는 leaf box여야 한다고 제한하고 text/replaced/inline, 다른 writing mode/direction을 fail-closed에 포함했다. |
| 14 | sticky containing block에 부모가 아닌 positioned ancestor나 viewport를 사용해 끝을 넘어가는가 | 현재 HostDocument parent와 해당 parent의 layout frame을 containing block으로 고정했다. transform/contain 등의 미지원 ancestor effect는 success profile에서 제외했다. |
| 15 | sticky frame은 고정하지만 child paint/hit-test 및 clip만 예전 위치에 남는가 | frame, paint clip, hit clip이 모두 같은 `ScrollStateRevision`에서 만들어져야 하고 exact NodeId/control을 비교하도록 했다. |
| 16 | stale style/layout/scroll revision을 조합해 부분적으로 새 위치를 게시하는가 | revision mismatch에서는 전체 immutable snapshot을 거부하고 이전 frame을 새 revision으로 재사용하지 않는 pass condition을 명시했다. |
| 17 | node remove/reparent 뒤 이전 sticky owner나 offset이 재사용되는가 | scroll owner removal/reparent와 document generation 변경을 runtime negative control 목록에 넣고 NodeId 기반 identity를 유지한다. |
| 18 | `display:none` ancestor 아래 sticky box가 scroll/stacking tree에 남는가 | hidden ancestor를 target/control matrix에 포함하고 visible node/frame·stacking context set에서 제외해야 성공한다. |
| 19 | sticky가 `z-index:auto`에서도 만드는 stacking context를 무시해 paint가 일반 in-flow 순서에 섞이는가 | C12.4 painter model을 사용하되 sticky의 auto context를 명시적으로 생성하도록 plan과 0058 update를 요구한다. 겹침·paint order regression은 구현 검토로 확인한다. |
| 20 | fixture test symbol/payload가 기본 artifact에 유입되거나 C12/CSS 전체 완료로 표기되는가 | `c12-5-sticky-fixture-test` 전용 opt-in, default/positive artifact 경계, C12.5/C12/CSS 미완료 및 공식 완료 수 보존을 명시했다. |

## 결론

초기 review에서 저장 artifact provenance, scrollport padding 시작점, Chrome reverse-scroll 미관찰 세 항목을 좁혔다. 첫 관찰 stdout의 provenance를 기록했고 top constraint의 6px padding component를 plan에 추가했으며 reverse case를 browser-observed 값이 아닌 runtime state invariant로 분리했다. 이 수정 후 계획은 구현을 시작할 비교·실패 경계를 갖춘다. 본 문서는 20개의 서로 다른 계획 실패 관점을 각각 확인했으며, runtime 구현·플랫폼 통과나 PR merge를 주장하지 않는다.

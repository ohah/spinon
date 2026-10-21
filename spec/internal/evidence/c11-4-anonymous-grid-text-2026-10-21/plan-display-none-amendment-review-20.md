# C11.4 anonymous Grid text 계획 범위 보강 검토 20개

2026-10-21 `display:none` direct child가 직접 text run을 나누지 않는 Chrome 관찰을 계획에 추가한 뒤 별도 관점 20개로 다시 검토했다. 기존 계획 검토 20개와 섞지 않았다. 기준은 [CSS Grid §6](https://www.w3.org/TR/css-grid-1/#grid-items), 고정 Chrome 155 observation, 수정된 계획·fixture다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | `display:none` direct child가 text run을 잘못 끊음 | `anonymous-hidden-child-continuation` oracle·fixture 모두 하나의 `40px` anonymous item과 `40px 80px` tracks를 요구한다. 반영됨. |
| 2 | hidden element 내부 text가 anonymous item에 섞임 | Chrome fixture의 `ignored` descendant는 Range 결과에 없고 layout test provenance에서도 제외한다. 반영됨. |
| 3 | hidden subtree가 geometry를 남김 | 계획은 숨김 subtree frame 제외를 명시하고 Android/iOS runtime result에 root만 남는 기대값을 고정했다. 반영됨. |
| 4 | 숨김 자식이 auto-placement cell을 소비함 | oracle의 후행 direct text가 두 번째 auto track이 되고 중간 hidden child는 item 수·track 수를 바꾸지 않는다. 반영됨. |
| 5 | style source가 inline attribute에만 한정됨 | 계획은 computed `display`를 사용하고 별도 parser나 새로운 스타일 입력을 도입하지 않는다. fixture 입력은 고정된 computed style로 제한한다. 범위를 명시함. |
| 6 | `display:none`과 `visibility:hidden` 혼동 | 계획에서 box 생성 여부를 기준으로 `display:none`만 추가하고 다른 visibility 값은 지원하지 않는다고 둔다. 반영됨. |
| 7 | 숨김 자식 뒤에 source order가 변함 | visible run은 첫 non-whitespace text node anchor와 child-list 순서를 보존하며 hidden node는 item ordinal을 만들지 않는다. 반영됨. |
| 8 | whitespace-only text가 hidden child 앞뒤에서 별도 item을 만듦 | collapsible-only run은 전체 그룹에서 제외하며 hidden child도 flush 경계가 아니다. 반영됨. |
| 9 | 빈 text node가 hidden child를 경유해 anchor를 선점함 | anchor 정의는 첫 non-whitespace text node로 유지하고 빈 node는 anchor가 될 수 없다. 반영됨. |
| 10 | NBSP 등 비-collapsible 문자를 공백으로 제거함 | oracle·계획은 HTML collapsible whitespace만 제외하고 일반 문자/다른 whitespace를 처리 범위 밖으로 둔다. 지원 계약이 과장되지 않음. |
| 11 | comment와 hidden element를 같은 종류의 DOM boundary로 취급 | comment는 HostDocument layout node가 아니고 hidden element는 존재하되 boxless다. 두 경계를 별도 입력으로 관찰한다. 반영됨. |
| 12 | visible block separator의 앞뒤 run이 합쳐짐 | block child 장면은 세 tracks와 중간 block frame을 고정하고 text run 두 개를 분리한다. 그대로 유지됨. |
| 13 | visible inline child를 숨김처럼 건너뜀 | inline/flex/grid/flow-root는 지원 목록에 없고 구현에서 명시 오류로 끝내도록 계획에 적었다. 반영됨. |
| 14 | nested hidden descendant에 직접 style이 없어 임의 표시됨 | 숨김 subtree는 layout 제외 traversal로 처리하고 누락된 element style은 오류로 반환한다. 반영됨. |
| 15 | hidden child가 DOM에서 삭제된 것으로 오해됨 | 계획은 원본 HostDocument 노드를 합치거나 삭제하지 않고 layout input·frame만 제외한다고 구분한다. 반영됨. |
| 16 | text provenance가 hidden node ID를 포함함 | hidden descendant ID는 source 목록에서 빠지고 양쪽 visible text source만 유지하도록 fixture test에서 검사한다. 반영됨. |
| 17 | CSS Grid 규칙과 Spinon의 제한 fixture가 혼동됨 | 문서는 W3C 규칙을 근거로 제시하지만 구현 상태를 opt-in fixture subset으로 한정하고 제품 지원·완료 수는 유지한다. 반영됨. |
| 18 | AVD/iOS 결과를 Chrome 관찰과 혼합 | Chrome observation은 별도 사전 oracle이고 runtime 결과는 별도 evidence JSON·APK/app hash로 기록한다. 반영됨. |
| 19 | 장면이 simulator 앱에서만 작동하고 Rust fixture가 누락됨 | 계획은 Rust layout/runtime, Android·iOS adapter, trace와 frame을 각기 검사하도록 한다. 다섯 번째 scene을 모든 경로에 추가했다. 반영됨. |
| 20 | 연결된 Android 실기기 검증을 simulator 결과로 대체 | 계획은 API 37 AVD와 SM-S731N 실기기를 별도 관문으로 둔다. 이번 host ADB·USB·mDNS에서 기기를 찾지 못해 실기기 관문은 pending으로 남긴다. |

결론: `display:none` 연속 규칙을 계획에 추가하고 다섯 번째 oracle/runtime case로 검증하도록 범위를 보강했다. 물리 Android 결과가 없다는 제한은 그대로 표시한다. 새 코드 검토는 [구현 실패 관점 20개](./implementation-review-20.md)에서 별도로 수행한다.

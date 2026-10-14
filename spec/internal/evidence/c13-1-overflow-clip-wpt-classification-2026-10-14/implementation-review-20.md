# C13.1 WPT 후보 분류 구현 관점 검토 · 20개 · 2026-10-14

검토 대상은 고정 WPT source inventory와 C13.1 범주 manifest다. 이 검토는 WPT 실행 결과나 CSS 동작 성공을 대신하지 않는다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 이동하는 upstream branch를 pin으로 오인 | WPT commit SHA `a73e14cfbaeba97744ff40cbc34d1ca96e5a92c7`와 UTC commit date를 함께 기록했다. |
| 2 | 현재 master의 날짜를 관찰값 날짜로 추정 | API commit metadata의 `2026-10-14T10:07:55Z`를 고정했다. |
| 3 | css/css-overflow 전체를 C13.1 scope라고 오인 | 파일명 prefix filter를 clip, overflow-clip, single-axis-overflow-clip으로 한정했다. |
| 4 | reference 파일을 실행 testcase로 집계 | `-ref` source를 inventory filter에서 제외했다. |
| 5 | source 파일 수와 directory entry 수를 혼동 | direct test HTML source만 58개로 별도 기록했다. |
| 6 | API listing에서 source 이름을 빠뜨리거나 manifest 목록을 함께 바꿔 검사를 우회 | exact WPT commit와 sorted sourceInventory SHA-256을 validator에 고정하고 classification path set과 비교한다. |
| 7 | 경로 중복이나 같은 개수 category swap이 검사를 통과 | validator가 58 unique path, classification tuple SHA-256을 요구하며 duplicate와 balanced relabel negative control을 거부한다. |
| 8 | 빠진 test가 count만으로 숨음 | sourceInventory에서 classification row를 뺀 negative control이 거부됐다. |
| 9 | 같은 수의 unlisted path로 누락을 대체 | unlisted path replacement mutation이 sourceInventory set 검사에서 거부됐다. |
| 10 | 임의 범주를 성공으로 세탁 | 3개 허용 category만 인정하며 `pass` category mutation이 거부됐다. |
| 11 | 카테고리 수 합계가 source 수와 다름 | 13 + 17 + 28이 58이며 validator가 각 count를 고정 확인한다. |
| 12 | candidate 표기를 이미 Spinon이 실행했다는 뜻으로 해석 | fixture candidate는 local adapter 후보로 정의하고 WPT 실행은 not-run으로 고정했다. |
| 13 | hit test candidate가 DOM event 지원 주장으로 확대 | 두 hit-testing candidate는 fixed box query adapter에만 한정하고 event dispatch를 분리했다. |
| 14 | hidden의 paint-only reftest가 scroll engine을 요구한다고 과대 분류 | 018·021 source는 overflow:hidden의 padding-edge visual clip을 그리며 scroll API나 scroll size를 읽지 않아 C13.1 candidate로 포함했다. |
| 15 | negative clip margin이 다른 문법 오류에 가려짐 | 016·017·019·020·022는 pinned Chrome/W3C WD 차이 때문에 Chrome oracle-only로 분리했다. |
| 16 | box-shadow 비교를 solid background paint와 동일시 | margin 003·006과 content visual overflow는 ink effect 경계로 두고 현재 runtime candidate에서 제외했다. |
| 17 | opacity/outline failure를 clip geometry 결과로 해석 | alpha compositing과 outline을 포함한 reftest는 Chrome oracle-only다. |
| 18 | scroll/CSSOM 동작이 C13.1에서 완료된 것으로 확장 | scroll offset, scroll range, scrollbar tests는 current C13.1 후보에서 빼고 후속 owner를 썼다. |
| 19 | radius/transform/RTL/SVG/multicol 소유권을 C13.1로 당김 | C22/C23/C17 및 현재 HTML Block/Flex profile 밖으로 분류했다. |
| 20 | Chrome oracle 또는 WPT upstream 상태를 Spinon WPT PASS로 오인 | manifest·README·test 모두 WPT not-run을 명시하고 completion claim을 거부한다. |

초기 PR delta 검토에서 count-preserving category swap과 동시 inventory 교체가 검사를 우회할 수 있음을 찾아 source/inventory/classification digest pin을 추가했다. 보강한 고정 manifest 검증 3/3이 통과했다. 남은 전체 C13.1 사전 비교 matrix와 paint/hit/event negative-control breadth는 이 분류로 닫히지 않는다.

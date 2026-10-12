# C11.2.2 Chromium 사전 비교 도구·fixture 구현 검토

**대상:** 15-scene Chrome 154 fixture, capture 도구, JSON reference, PNG 증거  
**성격:** 비교 fixture와 capture 도구 검토다. C11.2.2 runtime layout 구현 또는 WPT 적합성 검토가 아니다.  
**실행:** `node --check` 두 파일 통과 · 단독 Node test 4/4 통과 · Chrome 154 캡처 15/15 scene 성공.

| # | 적대적 실패 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 기존 precomparison을 새 실행으로 덮어쓴다. | capture는 versioned JSON 경로와 screenshot에 `wx`를 사용하고 시작 시 reference 존재를 거부한다. 초기 출력은 보존하고 최종 v3 경로로 고정했다. |
| 2 | 경로가 달라도 Chrome 버전이 변했는데 기준을 섞는다. | Chrome product `154.0.8037.98` 및 revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`를 실행 중 assert하고 reference에 저장한다. |
| 3 | 실행 파일 교체·재빌드 뒤 같은 기준 ID를 재사용한다. | executable SHA-256, inventory·HTML·capture 도구·helper SHA-256을 reference에 기록하고 test에서 현재 bytes와 대조한다. |
| 4 | inventory와 HTML scene이 서로 달라 일부 장면이 빠지거나 순서가 바뀐다. | parse5 test가 15 scene 순서와 각 scene의 전체 `data-node` preorder를 inventory에 대조한다. |
| 5 | 중복 NodeId 또는 잘못된 parent가 비교 결과를 우연히 통과시킨다. | 전역 중복 ID를 거부하고 각 관측 parent를 HTML DOM에서 계산한 가장 가까운 `data-node` parent와 비교한다. |
| 6 | 숨겨진 이전 scene 또는 둘 이상의 활성 scene이 capture에 들어온다. | Chrome 실행 시 각 평가 전에 선택 scene 하나만 활성인지 확인하고, 관측 ID 전체를 다시 검사한다. |
| 7 | DPR 1/2 수집이 CSS viewport나 환경 차이 때문에 달라진다. | viewport·locale·timezone·media·touch 환경을 고정하고 DPR 간 nodes, Grid properties, track counts의 전체 equality를 확인한다. |
| 8 | DPR 차이를 CSS px와 pixel coordinate를 혼합해 숨긴다. | 관측은 `getBoundingClientRect()` CSS px이며 DPR 1·2 비교에서 동일해야 한다. PNG는 별도로 픽셀 크기와 hash를 검사한다. |
| 9 | 계획에서 요구한 explicit/implicit track 수를 복원할 수 없다. | v2 초안 검토에서 수량을 직접 저장하지 않는 점을 찾아 capture schema v3에 authored explicit와 resolved computed count 및 implicit 차이를 넣었다. |
| 10 | computed track 수 parser가 `minmax()`·repeat 같은 미지원 문법을 공백 split으로 잘못 센다. | 이 고정 fixture는 px track list만 받도록 author input을 assert한다. 해당 subset 밖 문법을 이 도구가 조용히 세지 않는다. |
| 11 | explicit item이 source order 뒤에 있어도 먼저 배치되는 CSS 규칙이 빠진다. | `definite-first-mixed`의 고정 좌표에서 뒤에 선언된 definite item이 첫 cell, 앞 auto item이 두 번째 cell임을 assert한다. |
| 12 | row-locked placement가 column을 고정하지 않거나 column-locked가 row를 고정하지 않는다. | 두 독립 scene의 두 번째 item 좌표를 각각 `[75,20]`, `[64,44]`로 고정했다. |
| 13 | span을 한 cell로 계산해 occupancy overlap을 놓친다. | sparse scene의 2-column span frame `[20,44,84,18]`과 뒤 item `[108,44]`를 확인한다. |
| 14 | sparse와 dense가 같은 위치를 보여 주어 잘못된 비교 기준을 만든다. | sparse scene은 row 1의 세 번째 cell을 비워 둔 채 row 2로 진행하고, dense oracle-only control은 해당 hole로 되돌아온다. 두 좌표를 별도 assert한다. |
| 15 | implicit row의 크기나 gap이 빠져 item 위치만 비교한다. | Chrome resolved row list와 item 좌표를 함께 검사해 25px·24px implicit row 및 row/column gap을 기록한다. |
| 16 | `display:none` child가 occupancy에 포함되어 보이는 child를 밀어낸다. | hidden child computed display와 다음 두 visible child의 첫·두 번째 column 좌표를 고정했다. |
| 17 | nested Grid가 부모 placement mode를 공유하거나 nested node parent가 깨진다. | outer explicit grid 및 inner mixed auto grid 세 container의 parent 순서·Grid style·child frame을 함께 확인한다. |
| 18 | unsupported 입력의 Chrome 동작을 Spinon 지원으로 잘못 분류한다. | implicit column·auto implicit row·repeating auto row·dense·column flow는 모두 `chromium-oracle-only`이며 다섯 rejection reason을 요구한다. 구현 거부 결과는 수집하지 않았다. |
| 19 | CSSOM shorthand 직렬화 차이를 입력 의미로 오해한다. | Chrome이 authored `row dense`를 computed `dense`로 직렬화함을 발견해 기대값을 고쳤고 dense hole-backfill 동작도 함께 고정했다. |
| 20 | screenshot·WPT hash·결과를 과장하거나 연결을 끊는다. | 15 PNG의 SHA-256 및 크기, WPT pinned revision/source hash를 보존했다. WPT는 실행하지 않았고 Chrome oracle 결과를 Spinon 적합성으로 표기하지 않는다. |

## 수정 및 남은 경계

초기 검사에서 발견한 CSSOM `grid-auto-flow` 직렬화 차이를 테스트에 반영했다. 이어 구현 계획이 요구하는 explicit/implicit track count가 reference에 직접 남지 않는 점을 찾아 schema v3에 수량을 추가했다. capture 도구는 px track list 외의 형식이면 실패한다. 이 제한은 fixture 형식 검증이며 일반 CSS track-list parser가 아니다.

Android/iOS runtime 실행, GPU surface 비교, Spinon WPT runner, C11.2.2 layout algorithm은 이 변경에 포함하지 않는다. C11.2.1이 별도 통합되기 전까지 runtime feature 구현은 대기한다.

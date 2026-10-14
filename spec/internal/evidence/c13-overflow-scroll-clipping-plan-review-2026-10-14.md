# C13 · 계획 실패 관점 20개 검토

검토 대상은 [`C13 overflow·scroll·클리핑 계획`](../../../plan/c13-overflow-scroll-clipping.md)과 agent-browser로 수집한 [Chrome 155 사전 비교](./c13-overflow-precomparison-2026-10-14/README.md)다. 계획 문서 검토이며 기능 구현 검토나 PR 변경 검토 횟수로 재사용하지 않는다. 각 항목은 계획의 서로 다른 의미·수명·입출력 경계를 공격했다.

| # | 독립 실패 관점 | 검토 결과와 계획 반영 |
| ---: | --- | --- |
| 1 | `overflow-x:visible` 또는 `clip`을 반대 축의 scrollable value와 조합했을 때 computed value 정규화를 생략해 computed style은 틀리고 paint는 우연히 맞는다. | C13.1에 shorthand/longhand, mixed-axis normalization, 정확한 Chrome computed value 비교를 둔다. 사전 관찰에는 `visible/hidden → auto/hidden`, `clip/auto → hidden/auto` 결과를 고정했다. |
| 2 | `clip`을 `hidden`의 별칭으로 구현해 scroll container 및 BFC를 만들거나 programmatic offset을 허용한다. | C13.1 float/`flow-root` 대조와 C13.2 scroll offset oracle을 분리했다. 사전 비교에서 clip은 offset 0, hidden은 변경 가능한 결과이며 float 경계도 따로 기록했다. |
| 3 | `overflow-clip-margin`을 모든 overflow 값과 edge keyword에 적용하거나 computed value만 비교하고 실제 clip edge를 놓친다. | C13.1은 edge keyword와 적용 조건을 명시하고 clip 바깥 draw/hit 및 geometry를 함께 비교한다. scrollable value에서 무시되는 조건은 실패 관문에 남겼다. |
| 4 | shadow/outline 같은 ink overflow를 scrollable overflow로 합쳐 `scrollWidth`가 커지거나, clip 시 ink 그리기와 hit-test가 서로 다른 경계가 된다. | scrollable area와 ink overflow를 구별하고 clip draw, hit rejection, `scrollWidth`/`scrollHeight`를 별개 관찰로 고정했다. |
| 5 | 부모 clip은 paint에만 적용하고 hit-test는 자식의 원래 frame을 써서 보이지 않는 자식을 누를 수 있다. | C13.1의 layout→paint→hit-test clip chain과 clip 바깥 draw/hit negative control을 완료 관문으로 둔다. |
| 6 | nested clip과 scroll offset의 합성 순서가 달라 descendant가 이중 이동하거나 clip이 scroll 전 좌표에 남는다. | `ScrollStateRevision`과 같은 revision의 immutable render/hit-test snapshot을 요구하고 stale clip/scroll snapshot을 거부한다. C13.1/13.2 사이 통합 관문으로 기록한다. |
| 7 | `<html>`과 `<body>` overflow가 viewport로 전파될 때 실제 root scroll owner와 document element의 scroll API 값이 엇갈린다. | C13.2에서 root/body propagation뿐 아니라 DOM scroll owner를 확인하고 `scrollTop/Left`, client/scroll dimension oracle를 수집한다. |
| 8 | scrollport 크기에서 border, padding, classic scrollbar 또는 gutter를 중복 차감해 client 크기와 scroll range가 어긋난다. | C13.2 client/scroll dimensions와 C13.4 gutter·classic/overlay geometry를 별도 단계에서 Chrome 및 Simulator와 대조한다. |
| 9 | absolute/fixed descendant나 overflow-visible descendant의 scrollable overflow 포함 여부가 일반 in-flow child와 뒤섞여 range가 달라진다. | C13.2에 nested·out-of-flow 경계를 추가하고 C12.3 fixed 및 C12.5 sticky의 선행 조건과 교차 확인하도록 했다. |
| 10 | scroll offset을 document/style revision에만 묶어 scroll-only 변경 뒤 frame과 hit-test가 구 revision으로 남는다. | scroll-only 변경은 별도 monotonic `ScrollStateRevision`을 올리고 render snapshot 및 hit-test가 동일 revision을 요구한다. overflow/counter/allocation 실패는 부분 게시를 허용하지 않는다. |
| 11 | NodeId를 제거 후 재사용하거나 부모 이동 때 이전 요소의 offset을 새 요소에 붙인다. | scroll state를 NodeId와 document generation에 묶고 제거·재사용·source parent 이동·root 교체 시 offset 소유권을 확인하도록 명시했다. |
| 12 | 내용이 줄어 scrollable range가 축소됐는데 기존 offset을 유지하거나 offset clamp와 snapshot 게시 사이에 torn state가 노출된다. | C13.2의 재계산 뒤 offset clamp·원자 snapshot·오류 후 복구 gate 및 style/layout 변화 stale control로 포함했다. |
| 13 | nested gesture가 inner 경계에서 멈추거나 남은 delta를 버려 부모가 움직이지 않는다. | C13.3에 축별 remaining delta와 inner→outer chain trace를 둔다. no-scroll-axis와 `overscroll-behavior`의 분리 케이스를 추가했다. |
| 14 | 대각 gesture에서 한 축은 소진됐지만 다른 축 이동량까지 잃거나, axis-lock 임계값을 플랫폼별 임의 기본값으로 고정한다. | C13.3에 diagonal/axis-lock trace와 남은 축 전달을 추가했다. 임계값은 고정 oracle 없이는 제품 계약으로 정하지 않도록 보완했다. |
| 15 | pointer cancel·두 번째 touch·surface 재생성 뒤 늦은 move/up이 새 gesture 또는 새 scroll target에 적용된다. | C13.3 및 실패·수명 관문에 cancel, 다중 pointer, stale frame/generation, 늦은 이벤트 재결합 방지 negative control을 둔다. |
| 16 | overscroll `contain`/`none`이 한 축에서만 선언됐는데 다른 축의 chaining이나 root viewport action까지 막거나 허용한다. | overscroll x/y를 각각 검사하고 실제 플랫폼 boundary action과 chaining 차단을 구분한다. OS bounce/glow 미제공을 PASS로 간주하지 않는다. |
| 17 | scrollbar가 overlay인지 classic인지 또는 OS에서 숨겨졌는지 모른 채 content width, gutter, thumb geometry를 같은 결과로 비교한다. | C13.4는 mode와 실행 환경을 기록하고 overlay/classic 비교를 분리하며 측정 불가 geometry를 미측정으로 남긴다. Chrome baseline의 `--hide-scrollbars` 한계도 README에 고정했다. |
| 18 | `scrollbar-width:none`이 scroll input도 비활성화하거나 keyboard 접근 수단을 없앤다. | `none`에서도 touch/keyboard/programmatic scroll 및 접근 가능한 대체 조작을 유지하고 C13.4에서 키보드·접근성 입력을 별도 검증한다. |
| 19 | mandatory snap이 oversized target의 중간 콘텐츠 접근을 막거나, layout 변경·target 삭제 뒤 사라진 snap target을 재사용한다. | C13.5에 oversized target 접근성, DOM update 뒤 target 유지/삭제, mandatory/proximity 분리와 끝 offset·선택 target oracle를 명시했다. |
| 20 | plan/WPT/Chrome/Simulator 결과를 합쳐 public CSS 완료로 표시하거나 fixture adapter가 기본 제품 artifact에 포함된다. | 상태 대장은 미완료다. WPT는 고정 commit/source hash로 분류하고 Chrome oracle PASS를 Spinon 적합성으로 확대하지 않는다. 구현 PR은 별도 20개 검토, simulator 경계, 기본 artifact 부재와 opt-in fixture-test 존재의 양방향 control을 요구한다. |

## 검토 표 자체의 적대적 재검토

- 같은 의미를 단어만 바꿔 두 항목으로 세지 않도록 cascade, paint/hit-test, state identity/revision, input chain, scrollbar, snap, artifact boundary로 실패 지점을 나눴다.
- 계획 검토는 실제 구현 검증을 대신하지 않는다. 표의 모든 항목을 실행 PASS로 표현하지 않았으며 실행은 C13 단계별 gate에 남겼다.
- 계획 자체 검토 20개와 별도로 모든 PR이 받아야 하는 변경 검토 30개, 기능 구현별 신규 코드·실행 검토 20개를 구분했다. 한 검토 기록을 다른 관문에 재사용하지 않는다.
- 사용자가 실기기 연결이 불필요하다고 정정한 범위를 반영해 Android API 37 ARM64 AVD와 iOS Simulator만 계획에 남겼다. 이 근거를 실기기 또는 실제 display 동등성으로 확대하지 않는다.
- 사전 Chrome fixture는 overflow semantics 일부만 관찰한다. gesture, scrollbar 모양, snap, root scroll owner, 실제 Spinon runtime 결과를 이미 확인한 것처럼 인용하지 않는다.

계획에 발견한 누락은 ink overflow와 scrollable overflow의 구분, edge keyword 적용 조건, 대각 gesture의 남은 delta/axis-lock 경계, root/body의 실제 DOM scroll owner, 키보드·접근성 대체 조작이다. 위 항목은 계획 표와 실패 관문에 반영했다. 계획 검토에서 구현 차단 결함은 남아 있지 않으며, C13은 미완료다.

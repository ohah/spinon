# C13.1 paint/hit border-strip PR delta 검토 30개 · 2026-10-14

이번 검토는 이전 C13.1 리뷰와 별도로 `RuntimeRenderBox.hit_test_order`, C13.1 hit 순서 생성, static/positioned border-strip 동작, 새 Chrome/Rust 근거와 PR 주장을 대조했다. 각 항목은 다른 회귀 경로나 계약 경계를 확인한다.

1. **기존 생성자 호환성** — 새 필드가 지정되지 않은 `RuntimeRenderBox::new`에서 hit 순서가 paint 순서와 같아지는가? 기본값과 runtime scene 회귀 assertion이 이를 확인한다.
2. **정상 paint 순열 유지** — 기존 `paint_order`의 연속 인덱스 검증이 바뀌지 않았는가? 중복/재배열된 paint 순서 테스트는 계속 `InvalidPaintOrder`를 기대한다.
3. **중복 hit 순서 거부** — 두 box가 같은 우선순위를 가져 선택이 반복마다 달라질 수 있는가? snapshot 생성이 `InvalidHitTestOrder`로 거부한다.
4. **범위 밖 hit 순서 거부** — `boxes.len()` 이상인 우선순위가 승인되는가? out-of-range 생성 테스트가 거부를 확인한다.
5. **빈 snapshot 경계** — box가 없는 snapshot의 hit query가 가짜 target을 돌려주는가? 빈 scene 회귀 테스트에서 `None`이다.
6. **비유한 query 좌표** — NaN 또는 무한 좌표가 geometry 비교를 통과하는가? 기존 비유한 입력 테스트는 계속 `None`을 기대한다.
7. **viewport 바깥 query** — hit 순서 분리로 viewport 밖 node가 보이게 되는가? runtime 경계 테스트와 iOS 앱의 null control 4/4가 이를 제한한다.
8. **반열린 frame edge** — 오른쪽/아래쪽 끝 좌표가 새 순서로 inclusive가 되는가? 기존 half-open edge 테스트가 유지된다.
9. **축별 clip 보존** — x-only clip이 y까지 제한하거나 clip 없는 축이 사라지는가? 기존 axis clip 테스트와 새 y=40 Chrome 표본이 통과한다.
10. **static 내부 child hit** — paint edge 안의 `(125,40)`이 부모에 가려지는가? Chrome과 Rust가 `static-target`을 반환한다.
11. **static border-strip 누출** — child rect 안이지만 paint edge 밖인 `(128,40)`에서 child가 hit되는가? Chrome과 Rust가 `static-clip`을 반환한다.
12. **static border 바깥 parent 과잉 hit** — `(131,40)`에서 clip parent가 border 밖까지 확장되는가? Chrome과 Rust가 `root`를 반환한다.
13. **positioned clip 내부 hit** — `(315,40)`에서 absolute child가 paint 순서상 뒤의 parent에 가려지는가? 독립 hit 순서로 `positioned-target`을 반환한다.
14. **positioned border paint/hit 분리** — `(318,40)`에서 parent border paint가 유지되면서 child가 hit되는가? Chrome border pixel은 gold이고 Chrome/Rust target은 child다.
15. **positioned border 바깥 누출** — `(321,40)`에서 child hit가 parent border box 밖으로 나가는가? Chrome과 Rust 모두 `root`를 반환한다.
16. **paint geometry 불변** — hit clip 수정이 실제 clip bounds를 이동·확장하는가? static `[14,126]`, positioned `[204,316]` paint interval assertion이 그대로다.
17. **painter order 불변** — hit 우선순위 설정이 box painter order를 재정렬하는가? fixture는 positioned target의 `paint_order`가 parent보다 작음을 assertion한다.
18. **query 정렬 방향** — 더 큰 hit 순서가 앞에서 검사되는가? 겹치는 두 box 단위 테스트에서 독립 hit 우선순위 target이 선택된다.
19. **C12 profile 회귀** — 공통 runtime snapshot 변경이 기존 C12 stacking profile의 target order를 바꾸는가? profile 회귀 assertion은 모든 box에서 `hit_test_order == paint_order`를 확인한다.
20. **C13 profile 범위** — preorder 재계산이 지원되지 않는 profile 또는 C12 호출에서 실행되는가? 계산은 `RuntimeBlockFlexOverflowClipV1` 분기에만 적용된다.
21. **형제 stacking rank** — DOM sibling 순서와 달라진 paint rank가 무시되는가? 형제는 subtree의 첫 painter rank로 정렬하며 기존 flex/reverse-painter 행렬이 회귀를 검사한다.
22. **중첩 context 경계** — 자손 z-index가 부모 stacking context 밖으로 새는가? 기존 negative/zero/positive 및 nested context 비교가 통과한다.
23. **display:none 자손** — render node가 아닌 숨김 subtree가 순위 빈틈이나 유효 순서 누락을 만드는가? 결과 map 크기와 render node 개수가 일치하지 않으면 `InvalidRoot`로 거부한다.
24. **문서 parent 연결 누락** — 부모 정보가 없는 자식이 조용히 root 순서에 합쳐지는가? 순회 중 parent lookup 실패는 `InvalidRoot`로 거부한다.
25. **순위 누락 검출** — paint node가 hit preorder에서 빠져 일부 snapshot만 만들어지는가? 최종 순위 개수 비교가 누락을 거부한다.
26. **깊은 트리 호출 스택** — 적대적으로 깊은 DOM에서 재귀 순회가 stack overflow를 일으키는가? 새 hit preorder 함수는 명시적인 pending Vec를 사용한다. 기존 1,024-node deep-graph 검증은 C12 profile이라 새 C13 순회 경로의 깊이 한도는 별도 측정하지 않았다.
27. **render-node 예약 실패** — hit map node 예약 거부가 부분 snapshot으로 노출되는가? `hit-test render nodes` fault-injection test가 정확한 `AllocationFailed` stage를 확인한다.
28. **child 예약 실패** — parent-child index 확장 실패가 성공처럼 처리되는가? `hit-test children` fault-injection test가 같은 오류 경계를 확인한다.
29. **traversal 예약 실패** — pending stack 확장 실패가 누락 순위를 게시하는가? `hit-test traversal` fault-injection test가 중단과 정확한 stage를 확인한다.
30. **증거 범위 과장** — 새 six-point 앱 parity를 이미 입증했다고 PR이 말하는가? Chrome/Rust 여섯 점만 새 경계 결과이며 iOS Simulator는 기존 fixture 13/13 frame, 15/15 query 재실행으로 분리했다. 앱의 새 경계 좌표, Android 앱 재실행, drawable readback 및 DOM event dispatch는 남은 범위다.

## 판정

코드·fixture·기존 회귀 테스트와 저장된 실행 근거에서 수정이 필요한 추가 회귀는 발견하지 못했다. workspace 테스트 733 passed, 0 failed, 4 ignored, Clippy 및 format 검증은 별도 로그에 기록돼 있다. `BTreeMap` 내부 allocator OOM, 새 좌표의 Android/iOS 앱 parity, 전체 C13.1 matrix와 event dispatch는 이번 PR delta가 증명하지 않는다. 상위 C13과 C13.1 완료 표시는 계속 미완료로 둔다.

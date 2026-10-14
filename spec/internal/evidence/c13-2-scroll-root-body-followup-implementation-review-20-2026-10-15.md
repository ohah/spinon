# C13.2 root/body 후속 구현 실패 경로 검토 20개

- **대상:** C13.2 Android·iOS app fixture에 추가한 html/body root propagation 장면과 hidden-root 독립 body scroller.
- **검토 범위:** 기존 C13.2 구현 review 20개와 분리한 새 검사다. 각 항목은 이번 root/body adapter의 별도 실패 조건이다.
- **실행 근거:** Android API 37.0 ARM64 read-only AVD, iPhone 17 Pro/iOS 26.2 Simulator, FFI feature test, 전체 workspace test, Chrome 155 root/body oracle.
- **판정:** 구현된 경계 20개에서 미해결 오류는 없다. 반복 FFI invocation-on-same-host, app-level stale/failure injection, simulator pixel readback은 별도 미검증으로 남긴다.

| # | 공격/실패 경로 | 확인과 판정 |
|---|---|---|
| 1 | 같은 V8 global scope에서 visible·hidden fixture의 `const` 선언이 재선언 오류를 낸다. | 각 source를 IIFE로 감쌌다 (`runtime-scroll-container-c13-2-root-body.js:1-49`, `...hidden.js:1-52`). 두 Simulator에서 visible → hidden eval 모두 `layout=ready`를 반환했다. **PASS** |
| 2 | hidden fixture를 만들 때 visible html subtree가 연결된 채 남아 owner 수가 늘어난다. | hidden script가 이전 html root를 먼저 detach한다 (`...root-body-hidden.js:2-5`). 두 Simulator hidden 장면에서 예상 owner 수 2를 만족했다. **PASS** |
| 3 | id map에 `html`·`body` alias가 없어 다음 matrix cleanup이 실제 root를 찾지 못한다. | source map에 별도 alias를 넣고 matrix 시작 때 alias로 detach한다 (`...root-body.js:20,27`, `runtime-scroll-container-c13-2.js:3-9`). 이후 matrix owner 9개와 lifecycle가 통과했다. **PASS** |
| 4 | 첫 eval이 layout timeout 또는 실패 report를 내는데 root offset 검증이 계속 실행된다. | FFI route가 `layout=ready`가 아니면 `-10` 또는 원래 오류로 즉시 반환한다 (`ffi/fixtures/c13_2.rs:58-80,128-134`). FFI invalid argument test와 정상 app route가 각각 통과했다. **PASS** |
| 5 | layout frames와 render snapshot이 서로 다른 document/style/environment revision인데 frame을 비교한다. | `c13_2_root_body_frames`가 completed layout render key와 입력 scene key를 모두 completed key에 대조한다 (`root_body.rs:264-283`). 두 Simulator app route에서 통과했다. **PASS** |
| 6 | 일부 node만 생성되어 잘못된 frame 인덱스를 정상 장면처럼 처리한다. | helper가 frame 수를 5로 고정하고 다른 수는 fail-closed한다 (`root_body.rs:285-291`). 양 플랫폼 log의 node 1–5 순서 및 node frame 값을 확인했다. **PASS** |
| 7 | visible html/body 장면에 html viewport owner가 없거나 여러 owner가 생성된다. | snapshot owner 수를 1로 제한하고 html node owner 존재를 요구한다 (`root_body.rs:17-29`). Android·iOS 모두 `viewport_owner=html`을 기록했다. **PASS** |
| 8 | visible html의 owner가 일반 nested scroller처럼 parent를 갖는다. | viewport owner의 `parent()`가 `None`인지 확인한다 (`root_body.rs:30-34`). 두 Simulator에서 root propagation assertion가 통과했다. **PASS** |
| 9 | visible root에서 body `auto`가 html과 body의 이중 owner로 남는다. | body node에 별도 owner가 없는지 검사한다 (`root_body.rs:30-34`). Chrome JSON은 body offset 0, app log는 `body_owner=none`을 기록했다. **PASS** |
| 10 | x/y range가 섞이거나 visible root overflow를 body owner의 range로 잘못 계산한다. | html viewport owner의 max range를 각각 64·84 CSS px exact 비교한다 (`root_body.rs:30-39`). Chrome 155와 양 Simulator log에서 일치했다. **PASS** |
| 11 | fixed child의 right/bottom 위치가 viewport 크기나 scroll 앞 단계에서 틀려도 성공한다. | 두 장면 모두 scroll 전에 fixed rect `(340,204)`를 확인한다 (`root_body.rs:53-68,182-197`). Chrome JSON과 app frame report가 일치했다. **PASS** |
| 12 | visible root offset이 snapshot frame은 옮기지만 owner state에는 반영되지 않는다. | moved html owner `offset_x/y`를 64·84로 exact 확인한다 (`root_body.rs:98-104`). 두 Simulator root propagation이 통과했다. **PASS** |
| 13 | root offset이 content만 옮기고 positioned descendant marker에는 적용되지 않는다. | content와 marker frame 양 축의 이동량을 비교한다 (`root_body.rs:104-107`). moved marker hit identity도 양 플랫폼에서 통과했다. **PASS** |
| 14 | viewport scroll이 fixed child에 적용되거나 fixed child가 root owner extent에 포함된다. | fixed frame을 scroll 전후 동일 비교하고 초기 좌표도 고정한다 (`root_body.rs:53-68,92-108`). 양 플랫폼에서 `(340,204)`로 유지됐다. **PASS** |
| 15 | paint 위치만 이동하고 hit-test는 예전 marker를 가리킨다. | 새 marker 중심은 marker NodeId와 일치하고 옛 중심은 marker NodeId를 반환하지 않아야 한다 (`root_body.rs:109-115`). 두 Simulator route가 통과했다. **PASS** |
| 16 | html hidden 상태에서도 viewport range가 body range를 상속하거나 root owner가 중첩된다. | hidden html owner의 두 range를 0, parent를 없음으로 검사한다 (`root_body.rs:148-169`). Chrome의 html client/scroll 크기와 app assertion이 일치했다. **PASS** |
| 17 | hidden html 아래 body owner가 사라지거나 html과 다른 parent를 갖는다. | owner 수 2와 body owner의 `parent == html`을 각각 강제한다 (`root_body.rs:139-169`). 두 Simulator에서 `root_body_hidden=passed`를 기록했다. **PASS** |
| 18 | body owner의 초기 offset이 0이 아니거나 range가 Chrome과 다르다. | 초기 body offset `(0,0)`, max `(64,84)`를 exact 검사한다 (`root_body.rs:152-163`). Chrome 155 JSON 및 두 Simulator 장면에서 통과했다. **PASS** |
| 19 | hidden-root 장면에서 body를 움직일 때 viewport도 같이 움직이거나 body state만 달라지고 paint는 고정된다. | moved html offset `(0,0)`, body offset `(64,84)`와 content·marker frame 이동을 확인한다 (`root_body.rs:227-249`). 두 Simulator에서 통과했다. **PASS** |
| 20 | FFI가 중간 root/body 단계 실패를 성공 status로 감싸거나 실패 scene을 다음 matrix로 넘긴다. | 각 stage가 오류 시 즉시 반환하고 모든 report가 있을 때만 status 0을 쓴다 (`ffi/fixtures/c13_2.rs:58-134`). 정상 경로는 양 플랫폼 `status=0`; app-level stale/fault injection은 아직 없다. **PARTIAL** |

## 남은 경계

root/body 두 장면과 scroll matrix/lifecycle은 API 37 ARM64 AVD 및 iOS Simulator에서 실제 앱 경로로 통과했다. 별도 app-level stale/fault injection과 simulator pixel readback은 아직 하지 않았다. ink-only `box-shadow`는 현재 runtime에서 fail-closed이며 이 검토의 pass 범위에 넣지 않는다.

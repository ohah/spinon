# C13.2 계획 실패 관점 20개 검토

- **검토 종류:** 구현 전 계획 실패 관점 20개
- **기준:** `plan/c13-2-scroll-container-state.md`, `spec/internal/0062-c13-2-scroll-state.md`, Chrome 155 reference v1, source commit `e2f346f3dda6d56f68fd6ef2d0255610a282d0f3`
- **실행 oracle 검사:** `node --test tools/css-reference/c13-2-scroll-precomparison.test.mjs` — 5/5 PASS
- **상태:** 발견 사항을 계획/계약에 반영했다. 기능 코드는 아직 시작하지 않았다.

| # | 적대 관점 / 실패 시나리오 | 계획의 차단 기준과 확인 |
| --- | --- | --- |
| 1 | `overflow:visible` 자식이 부모 border 밖에 보이는데 scroll owner로 오인한다. | visible fixture에서 화면상 escape와 offset `(0,0)`을 함께 고정했다. C13.2 pass는 paint와 scroll state를 별도 비교한다. |
| 2 | `scrollWidth > clientWidth` 하나만 보고 scrollable이라고 판정한다. | visible 및 clip의 scroll dimensions는 크더라도 offset 소유권이 없음을 계약에 분리했다. scrollWidth만 쓰는 assertion을 허용하지 않는다. |
| 3 | absolute descendant가 containing block/scroll owner 밖인데 range에 포함되거나 안쪽 descendant를 빠뜨린다. | oracle은 scrollport 안에 containing block이 있는 positive out-of-flow case를 둔다. fixed/transform/negative origin은 profile 밖으로 명시했다. |
| 4 | box-shadow·ink extents가 scroll range를 부풀린다. | ink-only 대조가 `scroll/client=(114,76)`, max `(0,0)`을 요구한다. C13.1 clip/paint evidence와 합산하지 않는다. |
| 5 | border edge, padding/client scrollport와 border-box frame을 한 값으로 사용한다. | 고정 box `120×82`, border `3`, client `114×76`을 저장하고 `scrollport=client/padding 영역`으로 정의했다. |
| 6 | x overflow가 y range를 만들거나 y overflow가 x range를 만든다. | x-only max `(86,0)`, y-only `(0,74)`를 독립 고정하고 축별 range assertion을 둔다. |
| 7 | LTR negative offset이나 과대 요청이 음수/무한 range를 만든다. | negative setter writes 0으로 돌아오는 고정 probe와 `[0,max]` contract를 둔다. RTL negative origin은 C13.6에 남겼다. |
| 8 | `hidden`을 `clip`과 합쳐 programmatic scroll을 잃는다. | hidden은 `(55,44)`를 유지하고 clip은 동일한 script write 뒤 `(0,0)`인 별도 case다. |
| 9 | overflow 없는 `auto`와 `scroll`을 동일한 의미로 축약한다. | 둘 다 range 0인 dimension을 대조하지만 computed value는 각각 auto/scroll로 보존한다. `scroll`의 scroll owner 의미를 계획에 포함했다. |
| 10 | nested outer offset을 inner에 더해 저장하거나 두 offset을 하나의 viewport delta로 합친다. | outer `(22,17)`과 inner `(42,29)`를 동시에 고정하고 owner별 독립 state를 요구한다. |
| 11 | overflow extent가 늘거나 같은 owner 내부 descendant가 이동하면 유효 offset을 초기화한다. | growth 및 `display:contents` wrapper reparent 뒤 `(30,20)` 보존 control을 추가했다. |
| 12 | overflow extent가 줄었는데 이전 offset이 새 max를 넘는다. | over-max 입력은 `(146,124)`에 clamp되고 extent 소멸 뒤 `(0,0)`이어야 한다. |
| 13 | scroll owner node를 detach 후 reinsert해도 stale offset을 되살린다. | 같은 node remove/reinsert 뒤 Chrome `(0,0)` reference와 owner teardown/zero initial contract를 고정했다. |
| 14 | root `html`과 body overflow propagation을 일반 element처럼 계산한다. | default, body auto, html auto, html hidden + body auto의 네 조합과 `window`/HTML/body writes를 별도 fixture로 기록했다. |
| 15 | `document.scrollingElement`가 HTML이라는 이유로 body 독립 scrollport를 누락한다. | html hidden/body auto에서는 document scrollingElement가 HTML이어도 body offset만 `(64,83)`으로 변하는 oracle을 둔다. |
| 16 | root scroll offset을 fixed descendant에도 적용한다. | root fixture의 fixed control rect `[580,444,52,28]`이 모든 root window-scroll 조건에서 유지돼야 한다. |
| 17 | fractional CSSOM assignment 관찰을 C13 내부 state 의미로 오인한다. | Chrome Headless `55.25→55`, `55.5→56` probe를 보존하되 C29/S05 owner로 분리하고 C13.2 runtime offset gate는 정수값으로 고정했다. |
| 18 | document/style/environment revision이 같다는 이유로 stale scroll snapshot을 허용한다. | 독립 monotonic `ScrollStateRevision`을 runtime render/hit key에 넣고 scroll-only mismatch 거부를 pass 조건으로 둔다. |
| 19 | 새 layout range는 게시했지만 일부 owner의 offset clamp가 빠지거나 paint/hit이 서로 다른 offset을 본다. | 후보 state 전체를 만들고 paint/hit에 동일 ancestor chain을 적용한 뒤 atomic publish; 중간 혼합 snapshot은 거부한다. |
| 20 | 오류 뒤 worker가 부분 scroll map을 재사용하거나 fixture ABI가 제품에 들어간다. | reserve/invalid/stale failure와 동일 worker 복구, default archive absence 및 opt-in positive control, API 37 ARM64 AVD와 iOS Simulator를 각각 요구한다. 실기기는 gate에 넣지 않았다. |

## 판정

계획은 스크린상 border escape, scrollable overflow, ink-only extent, root/body 실제 owner, fixed control, move/reset과 revision/atomicity를 서로 다른 oracle/assertion으로 구분한다. headless scrollbar gutter와 public CSSOM setter rounding은 미확정 제품 동작으로 격리했다. 이 계획 리뷰는 구현 검증 또는 PR 변경 관점 검토를 대신하지 않는다.

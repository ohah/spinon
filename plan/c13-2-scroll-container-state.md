# C13.2 · Scroll container와 revision 분리

- **상태:** Chrome 155 사전 기준·내부 계약 고정 · 구현 시작 전
- **기준 커밋:** `e2f346f3dda6d56f68fd6ef2d0255610a282d0f3` (C13.1 surface pixel-center gate 통합 뒤)
- **내부 계약:** [`0062-c13-2-scroll-state`](../spec/internal/0062-c13-2-scroll-state.md), 숫자 버전 `0.1.0-draft` 유지
- **구현 전 비교:** [C13.2 Chrome/agent-browser oracle](../spec/internal/evidence/c13-2-scroll-container-precomparison-2026-10-15/README.md)
- **계획 실패 관점 20개:** [검토 기록](../spec/internal/evidence/c13-2-scroll-container-plan-review-2026-10-15.md)
- **계획 PR 변경 관점 30개:** [검토 기록](../spec/internal/evidence/c13-2-scroll-container-pr-review-2026-10-15.md)

## 목표

C13.1의 clip-only `hidden`·`scroll`·`auto` 경로에 scroll container, scrollable overflow area, scrollport, CSS px offset과 clamp를 연결한다. 스크롤 이동은 같은 immutable render snapshot의 paint와 hit-test에 반영하고 별도 `ScrollStateRevision`으로 오래된 입력을 거부한다.

이 단계는 내부 런타임 fixture 구현 계획이다. 공개 CSSOM scroll 메서드·속성, `scroll` event와 실제 touch/trackpad gesture는 C29/S05 및 C13.3 이후 책임이다. 스크롤바 geometry는 C13.4, snap은 C13.5, negative RTL/logical axis는 C13.6이다. 이 단계만으로 C13 또는 제품 CSS 지원을 완료 처리하지 않는다.

## 고정 Chrome oracle

Chrome for Testing `155.0.8059.40` / revision `cfaadc5a132d78e1828635aa8405a499f3e14864`, macOS arm64, agent-browser `0.31.1`, viewport `640×480` CSS px, DPR 1을 사용한다. [고정 source·screenshot SHA 및 전체 observation](../tests/fixtures/css/references/c13-2-scroll-container-precomparison-v1.json)은 두 HTML fixture의 bytes, computed overflow, rect, client/scroll dimensions, offset, root/body owner 결과를 보존한다. 재수집은 `bun run css:reference:c13-2-scroll-precomparison`이며 `bun run test:css-c13-2-scroll-precomparison`가 입력·캡처 artifact hash와 주요 경계를 고정한다.

| 비교 장면 | Chrome 155 기준 | 실패를 잡는 조건 |
| --- | --- | --- |
| visible child border escape | box `120×82`, client `114×76`; 자식은 border 바깥까지 paint, `scrollWidth=160`, offset write 후 `(0,0)` | visible paint를 scroll owner/offset으로 오인 |
| out-of-flow positive overflow | `overflow:auto`, `scrollWidth=178`, client width `114`, max x `64` | absolute child의 border box를 range에서 누락 |
| ink-only shadow | scroll/client `(114,76)`, max `(0,0)` | box-shadow를 scrollable overflow로 계산 |
| no-overflow auto와 scroll | 둘 다 range `(0,0)`; `auto` offset write 뒤 `(0,0)` | overflow가 없는데 offset 또는 range를 임의 생성 |
| one axis / nested | x max 86, y max 74; nested outer `(22,17)`, inner `(42,29)` | 축 혼합, ancestor/child offset 합산·소유권 혼선 |
| hidden와 clip | hidden scripted `(55,44)`; clip write `(0,0)` | clip을 hidden과 같은 programmatic scroll owner로 취급 |
| layout extent 변경 | growth 후 유효 `(30,20)` 보존; 큰 입력은 max `(146,124)`로 clamp; 축소 후 `(0,0)` | style/layout 재계산 중 offset 유실, range 밖 offset, 부분 clamp |
| node reparent | 같은 node remove/reinsert 뒤 `(0,0)` | tree detach 뒤 stale offset 재사용 |
| root/body | root visible에서 body auto는 viewport/HTML로 전파; `document.scrollingElement`는 HTML; html hidden/body auto에서 body만 자체 offset `(64,83)` | body computed style과 실제 viewport/body scroll owner를 혼합 |
| fixed child | viewport/root가 스크롤돼도 fixed rect `[580,444,52,28]` 유지 | root scroll offset을 fixed child에 잘못 적용 |

`scrollWidth`가 client width보다 커 보이는 것만으로 scrollable range가 있는 것은 아니다. 특히 `visible`은 border 밖 paint와 더 큰 `scrollWidth`를 동시에 가질 수 있지만 offset은 움직이지 않는다. 구현 비교는 owner 종류·scrollability·offset을 따로 대조한다. ink bounds와 scrollable bounds도 별도 입력으로 유지한다.

Chrome Headless DOM setter는 fractional `scrollLeft/scrollTop` 요청을 정수로 반올림하는 probe도 저장한다. 이 observed public CSSOM behavior는 C29/S05에 귀속하고 C13.2 internal adapter pass vector에는 넣지 않는다. C13.2 내부 state는 CSS px geometry를 사용하며 subpixel gesture 처리와 공개 setter normalization을 섞지 않는다.

## 구현 순서

1. **Typed overflow 보존:** cascade/layout projection에서 `visible`, `clip`, `hidden`, `scroll`, `auto`를 구분한다. mixed-axis computed value는 C13.1 normalization 결과와 일치해야 한다. 미지원 author value는 기본값으로 덮지 않고 실패시킨다.
2. **Scroll geometry:** 각 connected scroll owner의 client scrollport, content/descendant scrollable area, 축별 max range를 산출한다. positive absolute descendant 포함, ink-only 제외, C13.1 ancestor clip 적용 순서를 고정한다. 동일 axis를 제외한 숫자 분기와 비유한 산술은 오류 처리한다.
3. **State/revision:** owner별 `scrollLeft/scrollTop` 내부 state를 추가한다. initial zero, 유효 owner 유지, 새 max clamp, owner 제거/reparent reset을 단일 draft state에 적용한다. scroll owner/range/effective offset이 바뀔 때만 monotonic `ScrollStateRevision`을 올린다.
4. **Snapshot/presentation:** `RuntimeRenderKey`와 hit-test key에 `ScrollStateRevision`을 더한다. ancestor scroll offsets와 viewport scroll을 paint/hit 양쪽에서 같은 CSS px 변환으로 적용한다. stale key를 거부하며 fixed child에는 viewport offset을 적용하지 않는다.
5. **Root/body owner:** standards-mode root propagation 규칙을 내부 owner mapping에 구현한다. root visible/body auto의 viewport 전파와 html hidden/body auto의 독립 body scrollport를 분리한다. `document.scrollingElement` 공개 facade는 추가하지 않는다.
6. **오류 원자성:** range/owner vector reserve 실패, invalid geometry, stale request를 fault injection한다. 후보 offset clamp, scroll revision과 render snapshot은 모두 성공해야 함께 게시한다. 실패 시 이전 유효 state 유지 후 다음 정상 document/environment revision에서 복구한다.

## Pass/fail 기준

- **Oracle parity:** 새 opt-in runtime fixture는 위 모든 case의 computed overflow, scroll/client width·height, initial/programmatic offset, max range, root/body owner, fixed rect를 고정 JSON과 대조한다. 입력 고정 scene의 정수 CSS px 값은 exact 비교하고 non-integral layout rect만 최대 절대 오차 `0.5 CSS px`를 허용한다.
- **Scroll geometry:** no-overflow, x-only/y-only, nested, positive out-of-flow, ink-only, visible escape, hidden, scroll, auto, clip의 각 경계가 독립 assertion을 가진다. border 밖 paint sample은 실제 screenshot에 남기되 CSS `visible` overflow를 scroll state와 합치지 않는다.
- **Offset lifecycle:** 초기 0, 범위 내 scroll, negative/over-max clamp, layout growth 보존, shrink/overflow removal clamp, owner detach/reinsert reset, descendant move, deleted owner cleanup을 각각 검사한다. 동일 offset이어도 range/owner mapping이 변경됐으면 revision 변화를 확인한다.
- **Revision rejection:** 같은 document/style/environment revision에서 scroll revision만 다른 stale render와 hit query를 거부한다. 일치 key는 paint와 hit에 같은 transformed frame을 사용한다. offset no-op은 불필요한 revision advance를 만들지 않는다.
- **Atomic recovery:** offset/range/revision/snapshot 각 publish 직전 오류에서 이전 snapshot의 hash와 offsets가 그대로인지 확인한다. fault 이후 동일 worker의 유효 재계산이 전부 새 revision을 게시하고 mixed generation이 없음을 확인한다.
- **Root/body:** `document.scrollingElement` oracle와 내부 viewport/body owner를 각각 비교한다. body `auto` 전파, root `auto`, root `hidden` + body `auto`, default visible, fixed child scroll invariance를 포함한다.
- **Build 경계:** 새 Android/iOS FFI fixture entry, source payload는 전용 opt-in `c13-2-scroll-fixture-test` feature에 둔다. 기본 archive에서 symbol/payload absence, opt-in archive에서 지정 positive marker를 확인한다.
- **플랫폼 실행:** Chrome reference와 함께 API 37 ARM64 Android AVD 및 지정 iOS Simulator에서 실제 runtime → layout → render → paint/hit fixture를 실행한다. 각 실행은 device/simulator, app/source hash, viewport, surface evidence, runtime revision, failure/negative controls를 기록한다. 실기기는 요청되지 않았고 pass 조건으로 넣지 않는다.
- **회귀:** `cargo fmt --check`, 관련 Rust/reference tests, `cargo test --locked --workspace`, workspace Clippy, Android/iOS simulator build 및 지정 fixture run을 수행한다. 비교하지 않은 WPT·제품 CSSOM·gesture·scrollbar·snap은 미실행 경계로 남긴다.

## 미포함 항목과 완료 문구

- `scrollTop/scrollLeft`, `scroll()`, `scrollTo()`, `scrollBy()`, `scrollIntoView()`, scroll event 공개 계약: C29/S05.
- touch, wheel, keyboard gesture chaining/overscroll: C13.3 및 입력 owner.
- scrollbar thickness/gutter/thumb, `scrollbar-width/color`: C13.4.
- scroll snap/smooth completion: C13.5.
- negative RTL origin, vertical writing mode, logical scroll axes: C13.6.
- transform/filter/perspective, clip-path/mask, radius, line boxes, replaced content: 각 C15/C14/C18/C23 owner 계약 전 fail-closed.
- OS global OOM, mobile physical GPU/display scanout, WPT pass는 시뮬레이터/Chrome oracle 통과로 주장하지 않는다.

**허용 완료 문구:** “C13.2 내부 제한 fixture에서 Chrome 155 기준의 scroll owner, range, offset lifecycle, stale revision 거부 및 오류 복구를 API 37 ARM64 AVD와 iOS Simulator에서 검증했다.” 이 문구는 해당 증거가 모두 생긴 뒤에만 쓴다. 부모 C13과 제품 CSS 지원은 계속 미완료로 둔다.

## 검토 순서

이 구현 계획은 코드 전 20개 별도 실패 관점으로 검토한다. 계획 PR을 포함한 PR 변경은 프로젝트 규칙에 따라 기존 계획 리뷰와 독립적인 30개 관점으로 병합 전에 확인한다. 구현 PR에서는 이 계획 리뷰를 재사용하지 않고 새 구현 실패 관점 20개와 PR 변경 관점 30개를 각각 남긴다. 결함이 발견되면 계획·fixture·실행 결과를 갱신하고 영향을 받은 관점을 다시 확인한다.

# 0063 · C13.3 scroll gesture runtime 내부 계약

- **계약:** `0.1.0-draft` · **상태:** per-axis snapshot router, typed style projection, 단일 pointer pure Rust session 조각 구현 · 전체 gesture runtime 미완료
- **제품 CSS 지원 선언:** 아님
- **기준:** [C13.3 계획](../../plan/c13-3-scroll-gestures.md) · [Chrome 155 precomparison](evidence/c13-3-scroll-gesture-precomparison-2026-10-15/README.md)
- **기존 기반:** [0062 C13.2 scroll state](0062-c13-2-scroll-state.md)의 immutable snapshot·owner chain·`ScrollStateRevision`

현재 후보에는 policy 입력형 순수 Rust delta router, `RuntimeBlockFlexScrollGestureV1` computed-style profile, 단일 pointer `RuntimeScrollGestureSession`이 있다. profile은 `overscroll-behavior` 물리 축 shorthand/longhand와 `touch-action`을 typed output으로 보존하고 기존 C13.1 overflow layout projection을 유지한다. 제한 지원 집합 밖의 유효 `touch-action` 값은 `Unsupported`로 보존한다. style profile은 renderer scroll policy 및 session에 연결되지 않았고 session은 native queue, Android/iOS callback 또는 fixture FFI에 연결되지 않았다. 세 조각의 통과를 제품 제스처나 C13.3 완료로 해석하지 않는다. [router 실행 근거](evidence/c13-3-scroll-gesture-runtime-core-2026-10-15/README.md) · [typed style 실행 근거](evidence/c13-3-scroll-style-projection-2026-10-15/README.md) · [core session 실행 근거](evidence/c13-3-scroll-gesture-session-2026-10-15/README.md)

## 범위와 소유자

이 계약은 C13.1/C13.2 제한 box profile에 직접 touch pan 및 platform adapter가 CSS px로 정규화한 wheel/trackpad delta를 연결한다. scroll target/range/offset/snapshot은 C13, raw input 수집과 surface lifecycle은 Android/iOS host, JavaScript callback queue는 S05, presentation 확인 입력은 S04.10, 공개 CSSOM은 C29가 소유한다.

테스트용 native input route와 event log는 `c13-3-scroll-gesture-fixture-test`에서만 연결한다. 이 계약은 public `PointerEvent`, `WheelEvent`, `scroll` event, `Window`/`Element` scroll API, displayed-frame hit target, keyboard focus behavior 또는 OS native bounce/glow를 제공하지 않는다. S04.10 입력/displayed-frame 결합과 S05.2 adapter는 별도 미완료 경계다.

## Typed input style profile

- `overscroll-behavior`, `overscroll-behavior-x`, `overscroll-behavior-y`: `auto`, `contain`, `none`만 지원한다. shorthand는 CSS 규칙에 따라 두 물리 축에 매핑하며 initial은 `auto/auto`, non-inherited다.
- `touch-action`: 이번 profile은 `auto`, `none`, `pan-x`, `pan-y`만 지원한다. initial은 `auto`, non-inherited다. Stylo가 계산값을 보존하는 유효 방향 조합, `manipulation` 및 다른 합법 값은 `Unsupported`로 남기며 `auto`로 대체하지 않는다. pinned Stylo 0.22가 기본값으로 조용히 버리는 유효 `pan-left/right/up/down`과 동적 `var()` 선언은 원래 계산값을 잃기 전에 profile 오류로 거부한다.
- `touch-action` direct-pan allowed axes는 hit target에서 nearest inclusive scroll container까지의 supported value 교집합으로 결정한다. `none`은 빈 집합이다. 값과 축 결정은 gesture 시작에서 동결하며 active gesture 중 새 computed value는 current gesture에 적용하지 않는다. 해당 policy-only style 변경은 current gesture를 cancel하지 않는다.
- `overscroll-behavior`는 delta sample을 처리할 때 현재 committed style에서 읽는다. active gesture 중 policy 변경은 다음 delta sample부터 적용한다. geometry/owner/range 변경은 별도 geometry-key 비교로 취소한다.
- `touch-action`은 wheel/trackpad, keyboard, text selection, tap/click activation을 제한하지 않는다. keyboard 기본 scroll은 S05/S07 경계다.
- wheel adapter는 host가 scroll 의도로 분류한 wheel/trackpad-scroll만 받는다. source와 modifier flags를 기록하고, 플랫폼이 magnification/zoom으로 분류한 입력은 scroll state를 바꾸지 않고 거부한다. pinch/magnification은 이 profile 밖이다.
- unsupported style 또는 유효 ancestor 값 조합이 입력 target 경로에 있으면 해당 direct gesture를 fail-closed한다. 해당 style을 기본 `auto`로 조용히 복구하지 않는다.

## 입력 타입과 상태

모든 좌표·delta는 CSS px `f32`이며 finite여야 한다. Android physical pixel, UIKit point 또는 drawable pixel을 CSS px로 간주하지 않는다. adapter는 현재 viewport와 surface scale로 변환하고 그 scale/revision을 입력 log에 남긴다. touch move 좌표는 시작 뒤 viewport 바깥까지 이동할 수 있지만 시작점은 viewport 및 clip-aware hit-test 대상이어야 한다.

| 입력 | 필수 식별값 | 동작 |
| --- | --- | --- |
| `Begin` | 단조 증가 input sequence, 새 gesture ID, runtime/document/tree/environment/style revision tuple, surface generation, expected `ScrollStateRevision`, pointer ID, CSS viewport point 및 coordinate scale | 현재 snapshot에서 hit target, root/body effective viewport owner, scroll owner chain, scrollport/clip/range geometry key를 보존한다. `touch-action`은 이 시점의 computed value와 source revision을 동결한다. finite point와 tap/click pass-through도 검사한다. |
| `PanBegan` | input sequence, active pointer ID, cumulative CSS displacement, native slop source | supported touch-action과 native pan threshold를 통과한 첫 sample에서 active axis를 정하고 `Tracking`에서 `Panning`으로 한 번만 전이한다. |
| `Move` | gesture ID, input sequence, pointer ID, surface generation, current style revision, expected geometry key, CSS point 또는 normalized axis delta | geometry key가 같으면 frozen touch action과 queue-order상 현재 overscroll policy로 per-axis router를 원자 실행한다. pointer position 기반 input은 직전 accepted point에서 delta를 계산한다. expected `ScrollStateRevision`은 active session이 소유해 앞선 accepted Move의 revision으로 갱신한다. |
| `WheelDelta` | 단조 증가 input sequence, surface generation, runtime/document/tree/environment/style tuple과 enqueue 시점의 basis `ScrollStateRevision`, CSS viewport point/coordinate scale, CSS px `deltaX/deltaY`, scroll-intent 분류(`wheel` 또는 `trackpad-scroll`), modifier flags, source unit/scale/momentum phase | enqueue 시점 snapshot에서 point의 clip-aware hit target/owner chain을 정한다. 인접한 같은 snapshot/target/policy epoch의 wheel sample만 합칠 수 있다. 앞선 동일 input-queue commit 때문에 바뀐 revision은 queue lineage로 반영하되, unrelated scroll/style/tree/surface barrier 뒤 stale sample은 target을 재해석하지 않고 전체 거부한다. touch slop/axis-lock 없이 각 축을 독립 라우팅한다. host가 magnification/zoom으로 분류한 입력은 offset을 바꾸지 않고 거부한다. 공개 `WheelEvent` 단위는 받지 않는다. |
| `End` | input sequence, gesture ID, pointer ID, surface generation | 앞서 수락한 gesture sample 뒤 terminal 처리하고 상태를 해제한다. inertia/fling은 시작하지 않는다. |
| `Cancel` | input sequence, gesture ID, pointer ID 또는 generation-change reason | 명시된 cancel boundary 순서에서 transaction을 종료하고 이미 게시한 offset은 보존한다. 해당 gesture의 이후 move/end는 무시한다. |

내부 상태는 `Idle → Tracking → Panning → Ended/Cancelled → Idle`이다. `Begin` 전 target과 gesture identity가 검증되고, `End`/`Cancel`은 idempotent terminal 처리 후 pointer id 재사용을 막을 generation을 증가시킨다. tracking 중 pan slop을 넘지 않은 tap은 scroll offset/revision을 바꾸지 않는다.

Platform callback은 worker completion을 기다리지 않는다. gesture session은 `Begin`의 scroll revision을 baseline으로 소유하고 queue가 수락한 Move sequence마다 committed revision을 전진시킨다. 따라서 연속 OS Move callback이 앞선 publish 응답을 모른 채 도착해도 자신의 유효 session lineage에서 처리한다. 별도 scroll command나 style/tree/surface barrier는 ordered invalidation으로 session을 종료한다. wheel sample은 enqueue 시 사용한 snapshot/basis revision과 위치를 보존하며 같은 input burst의 이전 commit만 queue lineage로 따라간다.

축 선택은 Android `ViewConfiguration.getScaledTouchSlop()` 및 iOS `UIPanGestureRecognizer`의 `.began`을 native 시작 경계로 사용한다. 시작 sample의 translation에서 touch-action이 허용한 축 중 절대값이 큰 축을 latch하고, 값이 같으면 두 축을 모두 유지한다. gesture 중 새 축을 추가하지 않는다. 제한된 `pan-x`/`pan-y`는 해당 축만 선택 후보가 된다. CSS px slop/각도 상수는 두지 않는다. UIKit `translation(in:)`은 누적 displacement이므로 adapter가 직전 accepted translation을 빼 per-sample delta를 만들며, 같은 누적값 반복을 두 번 적용하지 않는다. adapter는 slop source/value 또는 recognizer state, raw/cumulative/differential translation, 최초 active sample, active axes를 증거에 남긴다.

두 번째 active direct pointer가 들어오면 기존 single-pointer gesture를 cancel한다. 기존 offset은 유지하고 마지막 active pointer까지 끝날 때까지 해당 touch stream에서 새 pan owner를 선택하지 않는다. Android adapter는 `MotionEvent` pointer count를, iOS host는 `UIPanGestureRecognizer`와 별도로 active `UITouch` identity 수를 감시해 두 번째 contact가 recognizer fail/transition 과정에서 누락되지 않게 한다. pinch zoom은 지원하지 않는다. 새 gesture는 기존 pointer들이 모두 종료된 뒤 새 ID와 현 revision tuple로 시작한다. `touch-action` 제한은 pan만 막고 slop 이전 tap/click activation은 host의 기존 경로에 보존한다. surface destroy 외에도 window/app deactivation, background와 host 종료는 active pointer set을 terminal 처리한다.

Native adapter의 bounded input queue는 모든 accepted input에 단조 증가 sequence를 부여한다. `Begin`은 active state를 게시하기 전에 해당 gesture가 사용할 terminal control slot 하나를 예약한다. `Begin` 실패 또는 terminal 처리 시 slot은 정확히 한 번 반환한다. End는 앞서 수락된 Move 뒤에, Cancel은 명시된 cancel boundary 뒤에 처리한다. 같은 gesture/pointer/generation과 동일 snapshot/style/policy epoch의 인접 `Move`만 마지막 point 또는 누적 delta로 coalesce할 수 있고 coalesced sample 수를 기록한다. `Begin`/`End`/`Cancel`과 style/snapshot/lifecycle barrier를 넘어 합치지 않는다. Move를 저장·coalesce할 수 없으면 예약 terminal slot으로 gesture를 cancel하고 마지막 게시 offset을 유지한다. WheelDelta는 같은 hit point/target/owner/revision/policy에서만 coalesce하며 capacity가 없으면 sample 전체를 reject하고 진단 카운터를 남긴다. UI thread는 Rust/render worker 완료를 동기 대기하지 않는다.

`GestureGeometryKey`는 runtime/document/tree/environment generation, surface generation, 시작 hit `NodeId`, root/body effective viewport owner, ordered scroll owner chain, ancestor hit/paint clip, scrollport rect 및 각 owner max range를 포함한다. 현재 offset, `ScrollStateRevision`, `touch-action`/`overscroll-behavior` 값, paint color는 key 자체에는 넣지 않는다. offset/style revision은 각 queued input에 별도로 stamp하고 처리 시 현재 snapshot과 대조한다. policy를 바꾼 input이 queue에 대기 중이면 이전 epoch delta를 먼저 처리하거나 입력을 거부해야 한다. touch action/overscroll property가 아닌 geometry-key constituent가 달라지면 active gesture를 취소한다.

## 현재 구현된 Rust 단일 pointer session slice

`spinon-render`는 공개 Rust 타입 `RuntimeScrollGestureSession`, `RuntimeScrollGestureBegin`, `RuntimeScrollGesturePanSample`, `RuntimeScrollGestureTerminal`과 결과·오류 타입을 제공한다. 이 API는 내부 `0.1.0-draft` 계약 조각이며 웹 Pointer/Wheel/scroll 이벤트 API가 아니다.

- Begin은 현재 `RuntimeRenderKey`, surface generation, 증가 input sequence, gesture/pointer ID, CSS viewport point, coordinate scale 및 cascade가 이미 계산한 제한 `RuntimeTouchAction`을 받는다. key가 snapshot과 다르거나 좌표/scale/action/hit/geometry 검증에 실패하면 active session을 만들지 않는다. owner chain은 target 자신이 owner일 때 포함하고 nearest-to-outer 순서를 유지한다.
- Pan sample도 현재 snapshot key, surface generation, sequence, gesture/pointer ID, scale과 cumulative CSS px translation을 받는다. host가 native slop 또는 recognizer `.began`을 판정한 뒤 첫 `pan_began`을 보낸다. 이 session은 native slop 값/source를 계산하거나 기록하지 않는다.
- 첫 pan은 시작 action과 translation의 dominant axis 교집합을 고정한다. 크기가 같은 대각선은 허용된 두 축을 유지한다. 허용 축이 없으면 `PanRejected`가 되며 같은 gesture에서 다시 축을 고르지 않는다. 후속 `pan_moved`는 직전 accepted cumulative 값과의 차이만 delta router에 넘긴다.
- 각 accepted pan은 기존 immutable snapshot delta router를 호출하고 결과 snapshot의 `ScrollStateRevision`을 session baseline으로 삼는다. no-op translation은 revision을 올리지 않는다. renderer worker나 플랫폼 표면으로 snapshot을 게시하는 queue는 포함하지 않는다.
- geometry key는 document generation/document/tree/environment/intrinsic revisions, surface generation, viewport, target frame/hit clip/order, ordered owner chain, owner parent/scrollport/range, clip layers를 비교한다. style revision·scroll revision·offset·touch-action·overscroll policy 값 자체는 key에서 제외한다. 따라서 geometry가 같은 style-only snapshot 교체는 시작 action을 유지하고 새 snapshot의 overscroll policy를 다음 accepted sample에 쓴다.
- document/tree/environment/surface/geometry 또는 외부 scroll revision이 달라진 pan sample은 현재 offset을 바꾸지 않고 active pointer를 suppressed set에 넣는다. 두 번째 pointer도 active pan을 취소하고 모든 contact terminal 전까지 새 pan을 막는다. sequence·gesture/pointer identity가 맞지 않는 terminal은 상태를 바꾸지 않는다.
- coordinate scale은 CSS px 변환에 사용하지 않는다. adapter가 이미 CSS px로 정규화한 좌표에 scale identity를 함께 전달하며, active gesture 중 scale이 bitwise 달라지면 surface barrier로 취소한다.

이 slice에는 native input queue/coalescing/terminal reservation, wheel adapter, touch-action ancestor cascade 연결, Android/iOS callback, FFI fixture route, S04.10 displayed-frame hit target, 앱 화면이 없다. Android AVD와 iOS Simulator 검증을 수행하지 않았다. focused Rust unit tests는 위 state/session 경계만 확인하며 Chrome oracle나 화면 적합성 근거가 아니다.

## 축별 delta 소비와 snapshot 게시

finger movement `(dx,dy)`에 적용할 scroll delta는 `(-dx,-dy)`다. CSS px fraction을 중간에서 정수화하지 않는다. `WheelDelta` 부호는 adapter가 platform 입력 방향에 맞춰 W3C wheel CSS 좌표계 기준으로 정규화한다.

C13.2 owner 의미를 유지한다. `overflow:auto`/`scroll` owner만 user delta를 offset에 소비할 수 있다. `overflow:hidden`은 C13.2의 programmatic-only owner로서 user delta를 소비하지 않는 boundary다. 그 owner의 `overscroll-behavior`가 `auto`면 남은 축 delta가 다음 ancestor로 가고 `contain`/`none`이면 멈춘다. `overflow:clip`/`visible`은 scroll owner가 아니므로 자체 offset이나 overscroll policy 없이 조상 chain을 계속 찾는다.

각 축에 대해 다음 순서를 수행한다.

1. frozen hit target에서 시작한 scroll owner chain을 inner→outer 순으로 방문한다. owner 없는 node나 `visible`/`clip` box는 offset을 갖지 않는다.
2. requested offset을 해당 owner `[0,max]`로 clamp한다. `consumed = newOffset - oldOffset`, `remaining = requested - consumed`로 계산한다.
3. 남은 값이 0이면 그 축 순회를 끝낸다. `auto`면 다음 ancestor로 남은 delta를 넘긴다. `contain`/`none`이면 해당 축의 non-local chain을 멈춘다. 반대 축은 계속 독립 계산한다.
4. 범위가 0인 축은 항상 boundary다. `auto`이면 남은 delta를 parent로 전하고, `contain`/`none`이면 멈춘다.
5. 지원하지 않는 owner/policy/유한성 검증 실패는 후보 offset map 전체를 버린다. 변경된 inner만 또는 한 축만 먼저 게시하지 않는다.

한 native input sample에서 여러 owner가 움직여도 offset candidate와 per-owner internal change record 용량을 모두 예약·검증한 뒤 최종 snapshot을 한 번에 publish하고 `ScrollStateRevision`을 최대 한 번 올린다. offset이 모두 같으면 revision은 유지한다. per-owner internal change record는 최종 revision과 같은 입력 sequence에 연결하고 leaf→ancestor consume order를 보존한다. record allocation/revision exhaustion은 offset이나 event record 어느 쪽도 게시하지 않는다. public JS `scroll` event dispatch는 하지 않는다.

## 수명·취소·stale 처리

- active gesture는 runtime/document/tree/environment revision, surface generation, 시작 hit target/root-body effective owner 및 scroll owner/clip/scrollport/range geometry key에 고정한다. style revision이 바뀌어도 geometry key가 그대로면 gesture를 유지한다. `touch-action`은 시작 policy를 유지하고 `overscroll-behavior`는 생성 시 snapshot/style revision이 지정한 다음 sample부터 적용한다. geometry key, tree/document/environment 또는 owner mapping이 달라지면 gesture를 cancel하고 다음 입력은 새 snapshot에서 시작한다.
- gesture 자신의 accepted `Move`/`WheelDelta`가 올린 `ScrollStateRevision`은 ordered queue lineage 안에서 current expected revision으로 갱신한다. UI callback은 worker publish 응답을 기다리지 않는다. 외부 명령이 별도 scroll revision을 올리면 invalidation barrier로 진행 중 gesture를 cancel하고 stale wheel sample은 다시 hit-test하지 않고 거부한다.
- surface destroy, resize/rotation에 따른 새 generation, window/app deactivation/background와 host 종료는 active pointer set을 terminal 처리하고 예약 slot을 반환한다. old generation의 late move/end는 offset/revision/event를 바꾸지 않는다. 같은 connected NodeId owner가 남으면 C13.2 보존/clamp 규칙으로 현재 offset을 유지하고, document owner가 제거/재생성되면 C13.2 초기화 규칙을 쓴다.
- pointer cancel, native recognizer failure, second pointer, invalid coordinate, allocation/revision exhaustion은 부분 publish 없이 마지막 성공 snapshot으로 끝난다. 다음 정상 generation/revision에서 새 입력은 같은 host를 복구할 수 있어야 한다.
- active `touch-action` 변경은 현재 gesture에 영향을 주지 않고 cancel 사유도 아니다. `overscroll-behavior` 변경은 다음 input sample에서 적용한다. 그 외 style revision 변경은 geometry key가 유지되는지 검사한다. key가 달라지면 cancel하고, key가 같으면 hit target/owner chain은 시작 상태를 유지한 채 현재 render snapshot에 offset만 원자 적용한다.
- no-op move, duplicate terminal, stale ID, unknown owner, non-finite delta, out-of-order state transition은 별도 오류/무시 결과를 기록하고 scroll event record를 만들지 않는다.

## 경계 affordance와 입력 범위

CSS Overscroll 초안은 `contain`과 `none`이 ancestor chaining을 막고, `none`만 local overscroll affordance까지 막는 차이를 정의한다. 현재 C13.3 runtime에는 local glow/bounce가 없으므로 offset은 range에서 clamp되고 그 차이를 그려내지 않는다. `auto`는 남은 delta를 ancestor로 넘기며 `contain`/`none`은 해당 축 chain을 막는다. 이후 local affordance 구현은 별도 변경과 AVD/iOS Simulator 실행으로 검증한다.

touch pan, host가 scroll 의도로 분류한 normalized wheel/trackpad delta와 기록된 momentum phase, cancel, multi-pointer cancellation만 범위다. Spinon이 touch release 뒤 inertia/fling을 생성하지 않는다. host magnify/zoom input, keyboard focus/default scroll, pinch zoom, scroll snap, scrollbar, RTL/logical axis, public Pointer/Wheel/scroll event는 별도 owner에 남긴다.

## Oracle·fixture·완료 관문

[Chrome 155 사전 비교](evidence/c13-3-scroll-gesture-precomparison-2026-10-15/README.md)는 400×240 agent-browser fixture, source/screenshot hash, direct touch/wheel offset·event/pointer trace를 고정한다. inner `overflow:hidden`은 gesture offset을 소비하지 않고 `auto` remainder를 outer로 넘겼으며 `clip`/`visible`도 자체 offset 없이 outer로 연결됐다. 주요 oracle 차이는 diagonal `overscroll-x:contain; overscroll-y:auto`에서 세로 parent chaining이 막힌 것이다. 내부 contract 테스트는 x contain은 x만 차단하고 y auto는 y remainder를 전달하는지 명시적으로 검증한다. Chrome 결과를 제품 적합성 pass로 쓰지 않는다.

구현 PR은 다음을 모두 연결한다.

- Rust 단위 검증: per-axis consumed/unconsumed, no range, two ancestors, diagonal, exact revision count, error atomicity, stale revisions, cancellation, pointer reuse, surface generation.
- Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 동일 fixed fixture 실제 app route: inner/outer/root offsets, scroll revision sequence, touch-action, axis lock, `auto/contain/none`, diagonal remainder, cancel/multipointer, app lifecycle/generation change, layout/style invalidation, native slop 전 tap/click pass-through. `touch-action:none` 아래 native slop 전 tap은 기존 activation을 한 번 발생시키고 offset은 바꾸지 않으며, pan 시작 뒤에는 tap activation이 발생하지 않아야 한다. iOS recognizer가 이 tap을 선점하지 않는지도 본다. host가 magnify/zoom으로 분류한 입력은 scroll state를 바꾸지 않는다. Simulator에 실제 mouse/trackpad가 없으면 wheel sample 주입 결과를 native wheel 입력인 것처럼 보고하지 않는다.
- App/source hash, simulator identity, CSS viewport/surface scale, render/scroll revision, input log, 화면 screenshot. 실제 device/hardware GPU가 아닌 AVD/Simulator로 표시한다.
- fixture code/FFI/payload는 opt-in `c13-3-scroll-gesture-fixture-test`에만 넣고 default artifact negative control 및 opt-in positive control을 확인한다.
- `cargo fmt --all -- --check`, focused tests, `cargo test --locked --workspace`, workspace Clippy, 관련 CSS reference tests와 `git diff --check`.
- [S04.10](../STATUS.md) displayed-frame confirmation/input correlation과 [S05.2](../STATUS.md) public adapter/JS events는 미검증으로 계속 분리한다.

통과 결과는 이 제한된 내부 gesture router와 두 Simulator fixture만 가리킨다. 제품 CSS 적합성, public DOM event/API, optical scanout, real-device/hardware GPU, C13.1/C13 전체 완료를 의미하지 않는다.

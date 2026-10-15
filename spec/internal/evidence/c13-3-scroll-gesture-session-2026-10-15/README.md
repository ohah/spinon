# C13.3 core gesture session 사전 판정

2026-10-15에 시작하는 core-only session slice다. 비교 입력 기반은 [고정 Chrome 155 touch/wheel oracle](../c13-3-scroll-gesture-precomparison-2026-10-15/README.md)이고, renderer state oracle는 기존 `RuntimeRenderSnapshot`·`ScrollStateRevision` 불변 조건이다. 이 단계는 app/native 실행이 아니며 Chrome의 direct gesture 차이를 제품 기대값으로 승격하지 않는다.

## 대상과 경계

- `Begin`에서 현재 clip-aware hit target, nearest-to-outer scroll owner chain, surface generation, document/tree/environment/style/scroll revision tuple, coordinate scale과 resolved `touch-action`을 캡처한다.
- platform adapter가 native slop 통과를 알린 `PanBegan`부터 CSS px finger translation을 scroll delta로 바꾸고 시작 action으로 허용 축을 고정한다. core가 플랫폼별 slop/recognizer를 재구현하지 않는다.
- cumulative pan translation은 직전 accepted translation에서만 빼서 한 번 적용한다. 새 immutable snapshot과 scroll revision을 다음 Move baseline으로 전진시킨다.
- geometry가 같은 style-only 변경은 시작 `touch-action`을 유지하고 현재 snapshot의 overscroll 정책을 다음 sample부터 쓴다. geometry, tree/document/environment, surface generation 또는 외부 scroll revision이 바뀌면 현재 gesture를 cancel한다.
- 두 번째 pointer는 active gesture를 취소하고 모든 접촉이 끝날 때까지 새 pan 시작을 막는다. End/Cancel 및 stale/중복 input은 순서와 identity를 확인한다.
- input queue, coalescing/terminal reservation, wheel adapter, native slop, FFI, app route, displayed-frame target 연결은 범위 밖이다.

## 사전 pass/fail 기준

1. finite in-viewport Begin은 snapshot의 clip-aware hit test와 일치해야 한다. hit miss, stale expected key, invalid coordinate/scale, unsupported touch-action은 session state를 만들지 않는다.
2. Begin은 inclusive target owner와 nearest-to-outer ancestor chain을 고정한다. 후속 hit test를 다시 하지 않는다.
3. PanBegan이 허용된 dominant axis만 latch하고 equal-magnitude diagonal은 허용 축 교집합을 latch한다. 반대 방향으로 축을 추가하지 않는다. disallowed dominant direction과 `none`은 scroll하지 않는다.
4. cumulative translation sample은 delta 한 번만 소비한다. no-op은 snapshot revision을 올리지 않고 실제 offset 변경은 기존 delta router의 atomic update를 사용한다.
5. 다음 sample은 session이 소유한 최신 `ScrollStateRevision`을 사용한다. unrelated external scroll revision은 gesture를 취소하고 추가 offset을 적용하지 않는다.
6. style revision만 바뀌고 geometry가 같으면 session은 유지된다. touch-action은 시작 값을 유지하고 다음 Move의 overscroll은 새 snapshot policy를 따른다. target frame/clip/owner/range, document/tree/environment, surface generation 변경은 cancel한다.
7. 두 번째 pointer는 기존 pointer와 새 pointer를 blocked set에 넣고, 모두 종료되기 전에는 새 gesture를 만들지 않는다. 정확한 pointer/gesture identity와 strictly increasing input sequence를 요구한다.
8. End/Cancel은 terminal 한 번만 허용한다. cancel은 마지막 정상 snapshot을 보존하고 늦은 move/end는 offset/revision을 바꾸지 않는다.

이 기준은 순수 Rust state/session 검사다. Android API 37 AVD, iPhone 17 Pro iOS 26.2 Simulator 및 앱 화면은 이 단계에서 실행하지 않는다.

## 구현 및 검증 결과

단일 pointer `RuntimeScrollGestureSession`을 `spinon-render`에 추가했다. Begin은 현재 snapshot의 clip-aware target과 ordered owner chain을 고정한다. host가 native pan 시작을 통지한 뒤 제한 `touch-action` 축으로 cumulative CSS px translation을 기존 snapshot delta router에 전달한다. geometry/document/tree/environment/surface/external scroll revision 경계에서는 late delta 없이 취소하고, 두 번째 pointer와 suppressed contact terminal을 identity로 추적한다. style-only snapshot은 시작 action을 유지하고 새 snapshot의 overscroll policy를 다음 accepted sample에 적용한다.

적대 검토에서 두 격리 결함을 재현해 수정했다. suppressed contact의 임의 큰 gesture ID가 다음 정상 Begin을 막던 동작을 제거했고, 다른 surface generation의 terminal이 suppressed pointer를 해제하지 못하도록 gesture ID와 surface generation을 함께 확인한다. 각각의 회귀 검사는 focused suite에 포함돼 있다.

- `cargo test --locked -p spinon-render scroll_gesture_session_tests -- --nocapture`: 16 passed, 0 failed. [전체 로그](focused-test.log)
- `cargo test --locked --workspace --quiet`: 837 passed, 0 failed, 4 ignored across 26 test suites. [전체 로그](workspace-test.log)
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과. [로그](workspace-clippy.log)
- `cargo fmt --all -- --check`: 통과. [로그](rustfmt.log)
- `git diff --cached --check`: 전체 staged diff 통과. [로그](diff-check.log)
- 실패 경로 20개 구현 검토: [implementation-review-20.md](implementation-review-20.md)
- PR 변경 관점 30개 검토: [pr-delta-review-30.md](pr-delta-review-30.md)

검증은 순수 Rust session 경계에 한정된다. Chrome oracle는 동작 화면 증거가 아니며, 이번 slice에는 앱 route가 없어 Android/iOS Simulator 실행이나 Spinon 앱 화면 캡처를 만들지 않았다. Native queue/coalescing/terminal reservation, style profile 연결, Android/iOS adapter, FFI fixture와 S04.10 displayed-frame target은 남아 있으므로 C13.3은 미완료다.

## 병합 기록

구현 [PR #180](https://github.com/ohah/spinon/pull/180)은 2026-10-15 `Rebase and merge`로 `main`에 병합됐다. merge commit은 `2d9f406815768e8026f7b506e4b112811cd2a66a`다. C13.3의 플랫폼/fixture/displayed-frame gate는 남아 있어 상태를 완료로 올리지 않았다.

병합 상태 문서 delta 검토: [30개 PR 검토](pr-merge-record-review-30.md).

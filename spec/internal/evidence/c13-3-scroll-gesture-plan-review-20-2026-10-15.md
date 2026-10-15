# C13.3 계획 실패 관점 20개 검토 · 2026-10-15

- **검토 종류:** 구현 전 계획 실패 관점 20개
- **기준:** `plan/c13-3-scroll-gestures.md`, `spec/internal/0063-c13-3-scroll-gestures.md`, 고정 Chrome 155 reference와 캡처 도구
- **기준 commit:** `edf705786ed3a438e53fda71efd5f366e1b27d7a`
- **oracle 검사:** `bun run test:css-c13-3-scroll-gesture-precomparison` — 5/5 통과
- **상태:** 발견한 계획 결함을 문서에 반영했다. C13.3 런타임 구현과 Android/iOS Simulator 검증은 시작하지 않았다.

| # | 적대 관점 / 실패 시나리오 | 계획의 차단 기준과 확인 |
| --- | --- | --- |
| 1 | Android physical px, UIKit point, drawable px를 CSS px로 혼동해 hit 위치나 delta가 배율만큼 틀어진다. | 입력 계약은 CSS px와 coordinate scale/revision을 함께 기록한다. 양 플랫폼 실행 gate가 raw/cumulative/differential 좌표와 변환을 요구한다. |
| 2 | 시작점이 viewport 밖이거나 ancestor clip에 가려진 target인데 gesture owner를 만든다. | Begin은 viewport와 clip-aware hit-test를 통과해야 한다. 이동 좌표만 시작 뒤 viewport 밖을 허용한다. wheel은 sample 좌표에서 별도로 hit-test한다. |
| 3 | 자식 `touch-action`만 보고 조상 정책을 빠뜨리거나 scroll container 너머 조상까지 잘못 교집합한다. | target부터 nearest inclusive scroll container까지 경로를 정의하고, inner-content `pan-x` fixture와 바깥 outer 경계 control을 둔다. |
| 4 | profile 밖 `touch-action` 값이나 조상 조합을 `auto`로 바꾸어 의도치 않은 pan을 허용한다. | unsupported value는 진단과 함께 fail-closed하며 기본값으로 조용히 복구하지 않는다. |
| 5 | 앱 상수 slop이나 매 Move마다 바뀌는 축 선택 때문에 짧은 흔들림이 scroll이 되거나 축이 튄다. | Android scaled touch slop과 iOS recognizer `.began`을 시작 경계로 쓰고 최초 active 축을 latch한다. 동률과 restricted pan 축 규칙이 고정돼 있다. |
| 6 | UIKit cumulative translation을 sample delta로 오해해 매 콜 누적량을 재적용한다. | adapter가 직전 accepted translation을 빼며, 같은 누적값 반복 no-double-apply negative control을 Rust와 Simulator gate에 둔다. |
| 7 | finger delta와 scroll delta 부호, fractional offset 또는 inner/outer 소비 합계가 틀린다. | finger movement 부호를 반전하고 fractional CSS px 보존, 양/음·경계·finite 입력, `consumed + remaining = requested`와 원자 검증을 요구한다. |
| 8 | X가 `contain`이어도 대각 입력의 Y remainder까지 막거나 반대로 X를 넘긴다. | 두 축을 독립 계산한다. Chrome diagonal 차이는 별도 discrepancy이고 계약 pass는 Y auto remainder 전달을 요구한다. |
| 9 | 안쪽 boundary에서 남은 delta가 한 조상에서 멈추거나 root/body effective owner를 건너뛴다. | mid-range handoff, boundary 시작, 같은 gesture의 clamp, 두 ancestor와 root를 구분하고 root/body owner를 geometry key에 넣는다. |
| 10 | `hidden`/`clip`/`visible`을 같은 scroll owner로 처리하거나 `hidden` offset을 직접 gesture로 움직인다. | C13.2 owner semantics를 그대로 적용한다. hidden은 입력을 소비하지 않는 boundary, clip/visible은 자체 owner 없는 상자로 분리한다. |
| 11 | wheel 위치·단위·방향을 잃거나 trackpad 확대를 scroll로 라우팅한다. | 각 sample의 CSS viewport point, 단위/scale, source, momentum phase, modifier와 scroll-intent를 보존한다. host가 magnification/zoom으로 분류한 입력은 offset 변경 없이 거부한다. |
| 12 | 연속 같은 queue input이 앞선 scroll commit 때문에 false-stale가 되거나, 외부 update 뒤 오래된 target을 새 위치에서 재해석한다. | active gesture는 accepted revision lineage를 소유한다. 동일 queue 선행 commit만 lineage로 반영하고 unrelated scroll/style/tree/surface barrier 뒤 sample은 재-hit-test 없이 거부한다. |
| 13 | Move/wheel coalescing이 policy 또는 snapshot 경계를 넘어 delta를 합쳐 다른 owner로 이동시킨다. | 인접 sample의 gesture/source/target/owner와 snapshot/style/policy epoch 일치를 요구한다. End/Cancel 및 lifecycle/style/snapshot barrier를 넘지 않는다. |
| 14 | `touch-action` 변경을 현재 pan에 소급하거나 geometry가 바뀌었는데 active chain을 계속 쓴다. | touch-action은 시작 때 동결, overscroll policy는 다음 revision-stamped sample부터 적용한다. geometry key 변화만 취소 원인으로 분리한다. |
| 15 | resize/rotation, surface 재생성, background 복귀 뒤 이전 pointer의 늦은 Move가 새 화면을 변경한다. | surface generation과 app lifecycle을 key로 삼고 old generation terminal 처리 및 stale input 거부, 새 generation recovery를 요구한다. |
| 16 | offset은 publish됐지만 internal record allocation 또는 revision exhaustion에서 실패해 관찰 로그가 상태와 달라진다. | 모든 owner offset과 record 용량을 먼저 예약·검증하고 snapshot 및 revision을 한 번에 게시한다. 실패 시 둘 다 게시하지 않는다. |
| 17 | bounded queue 포화, 합치기, terminal 반환 누락 때문에 gesture가 영구 active가 되거나 일부 delta만 적용된다. | Begin 전 terminal slot을 예약해 실패/종료 때 정확히 한 번 반환한다. Move saturation은 예약 cancel, wheel saturation은 sample 전체 reject와 진단으로 정한다. |
| 18 | 이미 pan 중 두 번째 pointer가 들어왔는데 Android/iOS adapter가 누락해 한 손가락 scroll 또는 pinch가 계속된다. | core policy는 cancel·offset 보존·나머지 contact movement 차단을 정한다. AVD MotionEvent count와 iOS active UITouch set을 감시하고 active-pan 중 두 번째 contact를 simulator gate에서 직접 넣는다. Chrome의 simultaneous-start trace를 그 증거로 대체하지 않는다. |
| 19 | `touch-action:none`이 slop 전 탭을 억제하거나 iOS pan recognizer가 기존 tap handler를 선점한다. | tap/click은 pan과 별개로 보존한다. 이번 검토에서 두 Simulator에 slop 전 `none` tap 1회 활성화·offset 무변경 및 pan 시작 뒤 tap 비활성화를 명시했다. |
| 20 | Chrome fixture, 내부 change record 또는 opt-in simulator ABI를 제품 CSS/API 지원으로 과장한다. | Chrome oracle, 앱 simulator trace, public DOM event/CSSOM/S04.10을 분리한다. feature-test 기본 artifact negative control과 opt-in positive control을 구현 PR 완료 조건으로 남긴다. |

## 판정

계획은 입력 좌표, hit target, touch-action 축 제한, per-axis owner handoff, queue/revision 원자성, platform lifecycle과 검증 범위를 서로 다른 실패 경로로 나눈다. 검토 중 발견한 post-cancel CDP 제한은 Chrome 관찰 표에 harness 거부로 기록했고, active-pan multipointer 결과를 Chrome oracle가 입증했다고 주장하지 않는다. tap arbitration과 zoom intent의 누락도 계획/계약 관문에 추가했다. 이 문서는 계획 검토이며 구현 실패 경로, 앱 실행, Android/iOS 동작을 통과했다는 뜻이 아니다.

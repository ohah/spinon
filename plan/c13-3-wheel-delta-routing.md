# C13.3 normalized wheel delta routing 계획

## 목표와 경계

고정 C13.3 400×240 CSS fixture에서 이미 queue에 들어오는 `WheelDelta`를 실제 immutable render snapshot의 scroll router까지 처리한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 opt-in 앱 fixture가 같은 normalized CSS px 입력을 넣어 화면 결과와 로그를 남긴다. 이 입력은 OS가 보낸 native wheel/trackpad event가 아니다. 일반 앱 입력 경로, OS별 delta normalization, public `WheelEvent`/`scroll` callback, keyboard scrolling, fling, zoom/magnification gesture는 범위에 포함하지 않는다.

작업 구현 전에 별도로 기록한 실패 관점 20개는 [계획 검토](../spec/internal/evidence/c13-3-wheel-delta-routing-plan-review-20-2026-10-15.md)에 둔다.

## 비교 기준과 고정 입력

- 기준은 `tests/fixtures/css/references/c13-3-scroll-gesture-precomparison-v1.json`의 Chrome for Testing `155.0.8059.40` 관찰과 `plan/c13-3-scroll-gestures.md`의 per-axis contract다. 원본 캡처는 Agent Browser `0.31.1`, 400×240 CSS px, DPR 1, fixture SHA-256 `a4eff62477a8a5d01f58f8033472fd3dcd6e8bc7638723d3386f7060d5787c09`다. 해당 JSON의 DOM event는 Chrome oracle에만 속한다.
- 입력 point는 inner content의 `(132,112)` CSS px다. boundary scenario는 inner `(86,106)`, outer/root `(0,0)`에서 시작한다. `WheelDelta`는 point에서 clip-aware target을 enqueue할 때 한 번 정하고, 처리 중 재-hit-test하지 않는다.
- `wheelAutoDiagonal`: `(40,40)` CSS px, `auto/auto`; Chrome 기대 offset은 inner `(86,106)`, outer `(40,40)`, root `(0,0)`이다.
- `wheelXContainYAutoDiagonal`: `(40,40)` CSS px, inner `contain/auto`; Chrome은 `(0,0)` remainder를 관찰했지만 구현 oracle은 C13.3의 축별 계약을 따른다. 따라서 inner `(86,106)`, outer `(0,40)`, root `(0,0)`이어야 한다. 이 oracle 차이는 근거에 보존하고 숨기지 않는다.
- `wheelTouchActionNone`: initial offset 0, `(0,40)` CSS px, touch-action `none`; touch-action은 wheel을 제한하지 않으므로 inner `(0,40)`, outer/root `(0,0)`이어야 한다.
- 이 세 입력만으로 전체 C13.3 또는 OS wheel 지원을 완료로 판정하지 않는다.

## 구현 범위

1. `c13-3-gesture-fixture-test` 전용 FFI에 normalized wheel enqueue 경로를 둔다. 입력에는 sequence, surface generation, CSS point/delta, enqueue snapshot key와 basis scroll revision, source/unit/raw delta/coordinate scale, momentum phase, modifier flags를 기록한다. `wheel`/`trackpad-scroll` 의도만 받으며 host가 zoom/magnification으로 분류한 intent는 queue에 넣지 않고 거부한다.
2. enqueue 시 snapshot에서 finite point를 hit-test하고 target을 고정한다. queue consumer는 stored target에 router를 호출하고 결과 snapshot을 publication path로 보낸다. 축별 remainder는 현재 C13.3 per-axis 정책을 따른다. touch-action은 이 입력을 막지 않는다.
3. queue에서 앞서 수락한 pan/wheel commit이 바꾼 scroll revision은 같은 render/style/surface lineage의 accepted input에 한해 이어간다. 스타일·tree·environment·surface barrier와 별도 snapshot publication 뒤의 stale sample은 target을 다시 계산하지 않고 거부한다. rejected sample은 lineage/revision을 바꾸지 않는다.
4. queue의 인접 sample coalescing 및 saturation은 기존 `RuntimeScrollGestureInputQueue` 계약을 유지한다. 서로 다른 source, phase, unit, point, target, modifiers, style/policy epoch를 병합하지 않는다. wheel pressure는 sample 전체를 거부하며 gesture terminal reservation에는 영향을 주지 않는다.
5. Android/iOS opt-in fixture runner는 fixed probe flag가 있을 때만 같은 세 입력을 순서대로 enqueue/process한다. 완료 보고에 `synthetic_normalized_fixture_input=true`를 분명히 남긴다. 실제 wheel/trackpad input이라고 표시하지 않는다.

## 관찰값과 사전 판정

- 각 simulator run에서 AVD/Simulator 및 OS 버전, opt-in build flag, app/library hash, scenario, CSS point/delta, source/unit/phase, scale, sequence, enqueue target/key/basis, admission, dequeue/coalescing count, before/after key 및 owner offset을 함께 기록한다.
- 위 3개 fixed scenario의 final offsets는 정확히 일치해야 한다. delta가 owner들에 소비된 합은 각 축의 scrollable/chain-stop 경계 결과와 일치해야 한다. 수평/수직·대각 입력은 독립 결과를 기록한다.
- finite가 아닌 point/delta/scale, hit miss, malformed enum/intent, zoom intent, 잘못된 target, style/tree/environment barrier 이후 stale key는 offset과 revision을 바꾸지 않고 거부해야 한다. stale sample은 원래 target을 유지하며 새 위치에서 hit-test하지 않는다.
- 같은 input queue의 선행 accepted scroll update 뒤의 queued wheel sample은 ordered lineage 안에서 한 번 처리되고, 외부 snapshot barrier 뒤에는 fail-closed해야 한다. queue capacity 거부·overflows 후의 sample과 fresh gesture가 host를 복구해야 한다.
- 각 플랫폼 screenshot은 실제 fixture surface를 캡처하고 inner/outer/root border 안에서 기대 content 이동을 보여야 한다. 화면 바깥 pixel은 변하지 않아야 한다. 스크린샷은 입력을 synthetic fixture injection으로 얻었다고 표시한다.
- feature-off Android/iOS artifact에는 wheel fixture symbol/string/argument path가 없어야 하며, opt-in positive control에는 있어야 한다.

## 실행 및 남은 gate

Chrome 결과는 고정 reference JSON 및 Agent Browser 재현물과 대조한다. Spinon 입력은 API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서만 고정 fixture adapter를 통해 주입하고, Agent Browser를 제품 화면 결과를 읽는 브라우저 증거에 사용한다. 터미널 브라우저는 사용하지 않는다. 실기기 연결은 요구하지 않는다.

통과해도 증명 범위는 내부 C13.3 normalized wheel sample의 bounded queue→snapshot router→화면 publication 경로뿐이다. Android `AXIS_VSCROLL`/iOS scroll-wheel OS event 수집, 단위별 host normalization, 실제 trackpad momentum/magnification 분류, public web event/API, WPT, compositor 이후 scanout, 전체 CSS box profile 및 C13.3 완료 판정은 남는다.

# C13.3 Android/iOS 입력 어댑터 구현 실패 관점 검토 20개

검토일은 2026-10-15다. 대상은 `feat/c13-3-platform-gesture-adapters-20261015`의 현재 변경분이며, 이미 병합된 router/style/session/queue 검토 표를 재사용하지 않고 platform callback부터 FFI host·snapshot 게시까지 실패 경로를 다시 확인했다. `확인`은 코드 또는 실행 근거로 경계가 확인됐다는 뜻이고, `정적`은 해당 실패를 의도적으로 주입한 실행 근거가 없다는 뜻이다.

## 검토

1. **host 생성 인수 실패 — 확인.** C13.3 FFI host 생성은 null output 또는 0 capacity에서 null을 반환하고, Rust host 생성 실패를 report에 쓴다. registry 등록 전 report 쓰기가 실패하면 아직 raw `Box`로 넘기지 않아 로컬 host가 해제된다. 근거: `crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_3.rs`.
2. **잘못된 host 또는 해제된 registry key — 확인.** enqueue는 registry에서 state를 찾지 못하면 `ERR_ARGUMENT`로 끝나고, eval/process는 null host를 검사한다. host free는 registry entry를 제거한 뒤 host를 해제한다. 동시 free는 C ABI 안전 계약상 금지한다. 근거: 같은 FFI 파일, `runtime_gpu/ffi/surface.rs`.
3. **output 포인터·길이·timeout·drain 크기 오류 — 확인.** eval은 null/빈 output과 timeout 0, process는 null/빈 output과 max_events 0을 거부한다. Android/iOS bridge는 32 KiB report buffer를 제공한다. 근거: `c13_3.rs`, `spinon_jni.cc`, `SpinonRunner.mm`.
4. **알 수 없는 action — 확인.** 0–4 이외 입력은 queue에 넣지 않고 오류를 반환한다. null handle과 action 9 거부는 `c13_3_ffi_rejects_invalid_handles_and_actions`에서 확인된다.
5. **잘못된 cancel reason — 확인.** End/Cancel의 reason은 0–4로 제한되고, 다른 값은 enqueue 전 거부된다. 네이티브 callback별 reason은 lifecycle·second-pointer·saturation·recognizer 실패에 대응한다. 근거: `c13_3.rs`와 Java/Swift adapter.
6. **평가 전 snapshot 누락 — 확인.** Begin/motion enqueue는 현재 immutable snapshot이 없으면 거부한다. 앱 route는 고정 scene의 ready report를 확인한 뒤 touch adapter를 준비 완료로 표시한다. 근거: `c13_3_scroll_gesture_host.rs`, `android-api37-avd-ready.png`, `ios-iphone17pro-ready.png`.
7. **입력 sequence 소진 — 확인.** Atomic sequence는 `checked_add`로 증가하고 `u64` 최대에서 wrap하지 않고 오류를 돌려준다. 근거: `C13_3GestureState::next_sequence`.
8. **잘못된 scene owner 또는 geometry — 확인.** 고정 scene은 frame 수·scroll owner 수·부모 관계·각 축 max offset을 검사하고, 불일치 시 ready 상태를 게시하지 않는다. 근거: `eval_c13_3_scroll_gesture_fixture`와 `ios-c13-3-touch-summary.log`, Android ready log.
9. **surface 크기·scale 누락 또는 비균일 — 확인.** Android는 surface가 준비되지 않았거나 x/y scale이 다르면 입력을 거부한다. iOS도 bounds/drawable 크기와 x/y scale을 확인하고 비균일이면 거부한다. 실행된 두 simulator는 고정 비율 경로다. 실패 입력 자체는 정적 검사다.
10. **native touch slop 미만 움직임 — 실행 확인.** Android AVD의 5 physical-pixel control은 관찰된 21px native slop보다 작아 PanBegan·offset 변경을 만들지 않았다. iOS는 UIKit native slop 전에는 recognizer callback이 시작되지 않는 control을 실행했으며 UIKit 임계값은 숫자로 주장하지 않는다. 근거: `android-api37-avd-below-slop-final.log`, `ios-iphone17pro-below-native-slop.png`.
11. **두 번째 pointer가 기존 pan을 계속 움직이는 경로 — 제한 확인.** Android `ACTION_POINTER_DOWN`은 active gesture를 Cancel하고 모든 pointer가 풀릴 때까지 새 pan을 막는다. 실제 ADB multi-touch 결과가 아니라 synthetic `MotionEvent` probe로만 확인했다. iOS는 active touch count가 2 이상일 때 같은 suppression 경로로 들어가지만 실제 multi-touch는 실행하지 않았다. 근거: `android-api37-avd-second-pointer.log`.
12. **active Android pointer index 소실 — 정적.** `findPointerIndex`가 음수면 Cancel을 enqueue하고 남은 pointer 입력을 억제한다. 이 경로는 일반 simulator swipe 로그에서 재현하지 않았다.
13. **플랫폼이 touch stream을 취소 — 실행·정적 혼합.** Android `ACTION_CANCEL`과 iOS recognizer `.cancelled/.failed`는 terminal Cancel을 보낸다. iOS HOME 전환 로그에는 recognizer cancellation 뒤 새 gesture가 성공한 기록이 있다. 강제 OS interruption 자체를 매번 재현했다고 주장하지 않는다.
14. **Move queue 포화 — 정적.** queue는 regular Move가 포화되면 예약 terminal 슬롯으로 Cancel을 넣고 native adapter는 `GestureCancelledForCapacity` 결과에서 terminal 상태와 pointer suppression을 설정한다. 별도의 강제 포화 ADB burst는 실행하지 않았다.
15. **64 regular + 8 reserved terminal 전체가 한 번의 drain보다 큰 경우 — 결함 수정, 실행 범위 제한.** 양쪽 bridge drain 상한을 queue 최대 `64 + 8`과 같게 했다. 구현 전 64 상한이면 wake가 합쳐진 상황에 terminal이 남을 수 있었다. 72개를 동시에 채우는 재현은 하지 않았으며, 현재 nested scroll·worker-delay 로그는 정상 동작 근거이지 72개 burst 증거가 아니다. 근거: `spinon_jni.cc`, `SpinonRunner.mm`, `android-api37-avd-worker-delay.log`.
16. **worker가 느려 UI callback이 막히는 경우 — 실행 확인.** 1초 worker 지연 중 ADB callback은 `waits_for_worker=false`로 반환했고 worker가 나중에 queue를 처리했다. 근거: `android-api37-avd-worker-delay.log` 및 iOS callback summary.
17. **coalesced wake에서 새 입력을 놓치는 경우 — 코드 경계 확인.** Android atomic `drainAgain`과 iOS locked `drainAgain`을 이용해 이미 consumer가 예약돼 있을 때 도착한 wake를 다시 drain한다. 빠른 반복 swipe 로그는 정상 경로를 확인하지만 동시 producer race 주입은 하지 않았다.
18. **stale scroll snapshot 게시 — 정적 방어 확인.** offset revision이 바뀐 결과만 invalidate/publish하고, stale sequence 또는 publish 거부는 오류로 반환해 앱 로그/status에 드러낸다. stale publication injection은 미실행이다. 근거: `publish_c13_3_delta`와 각 bridge error path.
19. **pause·surface 변경·destroy 중 gesture 잔류 — 실행·정적 혼합.** Android activity pause에서 Cancel 뒤 같은 host의 fresh swipe를 확인했다. surface resize/destroy와 iOS background/view disappear는 cancel 및 generation 변화 코드를 갖지만 모든 lifecycle 조합을 simulator에서 실행한 것은 아니다. 근거: `android-api37-avd-lifecycle-cancel.log`, `android-api37-avd-after-cancel-fresh-swipe.log`.
20. **기본 앱으로 fixture 입력을 호출하거나 host shutdown과 drain이 경합 — 확인.** 기본 artifact에서는 C13.3 FFI 심볼이 없고 disabled route만 남는다. opt-in shutdown은 Android queue 종료/host free 순서와 iOS serial runtime/render queue에서 drain 후 host free 순서를 사용한다. 종료 race 자체는 강제 주입하지 않았다. 근거: `android-default-artifact-check.txt`, `ios-default-artifact-check.txt`, 각 platform `dispose/beginShutdown`.

## 결과와 한계

검토 중 발견한 drain 상한 64/queue 최대 72의 불일치는 Android JNI와 iOS Objective-C++ bridge에서 모두 수정했다. 코드상 72개 전체를 한 drain에서 비울 수 있으며, 해당 burst의 직접 재현은 없으므로 이 경로의 실행 증거로 표현하지 않는다. 현재 C13.3 AVD swipe·slop·pause 로그는 성공한 입력/negative-control 실행이며 adapter 실행 자체의 실패 로그는 없다. 앞서 보고된 별도 R05 fake-ADB 회귀 실패는 Android 기기 연결 오류가 아니라 `/usr/bin/env node` shim 실행 중 `ps` command identity가 바뀌어 안전 종료가 거부된 race였고, `process.execPath` 직접 실행으로 `origin/main`에서 수정됐다(`315f7d69`). 이 과거 harness 수정은 현재 adapter 구현의 직접 실패 재현으로 세지 않는다.

Android AVD와 iOS Simulator의 실제 single-pointer touch 및 화면 경계 결과는 [실행 근거](c13-3-platform-input-adapters-2026-10-15/README.md)에 있다. `contain/none` style-policy 연결, iOS multi-touch, 실제 Android multi-touch, wheel/trackpad, tap activation, S04.10 displayed-frame input 및 S05.2 public events는 이 구현 검토 범위를 넘어 미완료다.

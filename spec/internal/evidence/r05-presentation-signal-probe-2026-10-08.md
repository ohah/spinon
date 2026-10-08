# R05.3 입력·제출 신호 시뮬레이터 probe

## 실행 환경

- iOS: Xcode 26.2 (17C52), iOS 26.2 Simulator, iPhone 17 Pro, arm64.
- Android: API 36 arm64 Emulator `emulator-5554`, 1080×2400, density 420.
- V8 고정 revision: `7b50b62cb18f28617959e8452e2cd18195b38bcf`.
- 빌드: 고정 V8 checkout을 `SPINON_V8_DIR`로 지정해 `mise exec -- bun run build:ios-sim`과 `mise exec -- bun run build:android`를 실행했고 둘 다 성공했다. iOS build는 최종 Swift gesture/surface 변경 뒤 다시 성공했다.
- 입력은 iOS `idb ui tap`/`idb ui swipe`와 Android `adb shell input tap`으로 주입했다. 모두 synthetic이며 앱 내부에서는 출처를 판별할 수 없어 `input_source=unknown`으로 남긴다. 실기기 입력은 하지 않았다.

## 관측 결과

| 경로 | 실행 | 관측 | 해석 |
|---|---|---|---|
| Android wgpu / Vulkan | 바깥 좌표 1회, 도형 중앙 1회 | 바깥 입력은 `outside_target`으로 제외. 중앙 입력 `seq=1 → revision=1`, surface generation 1, `draw_accepted=true` | 입력 종료와 draw 호출 반환까지 연결됐다. `present_signal=unavailable`이므로 실제 표시 시각은 없다. |
| iOS wgpu / Metal | 바깥 좌표 1회, 도형 중앙 1회 | 바깥 입력 제외. 중앙 입력 `seq=1 → revision=1`, generation 1, Rust/wgpu draw 결과 0 | wgpu 호출 반환만 확인했다. drawable ID/presentation callback은 얻지 못했다. |
| iOS native Metal 대조 | 바깥 좌표 1회, 도형 중앙 1회 | 중앙 입력 `seq=1 → revision=1`; command buffer completion callback 수신 | 완료 callback은 GPU command buffer 상태다. 표시 완료 신호로 취급하지 않았다. `presentation_signal=unavailable`. |
| R05 비활성 대조 | Android wgpu, iOS wgpu, iOS native Metal 기존 R08 경로에서 중앙 탭 | 세 경로 모두 기존 탭 동작, R05 비활성 로그에서 R05 marker 0개 | probe flag가 꺼진 기존 입력·그리기 경로를 확인했다. iOS 로그는 [wgpu](r05-presentation-signal-probe-2026-10-08/ios-baseline-followup.log)와 [native Metal](r05-presentation-signal-probe-2026-10-08/ios-metal-baseline.log)에 있다. |

iOS gesture follow-up에서 wgpu와 native Metal 대조 모두 `idb ui swipe 20 420 196 420 --duration 0.5 --delta 5`를 실행했다. 도형 바깥에서 시작해 안에서 끝난 swipe는 `outside_target`으로 제외했다. `idb ui swipe 100 420 196 420 --duration 0.5 --delta 5`는 도형 안에서 시작·종료했지만 8pt probe movement 기준을 넘어 `gesture_moved`로 제외했다. 이후 `idb ui tap 196 420`은 두 경로에서 `seq=1 → revision=1`로 연결됐다. [추가 실행 로그](r05-presentation-signal-probe-2026-10-08/ios-gesture-followup.log). 비활성 대조는 최종 변경 빌드에서도 다시 실행했다. [iOS 비활성 경로 로그](r05-presentation-signal-probe-2026-10-08/ios-baseline-followup.log).

iOS wgpu의 R13 window detach/reattach 대조에서 surface generation `1 → 2`를 기록했고, 재부착 뒤 입력과 제출 모두 generation 2였다. 이는 재부착의 로그 귀속 smoke test다. detach 도중 활성 입력을 강제 주입하지는 않았다.

Android API 36 표본의 `MotionEvent.getEventTimeNanos()` 값은 나노초 API 표현을 사용했지만 관측값은 1ms 경계에 정렬됐다. 나노초 표현은 나노초 정확도를 보장하지 않으므로 시간 지연 결과로 일반화하지 않는다. 입력·handler에는 uptime 계열을 사용했다. API 35 이상은 `SystemClock.uptimeNanos()`, 그 아래는 밀리초 fallback을 별도 표시한다.

iOS 표본의 `UITouch.timestamp`, `ProcessInfo.systemUptime`, `CACurrentMediaTime()` anchor는 이 시뮬레이터 로그의 1µs 소수 자릿수에서 같은 epoch 값으로 관측됐다. 표본이 제한적이므로 clock 변환 오차 상한을 확정하지 않았다.

이 실행은 소수 synthetic 입력으로 한 연결 smoke test다. p50/p95, 입력→표시 지연, 프레임 순위, 실기기 동작 또는 release 성능을 주장하지 않는다. Android 한 입력에서는 submit 호출 구간이 약 59.2ms로 관측됐다. 단일 시뮬레이터 표본이고 Trace 계측과 `drawCurrentFrame` 호출 경계를 포함하므로 원인이나 화면 표시 시각으로 단정하지 않는다. 다음 Android 조사 항목으로 남긴다.

## 표시 신호 capability 판정

### Android

현재 R08 wgpu 출력 표면은 `SurfaceView`다. Perfetto FrameTimeline 문서는 `SurfaceView`를 지원 대상으로 설명하지 않으므로 앱은 capability 실패를 명시하고 값을 대체하지 않는다. [Perfetto FrameTimeline 지원 범위](https://perfetto.dev/docs/data-sources/frametimeline). API 36 Emulator 로그에는 `Perfetto_FrameTimeline_on_SurfaceView available=false`가 기록됐다. 이 probe에서는 `JankData`나 SurfaceFlinger의 다른 timestamp를 actual-present 값으로 대입하지 않았다.

### iOS

Apple 문서는 `MTLDrawable.presentedTime`과 `addPresentedHandler`를 표시 시각/callback API로 설명한다. [MTLDrawable.presentedTime](https://developer.apple.com/documentation/metal/mtldrawable/presentedtime) · [MTLDrawable.addPresentedHandler](https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler%28_%3A%29). 설치된 Xcode 26.2 Simulator SDK의 `CAMetalDrawable` Swift 타입에는 `drawableID`, `addPresentedHandler`, `presentedTime` 멤버가 노출되지 않는다. 현재 SDK를 대상으로 한 [최소 API probe](r05-presentation-signal-probe-2026-10-08/metal-drawable-api-probe.swift)와 [타입 검사 결과](r05-presentation-signal-probe-2026-10-08/metal-drawable-api-probe.log)에서 이 한계를 재현했다. 비공개 API나 selector로 우회하지 않았다.

대조 경로는 `MTLCommandBuffer.addCompletedHandler`에서 completion status와 callback uptime만 기록한다. 제출 로그와 완료 로그는 실제 화면 표시 완료가 아니다. wgpu 쪽도 drawable identity를 앱 계층에 공개하지 않아 같은 제한을 가진다.

## 시뮬레이터 화면과 원본

| 화면 | 캡처 |
|---|---|
| Android R05 wgpu | ![Android R05 wgpu 시뮬레이터 화면](r05-presentation-signal-probe-2026-10-08/android-wgpu.png) |
| iOS R05 wgpu | ![iOS R05 wgpu 시뮬레이터 화면](r05-presentation-signal-probe-2026-10-08/ios-wgpu.png) |
| iOS native Metal 대조 | ![iOS native Metal 대조 화면](r05-presentation-signal-probe-2026-10-08/ios-metal-control.png) |
| Android R05 비활성 대조 | ![Android R05 비활성 대조 화면](r05-presentation-signal-probe-2026-10-08/android-baseline.png) |
| iOS R05 비활성 대조 | ![iOS R05 비활성 대조 화면](r05-presentation-signal-probe-2026-10-08/ios-baseline.png) |
| iOS native Metal R05 비활성 대조 | ![iOS native Metal R05 비활성 대조 화면](r05-presentation-signal-probe-2026-10-08/ios-metal-baseline.png) |

원본 OS 로그와 캡처는 [실행 자료 디렉터리](r05-presentation-signal-probe-2026-10-08/)에 두었다. SHA-256은 [체크섬 파일](r05-presentation-signal-probe-2026-10-08/SHA256SUMS)에 기록했다.

## 구현 적대 검토

아래 관점은 서로 다른 표본 오염·플랫폼 경계·기능 비활성 경로를 대상으로 코드를 다시 읽고, 가능한 항목은 실제 시뮬레이터 출력과 대조했다. “정적 검토”로 표시한 포인터 취소·다중 터치·접근성 경로는 시뮬레이터에서 직접 주입하지 않았다.

| # | 공격 관점 | 확인과 결과 |
|---:|---|---|
| 1 | probe flag가 꺼져도 새 로그나 동작 변경이 새어 나오는가 | Android·iOS R05 비활성 실행에서 R05 표식이 0개이고 기존 색상 변경은 동작했다. 통과. |
| 2 | iOS native Metal 대조를 wgpu 결과로 오인하는가 | renderer 이름이 로그·화면에 분리되고 대조값을 wgpu 제품 값으로 대체하지 않는다. 통과. |
| 3 | Android ACTION_DOWN을 입력 완료 시각으로 사용하는가 | 표본은 단일 gesture의 ACTION_UP에서만 생성한다. API 36 로그 phase와 일치. |
| 4 | 두 번째 포인터를 올린 뒤 마지막 단일 ACTION_UP을 단일 터치로 오인하는가 | ACTION_POINTER_DOWN/UP에서 gesture 전체 multi-pointer 표식을 유지한다. 코드 검토 통과; multi-touch는 미주입. |
| 5 | ACTION_CANCEL을 성공 탭으로 계산하는가 | pending 표본을 지우고 취소 사유만 남긴다. 코드 검토 통과; 취소 입력은 미주입. |
| 6 | 보이는 도형 밖을 눌러 입력 시퀀스가 생기는가 | Android·iOS 바깥 좌표에서 outside_target 제외 로그를 관측했다. 통과. |
| 7 | 바깥에서 시작해 안에서 끝나거나, 도형 안에서 크게 이동한 swipe를 탭으로 오인하는가 | Android는 touch slop 초과·target 이탈 시 gesture를 영구 제외한다. iOS는 시작점·이동·끝점을 추적하고 target 이탈 또는 8pt 초과 이동 뒤에는 계속 제외한다. 두 iOS renderer에서 synthetic swipe 두 유형을 실행해 `outside_target`/`gesture_moved`를 관측했고, 뒤이은 정상 탭만 seq=1로 연결했다. |
| 8 | API 34 미만에서 없는 나노초 getter를 호출하는가 | API 34 gate 뒤 나노초 event getter를 쓰고 그 아래는 ms fallback을 기록한다. 코드 검토 통과. |
| 9 | Android event와 handler에 서로 다른 monotonic epoch를 섞는가 | event/handler 모두 uptime 계열이며 handler 정밀도 fallback을 별도 출력한다. API 36 실행값 관측. 통과. |
| 10 | 이전 surface 입력을 재부착 뒤 새 generation과 합치는가 | Android는 surface destroy 때 pending을 제외한다. iOS wgpu detach/reattach에서 generation 1→2를 관측했고 재부착 뒤 input/submit은 모두 2였다. detach 중 활성 gesture 강제 주입은 하지 않았다. |
| 11 | revision이 빠졌는데 submit 성공으로 집계하는가 | Android 로그에서 input_seq=1, revision=1, draw_accepted=true가 연결됐다. 이는 API 호출 성공이지 표시 성공은 아니다. |
| 12 | 접근성 activation을 물리 터치로 기록하는가 | 입력 없는 activation은 input_seq=0/attribution=unmatched이며 iOS도 touch 없는 revision을 미연결로 기록한다. 코드 검토; VoiceOver/TalkBack 미주입. |
| 13 | iOS touchesEnded 한 개만 보고 동시 touch를 놓치는가 | probe는 began/moved/ended/cancelled 전체 gesture와 `event.allTouches`를 검사한다. R05 비활성 경로는 기존 끝점 판정을 유지한다. 코드 검토; 다중 touch 미주입. |
| 14 | 자동 주입을 physical 입력으로 오인하는가 | 앱 로그는 input_source=unknown; 실행 증거는 idb/ADB 입력으로 synthetic 분류했다. 통과. |
| 15 | iOS 입력과 handler clock epoch를 확인 없이 빼는가 | uptime/host anchor를 원시 로그로 남겼고 변환 오차 상한은 주장하지 않았다. 제한된 simulator 표본 통과. |
| 16 | iOS 접근성 callback revision을 직전 touch에 잘못 붙이는가 | touch == nil이면 입력 제외와 unmatched revision을 따로 기록한다. 코드 검토; 보조기기 미주입. |
| 17 | 직전 iOS 표본이 그려지지 않았는데 다음 revision과 연결되는가 | 중첩, renderer 부재, drawable/pipeline 부재에서 pending을 제외하고 지운다. 코드 검토; fault injection 미실행. |
| 18 | SDK에 없는 iOS 표시 callback을 있다고 가정하는가 | Xcode 26.2 SDK 타입 검사에서 drawableID/addPresentedHandler 부재를 재현하고 unavailable로 fail-closed 처리했다. 통과. |
| 19 | GPU command completion 또는 present() 반환을 화면 표시 시각이라 부르는가 | 로그를 GPU_COMPLETE/SUBMIT으로 분리하고 두 경로 모두 presentation_signal=unavailable임을 확인했다. 통과. |
| 20 | Android API 36 SurfaceView에 FrameTimeline이 나온다고 가정하는가 | 실제 surface와 Perfetto 지원 계약을 대조해 unavailable로 남겼고 actual-present 지연을 계산하지 않았다. 통과. |

이 실행으로 확인한 것은 입력→revision→제출 호출 연결과 capability 실패의 안전한 표기다. **R05.3 및 R05 전체는 미완료**다. 다음은 공개 SDK/API에서 exact surface/frame 표시 시각을 얻을 지원 경로 또는 별도 광학 계측을 탐색하고 Android의 단일 59.2ms submit-call 관측을 통제 조건으로 재현하는 일이다. 실기기 실행은 사용자 요청 전까지 보류한다.

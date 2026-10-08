# R05.3 FrameTimeline ID 연결 probe 실행 근거

**실행일:** 2026-10-08 · **결과:** Android API 37.2 시뮬레이터의 제한된 on-demand fixture에서 frame-token/revision 연결 확인 · **R05.3/R05:** 미완료

## 실행 조건

- Android 17.2 / API 37.2 ARM64 AVD `emulator-5562`, 1080×1920, 420 dpi, 화면 주사율 60 Hz.
- WGPU/Vulkan은 `Goldfish GFXStream (llvmpipe)` 경로, GLES API 대조는 `ANGLE / Vulkan SwiftShader` 경로다. 렌더러·가상 GPU stack이 다르므로 성능 비교에 쓰지 않는다.
- `SurfaceView`는 이벤트 때만 draw를 제출하는 on-demand fixture였다. WGPU/GLES 각각 3개 block을 새 앱 프로세스로 실행했다. block마다 중앙 synthetic 탭 10개를 점수화하고, 마지막 비동기 JankData 배출용 11번째 탭은 unscored drain control로 분리했다.
- 빌드: 저장소 고정 mise 환경, `:app:assembleDebug`, Android compile SDK 37.2. Android Gradle Plugin 8.13.2가 compile SDK 37.2에서 검증되지 않았다는 경고가 있었지만 빌드는 통과했다.

## 판정 방법

각 점수 입력의 `input_seq`·`revision`에 대해 `Choreographer.FrameData.getPreferredFrameTimeline()`의 VSync ID를 고르고, `SurfaceControl.Transaction.setFrameTimeline(id)`을 적용한 transaction을 `SurfaceView.applyTransactionToFrame()`에 넘긴 뒤 한 번 draw했다. JankData에서 받은 `getVsyncId()`를 target과 정수 동일성으로 대조했다. 시간 근접성·배열 순서·callback 도착 순서는 join 기준으로 사용하지 않았다.

| Renderer | Block | 입력 시도 | 점수 입력 exact unique | 점수 표본 duplicate | exact 표본 present time unknown | unscored drain |
|---|---:|---:|---:|---:|---:|---:|
| WGPU/Vulkan | 1 | 11 | 10/10 | 0 | 10/10 | 점수 제외 |
| WGPU/Vulkan | 2 | 11 | 10/10 | 0 | 10/10 | 점수 제외 |
| WGPU/Vulkan | 3 | 11 | 10/10 | 0 | 10/10 | 점수 제외 |
| GLES 2.0 API | 1 | 11 | 10/10 | 0 | 10/10 | 점수 제외 |
| GLES 2.0 API | 2 | 11 | 10/10 | 0 | 10/10 | 점수 제외 |
| GLES 2.0 API | 3 | 11 | 10/10 | 0 | 10/10 | 점수 제외 |

결과는 renderer별 점수 표본 30/30 exact unique match다. 두 renderer의 exact 표본 60/60에서 `present_state=unknown` (`presentTimeNanos=-1`)이었다. 이는 해당 fixture에서 VSync token으로 buffer frame과 revision을 연결할 수 있음을 보일 뿐, actual-present 지연을 계산할 시각은 제공하지 않았다. API 36 fallback에서는 direct SurfaceView 표시 신호가 unavailable일 때 frame-token join을 건너뛰고 기본 GPU draw가 계속됐다. 최종 APK에서도 API 36 WGPU 입력 1건을 다시 보내 capability gate가 닫힌 채 draw가 수락되는 것과 fatal exception 부재를 확인했다. [기존 API 36 fallback](r05-presentation-signal-probe-2026-10-08/android-api36-frame-timeline-fallback.log) · [최종 APK API 36 재확인](r05-presentation-signal-probe-2026-10-08/android-api36-after-api372-final-build.log).

초기 단일 WGPU smoke에서는 첫 callback batch가 직전 surface frame만 반환했다. 다음 입력 뒤 JankData record가 늦게 도착하는 것을 확인하고 각 draw 후 250 ms delayed flush를 추가했다. 3×10 점수 표본 뒤 한 번 더 보낸 unscored drain tap으로 마지막 점수 frame을 배출했다. 실제 본 block별 점수 결과는 아래 원본 log에 있다.

- [WGPU block 1 log](r05-presentation-signal-probe-2026-10-08/android-api37_2-frame-timeline-wgpu-block-1.log) · [화면 캡처](r05-presentation-signal-probe-2026-10-08/android-api37_2-frame-timeline-wgpu-block-1.png)
- [WGPU block 2 log](r05-presentation-signal-probe-2026-10-08/android-api37_2-frame-timeline-wgpu-block-2.log)
- [WGPU block 3 log](r05-presentation-signal-probe-2026-10-08/android-api37_2-frame-timeline-wgpu-block-3.log)
- [GLES block 1 log](r05-presentation-signal-probe-2026-10-08/android-api37_2-frame-timeline-gles-block-1.log)
- [GLES block 2 log](r05-presentation-signal-probe-2026-10-08/android-api37_2-frame-timeline-gles-block-2.log)
- [GLES block 3 log](r05-presentation-signal-probe-2026-10-08/android-api37_2-frame-timeline-gles-block-3.log) · [화면 캡처](r05-presentation-signal-probe-2026-10-08/android-api37_2-frame-timeline-gles-block-3.png)

## 플랫폼 경계

- Android API 36에서는 `SurfaceView.JankData` callback 신호가 없으므로 join을 시도하지 않았다. fallback 원본은 입력·draw가 살아 있고 `present_signal_api_unavailable`로 빠지는 것을 기록한다.
- Apple 현재 공식 문서는 `MTLDrawable.drawableID`, `addPresentedHandler`, `presentedTime`을 설명한다. Xcode 26.2 iPhoneOS SDK device target에서는 세 심볼의 type-check와 격리 앱 compile/link가 통과했지만, 같은 Xcode의 iOS Simulator SDK에는 세 심볼이 없다. 후속 Simulator XCTest는 WGPU `nextDrawable()` acquire 및 입력 ticket 귀속을 확인했으나 device callback runtime은 실행하지 않았다. 이 Android 실험 자체는 iOS runtime 결과를 만들지 않는다. [SDK matrix](r05-presentation-signal-probe-2026-10-08/ios-sdk-matrix-typecheck.log) · [iOS 후속 근거](r05-ios-present-feedback-2026-10-08.md).
- invalid/stale VSync ID 직접 주입, surface generation 변경 중 callback 취소, callback 등록 실패, transaction 예외, 연속 렌더링 경합은 실행하지 않았다. 실기기 입력과 광학 측정도 하지 않았다.
- 실제 present timestamp가 unknown이므로 event-to-present latency, p50/p95, WGPU/GLES 우위는 산출하지 않았다.

## 구현 적대 검토 · 20개 독립 실패 관점

이 표는 구현 뒤 코드와 실제 API 37.2 block log, API 36 fallback, 설치 SDK를 서로 다른 실패 관점으로 대조한다. 실행하지 않은 고장 주입은 코드 검토 통과와 runtime 통과를 구분해 적었다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | preferred/expected presentation을 actual time으로 오인 | 코드에서 expected·deadline을 후보 로그로만 기록했다. actual time은 JankData의 `presentTimeNanos`만 보며 60개 모두 unknown이었다. latency 계산은 하지 않았다. |
| 2 | timestamp 근접도로 입력과 frame을 연결 | target map은 VSync ID exact key만 쓴다. block log 60개 점수 record가 모두 정확한 target ID와 일치했다. |
| 3 | target VSync ID와 callback frame ID가 서로 다른 경우를 숨김 | parser에서 `target_vsync_id == JankData.vsync_id` 및 해당 입력의 target table을 독립 확인했다. WGPU 30/30, GLES 30/30이다. |
| 4 | callback 배열 순서로 입력 순서를 추정 | callback batch에 두 record가 같이 온 구간도 `vsyncId`로 각각 매핑됐다. 두 block log에서 이를 확인했다. |
| 5 | 같은 ID record 중복을 표본 하나로 축약 | 각 점수 입력은 block 내 한 번만 exact match했고 duplicate 0건이다. Registry는 중복 target 등록을 거부한다. |
| 6 | JankData의 직전 batch 또는 초기 surface frame을 점수 입력으로 오분류 | 초기 unmatched `-1`/직전 frame record와 input 11 drain control은 점수에서 제외됐다. 독립 파서가 input 1–10만 세 renderer별 30/30으로 집계했고, drain ID는 별도 분류했다. |
| 7 | 최종 입력이 flush 전에 남아 손실 | 첫 smoke에서 callback 지연을 재현하고 250 ms delayed flush와 unscored drain control을 추가했다. 각 block의 처음 10개 표본은 모두 배출됐다. |
| 8 | drain control을 입력 latency 표본에 포함 | 11번째 탭은 각 block의 unscored drain으로 표에서 제외했다. latency 값 자체는 전혀 산출하지 않았다. |
| 9 | 렌더 중 SurfaceView frame transaction이 어느 buffer에 붙을지 불명확 | 이 실행은 on-demand 경로만 썼다. API 문서가 지속 렌더링에서 대상 frame을 정의하지 않는다는 제한을 기록했고, 연속 렌더링 경로는 검증했다고 주장하지 않는다. |
| 10 | revision 변경과 draw 요청이 1:1이 아님 | 점수 입력별 revision과 target request ID를 기록했고, 각 block의 scored revision이 exact unique record 하나와 연결됐다. |
| 11 | WGPU 대 GLES 결과를 성능 순위로 확대 | AVD renderer stack이 llvmpipe와 ANGLE/SwiftShader로 다름을 기록했고 capability 비교만 허용했다. |
| 12 | API 37.0/37.1이나 API 36에서 37.2 listener가 있다고 가정 | API 36 실행은 `present_signal_api_unavailable`에서 빠지고 draw 성공을 확인했다. API 37.0/37.1 runtime은 미실행이다. |
| 13 | API 35+ FrameTimeline API가 API 29~34에서 class verification crash | 새 API 참조는 `Api35` 내부 class로 격리했다. API 36 debug APK에서 flag를 켠 fallback 실행이 crash 없이 끝났다. API 29~34는 미실행이다. |
| 14 | surface destroy/host pause 이후 pending callback이 stale generation을 그리거나 누수 | 코드상 generation별 Choreographer callback 취소와 pause 입력 제외를 추가했다. rotation 중 pending callback 직접 주입은 실행하지 않았다. |
| 15 | transaction 전달 전후 소유권 오류·예외가 표본 성공 처리 | 전달 실패 때 target mapping을 제거하고, ownership handoff 전 transaction만 close한다. transaction 예외 주입은 실행하지 않았다. |
| 16 | VSync target registry 또는 pending callback registry가 무제한 성장 | 두 registry에 capacity 64 fail-closed 경계를 두었다. 이 시험은 block당 11건이라 saturation 시나리오를 실행하지 않았다. |
| 17 | callback 등록/flush가 실패해도 exact match로 집계 | 일반 block의 queue reject counter는 0이었고 API36은 capability unavailable로 분리됐다. callback API 실패 injection은 미실행이다. |
| 18 | surface renderer/generation이 바뀐 record를 current 표본으로 포함 | 매치 함수는 renderer, generation, current surface를 확인한다. renderer/generation 오염 injection은 실행하지 않았다. |
| 19 | 시뮬레이터 입력을 물리 touch로 또는 광자 시각으로 보고 | 로그의 input source는 synthetic으로 남겼다. 실기기 입력과 광학 측정은 하지 않았다. |
| 20 | Android 결과로 iOS 지원을 대신하거나 R05 완료로 체크 | 이 Android 실행은 iOS를 대신하지 않는다. 후속 iOS Simulator acquire test와 iPhoneOS target compile/link는 별도 근거에 기록했으며 device callback runtime은 아직 미실행이다. unknown timestamp와 failure injection 한계 때문에 R05.3/R05는 계속 미완료다. |

## 다음 gate

Android frame identity 연결은 이 AVD의 on-demand 경로에서 재현됐다. 남은 Android 작업은 invalid/stale token과 surface lifecycle failure path를 주입하고 `presentTimeNanos` unknown 원인을 SurfaceFlinger/가상 display capability와 분리하는 것이다. iOS 후속에서는 공개 iPhoneOS API target compile/link와 Simulator의 실제 WGPU acquire/ticket join을 확인했다. 다음 iOS gate는 bounded callback ledger·timeout/lost 처리, clock residual, 기기 callback runtime 및 전체 V8 앱 통합이다. 어느 단계에서도 `CADisplayLink`, command-buffer completion, expected presentation을 actual present 대체값으로 쓰지 않는다. 실기기 검증은 사용자 요청 뒤에 한다.

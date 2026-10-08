# R05.3 · 입력 이벤트에서 GPU 프레임 표시 신호까지 계측 계획

**상태:** Android API 36·37.2 transaction present-fence와 FrameTimeline ID/revision 대조를 AVD에서 확인 · iOS runtime·실기기·광학 scanout 미완료 · R05 미완료

**상위 항목:** [R05 계측 상태](../spec/STATUS.md) · [R05 계측 계약](../spec/internal/0022-r05-benchmark-attribution.md) · [비교 측정 계획](../docs/plans/benchmark.md)

## 목적과 주장 경계

현재 iOS R05 probe는 버튼 selector와 `CADisplayLink`를 사용하고 Android R05 대조 화면은 View `onClick`을 사용한다. 두 경로 모두 물리 입력 시각과 GPU 표면의 특정 프레임이 표시된 시각을 직접 연결하지 않는다. 따라서 현재까지의 R05 결과는 입력→픽셀 지연 또는 입력→화면 표시 완료로 해석하지 않는다.

R05.3의 목표는 아래 세 사건을 하나의 입력 sequence 및 렌더 revision에 연결하는 계측 경로를 확정하고, 우선 시뮬레이터에서 그 연결이 실제로 관측되는지 검증하는 것이다.

1. OS가 전달한 입력 event timestamp와 앱 입력 handler 진입.
2. 입력으로 바뀐 고유 화면 revision의 GPU frame 제출.
3. 그 frame에 대해 Android compositor 또는 iOS Metal이 보고한 표시 시각.

핵심 지표 이름은 **OS 입력 이벤트 → OS 표시 신호**다. Android `MotionEvent.getEventTimeNanos()`는 uptime 기준의 나노초 표현이며 API 문서도 정밀도와 정확도를 구분한다. Android `Choreographer.FrameTimeline`은 예상 표시 시각과 VSync ID를 제공하고, 실제 frame과의 연계는 Perfetto FrameTimeline 또는 검증된 frame ID 경로로 확인해야 한다. Android Perfetto의 `actual_frame_timeline_slice`는 compositor가 관측한 actual frame을 제공하지만 픽셀 발광의 광학 측정은 아니다. [MotionEvent](https://developer.android.com/reference/android/view/MotionEvent#getEventTimeNanos()) · [Choreographer.FrameTimeline](https://developer.android.com/reference/android/view/Choreographer.FrameTimeline) · [Perfetto FrameTimeline](https://perfetto.dev/docs/data-sources/frametimeline)

iOS는 `UITouch.timestamp`와 `MTLDrawable.presentedTime`을 후보로 삼는다. Apple은 후자를 drawable이 화면에 표시된 host time으로 정의하며, 미표시 또는 drop된 frame의 값은 `0.0`일 수 있다고 명시한다. 실제 출력 경로에서 drawable ID·렌더 revision을 `addPresentedHandler` 결과까지 보존할 수 있어야 한다. Xcode 26.2 iPhoneOS SDK는 관련 세 멤버를 제공하지만 같은 버전의 iOS Simulator SDK에는 선언이 없다. 따라서 Simulator는 drawable acquire 경로만 검증하고 display feedback을 흉내 내지 않는다. device-only callback은 별도 계획 [iOS Metal 표시 feedback](r05-ios-present-feedback.md)에 따른다. `CADisplayLink.timestamp/targetTimestamp`는 frame callback 시간표시로만 기록하며 drawable 표시 완료를 대체하지 않는다. 이 신호도 광학적 scanout 측정은 아니다. [UITouch.timestamp](https://developer.apple.com/documentation/uikit/uitouch/timestamp) · [MTLDrawable.presentedTime](https://developer.apple.com/documentation/metal/mtldrawable/presentedtime) · [addPresentedHandler](https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler%28_%3A%29) · [CADisplayLink.timestamp](https://developer.apple.com/documentation/quartzcore/cadisplaylink/timestamp)

**finger-to-photon** 또는 실제 픽셀 발광 시각은 이 작업의 지표가 아니다. 그런 주장은 동기화된 외부 광학 센서와 그 교정 자료가 있을 때 별도 측정으로 추가한다. 계측 API의 시간 단위가 ns라고 해서 정확도·광학 오차가 ns라는 뜻은 아니다.

## 사전 비교 모델과 단위

각 표본은 `input_sequence`, `render_revision`, `surface_generation`, 플랫폼 frame 식별자, 입력 출처, 원시 timestamp와 clock domain을 함께 가진다. 기준 탭 동작은 Android 단일 포인터 `ACTION_UP`과 iOS 단일 touch `UITouch.Phase.ended`로 고정한다. 이는 OS가 전달한 종료 event 기준이며 손가락 접촉 시작이나 digitizer 원시 시각이 아니다. `ACTION_DOWN`/move, 취소, 다중 touch/pointer, 접근성 activation은 기준 표본에서 제외하고 별도 실험으로 분리한다. 입력마다 고유 sequence가 GPU 표면에 실제 색상/도형 변화로 보여야 한다. 텍스트 상태 label만 바뀌고 GPU 표면은 그대로인 경우는 GPU 표시 표본으로 인정하지 않는다.

플랫폼별로 별도 계산한다.

- `event_to_handler`: OS 입력 event timestamp → 앱 handler timestamp. handler 시각도 해당 플랫폼의 단조 clock으로 기록하고 원시 event 시각과 같은 clock으로 직접 비교할 수 있는지 확인한다.
- `handler_to_submit`: handler 진입 → 해당 revision을 포함한 GPU frame 제출.
- `event_to_os_present`: 입력 event timestamp → 같은 revision의 OS actual-present timestamp. exact frame correlation을 입증했을 때만 산출한다.
- 선택적인 `physical_contact_to_photon`: 이 작업에서는 산출하지 않는다. 별도 광학 센서·공통 clock·교정 오차가 확보되기 전에는 `미측정`이다.

Android event timestamp와 Perfetto timestamp를 직접 빼지 않는다. 단조 시계·trace clock 변환을 실행 전후 anchor로 검증하고 offset·분해능·불확실성 범위를 보존한다. `JankData.getPresentTimeNanos()`를 쓰는 경우에도 공식 문서가 명시하지 않는 clock epoch를 같은 입력/handler clock과 anchor로 확인하기 전에는 빼지 않는다. Android API 34 미만 fallback의 millisecond event time은 별도 정밀도 등급으로 기록하며 ns 표본과 합치지 않는다. FrameTimeline의 expected timestamp, `CADisplayLink` callback, `Queue::present()` 반환은 actual-present 값으로 대입하지 않는다.

Android 경로는 capability 확인을 먼저 한다. 현재 R08 GPU renderer의 Android surface는 SurfaceView이며 Perfetto FrameTimeline 문서의 SurfaceView support 제한이 있으므로 Perfetto timeline slice를 곧바로 exact input frame으로 쓰지 않는다. 첫 조사 경로는 API 37.2 SurfaceView JankData의 presentTimeNanos·VSync ID다. JankData timestamp가 unknown이거나 해당 API 경로가 없는 경우 Android 표시 신호 전체를 unavailable로 단정하지 않고, API 35부터 제공되는 SurfaceControl.TransactionStats present fence를 같은 SurfaceView frame transaction에 붙이는 대안을 조사한다. 각 경로는 독립적으로 surface/frame exact join, timestamp 의미, clock domain을 통과해야 한다. callback 순서, callback 도착 시각, 가까운 timestamp, expected frame time만으로 frame을 추정하지 않는다. PRESENTATION_TIME_UNKNOWN·PRESENTATION_TIME_UNSET·미수신·중복 VSync ID·표면 세대 불일치는 미연결로 보존한다. Perfetto FrameTimeline의 ts/ts+dur, VSync 예상 시각, present() 반환은 actual-present 값으로 대입하지 않는다. exact surface·revision 연결과 timestamp 의미까지 입증한 source에만 event_to_os_present를 낸다. [Perfetto FrameTimeline 지원 범위](https://perfetto.dev/docs/data-sources/frametimeline) · [SurfaceView.registerOnJankDataListener](https://developer.android.com/reference/android/view/SurfaceView#registerOnJankDataListener(java.util.concurrent.Executor,android.view.SurfaceControl.OnJankDataListener)) · [SurfaceControl.JankData](https://developer.android.com/reference/android/view/SurfaceControl.JankData) · [TransactionStats present fence](https://developer.android.com/reference/android/view/SurfaceControl.TransactionStats).

iOS에서는 `UITouch.timestamp`와 `MTLDrawable.presentedTime`을 원시값으로 보존한다. `UITouch.timestamp`는 system uptime 기준이고 drawable 값은 host time이므로, `ProcessInfo.systemUptime`·`CACurrentMediaTime` anchor를 실행 전후 수집해 관계와 residual을 확인한다. 단위가 초라는 이유만으로 두 값이 같은 epoch이라고 가정하지 않는다. 공통 시간축과 최대 변환 오차를 설명하지 못하면 `event_to_os_present`를 내지 않는다. `presentedTime == 0`은 0ms 성공으로 바꾸지 않고 미표시/drop 표본으로 분류한다.

오차 보고서는 적어도 입력 timestamp의 API 분해능·정확도 한계, clock 변환 residual, frame identity match 상태, drop/누락 표본 수를 포함한다. OS가 보장하지 않는 값에 임의의 수치 오차 한계를 만들어 넣지 않는다. 잔차 상한을 측정할 수 없으면 숫자 지연과 함께 “측정 오차 미확정”을 표기하고 플랫폼 간 순위를 내지 않는다.

## 실행 단계와 합격 조건

### 1단계 · 시뮬레이터에서 연결 가능성 증명

- Android API 36 ARM64 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 실행한다. 앱·surface·OS·빌드·refresh mode·source/app digest를 기록한다.
- 자동 입력은 계측 파이프라인 smoke에만 사용하고 `synthetic`으로 표시한다. 기존 selector/ADB 입력을 physical input 데이터로 분류하지 않는다.
- 입력마다 단조 증가 sequence와 별도 render revision을 만들고, 업데이트한 frame에 exact revision·frame identity를 연결한다. 입력에 귀속하는 표시는 해당 revision을 포함한 target surface의 첫 유효 presentation signal로 고정한다. 다음 입력은 앞 입력의 결과가 표시 신호와 연결된 뒤 전달하는 isolated baseline으로 한다. 겹친 입력, 한 frame에 합쳐진 revision, 입력 없이 생긴 frame은 서로 억지로 1:1 연결하지 않고 `overlap/coalesced/unattributed`로 센다. burst는 별도 workload로 둔다.
- Android에서는 실행 전에 API/extension 수준, renderer surface 종류, JankData·SurfaceControl.TransactionStats capability를 기록한다. 선택한 source의 exact frame/revision join, 입력 event와 표시 signal의 clock 변환, trace 품질·유실·중복을 확인한다. JankData timestamp가 unknown이거나 해당 surface 경로를 지원하지 않으면 그 경로만 capability 실패로 분류하고, transaction present fence 등 별도 공개 source의 가능성을 조사한다. ts, ts + dur, 예상 시각, 앱 제출 시각을 대체값으로 사용하지 않는다.
- iOS에서는 현재 wgpu/CAMetalLayer 경로의 `drawableID`, revision, surface generation, `presentedTime` 결합을 확인한다. 2026-10-08 Simulator에서 동일 R08 root layer subclass의 `nextDrawable()`이 실제 WGPU draw 중 호출되는 것을 확인했고, XCTest tap의 `input_seq=1`·`revision=1`은 `draw_seq=4` acquire와 exact match됐다. 일반 R08 probe-off 대조에는 acquire 로그가 없었다. iPhoneOS device target callback 분기는 격리 WGPU app harness에서 compile/link했지만 실제 callback runtime은 미검증이다. 전체 V8 app bundle도 고정 V8 checkout 부재로 미검증이다. native Metal 대조값을 WGPU 제품 경로의 대체값으로 쓰지 않고, Simulator의 `CADisplayLink`나 command completion을 표시 callback으로 대체하지 않는다. [iOS 구현·실행 근거](../spec/internal/evidence/r05-ios-present-feedback-2026-10-08.md).
- 각 플랫폼은 최소 30개의 sequence를 사용해 생성·handler·revision·frame·presentation 레코드의 개수와 1:1 관계를 검사한다. block 종료 때 presentation callback/trace 수집기를 bounded drain하고, 기한 뒤 도착하거나 끝내 미도착한 레코드는 pending/lost로 남긴다. 입력 누락, 중복 ID, stale surface generation, trace drop, 잘못된 clock join은 집계 성공으로 상쇄하지 않는다.
- 계측을 추가한 뒤 trace on/off 영향을 다시 확인한다. 이전 R05 계측 오버헤드 표본은 새 marker·새 frame correlation 비용을 포함하지 않으므로 새 계측의 오버헤드 근거로 재사용하지 않는다.

### 1단계 실행 결과 · 2026-10-08

Android API 36 Emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 synthetic 바깥 탭과 중앙 탭을 실행했다. Android·iOS wgpu 경로 및 iOS native Metal 대조에서 중앙 입력 sequence와 revision이 연결되고 실제 색상 변경을 확인했다. iOS 양 renderer에서는 바깥 시작→안쪽 종료 swipe와 8pt를 넘는 내부 이동 swipe를 표본에서 제외했고, R13 window 재부착 뒤 surface generation 2 연결도 확인했다. 8pt는 probe의 gesture 오염 필터일 뿐 공개 입력 계약이 아니다. R05 비활성 대조에서도 기존 R08 입력 경로가 동작했다. [실행 로그·화면·체크섬 및 구현 적대 검토](../spec/internal/evidence/r05-presentation-signal-probe-2026-10-08.md).

API 37.2 AVD의 잘못된 `target=android-0` 메타데이터를 `android-37.2`로 고치고 테스트용 데이터 파티션을 10GB로 확장한 뒤 초기화했다. 최종 AVD boot log에는 Hypervisor.Framework 활성 및 `CPU Acceleration: working`이 기록됐다. 기존 capability 비교에서는 API 37.2 wgpu/Vulkan과 GLES 2.0 경로의 callback `present_time_ns`가 모두 unknown이었다. WGPU는 `Goldfish GFXStream (llvmpipe)`, GLES는 `ANGLE / Vulkan SwiftShader`였으므로 성능 비교가 아니다. [대조 계획과 실행 결과](r05-surface-signal-control.md).

후속 [FrameTimeline ID 연결 probe](r05-frame-timeline-join.md)에서는 preferred VSync ID를 Transaction.setFrameTimeline 및 SurfaceView.applyTransactionToFrame으로 다음 on-demand frame에 전달했다. Android 17/API 37.2 ARM64 AVD의 WGPU와 GLES 각각 세 독립 block, block당 점수 입력 10건에서 JankData.getVsyncId() exact join은 30/30, 중복·누락은 0건이었다. 각 block 끝의 11번째 synthetic drain 입력은 점수에서 제외했다. 그 실험에서 exact record 60건의 JankData presentTimeNanos는 모두 unknown(-1)이어서 JankData 값만으로는 event-to-present 지연을 계산할 수 없었다. 이것은 다른 표시 timestamp 경로가 없다는 뜻은 아니다. 후속 TransactionStats present-fence 행렬 결과는 아래에 별도 기록한다. [기존 frame-token 실행 결과](../spec/internal/evidence/r05-frame-timeline-join-2026-10-08.md).

기존 JankData listener·flush 경로는 단일 worker·용량 64 queue로 제한했다. API 37.2 포화 fixture의 callback 64개 수락, callback/flush 초과 1개씩 거부와 worker 복구, API 36 GLES fallback, 최종 APK 회전·모드 충돌 smoke를 확인했다. Xcode 26.2 iPhoneOS SDK device target은 Metal drawable 표시 API type-check를 통과했고 Simulator SDK는 같은 probe에서 실패했다. 후속 iOS Simulator harness는 실제 WGPU root layer acquire를 확인했고 XCTest 단일 tap의 input sequence 1·revision 1을 draw sequence 4에 연결했다. 기기 callback runtime과 event→present latency는 미검증이다. 후속 Android transaction present-fence 실행은 API 36·37.2의 WGPU/GLES 네 조합에서 각각 3×10 scored 입력을 반복했다. usable fence는 API36 WGPU 30/30·GLES 30/30, API37.2 WGPU 29/30·GLES 30/30이다. API37.2 JankData 60/60은 VSync ID exact join이지만 timestamp는 -1이며, WGPU pending fence 하나는 제외했다. bracket 교집합을 사용한 event→OS transaction-present 후보를 계산했으나 synthetic AVD 결과이므로 실기기 지연이나 renderer 우열이 아니다. API29–35 및 37.0/37.1, lifecycle·timeout 고장 주입과 iOS display feedback join은 남아 있다. [present-fence 계획](r05-android-present-fence.md) · [iOS Metal feedback 계획](r05-ios-present-feedback.md) · [iOS 실행 증거](../spec/internal/evidence/r05-ios-present-feedback-2026-10-08.md) · [반복 실행·구현 실패 관점 검토](../spec/internal/evidence/r05-android-present-fence-2026-10-08.md).

### 남은 실행 · iOS 표시 join·failure-path·실기기 gate

#### Android API 37.2

- 내부 probe의 `SurfaceView.registerOnJankDataListener()` 연결과 API 37.2 compileSdk 빌드는 준비됐다. API 36 경로의 `available=false`와 `SDK_INT_FULL` 기록을 에뮬레이터에서 회귀 확인했다.
- 처음 만든 API 37.2 AVD는 `.ini`와 설정의 `target=android-0` 메타데이터가 잘못되어 있었다. 이를 `android-37.2`로 수정하고 테스트 전용 AVD만 초기화했다. 800MB 데이터 파티션도 10GB로 확장한 뒤 ADB online, `sys.boot_completed=1`, `-enable-hvf` 및 `CPU Acceleration: working`을 확인했다. 원인은 기기 API 자체가 아니라 AVD 메타데이터·테스트 이미지 설정이었다. 이 AVD에서 API 37.2 listener 등록 및 callback 런타임은 확인됐다.
- listener 등록을 표면 세대에 묶고, `surfaceDestroyed`에서 해당 registration의 제거를 요청한다. API 37.2 회전 시험에서 세대 1→2→3, 이전 listener 제거 요청 및 새 listener 등록을 확인했다. callback은 UI와 분리된 serial executor에서 원시 `vsyncId`, `presentTimeNanos`, `jankType`을 기록한다. executor queue는 용량 64로 제한하며 callback batch와 flush 거부를 별도로 계수한다. 포화 fixture에서 64개 수락, 초과 callback/flush 각 1개 거부, caller-runs 없음, worker 복구를 확인했다. callback의 late-arrival 자체 주입과 idle 장기 수집은 미실행이다.
- API 37.2의 JankData 경로에서 두 renderer의 3×10 scored sequence VSync ID join을 확인했다. JankData 자체 timestamp는 모두 unknown이므로 그 API 경로만으로는 event_to_os_present를 계산하지 않는다. 별도 TransactionStats present fence는 같은 transaction의 OS 표시 신호를 제공해 이번 Android 행렬에서 후보 event→transaction-present를 산출했다. API 36 fallback은 입력·draw를 유지하며 JankData listener를 호출하지 않는다.
- 이전 capability 및 FrameTimeline ID join 실험의 record 수는 탭마다 생성된 frame 수를 뜻하지 않는다. 해당 ID join fixture는 block별 10개 scored input과 1개 unscored drain input을 분리해 WGPU·GLES 각 30/30 exact unique match를 확인했고, 그 실험에서 나온 JankData record 60개의 presentTimeNanos는 모두 unknown(-1)이었다. 후속 TransactionStats fence는 별도 timestamp 경로이며 [반복 행렬 근거](../spec/internal/evidence/r05-android-present-fence-2026-10-08.md)를 따른다. invalid/stale ID, pending lifecycle race, frame drop, 연속 렌더링, refresh mode 전환, iOS drawable API runtime은 미검증이다.
- 후속 구현 검토에서 surface generation 최대치가 holder callback 예외로 이어질 수 있음을 발견했다. WGPU/GLES 양 경로가 무효 generation sentinel에서 표시 listener 등록만 건너뛰도록 수정했다. 새 APK를 다시 빌드해 API 37.2 WGPU/GLES 1회 탭, API 36 GLES fallback 3회, queue 포화, 충돌 모드 거부를 재확인했다. 앞선 세 block 비교는 이 overflow 전용 분기 변경 전 실행이며, 정상 경로를 최종 APK smoke로 재확인했다. 원본은 [최종 구현 검토와 로그](../spec/internal/evidence/r05-presentation-signal-probe-2026-10-08.md#r05-android-final-hostile-review)에 있다.
- 이번 실행 환경에는 Android 37.2 시스템 이미지가 설치되어 listener runtime까지 확인했다. 다른 환경에서 이미지나 공개 API가 없으면 미지원으로 기록하며, API37.0을 API37.2 대조로 대신하지 않는다.

#### iOS 공개 SDK 경로

- Apple 공개 Metal API는 iPhoneOS SDK에 있고, Xcode 26.2 iPhoneOS target type-check가 통과했다. 같은 Xcode의 Simulator SDK header/type-check에는 관련 선언이 없다. 상세 실행 근거는 [iOS SDK capability matrix](../spec/internal/evidence/r05-presentation-signal-probe-2026-10-08/ios-sdk-matrix-typecheck.log)다.
- 후속 R05.3 실행에서 같은 root layer subclass의 acquire hook을 Simulator WGPU 경로에서 확인했다. XCTest 단일 tap의 input sequence 1·revision 1을 draw sequence 4에 exact join했고 probe-off R08 대조에는 acquire 로그가 없었다. device-only callback 분기는 iPhoneOS SDK target 격리 app harness의 compile/link까지 통과했다. [iOS 실행 근거](../spec/internal/evidence/r05-ios-present-feedback-2026-10-08.md).
- wgpu frame ID와 drawable ID 사이의 device callback closure ticket은 캡처하지만 callback ledger/timeout/lost accounting은 미구현이다. surface 교체 뒤 늦은 callback, frame drop, 중복/누락 callback, host-time과 input uptime 변환 오차는 미검증이다. command-buffer completion·`CADisplayLink`는 대체 신호가 아니다.

#### 공통 정지 조건

실제 OS 표시 timestamp와 해당 renderer의 frame/revision exact join 및 clock 변환 오차가 모두 플랫폼별로 검증되기 전에는 제품 입력 지연으로 보고하지 않는다. Android AVD에서는 TransactionStats present-fence와 clock bracket으로 synthetic 입력의 transaction-present 후보를 계산했으나 API별 기기 일반화가 남았다. iOS는 Simulator acquire join까지만 통과했고 device `presentedTime` runtime join은 미완료다. JankData presentTimeNanos 자체는 API 37.2에서도 unknown이며 이 경로만으로 지연을 산출하지 않는다. OS display signal은 물리적 scanout/광자 시각의 증명이 아니다.

### 2단계 · 물리 입력 검증 (사용자 요청 뒤에만 실행)

- 기기 연결·화면 잠금 해제·앱 foreground를 확인한 후 사용자가 명시적으로 요청한 경우에만 진행한다. 현재는 Android/iOS 실기기 조작·캡처를 하지 않는다.
- 물리 입력은 `MotionEvent`/`UITouch` 경로로만 분류한다. `adb shell input`, `simctl`, UI test 주입은 synthetic 별도 그룹이며 물리 입력 결과를 대체하지 않는다.
- 기기당 사전 warm-up과 최소 10개 독립 실행 block, block당 유효 입력·표시 표본 최소 30개(총 300개)를 기준으로 한다. 이는 탐색적 p95를 계산할 최소 수집량이지 비교 승자를 판정할 충분성 보장이 아니다. 입력 시퀀스·누락·추가 frame·중복 frame을 보존한다. p95는 유효 표본 300개 이상일 때만 95% block-bootstrap 신뢰 구간과 함께 탐색치로 보고하고, 미달이면 개별 값·중앙값·표본 수만 낸다. 기기 간 우열 또는 성능 우위는 이 작업에서 결론 내리지 않는다.
- 기기별 결과를 분리한다. Android와 iOS의 latency 분포를 한 모집단으로 합치거나 기기 간 absolute rank를 만들지 않는다. debug/trace-on 진단과 release/trace-off 결과도 각각 분리한다.
- 열 상태, display refresh mode, 앱 전경/프로세스, surface generation, orientation, 배터리/전원 상태를 매 block 기록한다. 유효 표본 실패와 느린 outlier를 구분하고, 느리다는 이유만으로 제외하지 않는다.

## 표본 실패·제외 규칙

| 상황 | 처리 |
|---|---|
| 입력 event 출처가 synthetic 또는 확인 불가 | 물리 입력 표본에서 제외하고 synthetic/unknown 수를 보고 |
| event timestamp clock 또는 변환 anchor 없음 | 해당 latency 계산 불가로 기록, 다른 시각으로 추정하지 않음 |
| frame revision/sequence와 actual presentation 사이 연결이 모호하거나 다중 match | 표본을 미연결로 분류하고 `event_to_os_present`에서 제외; 연결 실패율 보고 |
| presentation timestamp가 unknown/unset/0, frame drop | 미표시 표본으로 세고 latency 0으로 처리하지 않음 |
| 앱 foreground 상실, 프로세스 재시작, surface generation 전환 | 해당 block을 무효 처리하고 이유·원본 보존 |
| trace packet loss, 필수 marker 누락, FrameTimeline이 target surface와 다름 | 캡처 전체를 무효 처리; 값 일부만 집계하지 않음 |
| 큰 latency, jank 또는 늦은 presentation | 유효한 연결이면 표본에 유지; 결과 확인 뒤 이상치 제외 금지 |
| instrumentation marker 추가로 인한 비용 | 별도의 on/off 대조와 실행 digest를 기록; 계측 영향 미확정이면 진단치로만 분류 |

## 산출물

1. 양 플랫폼 입력·revision·frame·presentation 간 상관 규칙과 clock/error 검증 결과.
2. raw log/trace, checksum, 기기·OS·빌드·refresh mode, 입력 출처와 표본 제외 manifest.
3. per-run 원시 개수, 중앙값·범위·표본 수; 충분한 경우 p95와 신뢰 구간. 불확실한 광학 오차는 숫자로 꾸미지 않는다.
4. 정확한 frame join이 한 플랫폼에서 성립하지 않으면 무엇이 빠졌는지 구체적인 trace/API 경계와 미지원 결과.
5. `spec/STATUS.md`에서 R05를 미완료로 유지한다. 시뮬레이터 성공이나 OS present 신호는 실기기, 광학 scanout, 제품 비교 앱의 완료 근거가 아니다.

## 계획 적대 검토

계획 본문·Android `FrameAttributionActivity`/R08 `SurfaceView` 경로·iOS R08 Metal/wgpu 경로·플랫폼 원시 문서를 다음 20개 독립 실패 관점에서 대조했다. 2·4·5·7·8·9·11·12·19번에서 계약을 더 명확히 해야 하는 점을 발견해 본문에 반영했다. 각 항목은 단순 반복이 아니라 별도 시간 의미, 객체 수명, API capability 또는 표본 무결성을 확인한다.

| # | 독립 실패 관점 | 실제 대조와 결과 |
|---:|---|---|
| 1 | 자동 입력을 물리 touch로 오분류 | Android/iOS 시뮬레이터 경로와 계획을 대조했다. 출처 필드 및 synthetic 별도 집계가 있어 통과. |
| 2 | Android ACTION_DOWN과 iOS touch 종료를 섞거나 접촉 시작으로 과장 | Android `onTouchEvent`·iOS `touchesEnded`를 확인했다. 기존 계획에 phase 계약이 없어 단일 포인터 `ACTION_UP`/`.ended`로 고정하고 cancel·다중 touch·접근성은 분리하도록 수정. |
| 3 | ns 단위 표현을 ns 정확도로 해석 | Android 공식 API는 ns precision이 ns accuracy를 보장하지 않는다고 명시한다. 오차 한계·정확도 기록 계약이 있어 통과. |
| 4 | Android uptime, JankData, trace clock을 직접 빼거나 sleep 차이를 무시 | Perfetto 변환에 더해 JankData의 문서화되지 않은 clock epoch도 anchor로 확인하기 전 계산하지 않도록 보강. |
| 5 | iOS 초 단위 timestamp들이 같은 epoch이라고 가정 | `UITouch.timestamp`는 system uptime, Metal은 host time이다. 기존 anchor source가 모호해 `ProcessInfo.systemUptime`/`CACurrentMediaTime` 검증으로 구체화. |
| 6 | 예상 presentation을 실제 시각으로 기록 | `Choreographer.FrameTimeline` 문서의 expected 시각/VSync ID와 Perfetto actual timeline을 구분한다. 통과. |
| 7 | Perfetto actual timeline의 slice 시작/끝을 곧바로 present time으로 취급 | 공식 FrameTimeline 설명을 확인했다. `ts`/`ts+dur`를 직접 표시 시각 대체로 쓰지 않도록 금지하고, 의미가 검증된 direct signal만 latency에 쓰게 수정. |
| 8 | 지원되지 않는 현재 Android GPU surface를 지원된다고 가정 | 코드상 현재 R08 wgpu 출력은 `SurfaceView`; Perfetto 공식 문서는 SurfaceView FrameTimeline 미지원이라고 밝힌다. 계획에 사전 capability gate와 API36 결과를 미지원으로 남기는 조건을 추가. |
| 9 | Android API 36에 없는 direct present getter를 호출 | 공식 `JankData` 문서에서 `getPresentTimeNanos()`는 version 37.2로 확인했다. runtime API/extension capability 확인, 없으면 대체 timestamp 금지를 추가. |
| 10 | navigation/status overlay 또는 다른 앱 surface를 target으로 연결 | package/process, exact output layer, surface generation을 표본 key 및 합격 조건에서 확인했다. 통과. |
| 11 | 입력 revision과 렌더 frame이 합쳐지거나 서로 어긋남 | 기존 계획은 연속 입력 coalescing을 명시하지 않았다. isolated baseline, `render_revision`, 겹침/coalescing 무귀속 집계를 추가. |
| 12 | 한 revision에 여러 frame이 연결되어 latency 선택 편향 | 첫 유효 presentation signal을 귀속 기준으로 본문에 명시하고 후속 동일 revision frame은 별도 보존한다. |
| 13 | dropped/zero/unknown 값을 0ms 성공으로 셈 | Android unset/unknown, iOS `presentedTime == 0`, drop을 실패/누락으로 기록한다. 통과. |
| 14 | surface 재생성 뒤 stale frame을 새 입력에 연결 | generation을 join key로 쓰고 lifecycle 전환 block을 무효화한다. 통과. |
| 15 | trace packet loss, callback 지연 또는 필수 anchor 누락을 부분 결과에 숨김 | 품질 오류·marker·anchor 누락 시 capture/block 전체 무효화하며 끝의 비동기 callback은 bounded drain 뒤 pending/lost로 남기도록 보강. |
| 16 | 느린 표본을 결과를 본 뒤 제거 | 사전 제외 규칙과 느린 유효 표본 유지 조건을 확인했다. 통과. |
| 17 | refresh/thermal/background 변화를 런타임 차이로 오해 | 플랫폼·기기별 분리와 refresh/열/전원/foreground 보존 조건이 있다. 통과. |
| 18 | 새 marker 비용을 이전 계측 오버헤드로 대신 | 이번 revision/frame correlation 표식에 대한 새 on/off 비교를 요구한다. 통과. |
| 19 | 적은 표본으로 p95나 비교 승자를 과장 | 기존 “충분한 표본”이 모호했다. 총 300개를 탐색 p95 최소 수집량으로 명시하고 95% block-bootstrap 구간 및 비승자 판정 제한을 추가. |
| 20 | OS signal을 photon/screen scanout 완료로 표현 | OS signal의 의미와 광학 측정의 경계를 명시하고 센서 없이는 input-to-photon을 내지 않는다. 통과. |

남은 확인점은 stale/invalid VSync token과 surface lifecycle failure path, Android present timestamp가 unknown인 원인과 clock conversion residual, iOS device callback runtime·bounded callback/lost accounting·drawable/ticket join·clock conversion residual, 전체 V8 app bundle 통합, 실기기 적용성이다. Android API 37.2 AVD에서는 renderer당 30 sequence의 on-demand token join만 확인했다. iOS Simulator는 acquire 귀속만 확인했다. 실기기 검증은 사용자의 요청 전에는 실행하지 않는다.

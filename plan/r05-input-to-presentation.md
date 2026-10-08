# R05.3 · 입력 이벤트에서 GPU 프레임 표시 신호까지 계측 계획

**상태:** 적대적 계획 검토 20개 완료 · 시뮬레이터 입력→제출 probe 부분 구현 · actual-present 신호 미지원 확인 · 실기기 실행은 사용자 요청 대기

**상위 항목:** [R05 계측 상태](../spec/STATUS.md) · [R05 계측 계약](../spec/internal/0022-r05-benchmark-attribution.md) · [비교 측정 계획](../docs/plans/benchmark.md)

## 목적과 주장 경계

현재 iOS R05 probe는 버튼 selector와 `CADisplayLink`를 사용하고 Android R05 대조 화면은 View `onClick`을 사용한다. 두 경로 모두 물리 입력 시각과 GPU 표면의 특정 프레임이 표시된 시각을 직접 연결하지 않는다. 따라서 현재까지의 R05 결과는 입력→픽셀 지연 또는 입력→화면 표시 완료로 해석하지 않는다.

R05.3의 목표는 아래 세 사건을 하나의 입력 sequence 및 렌더 revision에 연결하는 계측 경로를 확정하고, 우선 시뮬레이터에서 그 연결이 실제로 관측되는지 검증하는 것이다.

1. OS가 전달한 입력 event timestamp와 앱 입력 handler 진입.
2. 입력으로 바뀐 고유 화면 revision의 GPU frame 제출.
3. 그 frame에 대해 Android compositor 또는 iOS Metal이 보고한 표시 시각.

핵심 지표 이름은 **OS 입력 이벤트 → OS 표시 신호**다. Android `MotionEvent.getEventTimeNanos()`는 uptime 기준의 나노초 표현이며 API 문서도 정밀도와 정확도를 구분한다. Android `Choreographer.FrameTimeline`은 예상 표시 시각과 VSync ID를 제공하고, 실제 frame과의 연계는 Perfetto FrameTimeline 또는 검증된 frame ID 경로로 확인해야 한다. Android Perfetto의 `actual_frame_timeline_slice`는 compositor가 관측한 actual frame을 제공하지만 픽셀 발광의 광학 측정은 아니다. [MotionEvent](https://developer.android.com/reference/android/view/MotionEvent#getEventTimeNanos()) · [Choreographer.FrameTimeline](https://developer.android.com/reference/android/view/Choreographer.FrameTimeline) · [Perfetto FrameTimeline](https://perfetto.dev/docs/data-sources/frametimeline)

iOS는 `UITouch.timestamp`와 `MTLDrawable.presentedTime`을 후보로 삼는다. Apple은 후자를 drawable이 화면에 표시된 host time으로 정의하며, 미표시 또는 drop된 frame의 값은 `0.0`일 수 있다고 명시한다. 실제 출력 경로에서 drawable ID·렌더 revision을 `addPresentedHandler` 결과까지 보존할 수 있어야 한다. `CADisplayLink.timestamp/targetTimestamp`는 frame callback 시간표시로만 기록하며 drawable 표시 완료를 대체하지 않는다. 이 신호도 광학적 scanout 측정은 아니다. [UITouch.timestamp](https://developer.apple.com/documentation/uikit/uitouch/timestamp) · [MTLDrawable.presentedTime](https://developer.apple.com/documentation/metal/mtldrawable/presentedtime) · [addPresentedHandler](https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler%28_%3A%29) · [CADisplayLink.timestamp](https://developer.apple.com/documentation/quartzcore/cadisplaylink/timestamp)

**finger-to-photon** 또는 실제 픽셀 발광 시각은 이 작업의 지표가 아니다. 그런 주장은 동기화된 외부 광학 센서와 그 교정 자료가 있을 때 별도 측정으로 추가한다. 계측 API의 시간 단위가 ns라고 해서 정확도·광학 오차가 ns라는 뜻은 아니다.

## 사전 비교 모델과 단위

각 표본은 `input_sequence`, `render_revision`, `surface_generation`, 플랫폼 frame 식별자, 입력 출처, 원시 timestamp와 clock domain을 함께 가진다. 기준 탭 동작은 Android 단일 포인터 `ACTION_UP`과 iOS 단일 touch `UITouch.Phase.ended`로 고정한다. 이는 OS가 전달한 종료 event 기준이며 손가락 접촉 시작이나 digitizer 원시 시각이 아니다. `ACTION_DOWN`/move, 취소, 다중 touch/pointer, 접근성 activation은 기준 표본에서 제외하고 별도 실험으로 분리한다. 입력마다 고유 sequence가 GPU 표면에 실제 색상/도형 변화로 보여야 한다. 텍스트 상태 label만 바뀌고 GPU 표면은 그대로인 경우는 GPU 표시 표본으로 인정하지 않는다.

플랫폼별로 별도 계산한다.

- `event_to_handler`: OS 입력 event timestamp → 앱 handler timestamp. handler 시각도 해당 플랫폼의 단조 clock으로 기록하고 원시 event 시각과 같은 clock으로 직접 비교할 수 있는지 확인한다.
- `handler_to_submit`: handler 진입 → 해당 revision을 포함한 GPU frame 제출.
- `event_to_os_present`: 입력 event timestamp → 같은 revision의 OS actual-present timestamp. exact frame correlation을 입증했을 때만 산출한다.
- 선택적인 `physical_contact_to_photon`: 이 작업에서는 산출하지 않는다. 별도 광학 센서·공통 clock·교정 오차가 확보되기 전에는 `미측정`이다.

Android event timestamp와 Perfetto timestamp를 직접 빼지 않는다. 단조 시계·trace clock 변환을 실행 전후 anchor로 검증하고 offset·분해능·불확실성 범위를 보존한다. `JankData.getPresentTimeNanos()`를 쓰는 경우에도 공식 문서가 명시하지 않는 clock epoch를 같은 입력/handler clock과 anchor로 확인하기 전에는 빼지 않는다. Android API 34 미만 fallback의 millisecond event time은 별도 정밀도 등급으로 기록하며 ns 표본과 합치지 않는다. FrameTimeline의 expected timestamp, `CADisplayLink` callback, `Queue::present()` 반환은 actual-present 값으로 대입하지 않는다.

Android 경로는 **capability 확인을 먼저** 한다. 현재 R08 GPU renderer의 Android surface는 `SurfaceView`이며, Perfetto 문서는 FrameTimeline이 `SurfaceView`를 지원하지 않는다고 명시한다. 따라서 API 36의 현재 surface에서 FrameTimeline 표본이 보일 것이라고 가정하지 않는다. API 36의 `SurfaceControl.JankData` 자체는 jank·앱 frame time·VSync ID 정보를 제공하지만 `getPresentTimeNanos()`는 공식 문서상 version 37.2부터다. 런타임 API/extension 수준과 실제 surface의 signal 제공 여부를 기록하고, direct present timestamp를 얻을 수 없으면 API 36 결과는 **표시 지연 미지원**으로 남긴다. FrameTimeline의 `ts` 또는 `ts + dur`를 실제 표시 시각으로 임의 대입하지 않는다. 해당 데이터는 검증된 의미에 따라 frame 실행/제출 진단치로만 기록한다. exact surface·revision 연결과 표시 timestamp 의미까지 입증된 경로가 있을 때만 `event_to_os_present`를 낸다. [Perfetto FrameTimeline 지원 범위](https://perfetto.dev/docs/data-sources/frametimeline) · [SurfaceControl.JankData](https://developer.android.com/reference/android/view/SurfaceControl.JankData)

iOS에서는 `UITouch.timestamp`와 `MTLDrawable.presentedTime`을 원시값으로 보존한다. `UITouch.timestamp`는 system uptime 기준이고 drawable 값은 host time이므로, `ProcessInfo.systemUptime`·`CACurrentMediaTime` anchor를 실행 전후 수집해 관계와 residual을 확인한다. 단위가 초라는 이유만으로 두 값이 같은 epoch이라고 가정하지 않는다. 공통 시간축과 최대 변환 오차를 설명하지 못하면 `event_to_os_present`를 내지 않는다. `presentedTime == 0`은 0ms 성공으로 바꾸지 않고 미표시/drop 표본으로 분류한다.

오차 보고서는 적어도 입력 timestamp의 API 분해능·정확도 한계, clock 변환 residual, frame identity match 상태, drop/누락 표본 수를 포함한다. OS가 보장하지 않는 값에 임의의 수치 오차 한계를 만들어 넣지 않는다. 잔차 상한을 측정할 수 없으면 숫자 지연과 함께 “측정 오차 미확정”을 표기하고 플랫폼 간 순위를 내지 않는다.

## 실행 단계와 합격 조건

### 1단계 · 시뮬레이터에서 연결 가능성 증명

- Android API 36 ARM64 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 실행한다. 앱·surface·OS·빌드·refresh mode·source/app digest를 기록한다.
- 자동 입력은 계측 파이프라인 smoke에만 사용하고 `synthetic`으로 표시한다. 기존 selector/ADB 입력을 physical input 데이터로 분류하지 않는다.
- 입력마다 단조 증가 sequence와 별도 render revision을 만들고, 업데이트한 frame에 exact revision·frame identity를 연결한다. 입력에 귀속하는 표시는 해당 revision을 포함한 target surface의 첫 유효 presentation signal로 고정한다. 다음 입력은 앞 입력의 결과가 표시 신호와 연결된 뒤 전달하는 isolated baseline으로 한다. 겹친 입력, 한 frame에 합쳐진 revision, 입력 없이 생긴 frame은 서로 억지로 1:1 연결하지 않고 `overlap/coalesced/unattributed`로 센다. burst는 별도 workload로 둔다.
- Android에서는 실행 전에 API/extension 수준, renderer surface 종류, FrameTimeline/JankData capability를 기록한다. 해당 앱 surface의 exact actual timeline, 입력 event와 frame ID의 join, 직접 표시 timestamp의 의미, trace 품질·유실·중복을 모두 확인한다. 현재 `SurfaceView`가 FrameTimeline에서 제외되거나 API 36 runtime에 직접 표시 timestamp가 없으면 이 단계는 해당 경로의 capability 실패다. `ts`, `ts + dur`, 예상 시각, 앱 제출 시각을 대체값으로 사용하지 않는다.
- iOS에서는 현재 wgpu/CAMetalLayer 경로에서 같은 출력 drawable의 `drawableID`, revision과 `presentedTime`을 callback까지 연결할 수 있는지 확인한다. wgpu가 drawable identity/callback을 노출하지 않으면 필요한 계층과 변경 범위만 기록하고, native Metal 대조값을 wgpu 제품 경로의 대체값으로 쓰지 않는다. 시뮬레이터에서 값이 0이거나 path가 노출되지 않으면 실패/미지원으로 남기고 `CADisplayLink`로 대체하지 않는다.
- 각 플랫폼은 최소 30개의 sequence를 사용해 생성·handler·revision·frame·presentation 레코드의 개수와 1:1 관계를 검사한다. block 종료 때 presentation callback/trace 수집기를 bounded drain하고, 기한 뒤 도착하거나 끝내 미도착한 레코드는 pending/lost로 남긴다. 입력 누락, 중복 ID, stale surface generation, trace drop, 잘못된 clock join은 집계 성공으로 상쇄하지 않는다.
- 계측을 추가한 뒤 trace on/off 영향을 다시 확인한다. 이전 R05 계측 오버헤드 표본은 새 marker·새 frame correlation 비용을 포함하지 않으므로 새 계측의 오버헤드 근거로 재사용하지 않는다.

### 1단계 실행 결과 · 2026-10-08

Android API 36 Emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 synthetic 바깥 탭과 중앙 탭을 실행했다. Android·iOS wgpu 경로 및 iOS native Metal 대조에서 중앙 입력 sequence와 revision이 연결되고 실제 색상 변경을 확인했다. iOS 양 renderer에서는 바깥 시작→안쪽 종료 swipe와 8pt를 넘는 내부 이동 swipe를 표본에서 제외했고, R13 window 재부착 뒤 surface generation 2 연결도 확인했다. 8pt는 probe의 gesture 오염 필터일 뿐 공개 입력 계약이 아니다. R05 비활성 대조에서도 기존 R08 입력 경로가 동작했다. [실행 로그·화면·체크섬 및 구현 적대 검토](../spec/internal/evidence/r05-presentation-signal-probe-2026-10-08.md).

이 단계는 actual-present 연결 가능성 검증에는 실패했다. Android R08 surface가 Perfetto FrameTimeline의 지원 surface가 아닌 `SurfaceView`이며, 현재 iOS Simulator SDK는 문서상 Metal drawable presentation callback 멤버를 제공하지 않는다. 두 플랫폼 모두 제출/command completion을 표시 완료 신호로 대체하지 않았고 event-to-present 값은 산출하지 않았다. 단일 submit-call 지연 관측은 재현 측정 전까지 성능 근거로 쓰지 않는다. 다음 gate는 공개 SDK/API에서 exact surface/frame 표시 신호를 얻는 지원 경로나 별도 광학 측정 경로를 찾는 것이다. 실기기 검증은 사용자 요청 후에만 실행한다.

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

남은 확인점은 Android에서 지원되는 다른 surface/API로 exact frame과 입력 revision을 join할 수 있는지, 현재 배포 SDK에서 iOS drawable 표시 신호를 제공하는지, 양쪽 clock conversion residual, 최소 30 sequence의 레코드 무결성이다. 현재 R08 시뮬레이터 probe는 capability 한계까지만 기록했다. 실기기 검증은 사용자의 요청 전에는 실행하지 않는다.

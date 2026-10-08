# R05.3 Android FrameTimeline ID 연결 가능성 확인

**상태:** 계획 검토 완료 · Android API 37.2 AVD 실행 완료 · 표시 timestamp gate 미통과

## 목적

기존 Android API 37.2 `SurfaceView.registerOnJankDataListener()` probe는 콜백과 `vsyncId`를 받았지만 `presentTimeNanos`는 모두 unknown이었다. 이 계획은 입력으로 바뀐 revision을 특정 SurfaceView buffer frame에 붙이고, 콜백의 `JankData.getVsyncId()`와 정확히 같은 ID인지 확인한다. 시간 근접도로 연결하지 않으며, 성공해도 표시 timestamp가 unknown이면 지연 시간은 계산하지 않는다.

## 공개 API 근거

- [`Choreographer.postVsyncCallback`와 `FrameTimeline`](https://developer.android.com/reference/android/view/Choreographer): API 33부터 다음 VSync의 timeline 후보를 받을 수 있다.
- [`SurfaceControl.Transaction.setFrameTimeline`](https://developer.android.com/reference/android/view/SurfaceControl.Transaction#setFrameTimeline(long)): API 35부터 선택한 `FrameTimeline.vsyncId`를 SurfaceFlinger에 전달한다. 문서는 stale/invalid ID는 무시한다고 설명한다.
- [`SurfaceView.applyTransactionToFrame`](https://developer.android.com/reference/android/view/SurfaceView#applyTransactionToFrame(android.view.SurfaceControl.Transaction)): API 34부터 transaction을 다음 SurfaceView frame에 동기화한다. 정확한 대상 frame은 호출 전 SurfaceView rendering이 정지된 때만 정의되며, 계속 렌더링 중이면 어느 frame에 붙을지 정의되지 않는다.
- [`SurfaceControl.JankData.getVsyncId`](https://developer.android.com/reference/android/view/SurfaceControl.JankData#getVsyncId()): JankData의 frame ID를 돌려주며 `FrameTimeline` 및 `Transaction.setFrameTimeline`과 연결할 수 있음을 문서화한다.
- [`SurfaceControl.JankData.getPresentTimeNanos`](https://developer.android.com/reference/android/view/SurfaceControl.JankData#getPresentTimeNanos()): 실제 표시 시각은 API 37.2부터 제공하며 unknown/unset sentinel을 반환할 수 있다.

## 시험 절차

1. Android API 37.2 시뮬레이터의 R05 전용 WGPU·GLES 경로에서 surface가 유효하고 이전 draw 요청이 끝나 렌더링이 멈춘 상태임을 확인한다. 두 renderer 모두 이벤트 사이에는 새 buffer를 제출하지 않는 현재 on-demand 경로만 사용한다.
2. 중앙 synthetic 탭 하나를 `input_seq`와 새 `revision`으로 고정한다. 이벤트를 처리하는 main thread에서 `Choreographer.postVsyncCallback`을 한 번 요청한다.
3. callback에서 `FrameData.getPreferredFrameTimeline()`이 반환한 `FrameTimeline`의 `vsyncId`, deadline, expected presentation time, surface generation을 원시값으로 기록한다. 선택한 timeline은 예정 시각일 뿐 actual presentation으로 취급하지 않는다.
4. 현재 세대와 Surface가 그대로이고 중간 draw가 없을 때만 `new SurfaceControl.Transaction().setFrameTimeline(vsyncId)`를 만들고 `applyTransactionToFrame(transaction)`을 호출한 뒤 딱 한 번 draw를 제출한다. 입력·revision·generation·target vsync ID를 `draw_seq`에 연결해 기록한다.
5. JankData callback에서 exact integer equality로 target ID를 찾는다. record 배열 순서·callback 시각·expected presentation time을 연결 기준으로 쓰지 않는다. 즉시 flush가 제출보다 앞선 batch만 반환할 수 있으므로, draw 뒤 bounded delayed flush를 한 번 더 요청해 마지막 입력까지 배출한다. 동일 ID가 중복되거나 target 누락·다중 match면 해당 입력을 실패/미연결로 남긴다.
6. 각 renderer에 3개 독립 block을 실행한다. 각 block에서 처음 10개 입력만 점수화하고, 마지막 frame의 비동기 JankData 배출을 유도하는 11번째 synthetic tap은 `unscored_drain_control`로 구분한다. block은 앱 프로세스를 새로 시작해 고정 용량 64개 target registry를 넘지 않게 한다. renderer별 점수 대상 30개가 모두 exact match되어야 frame identity 경로를 통과로 판정한다. 성능 비교나 기기 일반화는 하지 않는다.
7. 별도 부정 대조에서 실제 ID보다 오래된 token을 재사용하고, surface generation 전환 뒤 callback이 도착한 경우를 수집한다. 무효 token, 예외, surface 변경은 latency 표본에서 제외하고 이유를 원본 로그에 보존한다.
8. `presentTimeNanos`는 기존 sentinel 의미대로 따로 집계한다. exact ID가 맞아도 timestamp unknown/unset이면 event-to-present latency는 미측정이며 R05.3을 완료 처리하지 않는다.

## 안전 조건 및 제외

- 이 코드는 API 37.2 AVD의 내부 탐색 probe다. 공개 API가 있는 API 35 이상이라고 runtime 동작이 모두 같다고 일반화하지 않는다.
- 지속 렌더링을 시작했거나 surface 세대가 바뀐 뒤에는 transaction을 적용하지 않는다. 연속 렌더링 경로를 별도로 재현하면 해당 표본을 미연결로만 기록한다.
- `applyTransactionToFrame`이 반환했다는 사실은 transaction 적용이나 화면 표시를 증명하지 않는다. 일치하는 JankData ID만 frame-token 연결 증거로 취급한다.
- VSync ID 일치는 입력 발생 자체의 인과성을 단독으로 증명하지 않는다. 이 fixture에서 이벤트별 고유 revision을 draw 로그와 함께 보존하고, 다른 입력·기본 surface frame과 ID가 중복되지 않아야 한다.
- API timestamp가 positive여도 clock epoch·clock 변환 오차 검증 전에는 latency를 계산하지 않는다. 광학적 pixel 발광 시각은 측정하지 않는다.
- iOS는 이번 Android 실험에 포함하지 않는다. 설치된 Xcode SDK의 공개 Metal drawable identity/presentation 심볼 availability를 별도 gate로 보존하고, 심볼 또는 wgpu drawable 연결이 확인되지 않으면 `CADisplayLink`/command-buffer completion으로 대체하지 않는다.
- 실기기 실행은 사용자가 요청하기 전에는 하지 않는다.

## 중단 기준

다음 중 하나면 해당 renderer의 방법을 no-go로 기록하고 정확한 귀속 경로가 확보될 때까지 후속 latency 계산을 중단한다.

1. 렌더링이 실제로 정지된 상태임을 확인할 수 없다.
2. 특정 revision의 draw가 transaction보다 앞서 제출됐거나 같은 세대에서 여러 draw가 경쟁한다.
3. JankData ID가 target ID와 다르거나, 누락·중복·세대 혼합이 발생한다.
4. callback이 target frame을 관찰하기 전에 registration 제거·프로세스 종료·queue 손실이 발생한다.
5. timestamp가 unknown/unset이거나 clock domain 변환을 검증하지 못했다.

## 계획 적대 검토 · 20개 독립 관점

| # | 실패 관점 | 계획에서 고정한 대응 |
|---:|---|---|
| 1 | preferred VSync ID를 actual present time으로 오해 | deadline/expected 시각은 예정값으로 로그에 분리하고 실제 표시는 JankData만 사용한다. |
| 2 | `setFrameTimeline` 호출만으로 ID가 buffer에 붙었다고 오해 | `applyTransactionToFrame` 뒤 그 ID와 같은 JankData record를 받아야 연결로 인정한다. |
| 3 | SurfaceView가 계속 렌더링 중이라 transaction 대상이 비결정적 | 호출 전 idle draw 상태를 요구하고 연속 렌더링 표본은 제외한다. |
| 4 | 입력 하나에 여러 렌더 요청이 경쟁 | 입력당 하나의 revision·하나의 draw만 허용하고 `draw_seq`를 함께 기록한다. |
| 5 | transaction 직전 SurfaceView가 제거되거나 새 세대가 됨 | generation과 유효 Surface를 호출 직전 재검사하고 stale 결과를 제외한다. |
| 6 | callback의 같은 ID를 우연한 이웃 frame으로 매칭 | 시간 거리나 배열 위치를 금지하고 정확한 token equality만 쓴다. |
| 7 | callback batch에 같은 ID가 중복 포함 | 다중 match를 실패로 분류하고 중복 수를 보존한다. |
| 8 | target callback이 늦거나 즉시 flush가 새 buffer보다 먼저 실행 | draw 직후 bounded delayed flush를 요청하고, block 마지막에는 별도 unscored drain control 뒤까지 대기한다. drain timeout·pending은 미연결로 남긴다. |
| 9 | 잘못된/stale token이 기본 timeline으로 조용히 대체됨 | stale token 부정 대조를 두고 실제 callback ID가 target과 다르면 실패다. |
| 10 | API 버전별 method linkage 오류가 앱 종료 | API 경계를 검사하고 API 37.2 AVD만 runtime 판정 대상으로 둔다. |
| 11 | JankData listener 제거가 pending callback을 놓침 | target 관찰 전 listener를 제거하지 않고 종료 시 pending 표본 수를 기록한다. |
| 12 | `applyTransactionToFrame` 예외가 성공으로 집계 | 호출 실패·surface 없음·linkage 오류를 별도 outcome으로 기록한다. |
| 13 | synthetic 입력을 실제 사용자 입력으로 표현 | 전 표본을 synthetic으로 표시하고 실기기 입력 통계와 섞지 않는다. |
| 14 | 한 renderer에서만 일치한 결과를 양쪽 renderer로 일반화 | WGPU와 GLES를 독립 판정하고 renderer별 exact match율을 낸다. |
| 15 | 블록 합계가 block 내 누락·중복을 숨김 | renderer별 3×10 sequence를 독립 검증하고 sequence별 원시 매핑을 남긴다. |
| 16 | 입력 revision과 VSync ID만으로 실제 pixel buffer 내용을 입증 | 해당 revision을 그린 draw 로그와 surface generation도 요구하고 광학 측정과 분리한다. |
| 17 | positive timestamp를 입력 시계와 바로 빼서 latency 계산 | clock anchor/residual gate 전에는 수치 계산을 금지한다. |
| 18 | queue saturation 또는 trace 누락으로 callback 데이터 유실 | reject/drop counter와 필수 marker를 검사하고 손실이 있으면 block 무효화한다. |
| 19 | iOS 미측정을 Android 결과로 채움 | iOS는 별도 SDK capability gate로 유지하며 Android 데이터로 대체하지 않는다. |
| 20 | 성공한 AVD 결과를 지원 기기 전체 또는 제품 지원으로 과장 | API 37.2 AVD 내부 probe로 범위를 제한하고 R05.3 완료 체크는 금지한다. |

### 계획 검토 결과

계획 자체의 20개 실패 관점을 원시 API 계약, Android 37.2 `android.jar`, 기존 R05 draw/lifecycle 경로, 입력·revision 기록 조건과 대조했다. 대조 중 `FrameData` API를 preferred index가 아닌 `getPreferredFrameTimeline()`으로 바로잡았다. 공개 문서상 exact next-frame 조건이 정지 rendering에 한정됨을 전제에 반영했다. 이 검토는 아직 실행되지 않은 probe의 구현 검토나 runtime 성공 판정을 대신하지 않는다.

## 실행 결과

Android 17/API 37.2 ARM64 AVD(`emulator-5562`, 1080×1920, 420 dpi, 60 Hz)에서 최종 debug APK를 실행했다. WGPU/Vulkan과 GLES 2.0 API 경로를 각각 3개 독립 프로세스 block으로 나누고, block마다 10개 점수 표본 뒤에 마지막 비동기 record 배출용 1개 synthetic drain tap을 보냈다. WGPU와 GLES 각각 30개 점수 표본 전부에서 `Choreographer` preferred VSync ID, `Transaction.setFrameTimeline`, `SurfaceView.applyTransactionToFrame`, `JankData.getVsyncId()`가 exact equality로 연결됐다. 점수 표본 중 중복·누락은 각각 0건이다.

exact match된 60개 record 모두 `presentTimeNanos == PRESENTATION_TIME_UNKNOWN`이었다. 따라서 이번 결과는 이 on-demand AVD fixture의 **frame-token/revision 연결만** 확인한다. event-to-present 지연, positive actual-present timestamp, 광학 표시 시각은 측정하지 않았다. WGPU AVD 경로는 Goldfish GFXStream/llvmpipe, GLES 경로는 ANGLE/Vulkan SwiftShader이므로 renderer 성능 비교가 아니다. API 36 fallback fixture는 `present_signal_api_unavailable`로 join을 닫고 기존 WGPU draw를 실행했다.

stale/invalid VSync token 자체 주입, surface generation 전환 중 pending `VsyncCallback`, 연속 렌더링, exception/linkage failure 주입은 아직 실행하지 않았다. 계획의 해당 경계는 미완료로 남기며 `spec/STATUS.md` R05.3/R05를 완료 처리하지 않는다. 원시 block log, API 36 fallback log와 두 renderer의 화면 캡처는 [실행 근거](../spec/internal/evidence/r05-frame-timeline-join-2026-10-08.md)에 보존했다.

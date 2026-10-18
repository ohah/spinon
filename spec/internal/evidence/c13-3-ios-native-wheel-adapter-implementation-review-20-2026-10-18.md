# C13.3 iOS native wheel adapter · 구현 적대 검토 20회

**대상:** `platforms/ios/Sources/C0410RuntimeGpuDemo.swift`의 opt-in iOS discrete/continuous wheel adapter
**검토 기준:** 내부 계약 0063, `plan/c13-3-native-wheel-input.md`, 2026-10-18 iOS Simulator 결과
**검토 방식:** 구현 경로를 서로 다른 실패 유발 관점 20개로 재검토했다. 시뮬레이터에서 실제 발생하지 않은 OS 입력·lifecycle 조건은 정적 경로 검토로만 판정한다.

| 회차 | 실패 관점과 공격 | 결과·근거 | 판정 |
|---:|---|---|---|
| 1 | 기본 앱에 adapter launch marker 또는 FFI가 남는가 | 첫 검토에서 route allowlist 문자열이 compile-time guard 밖에 있음을 발견했다. allowlist를 feature 조건부 집합으로 바꿨다. opt-in 앱에는 marker와 wheel FFI symbol이 있고, default 앱에서는 marker와 `_spinon_runtime_gpu_host_enqueue_c13_3_wheel`가 모두 없다. | 수정·artifact 검증 통과 |
| 2 | adapter와 preflight를 동시에 요청해 서로 다른 route가 동작하는가 | 두 launch argument 동시 입력은 feature-enabled 경로에서 즉시 거부한다. 별도 gesture fixture route가 없을 때도 거부한다. | 정적 통과 |
| 3 | discrete mouse가 trackpad로 또는 그 반대로 분류되는가 | 서로 다른 recognizer의 identity로 source `MouseWheel=0`, `TrackpadScroll=1`을 선택한다. iPad Simulator에서 실제 확인된 source는 continuous trackpad 하나다. | 일부 통과 |
| 4 | finger pan이 wheel recognizer에서 중복 전송되는가 | 두 recognizer 모두 `allowedTouchTypes=[]`이며 기존 pan recognizer와 분리했다. | 정적 통과; physical touch 회귀는 미검증 |
| 5 | 누적 translation 전체를 매 callback 보내 중복 scroll되는가 | callback translation에서 직전 translation을 빼서 축별 차분만 만든다. | 정적 통과; OS callback 로그와 일치 |
| 6 | `.began`이 항상 zero baseline이라 첫 실제 sample이 사라지는가 | `.began`을 입력 가능한 state에 포함했다. iPad Simulator continuous recognizer의 `.began`에서 nonzero translation을 관찰했다. | 통과 |
| 7 | `.ended` duplicate translation이 추가 scroll되는가 | 차분이 0이면 enqueue 전에 거부하고 terminal state에서 baseline을 제거한다. Simulator에서 `reason=zero-delta`를 관찰했다. | 통과 |
| 8 | cancelled/failed recognizer의 이전 translation이 다음 입력에 재사용되는가 | terminal cleanup이 모든 terminal state에서 baseline을 지운다. | 정적 통과 |
| 9 | NaN·infinite native translation이 FFI 경계에 도달하는가 | translation과 delta를 먼저 finite 검사하고 위반 시 baseline 제거 후 거부한다. | 정적 통과 |
| 10 | surface 크기/viewport scale 0, 음수, 축 불일치가 좌표를 왜곡하는가 | positive·finite X/Y scale과 최대 `0.01` 차이만 허용한다. | 정적 통과; 현재 Simulator는 scale 1.0만 관찰 |
| 11 | surface 바깥 또는 마지막 경계 점이 잘못 hit-test되는가 | half-open bounds 검사(`min <= point < max`) 뒤에만 CSS 점을 만든다. | 정적 통과 |
| 12 | UIKit delta 부호가 CSS 오른쪽/아래 양수와 반대인가 | `delta_css=-translationDelta/pointsPerCssPx`. 현재 최종 opt-in 빌드 callback에서는 `+X`와 `-Y`를 확인했다. 전체 4방향과 매 실행의 callback 전달은 안정적으로 재현되지 않았다. | 부분 통과; 4방향 재현성 미완료 |
| 13 | raw unit과 CSS delta 변환 scale이 서로 다른 단위를 가리키는가 | raw는 UIKit points, source unit은 `UIKitPoints=2`, coordinate scale은 points per CSS px로 전달한다. Simulator에서 scale 1.0 및 report metadata를 확인했다. | 통과; 다른 scale 미검증 |
| 14 | modifier bit order가 Android/Rust contract와 어긋나는가 | Shift/Control/Alt/Meta를 `1/2/4/8`로 정규화한다. | 정적 통과 |
| 15 | Control-wheel이 queue에 도달해 zoom/pinch처럼 해석되는가 | normalized Control bit가 있으면 enqueue 전에 `control-wheel`로 거부한다. Simulator Control-key input은 주입하지 않았다. | 정적 통과; runtime negative 미검증 |
| 16 | UIKit `.ended`를 momentum phase로 잘못 추정하는가 | 모든 UIKit wheel sample에 phase `None=0`을 보낸다. | 통과 |
| 17 | 새 위치에서 새 target을 찾아 stale target을 다른 scroller로 전파하는가 | OS recognizer 위치로 sample point를 계산하지만 target 고정/hit-test는 기존 Rust snapshot router가 수행한다. iPad logs show target 5/7 및 기존 router 결과. | 부분 통과 |
| 18 | resize 뒤 이전 generation의 누적 이동량이 재전송되는가 | baseline에 surface generation을 저장한다. layout replacement 때 지우지 않아 다음 callback을 새 baseline으로 저장하고 한 sample reject한 뒤 이후 incremental delta를 보낸다. | 정적 통과; resize callback runtime 미검증 |
| 19 | runtime close/readiness 경쟁에서 stale handle을 enqueue하는가 | lock 안에서 ready/closing/handle/generation/viewport를 함께 읽고 ready 및 nonzero handle을 검사한다. UIKit callback은 main-thread recognizer 경로다. | 정적 통과; 동시 close stress 미검증 |
| 20 | queue rejection에도 worker drain을 깨우거나 accepted input을 누락하는가 | enqueue report가 `status=0`과 `admitted=true`를 모두 가질 때만 drain을 예약한다. Simulator에서는 queue→process와 revision/offset 변화를 확인했다. | 통과 |

## 종합 판정

- 구현은 iOS Simulator에서 관찰한 continuous scroll 경로를 bounded Rust wheel queue와 기존 snapshot router에 연결한다.
- 이 결과는 실제 iPad/외장 trackpad, discrete mouse, Control-wheel, background/resize 복구, 모든 축의 결정적 simulator 입력을 증명하지 않는다.
- C13.3, 공개 `WheelEvent`, displayed-frame targeting, 실기기·hardware GPU·compositor 이후 결과는 미완료 상태로 유지한다.

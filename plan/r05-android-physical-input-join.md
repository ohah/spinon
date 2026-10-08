# R05.3 · Android 실기기 물리 입력과 present-fence 연결 계획

**상태:** 계획 20관점 검토 완료 · 실기기 synthetic provenance 재대조 통과 · 실제 touchscreen positive sample 대기
**상위 계획:** [입력→표시 신호 상관 계측](r05-input-to-presentation.md) · [R05 상태 대장](../spec/STATUS.md) · [비동기 fence 관찰 구현](r05-android-async-present-fence.md)

## 목적과 범위

기존 Android 16 / API 36 / SDK_INT_FULL 36.1 실기기 실행은 ADB가 주입한 synthetic `MotionEvent`였고, app log의 `input_source`도 `unknown`이다. 따라서 이전 33/33 input·submit·callback·async-fence exact join은 worker signal 경로의 근거이지 실제 touchscreen 입력의 근거가 아니다.

다음 실행에서는 Android 입력 노드의 raw touchscreen event와 app `MotionEvent` 기록을 함께 수집해 **실기기 직접 입력인지**를 먼저 판별한다. 같은 단일 탭의 `input_seq`·revision·surface generation을 R08 WGPU submit, 실제 `TransactionStats` callback, 복제 fence의 async signal까지 exact join한다. 한 번의 exploratory block에서 최대 10개 직접 입력을 확인하며, 이 표본으로 p95·제품 성능·기기간 우열을 계산하지 않는다. 지연 후보를 만들 수 있더라도 endpoint는 OS transaction present fence signal이며 panel scanout이나 photon 시각은 아니다.

호스트 code/API를 변경하거나 공개 API를 추가하지 않는다. raw 입력 provenance가 분명하지 않거나 sequence/frame join이 모호하면 실패를 보존하고 latency를 산출하지 않는다. iOS 실기기는 범위 밖이다.

## 구현 전 비교 모델

- **synthetic negative control:** `adb shell input tap` 한 건은 app에서 tap으로 처리되어야 하지만 커널 touchscreen raw event stream에는 대응 contact가 없어야 한다. AOSP [`InputShellCommand`](https://android.googlesource.com/platform/frameworks/base/%2B/a41d75968d7d5b0468cb01276d108b73823d7cb3/services/core/java/com/android/server/input/InputShellCommand.java)는 `MotionEvent`를 `InputManager.injectInputEvent()`로 주입하고 source에 맞는 device ID를 고를 수 있다. 따라서 app의 source·device ID·finger tool type만으로는 실제 손가락 입력을 증명하지 않는다. raw stream에 control contact가 나타나면 입력 출처를 분리하지 못한 것이므로 실험을 중단한다.
- **물리 입력 positive probe:** 사용자가 기기 화면의 R08 GPU target을 직접 손가락으로 탭한다. `getevent`에서 이름이 `sec_touchscreen`이고 direct-touch property를 가진 현재 입력 장치의 down/move/up contact를 보존한다. `/dev/input/eventN` 번호는 재연결마다 달라질 수 있어 장치 이름과 capability로 실행 직전에 다시 찾는다. app의 `input_source=unknown`은 출처 증거가 아니며, 이번 결과는 Android framework의 raw touchscreen device에서 시작한 contact까지만 판정한다.
- **app event metadata:** 기존 `MotionEvent` `ACTION_UP` 기록의 `eventTimeNanos`, input uptime anchor 및 `System.nanoTime()` bracket을 쓴다. 이번 실행은 기존 app log의 `input_source=unknown`을 사용하며 device/source/tool metadata가 없거나 주입 이벤트와 구분되지 않을 수 있으므로 출처 판정에 쓰지 않는다. event-time 기준은 Android [InputEvent](https://developer.android.com/reference/android/view/InputEvent)와 [MotionEvent](https://developer.android.com/reference/android/view/MotionEvent)의 uptime contract로 확인한다. 실제 touchscreen source는 raw evdev device와 사용자 입력 protocol에서 판정한다.
- **frame/presentation oracle:** 기존 `R05PresentFenceProbe`와 debug-only `R05PresentFenceWaitExperiment`의 `input_seq`, `revision`, `generation`, `target_vsync_id`, `latch_time_ns`, `signal_time_ns`, callback·wait outcome을 사용한다. [SyncFence](https://developer.android.com/reference/android/hardware/SyncFence) signal은 OS 동기화 신호이며 광학 scanout 증거로 취급하지 않는다.

## 실행 절차

1. `origin/main`의 고정 commit으로 debug APK를 빌드하거나 기존 설치본을 재사용하려면 merged PR artifact SHA-256 및 source manifest와 일치함을 확인해 기록한다. ADB device list에서 대상이 유일한 물리 기기인지 확인한 뒤 명령에는 현재 기기의 serial을 명시하되 evidence에는 serial을 저장하지 않는다. 대상은 Samsung SM-S731N / Android 16 / API 36 / SDK_INT_FULL 36.1, R08 WGPU/Vulkan `SurfaceView`다. 기기 설정과 화면 주사율은 바꾸지 않고 화면이 켜져 있으며 앱이 foreground인지 확인한다.
2. 동일 APK를 설치하고 새 process를 시작한다. 기존 experiment flag `spinon_r05_async_fence_wait=true`를 사용한다. process별 logcat을 시작하고 현재 touchscreen device 이름을 확인한 `getevent -lt <device>` 원본 수집을 준비한다.
3. 새 process의 **synthetic negative control**에서 중앙 target을 `adb shell input tap`으로 한 번 누른다. app에는 하나의 input/action이 기록되어야 하며 raw touchscreen contact가 기록되면 안 된다. 이 결과는 입력 장치 경로 분리를 검증하는 control이며 scored physical sample이 아니다.
4. control process를 종료하고 설정·foreground를 확인한 뒤 새 process를 띄워 raw touchscreen과 app log 수집을 함께 시작한다. 사용자가 중앙 GPU target을 한 번에 한 손가락으로 10회 이하 직접 탭한다. 탭 사이에는 독립 frame 처리와 fence wait가 끝나도록 충분히 기다린다. automation/ADB touch injection, 화면 원격제어, 화면 녹화 overlay 입력은 사용하지 않는다.
5. raw event별 contact ID와 down/up, app의 `ACTION_UP`, submit, callback, async signal을 sequence·revision·generation으로 대조한다. 입력마다 유일한 callback/wait가 있어야 한다. block 종료 뒤 queue `pending/active/depth=0`을 확인하고 process 종료·Chrome 복귀 후 기기 설정을 전후 비교한다.
6. 입력 `eventTimeNanos`는 uptime 기준이다. `offset_low = input_uptime_anchor - input_monotonic_after`, `offset_high = input_uptime_anchor - input_monotonic_before`로 두면 event monotonic 구간은 `[event_time - offset_high, event_time - offset_low]`이다. fence signal은 Android `SyncFence` 문서상 `CLOCK_MONOTONIC` domain이므로 후보 구간은 `[signal_time - event_monotonic_upper, signal_time - event_monotonic_lower]`로 계산한다. 변환 구간이 잘못되거나 후보 구간이 음수·모순이거나 signal이 invalid/pending/future/latch 순서 오류면 해당 latency 후보를 미계산으로 둔다. Android API 36 (SDK_INT_FULL 36.1)의 `target_vsync_id=-1`은 그대로 보존한다. 이번 10회는 request/transaction-fence exact-link feasibility만 하며 VSync·화면 frame scanout을 증명하지 않는다. 분포 요약은 n과 범위까지만 기록한다.
7. logcat, getevent, APK/source digest, 환경·window focus, 시작/결과 screenshot 및 checksum을 내부 evidence에 보존한다. app을 종료하고 Chrome을 foreground로 복귀시킨 뒤 화면 켜짐·timeout·밝기·refresh 설정이 시작 전과 같은지 확인한다. app log에 원치 않는 개인정보/다른 앱 입력이 섞이면 공개 전 제거하고 원본은 접근 제한된 작업 공간에 둔다.

## 사전 판정 기준

| 구간 | 통과 조건 | 실패·보류 처리 |
|---|---|---|
| synthetic control | app synthetic tap 1건, 대응 raw touchscreen contact 0건 | control 오염·판별 불가; physical 결과를 시작하지 않음 |
| 물리 입력 출처 | target 기기에서 직접 발생한 touchscreen contact와 app `ACTION_UP` 수·순서가 일치 | unknown/누락/추가 contact는 physical 표본이 아님 |
| app/render 연결 | 각 인정 touch의 `input_seq`와 revision이 한 R08 submit, 같은 generation의 한 actual callback에 exact join | 누락·중복·coalesced·stale 표본은 별도 실패로 유지 |
| fence 연결 | 같은 request의 유효한 복제 fence가 bounded wait 종료 전 signal되고 queue가 종료 시 drain | timeout·rejection·invalid·누락은 성공으로 보충하지 않음 |
| clock | uptime-to-monotonic bracket이 유효하고 event→signal 구간이 음수·모순이 아님 | latency 미계산; 인접 timestamp로 보간하지 않음 |
| 제품 주장 | 해당 없음 | 이 실험은 앱 체감 지연, FrameTimeline VSync, optical scanout, p95 또는 R05.3 완료를 주장하지 않음 |

## 진행 중 대조 결과 · synthetic 입력 출처

2026-10-09 Android 16 / API 36 / SDK_INT_FULL 36.1 실기기에서 merged PR #79와 동일한 debug APK를 확인했다. 기기 설치 APK SHA-256은 `608fbdf9279ddded605c7129fb0123ce32a65d7452f6ff1b36702b9eed7b198e`로 공식 산출물과 일치한다. 새 process에서 `adb shell input tap 540 1170` 한 건은 app input sequence 1 → revision 1 submit → `TransactionStats` callback → usable async fence signal로 연결됐다. 동시 수집한 `sec_touchscreen` raw event는 0줄이었다. 이 synthetic 대조는 ADB 주입과 커널 touchscreen contact를 분리할 수 있음을 확인하며 physical 표본으로 집계하지 않는다. API 36 (SDK_INT_FULL 36.1) `target_vsync_id=-1`이며 latency는 계산하지 않았다. 상세는 [control 근거](../spec/internal/evidence/r05-android-physical-input-join-2026-10-09/control/summary.md)에 있다. 직접 입력 수집을 위해 앱 foreground와 raw/logcat capture를 준비했지만 capture window 동안 touchscreen 접촉이 없어 block을 시작하지 않았다.

사용자의 실기기 재검증 요청 뒤 같은 artifact hash의 앱을 새 process로 재실행했다. `adb shell input tap 540 1170` 11회는 input·submit·실제 `TransactionStats` callback·usable async fence signal까지 모두 exact join됐고 마지막 queue는 0/0/0이었다. 11회 중 raw `sec_touchscreen` contact는 없었으므로 이번에도 synthetic control이다. 시각적으로 활성화 11회와 GPU 도형 색상 변화를 확인했다. 기기 상태를 전후 비교하고 Spinon을 종료해 Chrome foreground로 복귀했다. VSync ID는 계속 -1이며 event-to-present latency는 계산하지 않았다. [실기기 재검증 자료와 별도 20관점 증거 검토](../spec/internal/evidence/r05-android-physical-input-join-2026-10-09/physical-retest/README.md). 직접 손가락 입력 positive sample은 여전히 없다.

## 계획 적대 검토 · 독립 실패 관점 20개

계획, 기존 Android 입력 처리·R05 probe 코드, 기존 physical-fence 실행 근거 및 Android 공개 API 계약을 서로 다른 실패 경계로 검토했다. 아래 항목은 계획 검토이며 실행 결과가 아니다.

| # | 실패 관점 | 계획에 반영한 방어·판정 |
|---:|---|---|
| 1 | ADB 주입 tap이 물리 입력으로 잘못 분류 | synthetic negative control에서 app event와 raw touchscreen event를 별도 수집한다. |
| 2 | 물리 장치 event 번호가 재연결 뒤 바뀜 | `/dev/input/eventN`을 고정하지 않고 `sec_touchscreen` 이름·capability를 매 run 재탐색한다. |
| 3 | raw 장치 이름만 보고 다른 센서·touchpad를 택함 | `INPUT_PROP_DIRECT`와 event capability를 확인하고 `sec_touchpad`는 제외한다. |
| 4 | getevent 권한·수집 실패를 입력 0건으로 오인 | 수집 시작·권한·process 종료 상태를 확인하고 수집기가 준비되지 않으면 run을 시작하지 않는다. |
| 5 | getevent buffer truncation/종료 누락으로 일부 입력만 보임 | raw capture 종료 코드, 마지막 `SYN_REPORT`, action/contact 수를 기록하며 잘린 block은 무효다. |
| 6 | 앱이 Chrome/overlay 뒤라 손가락 입력이 다른 surface에 도달 | 입력 전후 window focus와 package, 앱 process 및 target layer를 확인한다. |
| 7 | status bar/navigation inset 때문에 좌표가 target 밖으로 감 | 물리 contact와 `ACTION_UP`의 app-local target 판정을 둘 다 확인한다. |
| 8 | touchscreen 외의 mouse/touchpad 또는 요청하지 않은 접촉이 표본에 섞임 | 사용자는 손가락만 사용하고, raw contact는 direct touchscreen node에서 확인한다. 기존 app의 source/tool metadata는 unknown이므로 이를 검증 근거로 쓰지 않으며 finger와 stylus를 로그만으로 분리 주장하지 않는다. |
| 9 | MotionEvent source/device ID만으로 실제 출처를 증명 | AOSP shell input은 `InputManager.injectInputEvent()`와 source별 device ID를 쓸 수 있다. API metadata는 보조 근거일 뿐이고 raw kernel touchscreen contact와 사용자 직접 입력 protocol을 함께 확인한다. |
| 10 | 한 contact의 down/up를 여러 입력으로 계수 | raw tracking/contact ID를 보존하고 하나의 action-up으로 세며 multi-pointer는 제외한다. |
| 11 | 탭 도중 연속 이동·long press·ACTION_CANCEL이 다른 동작으로 해석 | raw action sequence와 app ACTION_UP/cancel 로그를 비교하고 제외 이유를 기록한다. |
| 12 | raw tap은 있으나 app handler가 target을 받지 못함 | raw 입력 수와 accepted app input sequence의 1:1 관계를 합격 조건으로 둔다. |
| 13 | 화면의 count 변경만 보고 submit 완료로 간주 | UI 캡처는 보조 확인이며 submit/callback/fence log exact join을 요구한다. |
| 14 | sequence/revision을 기다림 순서로 추측해 잘못 연결 | 고유 key와 generation을 모두 exact match하고 단조 순서만으로 join하지 않는다. |
| 15 | 한 입력이 여러 frame으로 분할되거나 여러 입력이 coalesce | 겹치는 입력을 피하고 다중 match/coalesced revision은 physical count와 latency 성공에서 분리한다. |
| 16 | callback 뒤 stale/destroyed surface의 fence signal을 현재 표본에 연결 | callback 당시와 async 결과의 generation·current-surface 검사를 모두 확인한다. |
| 17 | fence `await()==true`를 표시 완료로 표현 | 유효한 양수 signal timestamp와 latch 순서만 OS fence 결과로 부르고 scanout 의미를 금지한다. |
| 18 | uptime과 monotonic epoch/offset 오류가 지연값을 만든다 | 각 event의 before/after monotonic bracket에서 변환 구간을 산출하고 겹침·부호 실패 시 계산하지 않는다. |
| 19 | synthetic control과 physical taps를 한 분포로 합침 | process와 입력 source를 분리하고 control은 scored 표본에서 제외한다. |
| 20 | 10개 feasibility 표본으로 p95·성능 순위·완료 선언 또는 테스트 뒤 설정/앱을 방치 | n/범위만 보고하고 stage-2 300표본·독립 block 요구를 대체하지 않으며 R05.3을 미완료로 둔다. 실행 뒤 app을 종료하고 Chrome 및 기기 설정을 복구·대조한다. |

### 계획 검토 결과

20개 실패 관점을 각각 확인했다. raw evdev positive/negative separation, 입력-source 보조 필드의 한계, exact frame join, clock interval, scanout 표현 경계를 계획에 반영했다. 이 계획의 적대 검토는 실행 후 구현·수집물 검토를 대체하지 않는다. 실행 뒤에는 실제 로그·프로세스 수명·원본 개수를 대상으로 새로운 20개 관점을 검토한다.

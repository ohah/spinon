# R05 phase-2 실기기 수집 계획 검토

이 문서의 첫 검토 표는 block 01 수집 전에 작성한 10×30 계획을 기록한다. block 01 결과 뒤에 바뀐 12×30 고정 계획은 아래 보충 검토가 적용된다.

## 검토 대상과 범위

- 계획: [`plan/r05-android-physical-sample-collection.md`](../../../plan/r05-android-physical-sample-collection.md)
- 현재 계약 기준: Android 16 / API 36 / Samsung SM-S731N / `b87a5cdd…f4db5669` 시험 APK / source commit `364bbc3105300da4a45ea8d3badd12c814cd7942`
- 검토 경계: Android 실기기 10개 block × 30회 직접 입력을 수집하는 절차. 이 검토는 새 표본이나 성능 결과가 아니다.
- 당시 기기 상태 확인: 한 대의 ADB 대상, 화면 켜짐, Spinon MainActivity foreground, portrait 1080×2340 / mode 2 60Hz, USB 전원 연결. `dumpsys input`은 `sec_touchscreen`을 DIRECT touchscreen으로 표시하고 active viewport 1080×2340, Rotation0, raw axes 0–4095, `RawToDisplay Transform` scale 약 0.2637×0.5713을 표시했다. 이 값은 검토 시점의 상태이며 과거 run의 원본 계측값으로 소급하지 않는다.

## 서로 다른 실패 관점 검토

| # | 공격 관점 | 확인한 근거와 판정 | 결과 |
|---:|---|---|---|
| 1 | 다른 APK 또는 로컬 산출물을 설치해 기존 dataset에 혼합 | plan은 기기 설치 APK와 매 block의 SHA-256을 고정하고 불일치 시 dataset을 분리한다. 사전 확인된 artifact와 pulled APK hash도 일치한다. | 통과 |
| 2 | R05 extra가 빠진 흰 화면을 block으로 세기 | 계획은 R05 제목과 GPU target 확인 전 시작을 금지한다. 실행 로그에는 `SPINON_R05_FENCE_WAIT_START`와 WGPU SurfaceView 준비 이벤트가 있어야 한다. | 통과 |
| 3 | 다중 ADB 대상에서 다른 기기의 raw/app log를 결합 | 계획은 serial을 명시하고 단일 대상 확인을 요구한다. 현재 연결 대상도 하나다. serial은 공개 evidence에 기록하지 않는다. | 통과 |
| 4 | 이전 process의 sequence, queue 또는 새 process 로그가 섞임 | block마다 fresh process를 만들고 PID·queue 종료 상태를 남기며 별도 synthetic control process를 분리한다. | 통과 |
| 5 | 기기가 잠겼거나 화면이 꺼진 상태에서 열린 빈 capture를 block으로 오인 | plan은 capture 전 foreground·화면·target을 확인하며 raw contact timeout을 둔다. 준비 확인 뒤에만 제한 시간 capture를 시작한다. | 통과 |
| 6 | `event6` 같은 과거 경로 번호를 이름 확인 없이 재사용 | plan은 매 block에 input node name, capability와 `INPUT_PROP_DIRECT`를 확인한다. 현재 `dumpsys input`도 `sec_touchscreen` 이름과 `/dev/input/event6` 연결을 보여 준다. | 통과 |
| 7 | 권한 거부·빈 출력·중간 종료·기록기 오류를 정상 0건으로 오인 | plan은 stdout/stderr, exit status, 완전한 contact·SYN_REPORT와 timeout을 확인하게 한다. 종료 조건과 빈 capture 판정을 분리했다. | 통과 |
| 8 | raw touchscreen과 다른 process의 app log를 시간으로만 잘못 묶음 | 계획은 raw/app log를 동시 수집하고 block PID·sequence를 기록한다. 다른 PID나 capture 경계 밖 event를 join하지 않는다. | 통과 |
| 9 | tracking ID 재사용·슬롯 변경·미완료 contact를 탭으로 중복 계산 | plan은 tracking/contact ID별 down/up, BTN_TOUCH, SYN_REPORT, multi-pointer 상태와 마지막 완전 group을 검사한다. | 통과 |
| 10 | raw ABS 좌표를 화면 크기로 단순 비례시켜 회전·viewport·보정을 놓침 | 이전 validator의 `/4095` 환산은 block별 InputReader transform 증거를 요구하지 않았다. 현재 `dumpsys input`은 명시적인 `RawToDisplay Transform`을 제공하고, AOSP `TouchInputMapper`는 raw offset·orientation·viewport·device mode를 합성한다. 계획에 매 block `dumpsys input` 저장과 transform 적용을 추가했다. 자료가 없으면 raw geometry를 미확인으로 남긴다. | 발견 · 수정 반영 |
| 11 | GPU 사각형 밖 탭을 target 입력이라고 분류 | pinned source의 `handleR05Touch`는 DOWN과 UP 모두 `isR05TargetPoint`를 통과한 뒤에만 accepted `SPINON_R05_INPUT`을 기록한다. plan은 이 코드와 accepted log를 대조한다. | 통과 |
| 12 | status/navigation inset 또는 SurfaceView 위치 변동을 무시 | target 판정은 SurfaceView-local 좌표를 쓰는 고정 source의 handler에서 확인한다. 독립 raw 좌표 판정 때만 display transform과 viewport를 추가로 적용하며, 회전·target이 변하면 표본을 합치지 않는다. | 통과 |
| 13 | 60/120Hz 전환이나 orientation 변경을 한 strata로 섞음 | plan은 실제 active mode와 orientation을 기록하고 달라진 block을 분리하며 refresh를 강제로 바꾸지 않는다. | 통과 |
| 14 | stylus 또는 다른 포인터를 손가락 입력으로 과장 | `INPUT_PROP_DIRECT`는 직접 touchscreen device를 뜻할 뿐 손가락을 독립 식별하지 않는다. plan은 한 손가락이라는 operator 확인만 기록하고 OS가 tool identity를 증명했다고 쓰지 않는다. | 통과 |
| 15 | callback 처리 중 다음 접촉이 들어와 queue 지연을 서로 혼합 | plan은 한 입력의 queue가 drain된 뒤 다음 탭을 요청하고 queue 상태와 각 sequence를 보존한다. 겹친 입력은 성공 분모에서 제거하지 않고 실패로 남긴다. | 통과 |
| 16 | surface 재생성 뒤 오래된 request/callback을 새 입력에 붙임 | `input_seq + revision + surface_generation + request_id`의 전 구간 유일성과 현재 generation을 요구한다. stale·중복은 accepted join이 아니다. | 통과 |
| 17 | getevent monotonic clock 설정이 실패했는데 앱 uptime과 절대시각 비교 | AOSP `getevent`는 `EVIOCSCLOCKID(CLOCK_MONOTONIC)` 실패를 비치명 경고로 출력하고 계속할 수 있다. plan에 stderr 보존·경고 거부·clock basis 확인 실패 시 interval 미계산을 추가했다. | 발견 · 수정 반영 |
| 18 | pending/invalid/late fence를 성공이나 0ms로 처리 | plan은 같은 request의 usable async fence만 accepted로 하고 callback/fence의 timeout·invalid·stale·close error를 별도로 보존한다. | 통과 |
| 19 | 느리거나 미연결된 입력을 추가 입력으로 보충해 선택 편향 유발 | block당 첫 30 raw group을 고정하고 미연결·target 밖·다중 포인터도 시도 분모에 남긴다. 결과를 본 뒤 재수집하지 않는다. | 통과 |
| 20 | 300개 접촉을 독립 표본으로 해석하거나 제품 input-to-photon p95로 홍보 | 10개 process block을 재표본 단위로 사용하되 독립성을 주장하지 않고, 300개 exact join 전에는 p95를 계산하지 않는다. frame ID/scanout 한계와 원시 로그 비공개도 남긴다. | 통과 |

## 수정에 사용한 일차 근거

- 기기 현상은 검토 시점의 `dumpsys input` 원문에서 직접 확인했다. raw 축 최댓값만으로 screen coordinate transform을 추론하지 않는 방향으로 계획을 보강했다.
- Android `TouchInputMapper`의 공식 AOSP 구현은 raw 좌표 offset·방향 회전·viewport 크기·direct/pointer mode를 결합해 `RawToDisplay`를 계산한다: [TouchInputMapper.cpp](https://android.googlesource.com/platform/frameworks/native/%2B/b08159b4fff065c4d9397fbeeff8de884a212758/services/inputflinger/reader/mapper/TouchInputMapper.cpp#L854).
- Android `getevent`는 monotonic clock 설정을 요청하지만 ioctl 실패가 비치명일 수 있다: [getevent.c](https://android.googlesource.com/platform/system/core/%2B/7664901/toolbox/getevent.c#L299).
- 앱 target 판정은 source commit `364bbc3105300da4a45ea8d3badd12c814cd7942`의 `platforms/android/app/src/main/java/dev/spinon/bootstrap/R08GpuDemo.java`, `handleR05Touch` / `isR05TargetPoint`다.

## 수집 상태

이 검토는 수집 계획만 보강했다. 현재 phase-2 유효 표본은 0/300이다. 원시 입력 기록기가 실행되지 않은 동안 사용자가 완료했다고 보고한 추가 다섯 번의 탭은 표본으로 더하지 않았다. 실기기 측정은 사용자의 준비 응답 뒤 raw/app capture를 먼저 시작하고, 한 block을 60초 제한 안에 실행해야 한다.

## Supervisor 연결 후 계획 갱신 검토

계획에 bounded collector 사용 절차와 실제 Android no-contact smoke 결과를 추가한 뒤 현재 문서-도구 계약을 다시 확인했다. 이 검토는 뒤의 코드 구현 검토와 별도다.

| # | 실패 관점 | 계획과 실행 조건 확인 | 판정 |
|---:|---|---|---|
| 1 | serial·PID·event·APK hash가 생략된 채 기본 기기를 실수로 선택 | 실행 예제는 네 값을 명시하고 `--serial`은 필수다. | 통과 |
| 2 | 연결 기기가 두 대 이상인데 지정 기기만 믿고 시작 | preflight는 unscoped `adb devices -l`에서 지정 serial만 online인지 확인한다. | 통과 |
| 3 | preflight와 stream이 서로 다른 serial을 사용 | 모든 대상 명령은 동일한 `-s`와 명시 serial을 사용한다. | 통과 |
| 4 | 여러 PID 또는 package mismatch 로그 결합 | `pidof` 결과가 지정된 앱 PID 하나와 정확히 일치해야 한다. | 통과 |
| 5 | 오래된 앱 process에서 이전 revision/queue 사용 | process start tick과 uptime에서 age를 계산해 기본 300초 초과를 거부한다. phase-2는 새 process를 별도 요구한다. | 통과 |
| 6 | 설치 APK와 실행 근거 APK가 다름 | 단일 설치 base.apk SHA-256을 매 실행에서 지정 hash와 대조한다. | 통과 |
| 7 | `/dev/input/event6`과 비슷한 `/dev/input/event60` node 혼동 | getevent listing의 header path를 정확 비교한다. | 수정 후 통과 |
| 8 | touchpad나 proximity를 direct touchscreen으로 선택 | node 이름·`INPUT_PROP_DIRECT`·`ABS_MT_TRACKING_ID`를 같은 node에서 검사한다. | 통과 |
| 9 | R05 flag가 없는 흰 bootstrap 화면 또는 다른 foreground | resumed activity, PID별 `SPINON_R05_FENCE_WAIT_START`, 화면 operator 확인을 모두 요구한다. | 통과 |
| 10 | 잘린/빈 screenshot이 정상 화면 확인으로 취급 | PNG signature, IHDR size, IEND를 preflight에서 검사하고 실제 operator가 R05 target을 확인한다. | 통과 |
| 11 | 수집 전에 남아 있던 기기 `getevent`를 현재 run의 것으로 잘못 세기 | preflight에서 이미 실행 중인 getevent를 검사하고 발견 시 시작을 거부한다. | 통과 |
| 12 | orientation·refresh·thermal·battery·viewport 자료가 없는데 기본값으로 보충 | 원문을 저장하고 required input/foreground 검사 실패는 시작을 막는다. 해석 불가한 값은 추정하지 않는다. | 통과 |
| 13 | raw stream 준비 전에 GO를 출력해 첫 탭을 놓침 | raw getevent와 PID logcat을 준비한 뒤 readiness를 확인하고 GO를 쓴다. | 통과 |
| 14 | host wall clock 조정으로 60초 deadline이 늘거나 줄어듦 | deadline은 `performance.now()` 단조 시계를 쓰며 GO 직전 기기 uptime을 보존한다. | 통과 |
| 15 | 2초 smoke override를 scored 60초 계획으로 오해 | 2초 smoke는 별도 no-contact collector 검증이고 표본 0으로 둔다. scored 실행은 기본 60초 window와 30 contact 설정이다. | 통과 |
| 16 | down/up만 있고 마지막 `SYN_REPORT`가 빠진 group을 count | complete contact는 release와 SYN_REPORT까지 확인하고 pending frame은 성공 gate를 닫는다. | 통과 |
| 17 | 30개 경계 뒤 이어진 입력을 first-30 scoring set에 넣음 | plan은 raw 시간순 첫 30개 group을 scoring set으로 고정하고 추가 raw는 별도 보존한다. | 통과 |
| 18 | 조기 완료/취소를 기다린 뒤 다음 입력으로 채움 | operator stop 즉시 닫으며 30 미만이면 incomplete로 남기고 보충하지 않는다. | 통과 |
| 19 | ADB disconnect·SIGINT 무시·PID 재사용·원격 getevent 잔존 | exact child identity, bounded SIGINT→TERM→KILL, remote 잔존 검사와 timeout 실패 분류를 적용한다. 실제 no-contact run에서도 잔존 0을 확인했다. | 통과 |
| 20 | raw capture gate를 app join 또는 latency p95 완료로 오해 | manifest는 `not_analyzed_join_required`를 유지하며 exact input/ACTION_UP/submit/callback/fence join 전 p95를 계산하지 않는다. | 통과 |

최초 검토 시점에는 계획의 목표 수집이 아직 수행되지 않았다. 실제 60초 no-contact attempt는 실행 종료와 수집 도구를 검증했을 뿐 scored block을 만들지 않았다. 이후 block 01은 아래 별도 분석에 추가됐다.

## block 01 결과 이후 표본 수 계획 재검토

첫 phase-2 block에서 고정된 30 raw contact 중 exact candidate가 28개였다. 원래 10×30 시도 설계에서는 남은 9개 block이 전부 성공해도 최대 298개여서 300개 candidate 기준을 만족할 수 없었다. 시각 구간 값은 표본 수 계획 변경의 기준으로 쓰지 않았다. block 02를 시작하기 전에 12×30 시도, 고정 360회 상한으로 계획을 고쳤다.

| # | 공격 관점 | 판정 |
|---:|---|---|
| 1 | 이미 수집한 28개와 남은 270회로 300개 기준에 도달한다고 계산 | 최대 298이라는 산술적 한계를 확인하고 12 block으로 수정했다. |
| 2 | feasibility 20건을 새 후보에 더해 부족분을 메움 | 기존 20건은 별도 집합으로 유지한다. |
| 3 | 새 결과를 본 뒤 다음 block 개수를 다시 조정 | block 02 입력 전에 총 12개를 고정했다. 이후 표본 수는 변경하지 않는다. |
| 4 | 블록마다 30회보다 더 받아 실패를 숨김 | 각 block의 scoring set은 첫 30 raw contact이며 30번째 뒤 입력은 drain으로 제외한다. |
| 5 | 총 시도 상한을 알 수 없게 둬 유리한 시점까지 수집 | 총 12×30=360 raw attempt 상한을 명시했다. |
| 6 | 300 candidate에 먼저 도달하면 좋은 block만 골라 조기 종료 | 12개 block을 모두 수집한 뒤에만 최종 gate를 평가한다. |
| 7 | 미연결·취소·다중 포인터를 분모에서 제거 | 모든 first-30 raw attempt와 실패를 block별로 보고한다. |
| 8 | 실패 이후 같은 block에서 입력을 더해 빈 곳을 채움 | block 안에서 보충 입력을 요청하지 않는다. |
| 9 | 부족한 raw contact block을 정상 30회인 것처럼 처리 | 30개에 못 미친 block을 불완전으로 남기고 실패 근거를 기록한다. |
| 10 | exact candidate가 300개보다 적어도 통계 gate를 낮춤 | 300개 미만이면 p95/bootstrap을 계산하지 않는 기준을 유지한다. |
| 11 | 300개를 넘긴 candidate 중 느린 값이나 뒤 block을 제거 | 300개 이상이면 모든 exact candidate를 분석에 포함한다. |
| 12 | 시각 구간이 좋거나 나쁜 것을 보고 표본을 추가/삭제 | 계획 수정은 exact join 개수만으로 했고 candidate 시각 구간은 선택 기준으로 사용하지 않았다. |
| 13 | 앱의 timestamp 없는 `cancelled`·`outside_target` 행을 특정 raw touch 실패로 붙임 | 식별자 부족으로 두 로그 집합을 따로 보고하며 일대일 원인 연결을 금지했다. |
| 14 | drain 이후 raw 접촉을 점수 set에 이동 | 고정 first-30 경계를 모든 block에서 유지한다. |
| 15 | 다른 OS·기기·refresh strata 결과로 후보 부족을 메움 | 동일 strata 밖 결과는 계속 분리한다. |
| 16 | 12개의 process를 통계적 독립 12개로 단정 | block bootstrap은 탐색적 불확실성이고 동일 기기·날짜 독립성을 보장하지 않는다고 명시한다. |
| 17 | 늦은 유효 fence 값을 outlier로 제외 | latency에 따른 후보 제거 규칙을 두지 않는다. |
| 18 | block 간 열 안정화나 대기 절차를 표본 수 변경과 함께 느슨하게 함 | 기존 preflight·최소 대기·thermal strata 조건은 그대로 유지한다. |
| 19 | 정확한 raw 시도 360과 후보 300의 차이를 숨김 | 360 attempts·exact candidate 개수·failure rate를 따로 보고하고 300은 후보 분석 gate로 표시한다. |
| 20 | 웹 미리보기 상태가 저장소 계획과 다르게 남거나 Pages로 공개 | plan·STATUS·Tailscale 작업 미리보기를 같은 수치로 맞추고 GitHub Pages는 배포하지 않는다. |

반영 위치는 [실기기 수집 계획](../../../plan/r05-android-physical-sample-collection.md), [상태 대장](../STATUS.md), Tailscale용 block 결과 미리보기다. 다음 block 결과는 이 개정 검토의 판단 근거로 섞지 않는다.

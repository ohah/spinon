# R05.3 · Android 실기기 직접 입력 표본 수집 계획

**상태:** 실행 진행 중 · phase-2 exact candidate 28/300 · 총 12개 block × 30회로 사전 고정 · 기존 직접 touch 20건은 feasibility 근거로만 유지

**상위 계획:** [입력→표시 신호 상관 계측](r05-input-to-presentation.md) · [Android 직접 입력 연결](r05-android-physical-input-join.md) · [R05 상태 대장](../spec/STATUS.md)

## 목적과 주장 경계

기존 run-02–run-05의 직접 touch 20건은 raw touchscreen→ACTION_UP→WGPU submit→현재 SurfaceView transaction callback→usable async fence wait 경로의 feasibility를 확인했다. 네 실행은 각 5회였고 독립적인 표본 block으로 설계되지 않았다. 이 20건으로 p95, 제품 입력 지연, 기기간 성능 우위 또는 input-to-photon을 계산하지 않는다.

이 계획은 Android 16 / API 36 실기기에서 기존 `TransactionStats` fence 후보를 반복 수집하는 내부 진단 절차를 고정한다. block 01에서 30 raw 시도 중 28건만 exact join돼, 10×30 계획으로는 300개 후보 목표에 도달할 수 없음을 확인했다. 이후 결과를 보기 전에 총 12개 block × 30회 시도로 계획을 고정한다. 정확한 `target_vsync_id`와 SurfaceView frame ID가 없는 후보값을 제품의 event-to-display latency로 부르지 않는다. `SyncFence` 신호는 OS transaction-present 후보 endpoint이며 실제 panel scanout·광자 시각을 뜻하지 않는다. iOS 실기기, 공개 API, 제품 renderer 동작 변경은 범위에 포함하지 않는다.

## 사전 고정 조건

- 기준 기기는 Samsung SM-S731N / Android 16 / API 36 / `SDK_INT_FULL=36.1`, WGPU/Vulkan/Xclipse 940이다. 다른 기기·OS·renderer 결과와 합산하지 않는다.
- APK SHA-256과 실험 extra `spinon_r05_async_fence_wait=true`를 매 block에서 확인한다. 현재 APK는 `b87a5cdd1d3589a5ffea66b784d9bf40f64843b69ad4aeb876086412f4db5669`, source commit은 `364bbc3105300da4a45ea8d3badd12c814cd7942`다. APK/source가 현재 근거와 다르면 새 dataset으로 분리한다.
- 기존 run-02–run-05의 20개 후보값은 feasibility dataset이다. phase-2 p95 dataset에 합산하지 않는다.
- 첫 phase-2 block에서 관찰한 orientation과 실제 active refresh mode를 고정 strata로 삼는다. 새 process 시작 뒤 현재 확인된 첫 block 조건은 portrait / 60Hz다. orientation 또는 active mode가 달라지면 같은 통계 집합에 합치지 않는다. 화면 회전·refresh mode를 강제로 바꾸지 않는다.
- 각 block은 새 앱 process 하나에서 수집한다. process 시작 뒤 화면·GPU target이 정상임을 확인하고 5초 안정화한다. 이 기간의 입력은 사용하지 않는다. 사용자는 각 block 전후에 한 손가락 직접 입력을 했다고 확인한다. OS 원본만으로 손가락과 stylus를 독립 구분했다고 주장하지 않는다.
- raw `sec_touchscreen` direct device, process별 app log, screen capture, device/host monotonic anchors를 동시에 수집한다. 캡처 시작 전에 APK hash, PID, foreground, orientation, active mode, thermal status, battery/plugged state를 기록한다. 각 값은 `dumpsys display`, `dumpsys thermalservice`, `dumpsys battery` 원문에서 얻고 읽을 수 없으면 추정하지 않고 unavailable로 남긴다.
- 각 block에서 `dumpsys input` 원문을 보존해 `sec_touchscreen`의 Touch Input Mapper 모드, associated display/viewport, orientation, raw axes와 `RawToDisplay Transform`을 기록한다. raw 좌표를 화면 좌표로 볼 때 저장한 변환을 적용하며 raw axis 최대값과 화면 크기만으로 선형 변환을 가정하지 않는다. 변환을 읽지 못하면 raw 좌표는 보존하되 기하학적 target 위치를 미확인으로 남긴다. target 성공은 고정 source의 R05 handler가 ACTION_DOWN과 ACTION_UP 모두에서 target을 확인하고 accepted `SPINON_R05_INPUT`을 남긴 사실로 판정한다.
- phase-2 첫 block 전에 별도 control process에서 ADB synthetic tap 1회를 실행한다. app event 1개와 raw `sec_touchscreen` contact 0개를 확인해야 한다. control process와 그 로그는 physical block에서 분리하고, 확인 뒤 fresh process로 block 1을 시작한다. 이 control은 scored 표본이 아니다.
- `GO` 후 60초 안에 첫 raw contact가 없으면 capture를 닫고 `prestart_no_contact`로 남긴다. 이는 표본 block이 아니며 뒤늦은 입력을 붙이지 않는다. 첫 contact 뒤 60초 안에 30개 group이 들어오지 않으면 불완전 block으로 닫는다. 사용자가 완료 또는 중단을 알리면 관측된 횟수와 무관하게 즉시 capture를 닫는다. 30개 미만이면 불완전 block이며 뒤늦게 더 입력하거나 보충하지 않는다. `getevent`가 operator stop으로 끝난 status는 마지막 완전한 contact group·`SYN_REPORT`·contact count가 온전한 경우에만 정상 종료로 취급한다. USB/ADB disconnect, collector crash, raw 파일 절단·count 모순은 품질 실패이며 종료 status만으로 완전성을 판정하지 않는다.
- 수집 supervisor는 시작한 host-side `adb logcat`·`adb shell getevent` client마다 PID, 실행 명령, parent PID, 시작 시각을 기록하고 이 identity가 일치하는 동안만 종료 신호를 보낸다. 모든 종료 경로에서 SIGINT→2초 bounded wait→SIGTERM→2초 bounded wait→SIGKILL→2초 bounded wait 순서로 정리한다. identity가 바뀌었거나 확인할 수 없으면 PID를 재사용해 신호를 보내지 않고 collector 실패로 분류한다. 전체 ADB server나 해당 run이 만들지 않은 ADB client는 종료하지 않는다. 각 client의 stdout/stderr는 별도 파일로 직접 보낸다. 최종 `wait`와 파일 flush/close 뒤 파일 크기·마지막 완전 event·checksum을 확정한다. host client가 없고 기기의 `getevent`도 2초 안에 종료됐는지 별도로 확인하며 기기 확인 불가도 성공으로 간주하지 않는다. 남은 process, 제한 시간 초과, 예상 밖 종료코드, 미기록 마지막 raw event 중 하나라도 있으면 block을 무효화한다. 강제 신호의 종료코드는 정상 수집 완료로 취급하지 않는다.

## 표본 단위와 실행

### 수집기 준비와 사용

수집 supervisor는 `tools/benchmark/capture-android-physical-touch.mjs`에 있다. 앱을 새 process로 띄운 뒤 R05 화면이 보이는지 확인하고 5초 안정화한다. `adb devices -l`, `pidof`, `getevent -lp`, 설치 APK의 SHA-256을 확인한 다음 `sec_touchscreen`의 실제 event 경로와 fresh PID를 지정한다. 대상은 Samsung 단일 Android 기기로 제한하며 serial은 저장소 증거와 외부 미리보기에 복사하지 않는다.

```sh
mise exec -- bun run benchmark:r05:physical-touch:android -- \
  --serial <ADB serial> \
  --pid <fresh app PID> \
  --event /dev/input/eventN \
  --apk-sha256 <설치된 base.apk의 SHA-256> \
  --out /tmp/spinon-r05-block-01-<고유 이름> \
  --confirm-r05-screen
```

`GO`가 출력된 다음에만 손가락 입력을 시작한다. 기본 수집기는 첫 raw contact까지 60초, 첫 contact 이후 목표 수까지 60초를 기다리고 완결된 contact group 30개와 2초 drain 뒤 자동으로 닫는다. 30개 전에 멈추거나 operator가 `Ctrl-C`를 누르면 incomplete로 남기며 이후 입력으로 채우지 않는다. 출력 디렉터리는 새 경로여야 한다. 수집 결과는 `capture-manifest.json`, `raw-touch.log`, PID 범위 `app.log`, 시작·종료 화면, preflight metadata, stdout/stderr, `SHA256SUMS`를 만든다. `captureWindowValid=true`는 raw capture와 종료 검증만 통과했다는 뜻이다. 정확한 raw→`ACTION_UP`→submit→callback→fence join이 끝나기 전에는 latency 결과나 유효 표본으로 세지 않는다. 수집 후 `mise exec -- bun run analyze:r05:physical-touch-android -- <capture-directory>`를 실행해 `join-summary.json`을 만든다.

앞선 새-process no-contact 시도는 `prestart_no_contact`로 무효였고 이전 수동 종료는 cutoff보다 67초 늦었다. 새 collector는 가짜 ADB 통합 검증에 이어 Android 16 실기기에서 2초 무접촉 smoke와 fresh R05 process의 60초 no-contact 시도를 통과했다. 두 run에서 자동 timeout이 작동하고 host getevent/logcat client가 SIGINT 뒤 종료됐으며 device `getevent` 잔존은 0개였다. [2초 smoke](../spec/internal/evidence/r05-android-direct-touch-exploration-2026-10-09/collector-device-smoke/README.md) · [60초 시도](../spec/internal/evidence/r05-android-direct-touch-exploration-2026-10-09/phase2-block01-supervised-no-contact/README.md).

2026-10-09 13:09 KST 새 R05 process의 bounded capture는 raw contact 32개를 보존했다. 사전 고정 규칙상 down timestamp와 원본 행 순서로 정한 첫 30개 중 28개가 raw release→`ACTION_UP`→WGPU submit→현재 generation callback→usable fence와 clock bracket까지 exact join됐다. 점수 집합의 나머지 raw 두 건에는 exact `ACTION_UP` 대응이 없었다. 앱 로그에는 별도로 `cancelled`·`outside_target` 제외 이벤트가 하나씩 있으나 해당 행에 event timestamp가 없어 미연결 raw 두 건과 일대일 대응한다고 단정하지 않는다. drain의 raw 추가 두 건은 점수에서 제외됐고 이들과 일치하는 accepted `ACTION_UP` 두 건도 분석기에서 따로 집계한다. 28개 candidate interval의 block envelope는 28.230–61.515ms다. 단일 block이므로 p95는 계산하지 않는다. `target_vsync_id=-1`로 frame/scanout을 연결하지 못하며 제품 latency를 주장하지 않는다. 현재 phase-2 후보는 **28/300**이다. [실행·분석 evidence](../spec/internal/evidence/r05-android-direct-touch-exploration-2026-10-09/phase2-block01-direct-touch/README.md).

1. **Block 정의:** 한 block은 동일 APK·기기·OS·orientation·active refresh mode·renderer/backend를 유지하는 단일 fresh app process의 capture 구간이다. 30개 시도는 block 안에 묶어 시간상 의존성을 보존한다. process 재시작은 환경 독립성을 증명하지 않으므로 결과에서 통계적 독립을 단정하지 않는다.
2. **입력:** 사용자가 중앙 GPU 사각형을 한 손가락으로 직접 누른다. 한 번에 한 접촉만 하고 가능한 한 초당 1회 정도 간격을 둔다. ADB/simulated injection·원격 touch·다른 포인터는 허용하지 않는다.
3. **블록당 목표:** operator가 raw capture 시작 뒤 `GO` 시각을 기록하고 사용자가 direct touch 30회를 수행한다. 총 12 block·360회 시도를 수집한다. 각 scoring set은 `GO` 뒤 tracking ID가 시작된 시각순 첫 30개 contact로 고정하고, 같은 timestamp이면 `getevent` 원본 행 순서를 유지한다. target 밖·불완전·다중 pointer·unjoined 접촉도 시도 분모에서 빼지 않는다. 30번째 contact 뒤 추가 접촉이 있으면 원본에 보존하되 scoring set에는 넣지 않는다. 실패나 미연결을 보충하려고 block 안에서 추가 touch를 요청하지 않는다. 30개 contact start를 확보하지 못하면 부족 block으로 남기고 p95 후보 수에 넣지 않는다. 느린 값이나 결과를 확인한 뒤 block을 선택적으로 다시 채우지 않는다. 12개 block은 다음 결과를 보기 전에 고정하며 중간 결과에 따라 종료·추가·교체하지 않는다.
4. **분리:** block 사이 앱 process를 종료하고 새 process로 시작한다. 각 block 시작 전 같은 preflight·5초 안정화를 적용한다. block 사이 최소 60초 쉬고 thermal status가 바뀌었으면 원래 strata로 돌아올 때까지 대기하거나 다른 strata로 분리한다. 블록 사이 대기·열·전원·배터리 상태를 기록하되 기기 설정을 임의 변경하지 않는다.
5. **Join:** tracking/contact ID와 slot으로 raw down/up을 묶고, raw release timestamp와 app `ACTION_UP`을 비교한다. accepted `input_seq + revision + surface_generation + request_id`가 WGPU submit, 동일 generation의 실제 callback, 같은 request의 usable fence에 각각 유일하게 연결되어야 한다. 한 입력의 queue가 drain된 뒤 다음 탭을 하도록 요청한다. `GO` 이후 첫 30개 raw contact를 start timestamp와 같은 timestamp의 원본 행 순서로 선정한다. 도착 순서만으로 앱 frame을 join하지 않는다. target 판정은 capture 대상 source commit의 `handleR05Touch`/`isR05TargetPoint` 계약과 accepted 로그를 대조한다. raw 좌표의 독립적인 화면 위치 판정은 같은 block의 `RawToDisplay Transform`과 viewport를 사용한다.
6. **Clock:** 기존 uptime→monotonic before/after bracket 산식을 그대로 사용한다. `getevent` 실행 때 stderr와 시작 진단을 보존하고 `EVIOCSCLOCKID`의 monotonic 설정 실패 경고가 없어야 한다. 이 ioctl 실패는 getevent에서 비치명적일 수 있으므로 경고가 있거나 timestamp clock basis를 확인할 수 없으면 절대시각 join과 후보 interval을 계산하지 않는다. bracket이 유효하지 않거나 event/fence 순서가 모순이면 해당 후보는 계산 불가로 보존한다. getevent timestamp의 microsecond 표현을 nanosecond 정확도로 표시하지 않는다.
7. **분석 gate:** 동일 strata에서 사전 고정한 12개 block, block당 의도된 30회 touch를 모두 수집하고 block별 360회 시도·성공·미연결·실패를 보고한다. 전체 block을 닫은 뒤 exact-joined 유효 후보가 300개 미만이면 p95와 bootstrap 구간을 계산하지 않는다. 300개 이상이면 느린 값이나 block을 제거하지 않고 모든 exact-joined 후보의 **transaction-fence 후보 p95**와 block 단위 재표본화 95% 구간을 탐색값으로 계산한다. bracket interval의 하한·상한을 각각 집계하고 midpoint로 축약하지 않는다. block bootstrap은 같은 기기·같은 날짜의 완전한 독립성 증거가 아니므로 탐색적 불확실성으로만 해석한다. p95가 계산되어도 제품 입력 latency, frame-accurate p95, scanout 또는 성능 우위라고 부르지 않는다.
8. **보존과 복구:** 원시 getevent/app log, 제외된 접촉, 시작·종료 화면, APK/source digest, block metadata와 checksum을 내부 evidence에 보존한다. 한 block의 process crash, log 누락, raw capture truncation, foreground 상실은 block 무효 사유로 기록한다. 측정 후 앱·화면 설정·foreground를 변경했다면 원래 상태를 확인해 복구한다. 외부 미리보기에는 aggregate와 무민감 화면 증거만 싣고 원시 입력 로그·ADB serial은 싣지 않는다.

## block 무효와 표본 처리

| 관측 | 처리 |
|---|---|
| 흰 화면, R05 flag 누락, 잘못된 APK, 기기 다중 연결 | 입력 전 중단하고 block을 시작하지 않는다. |
| 입력 출처가 raw direct touchscreen으로 확인되지 않음 | 물리 입력 성공으로 세지 않고 raw/app 기록을 보존한다. |
| 다중 포인터, 접촉 overlap, `ACTION_CANCEL`, target 밖 접촉 | 성공 표본에서 제외하고 실패 분모에 유지한다. |
| callback/fence missing·late·invalid·pending·stale·중복 | 성공 표본으로 대체하지 않고 해당 outcome을 기록한다. |
| invalid clock bracket, timestamp order 오류 | latency 후보는 미계산으로 남기고 다른 시각으로 보간하지 않는다. |
| 느린 유효 값·deadline miss | outlier라는 이유로 제거하지 않는다. |
| orientation/refresh mode 전환, process 재시작, 필수 로그 손실 | block을 무효 처리하고 이유와 원본을 보존한다. |
| 60초 안에 완결된 raw contact 30개 미달 | 부족 block으로 남긴다. 제한 시간 뒤 입력이나 별도 입력으로 시도 수를 보충하지 않는다. |

## 판정 결과물

- 12개 block별 입력 시도·raw contact·ACTION_UP·submit·callback·fence의 exact join 및 실패 표. 분석기 결과는 `join-summary.json`으로 보존한다.
- orientation·active refresh·thermal/battery·APK hash·process lifecycle이 포함된 block manifest.
- exact 후보가 300개 미만이면 개별 interval·개수·실패율만 보고한다. 300개 이상이어도 모든 후보의 transaction-fence p95와 block-bootstrap 구간만 보고하며 제품 성능 결론을 내리지 않는다.
- block manifest에는 `dumpsys input`의 viewport·orientation·raw axes·`RawToDisplay Transform`과 `getevent` 실행 진단/clock 설정 경고 여부를 포함한다.
- `target_vsync_id=-1`과 frame ID 부재를 유지한다. Android의 exact frame correlation, iOS device callback runtime, scanout은 별도 미완료 gate다.
- 계획·실행 후 결과를 `spec/STATUS.md`와 Tailscale 미리보기에 동기화한다. 이 계획 실행은 R05.3을 완료로 바꾸지 않는다.

## 계획 적대 검토 · 서로 다른 실패 관점 20개

| # | 실패 관점 | 계획에서 고정한 방어 |
|---:|---|---|
| 1 | 다른 APK나 stale APK를 같은 dataset에 포함 | 매 block APK hash와 experiment flag를 확인하고 불일치면 dataset을 분리한다. |
| 2 | R05 extra가 빠져 흰 화면 입력을 정상 표본 처리 | 화면의 R05 제목·GPU target을 확인하기 전에는 block을 시작하지 않는다. |
| 3 | 여러 ADB 기기 중 다른 기기에 입력·로그를 수집 | 대상 serial을 명시하고 단일 대상 여부를 매 block 검사한다. |
| 4 | 기존 process의 sequence나 queue 상태가 새 block에 섞이거나 앱이 중간 재시작 | block마다 fresh process를 사용하고 시작/종료 PID·crash marker·queue 초기 상태를 기록한다. |
| 5 | ADB injected tap이 raw touchscreen positive로 집계 | 물리 block에는 injection을 금지하고 raw direct contact과 ACTION_UP join을 요구한다. |
| 6 | `/dev/input/eventN` 번호가 바뀌거나 다른 input node를 읽음 | 이름·`INPUT_PROP_DIRECT` capability를 block 시작 때 확인한다. |
| 7 | raw log truncation/권한 오류를 0건으로 처리하거나 빈 capture를 무기한 열어 두거나 종료 신호를 무시한 ADB 자식이 남음 | collector readiness·마지막 완전 contact group/`SYN_REPORT`·raw/app count와 60초 prestart/in-block timeout을 확인한다. PID·명령·parent·시작 시각이 일치하는 host client에만 SIGINT→SIGTERM→SIGKILL과 단계별 2초 wait를 적용하고 각 stdout/stderr 파일 flush/close 뒤 크기·마지막 event·checksum을 확정한다. host client와 device `getevent`가 모두 사라졌는지 확인하며, 강제 종료는 정상 수집 완료로 취급하지 않는다. |
| 8 | 여러 손가락의 contact ID를 한 탭으로 또는 여러 탭으로 중복 계산 | tracking ID별 down/up 수명과 동시 contact를 보존하고 단일 pointer만 인정한다. |
| 9 | status/navigation inset, InputReader affine/rotation 또는 raw axis 범위 차이로 target 밖 좌표를 안쪽으로 오판 | `dumpsys input`의 viewport·orientation·`RawToDisplay Transform`을 block마다 보존해 적용한다. 변환 자료가 없으면 raw 좌표의 target 위치는 미확인으로 두고, app R05 handler의 down/up target 검사와 accepted 로그만 target 판정 근거로 쓴다. |
| 10 | 화면 잠금·알림·IME·다른 window가 입력을 가로챔 | foreground package/window와 화면 상태를 시작·종료 시 확인한다. |
| 11 | orientation·refresh-rate 변화가 동일 분포에 섞임 | orientation과 active refresh mode를 고정 strata로 기록하며 다른 strata는 합산하지 않는다. |
| 12 | 두 입력의 callback이 겹쳐 sequence를 잘못 연결 | 단일 손가락 입력 간격을 확보하고 input sequence와 generation의 유일성을 확인한다. |
| 13 | 앱 input count나 화면 색상만으로 GPU present를 추정 | UI는 보조 증거로만 두고 submit·실제 transaction callback·usable fence의 exact join을 요구한다. |
| 14 | 오래된 surface의 callback/fence가 현재 generation으로 연결 | request key와 `surface_generation`을 전 구간 비교하며 stale 값은 제외한다. |
| 15 | pending·invalid·late fence를 0ms 성공으로 집계 | 유효한 같은 request fence만 후보 성공으로 분류하고 실패 outcome을 보존한다. |
| 16 | uptime과 monotonic 차이를 한 offset으로 고정하거나 getevent clock 설정 실패를 놓침 | 양쪽 clock anchor bracket과 getevent stderr를 보존한다. monotonic 설정 실패 경고 또는 invalid/모순 bracket이 있으면 후보를 계산하지 않는다. |
| 17 | microsecond source timestamp를 nanosecond 정확도로 과장 | source 해상도를 기록하고 정밀도와 정확도를 별도 표현한다. |
| 18 | 실패를 사후에 추가 입력으로 보충해 선택 편향을 만듦 | block마다 의도된 touch 30회에서 capture를 끝내고 실패·미연결을 보충하지 않는다. |
| 19 | 360개 raw 시도를 360개의 독립 관측으로 간주하거나 결과를 확인하고 block 수를 조절 | 12개 사전 고정 capture block으로 묶고 전부 수집한다. block bootstrap을 쓰되 같은 기기·날짜의 독립성을 주장하지 않는다. |
| 20 | candidate fence p95를 제품 input-to-photon 수치로 홍보하거나 원시 로그를 공개 | 결과 이름을 transaction-fence 후보로 제한하고 frame ID·scanout 미완료 및 원시 자료 비공개를 명시한다. |

재검토에서 발견한 raw-to-display 가정과 getevent clock 설정 진단을 위 수집 계약과 실패 방어에 반영했다. 세부 판정은 [R05 phase-2 계획 적대 검토](../spec/internal/evidence/r05-android-physical-sample-collection-plan-review-2026-10-09.md)에 기록했다. 실제 무접촉 timeout 실행에서 background host ADB client가 SIGINT를 무시했고 수동 SIGTERM 뒤에야 종료됐다. 이 실패를 별도의 [종료 절차 재검토](../spec/internal/evidence/r05-android-physical-sample-shutdown-2026-10-09/README.md)에서 분석해 process identity를 확인하는 exact PID 추적, bounded SIGINT→SIGTERM→SIGKILL escalation, 단계별 wait, host/device 잔존 검사, 사용자 조기 완료 보고 시 즉시 닫고 미완료로 분류하는 조건을 계약에 추가했다.

### 검토에서 반영한 수정

기존 20개 run-02–run-05 touch를 phase-2 표본에 합치지 않고 별도 feasibility 자료로 분리했다. 검토 중 30개 유효 표본까지 추가 입력을 허용하면 결과 의존 stop/selection이 생길 수 있어 raw `GO` 뒤 첫 30개 contact group으로 고정했다. 계획을 실제 장치에서 준비하던 중 4분 넘는 무접촉 capture가 열린 채 남을 수 있음을 확인해 60초 prestart/in-block timeout을 추가했다. 새 APK/source provenance, 기기·refresh·orientation/thermal 섞임, control contamination, clock precision, process crash·collector의 의도된 종료와 오류, bootstrap 단위를 분리했다. 재실행 중 landscape 화면이 종료·재시작 뒤 portrait로 돌아와, 계획의 초기 landscape 가정을 실제 preflight 결과인 portrait / 60Hz로 고쳤다. 첫 실행 결과 exact candidate가 28개라 원래 10×30 설계로 300개 후보에 도달할 수 없는 수학적 불일치를 확인했다. latency 값이 아니라 성공 후보 수만 보고 다음 실행 전에 총 12×30으로 사전 고정했다. 이후에는 전 block을 완료하고 전체 후보를 보존하며, 실패를 채우거나 block을 사후 선택하지 않는다. 이 계획 검토는 이후 각 실행의 원본·구현 후 실패 검토를 대체하지 않는다.

# R05 · Android 실기기 callback 실패 fixture 계획

**상태:** 실행 완료 · 두 fixture run 통과 · 첫 수집기 시작 race 수정 · usable present fence는 미확인

**목표:** 사용자가 연결한 Android 실기기에서 R05 callback 실패 주입과 R08 GLES `SurfaceView` 실제 applied-transaction 수명주기를 확인한다.

**범위:** 단일 물리 Android 기기, debug APK, synthetic fixture 요청이다. 실제 손가락 입력·성능 순위·물리 화면 scanout·광자 지연은 검증하지 않는다. 제품 코드와 일반 emulator-only runner는 변경하지 않는다.

## 비교 모델

- **기준 동작:** [기존 emulator runner](../tools/verify-r05-android-callback-faults.sh)의 고정 fixture가 기대 동작 oracle이다. API 37.0 AVD에서 통과한 12개 check, 100회 timeout/callback 경합, 60초 idle, 실제 baseline/stale/recovery callback, 요청 revision별 GLES draw를 비교한다. 이 runner는 `emulator-*`와 API 35/37.x AVD만 허용하므로 실기기 serial을 해당 인자에 넣지 않는다. 아래 물리 기기 실행은 같은 APK/intent/marker/assertion을 쓰는 별도 driver이며 emulator runner 결과와 분리한다.
- **실기기:** preflight에서 USB ADB로 연결된 Samsung `SM_S731N`; Android 16, SDK 36, `SDK_INT_FULL=36.1`, ARM64, guest page size 4096을 읽었다. OS fingerprint와 모델은 결과에 기록하고 ADB serial은 공개 증거에서 제거한다.
- **코드/의존성:** 새 작업 worktree의 앱 소스는 `origin/main`과 같고 차이는 이 계획/evidence 문서뿐이다. pinned V8 revision은 `tools/v8/v8-revision.txt`와 checkout이 일치하고 checkout은 clean이어야 한다. run에서 APK digest와 source checksum을 보존한다.
- **기기 상태:** 시작 시 device state가 `device`, 화면이 awake/unlocked, foreground package가 Chrome이었다. 시험은 Spinon 앱만 `force-stop`하며 Chrome은 종료하지 않는다. 끝나면 Chrome을 다시 foreground로 연다. 화면 잠금 설정·전원 설정은 변경하지 않는다.
- **화면/로그:** `logcat -c`를 사용하지 않는다. 기존 global log buffer를 지우지 않고 Spinon tag stream을 fixture 시작 전에 연 다음, 새 Spinon process PID와 `SPINON_R05_CALLBACK_FAULT_START` 이후 이벤트만 결과로 분류한다. runner bitmap, UI hierarchy, Activity/window 상태를 따로 저장한다.
- **fixture pass 기준:** 새 process에서 summary `status=PASS`, `checks=12`, `failures=0`, 100 race iterations, idle 60 samples와 최종 pending/queue/active 0이어야 한다. 실제 applied-transaction baseline, surface recreation 뒤 `late_after_cancel`, generation 2 recovery 세 경로, queued callback 3건, `opengl_es` draw revision/generation, main-handler timeout, callback executor overflow marker를 모두 확인한다. 하나라도 빠지면 fixture run은 실패 또는 불완전이다.
- **복구 기준:** fixture 뒤 앱을 종료하지 않은 상태로 bitmap을 저장한다. 결과와 현재 foreground 상태를 기록한 뒤 원래 foreground였던 Chrome을 다시 연다. `adb install -r` 외의 uninstall/data wipe/ADB reset은 하지 않는다.
- **성능 제한:** 이 fixture의 race winner 비율·host screenshot 요청/수신 시간은 성능 측정이 아니다. 실제 입력 지연·refresh·광학 결과를 만들지 않는다.

## 실행 절차

1. 대상이 하나의 USB 물리 기기인지 ADB 상세 목록으로 확인하고, 에뮬레이터 serial을 거부한다. SDK full/API, page size, ABI, fingerprint, 화면/잠금/foreground, 설치 APK 경로, source/V8 revision을 저장한다. 값이 바뀌거나 화면 잠금 상태면 중단한다.
2. 새 결과 경로를 만든다. 기존 logcat은 지우지 않고 앱 tag stream을 시작한다. `mise exec -- bun run build:android`로 debug APK를 만들고 SHA-256을 보존한 뒤 `adb install -r`한다. 설치 실패 시 실행하지 않는다.
3. Spinon 앱만 force-stop하고 `MainActivity`에 `spinon_r05_callback_faults=true` extra를 지정해 실행한다. 새 app PID를 확인하고 최대 100초까지 기다린다. 새 PID의 marker만 파싱한다.
4. pass 기준을 모두 검증한다. 요약만 PASS이고 세 actual transaction scenario, queue/draw, timeout/overflow marker가 없으면 통과로 세지 않는다. 최종 화면을 capture하고 hierarchy·Activity state를 저장한다.
5. 원래 foreground였던 Chrome을 다시 열고 package/focus를 확인한다. 기기에서 Spinon data를 초기화하거나 system update/설정을 바꾸지 않는다.

## 실행 결과

- Samsung SM-S731N / Android 16 SDK 36.1 / 4KB / ARM64에서 동일 APK를 두 번 설치·실행했다. 둘 다 12 checks, failures 0, timeout/callback 100회, idle 60 samples, pending·queue·active 0으로 끝났다. race winner는 run 01 87/13, run 02 93/7로 관찰됐고 비율은 성능 판정이 아니다.
- run 01 앱 fixture는 PASS했지만 driver가 시작 직후 `pidof` no-process exit 1에서 종료했다. 앱은 독립적으로 fixture를 끝냈고 logcat buffer를 지우지 않았기 때문에 summary와 bitmap을 사후 보존했다. pid lookup의 초기 exit 1을 허용하도록 driver를 수정했고 run 02에서 build/install/logcat filtering/assertion/screenshot/복구까지 통과했다.
- 세 실제 callback marker는 양 run에서 PASS다. baseline/recovery의 `TransactionStats`는 존재했지만 fence 상태가 pending이라 `fence_signal_usable=false`; stale callback은 generation 1→2 뒤 `late_after_cancel`로 남겼다. usable present timestamp는 얻지 못했다.
- 두 PNG는 PASS 화면을 표시한다. run 02의 ui hierarchy 및 Activity/focus도 동일 화면이고, 테스트 뒤 Chrome foreground와 기존 screen setting을 복구했다.
- 전체 원본과 실행 후 새 20개 실패 관점은 [실기기 보고서](../spec/internal/evidence/r05-android-physical-callback-faults-2026-10-08/README.md), [실행 후 검토](../spec/internal/evidence/r05-android-physical-callback-faults-2026-10-08/implementation-review.md)에 있다. 이는 synthetic fixture이며 physical touch, present latency 또는 R05.3 완료가 아니다.

## 중단·실패 기준

- physical USB device를 고유하게 식별할 수 없거나 ADB가 `offline`/`unauthorized`이면 중단한다.
- API 36.1 / 4KB / ARM64 조건이 달라지면 계획을 수정하고 별도 결과로 남긴다. API 37.0 결과로 physical SDK 조건을 대신하지 않는다.
- 빌드·설치·실행·필수 marker 중 하나라도 실패하면 원본 logcat/screenshot을 보존하고 제품 실패와 harness/기기 호환 실패를 나눠 기록한다.
- 앱 ANR/crash, 화면 잠금, 기기 분리, 사용자의 전면 조작이 관찰되면 결과를 무효 처리하고 기기 상태를 우선 복구한다.
- 사용자 데이터 파괴, AVD 이미지 설치, logcat 전체 삭제, 기기 설정 변경은 하지 않는다.

## 실행 전 적대 검토 · 20개 독립 실패 관점

| # | 실패 관점 | 계획의 차단·판정 |
|---:|---|---|
| 1 | 다른 USB 기기 또는 에뮬레이터에 APK 설치 | 명시 serial의 USB physical transport와 product/model을 교차 확인하고 emulator serial은 거부한다. |
| 2 | API 36을 36.1 또는 37.0으로 잘못 라벨링 | `SDK_INT`, `SDK_INT_FULL`, build fingerprint를 별도 기록하고 허용 조건을 고정한다. |
| 3 | host page size를 device page size로 오인 | 기기 `getconf PAGE_SIZE`만 기록한다. |
| 4 | ARM64 기기라고 추정만 하고 APK ABI를 확인하지 않음 | 기기 ABI 및 APK ABI 구성/설치 결과를 함께 보존한다. |
| 5 | 기존 emulator 전용 runner가 실기기를 차단하는 이유를 무시 | serial 검사를 우회하거나 환경변수에 device serial을 넣지 않고 별도 physical driver로 동일 assertion을 실행한다. |
| 6 | API36.1에서 TransactionStats API 또는 class loading이 실패 | 기기에서 실제 실행되는 3 lifecycle 경로를 통과 기준으로 둔다. compile만으로 runtime을 추정하지 않는다. |
| 7 | 기존 APK 또는 stale PID의 PASS 로그를 새 실행으로 판정 | run 직전 source marker를 기록하고 새 PID·start marker 이후 로그만 본다. |
| 8 | global `logcat -c`로 사용자/다른 앱 진단 기록 삭제 | logcat buffer를 지우지 않고 PID/tag/time boundary로 결과를 필터링한다. |
| 9 | 앱 재설치 과정에서 사용자 상태를 지움 | `install -r`만 허용하고 `pm clear`, uninstall, data wipe를 금지한다. |
| 10 | 앱 foreground 테스트가 Chrome 작업을 종료 | Spinon만 force-stop하며 Chrome process는 종료하지 않고 끝에 Chrome을 복귀시킨다. |
| 11 | 화면 잠금/절전이 검정 bitmap을 만듦 | 사전 awake/unlocked를 확인하고 실행 중 상태를 기록하며 설정을 변경하지 않는다. |
| 12 | synthetic fixture 입력을 실제 손가락 터치로 오분류 | 로그의 `input_source=synthetic_fixture`를 보존하고 본 실행에서 실제 touch를 주장하지 않는다. |
| 13 | race winner 비율을 성능 또는 deterministic outcome으로 해석 | iteration count와 상태 불변식만 pass gate로 쓰고 callback/timeout 비율을 비교 승자로 만들지 않는다. |
| 14 | 요약 PASS지만 actual applied transaction 세 경로 누락 | 각 baseline/recreation/recovery marker를 개별 필수 조건으로 검사한다. |
| 15 | callback queue가 포화됐지만 거부·worker 복구를 확인하지 않음 | overflow marker와 최종 queue/active/pending=0을 함께 요구한다. |
| 16 | 화면 캡처에 이전 앱 또는 system shade가 보임 | capture와 같은 시점의 current focus·resumed Activity·app PID를 기록한다. |
| 17 | hierarchy text를 실제 pixel 표시로 대체 | screenshot bitmap을 직접 열어 PASS 문구를 확인하고 hierarchy는 보조 증거로 둔다. |
| 18 | 기기 분리 후 다른 serial로 이어 실행 | 실행 전후 ADB serial/model/SDK를 재검사하고 변경되면 결과를 무효화한다. |
| 19 | test failure를 product defect로 단정하거나 harness 오류를 숨김 | build/install/launch/marker 단계별 로그와 종료 상태를 분리해 원인을 분류한다. |
| 20 | 합성 callback 시험으로 실제 사용자 지연/화면 주사를 주장 | latency·scanout·광자·성능 주장을 명시적으로 제외하고 R05.3 미완료를 유지한다. |

이 검토는 physical 실행 전 계약·환경·복구 경계만 판단한다. 실행 후에는 실제 결과 대상으로 별도 20개 실패 관점을 기록한다.

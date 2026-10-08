# R05.3 · Android API 37.1 applied-transaction 수명주기 검증 계획

**상태:** 새 AVD fresh boot 및 debug fixture 통과 · R05.3 미완료
**상위 항목:** [R05 입력→표시 신호 계획](r05-input-to-presentation.md) · [Android API 37.2 applied-transaction 실행](r05-android-applied-transaction-lifecycle.md) · [구현 상태 대장](../spec/STATUS.md)

## 목적과 범위

API 37.2에서 확인한 실제 `SurfaceControl.Transaction`·R08 GLES `SurfaceView` 수명주기 fixture를 API 37.1 / 16KB ARM64 emulator에서 그대로 실행한다. Android API 37.1 transaction-completed listener가 정상 surface, surface destroy/recreate 중 stale callback, 새 generation 회복 경로에서 같은 request 수명주기 규칙을 지키는지 확인한다.

이번 실행은 API 37.1 한 대상의 추가 행이다. API 35와 API 37.0의 applied-transaction lifecycle, API 29–34 fallback, iOS device callback, 실기기 및 R05.3 전체 완료를 대신하지 않는다.

## 비교 모델과 사전 조건

- 비교 기준은 병합된 API 37.2 실행의 동일 debug fixture와 [공식 Android API 계약](https://developer.android.com/reference/android/view/SurfaceControl.Transaction#addTransactionCompletedListener(java.util.concurrent.Executor,java.util.function.Consumer))이다. `addTransactionCompletedListener`는 API 35부터 제공되고 listener는 transaction이 presented되면 한 번 전달된다. callback은 전달한 executor에서 실행된다.
- `SurfaceView.applyTransactionToFrame`은 API 34부터 제공된다. 정확한 next-frame 관계는 호출 전에 renderer를 멈추고 호출 뒤 한 frame을 제출했을 때만 정의되며 시스템이 transaction 소유권을 인수한다. 이 fixture는 R08의 고정 frame 경로를 재사용하고 실제 Android callback의 `TransactionStats`를 확인한다. [SurfaceView API 계약](https://developer.android.com/reference/android/view/SurfaceView#applyTransactionToFrame(android.view.SurfaceControl.Transaction))
- 실행 전 runner가 다음을 모두 확인해야 한다: serial `emulator-5580`, API level 37, `sdk_full=37.1`, guest page size 16,384 bytes, ARM64 ABI, AVD name `spinon_api37_1_compat16k`, pinned V8 revision.
- 최초 run-01은 사용자 삭제로 AVD data directory가 사라진 뒤에도 실행 중이던 QEMU PID 6922에서 수행됐다. 이를 cold boot로 표기하지 않으며 기존 증거는 덮어쓰지 않는다.
- 사용자가 현재 실행 중인 Spinon AVD 종료와 필요한 AVD 재설치를 요청했다. API 37.1 검증에 필요한 기존 system image `android-37.1/google_apis_ps16k/arm64-v8a` revision 9는 설치되어 있다. 기존 AVD `.ini`는 data directory가 없어 `avdmanager list avd`에서 손상으로 판정됐다. 실행 전에는 Spinon API 37.1·37.2 serial만 정상 종료하고 종료된 PID·ADB serial·삭제 파일 handle을 확인한다. 다른 프로젝트의 `zl_poc`는 종료하지 않는다.
- 재실행 전에 API 37.1 AVD를 같은 이름으로 다시 만든다. 기존 `config.ini`가 삭제되어 정확한 하드웨어 프로필은 복구할 수 없다. 새 AVD는 기록 가능한 표준 `pixel_6` profile, 설치된 API 37.1 16KB ARM64 Google APIs image revision 9, host GPU 설정으로 만들고, 이 프로필 차이를 manifest에 명시한다. 이 실행은 API·ABI·page-size 호환 확인이지 이전 AVD와의 geometry/performance 비교가 아니다.
- AVD를 만들기 직전 여유 공간을 다시 확인한다. 10 GiB 미만이면 생성하지 않는다. 실행 뒤 guest API level 37, full version 37.1, page size 16,384 bytes, ARM64 ABI와 실제 AVD name을 기기에서 검증한다. 확인 실패 시 결과는 실패로 남기며 API를 추정하지 않는다.
- Android debug APK와 source digest를 별도 출력 폴더에 저장한다. API 37.2 기존 자료는 변경하지 않는다. 결과 대상은 [API 37.1 재실행 폴더](../spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/)다.

## 성공 판정

1. 기존 fixture가 12개 check를 통과하고 timeout/callback CAS 경합 100회에서 terminal winner가 하나씩 결정된다.
2. 실제 R08 draw revision marker가 baseline, stale lifecycle, recovery 세 request를 각각 확인하며 listener task가 production callback executor queue에 든 뒤 surface를 detach한다.
3. baseline 실제 callback은 non-null `TransactionStats`, 현재 generation, callback count 1, timeout/map 정리를 보여야 한다.
4. surface destroy가 timeout보다 먼저 이전 generation을 취소해야 한다. 뒤늦게 실행된 실제 callback은 `late_after_cancel`, `current_surface=false`, `fence_signal_usable=false`여야 한다.
5. 같은 view 재부착 뒤 generation이 증가하고, 새 세대의 actual callback이 정상 완료되어야 한다.
6. 종료 상태에서 pending·queue·active가 0이고 60초 idle 관찰에 회수 실패가 없어야 한다.
7. 최종 Android screenshot은 fixture PASS를 표시해야 하며 API·page size·AVD process·V8·APK/source digest·전체 logcat이 실행 자료와 일치해야 한다.

`MotionEvent`는 synthetic fixture 입력이다. callback/fence 관찰을 광학 scanout이나 event-to-present latency로 해석하지 않는다.

## 계획 적대 검토 · 독립 실패 관점 20개

| # | 공격 관점 | 계획의 차단·판정 | 계획 검토 결과 |
|---:|---|---|---|
| 1 | AVD 표시명만 믿고 API 37.1로 오분류 | `sdk_full=37.1`을 runner가 직접 검사한다. | 확인: sdk_full=37.1이며 runner 분기가 일치한다. |
| 2 | API 37.1이라도 4KB guest를 16KB 대상으로 오인 | `getconf PAGE_SIZE=16384`를 실행 전 gate로 요구한다. | 확인: guest page size 16,384이며 runner gate와 일치한다. |
| 3 | API 37.0/API 37.2 serial로 잘못 실행 | serial·AVD name·full API를 한 실행 manifest에 기록하고 고정한다. | 확인: emulator-5580과 spinon_api37_1_compat16k가 일치한다. |
| 4 | ARM64 아닌 ABI 결과를 같은 행렬로 혼합 | `ro.product.cpu.abi=aarch64` 계열을 확인하고 기록한다. | 확인: ABI는 arm64-v8a다. |
| 5 | 삭제된 backing directory 상태를 fresh boot로 주장 | 기존 run-01과 새 run-02를 분리하고 각 AVD 상태를 따로 기록한다. | 확인: run-01 manifest에 data directory 부재와 QEMU PID가 기록됐다. |
| 6 | 작업 중 QEMU 종료 또는 AVD 데이터 재생성 | 새 계획에서 지정한 Spinon serial만 종료하고 API 37.1을 새로 생성한다. | 계획 변경 전 검토: 기존 process를 유지하는 절차였다. |
| 7 | 테스트 소스와 pinned V8 revision 불일치 | runner의 source SHA와 고정 V8 revision을 결과에 포함한다. | 확인: V8 pin이 요구 revision과 일치하고 checkout이 clean이다. |
| 8 | 기존 37.2 성공 자료를 37.1 증거로 재사용 | 새 run directory·serial·APK digest·logcat을 요구한다. | 확인: run-01 경로가 비어 있어 기존 자료를 덮지 않는다. |
| 9 | synthetic callback을 Android callback으로 오인 | actual scenario에서 `TransactionStats != null`과 callback count 1을 확인한다. | 코드 확인: 실제 scenario에서 stats availability와 callback count를 검사한다. |
| 10 | R08 draw가 없는 상태에서 transaction 적용을 주장 | 각 scenario revision/generation의 `SPINON_R05_DRAW` marker를 검사한다. | runner 확인: baseline·recreate·recovery를 모두 필수 처리한다. |
| 11 | transaction listener가 frame에 apply되지 않았는데 callback 성공 취급 | production `applyTransactionToFrame` 실행 log와 actual OS callback을 모두 요구한다. | 코드 확인: production applyTransactionToFrame과 실제 callback 경로다. |
| 12 | callback queue가 아닌 임의 worker block을 사용 | production callback executor의 active=1·queue=1·request pending 상태 뒤 detach한다. | 코드 확인: production callback executor의 queue/active를 검사한다. |
| 13 | detach 전에 callback이 이미 완료된 경합을 stale 성공으로 셈 | destroy 전 callback count 0을 확인하고 실제 destroy event를 기다린다. | 코드 확인: queue 진입 뒤 실제 destroy callback을 기다린다. |
| 14 | UI thread를 막아 timeout 또는 surface 경계를 인위적으로 유발 | 기다림·polling은 fixture worker에서 수행하고 main handler 동작은 짧게 유지한다. | 코드 확인: fixture 대기는 worker에서 하고 main action은 시간 제한이 있다. |
| 15 | surface cancellation production 경로를 호출하지 않음 | 실제 `SurfaceHolder.Callback.surfaceDestroyed`와 R08 production cancel 경로를 관찰한다. | 코드 확인: 실제 R08 surfaceDestroyed가 production cancel 경로에 연결된다. |
| 16 | 이전 callback이 새 generation 성공으로 되살아남 | old generation, `late_after_cancel`, current surface false, unusable을 함께 요구한다. | 코드 확인: late_after_cancel, old generation, current_surface=false를 요구한다. |
| 17 | timeout이 cancellation보다 먼저 terminal이 됨 | cancellation 시 timeout count=0·timer 없음·pending 없음이어야 한다. | 코드 확인: timeout count, timer, pending을 확인한다. |
| 18 | 실패 cleanup 뒤 callback worker·timer·queue가 남음 | `finally` release, bounded drain, final pending/queue/active=0 및 idle sample을 요구한다. | runner 확인: finally cleanup, executor drain, 60초 idle gate가 있다. |
| 19 | 잘못된 APK나 중간 screenshot/log를 결과로 연결 | runner-created run dir, APK/source hashes, PASS marker 이후 capture를 묶는다. | runner 확인: PASS marker와 digest 이후 screenshot을 저장한다. |
| 20 | API 37.1 한 emulator를 전체 API 지원·성능 완료로 일반화 | 상태 대장에 API 37.1 한 행만 기록하고 R05.3 및 남은 API·기기 gate는 미완료로 둔다. | 범위 확인: API 37.1 한 행만 다루고 matrix와 R05.3 완료를 제외한다. |

최초 계획 검토는 run-01의 기존 QEMU·AVD 상태와 runner gate를 대조했다. 사용자가 AVD 종료·재설치를 요청한 뒤 이 계획은 새 AVD 부팅을 포함하도록 변경됐다. 아래 별도의 20개 관점으로 수정된 설치·종료 계획을 재검토한다.

위 표는 재설치 전 run-01의 실행 전 검토다. 구현·실행 뒤에는 새 AVD source와 실제 API 37.1 log를 대상으로 별도의 실패 경로 검토를 다시 작성한다.

## AVD 복구 계획 변경 · 별도 적대 검토 20개

| # | 독립 실패 관점 | 수정 계획의 차단·판정 |
|---:|---|---|
| 1 | `.ini` 경로를 믿고 다른 AVD 폴더를 지움 | `avdmanager`가 손상으로 표시한 정확한 이름 하나만 대상으로 삼고, 실제 ini path와 data path를 대조한다. |
| 2 | 열린 QEMU가 있는데 backing AVD를 덮어씀 | API 37.1·37.2의 지정 serial을 정상 종료한 뒤 PID와 ADB serial이 사라질 때까지 기다린다. |
| 3 | `adb emu kill` 응답만 보고 종료 성공으로 간주하거나 launcher 종료와 함께 새 QEMU가 사라짐 | QEMU PID·ADB devices·`lsof +L1`로 종료를 확인한다. 재부팅은 실행 세션을 유지하고, 셸 종료 뒤 process가 생존하는지와 ADB serial을 fixture 직전에 다시 확인한다. |
| 4 | 다른 프로젝트 `zl_poc`까지 함께 종료 | 허용 serial/name 목록을 API 37.1·37.2 Spinon AVD 두 개로 제한하고 `zl_poc`는 명시적으로 제외한다. |
| 5 | 종료한 프로세스가 이미 다른 이름의 emulator | 명령행 `-avd` 값과 `adb emu avd name`을 서로 대조한 뒤 종료한다. |
| 6 | AVD 생성 명령의 종료 코드만 믿고 등록 성공을 가정 | CLI 12.0의 `--force` 실행은 XML v4 경고 뒤 orphan config만 남겼고 원인은 미확정이다. CLI 23.0 실행 후 `avdmanager list avd`, `.ini` 경로, `config.ini`의 device/image를 대조한다. |
| 7 | 설치된 image package가 API 37.1이 아님 | `source.properties`의 API 37.1, ARM64, 16KB tag와 revision 9를 생성 전 확인한다. |
| 8 | API 37.1과 37.2 16KB image를 혼동 | 새 AVD package id를 `android-37.1/google_apis_ps16k/arm64-v8a`로 고정하고 boot 후 full version을 확인한다. |
| 9 | 기존 손상 ini와 새 AVD 이름이 충돌 | 이전 ini와 orphan config를 보존했다. partial directory는 `.avd` suffix에서 격리하고 새 `.ini`의 경로가 새 AVD directory를 가리키는지 확인한다. |
| 10 | 복구할 수 없는 기존 hardware profile을 동일하다고 주장 | 삭제된 `config.ini`는 복원할 수 없다고 기록하고 새 표준 Pixel 6 profile과 geometry 차이를 manifest에 남긴다. |
| 11 | 생성 시 잘못된 device profile ID를 사용 | CLI 23.0 `list device`에 실제 등록된 `pixel_6` ID인지 확인하고 `config.ini`의 `hw.device.name`도 대조한다. |
| 12 | 기본 API 37 image로 생성해 guest가 4KB가 됨 | 16KB 전용 package id를 지정하고 guest 내부 `getconf PAGE_SIZE`가 16384인지 검사한다. |
| 13 | AVD 이름은 맞지만 부팅이 이전 snapshot에서 복원됨 | 새 data directory와 `-no-snapshot-load -no-snapshot-save` boot를 사용하고 생성 시각·PID·boot completion을 기록한다. |
| 14 | API 37.1이 아닌데 API 37만 확인 | `SDK_INT=37`과 `ro.build.version.sdk_full=37.1`을 모두 확인한다. |
| 15 | ARM64 host와 guest ABI를 혼동 | boot 후 `ro.product.cpu.abi=arm64-v8a`를 확인하고 실행 manifest에 보존한다. |
| 16 | host page size를 Android guest page size로 오인 | page size는 반드시 대상 emulator shell의 `getconf PAGE_SIZE`에서 가져온다. |
| 17 | port/serial 충돌로 다른 emulator에 APK 설치 | 새 boot 전에 기존 serial 5580이 사라졌음을 확인하고 새 QEMU command의 port 5580/5581을 기록한다. |
| 18 | API image 재다운로드나 AVD 생성으로 저장 공간을 압박 | revision 9가 설치된 것을 확인하고 다운로드하지 않는다. 생성 직전 여유 공간 10 GiB 기준을 검사한다. |
| 19 | 이전 실행과 새 실행의 산출물이 섞임 | 새 결과 디렉터리 `run-02-fresh-avd`를 사용하고 새 boot PID·app PID·APK/source digest·screenshot을 같은 폴더에 묶는다. |
| 20 | 새 API 37.1 AVD 통과를 API 전체 또는 성능 완료로 일반화 | API 37.1 한 항목만 갱신하며 API 35·37.0 actual lifecycle, API 29–34, iOS device, 실기기와 R05.3은 미완료로 둔다. |

이 20개 검토는 사용자 지시로 변경된 종료·재설치 계획에만 적용한다. 실행 뒤 source·새 boot·callback lifecycle·cleanup은 별도의 구현 실패 관점 20개로 다시 검토한다.

## AVD 복구·새 부팅 절차

정상 종료 전에 `adb emu avd name`과 QEMU 명령의 `-avd` 값을 각각 확인한다. 다음 두 Spinon AVD만 종료한다.

```sh
adb -s emulator-5580 emu kill # spinon_api37_1_compat16k
adb -s emulator-5562 emu kill # spinon_api37_2
```

두 serial과 QEMU PID가 없어지고 해당 PID의 삭제된 AVD handle이 닫힌 것을 확인한다. `emulator-5554` / `zl_poc`는 다른 프로젝트용이므로 종료하지 않는다. 기존 API 37.1 `.ini`는 `avd-recovery/original-avd-registration.ini`, CLI 12.0이 남긴 orphan config는 `failed-cli12-partial-config.ini`로 보존한다. 부분 `.avd` 디렉터리는 `spinon_api37_1_compat16k.avd.cli12-partial`로 이동해 유지한다. 설치된 image revision 9로 같은 AVD 이름을 재생성한다.

```sh
mkdir -p spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/avd-recovery
sdk_dir="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
cp "$HOME/.android/avd/spinon_api37_1_compat16k.ini" \
  spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/avd-recovery/original-avd-registration.ini
cp "$HOME/.android/avd/spinon_api37_1_compat16k.avd/config.ini" \
  spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/avd-recovery/failed-cli12-partial-config.ini
avdmanager="$sdk_dir/cmdline-tools/latest-2/bin/avdmanager" # Android CLI 23.0
printf 'no\n' | mise exec -- "$avdmanager" \
  create avd --name spinon_api37_1_compat16k \
  --package 'system-images;android-37.1;google_apis_ps16k;arm64-v8a' \
  --device pixel_6
```

CLI 12.0의 `--force` 실행은 SDK XML v4 파싱 경고 뒤 `.ini` 없는 orphan config를 남겼다. CLI 버전이 원인인지는 확정하지 않는다. orphan과 기존 ini는 보존하고, CLI 23.0으로 등록된 AVD의 manager 목록 name, `.ini` path, `config.ini` device/image를 교차 확인한다. `avd.id`와 `avd.name`의 `<build>` 값은 이 환경에서 생성되어 이름 판정에 사용하지 않는다. 새 QEMU는 백그라운드 자식으로 시작하지 말고 유지되는 `exec_command` session에서 foreground로 실행한다. 이 단계가 끝난 뒤에도 process PID와 ADB serial이 살아 있는지 확인한다. emulator는 ports `5580,5581`, `-gpu host`, `-no-snapshot-load`, `-no-snapshot-save`를 사용한다. stdout/stderr는 `avd-recovery/emulator-startup.log`로 보내고 process PID·session 식별 기록을 보존한다. `sys.boot_completed=1`을 180초 deadline으로 기다린 다음 API 37/full 37.1, guest page size 16,384 bytes, `arm64-v8a`, AVD name을 검증하고 fixture를 실행한다. image revision·device profile·실행 전후 여유 공간도 `avd-recovery/`에 기록한다.

첫 boot는 guest gate를 통과했지만 emulator를 background child로 실행한 호출이 끝난 뒤 process가 사라졌다. runner는 device 부재를 감지해 빌드 전에 중단했다. 이는 fixture 결과가 아니다. 상세는 [첫 launcher 수명 실패 기록](../spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/avd-recovery/first-launch-lifetime-failure.md)에 남겼다.

유지되는 foreground session에서 사용할 실행 형식은 다음과 같다.

```sh
"$sdk_dir/emulator/emulator" -avd spinon_api37_1_compat16k \
  -ports 5580,5581 -gpu host -no-snapshot-load -no-snapshot-save \
  -no-boot-anim -verbose \
  > spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/avd-recovery/emulator-startup.log 2>&1
```

## 실행 명령

```sh
SPINON_R05_CALLBACK_EXPECTED_API_FULL=37.1 \
SPINON_ANDROID_EMULATOR_SERIAL=emulator-5580 \
SPINON_V8_DIR=/tmp/spinon-r05-present-api-audit/build/v8-source/v8 \
SPINON_R05_CALLBACK_OUTPUT_DIR=spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/run-02-fresh-avd \
mise exec -- bash tools/verify-r05-android-callback-faults.sh
```

## 새 AVD 실행 결과

CLI 23.0으로 재등록한 `spinon_api37_1_compat16k` AVD를 Pixel 6 profile로 새로 부팅했다. 이전 `.avd` data directory를 사용하지 않았고 `-no-snapshot-load -no-snapshot-save`를 지정했다. guest에서 API 37.1, 16KB page size, ARM64 ABI, AVD 이름을 확인한 뒤 기존 debug fixture를 실행했다. 이전에 삭제된 hardware profile과 geometry가 같다고 주장하지 않는다.

첫 background launcher 시도는 shell session이 끝난 뒤 QEMU가 종료되어 runner가 앱 빌드 전에 serial 부재로 차단했다. 그 시도는 fixture 실행이 아니다. QEMU를 유지되는 foreground session으로 띄운 뒤 다시 실행한 `run-02-fresh-avd`에서 actual R08 GLES applied-transaction 정상·stale·recovery 세 경로가 통과했다. 총 12 checks, 0 failures, timeout/callback 경합 100회(52/48), queue overflow 회복, 60초 idle 60개 표본이며 종료 상태 pending/queue/active는 모두 0이다. 새 QEMU PID 56444는 fixture 뒤에도 살아 있었고, 캡처 당시 `.android/avd` 아래 열린 삭제 파일은 없었다. fixture 확인 후 해당 emulator를 정상 종료했으며 재설치한 AVD와 data directory는 유지했다. 종료 뒤에는 `emulator-5580` process/serial이 없어졌고 AVD manager에는 새 API 37.1 AVD가 정상 등록된 상태다. 다른 프로젝트 `zl_poc`와 손상된 이전 AVD 등록은 수정하지 않았다. [종료 후 검증 기록](../spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/avd-recovery/post-shutdown-verification.txt).

입력은 synthetic `MotionEvent`다. 이 결과는 Pixel 6 profile의 API 37.1 debug emulator 한 대에 한정되며 제품 지연, optical scanout, 실기기, release, 다른 API의 lifecycle 또는 R05.3 전체 완료를 뜻하지 않는다. 상세 원본은 [fresh AVD 실행 보고서](../spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/README.md), [run-02 환경](../spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/run-02-fresh-avd/environment.txt), [전체 logcat](../spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/run-02-fresh-avd/logcat.txt), [화면 캡처](../spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/run-02-fresh-avd/result.png), [구현·실행 실패 경로 검토](../spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/implementation-review-fresh-avd.md)에 있다.

R05.3은 완료 처리하지 않는다. API 35·37.0 actual applied-transaction lifecycle, API 29–34 fallback, iOS device callback runtime·clock residual, 실기기와 optical measurement는 남아 있다.

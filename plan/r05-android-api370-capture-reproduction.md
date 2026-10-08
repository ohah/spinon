# R05.3 · Android API 37.0 결과 화면 캡처 재현 계획

**상태:** 비교 모델 확정 · 실행 전 독립 실패 관점 20개 검토 완료 · 재현 실행 전

**대상:** PR #77에서 확인한 API 37.0 debug fixture의 검정 결과 캡처와 activity 복귀 뒤 PASS 캡처 차이

**상위 근거:** [API 35·37.0 applied-transaction 실행](../spec/internal/evidence/r05-android-api35-370-applied-transaction-2026-10-08/README.md) · [R05.3 입력→표시 계획](r05-input-to-presentation.md) · [구현 상태](../spec/STATUS.md)

## 질문과 범위

PR #77의 API 37.0 실행에서 runner의 `result.png`와 즉시 재확인 캡처는 검정이었다. UI hierarchy는 `R05 callback 실패 주입: PASS` TextView를 포함했다. Android Settings 화면은 정상으로 캡처됐고 Settings에서 앱으로 돌아온 캡처에서는 PASS 문구가 보였다. 기존 원본과 AVD 데이터는 보존되어 있다.

이 계획은 같은 API 37.0 AVD를 snapshot 없이 cold boot해 위 차이를 다시 관찰한다. **캡처 시점, 앱 Activity/surface 생명주기, Android 화면 합성, adb screenshot 경로**를 분리해서 기록한다. AVD 삭제·재생성, system image 재설치/업데이트, renderer·fixture 수정은 실행 범위에 넣지 않는다. 재현에서 fixture 결과를 바꾸거나 합격 기준을 낮추지 않는다.

## 비교 모델

- **기준 실행:** PR #77의 source commit `f3167392241cc435a368d8dbc236d62f37525daa`가 현재 `origin/main`이다. 이전 API 37.0 run은 source commit `300443b868d06a4fdc59a0fb72770df4edb57ca8`였고, 동일한 `R05 callback 실패 주입: PASS` 문구를 만들었다. 이번 실행은 repository main의 현 코드와 clean pinned V8로 빌드한다. 실행 시 source/APK digest를 이전 실행과 각각 비교한다.
- **기기:** 등록된 기존 `spinon_api37_0_applied_txn` AVD와 동일한 Android API 37.0 Google APIs ARM64 image revision 6, guest page size 4096, Pixel 6 profile, 1080×2400·density 420, `-gpu host`를 사용한다. 이미지와 `.avd` data directory를 재사용한다.
- **cold boot 조건:** 실행 전 serial `emulator-5580`과 같은 AVD의 QEMU process가 없어야 한다. `-no-snapshot-load -no-snapshot-save`로 boot하고 `sys.boot_completed=1`, guest `api_full=37.0`, page size·ABI·AVD name·PID를 기록한다. `-wipe-data`를 사용하지 않는다.
- **화면 대조:** PASS 기준은 화면 픽셀에서 dark surface 위 `R05 callback 실패 주입: PASS`와 12 checks·100회 경합·60초 idle 문구가 보이는 것이다. UI hierarchy의 텍스트만으로 화면 표시를 판정하지 않는다. Android Settings 화면은 시스템 화면 캡처의 양성 대조이며, Settings 복귀 뒤 앱 화면은 Activity/surface resume 대조다.
- **동일 run 내 캡처 순서:** fixture 완료 시 runner 원본을 보존한다. 다음에 추가 screenshot을 즉시 저장하고 host `monotonic_ns`로 기준 시각을 잡아 1초·5초·15초에서 예정 시각을 넘기지 않도록 무입력 캡처한다. 각 screenshot을 먼저 취득하고 capture latency와 monotonic timestamp를 기록한 뒤 UI hierarchy와 foreground/window/surface 상태를 수집해 관찰 도구가 직전 bitmap을 바꾸지 않게 한다. 모든 예정 시각 캡처 뒤 Settings 대조와 앱 복귀를 수행한다.
- **반복:** 같은 기존 AVD를 재사용해 서로 분리된 두 cold-boot 세션에서 fixture를 한 번씩 실행한다. 첫 세션은 fixture·capture series·Settings/resume 뒤 정상 종료하고, 두 번째는 새 QEMU process로 같은 cold-boot 옵션을 적용한다. 각 세션은 기존 12 checks·100 race·60초 idle을 전부 통과해야 시각 캡처 비교에 포함한다.
- **환경 보존:** 다른 프로젝트 `zl_poc` / `emulator-5554`는 그대로 둔다. `emulator-5580`에 다른 AVD가 연결됐거나 10 GiB 미만의 여유 공간이면 실행을 중단한다. 기존 결과 경로는 덮어쓰지 않는다.

## 실행 절차와 판정

1. 새 worktree가 current `origin/main`에서 분기됐고 tracked/untracked 변경이 없는지 확인한다. pinned V8 revision과 clean 상태, 시스템 image `source.properties`, 기존 AVD `config.ini`·data directory, serial·QEMU 충돌 여부, 여유 공간을 evidence preflight에 기록한다. `dumpsys power`와 window/keyguard 상태를 확인해 화면이 awake·unlocked이고 회전·notification shade·다른 overlay가 없는지 저장한다. asleep이면 KEYCODE_WAKEUP으로 화면만 깨우고 재검사한다. 잠금이 남으면 fixture를 진행하지 않는다.
2. 기존 `spinon_api37_0_applied_txn`만 snapshot 없이 시작한다. cold boot 뒤 API·page size·ABI·fingerprint·AVD name·PID를 검증한다. API 값이 다르거나 기존 AVD data directory가 없어졌다면 그 조건을 로그로 남기고 fixture 실행을 시작하지 않는다.
3. `tools/verify-r05-android-callback-faults.sh`를 API 37.0에 고정해 첫 실행한다. runner가 12 checks, 100회 경합, 60초 idle, queue overflow, three actual applied-transaction paths와 draw revisions를 전부 통과해야 한다. API37.0 test output의 기존 디렉터리는 덮어쓰지 않는다.
4. PASS 요약 직후 저장된 runner screenshot을 보존한다. Python/adb orchestration으로 추가 `screencap`을 즉시 취득한 후 같은 시각에서 UI dump와 SurfaceFlinger/window/activity 상태를 순서대로 읽는다. 다음 캡처는 각 예정 시각(1초·5초·15초)에 대해 `monotonic_ns` deadline으로 취득하고, 매 capture의 시작·완료 monotonic time을 저장한다. screenshot command 지연이 예정 시각을 넘으면 예정 시각이 아니라 실제 취득 시각으로 분류한다.
5. 시간 대기 계열 캡처가 끝난 뒤 Android Settings를 표시해 시스템 UI 캡처를 확인하고 Back으로 원래 Activity에 복귀한다. 복귀 즉시 screenshot을 먼저 취득한 뒤 hierarchy·Activity/surface 상태를 수집한다. 1초 뒤 screenshot과 상태도 같은 순서로 저장한다.
6. 첫 세션이 끝나면 `adb emu kill`로 이 Spinon AVD를 정상 종료하고 AVD data directory 보존 및 5580 serial/QEMU 소멸을 확인한다. `zl_poc` process는 그대로 둔다. 동일한 API 37.0 AVD를 `-no-snapshot-load -no-snapshot-save`로 두 번째 cold boot하고 2–5단계를 반복한다. 결과는 `run-01-cold-boot/`와 `run-02-cold-boot/`로 분리한다.
7. PNG를 화면에서 육안으로 검사한다. PASS 문구가 hierarchy에만 존재하는 캡처는 visual PASS가 아니다. black frame/normal frame 영역을 구분할 수 있게 PNG checksum·크기·capture time을 보존한다.
8. 두 세션 뒤 Spinon AVD를 정상 종료한다. AVD data directory 보존, 5580 serial/QEMU 소멸, `zl_poc` 생존을 마지막 inventory로 확인한다.

| 관측 결과 | 해석 제한 |
|---|---|
| runner capture는 검정이고 대기 capture도 검정, 앱 resume 뒤 PASS | activity/surface 전환과 연관된 갱신 가능성. lifecycle trigger와 SurfaceFlinger composition을 더 분리하기 전 원인 확정 금지 |
| 즉시 capture 검정, 시간만 지난 capture에서 PASS | 첫 화면 합성 지연 또는 캡처 시점 연관. 두 실행에서 반복되는지 확인 |
| runner capture는 검정이나 추가 즉시 capture부터 PASS | runner capture timing/command path 후보. adb screenshot이 앱 첫 화면을 놓친 것인지 동일 타임라인 증거 필요 |
| Settings 대조도 검정 | 일반적인 app surface 원인으로 한정할 수 없음. emulator/display capture 경로 조사 필요 |
| cold boot부터 runner capture와 반복 캡처 모두 PASS | 기존 현상이 재현되지 않음. AVD 재설치가 원인이었다고 결론내리지 말고 transient/non-reproduced로 남김 |
| 두 run에서 결과가 서로 다름 | 변동 조건이 미분리. 시각 capture gate는 미확정으로 유지 |

화면 capture 진단은 실제 callback fixture의 semantic PASS와 별도 판정한다. host `monotonic_ns`는 adb screenshot command의 요청/수신 interval만 재며 Android frame-present 시각이 아니다. 캡처 원인이 확정되지 않아도 fixture runtime result를 다시 측정한 사실만으로 표시 신호·실기기·성능 주장을 만들지 않는다.

## 실패·중단 조건

- 다른 프로세스가 `emulator-5580`을 사용하거나 AVD 이름·image revision·guest API/page size가 다르면 boot/fixture를 시작하지 않는다.
- fixture가 12 checks 또는 race/idle/cleanup 조건 중 하나라도 실패하면 해당 결과 capture를 성공 비교로 세지 않는다. 원본 로그와 화면은 보존하고, 실패 뒤 수정·재시도는 별도 output directory를 쓴다.
- AVD data directory나 사용자 데이터는 삭제하지 않는다. `-wipe-data`, CLI force overwrite, system image 재설치/업데이트를 금지한다.
- Settings/Back 전환이 fixture를 다시 실행하거나 새로운 앱 프로세스를 만들면 해당 캡처를 기존 fixture의 동일 화면으로 취급하지 않고 별도 상태로 분류한다.
- UI hierarchy text, process state, SurfaceFlinger layer 존재 하나만으로 실제 픽셀 표시를 주장하지 않는다.

## 결과 산출물과 경계

- `spec/internal/evidence/r05-android-api370-capture-2026-10-08/README.md`: 대조 모델, 타임라인, 캡처별 판정, 종료·AVD 상태.
- `run-01-cold-boot/` 및 `run-02-cold-boot/`: environment, source/APK digest, fixture logcat, hierarchy/window/surface 상태, 시간별 screenshot, checksum.
- 이 조사는 화면 캡처 진단이다. 공개 API/인터페이스 변경은 없다. `spec/STATUS.md`의 R05.3/R05 상태를 완료로 바꾸지 않는다.
- 구현/실행 후 실제 evidence 대상으로 별도 20개 failure perspective를 검토한다. 이 계획의 관점으로 구현 후 검토를 대체하지 않는다.

## 계획 적대 검토 · 독립 실패 관점 20개

| # | 독립 실패 관점 | 계획상 방지·판정 |
|---:|---|---|
| 1 | 스냅샷 restore를 cold boot로 오인 | `-no-snapshot-load`, `-no-snapshot-save`, guest boot completion과 QEMU 시작 시각을 함께 기록한다. |
| 2 | 잘못된 API 37.0 system image로 실행 | AVD config image path, `source.properties` revision/tag, guest fingerprint와 `api_full=37.0`을 교차 확인한다. |
| 3 | host page size를 guest 조건으로 대체 | guest `getconf PAGE_SIZE=4096`을 직접 검사한다. |
| 4 | ARM64 host만 보고 guest ABI를 추정 | `ro.product.cpu.abi=arm64-v8a`와 system image ABI를 확인한다. |
| 5 | 이미 사용 중인 serial/AVD를 덮어 실행 | 5580 ADB, QEMU `-avd`, AVD name을 시작 전·실행 중 대조하고 충돌 시 중단한다. |
| 6 | AVD 데이터 재설치/초기화가 변수를 바꿈 | 기존 AVD와 data directory를 재사용하고 wipe/force/image reinstall을 금지한다. |
| 7 | `zl_poc` 또는 다른 프로젝트 emulator를 종료 | serial 5554와 process `-avd zl_poc`를 별도 기록하고 cleanup 대상에서 제외한다. |
| 8 | 새 worktree dirty code로 screenshot 원인을 오염 | current main commit, tracked/untracked 상태, pinned clean V8을 기록한다. |
| 9 | APK rebuild 차이를 AVD 차이로 오인 | source/APK SHA-256을 기존 run 및 두 반복 run과 대조한다. 다르면 별도 원인으로 남긴다. |
| 10 | fixture가 실패했는데 UI hierarchy의 잔여 PASS를 읽음 | 각 run에서 최신 process PID와 `SUMMARY status=PASS`를 요구하고 12 checks·0 failures를 확인한다. |
| 11 | 이전 run Logcat marker를 새 run 결과로 사용 | runner가 logcat을 clear하고 run별 별도 directory를 쓰는지 확인한다. |
| 12 | `screencap` PNG가 부분·빈 파일인데 정상으로 분류 | 각 시점의 PNG 존재·크기·decode 가능성·checksum을 확인하고 실제 화면도 육안 검사한다. |
| 13 | UI hierarchy 문구를 화면 표시 증거로 대체 | UI hierarchy와 bitmap을 별도 evidence로 두고 bitmap에 PASS 문구가 없으면 visual fail로 기록한다. |
| 14 | 단순 대기가 Activity resume 효과와 혼합 | 무입력 시점 0·1·5·15초 캡처를 먼저 끝내고 Settings/Back 전환을 그 뒤 수행한다. |
| 15 | Settings capture를 앱 surface 성공으로 오해 | Settings는 adb/global capture 경로의 양성 대조로만 판정한다. |
| 16 | Back 복귀가 앱을 재생성했는데 같은 surface로 오인 | PID, resumed Activity, SurfaceView layer/generation과 UI hierarchy를 전후 비교한다. |
| 17 | 화면 회전, 잠금, notification shade가 레이아웃을 바꿈 | config portrait geometry와 dumpsys power/keyguard/window 상태를 preflight에 기록한다. asleep만 wake key로 깨우고 잠금 상태가 남으면 중단한다. |
| 18 | screenshot/hierarchy 수집 순서가 상태를 바꾸거나 대기시각을 오염 | bitmap을 먼저 취득하고 host monotonic start/end를 기록한 뒤 hierarchy/window/layer를 읽는다. 예정 시각을 넘긴 캡처는 실제 취득시각으로 분류한다. |
| 19 | 두 번의 run을 서로 다른 API/APK로 합침 | 동일 API 37.0 guest, same boot AVD identity, same source/APK digest를 요구한다. |
| 20 | 미확정 root cause를 고친 결함 또는 AVD 손상으로 단정 | 관측표의 제한대로 transient/non-reproduced와 분리 미확정 결과를 유지한다. |

20개 항목은 계획·비교·실행 gate만 검토한다. 실제 cold boot와 capture 결과를 얻은 뒤 새 20개 관점으로 실행 근거를 다시 공격 검토한다.

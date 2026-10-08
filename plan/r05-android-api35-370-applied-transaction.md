# R05.3 · Android API 35·37.0 applied-transaction 수명주기 검증 계획

**상태:** 계획 검토 완료 · API 35·37.0 debug fixture 각 실행 통과 · API 37.0 첫 화면 캡처는 검정, 앱 복귀 뒤 PASS 캡처 확인 · 캡처 원인은 미확정 · R05.3 미완료

**범위:** API 35 / 4 KB와 API 37.0 / 4 KB ARM64 emulator의 actual R08 GLES applied-transaction lifecycle
**상위 항목:** [R05 입력→표시 신호 계획](r05-input-to-presentation.md) · [API 37.2 기준 실행](../spec/internal/evidence/r05-android-applied-transaction-lifecycle-2026-10-08/README.md) · [API 37.1 fresh AVD 실행](../spec/internal/evidence/r05-android-api371-applied-transaction-2026-10-08/README.md) · [구현 상태 대장](../spec/STATUS.md)

## 목적

R05.3에서 API 37.2와 37.1 actual applied-transaction lifecycle은 debug fixture로 확인했다. 다음 남은 Android lifecycle 경계인 callback 최소 API 35와 37.0에서 동일 R08 GLES `SurfaceView` fixture를 실행한다. 두 API 각각 정상 transaction callback, surface destroy/recreate 뒤 늦게 도착한 callback의 취소 분류, 새 generation 회복을 확인한다.

이 작업은 API별 내부 fixture 검증이다. API 29–34 fallback, API 36 lifecycle, WGPU, 실기기, iOS device callback, release, 광학 scanout, 제품 입력→표시 지연을 다루지 않는다. 결과를 넓은 Android 지원 완료로 해석하지 않으며 `spec/STATUS.md`의 R05.3/R05 체크는 유지한다.

## 비교 모델

- 기준 코드는 PR #76에서 병합한 source commit `300443b868d06a4fdc59a0fb72770df4edb57ca8`의 Android debug fixture와 runner다. 동일한 source commit, pinned V8, `pixel_6` hardware profile, host GPU emulator option, serial `emulator-5580`을 API별 순차 실행에 사용한다.
- 검증 경로는 R08의 실제 OpenGL ES `SurfaceView` frame에 `SurfaceControl.Transaction`을 연결하고 OS가 제공한 `TransactionStats` callback을 추적한다. 기존 API 37.2와 API 37.1 fresh AVD 실행을 이전 비교 행으로 보존한다.
- Android 공식 API 문서에서 `addTransactionCompletedListener`는 API 35부터이며 transaction이 presented된 뒤 한 번 지정 executor에서 callback을 실행한다. `SurfaceView.applyTransactionToFrame`은 API 34부터이며 호출 전에 rendering을 멈추고 호출 뒤 frame 하나를 제출해야 frame 관계가 명확하다. 두 API는 이번 API 35·37.0 대상에 존재한다. [Transaction callback 계약](https://developer.android.com/reference/android/view/SurfaceControl.Transaction#addTransactionCompletedListener(java.util.concurrent.Executor,java.util.function.Consumer)) · [SurfaceView frame transaction 계약](https://developer.android.com/reference/android/view/SurfaceView#applyTransactionToFrame(android.view.SurfaceControl.Transaction))
- 공식 API 계약과 비교할 fixture의 성공 기준은 API별 12 checks, timeout/callback terminal 경합 100회, callback queue overflow 회복, 60초 유휴 관찰, 최종 pending/queue/active=0이다. 기존 fixture의 scenario·check 수를 실행 전에 runner와 로그 parser에서 다시 확인하며, 기준이 달라졌다면 결과를 억지로 같은 표에 넣지 않고 계획을 수정한다.
- `MotionEvent`는 fixture가 합성 dispatch한다. OS callback 수신은 실제로 확인하지만 입력은 물리 터치가 아니다. callback 시각은 광학 표시 시각이나 제품 latency가 아니다.

## 대상 행렬과 사전 상태

| 대상 | 설치된 image | image revision | guest 조건 | 새 AVD 이름 |
|---|---|---:|---|---|
| Android API 35 | `system-images;android-35;google_apis;arm64-v8a` | 9 | API 35 · ARM64 · 4,096 bytes | `spinon_api35_applied_txn` |
| Android API 37.0 | `system-images;android-37.0;google_apis;arm64-v8a` | 6 | API 37.0 · ARM64 · 4,096 bytes | `spinon_api37_0_applied_txn` |

두 system image package는 이미 설치돼 있다. image revision을 고정해 실행하고 재다운로드하지 않는다. 기존 `spinon_api35_compat`와 `spinon_api37_0_compat`는 `.ini`만 남고 data directory가 없으며 `avdmanager`에서 손상으로 보고된다. 손상된 이름을 덮어쓰지 않고 새 이름으로 만들며 기존 ini와 기타 AVD 파일을 삭제·수정하지 않는다. 두 새 AVD는 같은 `pixel_6` profile을 사용한다. 이전에 복구한 API 37.1 AVD와 화면 geometry 동등성은 주장하지 않는다.

현재 `emulator-5554`는 `zl_poc`라는 다른 프로젝트의 실행 QEMU다. 이번 검증에서는 종료하거나 대상으로 삼지 않는다. Spinon API별 emulator는 각각 별도 data directory, fresh boot, `-no-snapshot-load`, `-no-snapshot-save`, host GPU로 순서대로 실행하고 fixture 뒤 정상 종료한다. 저장 공간이 실행 직전 10 GiB 아래면 생성·실행을 중단하고 원인을 기록한다.

## 실행 절차

1. 저장소가 이 계획의 source commit 후속 `origin/main`에서 분기됐는지, worktree tracked 변경이 없는지 확인한다. V8 checkout이 `tools/v8/v8-revision.txt`와 같은 revision이며 clean인지 확인한다.
2. Android SDK의 CLI 23.0 `avdmanager`, 설치된 두 package의 `source.properties`, API·tag·ABI·revision, AVD 이름 미사용 여부, 현재 process/serial과 여유 공간을 `avd-recovery/`에 저장한다.
3. 아래 명령 형식으로 기존 ini를 보존한 채 두 AVD를 새 이름으로 등록한다. `--force`를 쓰지 않는다.

```sh
sdk_dir="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
avdmanager="$sdk_dir/cmdline-tools/latest-2/bin/avdmanager"
printf 'no\n' | mise exec -- "$avdmanager" create avd \
  --name spinon_api35_applied_txn \
  --package 'system-images;android-35;google_apis;arm64-v8a' \
  --device pixel_6
printf 'no\n' | mise exec -- "$avdmanager" create avd \
  --name spinon_api37_0_applied_txn \
  --package 'system-images;android-37.0;google_apis;arm64-v8a' \
  --device pixel_6
```

4. `avdmanager list avd`, 각 ini와 `config.ini`의 image/device, 새 data directory 존재를 대조한다. 두 대상은 **순차 실행**하며 serial `emulator-5580`을 재사용한다. QEMU를 짧은 셸의 background child로 시작하지 않고 유지되는 foreground execution session으로 실행한다.

```sh
"$sdk_dir/emulator/emulator" -avd spinon_api35_applied_txn \
  -ports 5580,5581 -gpu host -no-snapshot-load -no-snapshot-save \
  -no-boot-anim -verbose
```

API 35 실행을 끝내고 process와 serial 종료를 확인한 뒤 API 37.0으로 바꿔 같은 옵션으로 실행한다. 각 boot 뒤 `sys.boot_completed=1`, API level/full version, guest `getconf PAGE_SIZE=4096`, `ro.product.cpu.abi=arm64-v8a`, `adb emu avd name`, process PID와 data directory를 직접 기록한다. API 35는 runner가 `sdk_full`을 필수 검사하지 않으므로 package metadata·guest build fingerprint도 함께 남겨 다른 image를 API 35로 오인하지 않게 한다.

5. 두 run은 각각 새 폴더에 둔다. 결과 경로는 `spec/internal/evidence/r05-android-api35-370-applied-transaction-2026-10-08/run-api35`와 `run-api37_0`다. 기존 경로가 있으면 덮어쓰지 않고 새 run suffix를 만든다.

```sh
SPINON_R05_CALLBACK_EXPECTED_API_FULL=35 \
SPINON_ANDROID_EMULATOR_SERIAL=emulator-5580 \
SPINON_V8_DIR=/tmp/spinon-r05-present-api-audit/build/v8-source/v8 \
SPINON_R05_CALLBACK_OUTPUT_DIR=spec/internal/evidence/r05-android-api35-370-applied-transaction-2026-10-08/run-api35 \
mise exec -- bash tools/verify-r05-android-callback-faults.sh
```

API 37.0은 `SPINON_R05_CALLBACK_EXPECTED_API_FULL=37.0`, `SPINON_R05_CALLBACK_OUTPUT_DIR=.../run-api37_0`로 실행한다. 실행 시점에 현재 worktree commit을 결과 manifest와 함께 기록한다.

6. runner가 저장한 source/APK digest, 전체 Logcat, 빌드·설치·launch log, environment, AVD process state와 결과 screenshot을 보존한다. 결과를 서로 섞지 않는다. 두 API source digest는 동일해야 한다. APK digest가 다르면 APK 내부 artifact를 비교해 차이를 규명하기 전 두 실행을 동일 바이너리 비교로 합치지 않는다.
7. 각 fixture 뒤 해당 Spinon AVD를 정상 종료한다. 종료 뒤 ADB serial/QEMU가 없고 AVD data directory가 남아 있음을 확인한다. 다른 프로젝트 `zl_poc`는 그대로 둔다.

## 통과 기준

각 API 대상이 모두 아래 조건을 만족해야 해당 API lifecycle fixture를 통과로 기록한다.

- 설치한 image와 guest의 API·ABI·page size가 대상 행렬과 일치하고 fresh AVD/data directory임을 증명한다.
- runner가 실제 source commit과 clean pinned V8 revision을 기록하고 debug APK build/install/launch가 성공한다.
- 실제 R08 `SurfaceView` draw와 applied transaction 후 baseline callback에서 `TransactionStats`가 non-null이고 callback count/current generation/pending 정리가 맞는다.
- callback executor에 실제 listener task가 들어간 뒤 같은 view의 실제 surface destroy/recreate를 수행한다. 이전 세대 요청은 timeout보다 먼저 취소되고 늦은 callback은 `late_after_cancel`, `current_surface=false`, `fence_signal_usable=false`로 끝난다.
- 새 generation이 증가하고 새 actual callback이 완료되며 executor가 drain된다. fixture 요약과 세 scenario·draw revision marker가 모두 PASS다.
- 100회 경합, queue overflow 회복, 60초 idle이 통과하고 마지막 pending/queue/active가 0이다. screenshot이 해당 실행의 PASS 화면이며 PID/logcat/APK digest/run manifest와 일치한다.
- API별 결과는 별도로 판정한다. 한쪽 실패를 다른 API PASS로 덮지 않는다. 두 API가 모두 통과해도 R05.3 전체 또는 Android API 전 범위를 완료 처리하지 않는다.

어느 gate에서 실패하든 해당 API 결과를 실패/미측정으로 남긴다. 재실행이 필요하면 이전 로그를 보존하고 새 output directory를 쓴다. API별 시나리오를 강제로 재현하거나 통과를 위해 fixture 의미를 바꾸지 않는다.

## 실제 실행 결과

2026-10-08 두 대상에 새 Pixel 6 profile AVD를 각각 만들고 foreground QEMU session에서 fresh boot했다. 동일 source commit `300443b868d06a4fdc59a0fb72770df4edb57ca8`, clean pinned V8 revision, 동일 APK SHA-256으로 순차 실행했다. 상세 로그·화면·AVD 상태는 [실행 보고서](../spec/internal/evidence/r05-android-api35-370-applied-transaction-2026-10-08/README.md)에 있다.

| 대상 | guest 확인 | fixture | 화면 캡처 | 판정 |
|---|---|---|---|---|
| API 35 / 4 KB | API 35, ARM64, page 4096, fingerprint `google/sdk_gphone64_arm64/emu64a:15/AE3A.240806.043/12960925:userdebug/dev-keys`; `sdk_full` 속성은 비어 있어 fingerprint와 API 35 package로 기록 | 12 checks, 0 failures, race 100회(36 callback / 64 timeout), idle 60/60초, 최종 pending/queue/active 0 | runner 화면에 PASS 표시 | debug fixture 통과 |
| API 37.0 / 4 KB | `api_full=37.0`, ARM64, page 4096, fingerprint `google/sdk_gphone64_arm64/emu64a:17/CE2A.260420.019/15611780:userdebug/dev-keys` | 12 checks, 0 failures, race 100회(81 callback / 19 timeout), idle 60/60초, 최종 pending/queue/active 0 | 첫 캡처와 재확인은 검정 화면. Settings 대조 화면은 정상이고 앱으로 돌아온 뒤 같은 PASS 문구가 보임 | fixture 통과, 최초 캡처의 표면 갱신/캡처 시점 원인은 미확정 |

두 실행의 APK SHA-256은 `d9a26b62d326d4f9310394b9eb06ffcecbb7c31e27758548c27eb1b454e40bb1`로 같고 source digest 파일도 같다. 서로 다른 race 승리 비율은 경쟁 outcome일 뿐 성능 차이가 아니다. API 35의 `SDK_INT_FULL`은 노출되지 않았으므로 fingerprint와 실제 package 경로로 대상을 교차 식별했다.

API 37.0은 AVD 전체 손상 증거가 없다. API 37.0 system image 경로와 1.5 GiB AVD data directory가 존재했고 Settings 대조 화면 및 앱 재개 후 PASS 문구가 렌더링됐다. 기존 AVD를 지우거나 system image를 재설치하지 않았다. 검증 후 `adb -s emulator-5580 emu kill`로 두 Spinon emulator를 종료했다. 종료 확인 시 유일하게 남은 실행 emulator는 다른 프로젝트의 `zl_poc` / `emulator-5554`였고 그대로 두었다. 삭제된 과거 호환 AVD는 등록 `.ini`만 남아 `.avd` data directory가 없는 상태로 기록했다.

이 결과는 synthetic fixture의 Android debug 수명주기만 확인한다. 최초 API 37.0 screenshot이 검정으로 두 번 저장된 현상은 사라진 것으로 단정하지 않는다. 캡처 후 activity 재개가 표면 갱신을 촉발했을 가능성은 있지만 첫 frame 미합성, 앱 표면 재개, 캡처 타이밍 중 무엇인지 분리 확인하지 못했다. 실기기, 제품 입력 latency, optical scanout 및 R05.3 전체 완료로 일반화하지 않는다.

구현·실행 후 20개 독립 실패 관점은 [별도 검토](../spec/internal/evidence/r05-android-api35-370-applied-transaction-2026-10-08/implementation-review.md)에 기록했다. 이는 아래 계획 검토와 다른 관점이며 서로 대체하지 않는다.

## 계획 적대적 검토 · 독립 실패 관점 20개

| # | 독립 실패 관점 | 계획에서 확인한 근거와 차단 기준 |
|---:|---|---|
| 1 | API 35만 시험하고 API 37.0 빈칸을 완료로 오인 | 두 image와 guest를 독립된 run으로 실행하고 둘 다 성공해야 두 행을 통과로 쓴다. |
| 2 | API 이름만 믿고 실제 image package가 다른 대상을 API 35로 분류 | `source.properties`, config image path, guest build fingerprint와 SDK level을 교차 확인한다. |
| 3 | API 37.0에서 `sdk_full` mismatch를 놓치고 37.2를 37.0으로 셈 | runner의 exact `sdk_full=37.0` gate와 AVD package metadata를 함께 확인한다. |
| 4 | host의 4 KB page size를 guest page size로 보고 | 두 guest에서 각각 `getconf PAGE_SIZE=4096`을 직접 읽고 기록한다. |
| 5 | ARM64 host를 guest ABI 증거로 대체 | guest `ro.product.cpu.abi=arm64-v8a`와 system image ABI를 대조한다. |
| 6 | 설치 목록만 보고 image revision이 바뀌거나 누락됨 | 각 run 전에 `source.properties`의 API/tag/ABI/revision을 복사하고 실행 기록과 연결한다. |
| 7 | 이미 손상된 AVD ini/data를 덮어써 사용자 자료를 잃음 | 기존 두 AVD와 이름을 분리한 새 이름을 쓰고 기존 경로는 수정·삭제하지 않는다. |
| 8 | 잘못된 `pixel_6` id나 profile로 서로 다른 device를 비교 | CLI device 목록, generated `hw.device.name`, 화면 해상도·density를 각 manifest에서 대조한다. |
| 9 | 활성 `zl_poc` 혹은 다른 emulator에 설치 | serial `emulator-5580`, `adb emu avd name`, QEMU `-avd` 값이 세 방향으로 일치해야 진행한다. |
| 10 | 남아 있는 stale serial이 새 QEMU로 오인 | 실행 전 5580 미사용, 실행 중 PID/name 연결, 종료 후 5580 소멸을 검사한다. |
| 11 | background launcher 수명 문제로 QEMU가 종료됐는데 fixture를 시작 | 유지되는 foreground tool session에서 boot하고 runner 직전 serial/PID를 재확인한다. |
| 12 | snapshot이 이전 boot 상태를 복원해 fresh run으로 표기 | 새 data directory와 `-no-snapshot-load -no-snapshot-save`, `sys.boot_completed=1`을 함께 남긴다. |
| 13 | emulator GPU backend가 API별로 달라짐 | 두 실행 모두 `-gpu host`와 동일 profile/device configuration을 기록한다. |
| 14 | AVD 생성/boot가 저장 공간을 압박하거나 실패 원인을 숨김 | 생성 전 10 GiB gate, image 재다운로드 금지, 생성·boot 로그와 전후 여유 공간을 보존한다. |
| 15 | 앱·V8 source revision을 API별로 다르게 빌드 | 같은 clean worktree commit과 pinned clean V8 revision을 요구하고 source digest 두 벌을 비교한다. |
| 16 | 같은 source인데 재빌드 APK 차이를 무시 | API별 APK SHA-256을 기록한다. digest가 다르면 APK 내부 파일 비교 후 원인을 밝히고 바이너리 동등성을 주장하지 않는다. |
| 17 | API 35 listener 미지원인데 callback 성공으로 오인 | API 35 공식 문서가 callback 최소 API이며 API 34 `applyTransactionToFrame` 존재를 확인한다. runtime 성공은 non-null OS `TransactionStats`로 따로 입증한다. |
| 18 | synthetic request/callback을 actual SurfaceView transaction으로 오인 | 실제 R08 draw revision, transaction apply, callback queue marker와 non-null OS stats를 요구한다. 입력은 synthetic이라고 명시한다. |
| 19 | 취소·재생성 race에서 늦은 callback이 새 세대 성공으로 재분류 | 같은 view identity와 세대 증가, timeout 전 cancel, `late_after_cancel`, current false, unusable fence를 한 request tuple로 대조한다. |
| 20 | 결과·latency·전체 지원을 과장하거나 API별 실패를 숨김 | per-API 결과·실패 로그·screenshot·digest를 분리하고 synthetic input/debug 범위, 미측정 API, R05.3 미완료를 유지한다. |

위 20개는 계획·대상·실행 절차 검토다. 구현·실행 뒤에는 source, runtime log, AVD, 캡처와 cleanup을 대상으로 별도의 20개 독립 실패 관점을 검토했다. 두 검토는 서로 대체하지 않는다.

## 결과 동기화

두 실행 뒤 [공식 상태 대장](../spec/STATUS.md), [R05.3 실행 계획](r05-input-to-presentation.md), 이 계획과 evidence README를 같은 PR에서 갱신한다. API별 실행은 성공 또는 실패를 모두 기록하고 R05.3 체크는 미완료로 둔다. API 명세 변경이나 제품 API는 없으며 결과물은 debug fixture 검증 자료다. Tailscale preview의 R05.3 기록도 동기화하되 GitHub Pages를 배포하지 않는다.

# R05.3 Android API 37.1 applied-transaction lifecycle

**실행일:** 2026-10-08 · **결과:** debug fixture 통과 · **R05.3/R05:** 미완료

## 실행 환경

| 항목 | 결과 |
|---|---|
| Android 대상 | `emulator-5580`, `sdk_gphone16k_arm64`, API 37.1, ARM64, guest page size 16,384 bytes |
| AVD | `spinon_api37_1_compat16k`; 등록 `.ini`는 남아 있지만 `.avd` data directory는 실행 전에 이미 삭제된 상태 |
| QEMU | PID 6922의 기존 실행 process를 사용. `lsof +L1`에서 삭제된 AVD backing files를 열어 둔 상태를 확인. 새 부팅·초기화 아님 |
| V8 | 요구 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`; checkout clean |
| source | base `5313dede14d326c46074bec58805418f8eaee8aa`; tracked changes 0 |
| Android build | debug APK build·설치·실행 성공. APK/source digest 및 전체 Logcat을 보존 |

## 관측 결과

실제 R08 GLES `SurfaceView`에 `SurfaceControl.Transaction`을 적용한 뒤 Android transaction-completed listener가 전달한 non-null `TransactionStats`를 확인했다.

1. **현재 surface 대조:** generation 1의 R08 draw revision 1에 transaction을 적용했다. 실제 callback 1회, stats 존재, current surface 일치, timeout/map 정리가 통과했다.
2. **destroy/recreate stale callback:** 실제 callback task가 같은 production executor queue에 들어간 뒤 같은 `R08GpuSurface`를 detach했다. `surfaceDestroyed`가 timeout보다 먼저 generation 1 request를 취소했다. reattach는 generation 2로 증가했고, 대기 중이던 실제 stats callback은 `late_after_cancel`, `current_surface=false`, `fence_signal_usable=false`로 분류됐다.
3. **새 세대 복구:** generation 2에서 draw revision 3을 제출했다. 실제 callback이 정상 1회 수신되어 executor 복구와 drain을 확인했다.

요약은 12 checks, 0 failures, timeout/callback 경합 100회( callback 58회·timeout 42회 승리), callback queue overflow 회복, 60초 idle 60개 표본이다. 종료 상태는 pending/queue/active 모두 0이다. stale callback의 fence도 signaled였지만 취소된 request에서는 unusable로 남았다.

fixture가 직접 dispatch한 `MotionEvent`를 사용했다. 물리 입력, 제품 입력 지연, optical scanout 또는 API 37.1 전체 지원을 입증하지 않는다. 다른 API의 actual applied-transaction lifecycle, 실기기와 R05.3 전체는 미완료다.

## 원본 자료

- [실행 환경·V8·AVD 상태](run-01/environment.txt)
- [QEMU PID와 삭제된 AVD backing files](run-01/avd-process-state.txt)
- [전체 Logcat](run-01/logcat.txt)
- [Android debug 빌드 로그](run-01/android-build.log)
- [설치·실행 로그](run-01/install.log) · [화면 시작 로그](run-01/launch.log)
- [APK digest](run-01/apk.sha256) · [source digest](run-01/source-files.sha256)
- [API 37.1 결과 화면](run-01/result.png)
- [실행 runner](../../../../tools/verify-r05-android-callback-faults.sh)
- [계획 및 계획 검토](../../../../plan/r05-android-api371-applied-transaction.md)
- [구현·실행 실패 경로 검토](implementation-review.md)

## AVD 복구 후 fresh boot 재실행

첫 `run-01`은 `.avd` 폴더가 이미 삭제된 뒤에도 살아 있던 QEMU에서 실행했기 때문에 cold boot가 아니었다. 사용자의 요청에 따라 Spinon API 37.1·37.2 emulator만 ADB 정상 종료했다. 그 뒤 AVD data directory가 열려 있던 두 process는 종료됐고 다른 프로젝트의 `zl_poc` PID 23588은 유지했다.

API 37.1 Google APIs 16KB ARM64 image revision 9는 이미 설치돼 있어 다시 다운로드하지 않았다. 기존 API 37.1 `config.ini`는 삭제돼 복구할 수 없었다. AVD는 CLI 23.0으로 같은 이름 `spinon_api37_1_compat16k`로 만들고 Pixel 6 profile(1080×2400, density 420)을 사용했다. 이전 하드웨어 profile과 같은 geometry라고 주장하지 않는다. 설치 command, 생성 전 ini/config, CLI 상태, boot 상태와 저장 공간은 [AVD 복구 자료](avd-recovery/)에 있다. 오래된 CLI 12.0의 `--force` 시도는 XML v4 경고와 등록되지 않은 orphan config를 남겼다. 원인을 CLI 버전 하나로 단정하지 않았고 orphan은 보존해 새 AVD와 분리했다.

첫 boot는 게스트 속성 확인까지 성공했으나 shell background child가 실행 세션 종료 뒤 사라졌다. runner가 APK build 전에 `emulator-5580` 부재를 감지해 중단했으며 이 시도는 fixture 결과가 아니다. 이 실행 실패와 보정은 [launcher 수명 기록](avd-recovery/first-launch-lifetime-failure.md)에 있다. 이후 QEMU를 지속되는 foreground execution session으로 실행했다.

| 항목 | fresh AVD run-02 |
|---|---|
| Android / image | API 37.1 (`api=37`, `sdk_full=37.1`), Google APIs 16KB image revision 9 |
| AVD / profile | `spinon_api37_1_compat16k`, Pixel 6, ARM64 |
| boot | 새 `.avd` directory, `-no-snapshot-load -no-snapshot-save`, `sys.boot_completed=1` |
| guest checks | `PAGE_SIZE=16384`, `arm64-v8a`, AVD name 일치 |
| fixture | 12 checks, 0 failures; timeout/callback 100회(52/48); idle 60/60초 |
| 종료 상태 | pending/queue/active 모두 0; 새 QEMU PID 56444와 ADB serial은 runner 뒤에도 유지 |

새 AVD에서 같은 actual R08 GLES applied-transaction 정상·stale·recovery 세 경로가 통과했다. `run-02-fresh-avd/`는 이전 `run-01/`과 별도이며 API 37.1 applied lifecycle 확인의 fresh boot 근거다. runner가 결과로 저장한 [새 화면](run-02-fresh-avd/result.png), [환경](run-02-fresh-avd/environment.txt), [QEMU/AVD 상태](run-02-fresh-avd/avd-process-state.txt), [전체 로그](run-02-fresh-avd/logcat.txt), [APK digest](run-02-fresh-avd/apk.sha256), [source digest](run-02-fresh-avd/source-files.sha256)을 보존했다. AVD 생성·boot 기록은 [첫 launcher 실패](avd-recovery/first-launch-lifetime-failure.md), [fixture 직후 상태](avd-recovery/post-run-verification.txt), [요청된 종료 후 상태](avd-recovery/post-shutdown-verification.txt), [복구 계획](../../../../plan/r05-android-api371-applied-transaction.md)에 있다. fixture 검증 후 새 API 37.1 emulator process는 정상 종료했고 AVD 설치 파일은 유지했다. 다른 프로젝트 `zl_poc`는 건드리지 않았으며, 종료 시점에도 data directory가 없는 stale AVD 등록 4개를 확인했다. 독립된 20개 실패 관점은 [fresh AVD 구현·실행 검토](implementation-review-fresh-avd.md)에 있다.

## 비교 계약

- [`SurfaceControl.Transaction.addTransactionCompletedListener`](https://developer.android.com/reference/android/view/SurfaceControl.Transaction#addTransactionCompletedListener(java.util.concurrent.Executor,java.util.function.Consumer))는 API 35부터 제공된다.
- [`SurfaceView.applyTransactionToFrame`](https://developer.android.com/reference/android/view/SurfaceView#applyTransactionToFrame(android.view.SurfaceControl.Transaction))는 API 34부터 제공된다. renderer를 호출 전에 멈추고 호출 뒤 다음 한 frame을 제출해야 transaction의 정확한 frame 관계가 정의된다.

API 37.1 한 Pixel 6-profile AVD 결과를 API 35·37.0·API 29–34, iOS, release 또는 실기기에 일반화하지 않는다. API 35·37.0 actual applied-transaction lifecycle, API 29–34 fallback, iOS device callback runtime, 실기기와 전체 R05.3은 미완료다.

# R05.3 Android callback 실패 fixture API matrix

**결과:** API 35·37.0·37.1 ARM64 emulator 통과 · 내부 fixture 전용 · R05.3/R05 미완료

## 범위와 판정

같은 Android debug APK와 callback failure fixture를 API 35 / 4KB, API 37.0 / 4KB, API 37.1 / 16KB에서 각각 새 앱 프로세스로 실행했다. 각 API에서 11개 scenario, timeout/callback 경합 100회, 종료 후 초당 1회씩 60초 유휴 관찰을 수행했다. 세 실행 모두 `failures=0`, pending·queue·active 최종값 0으로 끝났다.

이것은 `SurfaceControl.Transaction` listener 등록, 적용하지 않은 transaction의 Handler timeout·취소, callback bookkeeping의 실패 경로에 대한 **에뮬레이터 내부 검증**이다. 렌더러가 실제로 적용한 transaction의 present callback, 제품 입력→표시 latency, GPU 성능, 광학적 표시 시각, iOS device callback은 측정하지 않았다. R05.3 또는 R05 완료 근거로 사용하지 않는다.

## 실행 환경과 행렬

호스트는 Apple Silicon macOS 26.5, Android Emulator 37.2.12.0 (`16428233`)이다. `emulator -accel-check`는 Hypervisor.Framework를 보고했다. Android 문서는 macOS emulator가 기본 Hypervisor.Framework를 사용한다고 설명한다([공식 문서](https://developer.android.com/studio/run/emulator-acceleration)). 이 정보는 호스트 환경을 기록할 뿐, API별 emulator 차이의 원인을 설명하지 않는다.

| 대상 | 기기에서 읽은 환경 | AVD 실행 조건 | fixture 결과 |
|---|---|---|---|
| API 35 | SDK 35, `sdk_full` 미제공, 4,096-byte guest page, `arm64-v8a`, `sdk_gphone64_arm64` | API 35 Google APIs image revision 9. 임시 AVD `spinon_api35_compat`, command-line tools 12.0, GPU host, serial `emulator-5590`. 새 앱 PID 3565. | 11/11 PASS. 100 race: callback 42, timeout 58. timeout/cancel·executor overflow·60초 idle 통과. |
| API 37.0 | SDK 37, full version 37.0, 4,096-byte guest page, `arm64-v8a`, `sdk_gphone64_arm64` | API 37.0 Google APIs image revision 6, image metadata dependency `emulator#36.5.11`. 임시 AVD `spinon_api37_0_compat_v23`, command-line tools 23.0, GPU host, ports 5582/5583, serial `emulator-5582`. 새 앱 PID 4320. | 11/11 PASS. 100 race: callback 38, timeout 62. timeout/cancel·executor overflow·60초 idle 통과. |
| API 37.1 | SDK 37, full version 37.1, 16,384-byte guest page, `arm64-v8a`, `sdk_gphone16k_arm64` | Google APIs 16KB image revision 9. 기존 실행 중 `spinon_api37_1_compat16k` emulator process, GPU host, serial `emulator-5580`. 삭제된 backing AVD directory 때문에 emulator는 재부팅하지 않았다. fixture runner가 앱을 force-stop 후 새 앱 PID 4918로 실행했다. | 비교 기준으로 채택한 동일-source run 11/11 PASS. 100 race: callback 57, timeout 43. timeout/cancel·executor overflow·60초 idle 통과. |

API 35·37.0 emulator startup log 네 개는 내용은 유지하고 줄바꿈과 줄 끝 공백만 정규화해 저장했다.

API 35에서는 `ro.build.version.sdk_full`이 비어 있어 SDK base level과 guest page size로만 분류했다. API 37.0·37.1은 full version 값을 각각 보존해 합치지 않았다. page size는 macOS 호스트가 아니라 guest의 `getconf PAGE_SIZE`에서 읽었다.

세 비교 run은 같은 commit `e2339191c6353325a7d297b86dfc195c9e528ae4`, tracked dirty count 4, 같은 다섯 fixture source-file hash, 같은 APK SHA-256 `fbb8ea94ec2fad8cffd27afd79e4548fac32ad1fb18316aea8b8e1c8c8c0ba7d`를 기록했다. API 37.1의 첫 run은 실행 도중 기존 GPU-host emulator log가 추가되어 dirty count 3이었다. 이를 세 run 간 비교 기준으로 쓰지 않고, 동일한 count 4와 같은 source/APK hash가 확인된 `api-37.1-source-match/`를 비교 결과로 채택했다. 실행 직후 dirty count는 동적으로 기록된 tracked emulator log를 포함하며 source hash 목록은 다섯 fixture source를 직접 구분한다.

## API 37.0 emulator 시작 실패와 성공 조건

API 37.0에서 command-line tools 12.0으로 만든 임시 AVD는 두 번 부팅되지 않았다. GPU host·port 5590 실행은 QEMU exit 139와 hang 감시 로그를 남겼다. GPU software·port 5582 실행도 Android 초기화 중 QEMU exit 139로 끝났다. 이 두 로그는 `attempt-legacy-cli-host-gpu.log`와 `attempt-legacy-cli-software-gpu.log`에 있다. 두 시도 모두 앱 fixture를 실행하지 않았으며 API 37.0 callback 결과로 세지 않는다.

다음 시도에서는 command-line tools 23.0으로 별도 임시 AVD를 만들고 GPU host·권장 port 5582/5583을 사용해 API 37.0을 정상 부팅한 뒤 fixture를 통과시켰다. 도구 버전·GPU 설정·port·AVD 생성 파일이 실패 시도와 모두 같지 않으므로 어느 한 차이가 원인이라고 단정하지 않는다. 성공 startup log와 실패 원본을 나란히 보존한다.

API 35·37.0의 임시 AVD는 결과 저장 후 Android `avdmanager delete avd`로 제거했다. 별도 임시 AVD home은 비어 있고, 기본 `~/.android/avd`의 기존 `.ini`나 사용자가 켜둔 emulator process는 수정·종료하지 않았다. system image와 Android SDK는 남아 있다.

## 원본 실행 자료

- API 35: [`environment.txt`](api-35/environment.txt) · [`android-build.log`](api-35/android-build.log) · [`install.log`](api-35/install.log) · [`launch.log`](api-35/launch.log) · [`logcat.txt`](api-35/logcat.txt) · [`result.png`](api-35/result.png) · [`source-files.sha256`](api-35/source-files.sha256) · [`apk.sha256`](api-35/apk.sha256) · [emulator startup](api-35/emulator-startup.log)
- API 37.0: [`environment.txt`](api-37.0/environment.txt) · [`android-build.log`](api-37.0/android-build.log) · [`install.log`](api-37.0/install.log) · [`launch.log`](api-37.0/launch.log) · [`logcat.txt`](api-37.0/logcat.txt) · [`result.png`](api-37.0/result.png) · [`source-files.sha256`](api-37.0/source-files.sha256) · [`apk.sha256`](api-37.0/apk.sha256) · [successful emulator startup](api-37.0/emulator-startup.log) · [first failed startup](api-37.0/attempt-legacy-cli-host-gpu.log) · [software-rendering failed startup](api-37.0/attempt-legacy-cli-software-gpu.log)
- API 37.1 same-source comparison: [`environment.txt`](api-37.1-source-match/environment.txt) · [`android-build.log`](api-37.1-source-match/android-build.log) · [`install.log`](api-37.1-source-match/install.log) · [`launch.log`](api-37.1-source-match/launch.log) · [`logcat.txt`](api-37.1-source-match/logcat.txt) · [`result.png`](api-37.1-source-match/result.png) · [`source-files.sha256`](api-37.1-source-match/source-files.sha256) · [`apk.sha256`](api-37.1-source-match/apk.sha256)
- Runner 거부 제어: [실기기 serial](final-negative-physical-serial.log) · [허용되지 않은 API](final-negative-api-tuple.log) · [SDK/page-size 불일치](final-negative-api-mismatch.log) · [기존 결과 폴더 덮어쓰기](final-negative-output-overwrite.log). 네 경로 모두 build/install/launch 전에 거부했고 새 결과 폴더를 만들지 않았다.

## 구현 후 적대 검토 · 새 실패 관점 20개

| # | 공격 관점 | 확인과 판정 |
|---:|---|---|
| 1 | AVD 이름만 보고 guest API를 단정 | runner가 기기에서 `SDK_INT`를 읽었다. API 35·37.0·37.1 관찰값이 각각 35·37·37이었다. 통과. |
| 2 | 37.0과 37.1을 동일 API 37로 합산 | 두 run의 `ro.build.version.sdk_full`을 37.0·37.1로 보존했다. 통과. |
| 3 | 호스트 page size를 guest 값으로 오인 | 각 emulator의 `getconf PAGE_SIZE`를 저장했다. 4,096·4,096·16,384 bytes였다. 통과. |
| 4 | API 35의 빈 full-version 속성을 임의로 `35.0` 처리 | 원본 `api_full`은 `unavailable`로 기록하고 SDK base 35와 4KB로만 gate했다. 통과. |
| 5 | 16KB AVD에서 앱 동작 결과를 16KB 기준으로 확인하지 않음 | API 37.1 `getconf PAGE_SIZE=16384`를 runner가 검사했고 해당 run만 16KB로 분류했다. 통과. |
| 6 | stale `.ini` 목록을 실제 부팅 가능한 AVD 목록으로 간주 | 실제 `.avd` directory 부재를 확인했다. 임시 AVD를 별도 `ANDROID_AVD_HOME`에 만들고 제거했으며 기본 목록은 바꾸지 않았다. API 37.1은 기존 live process이며 emulator 재부팅은 아니라고 표시했다. 통과. |
| 7 | QEMU 시작 실패를 앱 fixture failure/pass로 계수 | API 37.0의 exit 139 두 건은 별도 startup log이며 fixture를 실행하지 않았다. 세 번째 정상 부팅 뒤의 결과만 matrix에 포함했다. 통과. |
| 8 | 성공한 API 37.0 실행의 단일 조건을 미검증 원인으로 과장 | 실패·성공 실행 사이 여러 조건이 달라 원인 미확정으로 기록했다. 통과. |
| 9 | emulator serial allowlist를 우회해 실기기 포함 | `physical-serial` negative control은 기기 호출 전에 거부됐다. 통과. |
| 10 | 허용되지 않은 API tuple이 build 단계까지 진입 | API 37.3 negative control은 allowlist에서 거부됐다. 통과. |
| 11 | API 37.1 기기를 API 37.0으로 오분류해 실행 | SDK full version/page-size mismatch control이 build 전에 거부됐고 결과 폴더도 생성하지 않았다. 통과. |
| 12 | 기존 결과 폴더에 성공물을 덮어씀 | output-overwrite control이 build 전에 거부됐다. 기존 API 37.2 evidence는 보존됐다. 통과. |
| 13 | 서로 다른 앱 binary를 API별로 비교 | 세 선택 run의 APK SHA-256이 모두 동일하다. 통과. |
| 14 | source commit이 같지만 fixture source가 바뀐 run을 혼합 | 다섯 Android source/runner hash를 run별로 저장하고 세 선택 run을 byte-for-byte 비교했다. 통과. |
| 15 | tracked worktree count 차이를 숨김 | API 37.1 첫 run(count 3)은 비교 기준에서 제외하고 count 4인 source-match run을 선택했다. run별 dirty metadata는 남아 있다. 통과. |
| 16 | 실제 Handler timeout을 synthetic state call로 대체 | 실제 미적용 `SurfaceControl.Transaction` listener를 등록했고 timeout 1·callback 0·transaction closed를 Logcat에서 각각 확인했다. 통과. |
| 17 | 취소 뒤 timeout Runnable 또는 transaction이 남음 | 실제 cancel PASS 조건은 cancelled state·pending 없음·timeout 없음과 `closeFailureTransaction()` 성공을 모두 포함한다. 통과 로그와 fixture 조건을 대조했다. |
| 18 | callback/timeout 경합에서 terminal 두 개 또는 표본 누락 | API마다 100 iteration, winner 합계 100, pending 0을 확인했다. 각 회차의 callback/timeout 승자는 하나였다. 통과. |
| 19 | bounded executor 포화 뒤 callback 누락·queue 잔류 | worker 1개·queue 64개 조건에서 caller-runs 1, queue drained, fence unusable, 최종 queue/active 0을 확인했다. 통과. |
| 20 | idle·screenshot·fixture 결과를 제품 안정성이나 성능으로 확대 | 세 run 모두 60/60 idle sample과 summary PASS, 1080×2400 화면 capture를 보존했다. 문서에는 emulator 내부 fixture만 결론 내고 release runtime·실기기·GPU 성능·광학 scanout은 미검증으로 남겼다. 통과. |

## 남은 범위

Android API 29–34 fallback, renderer가 적용한 실제 transaction의 late callback, Android lifecycle 변경·사용자 입력과 callback의 동시성, iOS device callback, 실기기, Release APK runtime, 성능 및 광학 scanout은 이 matrix에 포함하지 않았다. API별 fixture 통과만으로 API 지원 계약이나 R05.3/R05 완료를 선언할 수 없다.

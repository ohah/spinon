# R05.3 Android 16KB 재빌드 및 후속 구현 공격 검토

**실행일:** 2026-10-08 · **범위:** Android ARM64 V8 재빌드, APK 정렬, API 37.2 16KB AVD 로더 확인 · **결론:** 새 디버그 APK의 16KB 정적 기준과 네이티브 라이브러리 로드는 통과. R05.3/R05 계측은 미완료.

## 재현 환경과 결과

- V8: `7b50b62cb18f28617959e8452e2cd18195b38bcf` (`tools/v8/v8-revision.txt` 고정값), shallow checkout.
- depot_tools: `41c9bd890277c2f551499d171d215dfdf5dab97d` 고정값. V8 `.gclient`의 대상은 Android와 iOS다.
- Android NDK: `27.1.12297006`. 환경에 다른 NDK 경로가 지정되어도 고정 SDK 버전을 선택했다.
- V8 GN: Android ARM64, Release, `v8_jitless=false`(JIT 허용), monolithic archive.
- 전체 새 checkout은 격리 임시 경로에서 `fetch --no-history`와 DEPS 동기화를 끝내고 같은 V8 SHA를 확인했다. 별도 full history checkout은 만들지 않았다.
- `SPINON_V8_JOBS=4 bash tools/v8/build-android-macos.sh`: 종료 코드 0, Ninja 2,975/2,975, `libv8_monolith.a` 생성.
- `bash tools/build-android.sh`: 종료 코드 0, arm64-v8a `libspinon_bootstrap.so` 링크.
- `mise exec -- env ANDROID_HOME="$HOME/Library/Android/sdk" ANDROID_SDK_ROOT="$HOME/Library/Android/sdk" ./gradlew assembleDebug`: `BUILD SUCCESSFUL`, 37개 Gradle task.
- Gradle은 Android Gradle Plugin 8.13.2가 compile SDK 37.2까지 검증되지 않았다는 경고를 출력했다. Debug APK 빌드와 API 37.2 AVD 설치는 성공했지만, AGP의 API 37.2 지원 검증이나 release APK 빌드는 이번 범위에 없다.
- 빌드 스크립트가 임시로 바꾼 V8 `BUILDCONFIG.gn`은 복원됐고 V8 소스 checkout은 clean이다.

새 `.so`의 네 PT_LOAD segment는 모두 `Align=0x4000`이다. GNU_RELRO는 `VirtAddr=0x2d05e60`, `MemSiz=0x0d21a0`이며 끝 주소 `0x2dd4000`은 16KB 경계에 있다. 기존 APK에서 확인된 끝 주소 `0x2dd5000`의 4KB 나머지는 새 링크에서 사라졌다. `zipalign -c -P 16 -v 4`도 APK 내부 네이티브 라이브러리에서 성공했다. Gradle이 디버그 심볼을 제거하므로 빌드 전후 `.so` SHA는 다르며, APK에서 추출한 실제 파일에도 ELF 검사를 반복했다.

API 37.2 ARM64 AVD `spinon_api37_2`에서 `getconf PAGE_SIZE=16384`를 확인하고 APK를 설치·실행했다. `nativeloader`는 APK 내부 `libspinon_bootstrap.so`를 `ok`로 로드했고, 앱 프로세스는 살아 있었다. 앱 로그는 JS 부트스트랩이 메인 스레드 밖에서 실행된 것(`is_main_thread=false`)과 `nodes=2`, `document_revision=25`, `render_tree_revision=15`를 기록한다.

이 기본 부트스트랩 경로는 시각 렌더링을 하지 않는다. 캡처된 흰 화면은 화면 표시 성공 증거가 아니며, 이 결과는 네이티브 라이브러리 로드와 JS 문서 fixture 실행까지만 증명한다. 같은 logcat에 에뮬레이터 `android.hardware.uwb` 서비스의 별도 SIGABRT가 있었으나 앱 PID와 구분했다. 이 실행은 R05 fence, 실제 픽셀 표시, 성능 또는 실기기 동작을 검증하지 않는다.

재현 원본: [checkout·설정 거부 검증](r05-android-api-compatibility-2026-10-08/android-v8-checkout-validation.txt), [빌드 실행 기록](r05-android-api-compatibility-2026-10-08/android-v8-build-run.txt), [빌드 산출물 SHA](r05-android-api-compatibility-2026-10-08/android-v8-build-artifacts.sha256), [ELF 검사](r05-android-api-compatibility-2026-10-08/android-v8-elf-audit.txt), [APK 16KB 정렬](r05-android-api-compatibility-2026-10-08/android-v8-apk-zipalign.txt), [API 37.2 AVD 상태와 앱 PID](r05-android-api-compatibility-2026-10-08/api37_2-16k-bootstrap-smoke.log), [앱 logcat 발췌](r05-android-api-compatibility-2026-10-08/api37_2-16k-bootstrap-smoke-logcat-filtered.txt), [앱 전용 logcat](r05-android-api-compatibility-2026-10-08/api37_2-16k-bootstrap-app-logcat.txt), [전체 logcat](r05-android-api-compatibility-2026-10-08/api37_2-16k-bootstrap-smoke-logcat-full.txt), [기본 화면 캡처](r05-android-api-compatibility-2026-10-08/api37_2-16k-bootstrap-blank-screen.png).

## 공격 검토 중 발견해 수정한 문제

- depot_tools의 초기 bootstrap 전에 `fetch`를 호출하면 새 환경에서 `python3_bin_reldir.txt`가 없어 실패했다. 고정 depot_tools 경로를 PATH에 추가한 뒤 `ensure_bootstrap`을 먼저 실행한다.
- 전체 V8 이력을 받던 첫 checkout은 약 1.77GiB pack 전송에서 장시간 정체됐다. 새 fetch와 모든 gclient 동기화에 `--no-history`를 적용하고 정확한 고정 SHA를 shallow fetch한다.
- `.gclient`를 URL 부분 문자열만으로 확인하면 URL이 주석에 있거나 여러 `solutions` 할당 중 하나에만 있어도 잘못 통과할 수 있었다. Python AST에서 단일 `solutions` 할당과 V8 단일 저장소 목록을 확인하도록 바꿨다.
- `target_os` 정규식이 개행을 포함해 다중 행 목록을 단일 행처럼 받아들였다. 실제 부정 fixture에서 이 동작을 재현했다. 줄바꿈을 허용하지 않도록 고치고, AST 기준 중복·비표준 `target_os`는 덮어쓰지 않고 실패하도록 했다.

## 구현 적대 검토 · 독립 실패 관점 20개

| # | 공격 관점 | 검증과 결과 |
|---:|---|---|
| 1 | depot_tools bootstrap 자료가 없어 첫 fetch가 멈추는가 | 최초 실행에서 `python3_bin_reldir.txt` 오류를 재현했다. 현재는 `ensure_bootstrap`을 fetch 전에 호출한다. |
| 2 | V8 전체 Git 이력을 받아 디스크·네트워크를 과도하게 쓰는가 | full-history 전송이 약 1.77GiB 지점에서 정체된 것을 확인했다. 새 fetch와 동기화 경로를 shallow/no-history로 바꿨다. |
| 3 | 비어 있는 새 작업 디렉터리에서 checkout이 실제로 완주하는가 | 별도 임시 루트에서 최종 고정 SHA와 DEPS 동기화까지 종료 코드 0으로 완료했다. checkout은 shallow다. |
| 4 | 중간에 끊긴 fetch의 `.gclient` 때문에 재시작이 불가능한가 | 부분 checkout 복구 경로가 기존 `.gclient`를 이용해 pinned gclient sync를 끝내는 것을 확인했다. |
| 5 | 다른 저장소를 가리키는 `.gclient`를 V8로 덮어쓰는가 | 잘못된 URL 입력에서 중단했고 파일 SHA-256이 전후 동일했다. |
| 6 | 주석에만 V8 URL을 넣어 잘못된 설정을 통과시키는가 | URL이 주석에만 있고 실제 `solutions`는 다른 저장소인 입력을 거부했다. 원본 해시도 유지됐다. |
| 7 | 중복 `solutions` 할당이나 추가 저장소가 있는 설정을 허용하는가 | 중복 할당과 V8+다른 저장소 목록을 각각 거부했다. 파일은 바뀌지 않았다. |
| 8 | 문법 오류나 계산식이 든 `.gclient`를 실행·덮어쓰는가 | `ast.parse`/`ast.literal_eval`만 사용하며, 문법 오류·비리터럴 값은 동기화 전에 거부하는 코드를 확인했다. |
| 9 | 다중 행 `target_os`가 축약되어 사용자의 OS 목록을 잃는가 | 최초 공격 입력에서 정규식 결함을 재현했다. 수정 후 동일 입력은 안전하게 거부되고 파일 해시는 동일하다. |
| 10 | 문자열 등 목록이 아닌 `target_os`를 정상 설정으로 오인하는가 | `target_os = 'android'` 입력을 거부했고 원본을 유지했다. |
| 11 | 중복 `target_os` 중 마지막 값이 Android/iOS를 덮어쓰는가 | 두 번 할당된 입력을 거부했고 원본 해시는 동일했다. |
| 12 | 지원 대상 OS를 추가하면서 기존 한 줄 목록의 값을 지우는가 | `set(configured) | {android, ios}`로 합집합을 만들고 정렬한다. 실제 생성 결과에는 Android와 iOS가 모두 있다. |
| 13 | 기존 V8 checkout의 다른 SHA를 조용히 이동시키는가 | 기존 checkout은 SHA가 다르면 중단하는 가드를 확인했고 실제 빌드 입력 SHA는 고정값과 일치한다. |
| 14 | 기존 V8 소스의 사용자 수정이 동기화로 덮이는가 | gclient sync 전에 `git status --porcelain`을 검사한다. 실제 checkout도 clean이었다. |
| 15 | pin 적용 후 마지막 DEPS sync가 다시 전체 이력을 받는가 | 모든 `gclient sync` 명령에 `--no-history`가 있는지 확인했고 새 경로에서 완주했다. |
| 16 | 잘못된 depot_tools 버전으로 V8을 준비하는가 | depot_tools SHA `41c9bd…`를 확인하고 그 checkout의 `ensure_bootstrap`을 실행했다. |
| 17 | 환경변수가 다른 NDK를 가리켜 다른 링크 결과가 나오는가 | 불일치 경고 뒤 고정 NDK `27.1.12297006`을 선택한 빌드 로그를 확인했다. |
| 18 | Android 산출물이 JIT 없는 빌드거나 ARM64가 아니며, compileSdk 경고를 감추는가 | GN은 Android ARM64 Release, `v8_jitless=false`다. Debug 빌드는 성공했으나 AGP 8.13.2의 compile SDK 37.2 미검증 경고를 기록했고 release 빌드로 확대하지 않았다. |
| 19 | macOS cross-build 임시 수정이 V8 checkout에 남는가 | 빌드 종료 후 `BUILDCONFIG.gn`을 HEAD와 대조했고 checkout `git status`는 clean이다. |
| 20 | LOAD/RELRO, ZIP 정렬, 16KB 로더 결과를 서로 혼동하는가 | 네 PT_LOAD와 GNU_RELRO 끝 주소를 별도로 검사하고 `zipalign -P 16`을 통과시켰다. API 37.2 16KB AVD의 native loader도 `ok`; 단, 흰 기본 화면은 시각 렌더 성공으로 세지 않았다. |

## 남은 범위

이 수정으로 기존 바이너리의 16KB GNU_RELRO 결함은 새 디버그 APK에서 재현되지 않는다. Gradle의 AGP 8.13.2/compile SDK 37.2 경고와 release APK 빌드는 남아 있다. R05.3 자체는 아직 완료가 아니다. 이 실행에는 표시 callback과 실제 surface present 상관, callback 실패·timeout 고장 주입, iOS 표시 callback runtime, release 성능, 실기기 및 광학 계측이 없다. API 호환 행렬의 과거 실행 결과와 해당 시점 20개 검토는 [Android API 호환성 보고서](r05-android-api-compatibility-2026-10-08.md)를 따른다. 전체 R05 상태는 [상태 대장](../../STATUS.md), 기준은 [R05.3 계획](../../../plan/r05-input-to-presentation.md)에서 관리한다.

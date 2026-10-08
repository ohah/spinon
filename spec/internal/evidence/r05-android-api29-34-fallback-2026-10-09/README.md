# Android API 29·34 R05 fallback 최초 실행 · 2026-10-09

이 문서는 수정 전 APK의 최초 실행 기록이다. API 29에서 발견한 unwind link 실패는 후속 작업으로 수정·재검증했다. 현재 결과는 아래 [후속 수정](#후속-api-29-unwind-수정-및-재검증-2026-10-09)과 [수정 실행 근거](../r05-android-api29-unwind-link-2026-10-09/README.md)를 참고한다.

## 판정

| 대상 | 결과 |
|---|---|
| API 29 / Google APIs ARM64 / Pixel 2 | APK 설치는 됐지만 앱 시작이 `UnsatisfiedLinkError`로 종료됐다. `libspinon_bootstrap.so`의 `_Unwind_Resume`를 동적 링커가 찾지 못했다. R05 진단과 JS bootstrap은 실행되지 않았다. |
| API 34 / Google APIs ARM64 / Pixel 6 | JS bootstrap 성공. async fence 진단은 `api_below_35`로 비활성화됐고 Activity가 종료됐다. |
| API 34 / present-fence GLES fallback | `api_below_35`로 표시 신호는 unavailable 처리됐지만 GLES draw와 화면 색상 변경은 계속됐다. |
| API 34 / FrameTimeline GLES fallback | `api_below_35`로 unmatched 처리됐지만 GLES draw와 화면 색상 변경은 계속됐다. |

API 29에서 발견한 native-link blocker 때문에 해당 API의 R05 flag 세 가지는 실행하지 않았다. 모두 MainActivity의 native library 초기화 뒤에 진입하므로 반복해도 같은 linker crash만 재현한다. API 29–34 fallback gate는 미통과이며 API 30–33은 실행하지 않았다.

## 테스트 조건

- 두 새 AVD는 각각 `spinon_r05_api29_fallback` (Pixel 2) 및 `spinon_r05_api34_fallback` (Pixel 6) 이름으로 생성했다. 기존 AVD와 다른 프로젝트 `zl_poc`는 바꾸거나 삭제하지 않았다.
- 이미지: `system-images;android-29;google_apis;arm64-v8a` revision 13, `system-images;android-34;google_apis;arm64-v8a` revision 14.
- 둘 다 ARM64 / 4 KB page size / Android Emulator 37.2.12 / SwiftShader software graphics를 사용했다. 이 환경은 성능 비교용이 아니다.
- 동일 debug APK SHA-256: `608fbdf9279ddded605c7129fb0123ce32a65d7452f6ff1b36702b9eed7b198e`. API 36 실기기 재연결 실행과 같은 기존 PR #79 artifact이며 이 branch commit에서 새로 빌드하지 않았다.
- APK manifest/minSdk는 29, targetSdk는 36이다. 모든 ADB 명령은 emulator serial에만 보냈고 실기기는 연결 상태로 두었다.
- R05 입력은 ADB synthetic tap 1회씩이다. 손가락 입력으로 집계하지 않는다.

## 실패 원인 추적

API 29의 `MainActivity` class initializer에서 `System.loadLibrary()`가 실패했다. 앱 로그에는 `SPINON_R05_` marker가 없고 crash buffer는 `_Unwind_Resume`를 직접 지목한다. APK의 `libspinon_bootstrap.so`는 `_Unwind_Resume` 및 다른 `_Unwind_*` 심볼을 정의하지 않은 채 남겼으며 `DT_NEEDED`에는 동적 `libunwind` 의존성이 없다. [Android native link 명령](../../../../tools/build-android.sh)도 `-nostdlib++ --unwindlib=none`으로 연결한다. 따라서 확인된 현상은 Android API 29의 loader가 제공하지 않은 unwind 심볼을 앱이 import해 시작하지 못한 것이다. API 34에서 같은 APK가 시작하는 현상은 확인했지만 API별로 심볼을 제공한 system library의 상세 원인은 이 실행에서 분리하지 않았다.

수정 방향은 Android API 29 이상에서 unwind 참조를 충족하는 링크 구성을 정하고 새 APK로 API 29 cold-start 및 R05 fallback을 재검증하는 것이다. 이 문서는 링크 수정을 적용했다고 주장하지 않는다.

## API 34 상세 결과

- 기본 실행: `SPINON_BOOTSTRAP_RESULT=nodes=2 last_node=8 tag=text text=이벤트:7` 성공, R05 marker 없음. 기본 화면은 흰색 bootstrap 화면이다.
- async fence: `SPINON_R05_FENCE_WAIT status=UNAVAILABLE reason=api_below_35 api=34`; 앱은 정상 종료했고 crash/ANR은 관측되지 않았다.
- present-fence: GLES `input_seq=1`, `outcome=unavailable reason=api_below_35`, `draw_requested=true`, 이후 `draw_seq=3`, R08 activation count 1. 시작/완료 screenshot은 파랑→빨강 변화를 보여준다.
- FrameTimeline: GLES `input_seq=1`, `outcome=unmatched reason=api_below_35`, `draw_requested=true`, 이후 `draw_seq=2`, drain queue가 예약됐고 R08 activation count 1. screenshot에서 같은 색 변경을 확인했다.
- API 34 event timestamp는 코드가 `nanosecond_representation`으로 기록했다. 단위 표기를 정확도 보장으로 해석하지 않는다.

## 파일

- API 29: [환경](api29/environment.txt), [process/focus](api29/process.txt), [link 조사](api29/native-link-investigation.txt), [bootstrap/crash log](api29/crash-buffer.txt) · [launcher screenshot](api29/after-crash-launcher.png)
- API 34: [환경](api34/environment.txt), [bootstrap log](api34/bootstrap.logcat.txt) · [async fence](api34/async-fence.logcat.txt) · [async 종료 화면](api34/async-fence.png) · [present-fence](api34/present-fence.logcat.txt) · [FrameTimeline](api34/frame-timeline.logcat.txt)
- API 34 화면: [bootstrap](api34/bootstrap.png), [present-fence 전](api34/present-fence-before.png) / [후](api34/present-fence-after.png), [FrameTimeline 전](api34/frame-timeline-before.png) / [후](api34/frame-timeline-after.png)
- [실행 후 독립 실패 검토 20관점](implementation-review.md) · [SHA256SUMS](SHA256SUMS)

이 결과는 R05 API compatibility fallback의 부분 실행 증거다. API 29 native startup, API 30–33, real finger input, device renderer, latency·VSync·scanout, R05.3 전체는 미완료다.


## 후속 API 29 unwind 수정 및 재검증 · 2026-10-09

이 최초 실행에서 API 29 cold start를 막은 _Unwind_Resume는 고정 Android NDK의 AArch64 libunwind archive를 정적으로 연결해 수정했다. 새 Debug APK hash 1639d939a34c7a7dfcf99b53b906d73c12951eae99b87757ecc2e8589b1e572e로 API 29·34 ARM64 AVD를 재검증했다. 두 API 모두 native load와 JS bootstrap이 성공했고 API 35 전용 callback gate는 api_below_35로 닫히면서 GLES fallback draw와 화면 색상 변경을 유지했다. 세 R05 진단을 각각 새 process에서 실행했고 fatal/linkage/verifier/ANR은 관측하지 않았다.

새 API 36 실기기에서도 동일 APK load와 WGPU/Vulkan synthetic regression 13/13을 확인했지만, 이 결과를 API 29 실행 근거로 대신하지 않는다. API 30–33과 direct finger input은 미검증이다. 새 APK·ELF·실기기 자료와 구현 후 검토는 [후속 실행 근거](../r05-android-api29-unwind-link-2026-10-09/README.md)에 기록했다.

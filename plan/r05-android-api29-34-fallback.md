# R05.3 · Android API 29·34 present-fence fallback 계획

**상태:** 최초 API 29 실행은 native unwind link에서 차단됐으나 후속 정적 libunwind link 수정 후 API 29·34 AVD fallback 재검증 통과 · API 30–33 미측정 · R05.3 미완료

**상위:** [R05 계측 계약](r05-input-to-presentation.md) · [Android callback API 행렬](r05-android-callback-api-matrix.md) · [R05 상태 대장](../spec/STATUS.md)

## 목적과 범위

R05 transaction present-fence와 FrameTimeline 경로의 최소 지원 경계는 API 35다. 앱 `minSdk`는 API 29다. 이 실행은 API 29와 API 34 두 경계 AVD에서 같은 고정 Debug APK를 실행해 구버전에서 bootstrap이 동작하고, API 35 전용 진단은 `api_below_35`로 닫히며, 프레임 제출 fallback이 계속 동작하는지 확인한다.

최초 APK에서 API 29 native load가 막힌 현상은 아래에 역사적 실패로 보존한다. 후속 libunwind link 수정과 같은 API 29·34 AVD의 재검증 결과는 이 문서 끝의 follow-up에 추가했다. 두 경계 실행은 API 30–33의 전체 호환성이나 모든 기기·GPU backend 지원을 대신하지 않는다. 새 공개 API나 런타임 구현을 추가하지 않는다. 지원 확대, latency·VSync·scanout, 실제 손가락 입력은 이 작업의 범위가 아니다.

## 비교 모델

- `platforms/android/app/build.gradle.kts`의 `minSdk=29`가 APK 설치 하한이다.
- `R05PresentFenceWaitExperiment.enable()`은 `Build.VERSION.SDK_INT < 35`를 검사한 뒤 `api_below_35`를 반환한다. API 35 전용 `SyncFence` 대기 경로를 호출하지 않는다.
- `R05PresentFenceProbe.submitNextFrame()`과 `R05FrameTimelineProbe.submitNextFrame()`은 API 35 미만에서 `api_below_35`를 기록하고 원래 `submit.run()`을 계속한다.
- 같은 설치 APK가 PR #79 고정 Debug artifact SHA-256 `608fbdf9279ddded605c7129fb0123ce32a65d7452f6ff1b36702b9eed7b198e`와 일치해야 한다. 이 APK를 다시 빌드하거나 서명·설치 상태를 바꾸지 않는다.

## 실행

API 29 `system-images;android-29;google_apis;arm64-v8a` 13.0.0과 API 34 `system-images;android-34;google_apis;arm64-v8a` 14.0.0 이미지를 사용해 각각 `spinon_r05_api29_fallback`(Pixel 2)과 `spinon_r05_api34_fallback`(Pixel 6) 전용 AVD를 만든다. 기존 AVD 정의·데이터 및 타 프로젝트 `zl_poc`는 수정·삭제하지 않는다. 각 AVD에서 같은 APK로 다음 세 조건을 순서대로 새 process에서 실행한다.

1. **기본 bootstrap:** extra 없이 cold launch하고 `SPINON_BOOTSTRAP_RESULT`와 fatal/ANR 부재를 확인한다. R05 probe 실행이나 GPU 표시 성공으로 확대하지 않는다.
2. **비동기 fence gate:** `spinon_r05_async_fence_wait=true`로 실행한다. `SPINON_R05_FENCE_WAIT status=UNAVAILABLE reason=api_below_35 api=<실제 API>` 뒤 Activity가 종료되고, `NoClassDefFoundError`, `NoSuchMethodError`, `VerifyError`, fatal 또는 ANR이 없어야 한다.
3. **present-fence fallback:** `spinon_r05_present_fence=true`, `spinon_r05_gles_control=true`만 설정해 GLES R08 경로를 새 process로 실행한다. 중앙에 ADB synthetic tap 1회를 보내 `SPINON_R05_PRESENT_FENCE ... outcome=unavailable reason=api_below_35`와 `draw_requested=true` submit을 확인한다.
4. **FrameTimeline fallback:** 앞 조건과 분리된 새 process에서 `spinon_r05_frame_timeline_join=true`, `spinon_r05_gles_control=true`만 설정한다. 중앙에 ADB synthetic tap 1회를 보내 `SPINON_R05_FRAME_TIMELINE ... reason=api_below_35`와 `draw_requested=true` submit을 확인한다. 두 probe flag를 함께 켜면 FrameTimeline branch가 먼저 선택되어 present-fence 분기를 직접 검증하지 않으므로 합치지 않는다. ADB 입력은 물리 touch 표본으로 계산하지 않는다.

API 29의 event timestamp는 millisecond fallback, API 34는 nanosecond representation으로 기록되는지 보존하되 precision을 accuracy로 해석하지 않는다. 각 조건은 PID별 logcat을 분리하고 AVD의 guest API, system image, APK hash, 화면 캡처를 기록한다. 결과를 얻으면 AVD를 정상 종료하고 Android 실기기는 연결 상태로 둔다. 새로 만든 AVD는 후속 Android 호환성 검증에 재사용할 수 있도록 보존한다.

## 사전 판정

| 항목 | API 29 기대 | API 34 기대 |
|---|---|---|
| 기본 cold launch | JS bootstrap 결과, API 29 설치·실행, fatal 없음 | JS bootstrap 결과, API 34 실행, fatal 없음 |
| 비동기 fence gate | `UNAVAILABLE reason=api_below_35 api=29` 뒤 Activity 종료 | `UNAVAILABLE reason=api_below_35 api=34` 뒤 Activity 종료 |
| present-fence fallback | present-fence marker `api_below_35`; GLES draw request와 activation 1회 유지 | 같은 별도 condition을 통과 |
| FrameTimeline fallback | FrameTimeline marker `api_below_35`; GLES draw request와 activation 1회 유지 | 같은 별도 condition을 통과 |
| 부정 대조 | API 35 전용 callback·fence 성공 기록 0건 | API 35 전용 callback·fence 성공 기록 0건 |
| timestamp 표기 | `millisecond_fallback` 관찰; 지연 계산 안 함 | `nanosecond_representation` 관찰; 정확도·지연 계산 안 함 |
| 범위 주장 | API 29 경계만 통과 | API 34 경계만 통과; API 30–33은 별도 미검증 |

API별 세 조건이 모두 정확한 새 process에서 실행되고, 로그와 화면·API 정보가 일치해야 경계 실행을 통과로 기록한다. 오류 signature가 한 건이라도 있거나 앱이 예상 외로 종료·멈추면 실패를 보존하며 통과 수를 보충하지 않는다.

## 계획 적대 검토 · 독립 실패 관점 20개

| # | 실패 관점 | 계획의 방어·판정 |
|---:|---|---|
| 1 | API 29·34를 전체 API 29–34 호환으로 과장 | 두 경계만 검증하고 API 30–33은 미검증으로 유지한다. |
| 2 | compile SDK를 guest API로 오인 | 부팅한 AVD 안에서 `ro.build.version.sdk`를 직접 읽어 판정한다. |
| 3 | API 29 이미지를 x86로 실행해 ARM64 결과와 혼합 | 두 system image를 arm64-v8a로 고정하고 ABI를 기록한다. |
| 4 | 설치된 APK와 저장소 source가 다른데 현재 source 빌드로 설명 | 기기에서 pull한 APK hash를 PR #79 고정 artifact hash와 비교한다. |
| 5 | API 29 minSdk 설치가 실패해 런타임 fallback 검증을 건너뜀 | APK 설치 성공과 package/version을 각 AVD에서 먼저 확인한다. |
| 6 | 오래된 AVD `.ini`가 새 AVD를 가리켜 덮어씀 | 충돌 없는 새 AVD 이름을 사용하고 기존 AVD 목록·데이터를 보존한다. |
| 7 | 다른 프로젝트 `zl_poc`를 Spinon 작업으로 초기화 | `zl_poc` AVD는 수정·삭제하지 않는다. |
| 8 | 이전 emulator/QEMU 잔류 프로세스가 새 run에 붙음 | 시작 전 `adb devices`와 emulator 프로세스를 확인하고 run 뒤 종료 상태를 재확인한다. |
| 9 | AVD가 요청한 image와 다른 API로 부팅 | guest SDK level과 image package를 각각 기록한다. |
| 10 | 앱의 process 재사용으로 새 intent extra가 적용되지 않음 | 매 조건마다 `force-stop` 후 Activity를 새 process로 시작하고 PID를 기록한다. |
| 11 | 기본 bootstrap 성공을 R05 지원으로 해석 | bootstrap 항목은 JavaScript 기본 실행 smoke로만 판정한다. |
| 12 | debug-only flag가 release behavior로 설명됨 | artifact SHA로 기존 debug APK임을 고정하고 release 결론을 금지한다. |
| 13 | API 35 전용 class가 verifier에서 일찍 resolve되어 실행 중 crash | API 29·34에서 flag 경로를 실제 시작하고 fatal/linkage/verifier 오류를 검사한다. |
| 14 | 비동기 gate의 Activity 종료를 crash로 오인 | 예상 `api_below_35` log 뒤 정상 finish인지 Activity/process 상태로 구분한다. |
| 15 | R05 경로가 unavailable만 남기고 renderer submit을 중단 | present fallback 조건에서 `draw_requested=true`와 화면 활성화 1회를 확인한다. |
| 16 | 두 probe flag를 같이 켜 FrameTimeline branch가 present-fence fallback을 가림 | 서로 다른 fresh process에서 각 flag를 단독 실행하고 두 marker를 개별 확인한다. |
| 17 | tap을 손가락 입력으로 분류 | ADB synthetic이라고 명시하고 raw physical touch·latency 표본에서 제외한다. |
| 18 | GPU backend 차이가 API gate 결과를 흐림 | fallback은 explicit GLES control을 사용하고 GPU 성능 주장을 하지 않는다. |
| 19 | 화면 로그만 보고 오류·crash 누락 | PID별 logcat의 linkage/fatal/ANR과 화면 screenshot을 함께 대조한다. |
| 20 | endpoint 통과 뒤 설정·AVD 상태를 방치하거나 중간 API까지 완료 처리 | AVD를 정상 종료하고 기존 정의를 보존하며 API29·34 경계 결과와 API30–33 한계를 분리 기록한다. |

### 계획 검토 결과

첫 계획 공격에서 present-fence와 FrameTimeline flag를 함께 켜면 `R08GpuSurface.setActivationCount`의 조건 순서상 FrameTimeline branch가 먼저 실행되어 present-fence 경계가 검증되지 않는 점을 찾았다. 두 probe를 서로 다른 fresh process 조건으로 분리했다. API 29와 API 34를 전체 중간 API의 대체물로 주장하지 않도록 범위를 경계 확인으로 한정하고, API version은 guest에서 판별한다. 20개 독립 실패 관점을 다시 대조한 뒤 계획을 실행한다. 구현·R05.3 완료 판정은 실제 원본과 실행 후 검토를 따른다.

## 실행 결과 · 2026-10-09

기존 PR #79 debug APK를 API 29와 API 34 새 ARM64 AVD에 각각 설치했다. API 34에서는 기본 JS bootstrap 성공, async fence의 `api_below_35` unavailable 종료, GLES present-fence/FrameTimeline의 unavailable 표시 뒤 `draw_requested=true`와 화면 색상 변경을 확인했다. 두 진단 flag는 서로 다른 fresh process로 실행했다.

API 29에서는 APK 설치 후 cold launch가 `MainActivity` class initializer의 `System.loadLibrary()`에서 실패했다. `libspinon_bootstrap.so`의 unresolved `_Unwind_Resume` 때문에 native linker가 거부했고 R05 marker가 없었다. 현재 [Android link command](../tools/build-android.sh)는 `-nostdlib++ --unwindlib=none`을 전달하며 ELF에는 `_Unwind_*` undefined symbol이 남고 이를 제공할 `DT_NEEDED`가 없다. 따라서 API 29 R05 probes는 실행 불가다. API 29 startup gate는 실패로 남기고, 호환 unwind link 구성 적용과 새 artifact의 cold-start/R05 재실행을 후속 blocker로 둔다.

상세 원본·화면·환경·ELF 조사와 실행 후 새로운 20개 실패 관점은 [API 29·34 실행 근거](../spec/internal/evidence/r05-android-api29-34-fallback-2026-10-09/README.md) 및 [구현 검토](../spec/internal/evidence/r05-android-api29-34-fallback-2026-10-09/implementation-review.md)에 있다. 두 AVD는 정상 종료했고 새 AVD 정의는 다음 검증에 재사용하도록 보존했다. API 30–33, direct device touch, latency와 R05.3 전체는 미검증이다.

## 후속 API 29 수정과 재검증 · 2026-10-09

최초 API 29 실행의 _Unwind_Resume native-link 오류는 [별도 link 수정 계획](r05-android-api29-unwind-link.md)에 따라 고정 NDK AArch64 libunwind archive를 정적으로 연결해 수정했다. 새 Debug APK SHA-256 1639d939a34c7a7dfcf99b53b906d73c12951eae99b87757ecc2e8589b1e572e로 API 29·34 AVD를 fresh process에서 재실행했다. 두 API 모두 native load와 JS bootstrap이 성공했고 async fence는 api_below_35로 정상 종료했다. present-fence/GLES와 FrameTimeline/GLES는 각각 독립 실행에서 fallback draw 및 색상 갱신을 유지했다. fatal, linkage/verifier 오류, ANR은 관측되지 않았다.

사용자 요청에 따른 API 36 Android 실기기 WGPU/Vulkan synthetic 회귀는 새 APK load에 대한 추가 경계 확인이며 API 29·34 AVD 결과를 대체하지 않는다. API 30–33 및 직접 손가락 입력은 여전히 미검증이다. 상세 APK/ELF hash, 실행 로그와 별도 구현 검토는 [unwind link 실행 근거](../spec/internal/evidence/r05-android-api29-unwind-link-2026-10-09/README.md)를 따른다. Release 배포의 NDK NOTICE/license 수록 절차도 이번 Debug/AVD 작업 범위 밖이다.

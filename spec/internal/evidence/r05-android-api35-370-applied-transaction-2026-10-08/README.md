# R05.3 Android API 35·37.0 applied-transaction lifecycle 실행

**실행일:** 2026-10-08 · **fixture 결과:** API 35·37.0 각각 통과 · **캡처 관찰:** API 37.0 앱 복귀 뒤 PASS 화면 확인, 최초 캡처 검정 원인은 미확정 · **R05.3/R05:** 미완료

## 실행 요약

동일한 source commit `300443b868d06a4fdc59a0fb72770df4edb57ca8`, clean pinned V8, 같은 Android debug APK를 API 35 / 4 KB와 API 37.0 / 4 KB ARM64 fresh AVD에서 순차 실행했다. 양쪽 모두 실제 R08 GLES `SurfaceView`의 applied-transaction callback lifecycle fixture가 PASS했다.

| 항목 | API 35 / 4 KB | API 37.0 / 4 KB |
|---|---|---|
| AVD / serial | `spinon_api35_applied_txn` / `emulator-5580` | `spinon_api37_0_applied_txn` / `emulator-5580` |
| image | Google APIs ARM64, revision 9 | package `google_apis`, ARM64, revision 6; SDK tag display also includes AI Glasses Compatible |
| guest | API 35, page 4096, `arm64-v8a` | API 37.0, page 4096, `arm64-v8a` |
| fingerprint | `google/sdk_gphone64_arm64/emu64a:15/AE3A.240806.043/12960925:userdebug/dev-keys` | `google/sdk_gphone64_arm64/emu64a:17/CE2A.260420.019/15611780:userdebug/dev-keys` |
| source / V8 | `300443b…`; V8 `7b50b62cb18f28617959e8452e2cd18195b38bcf`, clean | 동일 |
| APK SHA-256 | `d9a26b62d326d4f9310394b9eb06ffcecbb7c31e27758548c27eb1b454e40bb1` | 동일 |
| callback 경로 | baseline · 취소 후 late callback · 새 generation 회복 | baseline · 취소 후 late callback · 새 generation 회복 |
| fixture | 12 checks, 0 failures | 12 checks, 0 failures |
| timeout/callback 경합 | 100회: callback 36, timeout 64 | 100회: callback 81, timeout 19 |
| idle / 종료 상태 | 60/60초, pending/queue/active 모두 0 | 60/60초, pending/queue/active 모두 0 |
| 결과 화면 | runner 캡처에 PASS | 최초 및 재확인 캡처는 검정. Settings 화면 대조 뒤 앱 복귀 캡처에 PASS |

API 35 image에서는 `ro.build.version.sdk_full`이 비어 있었다. 계획의 exact image gate를 우회하지 않도록 실제 API level, `source.properties`/image 경로, guest fingerprint, ABI와 page size를 함께 기록했다. API 37.0은 guest의 `api_full=37.0`도 확인했다. 두 설치 package의 전체 `source.properties`는 `avd-recovery/`에 보존했다.

## API 37.0 검정 캡처 조사와 AVD 판단

runner가 저장한 `run-api37_0/result.png`와 재확인한 `result-recheck.png`는 앱 콘텐츠가 검정으로 보인다. 저장된 UI hierarchy에는 PASS TextView가 있고 MainActivity도 resumed 상태였다. 이후 Android Settings를 열어 찍은 [플랫폼 대조 화면](run-api37_0/diagnostics/settings-control.png)은 정상 렌더링됐다. Settings에서 돌아온 [앱 재개 화면](run-api37_0/diagnostics/app-resume-screenshot.png)에는 `R05 callback 실패 주입: PASS`와 12 checks·100회 경합·60초 idle 문구가 보인다.

이로써 emulator 화면 캡처 전체가 검정이라는 가설은 배제되지만, 최초 앱 캡처가 검정이 된 정확한 원인은 확정되지 않았다. 앱 표면의 첫 frame 준비, activity 재개 시 surface 갱신, 또는 screenshot 시점 사이를 분리하는 반복은 하지 않았다. 그래서 최초 검정 캡처를 버리지 않았고 앱 재개 캡처와 나란히 보존했다. 이 결과만으로 영구적인 그래픽 결함이나 AVD 손상을 주장하지 않는다.

재설치는 하지 않았다. API 37.0 system image가 설치되어 있고 새 AVD의 `.avd` data directory도 남아 있었으며, Settings와 재개된 앱 콘텐츠가 모두 그려졌다. 종료 시점에 API 35와 37.0 Spinon emulator는 정상 종료했고 각각의 AVD 파일은 유지했다. ADB에는 다른 프로젝트의 `zl_poc` / `emulator-5554`만 남아 그 프로세스는 건드리지 않았다.

현재 등록 목록에서 아래 과거 항목은 `.ini`만 있고 `.avd` data directory가 없었다. 실행 중인 대상이 아니며, 이번 검증에 필요하지 않아 복구하거나 등록 파일을 삭제하지 않았다.

- `spinon_api35_compat`
- `spinon_api37_0_compat`
- `spinon_api37_2`
- `spinon_api37_2_modern`
- `zl_poc` (이름은 등록됐지만 기존 실행 process는 별도 프로젝트 소유)

반대로 새 검증용 `spinon_api35_applied_txn`, `spinon_api37_0_applied_txn` AVD directory와 설치 image는 존재한다. 종료 후 기기·process 확인은 [API 37.0 종료 기록](avd-recovery/post-api37_0-shutdown.txt)과 [API 35 종료 기록](avd-recovery/api35-shutdown.txt)에 보존했다.

## fixture 해석 경계

실제 `SurfaceControl.Transaction` 완료 callback의 non-null `TransactionStats`, surface destroy/recreate 시 이전 generation 취소와 late callback 분류, 다음 generation 회복을 확인했다. R08 GLES draw revision marker 세 개와 callback queue marker, queue overflow 회복을 runner가 검사했다. `SurfaceView_JankData`는 API 35에서는 API 36 미만, API 37.0에서는 API 37.2 미만 사유로 unavailable로 남았다.

입력은 fixture가 직접 dispatch한 synthetic `MotionEvent`다. 이번 callback 수명주기 결과는 물리 입력, renderer 성능, event-to-present latency, photon/scanout 시간, 실기기 호환성 또는 Android 전체 API 지원의 증거가 아니다.

## 원본 자료

- [API 35 실행 폴더](run-api35/): environment, build/install/launch, Logcat, result screenshot, source/APK digests
- [API 37.0 실행 폴더](run-api37_0/): environment, build/install/launch, Logcat, 초기·재확인 screenshot, UI hierarchy, 재개 진단 screenshot, source/APK digests
- [AVD 사전 조사·생성·종료 자료](avd-recovery/): 긴 emulator startup log는 trailing whitespace를 보존하도록 무손실 gzip 압축했다.
- [구현·실행 후 독립 실패 관점 20개](implementation-review.md)
- [실행 전 계획 및 별도 계획 검토 20개](../../../../plan/r05-android-api35-370-applied-transaction.md)
- [공통 callback fixture runner](../../../../tools/verify-r05-android-callback-faults.sh)

## 남은 범위

API 29–34 fallback, API 36 actual lifecycle, iOS device callback runtime, 물리 입력, 실기기, release app, event-to-present 상관 계측, optical scanout은 별도다. R05.3과 R05는 미완료 상태다.

# R05.3 Android API·16KB 호환성 실행 및 구현 적대 검토

**실행일:** 2026-10-08 · **범위:** Android API 35·37.0·37.1 AVD와 제한된 R05 표시 fence fixture · **당시 결론:** API 경계 fallback은 확인, 기존 16KB APK의 GNU_RELRO 기준은 실패 · **후속:** [16KB 재빌드에서 GNU_RELRO·APK 정렬 통과](r05-android-16kb-rebuild-2026-10-08.md) · **R05.3/R05:** 미완료

## 실행 범위와 결과

모든 입력은 1080×2400 AVD 화면의 중앙에 보낸 synthetic ADB 탭이다. 사용자 터치 시간·실기기 동작·성능 순위를 측정한 것이 아니다. 각 점수 block은 새 앱 프로세스에서 10개 입력을 기록하고, callback drain용 11번째 입력은 별도 집계했다. APK SHA-256은 모든 block에서 `325611b9427ac66a2c82722f89c3193a24b7f04671dadd38b98f9aff90c605cd`다.

| 이미지 | Renderer | 점수 입력 fence | drain fence | 미사용·pending | 판정 |
|---|---|---:|---:|---:|---|
| API 35 ARM64 Google APIs, 4KB | WGPU/Vulkan llvmpipe | 27/30 | 3/3 | 점수 3, drain 0 | 제한된 AVD 경로에서 신호 수신 |
| API 35 ARM64 Google APIs, 4KB | GLES/ANGLE·SwiftShader | 25/30 | 3/3 | 점수 5, drain 0 | 제한된 AVD 경로에서 신호 수신 |
| API 37.0 ARM64 Google APIs, 4KB | GLES/ANGLE·SwiftShader | 24/30 | 3/3 | 점수 6, drain 0 | 제한된 AVD 경로에서 신호 수신 |
| API 37.0 ARM64 Google APIs, 4KB | WGPU | 실행 안 함 | — | — | auto·host·SwiftShader에서 guest Vulkan adapter 없음 |
| API 37.1 ARM64 Google APIs 16KB | GLES/host MoltenVK | 28/30 | 3/3 | 점수 2, drain 0 | 제한된 AVD 경로에서 신호 수신 |
| API 37.1 ARM64 Google APIs 16KB | WGPU | 실행 안 함 | — | — | host 모드에서 guest Vulkan adapter 없음 |

각 실행 block에서는 input·submit·fence callback이 모두 11건이었다. 로그의 sequence 1–10만 scored 값이고 sequence 11은 drain이다. `pending` fence는 성공으로 바꾸지 않았다. 실제 집계는 [원본 로그와 화면 자료](r05-android-api-compatibility-2026-10-08/)를 따른다. API 35는 JankData가 `api_below_36`, API 37.0·37.1은 `api_below_37_2`로 닫혔고 R08 표면·입력은 계속 동작했다. 각 API의 R05-off 대조는 R05 marker 0건과 R08 touch count 1을 확인했다.

API 37.1 16KB AVD에서는 기본 V8 bootstrap이 `nativeRun` 결과 `nodes=2`를 반환했고, GLES 앱도 시작·입력·표시 fence callback까지 실행했다. WGPU 대조는 adapter 부재로 렌더링하지 못했다. 앱 실행 성공은 전체 API 호환이나 16KB 바이너리 적합성 판정과 같지 않다.

## 수정 linker flags 적용 전 기존 APK 검사와 빌드 차단

검사 APK에는 arm64-v8a `libspinon_bootstrap.so` 하나가 들어 있다. APK의 16KB zip alignment 검사는 성공했고, `.so`의 네 PT_LOAD segment는 모두 `2**14`(16,384 byte) 정렬이다. 하지만 GNU_RELRO의 `VirtAddr + MemSiz`는 `0x2dd5000`이며 `0x4000`으로 나눈 나머지가 `0x1000`이다. Android의 공식 16KB 지침이 제시하는 기준을 통과하지 못한다. [Android 공식 16KB 페이지 크기 지침](https://developer.android.com/guide/practices/page-sizes)

이를 고치기 위해 `tools/build-android.sh`의 최종 `.so` 링크 명령에 다음 linker flags를 추가했다.

```text
-Wl,-z,max-page-size=16384 -Wl,-z,common-page-size=16384
```

당시 검사 APK에는 수정 linker flags를 적용해 재링크하지 못했다. `tools/build-android.sh`는 고정 V8 source checkout 부재로 중단됐다. Java/Gradle APK 작업은 기존 네이티브 라이브러리를 재사용했고 APK SHA도 위 값 그대로였다. 따라서 당시 실행은 **기존 artifact의 16KB AVD 동작 확인**과 **수정 linker flags를 포함한 앱 바이너리 검사**를 혼합하지 않았다. 이 blocker는 후속 [재빌드 보고서](r05-android-16kb-rebuild-2026-10-08.md)에서 해소했으며, 그 결과만으로 R05.3 전체를 완료 처리하지 않는다.

- 기기: API 37.1 ARM64 Google APIs 16KB 전용 AVD, `getconf PAGE_SIZE=16384`.
- 해당 전용 AVD에서 linker 호환 속성은 `fatal`, package-manager 호환 비활성은 `true`로 확인했다.
- APK `zipalign -c -P 16 -v 4`: 성공.
- ELF PT_LOAD: 네 개 모두 16KB 정렬.
- ELF GNU_RELRO end: `0x2dd5000`; 16KB 기준 실패 (`0x1000` remainder).
- API 37.1 실행 APK SHA-256: `325611b9427ac66a2c82722f89c3193a24b7f04671dadd38b98f9aff90c605cd`.
- `android-native-rebuild.log`: 고정 V8 checkout 부재로 실제 재링크 중단.
- `api37_1_16k-binary-audit.txt`: APK, ELF, zip alignment와 기기 속성 원본.

## 구현 적대 검토 · 20개 독립 실패 관점

이 검토는 기존 계획의 20개 계획 실패 관점과 별개로, 이번 Android 구현·실행 결과와 linker 변경을 다시 대조했다. 각 행은 별도의 실패 경로다. 전체 행렬을 20번 반복 실행했다는 뜻은 아니다.

| # | 공격 관점 | 확인 결과와 남은 위험 |
|---:|---|---|
| 1 | API 이미지가 잘못 라벨링되어 경계 결과가 오염되는가 | AVD 이름·system image 경로·빌드 로그를 대조했다. API 35/37.0 4KB와 API 37.1 16KB를 분리했다. 각 실행 환경은 부록 파일에 기록했다. |
| 2 | API 37.0/37.1을 API 37.2로 오인하는가 | 실제 `SDK_INT_FULL` 기록은 각각 `3700000`, `3700001`; JankData는 두 경우 모두 `api_below_37_2`로 거부됐다. |
| 3 | API 35에서 새 API 참조가 조기 실행되어 앱 시작이 깨지는가 | 두 renderer 모두 API 35 앱 시작·R08 draw·입력을 확인했고 capability는 `api_below_36`으로 닫혔다. API 29–34는 여전히 미실행이다. |
| 4 | API 35 transaction API 신호와 API 37.2 JankData를 같은 API로 취급하는가 | raw marker와 gate를 별도 확인했다. 이번 결과는 TransactionStats fence callback이며 JankData 표시 timestamp를 성공으로 간주하지 않는다. |
| 5 | API 37.0/37.1에서 37.2 전용 listener가 호출되는가 | 둘 다 `api_below_37_2`가 기록됐고 정상 GLES path는 진행했다. 각 API에서 별도 확인했다. |
| 6 | 같은 앱이 아닌 다른 APK를 섞어 비교하는가 | 12개 scored block와 capability 실행 APK SHA가 같은 값을 가리킨다. API 37.1 ELF 검사는 같은 Gradle APK에서 추출했다. |
| 7 | 앱 프로세스가 이전 block의 request 상태를 물려받는가 | block마다 별도 launch log와 PID file이 있으며 process PID가 분리되어 있다. 이전 block state 재사용 증거는 없다. |
| 8 | 입력은 기록했지만 frame submit이 빠진 표본을 성공 처리하는가 | 각 block input/submit/callback은 11/11/11이다. callback sequence와 request id를 원본 로그로 확인했다. |
| 9 | drain 입력을 점수에 넣어 누락을 감추는가 | sequence 1–10과 11을 분리했다. 표의 scored denominator는 block당 10이며 drain은 별도다. |
| 10 | pending fence를 표시 완료로 처리하는가 | `fence_signal_usable=false`인 pending을 제외했다. API 35 GLES 5건, API 35 WGPU 3건, API 37.0 GLES 6건, API 37.1 GLES 2건은 scored 표본에서 제외된다. |
| 11 | request callback을 엉뚱한 revision/surface 세대에 붙이는가 | callback 로그의 request id·input sequence·revision·generation·현재 surface를 대조했다. 이번 block은 generation 1뿐이므로 회전 중 stale callback 안전성은 검증하지 않았다. |
| 12 | submit 반환을 OS present 신호로 과장하는가 | `R05_SUBMIT`과 fence callback을 구분했다. 이 문서는 transaction-present 후보 신호만 기록하며 광학 scanout 또는 제품 latency를 주장하지 않는다. |
| 13 | R05를 끈 상태에서도 probe marker가 새어 나오는가 | API 35, 37.0, 37.1 각 R05-off 대조에서 R05 marker 0건·R08 tap count 1을 확인했다. |
| 14 | WGPU adapter 부재를 OS/API 호환 실패로 오판하는가 | API 37.0의 auto·host·SwiftShader 및 API 37.1 host에서 adapter 없음 로그를 확인했다. 이 조합은 점수화하지 않고 GPU adapter 미제공으로 분류했다. |
| 15 | renderer/backend 차이를 성능 우열로 해석하는가 | API 35·37.0 GLES는 ANGLE/SwiftShader, API 35 WGPU는 Vulkan llvmpipe, API 37.1 GLES는 Apple M4 Max/MoltenVK 경로다. 성능 순위를 내지 않는다. |
| 16 | 앱 프로세스가 crash/restart한 결과를 정상으로 오인하는가 | 각 scored block launch 완료를 확인했고 API 37.1 마지막 crash buffer는 0 byte다. lifecycle·ANR fault injection은 수행하지 않았다. |
| 17 | 16KB ZIP alignment 실패를 ELF 정렬과 혼동하는가 | Android `zipalign -c -P 16` 성공을 별도로 기록했다. 이것만으로 ELF 적합성을 선언하지 않는다. |
| 18 | LOAD segment 정렬 실패를 놓치는가 | 추출한 유일한 `.so`의 네 PT_LOAD가 모두 16,384 byte alignment임을 확인했다. |
| 19 | GNU_RELRO misalignment를 runtime smoke 성공으로 덮는가 | `VirtAddr + MemSiz = 0x2dd5000`, remainder `0x1000`이어서 공식 정적 기준을 실패 처리했다. 16KB AVD 실행 성공은 이 결함을 해소하지 않는다. |
| 20 | linker 수정이 실제 APK에 들어갔다고 잘못 보고하는가 | build script에는 flags가 있지만 V8 checkout 부재로 앱 재링크가 실패했다. 검사 APK는 이전 SHA 그대로다. fix 적용 및 최종 ELF 재검사는 다음 gate다. |

이 검토 시점에는 GNU_RELRO 끝 정렬과 수정 linker flags를 포함한 실제 앱 재빌드가 blocker였다. 후속 빌드에서 16KB ELF/ZIP 기준은 통과했다. API 29–34, API 37.0/37.1 WGPU adapter 환경, surface lifecycle·callback timeout/failure injection, iOS OS display callback runtime, 실기기 터치, 광학 scanout은 여전히 이 행렬에서 검증하지 않았다. 그러므로 R05.3·R05는 미완료이며 이번 결과를 입력→실제 픽셀 지연·렌더러 성능 비교로 사용하지 않는다.

## 재현 자료

- [Android API matrix raw logs, captures, AVD startup records, build and ELF checks](r05-android-api-compatibility-2026-10-08/)
- [Hash list](r05-android-api-compatibility-2026-10-08/SHA256SUMS)
- Plan: [R05.3 input-to-presentation](../../../plan/r05-input-to-presentation.md)

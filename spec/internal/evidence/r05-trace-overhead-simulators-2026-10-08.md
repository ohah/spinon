# R05 · Android·iOS 시뮬레이터 계측 오버헤드 진단 · 2026-10-08

**상태:** 시뮬레이터 진단 완료 · R05 미완료 · 계측 오버헤드 결론 미확정

## 비교 범위

같은 디버그 앱에서 OS 수집기만 켜거나 끄고 앱 내부 R05 측정값이 얼마나 달라지는지 관찰했다. Android는 Perfetto session 유무를, iOS는 Time Profiler 수집 유무를 짝 비교했다. 이 결과는 진단 계측의 시뮬레이터 영향이며 제품 성능 비교가 아니다.

| 플랫폼 | 장치·OS | 빌드와 실행 | 쌍 순서 |
|---|---|---|---|
| Android | `sdk_gphone64_arm64` ARM64 에뮬레이터 · Android 16 / API 36 · build `13894323` | 동일 debug APK · 조건당 10개 독립 실행 · 회당 20초·ADB 탭 10회 · 5초 안정화 | 총 10쌍 · `on/off` 5쌍·`off/on` 5쌍 |
| iOS | iPhone 17 Pro Simulator · iOS 26.2 · UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D` | 동일 Debug-iphonesimulator 앱 · 조건당 10개 독립 실행 · 회당 12초·자동 probe 32개 | 총 10쌍 · `on/off` 5쌍·`off/on` 5쌍 |

두 플랫폼은 같은 Mac에서 순차 실행했다. Android 입력은 ADB 탭 자동화이고 iOS는 앱 selector 기반 자동 probe다. 물리 터치를 측정하지 않았다. 양쪽 모두 시뮬레이터·debug 빌드이며 release·실기기 결과가 아니다.

## 도구·빌드 식별

- Mac 호스트: macOS 26.5.1, Xcode 26.2 (build 17C52), Bun 1.4.2, Node.js 26.7.0.
- 앱 소스 기준 커밋: `892161a2c0aac2bd825e67a78db18bb5afa95cb0`.
- 빌드에 사용한 V8 소스 커밋: `7b50b62cb18f28617959e8452e2cd18195b38bcf`.
- Android debug APK SHA-256: `df13803120f8699d0e2ee9433cc0c328495288b304dcb3c0d1e29b8c36936d3c`.
- Android 각 실행 작업 트리 SHA-256: `08703182714b348c67ba1ab8013078a34939346892c406e4c35f3655af220b59`.
- iOS 앱 bundle SHA-256: `2e693eae265461bf1601f80d4f8adf98df8e110a499ce57f271223704da9bb1b`.
- iOS 앱 실행 파일 SHA-256: `d6048be1bf36a8e41cbf0f66b487885c2a03f6c86ebc12fe2083e0b1fadbd6db`.
- iOS 각 실행 작업 트리 SHA-256: `91c2cb2f0f5eb04b730273b09413b58dee139a9a238369292b3e76b97f044540`.
- Android 원본 품질 확인에는 Perfetto Trace Processor `v58.2-add693d8b` (`add693d8b338ba9599dbcbc3e300b1ab8c000897`)를 사용했다. 앱 빌드·실행 의존성은 아니다.

## 결과

각 실행에서 먼저 앱 표본의 중앙값을 계산하고, 그 뒤 각 쌍의 차이(`계측 on - 계측 off`)를 계산했다. 아래는 10개 쌍 차이의 중앙값과 최소–최대 범위이며 시간 단위는 µs다.

### Android · Perfetto session 영향

| 앱 내부 측정 | trace-on 중앙값 (범위) | trace-off 중앙값 (범위) | 쌍별 차이 중앙값 (범위) |
|---|---:|---:|---:|
| queue residence | 58.000 (38.500–121.000) | 60.500 (34.500–83.000) | 7.000 (-43.500–47.000) |
| V8 호출 | 229.250 (195.000–283.500) | 221.250 (189.000–261.000) | 10.250 (-66.000–94.500) |
| actor 응답 전 | 240.750 (206.000–293.500) | 231.250 (201.000–279.500) | 9.750 (-73.500–92.500) |
| 응답 대기 | 369.750 (248.000–493.500) | 352.500 (261.500–422.000) | 1.250 (-118.500–232.000) |

각 조건 10개 실행에서 runtime report는 10개씩, `status=0`과 연속 dispatch sequence는 모두 일치했다. trace-on Perfetto 원본 10개는 모두 Trace Processor로 열렸다. packet/data loss·error 통계·`ftrace_setup_errors`는 0이었고, runtime·V8·completion 표식 및 입력·dispatch counter는 각 10개였다. FrameTimeline 고유 token은 display 20–22개·surface 19–21개였다. `gfxinfo`는 별도 자료로 보존했으며 trace-off 프레임 자료와 같은 지표로 보지 않았다.

### iOS · Time Profiler 영향

| 앱 내부 측정 | 쌍별 차이 중앙값 (범위) |
|---|---:|
| runtime queue | 4.051 (-4.397–14.959) |
| Rust FFI | 23.657 (-56.291–96.646) |
| V8 호출 | 3.000 (-9.000–32.500) |
| actor 응답 전 | 5.500 (-12.500–38.500) |
| main queue handoff | 3.489 (-5.792–12.354) |
| callback → CADisplayLink | 4.073 (-1141.917–419.979) |
| 입력 → CADisplayLink | 23.469 (-1000.083–522.271) |

각 조건 10개 실행의 32개 표본 모두 `status=0`, PID 일치, sequence `1..32`, 증가하는 timestamp를 만족했다. trace-on 10개 전부에서 앱 PID와 Time Profiler PID가 일치했고 32개 앱 표본 전체가 trace TOC의 실제 기록 구간 안에 있었다.

## 해석과 한계

- 두 플랫폼의 모든 쌍별 범위가 0을 가로지른다. 이 표본과 시뮬레이터 변동만으로 OS 수집기의 안정적인 오버헤드 크기를 판정할 수 없다. 양수 중앙값을 확정 비용으로 해석하지 않는다.
- 10쌍은 기술통계다. p95/p99, 오버헤드 0, Android와 iOS의 성능 우열을 주장하지 않는다.
- Perfetto와 Time Profiler는 서로 다른 수집기라 두 플랫폼의 수치를 합치지 않는다. Time Profiler CPU sample은 thread state 대기가 아니며 `CADisplayLink`는 픽셀 발광 시각이 아니다.
- trace-on/off는 기존 앱 계측을 양쪽에 유지한 채 OS 수집기만 바꿨다. 따라서 앱 계측 자체의 비용을 제거하거나 측정한 결과가 아니다.
- 이 실행으로 실기기 foreground·물리 입력, 실제 GPU present·광학적 input-to-photon, release 성능, React Native·ReactLynx·네이티브 비교를 검증하지 않았다. R05 전체는 계속 미완료다.

## 실행 자료와 재현

- Android 행렬: `build/spinon/benchmark/android-trace-overhead/20261008T033650Z/`
- iOS 행렬: `build/spinon/benchmark/ios-trace-overhead/20261008T035038Z/`
- 전체 개별 원본 trace는 `.gitignore` 대상 `build/spinon/benchmark/`에만 보존하며 커밋하지 않았다. SHA-256 manifest는 [r05-trace-overhead-checksums-2026-10-08.sha256](r05-trace-overhead-checksums-2026-10-08.sha256)이다. Android는 `.pftrace` 파일 SHA-256이고, iOS는 `.trace` 안의 파일을 상대 경로 순으로 정렬해 `경로 + NUL + 파일 SHA-256 + 개행`을 연결한 tree SHA-256이다.
- 전체 실행 중 재시도·제외는 없었다. 원행렬 schedule의 열 누락은 각 run `started_utc`로 순서를 복원해 요약 manifest에 반영했다. 후속 수집기는 순서·상태·경로 열을 모두 저장하도록 보강했다.
- 수집·집계 코드를 보강한 뒤 Android 3입력/10초·iOS 32표본/12초 단일 pair 스모크를 다시 통과했다. 이 스모크는 10쌍의 본 결과와 별도로 취급한다.
- 재현 명령과 로그 검증기는 [R05 계측 오버헤드 대조 계획](../../../plan/r05-trace-overhead.md)에 있다.

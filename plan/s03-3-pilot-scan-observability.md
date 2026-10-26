# S03.3 · pilot baseline observability slice

**상태:** 구현 전 계획 · fixture-only 진단 기록
**상위 비용 분석:** [registry scan 비용 귀속 계획](./s03-3-scan-cost-attribution.md)
**비교 기준:** `origin/main` `8247a12b` · 내부 계약 [0021](../spec/internal/0021-s03-dom-node-lifecycle.md)
**구현 전 비교 모델:** [pilot scan observability](../spec/internal/evidence/s03-scan-pilot-observability-precomparison-2026-10-26/README.md)
**구현 위험 검토:** [서로 다른 20개 관점](../spec/internal/evidence/s03-scan-pilot-observability-implementation-risk-review-20-2026-10-26.md)

## 목적과 범위

기존 S03 대량 registry fixture는 같은 Isolate에서 10회 재scan한 `document_collection_last_scan_us` 표본을 기록한다. 각 operation report에는 이미 owner thread ID, `steady_clock` 시작 시각(ns), 전체 collector wall-duration(µs), operation sequence와 trace cookie가 있다. 하지만 플랫폼 fixture 출력은 scan 표본 시간만 나열해 원본 report와 TID·시작 시각을 함께 검토하기 어렵고, 기존 검증 결과 폴더에는 재현 source/build identity가 충분히 고정되지 않는다.

이 slice는 기존 자료를 opt-in S03 fixture 출력과 run manifest에 함께 남겨 **현재 동작의 pilot 관측 자료**를 만든다. 성능 최적화, 새 timer/counter, CPU 계측, scheduler trace, 성능 pass/fail 판정은 하지 않는다. 표본 10회는 기존 정확성 fixture 반복 수이며 통계적 N으로 간주하지 않는다.

## 확인된 기존 경로와 비교 기준

- `native/v8/src/spinon_v8.cc::CollectDocumentAtSafePoint`는 `std::chrono::steady_clock`으로 scan 진입 시 `last_scan_start_ns`를, 반환 직전 `last_scan_duration_us`를 기록한다. duration은 C++ registry 순회, Rust callback/collector, 반환값 검증과 weak wrapper 정리를 포함한 전체 wall-time이다.
- `crates/spinon-runtime/src/session/report.rs::operation_report`는 `owner_tid`, `callback_tid`, `seq`, `trace_cookie`, `document_collection_last_scan_start_ns`, `document_collection_last_scan_us`를 이미 반환한다. actor가 owner thread에서 V8 eval/dispatch와 safe-point 회수를 수행하므로 이 report의 `owner_tid`가 scan을 실행한 native owner TID다. OS scheduler trace용 TID/clock 상관은 아직 검증된 것으로 보지 않는다.
- Android `MainActivity.runLifecycleCollectionProbe` 및 iOS `RuntimeThreadExperimentViewController.runLifecycleCollectionProbe`는 같은 Isolate에서 대량 live registry를 10회 재scan한다. 두 harness의 기존 correctness oracle는 변경하지 않는다.
- S03 fixture는 `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1`로만 빌드되고 Android UI는 해당 `BuildConfig`가 켜졌을 때 표시된다. iOS 검증 경로도 fixture flag로 지정된다. baseline runner는 기존 `tools/verify-s03-dom-lifecycle-simulators.sh`다.

## 구현 산출물과 정확한 관측 필드

### 각 대량 재scan 표본

Android·iOS fixture 출력에 다음 key-value tuple을 표본마다 한 건씩 남긴다.

```text
scan_sample_index=<1..10> seq=<operation sequence> trace_cookie=<cookie> owner_tid=<native owner TID> scan_start_ns=<steady_clock epoch duration> scan_wall_us=<total safe-point collector wall-time>
```

기존 `large_registry_scan_samples_us` / `scan-us` 필드는 하위 호환 목적으로 유지한다. 각 플랫폼은 새 표본 필드가 비어 있거나 파싱되지 않으면 `scan_observability=FAIL`로 출력하고 verification harness가 이를 실패로 처리한다. 이는 metadata completeness gate이며 성능 합격선이 아니다. root/live/reclaimed 집합, 회수 동작, sample 반복 횟수와 기존 PASS 조건은 바꾸지 않는다.

`scan_start_ns`는 현재 프로세스의 C++ `steady_clock` epoch 값이다. OS trace와 공유되는 clock이라는 보장은 없으며 재시작 간 절대 비교에 사용할 수 없다. `owner_tid`는 OS 스케줄러 상태를 뜻하지 않는다. scheduler delay, thread CPU-time, process/thread lifetime correlation은 이번 출력으로 주장하지 않는다.

### 실행별 source/build identity

기존 runner의 고유 output directory에 `run-manifest.txt`를 만든다. 최소 기록은 다음과 같다.

- run ID, source commit, branch, tracked diff SHA-256, untracked source path/content digest
- V8 pinned revision, S03 fixture flag, Android Debug/iOS `Debug-iphonesimulator` task·configuration
- host macOS/Xcode/Android SDK 도구 버전과 지정 AVD serial·model·API·ABI, iOS Simulator UDID 및 booted device 행
- Android APK SHA-256, iOS `.app` 상대경로별 파일·symlink digest, build log SHA-256

manifest는 run 디렉터리의 산출물이며 저장소 문서 상태를 갱신하지 않는다. 입력 output 경로가 이미 있으면 기존 harness의 덮어쓰기 거부를 보존한다. 앱 artifact digest는 release artifact나 두 플랫폼 간 크기 비교로 해석하지 않는다.

## 경계와 제외 항목

변경 파일 후보는 다음으로 제한한다.

1. `platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java` — 현재 S03 fixture 결과에 10개 표본의 기존 report field tuple을 노출한다.
2. `platforms/ios/Sources/RuntimeThreadExperimentViewController.swift` — 동일 tuple을 S03 fixture report/log에 노출한다.
3. `tools/verify-s03-dom-lifecycle-simulators.sh` — 별도 output manifest 및 표본 metadata 완전성 assertion을 기록한다.
4. `spec/internal/evidence/s03-scan-pilot-observability-implementation-review-20-2026-10-26.md` — 구현 후 별도 adversarial review 결과를 기록한다.

`spec/STATUS.md`, `spec/public/roadmap.html`, C11.4 파일, Rust/C++ collector 구현·ABI, production report schema, benchmark algorithm, V8 build configuration은 변경하지 않는다. 실기기 실행·scheduler trace·release build·성능 효과 threshold·표본 수 재결정은 제외한다. 기존 시뮬레이터 검증 harness는 앱 설치, Android logcat clear/force-stop, iOS 설치/terminate를 하므로 현재 부팅된 세션을 보존할 수 없으면 실행하지 않는다. Android/iOS build-only 검증은 worktree 안의 build output에 한정하고 기존 시뮬레이터에 연결하지 않는다.

## 검증 및 판정

- plan 단계에서 고정된 기존 correctness oracle와 같은 Android/iOS app fixture build를 유지한다. 새 sample tuple은 각 platform output에 정확히 10개 있어야 하며 모든 필수 수치가 파싱되어야 한다.
- `run-manifest.txt`에서 source/build identity key가 누락되지 않았는지, APK와 `.app` digest 계산이 성공했는지 검증한다.
- Android Debug 및 iOS Simulator Debug compile, shell syntax, fixture-on/off build boundary를 확인한다. simulator 설치·실행은 현재 booted 세션 상태와 사용자 작업을 확인한 뒤 별도 판단한다.
- 성능 baseline은 아직 채집하지 않는다. 이 slice는 새 성능 pass/fail 기준이나 S03.3 완료 판정을 만들지 않는다.

## 선행 작업과 다음 단계

비교 정의와 성능 판정 유보는 [S03.3 비용 분석 계획](./s03-3-scan-cost-attribution.md)의 선행 조건을 따른다. 이 계측 PR이 병합된 뒤에만 동일 instrumentation revision을 기준으로 기존 AVD·Simulator harness를 실행해 pilot raw log와 manifest를 수집할 수 있다. 그 자료를 본 뒤 effect threshold, tail 허용치, 통계 N, release profile, scheduler trace 지원 및 장기 반복 예산을 별도로 확정한다. 해당 결정 없이 최적화 구현을 시작하거나 성능 개선을 주장하지 않는다.

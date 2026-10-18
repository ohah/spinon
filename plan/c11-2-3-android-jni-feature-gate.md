# C11.2.3 Android JNI feature gate 정합성

## 목표

Android native bridge의 C11.2.3 create/eval 진입점이 Rust FFI의 `c11-2-3-grid-fixture-test` opt-in 상태와 같은 전처리 조건을 사용하게 한다. 기본 빌드에서는 fixture 전용 C symbol을 참조하지 않으며, JNI entry point는 유지해 Java의 native method link가 깨지지 않도록 한다.

## 재현과 기준

기준 tree는 `origin/main` `0168e89e2ebff3543e4d2153bf21e9557bb7544a`다. Android API 37 AVD 빌드에서 `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST=1`, `SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=0` 조합은 `spinon_jni.cc`가 C11.2.3 심볼을 무조건 호출해 C++ compile error를 냈다.

## 변경과 판정

- 두 JNI 함수의 Rust FFI symbol 참조를 `SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST`와 동일한 `#if` 안에 둔다.
- feature off일 때 create는 `0`, eval은 `status=-90 feature-disabled`를 반환한다. JNI symbol은 유지한다.
- Android feature off 빌드와 opt-in positive-control 빌드를 각각 성공시킨다. off 빌드에서 C12.4 worker-stack 진단도 켜 빌드 결합을 재현한다.
- `SPINON_ENABLE_C04_RUNTIME_GPU=0`와 C11.2.3 opt-in 조합은 빌드가 거부해야 한다.
- 일반 앱의 C11.2.3 동작·지원 범위를 바꾸지 않으며 iOS JNI와 C11 Grid 동작은 범위 밖이다.

## 실행 결과

- Android ARM64 기본/off 빌드(`C04_RUNTIME_GPU=1`, `C124_WORKER_STACK_RESIDENCY_TEST=1`, `C11_2_3_GRID_FIXTURE_TEST=0`)와 opt-in positive-control 빌드가 모두 통과했다. 기본 ELF에는 JNI method 2개만 있고 C11.2.3 fixture FFI symbol은 없으며, positive-control ELF에는 두 fixture FFI symbol이 추가된다. [실행 근거](../spec/internal/evidence/c11-2-3-android-jni-gate-2026-10-19/README.md)
- C04 off/C11.2.3 on 조합은 기존 build-script guard가 exit 2로 거부했다. 기본/off 앱의 API 37 AVD 실행은 root HTML 기본 화면의 no-regression smoke이며 C11.2.3 화면 동작 검증이 아니다.
- 이 후속 작업은 C11.2.3 제한 profile 완료 판정을 바꾸지 않고 Android 빌드 경로 정합성만 기록한다. 공개 API·내부 계약 0059의 ABI/의미 변경은 없다.
- 구현 실패 관점 검토와 최종 PR 변경 적대 검토는 각각 별도 문서에 기록한다: [구현 검토 20개](../spec/internal/evidence/c11-2-3-android-jni-gate-implementation-review-2026-10-19.md) · [PR 변경 검토 30개](../spec/internal/evidence/c11-2-3-android-jni-gate-pr-review-2026-10-19.md).

# Runtime GPU 픽스처 모듈·빌드 경계

## 상태

구현과 소스·산출물 검증을 완료했다. 제품용 `c04-runtime-gpu`에는 앱 검증용 FFI와 fixture 데이터가 들어가지 않는다. 앱 검증 harness는 `c04-runtime-gpu-fixture-test`를 명시적으로 켜서 사용한다. C11.2.3 전용 진입점은 그 위에 `c11-2-3-grid-fixture-test`를 추가로 요구한다. 순수 Rust 단위 테스트 fixture는 `#[cfg(test)]` 아래에 남기며 앱 검증용 `fixtures/` 모듈과 구분한다. CSS 기능의 구현·적합성 판정은 이 정리의 완료 범위가 아니다.

## 문제와 변경 범위

runtime GPU fixture scene adapter와 FFI 진입점이 일반 runtime 파일에 섞여 있었고, `include_str!` payload는 C04 GPU feature가 켜지면 앱 검증 harness와 함께 빌드됐다. 파일을 `fixtures/`로 옮기는 것만으로는 payload가 제품 artifact에서 제외되지 않으므로 모듈 책임과 Cargo/native/header gate를 함께 맞췄다.

- C04–C12 앱 검증 FFI를 `crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/` 아래 작업 ID 모듈로 옮겼다.
- Grid runtime scene adapter와 C04–C12 JavaScript·JSON `include_str!` 데이터, host fixture 생성자를 `runtime_gpu/fixtures/` 아래로 옮겼다. 이후 추가되는 앱 검증 장면도 이 경계를 따른다.
- UA cascade/layout 개발 probe를 `runtime_ua_cascade/fixtures/`로 옮기고 `c04-runtime-ua-cascade-fixture-test` opt-in feature에 묶었다.
- C11.2.3 adapter와 FFI entrypoint는 `c11-2-3-grid-fixture-test`에서만 컴파일한다. 다른 GPU fixture harness는 `c04-runtime-gpu-fixture-test`에서 컴파일한다.
- Android/iOS 검증 빌더의 Cargo feature, native compile define, C header 선언 조건을 동기화했다. 기본 빌드는 fixture ABI를 선언·제공하지 않는다.
- 순수 Rust 단위 테스트와 그 안의 fixture include는 기존 `#[cfg(test)]` 모듈에 둔다. 테스트 전용 데이터를 이유로 제품 runtime 모듈을 더 만들지 않는다.
- `runtime_gpu.rs`는 session·operation orchestration을 맡고, 화면 장면의 publish/invalidate 경계는 `runtime_gpu/presentation.rs`, Android/UIKit surface 생성·resize·draw·destroy는 `runtime_gpu/renderer.rs`로 나눴다. 둘은 실행 동작을 바꾸지 않는 내부 모듈이며 sibling 호출 범위는 `pub(super)`로 제한한다.
- 기존 C ABI 함수 이름, 인자·반환 계약, 장면 선택 동작은 유지했다.

## 빌드 계약

| 조합 | 포함 범위 |
| --- | --- |
| `c04-runtime-gpu` | 제품용 GPU runtime core. fixture adapter·fixture FFI·고정 JS/JSON payload 제외 |
| `c04-runtime-gpu-fixture-test` | 앱 검증용 runtime fixture adapter·FFI·`include_str!` payload opt-in |
| `c11-2-3-grid-fixture-test` | 위 fixture-test feature와 GPU core에 더해 C11.2.3 전용 adapter·FFI opt-in |
| Rust `#[cfg(test)]` / Cargo integration test | 해당 test artifact에만 unit-test fixture 입력 포함 |

앱 검증용 feature는 `#[cfg(test)]`에만 의존하지 않는다. 실제 Android/iOS 앱을 실행하는 harness는 Cargo와 native/header 정의에서 같은 opt-in 상태를 가져야 한다. 반대로 소스가 `fixtures/` 폴더에 있거나 fixture 원본이 `tests/fixtures/`에 있다는 사실만으로 제품 artifact에서 빠졌다고 판정하지 않는다.

## 검증 결과

- `cargo build --locked -p spinon-ffi --no-default-features --features c04-runtime-gpu`와 fixture-test positive control 빌드 통과.
- 2026-10-14 재검증에서 기본 FFI 테스트 18/18, `c04-runtime-gpu-fixture-test,c11-2-3-grid-fixture-test,c04-runtime-ua-cascade-fixture-test` 조합은 43 passed·2 ignored·0 failed였다. 같은 조합의 Clippy `--all-targets -- -D warnings`, `cargo fmt --all -- --check`도 통과했다.
- 2026-10-14 `spinon-ffi` Debug static library 대조에서 기본 `c04-runtime-gpu` artifact는 fixture ABI·payload marker를 포함하지 않았고, fixture-test positive control에는 지정된 C11.2.3·UA probe marker가 있었다. SHA-256과 byte 크기는 [산출물 재검증 근거](../spec/internal/evidence/runtime-gpu-fixture-module-boundary-2026-10-14/README.md)에 기록했다. 이는 모바일 release app bundle 검증이 아니다.
- 2026-10-14 iOS build-flag Node test 9/9와 rustfmt가 통과했다. fixture/build 경계와 runtime GPU 책임 분할을 각각 20개 실패 관점으로 확인하고 [구현 검토](../spec/internal/evidence/runtime-gpu-fixture-module-boundary-implementation-review-2026-10-14.md)에 기록했다. 계획 검토 20개 관점은 [별도 기록](../spec/internal/evidence/runtime-gpu-fixture-module-boundary-plan-review-2026-10-13.md)에 있다.

전체 workspace test·Clippy, iOS 실기기·OS compositor 이후 pixel 비교는 이 모듈·빌드 경계 변경에서 실행하거나 완료 판정하지 않았다. 플랫폼 앱 build-flag 계약은 검증했지만 final APK/IPA의 release stripping과 bundle 크기는 검사하지 않았다.

## 완료 기준과 범위 밖

- 기본/product feature artifact에 앱 검증용 FFI symbol 및 embedded fixture payload가 없어야 한다.
- 앱 검증 feature positive control에는 요청한 fixture ABI와 payload가 포함돼야 한다.
- Cargo feature, Android/iOS native compile define, C header declaration 조건이 서로 일치해야 한다.
- 테스트 전용 fixture include는 `#[cfg(test)]` 또는 integration test target의 artifact로 제한한다.
- 이 정리는 CSS Grid 동작, 공개 API, 전체 CSS 지원 또는 제품 앱 번들 크기를 판정하지 않는다.

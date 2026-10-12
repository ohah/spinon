# C11.2.1 · Android/iOS 모바일 runtime 행렬 · 2026-10-12

**범위:** 고정 Chrome 154 fixture 10개를 Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 앱 경로에서 실행했다. Android 실기기는 이번 확인 시 ADB에 없었고 iOS 실기기는 범위에서 제외했다.

## 결과

| 플랫폼 | 결과 | 고정 실행 파일 |
| --- | --- | --- |
| Android 17/API 37 AVD (`sdk_gphone16k_arm64`) | 10/10 scene · 성공 후보 6/6 · 기대 거부 4/4 · 21 node · 최대 오차 0 CSS px (허용 0.5) | [요약·비교·원본 로그·PNG](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-12/c1121-android-20261012T085353973Z-8de87b90/README.md) · APK SHA-256 `3ee517b8c0f6328b1b4cedb92460a1f5f410e8f04a864cb4d55f4e68aa0f3bc4` |
| iPhone 17 Pro/iOS 26.2 Simulator | 10/10 scene · 성공 후보 6/6 · 기대 거부 4/4 · 21 node · 최대 오차 0 CSS px (허용 0.5) | [요약·비교·원본 로그·PNG](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-12/c1121-iossim-20261012T085128948Z-0f8801df/README.md) · executable SHA-256 `ff91c126f9888d37d48befdfd04189d4ce1a1e36a92724d4000d8aa17c261e17` |

두 플랫폼에서 inventory, HTML fixture, Chrome reference hash가 동일하다. 각 scene은 새 앱 process와 HostDocument에서 실행했다. 성공 case의 전체 frame, draw 제출, overlap paint order와 기대 거부의 code·node·property 경계를 분리해 비교했다. 실행 중 확인된 named-line 거부는 child placement 단계가 아니라 `grid-template-columns` computed cascade 단계의 `cascade_failed`이며 matrix 계약을 그 결과에 맞췄다.

Android AVD는 ANGLE/Vulkan/SwiftShader 소프트웨어 renderer다. Simulator 화면 PNG는 장면 실행을 확인하는 보조 근거이며 pixel parity가 아니다. 전체 10-scene surface pixel matrix는 별도 미완료 관문이다.

## 회귀 검사

- 고정 입력 생성 검사와 negative controls: 3/3 통과.
- 런타임 비교기 정상/실패 fixture 검사: 4/4 통과.
- 전체 CSS reference 도구 검사: 327/327 통과.
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu`: 37 passed, 2 ignored.
- `mise exec -- bun run test` 최신 로그: 예제 JS 2/2, CSS reference 327/327, Android touch 도구 29/29, Rust workspace 618 passed·4 ignored, Release 1,024-depth worker 경계 1/1.
- 별도 `RUST_MIN_STACK=33554432` 1,024-node document→cascade→layout→paint graph stress 통과. 단계 시간은 [deep-graph-1024.log](deep-graph-1024.log)에 있다. 이 host test는 모바일 8 MiB worker stack 증거와 구분한다.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `git diff --check`, Android/iOS runner syntax 검사 통과.
- 생성기 `--check`와 3개 generator test, 4개 comparator test 통과. 전체 출력은 [generator check](generator-check.log), [generator tests](generator-tests.log), [comparator tests](comparator-tests.log), [rustfmt](rustfmt.log), [diff check](diff-check.log), [Clippy](clippy.log), [FFI feature test](ffi-feature-test.log), [전체 test](full-test.log)에 보존했다.

## 아직 미완료인 범위

- Android 실기기 실행은 현재 ADB에 기기가 없어 미실행이다. AVD 결과를 실기기 GPU 증거로 일반화하지 않는다.
- C11.2.1 전체 10-scene surface PixelCopy/Metal readback과 Spinon 자체 WPT runner/적합성은 이 runtime matrix의 결과가 아니다.
- named lines/areas, implicit tracks, mixed auto/explicit placement, 전체 CSS Grid 및 공개 CSS 지원은 별도 범위다.

실패 경계 검토: [20개 구현·수집 실패 관점](implementation-review-2026-10-12.md) · [계획 단계 검토](../c11-2-1-mobile-runtime-matrix-plan-review-2026-10-12.md).

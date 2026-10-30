# C11.4 anonymous Grid text `white-space:pre-line` 실행 근거

이 문서는 [계획](../../../../plan/c11-4-anonymous-grid-text-pre-line.md)의 제한 구현 후보를 기록한다. 새 범위는 기존 opt-in C11.4 anonymous Grid text route에 normal LF 대조군과 `white-space:pre-line` 장면을 더한 것이다. 일반 CSS author admission, 공개 API, 전체 C11.4/C11 구현으로 승격하지 않는다. 내부 계약 숫자 버전 `0.1.0`은 유지한다.

## 고정 입력과 기대값

[Chrome 156 Agent Browser 사전 비교](../c11-4-anonymous-text-pre-line-precomparison-2026-10-31/README.md)의 Ahem 10px 관측과 CSS Text/Grid/Box Sizing 규칙에서 계산한 기대값을 사용한다. source text는 두 scene 모두 `XX\nXX`, root width는 20px, column은 `max-content`, row는 `auto`다.

| 장면 | White-space profile | Columns / Rows | Root / text item |
| --- | --- | --- | --- |
| `anonymous-normal-newline-max-content` | `collapse` / `wrap` | `[50px]` / `[10px]` | root `20×10px`, item `50×10px` |
| `anonymous-pre-line-max-content` | `preserve-breaks` / `wrap` | `[20px]` / `[20px]` | root/item `20×20px` |

두 결과는 LF를 접는 Normal과 강제 줄바꿈으로 남기는 PreLine을 구별한다. Root 너비가 20px인 normal 대조군은 max-content track과 item이 root 오른쪽으로 30px 넘친다. 이는 의도한 track 크기이며 overflow clipping이나 glyph pixel 검증은 아니다.

## 구현 범위

- Anonymous Grid text style gate에 `WhiteSpaceMode::PreLine`을 추가하고, `compute.rs` validation과 `text_paragraphs.rs` collection allowlist를 일치시켰다. 기존 `order:0`, HostDocument owner/source/revision 검사와 unsupported profile 거부를 유지한다.
- Runtime fixture는 고정 scene ID, 직접 text source, `preserve-breaks` computed style과 `PreLine` paragraph style을 일치시킨다. FFI/runtime scene matrix는 7개에서 9개로 늘었다.
- White-space Normal/NoWrap/PreLine layout tests는 `host_document_tests/c11_4_anonymous_text/white_space.rs` 하위 모듈로 모아 기존 fixture test module을 500줄 아래로 유지했다.
- Android/iOS 검증기는 root/item frame, intrinsic inputs, `Normal`/`PreLine` profile, Columns·Rows trace 순서와 최종 track/contribution을 검사한다. Capture matrix는 624개에서 626개 scene으로 늘었고 새 구조 검사 24개가 전체 경계를 확인한다.
- 변경은 기존 fixture-test feature 경로에만 연결된다. 일반 CSS cascade나 임의 HTML source에서 `white-space`를 새로 허용하지 않는다.

## Rust·도구 검증

| 검증 | 결과 |
| --- | --- |
| `spinon-layout` anonymous text tests, `c11-4-grid-track-sizing-fixture-test` | 12 passed |
| `spinon-runtime` anonymous text tests, `c11-4-grid-track-sizing-fixture-test` | 5 passed |
| FFI fixed nine-scene matrix, `c04-runtime-gpu-fixture-test` | 1 passed |
| `fixture-module-boundary.test.mjs` | 24 passed, 0 failed |
| `cargo fmt --all -- --check`, `git diff --check` | 통과 |
| workspace default Clippy, `-D warnings` | 통과 |
| layout/runtime/FFI opt-in fixture-feature Clippy, `-D warnings` | 모두 통과 |
| Android/iOS fixture app build | 모두 성공; [Android log](./validation/android-app-build.log), [iOS log](./validation/ios-app-build.log) |

원본 로그는 [validation](./validation/)에 보관한다. 첫 layout 실행에서 `compute.rs`의 두 번째 profile validator가 `Normal | NoWrap`만 허용해 pre-line 장면이 거부됐다. validator에 `PreLine`을 함께 추가했고 최종 layout tests 12개가 통과했다. Capture scene catalog를 626개로 늘린 뒤 fixture boundary 검사에 남아 있던 이전 `624` 기대값도 갱신하고 새 두 scene route 검사를 추가했다. Android report validator에서 malformed pass/stage 배열이 null 참조를 일으킬 수 있는 경로도 fail-closed로 바꿨다. 수정 후 Android/iOS 앱을 재빌드하고 양쪽 장면을 다시 캡처했다. 최종 구조 검사는 24/24다.

## Android·iOS 시뮬레이터

앱은 `SPINON_ENABLE_C04_RUNTIME_GPU=1` 및 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`로 빌드했다. 실행은 API 37 ARM64 Android AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 두 장면 모두 통과했다.

| 플랫폼 | Normal control | PreLine | 설치 artifact 일치 |
| --- | --- | --- | --- |
| Android API 37 ARM64 AVD `emulator-5554` / `sdk_gphone64_arm64` | root `20×10`, Columns `[50]`, Rows `[10]`; 7 trace chunks | root `20×20`, Columns `[20]`, Rows `[20]`; 7 trace chunks | APK SHA-256 `a226821bd80351187608be5259ca9e22a6187562fdd2e819e77903c93dadcf6d` 일치 |
| iPhone 17 Pro (`iPhone18,1`) / iOS 26.2 Simulator | root `20×10`, Columns `[50]`, Rows `[10]`; 13 trace chunks | root `20×20`, Columns `[20]`, Rows `[20]`; 13 trace chunks | App SHA-256 `af34ac31880a6ddd5b7aa475a1b249eed2a6600217f2ca9978a36200fb9165b9` 일치 |

두 플랫폼 모두 scene당 `drawPresented=true`, `supersededRetries=0`이다. 최종 원본 [Android 결과](./android-avd-c114-anonymous-grid-text-android-avd-20261030T162939857Z-b07d44d3/result.json), [Android source/device receipt](./android-avd-c114-anonymous-grid-text-android-avd-20261030T162939857Z-b07d44d3/simulator.json), [iOS 결과](./ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T162947943Z-36a95517/result.json), [iOS source/device receipt](./ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T162947943Z-36a95517/simulator.json)를 확인한다. 두 receipt의 source snapshot SHA-256은 `d0f5044a02b908b5ed8c7ecb8bb0cf08686c494d7a8974a62b60defb00b57942`이며 tracked diff hash는 `b2ad2486ac4a98548ca892872808e6a1f4d83a915ec2348d6f29c8008646a39c`다. 기준 commit은 `374d22f8bf358f0de81cca0ce5e62b44d56714e8`이다.

앱 화면은 fixture harness가 실행된 상태를 보여준다. 이 profile은 glyph paint를 켜지 않아 viewport는 검정색이며, 이미지는 글자 렌더링·pixel equality 증거가 아니다. 최종 Android PNG는 1080×2400, SHA-256 `dc0d87d54d03c98c2911a67a072c43d0efe8ae41c688f210afb14557e14f9a7c`이고 iOS PNG는 1206×2622, SHA-256 `284c7054f1b9728a4cdd15c11a40057184f17c3be24445fed8e1634ef7efa85b`다. [Android 화면](./android-avd-c114-anonymous-grid-text-android-avd-20261030T162939857Z-b07d44d3/anonymous-pre-line-max-content.png) · [iOS 화면](./ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T162947943Z-36a95517/anonymous-pre-line-max-content.png).

## 기본/fixture FFI archive 경계

macOS ARM64, Rust 1.96.1 release static archive에서 product feature `c04-runtime-gpu`와 fixture-test positive control `c04-runtime-gpu-fixture-test`를 별도 빌드했다. [명령 로그, scanner 및 JSON receipt](./artifact-boundary/README.md)를 참조한다.

| 조합 | 크기 | SHA-256 | 새 scene marker | C11.4 host FFI symbol |
| --- | ---: | --- | ---: | ---: |
| Product | 75,242,120 bytes | `68aec6b378914d7a70640b20fcdd32bf41c606fca46ebd6acff1be1c40f4d2ec` | 0/2 | 0/2 |
| Fixture-test positive control | 80,929,104 bytes | `6cc04bf48e26b54b80f4ff28e44f8c1f8d597a6dc351f03ab1342a0b9a45271a` | 2/2 | 2/2 |

이 scan은 두 scene string과 기존 C11.4 host constructor/eval export만 검사한다. 전체 runtime 내부 fixture symbol, APK/IPA 번들 또는 release 앱의 전체 fixture 제외 여부를 증명하지 않는다.

## 상태와 제한

C11.4와 상위 C11 checkbox는 미완료다. 공식 완료 집계는 `72/213` 그대로다. Android AVD는 hardware GPU나 실기기가 아니며, iOS drawable은 compositor 제출 이후 pixel이 아니다. 실제 glyph pixels, 일반 author CSS, WPT 적합성, 임의 폰트/스크립트, 다른 `white-space` mode는 범위 밖이다.

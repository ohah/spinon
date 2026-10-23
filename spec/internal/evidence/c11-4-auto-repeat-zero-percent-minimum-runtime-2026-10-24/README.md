# C11.4 `0%` auto-repeat minimum 제한 runtime 검증

## 판정

`repeat(auto-fill|auto-fit, minmax(0%, 1fr))`의 제한 구현을 추가했다. 반복 횟수를 계산할 때만 각 repeated track의 기여값에 1 CSS px floor를 적용한다. 실제 track minimum은 0%로 유지하고 auto-fit의 빈 track은 placement 이후 계속 collapse한다. C11.4 전체 CSS Grid 구현이나 공개 CSS 지원 완료가 아니다.

13개 고정 scene을 Android API 37 ARM64 AVD와 iPhone 17/iOS 26.2 Simulator에서 각각 13/13 통과했다. 각 scene의 track count·used size·collapse·node frame·trace와 draw/present를 검사했다. 이 캡처는 GPU scene 표시 근거다. PixelCopy/Metal pixel equality, Android hardware GPU, 실기기 및 iOS compositor 이후 화면은 검사하지 않았다.

## 구현 경로

- `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs`: computed percentage minimum이 유한한 0 이상이면 기존 supported-maximum gate와 함께 auto-repeat typed snapshot으로 보낸다.
- `crates/spinon-layout/src/grid.rs`: 0%도 기존 minimum-size·maximum-size gate, 양수 definite content axis 및 finite resolved value 검사를 거쳐서만 허용한다.
- Stylo가 `calc(0px + 0%)`를 typed `Percentage(0)`으로 정규화하는 경우도 같은 제한 경로를 따른다. 별도 calc evaluator는 없으며 음수·미지원 calc 형태는 계속 거부한다.
- `vendor/taffy/src/compute/grid/explicit_grid.rs`: 반복 그룹 안의 각 track contribution을 count 산정에서만 1px 이상으로 올린다. explicit prefix/suffix와 실제 track sizing은 바꾸지 않는다. invalid/비유한 입력과 음수 repeated size는 division·float-to-int 변환 전에 보수적인 1회 반복 경로로 보낸다.
- 앱 fixture는 기존 opt-in C11.4 runtime 경로를 사용한다. 새 공개 ABI를 추가하지 않았다. Android/iOS에 scene 별 root/item 기대값도 연결했다.

## 실행 결과

| 대상 | 환경 | 결과 | 근거 |
| --- | --- | ---: | --- |
| Android | API 37 ARM64 AVD `emulator-5558`, `sdk_gphone64_arm64` | 13/13 | [최종 scene 결과](android-avd-c114-auto-repeat-zero-percent-minimum-android-avd-20261023T170458589Z-c0537217/result.json) · [최종 실행 화면](android-avd-c114-auto-repeat-zero-percent-minimum-android-avd-20261023T170458589Z-c0537217/fit-col-first.png) |
| iOS | iPhone 17, iOS 26.2 Simulator | 13/13 | [최종 scene 결과](ios-simulator-c114-auto-repeat-zero-percent-minimum-ios-simulator-20261023T170458589Z-b80ae17b/result.json) · [최종 실행 화면](ios-simulator-c114-auto-repeat-zero-percent-minimum-ios-simulator-20261023T170458589Z-b80ae17b/fit-col-first.png) |

Chrome 156 / Agent Browser의 13-scene 사전 비교와 독립 기대값은 [구현 전 비교 기록](../c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/README.md)에 있다. 계획과 구현 범위는 [집중 계획](../../../../plan/c11-4-auto-repeat-zero-percent-minimum.md)을 따른다.

## 검사

- `CARGO_INCREMENTAL=0 cargo test --locked --workspace` — 최종 코드에서 26개 test suite, 1,243 passed, 0 failed, 4 ignored ([원본 로그](builds/workspace-test-final.log)).
- `cargo clippy --locked --workspace --all-targets -- -D warnings` — 최종 코드에서 성공 ([원본 로그](builds/workspace-clippy-final.log)).
- `cargo fmt --all -- --check` — 성공.
- C11.4 style·layout·FFI focused tests — 성공. 상세 로그는 [`builds/`](builds/)에 있다.
- `node tools/css-reference/fixture-module-boundary.test.mjs` — 최종 capture harness를 포함해 7/7 성공 ([원본 로그](builds/fixture-module-boundary-final.log)). scene 목록이 Android/iOS allowlist와 frame expectation block, 완전한 trace chunk 대기·검증에도 연결됐는지 검사한다.
- 최종 product `spinon-ffi` archive (75,137,640 bytes, SHA-256 `16bf2144a2d8f85a6297c009c2885aa8e9af8f1db6c7b029f2ecae43ee752921`)에는 scene ID와 `spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture` marker가 없다. `c04-runtime-gpu-fixture-test` positive control (78,217,904 bytes, SHA-256 `446ffed6e1b207fde8dc8c50da077d64e431261cc1c91e24946e10a7e7c74b4b`)에는 둘 다 있다 ([negative scan](builds/ffi-product-archive-marker-scan-final.json) · [positive scan](builds/ffi-fixture-archive-marker-scan-final.json)). 이 검사는 static archive 경계만 확인하며 APK/IPA 전체에서 fixture가 제거됐다는 뜻은 아니다.

초기 Android 실행에서 cascade의 별도 auto-repeat gate가 0%를 거부하는 결함을 찾아 수정했다. 이어서 두 플랫폼 앱 검증기가 새 scene의 기본 root/item frame을 사용하던 결함을 찾아 scene별 기대값을 연결했다. 최종 재검증에서는 Android 로그 burst 중 trace chunk 세 개가 빠지는 계측 수집 문제를 발견해 opt-in logger 전송 간격을 두고 capture가 전체 part set을 기다리도록 고쳤다. 최종 AVD/iOS 행렬의 모든 scene은 trace chunk 전체를 복원해 통과했다. 초기 실패 로그는 이전 `android-avd-*` capture 결과 및 `builds/*failure*` 파일에 보존했다. 최종 앱 build·capture 로그와 source/artifact hash는 [`builds/`](builds/) 및 [source revision](source-revision.json)에 있다.

## 적용 범위와 남은 한계

이 slice는 `minmax(0%, 1fr)` 중심의 opt-in fixture profile이다. indefinite/zero/cyclic content basis, 음수·비유한 percentage, 미지원 maximum/calc/track grammar는 계속 fail-closed다. WPT, 일반 authored CSS admission, 전체 C11.4, 실기기, hardware GPU 및 compositor 이후 pixel equality는 검증하지 않았다. 내부 계약 0059는 `0.1.0`으로 유지하고 공식 C11.4 checkbox 및 `72/213` 완료 수는 올리지 않는다.

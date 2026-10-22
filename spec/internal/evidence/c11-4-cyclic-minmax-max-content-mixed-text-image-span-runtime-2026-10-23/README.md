# C11.4 cyclic max-content 혼합 text/image span 실행 근거

2026-10-23에 고정 네 장면을 Rust layout/style 검증, Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator에서 확인했다. CSS와 기대값은 [계획](../../../plan/c11-4-cyclic-minmax-max-content-mixed-text-image-span.md), 사전 oracle은 [Chrome 156 비교](../c11-4-cyclic-minmax-max-content-mixed-text-image-span-precomparison-2026-10-23/README.md)에 있다.

## 결과

| 장면 | root border box | Columns | Rows | Android | iOS |
| --- | --- | --- | --- | --- | --- |
| `max-content-forward` | `114 × 92/3` | `[107/3,125/3,86/3]` | `[37/3,37/3]` | 통과 | 통과 |
| `max-content-reverse` | `114 × 92/3` | `[107/3,125/3,86/3]` | `[37/3,37/3]` | 통과 | 통과 |
| `definite-forward` | `44 × 44` | `[12,12,12]` | `[19,19]` | 통과 | 통과 |
| `definite-reverse` | `44 × 44` | `[12,12,12]` | `[19,19]` | 통과 | 통과 |

max-content content box는 `110 × 80/3px`다. 양방향 source order에서 ID별 frame은 같다. definite scene에서 image B는 `x=16px`, width `40px`로 root border 오른쪽을 `12px` 넘는다. 각 장면은 track·frame·trace와 draw/present를 확인했다.

각 definite scene은 세 표본을 검사했다. (1) CSS `(8,0)`의 top border `[240,198,116,255]`, (2) root 밖 image texel `(50,6)`의 초록 `[0,255,64,255]`, (3) image 뒤 `(58,6)`의 clear `[0,0,0,255]`. Android PixelCopy와 iOS compositor 제출 전 Metal drawable에서 각 플랫폼 `6/6` sample이 exact다. 세 지점은 지정된 색/경계를 확인하며 전체 border stroke 비교가 아니다.

앱은 glyph를 그리지 않는다(`runtime_text_count=0`). Ahem text의 줄 수·줄 너비와 intrinsic input은 layout trace에서만 확인했다. Simulator 화면은 geometry와 이미지 overflow를 보여주며 glyph raster 결과가 아니다.

## Simulator 자료

- Android AVD: API 37 ARM64 전용 `spinon_c114_cyclic_image_span_20261023`, ADB `emulator-5562`. [실행 결과 JSON](android-avd-c114-cyclic-max-content-mixed-text-image-span-android-avd-20261022T192950115Z-e7f5e7d8/result.json)
- iOS Simulator: iPhone 17 Pro, iOS 26.2, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`. [실행 결과 JSON](ios-simulator-c114-cyclic-max-content-mixed-text-image-span-ios-simulator-20261022T193154129Z-a392169c/result.json)

| 장면 | Android 화면 | iOS 화면 |
| --- | --- | --- |
| max-content forward | [PNG](android-avd-c114-cyclic-max-content-mixed-text-image-span-android-avd-20261022T192950115Z-e7f5e7d8/max-content-forward.png) | [PNG](ios-simulator-c114-cyclic-max-content-mixed-text-image-span-ios-simulator-20261022T193154129Z-a392169c/max-content-forward.png) |
| max-content reverse | [PNG](android-avd-c114-cyclic-max-content-mixed-text-image-span-android-avd-20261022T192950115Z-e7f5e7d8/max-content-reverse.png) | [PNG](ios-simulator-c114-cyclic-max-content-mixed-text-image-span-ios-simulator-20261022T193154129Z-a392169c/max-content-reverse.png) |
| definite forward | [PNG](android-avd-c114-cyclic-max-content-mixed-text-image-span-android-avd-20261022T192950115Z-e7f5e7d8/definite-forward.png) | [PNG](ios-simulator-c114-cyclic-max-content-mixed-text-image-span-ios-simulator-20261022T193154129Z-a392169c/definite-forward.png) |
| definite reverse | [PNG](android-avd-c114-cyclic-max-content-mixed-text-image-span-android-avd-20261022T192950115Z-e7f5e7d8/definite-reverse.png) | [PNG](ios-simulator-c114-cyclic-max-content-mixed-text-image-span-ios-simulator-20261022T193154129Z-a392169c/definite-reverse.png) |

실행별 `runtime-*.log`에는 원본 runtime summary, trace, frame, draw/present 및 pixel sample 결과가 있다. `simulator.json`, `installed-apk-verification.json`, 캡처 PNG와 `result.json`은 같은 실행 폴더에 있다.

## 검증 명령과 archive 경계

- Rust: `cargo test --locked -p spinon-style-to-layout --features c11-4-grid-track-sizing-fixture-test cyclic_minmax_max_content_mixed_text_image_span_matches_fixed_geometry_and_trace -- --nocapture` 및 이전 mixed fixture 회귀 테스트가 각각 통과했다. Runtime adapter와 FFI의 8-scene allowlist 테스트, 관련 세 crate의 fixture-enabled Clippy, workspace Clippy, `cargo fmt --all -- --check`, capture script의 `node --check`, `git diff --check`도 통과했다. 로그는 [`artifact-boundary/`](artifact-boundary/)에 있다.
- Android fixture build는 `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`로 성공했다. iOS fixture build는 여기에 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`도 켜서 성공했다. fixture debug APK와 iOS Simulator executable의 SHA-256은 각각 `7f9a7c31c2a7d76d4c604ac96a56427113da969fc1ee40618f222b90e903038a`와 `0d1788a1775a0cc08f903ff40864cf23b2ed52edcfe14ad84dd796663e3ca8b5`다. 이 둘은 fixture-enabled 실행 산출물이다.
- 제품 core 비교 archive: `aarch64-apple-ios-sim`, `--no-default-features --features c04-runtime-gpu`, SHA-256 `4f8e2aee3581cd8aaed1672c63557c6f3676a02efeff5585dc1ba0c9b8819e82`, `74,800,232` bytes. `strings`와 LLVM 22.1.8 `llvm-nm --demangle -g --defined-only`에서 새 scene ID·image ID·오류 marker, `cyclic_minmax_calc_mixed_text_image_span::runtime_source`, C11.4 fixture host create/eval symbol이 없었다.
- fixture-test positive archive: `--features c04-runtime-gpu-fixture-test,c11-grid-metal-surface-pixels-test`, SHA-256 `0df401411d73222b71cbc2eabb169afd368aa4a6d59f4655a1680cad6c919211`, `77,847,104` bytes. 같은 검사에서 네 scene ID, 두 image ID, `unsupported_c114_cyclic_mixed_text_image_span_fixture`, fixture `runtime_source`와 host create/eval symbol이 있었다.

두 static archive는 비교 후 보관하지 않았다. 기록된 hash는 각 빌드 직후 계산했다. 이 검사는 static archive feature 경계만 입증한다. 최종 release APK/IPA에서 fixture 코드나 payload가 제외되는지는 이 실행에서 비교하지 않았다.

## 재시도 기록

- Android의 첫 캡처는 자동 빌드하지 않는 capture script가 이전 APK를 설치해 새 scene ID에서 실패했다. fixture flag로 APK를 다시 빌드했다.
- 다음 Android trace 검증은 첫 cyclic Columns pass의 최종 `[24,34,48]`과 resolved Columns를 하나로 취급해 막혔다. Android/iOS/JS 검증기에 pass별 값을 분리했다.
- 다음 Android 검증은 JS 장면 기대값 object가 width boolean과 MaxContent contribution map에 같은 `maxContent` 키를 써서 definite scene에도 max-content track을 요구했다. width 판정을 `maxContentWidth`로 분리하고 네 장면을 다시 실행했다.
- iOS 첫 실행은 Metal pixel fixture compile flag가 꺼져 `status=-90`을 반환했다. `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`로 재빌드한 뒤 네 장면이 통과했다.

각 실패 실행의 로그는 같은 날짜 evidence group의 별도 run 폴더에 남아 있다. 최종 판정에는 위 표의 성공 실행만 사용했다. Android 실기기/hardware GPU, iOS compositor 이후 픽셀, WPT와 일반 CSS admission은 확인하지 않았으며 C11.4 완료를 뜻하지 않는다.

## PR #526 병합

구현 PR [#526](https://github.com/ohah/spinon/pull/526)은 `2026-10-23 04:48:48 KST`에 Rebase and merge되어 merge SHA `b87c2a7d5e68434335fed4418c9f95a3a0353c6f`를 남겼다. 병합 당시 status-check rollup은 비어 있었고 GitHub `main`에는 required status check이 설정되어 있지 않았다. 이 변경은 opt-in fixture slice이며 C11.4와 공식 `77/213`은 계속 미완료다.

- [구현/runtime 실패 관점 검토](implementation-runtime-review-20.md)
- [20개 scene·feature source 테스트 결과](artifact-boundary/ffi-scene-test.log)
- [PR #526 diff 실패 관점 검토 30개](pr-diff-review-30.md)
- [PR #526 화면의 Agent Browser 확인·프로세스 정리](pr526-agent-browser-verification.json) · [확인 화면](pr526-agent-browser-images.png)

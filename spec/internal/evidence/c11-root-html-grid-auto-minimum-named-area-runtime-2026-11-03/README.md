# C11.1 Root HTML named-area 자동 최소 크기 실행 근거

## 적용 범위

기존 `RuntimeRootHtmlTextPaintV1` 기본 Root HTML 경로에서 named-area 지원을 다음 여섯 개의 고정 scene으로 확장한 구현 후보를 기록한다. 새 CSS 파서, runtime profile, FFI 진입점 또는 product ABI는 추가하지 않았다. 자동 최소 크기 track admission은 기존 exact `header`/`side`/`content` 2×2 area map과 definite content-box 조건에 묶여 있다.

기준 fixture는 [source/fixture.html](./source/fixture.html)이다. `prepare-scene.mjs`가 fixture의 네 inline style만 바꾸어 여섯 정적 Root HTML 입력을 만들며, 각 입력은 [source/scenes](./source/scenes/) 아래 보존한다. Chrome precomparison에서 먼저 고정한 독립 기대값과 실행 전 Agent Browser 관측은 [전용 계획](../../../../plan/c11-root-html-grid-auto-minimum-named-areas.md) 및 [precomparison evidence](../c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/README.md)에 있다.

## 판정 행렬

각 root/item frame은 CSS px 기준이다. Chrome Agent Browser의 구현 후 DOMRect, used tracks, sample point의 element와 scroll extents는 [chrome-post-implementation-observation.json](./chrome-post-implementation-observation.json)에, 네이티브 sample RGBA·화면 경계·run ID와 artifact hash는 [platform-observations.json](./platform-observations.json)에 기록했다.

| 장면 | 첫 track | used tracks | root border box | content item | root border 밖 overflow | columns 표본 `(110,30)` | rows 표본 `(50,110)` |
| --- | --- | --- | --- | --- | ---: | --- | --- |
| columns-auto-one | `minmax(auto,1fr)` | columns `[80,40]`, rows `[20,20]` | `[0,0,104,44]` | `[82,22,40,20]` | 18px | 파랑 `#70a8d8` | 흰색 |
| columns-zero-one | `minmax(0,1fr)` | columns `[60,40]`, rows `[20,20]` | `[0,0,104,44]` | `[62,22,40,20]` | 0px | 흰색 | 흰색 |
| columns-auto-zero | `minmax(auto,0fr)` | columns `[80,40]`, rows `[20,20]` | `[0,0,104,44]` | `[82,22,40,20]` | 18px | 파랑 `#70a8d8` | 흰색 |
| rows-auto-one | `minmax(auto,1fr)` | columns `[40,40]`, rows `[80,40]` | `[0,0,84,104]` | `[42,82,40,40]` | 18px | 흰색 | 파랑 `#70a8d8` |
| rows-zero-one | `minmax(0,1fr)` | columns `[40,40]`, rows `[60,40]` | `[0,0,84,104]` | `[42,62,40,40]` | 0px | 흰색 | 흰색 |
| rows-auto-zero | `minmax(auto,0fr)` | columns `[40,40]`, rows `[80,40]` | `[0,0,84,104]` | `[42,82,40,40]` | 18px | 흰색 | 파랑 `#70a8d8` |

Android/iOS 표면 검사는 장면별로 이 두 root-relative point의 RGBA만 확인한다. Root/item의 전체 수치 geometry와 border 밖으로 나간 item frame은 Rust runtime assertion이 각각 ±0.02 CSS px 안에서 검사한다. Simulator screenshot은 배치의 시각 자료이며, screenshot 색 경계를 geometry oracle로 취급하지 않는다.

## Agent Browser와 Rust 검증

- Agent Browser 0.31.1 / HeadlessChrome 156, viewport `1280×633`, DPR 1에서 여섯 current-source HTML을 확인했다. Computed used tracks, root/item DOMRect, scroll extent 및 sample point의 `content` hit 결과가 기대값과 일치했다. [현재 입력 관측 JSON](./chrome-post-implementation-observation.json)과 [구현 전 Chrome 캡처](../c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/chrome/chrome156-agent-browser.png)을 함께 참고한다.
- `cargo fmt --all -- --check`: 통과.
- `cargo test --locked -p spinon-style-to-layout root_html_named_area --lib`: 14/14 통과. 양 축의 auto/zero/0fr matrix, item/container min/max·aspect ratio, area/tree boundary와 기존 named-area rejection을 포함한다.
- `cargo test --locked -p spinon-style-to-layout parsed_root_html_auto_minimum_flex_tracks_reach_the_default_profile_on_both_axes --lib`: 1/1 통과.
- `cargo test --locked -p spinon-style-to-layout fixed_named_area_accepts_percentage_and_existing_px_gaps --lib`: 1/1 통과.
- `cargo test --locked -p spinon-style-to-layout root_html_profile_still_rejects_intrinsic_tracks_with_fixed_implicit_rows --lib`: 수정 후 1/1 통과. 검사에서 fixed implicit row와 no-area `minmax(auto,fr)` 조합이 잘못 투영되는 기존 경계 문제를 재현했다. 해당 조합만 거부하도록 고쳤고 named-area 기본-auto 경로는 별도 테스트로 확인했다.
- `cargo test --locked -p spinon-style-to-layout root_html_profile --lib`: 19/19 통과. 기존 no-area explicit/fixed-implicit row profile 회귀를 확인한다.
- `cargo test --locked -p spinon-runtime root_html_named_area --lib`: 2/2 통과. 여섯 정적 입력의 item frame, overflow, style/layout key identity와 unsupported input의 partial layout 부재를 확인한다.
- `cargo test --locked -p spinon-runtime root_html_grid --lib`: 68/68 통과.
- `cargo clippy --locked -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings`: 통과.

실제 명령 출력은 [rust-validation](./rust-validation/) 아래 둔다. 이 검증은 전체 workspace test/Clippy 통과를 뜻하지 않는다.

## Android 및 iOS Simulator 실행

각 platform에서 여섯 장면을 독립 앱 빌드·설치·실행했다. `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`은 이미 존재하던 opt-in surface capture 경로에만 사용했고 다른 fixture-test flags는 껐다. Android PixelCopy와 iOS Metal drawable은 장면마다 두 표본을 모두 exact 비교했다. 최종 코드로 다시 빌드한 장면별 run ID, summary, PNG, APK/executable hash는 [최종 Android 실행](./pass-final/android/)과 [최종 iOS 실행](./pass-final/ios/)에 있다. 최초 실행 산출물은 덮어쓰지 않고 이 폴더의 `android/`와 `ios/`에 보존했다.

### Android

API 37 / Android 17 / ARM64 `sdk_gphone64_arm64` AVD `emulator-5556`, density 2.625에서 SurfaceView PixelCopy 12/12 sample이 exact였다. renderer는 ANGLE/Vulkan/SwiftShader software 경로다. 이는 실기기나 hardware GPU 결과가 아니다. PixelCopy PNG는 `1080×2209`이며 기기 screenshot은 system UI를 포함한다.

| 장면 | PixelCopy surface | Simulator screenshot |
| --- | --- | --- |
| columns-auto-one | [PNG](./pass-final/android/columns-auto-one-pixelcopy.png) | [화면](./pass-final/android/columns-auto-one-simulator.png) |
| columns-zero-one | [PNG](./pass-final/android/columns-zero-one-pixelcopy.png) | [화면](./pass-final/android/columns-zero-one-simulator.png) |
| columns-auto-zero | [PNG](./pass-final/android/columns-auto-zero-pixelcopy.png) | [화면](./pass-final/android/columns-auto-zero-simulator.png) |
| rows-auto-one | [PNG](./pass-final/android/rows-auto-one-pixelcopy.png) | [화면](./pass-final/android/rows-auto-one-simulator.png) |
| rows-zero-one | [PNG](./pass-final/android/rows-zero-one-pixelcopy.png) | [화면](./pass-final/android/rows-zero-one-simulator.png) |
| rows-auto-zero | [PNG](./pass-final/android/rows-auto-zero-pixelcopy.png) | [화면](./pass-final/android/rows-auto-zero-simulator.png) |

### iOS

iPhone 17 Pro / iOS 26.2 Simulator (`ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`), scale 3에서 Metal drawable 표본 12/12 exact였다. Drawable은 `1206×2334`이며 OS compositor 제출 전 값이다. iOS 화면 캡처는 system UI를 포함해 `1206×2622`다. 실기기 및 compositor 이후 pixel은 검증하지 않았다.

| 장면 | Metal sample log | Simulator screenshot |
| --- | --- | --- |
| columns-auto-one | [로그](./pass-final/ios/spinon-columns-auto-one.log) | [화면](./pass-final/ios/columns-auto-one-simulator.png) |
| columns-zero-one | [로그](./pass-final/ios/spinon-columns-zero-one.log) | [화면](./pass-final/ios/columns-zero-one-simulator.png) |
| columns-auto-zero | [로그](./pass-final/ios/spinon-columns-auto-zero.log) | [화면](./pass-final/ios/columns-auto-zero-simulator.png) |
| rows-auto-one | [로그](./pass-final/ios/spinon-rows-auto-one.log) | [화면](./pass-final/ios/rows-auto-one-simulator.png) |
| rows-zero-one | [로그](./pass-final/ios/spinon-rows-zero-one.log) | [화면](./pass-final/ios/rows-zero-one-simulator.png) |
| rows-auto-zero | [로그](./pass-final/ios/spinon-rows-auto-zero.log) | [화면](./pass-final/ios/rows-auto-zero-simulator.png) |

최종 iOS screenshot 실행 중 `simctl`이 `getpwuid_r did not find a match for uid 501`을 출력했지만 screenshot 파일 여섯 개가 기록됐고 capture script는 exit 0으로 종료했다. 오류 문구를 임의로 성공 판정에 더하지 않고, 파일 존재·Metal sample summary·script exit code를 각각 보존했다.

## Opt-in 경계

capture adapter 수정은 기존 C11 surface pixel test feature 안에서만 호출된다. Android 기본 debug APK는 `BuildConfig.SPINON_C11_GRID_METAL_SURFACE_PIXELS_TEST=false`, `BuildConfig.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST=false`이고 `assets/index.html`이 복원 원본과 byte-identical이다. APK assets에 named-area fixture나 scene payload 경로가 없었다. iOS 기본 Simulator build는 두 feature를 `0`으로 설정했으며 default executable에서 새 auto-minimum capture argument, `c11na-` scene marker 문자열을 찾지 못했다. [기본 artifact 대조 결과](./artifact-boundary/result.txt), [Android build log](./artifact-boundary/android-default-build.log), [iOS build log](./artifact-boundary/ios-default-build.log).

## 재현 및 범위 제한

- `bash spec/internal/evidence/c11-root-html-grid-auto-minimum-named-area-runtime-2026-11-03/capture-simulators.sh`는 저장된 여섯 입력을 차례로 빌드하고 실행한다. 개별 장면 실행은 예를 들어 마지막 인자로 `rows-auto-one`을 전달한다.
- 실행 script는 저장 당시 `examples/bootstrap/index.html`과 `root-app.js`가 백업과 일치하는지 먼저 확인하고, 종료 trap에서 원본을 복원한다. 검증한 원본 SHA-256은 `index.html` `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`, `root-app.js` `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`다.
- 입력 fixture·scene source, 빌드·설치·launch·runtime log, screenshot, PNG 및 앱 artifact SHA-256은 [SHA256SUMS.txt](./SHA256SUMS.txt)에 고정한다.
- 임의 area/track, implicit sizing, spanning, 일반 CSS admission, WPT 적합성, Android 실기기/hardware renderer, iOS 실기기·compositor 이후 색 관리는 범위 밖이다. 이 bounded slice는 C11.1/C11 또는 public CSS Grid 지원 완료가 아니며 공식 구현 완료 `72/213`은 그대로 둔다.
- 구현/runtime 실패 관점 검토 20개는 [implementation-runtime-review-20.md](./implementation-runtime-review-20.md), 별도 실제 구현 diff 검토 30개는 [implementation-pr-diff-review-30.md](./implementation-pr-diff-review-30.md)에 있다.
- 변경 source 및 capture fixture SHA-256은 [candidate-source.sha256](./candidate-source.sha256), 저장된 모든 실행 산출물 hash는 [SHA256SUMS.txt](./SHA256SUMS.txt)에 둔다.

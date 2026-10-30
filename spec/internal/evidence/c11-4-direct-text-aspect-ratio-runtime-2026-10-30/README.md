# C11.4 direct-text preferred aspect-ratio runtime evidence

## 판정 범위

고정된 Ahem direct-text 입력 4개를 기존 C11.4 opt-in fixture 경로로 실행했다. 이 결과는 일반 CSS 지원, WPT 적합성 또는 C11.4/C11 완료를 뜻하지 않는다. Chrome 156 관측과 독립 기대값은 [구현 전 비교](../c11-4-direct-text-aspect-ratio-precomparison-2026-10-30/README.md), 제한 입력은 [집중 계획](../../../../plan/c11-4-direct-text-aspect-ratio.md), 내부 계약은 [0059](../../0059-c11-grid-layout.md)에 있다. 계약 숫자 버전은 `0.1.0`으로 유지한다.

## 장면과 기대 geometry

모든 root는 `100px` 너비, `max-content max-content` 열, 0px gap, Ahem 10px/line-height 10px를 사용한다. ratio scene의 `AAAA` HTML block item에만 `aspect-ratio:2 / 1`을 지정한다. 대조군은 같은 입력에서 ratio만 제거한다.

| Scene | 열 track | 행 track | root | item frame `(x,y,w,h)` |
| --- | --- | --- | --- | --- |
| `grid-text-aspect-ratio-row` | `[10, 40]px` | `20px` | `100×20` | A `(0,0,10,20)`, AAAA `(10,0,40,20)` |
| `grid-text-aspect-ratio-row-control` | `[10, 40]px` | `10px` | `100×10` | A `(0,0,10,10)`, AAAA `(10,0,40,10)` |
| `grid-text-aspect-ratio-column` | `[80, 10]px` | fixed `40px` | `100×40` | AAAA `(0,0,80,40)`, A `(80,0,10,40)` |
| `grid-text-aspect-ratio-column-control` | `[40, 10]px` | fixed `40px` | `100×40` | AAAA `(0,0,40,40)`, A `(40,0,10,40)` |

Column ratio scene의 Grid item frame은 `80×40px`이지만 Ahem text line은 `40×10px`로 유지된다. Chrome 156 관측과 fixture local-coordinate 계산은 각각 [관측 JSON](../c11-4-direct-text-aspect-ratio-precomparison-2026-10-30/chrome-observation.json)과 precomparison README에 기록했다.

## 플랫폼 실행

최종 source-hash가 기록된 Android run은 [`c114-text-ratio-mvud6qie-49123a5e`](./android-avd/c114-text-ratio-mvud6qie-49123a5e/result.json), iOS run은 [`c114-text-ratio-mvud70h8-993b852b`](./ios-simulator/c114-text-ratio-mvud70h8-993b852b/result.json)다. 두 결과 모두 기준 checkout `0906ed5f47b4d3964c14ee0f96eca1b24a4bd945`와 현재 작업 파일 SHA-256을 보존한다.

| 실행 환경 | 실행 결과 | 설치 산출물 동일성 | 한계 |
| --- | --- | --- | --- |
| Android API 37 ARM64 QEMU AVD, `emulator-5554` | 4/4 scene, 각각 3/3 PixelCopy 표본 exact (12/12) | local APK = installed APK SHA-256 `233e9bb6a79e617a2c8ca7782d20ead2521c850213bc4a5311b5045a2329a2a1` | 에뮬레이터 결과이며 실기기 GPU 증거가 아님 |
| iPhone 17 Pro (`iPhone18,1`), iOS 26.2 Simulator | 4/4 scene, 각각 3/3 Metal drawable 표본 exact (12/12) | local executable = installed executable SHA-256 `a18c1a6769e3430d70f6505cb74f534032b7d069d596a3f773eaf424c4ec0245` | OS compositor 제출 전 drawable readback |

표면 표본은 red/blue item 배경과 사용하지 않은 root의 흰 배경만 확인한다. 글리프 raster, glyph paint, compositor 이후 픽셀은 검사하지 않았다. runner는 Android 앱을 `am force-stop`하고 iOS 앱을 `simctl terminate`한다. 원본 route/trace/pixel log, 설치 log, screenshot, `simulator.json`, source hash receipt는 각 run 폴더에 보존했다. Android row-control runtime log 한 줄의 끝 공백만 Git diff 검사에 맞게 제거했으며 진단 필드 내용은 바꾸지 않았다.

화면: [Android row ratio](./android-avd/c114-text-ratio-mvud6qie-49123a5e/grid-text-aspect-ratio-row.png) · [iOS column ratio](./ios-simulator/c114-text-ratio-mvud70h8-993b852b/grid-text-aspect-ratio-column.png).

## 빌드와 자동 검사

- Runtime direct-text suite: `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test --lib generic_direct_text -- --nocapture` — 26 passed.
- FFI scene registry: `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test --lib exposes_four_paired_direct_text_aspect_ratio_scenes -- --nocapture` — 1 passed.
- Runtime and FFI feature-enabled all-target Clippy with `-D warnings` — both passed.
- `cargo fmt --all -- --check`, capture runner `node --check`, and `git diff --check` — passed.
- Fixture module boundary plus iOS build-flag checks: `node --test tools/css-reference/fixture-module-boundary.test.mjs tools/css-reference/c11-ios-build-flags.test.mjs` — 36/36 passed.
- Android app build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 ./tools/build-android-app.sh` — `BUILD SUCCESSFUL`.
- iOS Simulator app build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 ./tools/build-ios-sim.sh` — `BUILD SUCCEEDED` on the final Swift validator source.
- Capture: `node tools/css-reference/capture-c11-4-direct-text-aspect-ratio-simulators.mjs android-avd emulator-5554`, then `node tools/css-reference/capture-c11-4-direct-text-aspect-ratio-simulators.mjs ios-simulator ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D` — both final runs passed 4/4.

An earlier strict iOS verifier run rejected the correct `collapsed` gutter values because the newly added checker compared its expected boolean expression incorrectly. Runtime frames and all three drawable samples were already exact in that failed run. The checker was corrected to compare against an explicit expected-collapse value; the final iOS build and four-scene rerun above passed. The failure log is preserved at [`runtime-grid-text-aspect-ratio-row.log`](./ios-simulator/c114-text-ratio-mvud4zre-cb9c14a9/runtime-grid-text-aspect-ratio-row.log).

## Fixture archive boundary

The scanner records raw byte markers in the inspected macOS ARM64 `spinon-ffi` release static archive. The default archive is [`default.json`](./artifact-boundary/default.json): all four scene strings and the fixture ABI name are absent (0/4 scenes). The `c04-runtime-gpu-fixture-test` positive control is [`fixture-test.json`](./artifact-boundary/fixture-test.json): all four scene strings and the fixture ABI name are present (4/4 scenes). SHA-256 values and archive sizes are included in those files. This static-archive check does not establish fixture exclusion from the final Android APK or iOS app bundle.

## 범위 밖

임의 aspect ratio, `auto <ratio>`, replaced elements, image/anonymous/rich text, 여러 줄 wrapping, spans, flex/percentage/calc, vertical writing, baseline, 일반 author CSS admission, 전체 WPT/전체 C11.4, 실기기·hardware GPU, glyph pixels와 iOS compositor-after pixels는 확인하지 않았다. C11.4와 C11은 계속 미완료이며 공식 집계 `72/213`은 바뀌지 않는다.

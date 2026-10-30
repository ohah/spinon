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

## 2026-10-30 finite-positive ratio 확장 후보

**판정:** 기존 PR #834의 4-scene baseline 위에 더한 fixture-test 전용 구현 후보. C11.4 및 C11은 미완료이며 공식 집계 `72/213`을 유지한다.

이 후속 candidate는 기존 `2 / 1` direct-text profile에 computed serialization과 일치하는 finite-positive typed ratio를 추가한다. 표준 근거와 Chrome 기준값은 [구현 전 비교](../c11-4-direct-text-aspect-ratio-range-precomparison-2026-10-30/README.md), 입력·거부 범위는 [계획](../../../../plan/c11-4-direct-text-aspect-ratio-finite.md)에 기록했다.

| 실행 | 기기·환경 | 장면 | 표면 표본 | 설치 artifact |
| --- | --- | ---: | ---: | --- |
| [Android API 37 ARM64 AVD](./android-avd/c114-text-ratio-mvuhobpm-b96aae77/result.json) | `emulator-5554`, `sdk_gphone16k_arm64`, API 37, arm64-v8a, 1080×2400 @ 420dpi | 16/16 | PixelCopy 49/49 exact | APK SHA-256 `ce54324f03547465a10464d673a9338ba8ba119059afeb7a695979cb42006b11` |
| [iPhone 17 Pro Simulator](./ios-simulator/c114-text-ratio-mvuhr3w3-23d87e87/result.json) | iOS 26.2, `iPhone18,1`, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D` | 16/16 | Metal drawable 49/49 exact | executable SHA-256 `9f5fa2162e713c4b9040a4947e778fa43d5495c99db388e8adf52a51b2466b66` |

두 설치 hash는 각 실행의 local artifact hash와 일치한다. 실행 receipt는 source HEAD `e56dd00264e9ed2ad05539a70f9e143e6d9ac1bf`, 당시 dirty source 목록과 파일별 SHA-256을 보존한다. Android는 QEMU/software renderer다. iOS 표본은 Metal drawable에서 읽었으며 OS compositor 제출 뒤의 화면을 뜻하지 않는다.

16개 장면은 기존 2/1 regression 4개, `1/2`, `4/3`, `3/2`, `5/2` 각각의 row·definite-row column scene 8개, ratio 없는 control 4개다. Chrome 156 geometry의 used value는 최대 1/64 CSS px 차이를 허용한다. `row-3-2`에서 독립 산술값 `26.666666…px`와 Chrome 관측 `26.65625px`는 이 허용치 안이다.

`column-1-2`의 별도 표면 검사는 root 바깥 overflow도 포함한다. 두 플랫폼에서 4/4 sample이 exact이며, CSS y=70은 60px root 아래에 있지만 40×80px ratio item의 red fill이 관측된다. 해당 화면은 [Android](./android-avd/c114-text-ratio-mvuhobpm-b96aae77/grid-text-aspect-ratio-range-column-1-2.png)와 [iOS](./ios-simulator/c114-text-ratio-mvuhr3w3-23d87e87/grid-text-aspect-ratio-range-column-1-2.png)에서 볼 수 있다. 나머지 표본은 item fill과 root 배경을 검사하며 glyph raster를 검사하지 않는다.

### 확장 후보 검증

- `cargo fmt --all -- --check`
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test`: 320 passed, 3 ignored.
- `cargo test --locked -p spinon-style --features c11-4-grid-track-sizing-fixture-test`: 253 passed.
- `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test`: 165 passed, 2 ignored.
- Focused direct-text aspect-ratio runtime tests: 7 passed; broader generic direct-text suite including legacy fallback: 20 passed. Style serialization carried test: 1 passed. FFI route test: 1 passed.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 23 passed.
- Feature Clippy with `-D warnings`: `taffy` library, `spinon-style`, `spinon-runtime`, and `spinon-ffi` passed. Android and iOS simulator builds succeeded.
- 기본 최적화 FFI archive SHA-256 `27a5e0d495acb2117c5d55a3847907c3703de1a9782e6fd51a3c4b24b2310c24`에는 ratio scene ID와 route symbol이 없다. Fixture positive-control archive SHA-256 `31ae2eb79c94a0c6e6381b44d634028b74020db44523db949b9adf90c44736e8`에는 scene ID 16개와 gated route symbol이 있다. 이는 검사한 static archive 범위의 결과이며 APK/IPA 전체에서 제외됨을 뜻하지 않는다.

이번 16-scene 비교 원장은 [default archive](./artifact-boundary/finite-ratio-default.json)와 [fixture positive control](./artifact-boundary/finite-ratio-fixture-test.json)에 있다. 기존 PR #834의 4-scene artifact JSON은 같은 디렉터리의 `default.json`과 `fixture-test.json`에 보존되어 있다.

저장소 workspace에서 Taffy는 제외되어 있고 자체 test target은 dev-dependencies를 요구하므로 `cargo test --locked -p taffy`를 직접 실행할 수 없다. Taffy library Clippy와 변경된 layout 경로를 호출하는 runtime/FFI tests는 통과했다.

### 초기 검증 실패와 수정

최종 실행 전의 실패 evidence도 보존했다. 네 종류의 검증기 기대 오류를 발견해 고쳤다.

1. Android app verifier가 Chrome의 1/64px geometry quantization을 허용하지 않아 3/2 row 비교를 실패 처리했다. 숫자 frame 비교를 1/64px tolerance로 바꿨다.
2. Capture runner가 app이 수치 tolerance로 허용한 값을 예전 요약 문자열과 exact 비교했다. Runner도 수치값 기준으로 통일했다.
3. `column-1-2` 기대값이 40×60px로 잘못되어 있었다. 독립 계산과 Chrome 결과에 맞춰 40×80px item 및 20px overflow로 고쳤다.
4. Android PixelCopy 기대색 목록에 root 밖 overflow 표본의 red가 빠져 있었다. 기대 표본을 보완하고 양 플랫폼을 다시 실행했다.

이전 실패 디렉터리를 삭제하지 않고 같은 evidence root에 남겨 두었다. 최종 판정은 위 표의 마지막 실행 receipt만 사용한다. 일반 CSS 입력 허용, 전체 WPT, 이 fixture 행렬 밖의 임의 ratio, `auto <ratio>`, 이 collector의 degenerate numeric ratio, 여러 ratio item, replaced/rich/anonymous text, wrapping, min/max·box-edge transfer, span, vertical writing, 실기기/hardware GPU, glyph 픽셀 일치, iOS compositor 이후 픽셀은 확장 후보의 보장 범위가 아니다. PR check 상태는 PR 생성 후 확인한다. 상세한 구현/runtime 검토는 [20개 실패 관점](./finite-ratio-implementation-runtime-review-20.md), 현재 branch diff 검토는 [30개 관점](./finite-ratio-pr-diff-review-30.md)에 둔다.

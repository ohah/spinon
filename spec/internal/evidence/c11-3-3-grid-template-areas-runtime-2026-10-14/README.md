# C11.3.3 template areas runtime 검증 · 2026-10-14

## iOS Simulator

- 대상: iPhone 17 Pro · iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`
- 최신 source 실행 ID: `c1133-ios-simulator-20261014T023918758Z-45918e12`
- 실행: `basic-rectangles` 성공 1 scene + `area-track-count-mismatch` 기대 거부 1 scene · success HostDocument 4 frame
- 성공 geometry: root 160×120 CSS px, head 100×40 at (0,0), main 120×40 at (40,40), foot 160×40 at (0,80)
- mismatch는 `unsupported_computed_value`, `grid-template-areas` property로 frame 없이 거부
- Chrome 154 DPR 1 고정 기준: Metal drawable 표본 2/2 RGBA exact, 최대 채널 오차 0
- 잘못된 표본 좌표: Metal readback 전 거부
- iOS 표면 픽셀 비교는 OS compositor 제출 전 drawable readback이다. `ios-screen.png`는 UI 확인 자료다.

[성공 실행 README](./ios-simulator-c1133-ios-simulator-20261014T023918758Z-45918e12/README.md) · [결과 JSON](./ios-simulator-c1133-ios-simulator-20261014T023918758Z-45918e12/result.json) · [Metal 비교](./ios-simulator-c1133-ios-simulator-20261014T023918758Z-45918e12/metal-surface-comparison.json) · [Simulator 화면](./ios-simulator-c1133-ios-simulator-20261014T023918758Z-45918e12/ios-screen.png)

## Android API 37 AVD 전체 runtime matrix

- 대상: `spinon_api37_0_applied_txn` · Android 17/API 37 ARM64 · serial `emulator-5554`
- 렌더러: ANGLE/Vulkan/SwiftShader software renderer
- 최신 source 실행 ID: `c1133-matrix-20261014T023829118Z-d027504f`
- 26 scene·65 node: 성공 19, cascade fallback 5, 기대 거부 2
- Chrome 154 DPR 1 고정 기준: visible node frame 61개 최대 geometry 오차 0 CSS px
- `basic-rectangles` Chrome 표면 PixelCopy 표본 2/2 RGBA exact, 최대 채널 오차 0
- 잘못된 표본 좌표: PixelCopy readback 전 거부
- 알 수 없는 scene 인자: runtime 평가·GPU draw 전에 거부
- 사용자가 이번 C11.3.3 작업에 AVD 실행을 허용했다. 이 결과는 SM-S731N 또는 hardware GPU 검증이 아니다.

[최신 AVD README](./android-avd-c1133-matrix-20261014T023829118Z-d027504f/README.md) · [결과 JSON](./android-avd-c1133-matrix-20261014T023829118Z-d027504f/result.json) · [PixelCopy PNG](./android-avd-c1133-matrix-20261014T023829118Z-d027504f/pixelcopy.png) · [scene allowlist negative log](./android-avd-c1133-matrix-20261014T023829118Z-d027504f/negative-scene.log)

## Android 실패 재현과 원인

ADB 연결·serial 자체는 실패하지 않았다. 첫 AVD build는 격리 clone 안에 pinned V8 source checkout이 없어 ADB 설치 단계 전에 사전 검사에서 중단됐다. 고정 V8/depot_tools checkout을 `SPINON_V8_DIR`로 지정해 APK를 빌드했다. 이 build 사전 조건 실패는 [log](./android-avd-c1133-android-avd-20261014T011159343Z-5dc5d8c3/build.log)와 [failure](./android-avd-c1133-android-avd-20261014T011159343Z-5dc5d8c3/failure.txt)에 보존했다.

첫 설치 후 앱 process는 실행됐지만 `MainActivity`에 C11.3.3 intent flag, run ID, scene validation 및 runtime route가 없어 `basic-rectangles`를 C11.1 Grid 검사로 보내고 `SPINON_C111_GRID_ARGUMENT_ERROR`를 냈다. C0410 runtime demo/JNI는 이미 C11.3.3 host를 제공했다. MainActivity route를 추가한 뒤 단일 smoke가 통과했고, scope guard 수정 후 최신 source 26-scene AVD matrix도 통과했다. [실패 재현 log](./android-avd-c1133-android-avd-20261014T011237779Z-24019ba5/failure.txt) · [route 수정 당시 build log](./android-avd-c1133-android-avd-20261014T011622837Z-13204dc1/build.log) · [최신 matrix](./android-avd-c1133-matrix-20261014T023829118Z-d027504f/README.md).

별도 R05 통합 회귀의 fake ADB failure는 test shim의 `#!/usr/bin/env node` 시작 경합이었다. `/usr/bin/env`가 Node로 exec되는 동안 host `ps` command identity가 달라져 안전한 종료가 `getevent: collector process identity changed (command); refusing SIGINT`로 거부됐다. shim을 `process.execPath`로 직접 실행하게 바꾸고 shutdown error에 collector별 원인을 넣었다. 수정 후 대상 테스트 8회 병렬 실행 8/8, 통합 `bun run test`가 통과했다. 수정 전후 근거는 [실패 로그](./full-regression-before-fix.log)와 [R05 수정 후 전체 회귀](./full-regression.log)에 있다. 이번 Android AVD scene 실행은 그 test shim 실패와 별개의 앱 route 결함을 드러냈다.

## 실행 범위와 회귀

최신 Android 실행은 Chrome 사전 비교 26 scene·65 node 전체를 같은 API 37.0 ARM64 AVD에서 재생했다. 19 success·5 cascade fallback·2 expected rejection이며 visible frame 61개 오차 0 CSS px, PixelCopy 2/2 exact다. 최신 iOS Simulator는 success/rejection 두 scene과 success 4 frame, Metal 2/2 exact를 확인했다. AVD는 ANGLE/Vulkan/SwiftShader software renderer이고 iOS readback은 compositor 제출 전이다. 실기기, hardware GPU 및 compositor 이후 pixel은 검증하지 않았다. Android AVD가 C11.3.3 full fixture matrix를 통과했지만 Spinon WPT 적합성과 전체 CSS Grid 지원은 미완료다.

최신 통합 `bun run test`는 [최종 회귀 로그](./full-regression-final.log)에 보존했다: JavaScript 2/2, CSS reference 412/412, R05 benchmark 29/29, Rust workspace 694 passed·4 ignored·0 failed. 별도 C11.3.3 fixture FFI test는 4/4 통과했다. workspace Clippy, `cargo fmt --all -- --check`, `git diff --check`, runtime matrix generator check도 통과했다. 첫 전체 회귀에서는 공용 초기화 함수 이름을 일반화한 변경 뒤 C12.4 source-level test가 이전 method name을 고정해 실패했다. 검사를 `scheduleRuntimeInitializationForReadySurface`와 C11.3.3 flag까지 확인하도록 맞춘 후 CSS suite와 전체 회귀가 통과했다. [초기 실패 로그](./full-regression-after-scope-guard.log) · [최종 Clippy 로그](./workspace-clippy-final.log) · [FFI artifact 비교](./ffi-artifact-boundary.json). 현재 source로 다시 빌드한 기본 static library에는 C11.3.3 symbol/payload가 없고 opt-in artifact에 의도한 symbol·scene ID·payload가 포함된다. 이 자료는 release app bundle 크기를 검증하지 않는다.

## Tailnet 문서 미리보기

Tailnet 미리보기는 2026-10-14 02:52 UTC에 동기화했다. 매니페스트 1,569개 파일의 SHA-256이 모두 일치했고 변경된 Markdown 57개를 HTML로 갱신했다. agent-browser에서 roadmap, 상태 대장, runtime 결과, 30개 관점 PR 검토를 열었다. 390×844 viewport 네 페이지에서 가로 overflow가 없었고, 네 페이지의 same-origin 링크 검사 694건은 모두 HTTP 200이었다. 화면 캡처는 [모바일 preview](./tailnet-preview-mobile-390x844.png)에 보존했다. GitHub Pages는 배포하지 않았다.

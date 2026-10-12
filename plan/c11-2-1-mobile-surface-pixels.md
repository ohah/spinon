# C11.2.1 · 모바일 표면 픽셀 행렬 계획

**상위 계획:** [C11 Grid](c11-css-grid.md) · [명시 line/span 배치](c11-2-1-explicit-grid-placement.md) · [10-scene 모바일 runtime 행렬](c11-2-1-mobile-runtime-matrix.md)
**상태:** 실행 완료 · Android API 37 AVD, Android 16/API 36 SM-S731N 실기기, iPhone 17 Pro/iOS 26.2 Simulator에서 성공 scene별 20 sample exact 및 기대 거부 4/4 no-draw · Spinon WPT runner·적합성은 X16 별도 추적
**계획 실패 관점 검토:** [20개 검토](../spec/internal/evidence/c11-2-1-mobile-surface-pixels-plan-review-2026-10-12.md)
**구현 실패 관점 검토:** [20개 검토](../spec/internal/evidence/c11-2-1-grid-placement-pixel-matrix-implementation-review-2026-10-12.md)

## 목적

기존 `numeric-lines` 한 scene의 Android PixelCopy·iOS Metal 표본만 확인한 검증을 C11.2.1 고정 fixture의 성공 scene 전부로 넓힌다. Chrome 154 DPR 1 screenshot에서 scene별 표본 좌표와 RGBA를 고정하고, 같은 V8 fixture를 모바일 앱의 WGPU 표면에 그린 뒤 표면 자체를 읽어 비교한다.

고정 행렬은 10 scene이다. `numeric-lines`, `positive-spans`, `reversed-lines`, `shorthand-cascade`, `overlap-source-order`, `nested-placement-modes` 6개는 제한 profile 성공 후보이므로 painted surface를 Chrome과 비교한다. `deferred-implicit-track`, `deferred-named-line`, `deferred-mixed-auto-explicit`, `deferred-equal-lines` 4개는 Spinon의 의도된 거부 입력이다. 이 네 장면은 Chrome의 정상 CSS 출력과 pixel parity를 주장하지 않고, 별도 새 process에서 정확한 실패 진단·부분 frame 부재·WGPU 제출 부재를 runtime 행렬 증거로 확인한다.

## 고정 입력과 표본 계약

- 입력은 `grid-placement-runtime-matrix.json`, `grid-placement-inventory.json`, `grid-placement.html`, pinned Chrome 154 DPR 1 precomparison 및 scene screenshot이다. 모든 입력·생성기·Chrome reference·screenshot SHA-256을 검증하고 결과에 저장한다.
- 성공 후보 scene과 Chrome reference scene ID, node preorder, geometry, parent graph가 일치하지 않으면 수집을 시작하지 않는다. 각 표본은 해당 scene의 painted node 내부에서 뽑으며 겹침이 있는 경우 실제 위쪽 paint color를 기준으로 좌표를 찾는다.
- 각 성공 scene은 각 보이는 colored item을 하나 이상 표본화하고, 가능하면 grid surface와 track gap도 추가한다. 색상 경계에서 떨어진 3×3 동일 RGBA 영역만 채택한다. 좌표는 node의 Chrome rect 중심에 가장 가까운 유효한 내부 pixel로 고르고, 거리가 같으면 y, x 오름차순을 사용한다. 숨겨졌거나 가려진 node의 중앙 좌표를 샘플로 간주하지 않는다.
- 모바일 runtime 표면은 320×240 CSS px다. Chrome 캡처는 360×800이므로 표본의 CSS 좌표가 x `0..319`, y `0..239`에 있고 Chrome screenshot과 실제 runtime viewport 양쪽 안에 드는지 확인한다. 기기 pixel 좌표는 실제 drawable 크기와 CSS viewport 비율로 환산한다.
- 비교는 모든 scene/node sample의 RGBA 네 채널을 각각 확인하고 허용 오차는 0이다. scene 누락·중복, sample 누락·중복, stale run/PID/surface generation/revision, PNG 손상, 잘못된 hash·좌표·채널은 전체 행렬 실패다.

## 플랫폼 실행

- Android API 37 AVD에서 앱 `SurfaceView`의 PixelCopy PNG와 원시 sample 로그를 교차 검증한다. AVD renderer가 ANGLE/SwiftShader이면 소프트웨어 렌더링 결과라고 표기하고 실기기 GPU 성능·동등성으로 일반화하지 않는다.
- Android 실기기가 연결되면 동일한 성공 scene 행렬을 별도 run으로 실행해 보완 증거를 저장한다. 연결되지 않은 상태에서 실기기 완료로 표시하지 않는다.
- iPhone 17 Pro / iOS 26.2 Simulator는 WGPU가 그린 Metal drawable texture를 OS compositor 제출 전에 읽는다. simulator screenshot은 화면 확인용으로만 저장하고 표면 readback을 대체하지 않는다.
- 한 플랫폼 실행기에서 앱을 한 번 빌드하고 같은 앱 binary hash로 10개 scene을 모두 실행한다. scene마다 별도 앱 process·V8 host·HostDocument·render revision을 사용한다. 여섯 성공 scene은 surface를 읽고, 네 거부 scene은 기대 진단과 부분 frame·WGPU 제출 부재를 같은 실행에서 확인한다. 결과는 같은 matrix run 아래 앱 binary hash, fixture/reference/sample hash, node frame, draw marker, surface generation, revision과 함께 보존한다.

## 실패 관점과 완료 조건

1. 생성기 입력 hash, scene/node order, screenshot PNG, viewport, hidden/overlapped sample을 변조한 단위 fixture가 모두 거부되어야 한다.
2. Android PixelCopy와 iOS Metal 비교기는 scene/sample 누락·중복, 다른 run, 이전 PID/generation/revision, 잘못된 JSON·PNG, RGBA 한 채널 불일치를 각각 실패시켜야 한다.
3. 앱 intent의 성공 후보 allowlist 밖 scene, sample scene 불일치, 너무 크거나 빈 payload, 잘못된 run ID는 surface capture 전에 거부되어야 한다.
4. 여섯 성공 후보 각각에서 고정 Chrome reference와 모든 선택 sample이 정확히 일치해야 한다. 한 scene이라도 빠지거나 실패하면 성공 summary를 쓰지 않는다.
5. 네 거부 scene은 같은 binary/run의 기대 code/node/property, 부분 frame 없음, WGPU non-empty present 없음과 연결한다. 거부 입력을 성공 픽셀로 세거나 Chrome 출력과 동일하다고 표시하지 않는다.
6. Android·iOS 캡처 중 앱 종료, surface 재생성, scene 변경, 캡처 timeout이 생기면 해당 run을 무효로 남기고 재시도는 새 run ID로 기록한다.
7. 전체 fixture/reference generator와 negative controls, JS/CSS test, Android build, iOS Simulator build, Rust workspace, Clippy, rustfmt, diff 검사를 실행하고 로그를 보존한다.

다 통과해도 이 계획은 고정 6개 성공 scene의 표면 색상 표본과 4개 지원 밖 입력의 no-draw 계약만 확인한다. Android 하드웨어 GPU, iOS 실기기, compositor 최종 화면·scanout, WPT 적합성, 성능 또는 전체 Grid/CSS 지원을 증명하지 않는다. 결과가 확인되면 [구현 상태 대장](../spec/STATUS.md), [C11.2.1 계획](c11-2-1-explicit-grid-placement.md), [로드맵 HTML](../spec/public/roadmap.html)과 Tailnet 공개본을 같은 변경에서 갱신한다.

## 실행 결과

- Android API 37 AVD: [실행 ID `c1121-android-20261012T093216817Z-43dfc994`](../spec/internal/evidence/c11-2-1-grid-placement-android-runtime-matrix-2026-10-12/c1121-android-20261012T093216817Z-43dfc994/README.md) · 성공 6/6, 기대 거부 4/4, PixelCopy 20/20 exact, 최대 channel 오차 0. APK SHA-256 `0a42647465a10c7917ce3b635b845a9813e475da37c42bb67dbe000f741457a6`. renderer는 ANGLE/Vulkan/SwiftShader다.
- iPhone 17 Pro/iOS 26.2 Simulator: [실행 ID `c1121-iossim-20261012T093253773Z-8df46c33`](../spec/internal/evidence/c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-12/c1121-iossim-20261012T093253773Z-8df46c33/README.md) · 성공 6/6, 기대 거부 4/4, Metal drawable 20/20 exact, 최대 channel 오차 0. executable SHA-256 `135cdec7fcee1d83dfc51ac443ea6ce65b162ff406649a9e6c25773c503172e8`. readback은 compositor 제출 전이다.
- [통합 실행·시각 증거](../spec/internal/evidence/c11-2-1-mobile-surface-pixels-2026-10-12/README.md) · [전체 test log](../spec/internal/evidence/c11-2-1-mobile-surface-pixels-2026-10-12/full-test.log) · [Clippy](../spec/internal/evidence/c11-2-1-mobile-surface-pixels-2026-10-12/clippy.log).
- Android 16/API 36 SM-S731N 실기기에서 성공 6 scene·20/20 PixelCopy sample exact·최대 channel 오차 0을 확인했다. [실행 ID `c1121-android-20261012T161006179Z-4ffd28bc`](../spec/internal/evidence/c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T161006179Z-4ffd28bc/README.md). 이 값은 고정 표면 색상 표본 결과이며 hardware performance 측정이 아니다. Spinon WPT runner·적합성은 X16에서 별도 관리한다.

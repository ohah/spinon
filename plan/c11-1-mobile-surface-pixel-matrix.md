# C11.1 · 모바일 10장면 표면 픽셀 비교 계획

**상위 계획:** [C11 CSS Grid](c11-css-grid.md) · [모바일 runtime 행렬](c11-1-mobile-runtime-matrix.md)
**상태:** 구현 및 실행 완료 · Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 10 scene·36 sample exact · Android 실기기와 iOS 실기기는 별도 미검증
**계획 검토:** [서로 다른 실패 관점 20개](../spec/internal/evidence/c11-1-mobile-surface-pixel-matrix-plan-review-2026-10-12.md)

## 목적과 범위

고정 Chromium 154의 10개 C11 Grid scene에서 모든 색상 item과 scene별 track 배경을 표본화하고, Android API 37 AVD의 `SurfaceView` 및 iPhone 17 Pro / iOS 26.2 Simulator의 WGPU Metal drawable 결과와 비교한다. 표본은 고정된 Chromium PNG에서 생성한다. 목표는 10개 장면·36개 시각 표본이며, 각 scene은 별도 앱 process·V8 host·HostDocument·surface generation에서 실행한다.

Android 결과는 PixelCopy의 SurfaceView 출력이며 AVD에서는 ANGLE/SwiftShader 결과다. iOS 결과는 Metal drawable texture의 compositor 제출 전 pixel이다. 최종 디스플레이 색상, 물리 GPU 동등성, iOS 실기기, WPT 적합성 또는 전체 CSS 지원을 뜻하지 않는다. Android 물리 기기가 연결되면 동일 collector를 별도 기기 run으로 추가할 수 있지만 이 계획의 pass 조건은 simulator/device 종류를 분명히 구분한다.

## 입력과 표본 계약

- inventory, HTML, Chrome 154 DPR 1 precomparison JSON과 10개 PNG의 원본 hash를 확인한다. reference observation의 scene/node ID·preorder·geometry가 inventory와 완전히 일치하지 않으면 생성과 실행을 중단한다.
- `blue`, `red`, `green`, `yellow`, `purple`, `cyan`, `orange`, `pink` class가 칠하는 모든 visible item을 Chromium `documentRect` 중심에서 표본화한다. `track-surface` 또는 고정 inline track 배경이 있는 노드는 각 scene에서 하나의 배경 sample을 제공한다. 부모 box 중심이 자식·track 경계에 걸릴 수 있으므로 투명 구조 노드는 무조건 표본화하지 않는다. 표본 주변 3×3 RGBA가 같아 anti-alias 경계에서 떨어져 있음을 검사한다.
- 표본은 `0 ≤ x < 320`, `0 ≤ y < 240`의 공통 CSS viewport 안에 있어야 한다. 폭·높이 0, PNG reference 밖 좌표, 균일한 3×3 영역이 없는 painted item/track, 1–64 범위를 벗어나는 scene sample 수는 오류다. 생성 fixture는 정확히 10개 scene·36개 sample 및 Chrome PNG/reference hash를 보존한다.
- Android는 각 좌표를 실제 surface/density 비율로 device pixel에 변환하고 PixelCopy PNG와 원시 RGBA를 보존한다. iOS는 실제 drawable 크기 ÷ 320×240 CSS viewport로 x/y 배율을 각각 계산하고 Metal readback에서 표본을 얻는다. Simulator screenshot은 보조 증거만이다.
- 비교는 scene 및 node 표본마다 RGBA 4채널을 개별 검사한다. 평균 색상 오차나 일부 scene 통과로 전체 결과를 통과 처리하지 않는다. 허용 channel error는 0이다.

## 실행 및 실패 경계

1. 생성기와 collector는 정확히 10개 scene 및 각 scene의 예상 NodeId 순서·개수를 확인한다. 임의 HTML/JS, 임의 scene ID, sample 좌표를 제품 경로로 전달하지 않는다.
2. Android/iOS intent는 scene allowlist, 고유 run ID, 1–64 sample 수, 고유 sample ID, viewport 내부 좌표를 검증한다. FFI 및 플랫폼 배열의 null·overflow·count 초과는 명시적 오류다.
3. 매 scene은 새 앱 process와 새 runtime host로 시작한다. collector는 PID, scene ID, app/source/reference/fixture hash, surface generation, document/render-tree revision, drawable 크기와 draw 성공을 같은 run에 묶는다.
4. PixelCopy는 같은 surface·renderer generation이 capture 전후 유지된 경우만 채택한다. Metal readback은 요청 표본과 실제 제출한 drawable/revision이 같은 경우만 채택한다. 화면 캡처나 이전 run의 RGBA를 표면 표본으로 대체하지 않는다.
5. 누락·중복·unknown sample, stale PID/run/generation, scene 바뀜, geometry/readback error, PNG 손상, 개별 RGBA 차이를 각각 실패시킨다. 한 장면 실패 뒤 전체 matrix가 성공 summary를 만들지 않게 한다.
6. AVD/Simulator/backend와 물리 기기 결과를 따로 보존한다. AVD software rendering을 hardware GPU 결과로 부르지 않고, Metal drawable sample을 compositor 이후 화면 pixel로 부르지 않는다.

## 완료 기준

- [x] Chromium reference generator가 10/10 PNG·36/36 visual sample 및 hash를 재현하고 3×3 interior, screenshot hash drift, node inventory drift를 확인했다.
- [x] Android API 37 AVD에서 10/10 scene·36/36 PixelCopy 표본이 정확히 일치했다. APK hash, SwiftShader renderer, PID, generation, PixelCopy PNG와 원시 로그를 보존했다.
- [x] iPhone 17 Pro / iOS 26.2 Simulator에서 10/10 scene·36/36 실제 Metal drawable 표본이 정확히 일치했다. app hash, drawable/revision, RGBA, unified log와 scene별 보조 screenshot을 보존했다.
- [x] 비교기의 scene/sample 누락·중복, hash 변조, stale run/PID/generation, 좌표, 잘못된 채널과 혼합 실행 로그를 거부하는 Android·iOS negative controls를 통과했다.
- [x] Rust workspace, Android/iOS Simulator build, JS/CSS reference suite와 `git diff --check`를 통과했고 공식 상태와 Tailnet preview를 동기화했다.

## 실행 결과

- [Android API 37 AVD 실행](../spec/internal/evidence/c11-1-mobile-surface-pixel-matrix-2026-10-12/android-avd-c11-pixels-20261012T014701830Z-e435929f/README.md): 10/10 scene·36/36 sample, channel 오차 0. SurfaceView PixelCopy PNG의 실제 RGBA·SHA-256도 로그와 대조했다. renderer는 ANGLE/SwiftShader이며 Android 실기기나 하드웨어 GPU 근거가 아니다.
- [iPhone 17 Pro / iOS 26.2 Simulator 실행](../spec/internal/evidence/c11-1-mobile-surface-pixel-matrix-2026-10-12/ios-simulator-c11-pixels-20261012T014858759Z-38b0e689/README.md): 10/10 scene·36/36 sample, channel 오차 0. 직접 Metal drawable readback과 장면별 Simulator screenshot을 보존했다. iOS 실기기나 compositor 이후 display pixel 결과가 아니다.
- [실행·테스트 모음](../spec/internal/evidence/c11-1-mobile-surface-pixel-matrix-2026-10-12/README.md): generator 2개, Android comparator 22개, iOS comparator 30개, JS 2개, Android touch 도구 29개, Rust workspace 596 passed·3 ignored, Clippy와 rustfmt 결과.
- Android ADB에는 실행 당시 `emulator-5554` API 37 AVD만 연결되어 있었다. Android 실기기 재측정은 완료로 표시하지 않는다.

완료해도 Android/iOS 실기기 전체 pixel matrix, Spinon WPT runner, 다른 GPU·OS, color management, 전체 Grid/CSS 적합성은 닫지 않는다.

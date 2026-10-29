# C11.3.3 Grid template areas Android runtime matrix

- 실행 ID: `c1133-matrix-20261029T203832317Z-36dd5796`
- 대상: Android API 37 AVD, SwiftShader
- source commit: `92c8502be33ada9e2a71e32bea69ff048ea3f109`
- Android Java SHA-256: `c5720351d9efdf9881a14422efed063a4a4a092375f7b3788088e165208f5032`
- APK SHA-256: `75deb03a2168b642a8b2773281d5b730b614183af226f4af01acd599fd1ea6b2`
- Runtime matrix: 37 scene / 118 node; 성공 32, cascade fallback 5, 기대 거부 0.
- Chrome DPR 1 geometry: visible node frame 118개 비교, 최대 오차 0 CSS px.
- Basic scene PixelCopy: 2/2 exact, 최대 channel 오차 0.
- Track-list cycling PixelCopy: 6 scene / 28 RGBA sample exact, 최대 channel 오차 0.
- Fixed unknown-area overflow: Chrome geometry node 3개, grid root 바깥 빨간 item을 포함해 PixelCopy 3/3 exact.
- Leading named implicit lines: 7 scene / visible node 14개, 최대 geometry 오차 0 CSS px, PixelCopy 7/7 exact.
- Negative control: 잘못된 pixel sample은 readback 전에 거부했고, unknown scene은 runtime draw 전에 거부했다.

이 AVD 실행은 physical device·hardware GPU 동작, WPT 적합성 또는 일반 CSS Grid 지원을 증명하지 않는다.

## 산출물

- `runtime-*.log`: scene별 runtime, geometry, 거부, draw 기록
- `result.json`: fixture hash, geometry 및 negative-control 결과
- `pixelcopy.png`: 기본 scene Android SurfaceView PixelCopy
- `pixelcopy-*.png`: track-list cycling PixelCopy 표본
- `unknown-area-overflow-pixelcopy.png`: fixed implicit-track overflow PixelCopy
- `pixelcopy-leading-*.png`: named implicit line PixelCopy 표본
- `android-track-list-cycle-screen.png`: 양축 cycling AVD 화면
- `android-screen.png`: 최종 AVD 화면

# C11.3.3 Grid template area 모바일 실행

- 실행 ID: `c1133-fitcap-ios-simulator-20261031T094545391Z-17a7f5d0`
- 대상: `ios-simulator`
- Chrome 기준: Google Chrome 156.0.8078.76 · DPR 1 · `fit-content-area-cap`
- runtime: 성공 scene 1개 · element frame 3개
- 성공 geometry: root 200×60 CSS px, item 40×60 CSS px at (20, 0)
- 표면 표본: 3/3 exact · 최대 채널 오차 0
- 좌표 allowlist negative control: rejected-before-readback
- 표면 비교는 Metal drawable readback이며 OS compositor 이후 픽셀은 확인하지 않았다.
- 화면 PNG는 UI 상태 확인용이며 표면 픽셀 비교에는 사용하지 않았다.
- 이는 고정 Chrome fixture의 제한된 runtime 결과이며 Spinon WPT 적합성이나 CSS Grid 전체 지원을 뜻하지 않는다.

- Source commit: `b815f483a253072cbbca1c0cd473d519fc95a220`; source tree: `8e74b832259990dc6cc3513a0a119efea9fafe8d`; capture script SHA-256: `fd4f43faf22bebb5c78dd32fa35cf5f11c9b63f1ec6e7554b99e83d3912bb9e4`. See `result.json` → `source`.

- `build.log`: Xcode build output with trailing whitespace removed.

## 산출물

- `runtime.log`: 해당 앱 프로세스의 success/rejection/draw/frame/pixel 로그
- `ios-screen.png`: iOS Simulator 화면
- `metal-surface-comparison.json`: Metal 표본 비교
- `result.json`: fixture·oracle·앱 hash와 실행 결과

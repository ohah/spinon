# C11.3.3 named-line auto implicit tracks 모바일 실행

- 실행 ID: `c1133-ios-simulator-20261031T143731498Z-ae43a911`
- 대상: `ios-simulator`
- Chrome 기준: HeadlessChrome/156.0.0.0 · DPR 1 · `named-line-auto-columns`
- runtime: 성공 scene 1개 · element frame 2개
- 성공 geometry: root 80×20 CSS px, item 30×10 CSS px at (80, 0)
- 표면 표본: 1/1 exact · 최대 채널 오차 0
- 좌표 allowlist negative control: rejected-before-readback
- 표면 비교는 Metal drawable readback이며 OS compositor 이후 픽셀은 확인하지 않았다.
- 화면 PNG는 UI 상태 확인용이며 표면 픽셀 비교에는 사용하지 않았다.
- 이는 고정 Chrome fixture의 제한된 runtime 결과이며 Spinon WPT 적합성이나 CSS Grid 전체 지원을 뜻하지 않는다.

## 산출물

- `runtime.log`: 해당 앱 프로세스의 success/rejection/draw/frame/pixel 로그
- `ios-screen.png`: iOS Simulator 화면
- `metal-surface-comparison.json`: Metal 표본 비교
- `result.json`: fixture·oracle·앱 hash와 실행 결과

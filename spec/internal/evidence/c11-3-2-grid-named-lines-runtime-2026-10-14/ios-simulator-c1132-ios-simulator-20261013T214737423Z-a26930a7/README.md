# C11.3.2 Grid named line 모바일 실행

- 실행 ID: `c1132-ios-simulator-20261013T214737423Z-a26930a7`
- 대상: `ios-simulator`
- Chrome 기준: Chrome/154.0.8037.92 · DPR 1 · `positive-occurrence`
- runtime: 성공 장면 1개 · 기대 거부 장면 1개 · node 4개
- 성공 geometry: root 160×120 CSS px, item 80×40 CSS px at (80, 0)
- 표면 표본: 2/2 exact · 최대 채널 오차 0
- 좌표 allowlist negative control: rejected-before-readback
- 표면 비교는 Metal drawable readback이며 OS compositor 이후 픽셀은 확인하지 않았다.
- 앱 SHA-256: `6db1b7af7f2903c90b4097c8d6e09c4b250b9a5c5ef48210b6b5de0890eb77d9`.
- 화면 PNG는 UI 상태 확인용이며 표면 픽셀 비교에는 사용하지 않았다.
- 이는 고정 Chrome fixture의 제한된 runtime 결과이며 Spinon WPT 적합성이나 CSS Grid 전체 지원을 뜻하지 않는다.

## 산출물

- `runtime.log`: 해당 앱 프로세스의 success/rejection/draw/frame/pixel 로그
- `ios-screen.png`: iOS Simulator 화면
- `metal-surface-comparison.json`: Metal 표본 비교
- `result.json`: fixture·oracle·앱 hash와 실행 결과

# C11.1 iOS Simulator 10-scene runtime 비교

- 실행 ID: c11-grid-ios-matrix-20261011T205619778Z-2032fc2e
- Simulator: iPhone 17 Pro · com.apple.CoreSimulator.SimRuntime.iOS-26-2 · ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D
- 앱 실행 파일 SHA-256: `a4c230d0872edb23adfbb084208a7fd953b1f2f032c3b1a8583b0ab3d326bb93`
- 고정 Chrome 154 reference의 10개 scene·40개 node를 각각 새 앱 프로세스에서 실행했다.
- wrapper frame과 모든 data-node x/y/width/height를 CSS px 기준 ±0.5 이내로 비교했다.
- Metal surface texture 직접 pixel readback은 하지 않았다. simulator screenshot은 보조 시각 증거다.
- [상세 비교](./comparison.json) · 각 scene 로그와 screenshot 및 build/install 로그를 함께 보존했다.
- iOS 실기기, WPT 적합성, 전체 CSS 지원은 별도 관문이다.

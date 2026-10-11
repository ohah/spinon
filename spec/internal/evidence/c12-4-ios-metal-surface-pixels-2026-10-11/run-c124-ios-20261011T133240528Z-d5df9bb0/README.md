# C12.4 iOS Metal surface 표면 표본

- 결과: 8/8 통과, 채널별 허용 오차 0
- 장치: iPhone 17 Pro · com.apple.CoreSimulator.SimRuntime.iOS-26-2 · ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D
- 빌드: Xcode 26.2 / Build version 17C52
- 각 조합은 새 앱 프로세스에서 V8 fixture를 변경하고 실제 `CAMetalLayer` WGPU surface texture를 복사해 읽었다.
- 화면 이미지는 case별 `.png`, 원본 통합 로그는 `unified.log`, 비교 결과는 `comparison.json`에 있다.
- 이 결과는 Simulator의 고정 8개 compatibility 표본이며 WPT 적합성이나 전체 CSS 지원 증거가 아니다.

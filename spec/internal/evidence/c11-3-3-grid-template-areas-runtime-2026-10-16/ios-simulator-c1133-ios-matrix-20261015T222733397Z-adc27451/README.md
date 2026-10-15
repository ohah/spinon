# C11.3.3 Grid template areas iOS Simulator 전체 행렬

- 실행 ID: c1133-ios-matrix-20261015T222733397Z-adc27451
- 대상: iPhone 17 Pro / com.apple.CoreSimulator.SimRuntime.iOS-26-2 Simulator
- 고정 Chrome DPR 1 기준: 26 scene / 65 node
- 결과: 성공 19, cascade fallback 5, 기대 거부 2
- node frame 비교: 61개, 최대 geometry 오차 0 CSS px
- 초기 runtime 평가 `status=-12` 재실행: 0회 (최초 로그와 재실행 로그 모두 보존)
- basic-rectangles Metal drawable 표본: 2/2 RGBA exact; 최대 채널 오차 0
- negative controls: sample 좌표 rejected-before-readback, unknown scene rejected-before-runtime-draw
- Metal drawable은 OS compositor 제출 전이다. 실기기·hardware GPU·WPT 적합성·전체 CSS Grid 지원은 이 실행에 포함하지 않는다.

## 산출물

- `build.log.gz`: iOS Simulator build transcript, gzip 압축 원본
- runtime-*.log: scene별 평가, frame, 거부 결과
- ios-screen.png: basic-rectangles UI 화면 캡처
- metal-surface-comparison.json: Chrome 고정 sample과 Metal drawable 비교
- result.json: fixture hash, geometry, pixel 및 negative-control 결과

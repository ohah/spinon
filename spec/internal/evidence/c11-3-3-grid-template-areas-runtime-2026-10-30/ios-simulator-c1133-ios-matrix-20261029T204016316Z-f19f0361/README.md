# C11.3.3 Grid template areas iOS Simulator 전체 행렬

- 실행 ID: c1133-ios-matrix-20261029T204016316Z-f19f0361
- 대상: iPhone 17 Pro / com.apple.CoreSimulator.SimRuntime.iOS-26-2 Simulator
- source commit: `92c8502be33ada9e2a71e32bea69ff048ea3f109`
- 고정 Chrome DPR 1 기준: 37 scene / 118 node
- 결과: 성공 32, cascade fallback 5, 기대 거부 0
- node frame 비교: 118개, 최대 geometry 오차 0 CSS px
- 초기 runtime 평가 `status=-12` 재실행: 0회 (최초 로그와 재실행 로그 모두 보존)
- basic-rectangles Metal drawable 표본: 2/2 RGBA exact; 최대 채널 오차 0
- track-list cycling Metal drawable 표본: 6 scene / 28 RGBA exact; 최대 채널 오차 0
- unknown-area overflow: 3개 Chrome geometry node, 3/3개 Metal 표본 exact; grid root 밖 item 포함
- leading named implicit lines: 7 scene / 14 visible node, 최대 geometry 오차 0 CSS px; 7/7 Metal 표본 exact
- negative controls: sample 좌표 rejected-before-readback, unknown scene rejected-before-runtime-draw
- Metal drawable은 OS compositor 제출 전이다. 실기기·hardware GPU·WPT 적합성·전체 CSS Grid 지원은 이 실행에 포함하지 않는다.

## 산출물

- runtime-*.log: scene별 평가, frame, 거부 결과
- ios-screen.png: basic-rectangles UI 화면 캡처
- ios-track-list-cycle-screen.png: 양 축 track-list cycling 화면 캡처
- ios-unknown-area-overflow-screen.png: fixed implicit-track overflow 장면 화면 캡처
- ios-leading-*-screen.png: leading named implicit line 장면 화면 캡처
- metal-surface-comparison.json: Chrome 고정 sample과 Metal drawable 비교
- result.json: fixture hash, geometry, pixel 및 negative-control 결과

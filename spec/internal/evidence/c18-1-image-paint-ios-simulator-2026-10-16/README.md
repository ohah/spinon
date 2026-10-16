# C18.1 iOS Simulator — 2026-10-16

## 실행 환경

- 대상: iPhone 17 Pro / iOS 26.2 Simulator, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- renderer: Metal / Apple iOS Simulator GPU.
- fixture process ID: `66309`.
- opt-in app executable SHA-256: `a40b9d549fd9ec31c388311dc69e39b6e3b8fba5b22856c0092731b778377479`.
- Metal drawable: `240×120` px, CSS viewport `80×40`, scale `3×3`, generation `1`.

## 결과

Metal drawable readback에서 고정 9개 CSS 좌표의 RGBA가 Chrome 155 oracle과 모두 exact였다. 전체 화면 캡처는 `1206×2622` px, 화면 scale은 `3`이다. canvas 원점은 `(161,417)` UI points, 크기는 `80×40` points다. 전체 화면 screenshot에서도 9개 대응 pixel과 fixture 바깥 네 색상 anchor가 일치해 comparator가 `matched`를 반환했다. Metal drawable readback은 OS compositor 이전 단계이며 full-screen screenshot 표본은 Simulator 화면 capture에서 별도로 읽었다.

| 근거 | 파일 | SHA-256 |
| --- | --- | --- |
| iPhone 전체 화면 | [full-screen.png](./full-screen.png) | `808ca9dbd53eabc55d0d7635cc9834f1e6c005cea4725525b6a320be96a65589` |
| 현재 process 원본 로그 | [simulator-log.txt](./simulator-log.txt) | `38565e220f43ae85add2c06aa116aba4dbd9f691f3bda7fa02d3739f9df1bfda` |
| machine-readable mapping | [post-compositor-observation.json](./post-compositor-observation.json) | — |

비교 명령은 `node tools/css-reference/compare-c181-compositor-pixels.mjs spec/internal/evidence/c18-1-image-paint-ios-simulator-2026-10-16/post-compositor-observation.json`이다. 결과는 drawable readback `9/9`, screenshot `9/9`, anchor `4/4` exact다.

이 증거는 지정 Simulator의 Metal drawable과 Simulator full-screen capture에 한정한다. iPhone 실기기, 별도 하드웨어 GPU, OS display controller가 최종 scanout한 물리 pixel과 제품 image loader는 검사하지 않았다.

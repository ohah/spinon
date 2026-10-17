# C13.1 iOS Simulator Metal full physical surface

- Capture 묶음: `c13-1-ios-full-physical-20261017T162023620Z-e570891e`
- 대상: iPhone 17 Pro · com.apple.CoreSimulator.SimRuntime.iOS-26-2 · ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D
- 새 앱 process 3개에서 각각 4,219 batch로 270,000 physical texel / 1,080,000 RGBA channel을 Chrome 155와 exact 비교했다.
- 각 실행에서 41/41 node frame, 52/52 고정 hit query와 30,000/30,000 hit identity가 같은 app run ID에 연결됐다.
- row-major `.rgba` artifact는 각 Metal readback의 256-byte padded row에서 RGBA 네 byte만 packed 저장하고 surface generation·format·byte length·SHA-256을 manifest에 고정한다.
- `positioned-escape`의 border 밖 texel을 포함해 전체 drawable을 비교하고 네 channel 1-byte mutation, 누락·추가·순서·padding·identity 변조를 거부했다.
- 잘못된 Metal route 단독 실행과 중복 C13.1 route가 host 생성 전에 거부됐다.
- [실행 요약](./summary.json) · build/install, 세 process log, screenshots, 직접 readback artifact/manifest와 Chrome 비교 결과를 보존했다.
- 이 결과는 iOS Simulator의 Metal drawable 표본만 검증한다. compositor scanout, 실기기·hardware GPU, DOM events, WPT 및 C13.1 전체 완료를 뜻하지 않는다.

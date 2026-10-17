# C13.1 stacking full viewport · iOS Simulator

- 실행 묶음: `c13-1-stacking-ios-20261017T195135274Z-ad00912d`
- 대상: iPhone 17 Pro · com.apple.CoreSimulator.SimRuntime.iOS-26-2 · ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D
- 새 앱 프로세스 3개에서 각각 13개 frame, 18개 hit/RGBA 표본, 11개 경계 제어와 69,120개 integer CSS point identity를 확인했다.
- 각 run에서 evaluation/draw의 generation·document/render-tree/style revision tuple이 같고 Chrome 155 identity digest가 일치했다.
- stale document revision과 route collision negative control을 보존했다. 성공 run 앱은 각 화면 캡처 뒤 종료했다.
- [실행 요약](./summary.json) · build/install 로그, process별 runtime/Metal 표본과 Simulator 화면을 보존했다.
- iOS Simulator 및 제한된 Metal sample만 확인했다. 실기기, full drawable pixel parity, DOM event dispatch, WPT 및 C13.1 전체 지원 완료를 뜻하지 않는다.

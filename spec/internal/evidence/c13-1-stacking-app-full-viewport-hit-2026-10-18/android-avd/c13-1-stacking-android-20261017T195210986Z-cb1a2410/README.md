# C13.1 stacking full viewport · Android AVD

- 실행 묶음: `c13-1-stacking-android-20261017T195210986Z-cb1a2410`
- 대상: emulator-5554 · sdk_gphone16k_arm64 · API 37 · arm64-v8a
- 새 앱 프로세스 3개에서 각각 13개 frame, 18개 hit/RGBA 표본, 11개 경계 제어와 69,120개 integer CSS point identity를 확인했다.
- 각 run에서 evaluation/draw의 generation·document/render-tree/style revision tuple이 같고 Chrome 155 identity digest가 일치했다.
- stale document revision과 route collision negative control을 보존했다. 성공 run 앱은 PixelCopy와 화면 캡처 뒤 종료했다.
- [실행 요약](./summary.json) · build/install/runtime 로그, PixelCopy 표면과 AVD 화면을 보존했다.
- API 37 ARM64 AVD만 확인했다. 실기기, hardware GPU, DOM event dispatch, WPT 및 C13.1 전체 지원 완료를 뜻하지 않는다.

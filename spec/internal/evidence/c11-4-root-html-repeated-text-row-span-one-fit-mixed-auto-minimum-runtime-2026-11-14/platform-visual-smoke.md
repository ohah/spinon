# C11.4 Android/iOS 화면 스모크 근거

- 실행일: 2026-11-14
- 입력: `root-html-mixed-screen.html`
- 입력 SHA-256: `a944c85929ac16065cd950d4edc58788f8eb98ab2759644aa6ceb23874b57676`
- 실행 경로: 앱의 C04.12 Root HTML 기본 렌더 경로 (`dev.spinon.bootstrap`, `--spinon-c0412-root-html-default-render`)
- 결과 범위: 두 플랫폼의 디버그 시뮬레이터에서 HTML 입력 로드, 레이아웃 준비, 그리기 제출과 화면 캡처를 확인했다. 픽셀 단위 비교나 GPU 하드웨어 렌더링 판정은 하지 않았다.

## Android

- AVD: `spinon_api37_2_presenttime_probe_20261018`, Android API 37, arm64-v8a
- 화면: 1080×2400, density 420; 앱 뷰포트 320×240 CSS px, device scale factor 2.625
- Light와 dark 로그 모두 입력 SHA가 일치하고 `layout=ready boxes=12`, `draw status=0 presented boxes=12`를 기록했다. Light 실행 초기에 superseded request `status=-12`가 한 번 있었고 같은 process의 다음 request가 성공했다. Dark 실행은 환경 갱신과 draw가 모두 성공했다.
- 렌더러 로그: Vulkan / Goldfish GFXStream의 llvmpipe 소프트웨어 장치. 실기기나 하드웨어 GPU 결과로 해석하지 않는다.
- 캡처: `android-light.png`, `android-dark.png` (각 1080×2400)
- 로그: `android-light.log`, `android-dark.log`

## iOS

- 기기 프로필: iPhone 17 Pro, iOS 26.2, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`
- 화면 캡처: 1206×2622; 앱 뷰포트 320×240 CSS px, scale 3
- Light와 dark 각각 입력 SHA가 일치하고 `layout=ready boxes=12`, `draw status=0 presented boxes=12`를 기록했다. dark 실행은 `dark=true`를 확인했다.
- Light와 dark 실행 모두 시작 직후 오래된 장면 요청이 `status=-12`로 거부된 뒤 새 요청이 `status=0`으로 성공했다. 성공 로그와 해당 화면 캡처를 함께 보존했다.
- 캡처: `ios-light.png`, `ios-dark.png` (각 1206×2622)
- 로그: `ios-light.log`, `ios-dark.log`

## 정리 상태

- 두 플랫폼의 임시 asset 덮어쓰기를 원본 SHA-256 `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`로 복구했다.
- Android AVD를 종료했고 `adb devices`에 연결 기기가 남지 않았다. AVD 프로필은 재사용을 위해 보존했다.
- iOS 앱을 종료하고 appearance를 원래 light로 돌린 뒤 해당 simulator를 `Shutdown` 상태로 만들었다. 기기 프로필은 보존했다.
- 캡처·로그는 PR 동작 근거로 보존한다. 일회성 원본 복사 staging 디렉터리는 해시 대조와 열린 파일 확인 후 삭제했다.
- Xcode `DerivedData`는 517 MB 공유 빌드 캐시여서 다른 작업 재빌드에 재사용할 수 있으므로 보존했다.

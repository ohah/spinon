# C11.4 three-fit row-span 플랫폼 visual smoke

## 범위

세 개의 `fit-content()` track이 있는 고정 Root HTML Grid를 기본 `C04.12` Root HTML route에서 실행했다. 같은 HTML을 iOS와 Android asset에 빌드 시 잠시 넣고, 빌드 후 두 저장소 asset을 원본 바이트로 복구했다. 앱 source log의 SHA-256과 fixture SHA가 일치한다.

- Fixture: [`platform-smoke.html`](./platform-smoke.html)
- Fixture SHA-256: `fe489c2d7387d36f6f7b75b728c48b52aef444049fc08488e1a9ab24ff8d1404`
- 두 기본 asset의 복구된 SHA-256: `38813ca802618bd20bdf3bbb9ed9bc7525c289f48c0928813c855567f9e7df85`
- fixture는 40×220 CSS px Grid, six-member auto-row list, span-2 repeated-text items, `min-height:0` light profile, `min-height:auto` dark profile, `prefers-color-scheme` media query를 담는다.
- 기존 tracked C11.1 percentage runtime test가 `include_str!`로 참조하지만 checkout에서는 미추적 상태였던 `root-html-grid-fit-content-percentage-runtime.html`도 함께 추가했다. clean checkout에서 해당 기존 test target이 컴파일되도록 보존한 fixture다.

첫 iOS 기동은 Root HTML 실행 인자 없이 실행되어 다른 기본 route를 사용했다. 그 실행은 fixture 결과로 취급하지 않고 캡처를 폐기했다. 올바른 `--spinon-c0412-root-html-default-render` route로 다시 실행한 로그와 캡처만 아래에 기록했다.

## iOS

- 기기: iPhone 17 Pro, iOS 26.2 Simulator
- 빌드: `mise exec -- bun run build:ios-sim`, C04 runtime GPU on, fixture-test flags off
- 앱 실행 인자: `--spinon-c0412-root-html-default-render`
- 앱 executable SHA-256: `00dc48371b9ab31fb75662bdffefd032f29b745b9b6f3c29262caadcbe9c599e`
- source log는 fixture `bytes=2581`과 위 SHA를 기록한다. 두 scheme 모두 `layout=ready`, `boxes=14`, `root_frame_css_px=320x220` 및 Metal `SPINON_C0410_DRAW status=0 presented`를 기록한다.
- iOS light 상태는 `dark=false`, `environment_revision=0`; OS appearance를 dark로 바꾼 뒤 앱 재기동 없이 `dark=true`, `environment_revision=1`과 새 draw를 기록했다.
- 화면: [`ios-light.png`](./ios-light.png), [`ios-dark.png`](./ios-dark.png)
- 로그: [`ios-build.log`](./ios-build.log), [`ios-light.log`](./ios-light.log), [`ios-dark.log`](./ios-dark.log)

검증 전 appearance는 dark였고 완료 후 dark로 복구했다. 스모크에 사용한 `dev.spinon.bootstrap` 앱을 시뮬레이터에서 제거하고 Simulator를 `Shutdown`했다. 공용 시뮬레이터 프로필과 다른 앱은 유지했다.

## Android

- 기기: Android 37.0 ARM64 AVD `spinon_api37_0_applied_txn`; 실제 연결된 SM-S731N에는 앱을 설치하거나 명령을 보내지 않았다.
- 빌드: `tools/build-android-app.sh`, C04 runtime GPU on, fixture-test flags off
- 실행: `dev.spinon.bootstrap/.MainActivity`에 `spinon_c0412_root_html_default_render=true`
- APK SHA-256: `b905959d1fcbcedea036e463b6629d71ffacd244316bbab9babd174d2e2e5c00`
- 설치 APK 안의 HTML SHA가 fixture SHA와 일치한다. 양 scheme 로그에 `layout=ready`, `boxes=14`, `root_frame_css_px=320x220`, draw `status=0 presented`가 있다. SwiftShader 기반 Android Emulator OpenGL surface는 840×630이다.
- 초기 night mode `no`에서 `yes`로 전환 후 재기동 없이 `dark=true`, `environment_revision=1` 및 새 draw를 기록했다.
- 화면: [`android-light.png`](./android-light.png), [`android-dark.png`](./android-dark.png)
- 로그: [`android-build.log`](./android-build.log), [`android-light.log`](./android-light.log), [`android-dark.log`](./android-dark.log)

기존 AVD 설치 앱의 APK SHA-256 `d53bab79298b13be877f4ebaac685f74731e792e784afbbabcef2f8cd8719711`을 설치 전에 백업했다. smoke 뒤 같은 APK를 재설치해 기기 안의 SHA가 동일한지 확인했고, 기존 night mode `no`로 복구했다. 앱 프로세스를 종료하고 AVD를 정상 종료했다. 데이터 삭제나 AVD 초기화는 하지 않았다. 임시 APK 백업 파일은 원래 APK 복원을 확인한 뒤 제거했다.

## 해석 범위

캡처는 Root HTML 화면의 OS screenshot이며 iOS compositor 후 drawable pixel readback, Android PixelCopy, 실기기 GPU, 물리 화면 출력 또는 pixel parity를 증명하지 않는다. Android surface는 SwiftShader software rendering이다. 이 자료는 지정된 C11.4 fixture의 시뮬레이터 visual smoke다.

# C11.1 Root HTML Grid automatic minimum 런타임 근거

이 자료는 C11.1의 제한된 `minmax(auto, finite-nonnegative-fr)` 후속 구현을 기본 Root HTML 앱 경로에서 확인한 실행 기록이다. 자동 minimum인 track과 `minmax(0,1fr)` control을 columns·rows 양쪽에서 대조한다. 단일 slice의 통과이며 C11/C11.1 공식 완료 판정은 아니다.

범위 계약은 [내부 계약 0059](../../0059-c11-grid-layout.md)에 정리했다.

## 결과

- 기본 Root HTML route에서 columns·rows auto minimum의 80px track base, fixed 40px neighbor, 2px root border와 18px visible overflow가 코드 geometry 및 화면에 나타났다. zero-minimum controls는 root 바깥 spill 없이 표시됐다.
- Rust runtime 테스트 4/4, style-to-layout auto-minimum 테스트 6/6 통과. 0fr의 style projection뿐 아니라 기본 runtime 배치도 두 축에서 추가 확인했다. ID·class 이름을 함께 바꾼 same-source control도 같은 paint frames를 냈다.
- Android API 37 ARM64 AVD PixelCopy 표본 4/4 exact. [화면](android/simulator-auto-minimum-pass03.png) · [SurfaceView PixelCopy PNG](android/pixelcopy-auto-minimum-pass03.png) · [원본 logcat](android/spinon-pixelcopy-auto-minimum-pass03.log)
- iPhone 17 Pro / iOS 26.2 Simulator Metal drawable 표본 4/4 exact. [화면](ios/simulator-auto-minimum-metal-pass01.png) · [원본 log](ios/spinon-auto-minimum-metal-pass01.log)
- Chrome 156 Agent Browser는 독립적인 사전 비교 관찰이다. [규범식에서 먼저 계산한 expected 값](../c11-root-html-grid-auto-minimum-precomparison-2026-11-03/expected.json), [Chrome 관찰](../c11-root-html-grid-auto-minimum-precomparison-2026-11-03/chrome-observation.json), [화면](../c11-root-html-grid-auto-minimum-precomparison-2026-11-03/chrome156-agent-browser.png).
- 구현/runtime 실패 관점 20개를 별도 기록했다: [검토 20개](implementation-runtime-review-20.md).

## 재현 대상과 한계

입력 fixture SHA-256은 `269bd653ed3cbb0e1602b2257548c9dd07534365026794ba7ea27f0a562b8611`이다. 앱 스크립트는 `globalThis.__spinonC11AutoMinimumBoot = true;`인 고정 46-byte marker이며, fixture 전송 중 원래 `examples/bootstrap/index.html`과 `root-app.js`는 백업본과 byte-for-byte 일치하도록 복구했다. 후보 코드 파일 hash는 [source-candidate-sha256.txt](source-candidate-sha256.txt), 플랫폼과 빌드 환경은 [target.json](target.json)에 있다.

Android는 `emulator-5556`의 `spinon_api37_0_applied_txn` AVD다. renderer는 ANGLE/SwiftShader software Vulkan 경로다. 실기기나 hardware GPU 근거가 아니다. Android 화면의 네 표본은 PixelCopy가 읽은 SurfaceView의 physical pixels다.

iOS는 iPhone 17 Pro / iOS 26.2 Simulator다. Metal 표본은 1206×2334 drawable의 compositor 제출 전 pixel이며 최종 UIKit 합성 화면, 실기기 GPU, 색 관리 출력의 증거가 아니다.

처음 사용한 기존 좌표는 다른 C11 fixture에서 온 것이어서 불일치했다. Android 재실행에서 run ID를 중복해 기존 PNG를 거부한 로그도 남겼다. 이 시도들은 성공 수에 포함하지 않는다. 유효 Android run은 새 ID `c11autominandroid06`에서 다시 얻은 `pass03`이다. 캡처 충돌 인수의 AVD 음성 확인은 [로그](android/conflicting-pixel-capture-pass01.log)에 있다.

## 기본 artifact 경계

기본 `spinon-ffi` staticlib에는 C11 surface-pixel fixture의 두 심볼이 없고, opt-in `c11-grid-metal-surface-pixels-test` positive control에서만 기존 심볼이 나타났다. 기본 Android build에서는 capture BuildConfig가 `false`이고 native artifact에도 같은 심볼이 없다. Android는 시스템 PixelCopy API를 사용해 읽기 때문에 C11.1 auto-minimum 전용 FFI route/payload를 추가하지 않았다. 빌드 로그, symbol 목록 및 hash는 이 폴더의 `ffi-*` 파일과 `android-default-*` 파일을 참조한다.

기본 Android build 로그는 capture 옵션을 끈 artifact를 검사하기 위한 것이다. 설치·화면 검증은 capture 옵션을 켠 debug APK에서 했으며 APK SHA-256은 `target.json`에 분리 기록했다.

## 후속 관문

현재 branch HEAD는 계획 병합 SHA `a7838b7de949f79ee7e39030845fb83ee3fafeb7`이고 구현은 아직 checkout의 미병합 후보다. GitHub PR diff 검토 30개, required CI, PR 병합 및 병합 후 상태·로드맵 동기화는 남아 있다. 공식 상태 대장 `72/213` 및 C11 checkbox는 변경하지 않았다.

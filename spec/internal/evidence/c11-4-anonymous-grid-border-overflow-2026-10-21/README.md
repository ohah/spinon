# C11.4 anonymous Grid border overflow 실행 근거

2026-10-21 고정 fixture의 두 child 순서를 Android AVD와 iOS Simulator에서 실행했다. 각 플랫폼에서 두 장면 모두 layout, track trace, 실제 image draw와 지정한 surface pixel 검사를 통과했다.

## 실행 결과

| 대상 | 런타임 | 장면 | 앱 식별값 | 화면 |
| --- | --- | --- | --- | --- |
| Android | API 37 / Android 17, ARM64 AVD `emulator-5558` | forward | APK SHA-256 `f4c0f90d3a9c357596f7d7d77424a189542546f2bbf94eafdc3f435696d915c1` | [forward](./android-avd-c114-anonymous-grid-border-overflow-android-avd-20261021T070322769Z-ef7ddc0e/anonymous-runs-two-images-forward.png) |
| Android | API 37 / Android 17, ARM64 AVD `emulator-5558` | reversed | 같은 APK SHA-256 | [reversed](./android-avd-c114-anonymous-grid-border-overflow-android-avd-20261021T070322769Z-ef7ddc0e/two-images-anonymous-runs-reversed.png) |
| iOS | iOS 26.2, iPhone 17 Pro Simulator | forward | App SHA-256 `c34d8123aceb44c1db1b4ca0d86ff735efe6fb7739eca9fa20cacc5b9a8b534c` | [forward](./ios-simulator-c114-anonymous-grid-border-overflow-ios-simulator-20261021T070603043Z-8ea2e515/anonymous-runs-two-images-forward.png) |
| iOS | iOS 26.2, iPhone 17 Pro Simulator | reversed | 같은 App SHA-256 | [reversed](./ios-simulator-c114-anonymous-grid-border-overflow-ios-simulator-20261021T070603043Z-8ea2e515/two-images-anonymous-runs-reversed.png) |

두 장면은 다음 결과를 기록했다.

- border-box root `40×184 CSS px`, content width `36px`, columns `[12, 28, 20, 20]`, rows `[60, 60, 60]`.
- 48×32 image frame은 `(14, 62)`에서 시작해 root 오른쪽 border-box 끝 `x=40`보다 `22px` 더 나간다.
- 각 화면에서 `runtime_image_count=2` 및 `image_draw_count=2`; `drawPresented=true`, retry 0회.
- Android PixelCopy는 CSS `(10,0)`에서 border `240,198,116,255`, `(51,71)`에서 overflow PNG `0,255,64,255`를 두 장면 모두 정확히 읽었다. 표면 배율은 `2.625`.
- iOS Metal drawable은 두 지점의 같은 RGBA를 정확히 읽었다. 표면 `960×720`, CSS viewport `320×240`, 배율 `3.0`. 읽기는 compositor 전 drawable에 한정된다.

상세 실행 로그와 결과 JSON은 [Android 결과](./android-avd-c114-anonymous-grid-border-overflow-android-avd-20261021T070322769Z-ef7ddc0e/result.json) 및 [iOS 결과](./ios-simulator-c114-anonymous-grid-border-overflow-ios-simulator-20261021T070603043Z-8ea2e515/result.json)에 있다. 설치 APK 해시는 [설치 검증](./android-avd-c114-anonymous-grid-border-overflow-android-avd-20261021T070322769Z-ef7ddc0e/installed-apk-verification.json)에 기록되어 있다.

공유 validator 변경의 회귀 범위를 위해 기존 anonymous text + image 장면 4개도 같은 앱 빌드로 다시 실행했다. [Android AVD 결과](../c11-4-anonymous-text-multi-image-2026-10-21/android-avd-c114-anonymous-text-multi-image-regression-android-avd-20261021T071647292Z-62f7749a/result.json)와 [iOS Simulator 결과](../c11-4-anonymous-text-multi-image-2026-10-21/ios-simulator-c114-anonymous-text-multi-image-regression-ios-simulator-20261021T071647292Z-fd85173e/result.json)에서 네 장면 모두 frame, trace와 draw assertion을 통과했다.

PR [#431](https://github.com/ohah/spinon/pull/431) 본문의 forward scene 이미지 두 장을 Agent Browser 0.31.1에서 확인했다. Android `1080×2400`, iOS `1206×2622` 이미지 모두 `complete=true`이고 화면에 표시됐다. [브라우저 확인 JSON](./pr-images-agent-browser.json)에 head commit과 크기를 기록했다. 확인 뒤 전용 browser session을 닫았고 Agent Browser 프로세스는 남지 않았다.

## 빌드와 검사

- [Android fixture 앱 빌드](./android-build.log), [iOS Simulator fixture 앱 빌드](./ios-build.log) 성공.
- [기본 workspace Clippy](./workspace-clippy.log) 및 [opt-in FFI fixture Clippy](./ffi-fixture-clippy.log) 경고 없음.
- [Rust 포맷](./fmt-check.log), [runtime 집중 테스트](./runtime-focused-test.log), [style-to-render 집중 테스트](./style-to-render-focused-test.log), [FFI 집중 테스트](./ffi-focused-test.log) 통과.
- [기본·fixture archive 경계 검사](./feature-boundary/byte-scan.json): macOS ARM64 기본 `spinon-ffi` archive에는 fixture ABI 심볼, 두 scene ID, border/source marker 및 97-byte PNG payload가 없고, opt-in fixture archive에는 모두 있다.

## 범위와 한계

이 결과는 opt-in 고정 fixture의 Ahem text, 두 direct image, 네 개 `min-content` track과 opaque solid border에 한정된다. fixture에서 명시적 `overflow:visible`은 cascade profile의 지원 범위 밖이라 생략했다. CSS 초기값인 visible을 사용한다. `justify-items:start` 선언도 fixture profile에서 처리하지 못해 생략했으며 image의 초기 정렬값으로 같은 frame을 확인했다. 이미지는 24×16 PNG natural size에서 CSS 기본 `object-fit:fill`로 24×16 및 48×32 box에 그린다.

Android 결과는 API 37 ARM64 AVD에서만 검증했으며 물리 Android 기기 검증이 아니다. iOS pixel readback은 compositor 이전이다. 단일 PNG 색 샘플은 overflow의 가시성 증거이며 전체 이미지 texel equality를 뜻하지 않는다. 이 fixture는 임의 HTML/CSS, 모든 border 종류, clipping/scroll, WPT 적합성 또는 전체 C11.4 지원을 입증하지 않는다. 공식 [상태 대장](../../../STATUS.md)은 미완료로 유지하며 완료 수를 변경하지 않는다.

# C11.4 반복 intrinsic text runtime 후보 실행 근거

**상태:** 미병합 구현 후보 · Android runtime gate 미완료 · 2026-11-08

실패 관점 기록: [구현/runtime 20개](./implementation-runtime-review-20.md) · [PR diff 30개](./pr-diff-review-30.md)

## Rust 비교와 회귀

고정 [Chrome 157 expected JSON](../c11-4-root-html-repeated-intrinsic-text-precomparison-2026-11-08/chrome157-expected.json)의 270 scene을 RuntimeSession에서 실행했다. 30개 고유 tail order × 4개 cap × 2개 alignment의 240 scene과 100px Grid visible-overflow 30 scene에서 Grid/item/payload frame 차이는 각 component별 최대 `0.01 CSS px` 이내였다. Typed track order, computed serialization, glyph 수와 baseline으로 계산한 line 수는 exact다.

| 검사 | 결과 |
| --- | --- |
| RuntimeSession 반복 텍스트 scene/rejection/recovery | 3 tests 통과; 270 geometry scene 포함 |
| Root HTML fit-content RuntimeSession 회귀 | 23 tests 통과 |
| `spinon-layout` fit-content 회귀 | 14 tests 통과 |
| `spinon-style-to-layout` fit-content 회귀 | 7 tests 통과 |
| 변경 crate Clippy | `cargo clippy -p spinon-layout -p spinon-style-to-layout -p spinon-runtime -- -D warnings` 통과 |
| Rust 서식 | `cargo fmt --all -- --check` 통과 |

## iPhone 17 Pro / iOS 26.2 Simulator

- 최종 모듈 분리 이후 Android/iOS app을 다시 빌드했다. 임시 iPhone 17 Pro Simulator UDID `83A02B03-200F-4C30-B61C-B6F96D027168`에서 최종 iOS build를 실행했다. Simulator를 임시 생성해 기존 앱 data를 사용하지 않았다.
- 실행 HTML은 2,758 bytes이며 fixture SHA-256은 `923a9d831a6509c1eb577428296535369146034336ac5af6658af0506157b8a9`다. 앱 bundle의 `index.html` SHA-256이 fixture와 같은 것을 install 전에 확인했다.
- 현재 process 로그는 `layout=ready boxes=20`, `page_scale=1`, `root_frame_css_px=402x220`을 기록한다. [process log](./ios/app-current-process.log) · [device metadata](./ios/device-metadata.json)
- [1206×2622 screenshot](./ios/screenshot.png)은 OS screenshot visual smoke다. 화면에 8개 고정 문장과 줄바꿈이 보인다. 이것은 Metal drawable readback, compositor 이후 pixel 비교 또는 100px overflow scene의 Simulator 검증이 아니다.

## Android API 37 ARM64 AVD

처음 사용한 `spinon_c11_4_fit_content_pass04_20261107` AVD는 Command-line Tools 12.0이 만들었고 root `.ini`의 `target=android-0`이 API 37.0 ARM64 시스템 이미지와 맞지 않았다. [공식 emulator 소스](https://android.googlesource.com/platform/external/qemu/+/emu-master-dev/android-qemu2-glue/main.cpp#2044)는 API 21 이상 ARM64 AVD에 macOS HVF를 활성화한다. 이 AVD는 API target을 0으로 읽어 QEMU argv에 `-enable-hvf`를 넣지 않았고, guest 부팅 중 `mprotect failed: Permission denied`가 반복된 뒤 exit 139로 종료됐다. 기존 로그는 [초기 mprotect 실패](./android/emulator-hardware-accel-failure.log), [이번 상세 실패](./android/emulator-invalid-target-20261108.log), [`-accel off` 재시도](./android/emulator-no-accel.log)에 보존했다.

Command-line Tools 23.0으로 만든 새 Pixel 6/API 37.0 ARM64 AVD `spinon_c114_android37_runtime_20261108`은 root target `android-37.0`, `CPU Acceleration: working`, QEMU `-enable-hvf`, `sys.boot_completed=1`, ADB `device`를 확인했다([성공 부팅 로그](./android/emulator-valid-target-20261108.log), [AVD 메타데이터](./android/device-metadata.json)). 같은 2,758-byte fixture의 SHA-256 `923a9d831a6509c1eb577428296535369146034336ac5af6658af0506157b8a9`이 APK `assets/index.html`과 일치했다([최종 build log](./android/final-build.log), [APK·fixture·screenshot hash](./android/final-candidate.sha256)).

Android 앱을 기본 Root HTML route에서 실행해 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`, `layout=ready boxes=20`, `presented boxes=20`, `runtime_text_count=8`을 확인했다([앱 process log](./android/app-current-process.log), [1080×2400 OS screenshot](./android/screenshot.png)). screenshot은 SwiftShader visual smoke다. PixelCopy/readback, hardware GPU, 실기기 또는 compositor 이후 pixel 비교가 아니다.

## 복원과 cleanup

- 테스트 전 `examples/bootstrap/index.html`, `examples/bootstrap/root-app.js`, 기존 Android APK 및 iOS Simulator app binary를 백업했다. 모두 복원한 뒤 [before](./before.sha256)와 [after](./after.sha256) SHA-256을 비교해 동일함을 확인했다.
- iOS 임시 Simulator는 종료·삭제했다. Android에서 잘못된 target을 가진 실패 AVD는 원인 로그를 보존한 뒤 제거하고, 재사용할 API 37.0 AVD는 다음 C11.4 검증을 위해 종료 상태로 유지한다. 임시 `examples/bootstrap` 입력은 HEAD 버전으로 복원했다. Android APK 출력은 최종 후보 build로 갱신됐고 별도 hash를 보존한다.
- 이전 실행에서 확인한 [before/after hash](./before.sha256)와 [후보 build hash](./candidate-builds.sha256)는 그 시점 기록으로 유지한다. 이번 최종 Android APK·fixture·screenshot은 [별도 hash 목록](./android/final-candidate.sha256)에 기록했다. 기존 Android APK 출력은 이번 후보 APK로 바뀌었으며 복원된 것으로 간주하지 않는다.

이 후보는 일반 CSS admission, 임의 text/font/content, WPT, 실기기, hardware GPU 또는 C11.4/C11 완료를 뜻하지 않는다. 공식 완료 `78/214`와 내부 계약 `0.1.0`은 그대로다.

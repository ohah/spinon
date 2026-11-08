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

최종 모듈 분리 이후 Android app build는 성공했다([build log](./android/build.log), [APK fixture hash](./android/candidate-index.sha256)). 앞서 임시 Pixel 6/API 37 ARM64 AVD를 새로 만들어 QEMU를 실행했지만, ADB 연결이 `device`에서 닫힌 뒤 `qemu_mprotect__osdep: mprotect failed: Permission denied`와 QEMU thread hang이 기록됐고 프로세스는 exit 139로 끝났다([첫 실행 log](./android/emulator-hardware-accel-failure.log)). ADB offline 상태에서 `-accel off`를 지정한 재시도에도 같은 mprotect 오류가 발생했다([재시도 log](./android/emulator-no-accel.log)). 이 실패는 모듈 분리 전이며 APK 설치 전에 발생했다. Android app runtime은 **미검증**이고 이 로그를 앱 오류로 해석하지 않는다.

## 복원과 cleanup

- 테스트 전 `examples/bootstrap/index.html`, `examples/bootstrap/root-app.js`, 기존 Android APK 및 iOS Simulator app binary를 백업했다. 모두 복원한 뒤 [before](./before.sha256)와 [after](./after.sha256) SHA-256을 비교해 동일함을 확인했다.
- Android/iOS 두 app build는 성공했으며, 이전 build output을 되돌렸다. iOS 임시 Simulator와 두 차례 사용한 임시 Android AVD는 종료·삭제했고 기존 simulator data는 삭제하지 않았다.
- `before/after` hash가 일치하므로 build inputs/outputs는 검증 전 상태로 복원됐다. [후보 APK/app binary hash](./candidate-builds.sha256)와 [Android](./android/candidate-index.sha256)·[iOS](./ios/candidate-index.sha256) fixture hash를 따로 보존한다.

이 후보는 일반 CSS admission, 임의 text/font/content, WPT, 실기기, hardware GPU 또는 C11.4/C11 완료를 뜻하지 않는다. 공식 완료 `78/214`와 내부 계약 `0.1.0`은 그대로다.

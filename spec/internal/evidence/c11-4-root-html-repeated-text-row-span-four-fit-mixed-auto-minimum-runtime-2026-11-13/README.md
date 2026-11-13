# C11.4 four-fit mixed minimum 런타임·플랫폼 실행 근거

이 기록은 four-fit repeated-text row-span의 item별 `min-height: 0|auto` 제한 구현과 그 검증 범위를 기록한다. 이 검증은 제한된 Root HTML fixture 범위이며, 일반 CSS Grid 적합성이나 C11.4/C11 전체 완료를 뜻하지 않는다.

구현 PR [#1375](https://github.com/ohah/spinon/pull/1375)은 `2026-11-13T14:51:55Z` Rebase and merge됐고 merge SHA는 `22161976cab0bfd68a1d545eeb616441a0ab7a0e`다. [병합 기록](./merge-PR1375.md)을 확인한다.

## RuntimeSession 후보 검증

- 기준 커밋: 계획 PR #1374 merge SHA `97a5ca9b5a5c2d4a585ff76ee12bfb73547bb40b`
- 이 문서에 기록한 Rust 구현·fixture patch SHA-256: `72b653eeae34270b7dc34a36efed5776e226ec852379fdbdcb430fe1569480c0`
- 최종 후보 mixed sample 명령: `mise exec -- cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test parsed_root_html_four_fit_mixed_auto_minimum -- --nocapture`
- 결과: 6 passed, 0 failed, 340.41초. 20개 track-list 묶음의 3,840 scene을 모두 실행했다. 전체 Chrome oracle 138,240 scene과 혼동하지 않는다.
- 출력: [`runtime-mixed.log`](./runtime-mixed.log)
- 관측된 기대 control 값: zero/auto mask의 automatic-floor shortfall `[1024, 0]`, Grid-end overflow `[8, 0]`; 자동 minimum이 필요한 auto mask에서는 둘 다 0이다.
- 같은 후보에서 실행한 기존 회귀 결과: uniform endpoint 11,520/11,520 passed, four-fit auto-placement 5,760/5,760 passed, track trace/control/rejection 관련 focused tests 9개 passed. 해당 이전 실행의 원시 Cargo 출력은 별도 파일로 보존하지 않았다.
- `mise exec -- cargo fmt --all -- --check`와 `mise exec -- cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --lib -- -D warnings`가 통과했다. workspace 전체 strict Clippy는 변경하지 않은 테스트 파일의 기존 lint 6개로 실패했으며, 이 결과를 후보 수정 실패로 계산하지 않았다.
- 새로운 구현/runtime 실패 관점 20개 검토와 전체 PR diff 실패 관점 30개 검토를 완료했다. [구현/runtime review](./implementation-runtime-review-20.md), [PR diff review](./pr-diff-review-30.md)

## iOS 26.2 startup smoke

- 첫 앱 빌드 시도는 iOS V8 archive가 아직 없어 실패했다. [`ios/build.log`](./ios/build.log)에는 missing archive 원인이 남아 있고, V8 archive 생성 후 `mise exec -- bun run build:ios-sim` 앱 빌드는 성공했다. 링크 단계의 `RawWindowMetalLayer` 중복 심볼 경고는 비치명적이었다. [`ios/v8-build.log`](./ios/v8-build.log), [`ios/app-build.log`](./ios/app-build.log)
- 임시 iPhone 17 Pro / iOS 26.2 Simulator에서 기본 Root HTML 경로를 light/dark 두 번 실행했다. 두 실행 모두 `source=index.html`, `layout=ready`, `boxes=3`, `SPINON_C0410_DRAW status=0 presented`를 기록했다.
- [`ios/light-run.log`](./ios/light-run.log), [`ios/dark-run.log`](./ios/dark-run.log), [`ios/light.png`](./ios/light.png), [`ios/dark.png`](./ios/dark.png)
- 이는 앱 startup/draw smoke이며 mixed Grid fixture 실행이나 화면 픽셀 parity 검증이 아니다. 사용한 임시 iOS Simulator profile은 종료 후 삭제했다.

## Android API 37 headless startup smoke

- Android V8 archive와 `mise exec -- bun run build:android` APK 빌드가 통과했다.
- 새 임시 API 37 ARM64 AVD는 cold boot 중 QEMU hang 후 segmentation fault로 실패했고 task-owned profile은 제거했다. [`android/emulator.log`](./android/emulator.log), [`android/avd-create.log`](./android/avd-create.log).
- 기존 공유 `spinon_api37_0_applied_txn` profile의 GUI 시작도 실패했다. 이전 crash-consent 대화상자를 그리는 Qt 경로에서 QEMU가 종료됐다. 원인은 [Android crash 분석](./android/crash-analysis.md), 원본 실행 기록은 [`android/shared-avd-emulator.log`](./android/shared-avd-emulator.log)에 있다.
- 이후 같은 공유 API 37 / Android 17 / ARM64 profile을 snapshot 저장 없이 headless로 시작해 기본 `index.html` route를 light와 dark로 실행했다. 양쪽 모두 `layout=ready`, `boxes=3`, `SPINON_C0410_DRAW status=0 presented`를 기록했다. 설치 전 앱이 없는 것을 확인하고 smoke 후 APK를 제거했으며, night mode를 복구하고 emulator를 종료했다. 기기와 빌드 정보는 [`android/device-info.txt`](./android/device-info.txt), 모드별 로그와 캡처는 [`android/light.log`](./android/light.log), [`android/dark.log`](./android/dark.log), [`android/light.png`](./android/light.png), [`android/dark.png`](./android/dark.png)에 있다.
- 현재 ADB device와 booted emulator는 없고 공유 API 37 profile은 다음 C11 Android 검증에서 재사용하기 위해 보존했다. 임시 AVD profile은 재사용하지 않아 제거했다.

## 세션 정리

- S03 pilot iOS Simulator UDID는 현재 device inventory에 없었고, booted iOS Simulator도 없었다.
- S03 pilot UDID와 사용한 임시 iOS Simulator profile은 현재 device inventory에 없다. 임시 Android AVD도 제거했고 ADB device와 booted iOS Simulator는 없다. 공유 API 37 AVD는 다음 C11 Android 검증에서 재사용할 대상이라 보존했다.

## 남은 한계

- Android light/dark smoke는 기존 snapshot 기반 headless API 37 AVD에서만 통과했다. fresh cold boot와 GUI launch는 QEMU/Qt 실패를 보였으며 [crash 분석](./android/crash-analysis.md)에 분리했다. 이 결과를 다른 AVD, 실기기 또는 hardware GPU로 확대하지 않는다.
- iOS/Android smoke는 mixed Grid fixture가 아닌 기본 Root HTML startup 경로만 대상으로 한다. mixed feature geometry나 surface pixel parity는 검증하지 않았다.
- 이 slice는 공식 구현 상태 `78/214`와 C11.4/C11 완료 판정을 바꾸지 않는다.

# C04.12 root HTML 환경 cascade 실행 근거

## 범위와 판정

이 실행은 앱 bundle의 `index.html`로 생성한 V8 root HTML session에 surface pixel 크기, DPR, OS light/dark 선호를 넣고 CSS viewport·제한 media cascade 입력이 갱신되는지 확인한다. HTML layout/paint, UA Canvas 색, native 배경·form control 테마, page-scale transform은 구현되지 않았고 이 자료도 이를 증명하지 않는다. 따라서 C04.12는 미완료이며 상태 대장의 체크 수는 바뀌지 않았다.

## 실행 환경

- Android Emulator: serial `emulator-5554`, model `sdk_gphone16k_arm64`, API 37 / Android 17.
- iOS Simulator: iPhone 17 Pro, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, iOS 26.2, Xcode 26.2 (17C52).
- V8 정적 산출물은 기존 workspace의 `/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8`에서 읽었다. 원본 workspace와 V8 checkout은 수정하지 않았다.
- 두 Simulator의 앱 화면은 비어 있는 native host다. HTML paint가 없으므로 동작 화면 이미지를 만들거나 PR에 첨부하지 않았다.

## Simulator 결과

### Android

APK를 설치하고 `MainActivity`를 실행했다. OS night mode를 light→dark→light로 바꾼 결과:

- 최초: viewport `411.428558×914.285706 CSS px`, DPR `2.625`, `prefers_color_scheme=0`, `document_color_scheme=0`, revision `0`.
- dark 전환: 두 scheme 값 `1`, revision `1`.
- light 복귀: 두 scheme 값 `0`, revision `2`.
- `SPINON_ROOT_HTML_BOOT status=pass source=index.html`, document revision 2 / 20 nodes.
- 완료 후 앱에 `am force-stop`을 적용해 앱 PID가 남지 않았고 night mode는 `no`로 복원했다. AVD는 유지했다.

원본은 [android-api37-logcat.txt](android-api37-logcat.txt), 설치·실행 명령 결과는 [android-install.log](android-install.log)와 [android-launch.log](android-launch.log)다.

### iOS

iPhone 17 Pro에서 앱을 실행하고 `simctl ui appearance`로 light→dark→light를 적용했다.

- 최초: viewport `402×874 CSS px`, DPR `3`, `page_scale=1`, scheme 값 `0`, revision `0`.
- dark 전환: scheme 값 `1`, revision `1`.
- light 복귀: scheme 값 `0`, revision `2`.
- `SPINON_ROOT_HTML_BOOT status=pass source=index.html`, document revision 2 / 20 nodes.
- 완료 후 앱을 종료하고 Simulator launchctl 목록에서 앱 process가 없는 것을 확인했다. Simulator는 유지했다.

원본은 [ios-iphone17pro-log.txt](ios-iphone17pro-log.txt)다.

## 빌드와 코드 검증

- `mise exec -- env SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 bun run build:android` — `BUILD SUCCESSFUL`.
- 같은 환경에서 `bun run build:ios-sim` — `BUILD SUCCEEDED`.
- Android 로그에는 AGP 권장 버전 경고가 남았지만 빌드는 성공했다. iOS 로그의 AppIntents metadata skip 및 V8 archive duplicate debug-map timestamp 경고도 빌드 실패가 아니다.
- `cargo fmt --all -- --check` 및 `git diff --check` 통과.
- `cargo test -p spinon-style --locked`: 174 passed.
- `cargo test -p spinon-style-to-layout --locked`: 187 passed.
- `cargo test -p spinon-runtime --locked`: 162 passed, 3 ignored; 별도 integration 대상 3 passed 및 16 passed.
- `cargo test -p spinon-ffi --locked`: 20 passed.
- `cargo clippy --locked --workspace --all-targets -- -D warnings` 통과.

각 명령의 원본 stdout/stderr는 `cargo-*.log`, `git-diff-check.log`, `android-build.log`, `ios-build.log`에 보존했다. log hash는 [SHA256SUMS](SHA256SUMS)에 있다.

FFI unit-test binary 안에서 V8 session을 만드는 추가 end-to-end test는 isolate 생성이 실패해 제거했다. 이를 FFI 전체 통과로 계산하지 않았다. null 인자 FFI 단위 검사와 양쪽 Simulator 앱에서 실제 C API를 호출한 실행 근거를 분리해 기록한다.

## 남은 제품 범위

viewport metadata는 제한 allowlist만 계산한다. 전체 viewport descriptor·pinch zoom·visual viewport, CSS `color-scheme`의 UA 우선순위, Canvas/system color, root HTML layout·paint, 전체 CSS media query conformance는 미완료다. 이 PR은 내부 후보 runtime slice만 추가하며 공개 CSS/API 완료를 뜻하지 않는다.

구현 실패 관점 20개는 [implementation-review-20.md](implementation-review-20.md), PR diff 실패 관점 30개는 [pr-delta-review-30.md](pr-delta-review-30.md)에 기록했다.

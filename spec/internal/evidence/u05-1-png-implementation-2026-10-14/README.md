# U05.1 PNG 구현과 현재 소스 회귀 · 2026-10-14

이 근거는 `feat/u05-image-resource`의 현재 U05.1 변경에서 수집했다. 범위는 정적 PNG decode, connected HTML `img src` identity, revision-stamped resource snapshot과 C14.1 HostDocument→Stylo→Taffy fixture다.

## 결과

- `cargo test --locked --workspace`: 729 passed, 4 ignored, 0 failed. 원본은 [`workspace-test.log`](./workspace-test.log).
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과. 원본은 [`workspace-clippy.log`](./workspace-clippy.log).
- `cargo test --locked -p spinon-image`: 10 passed, 0 failed. 원본은 [`spinon-image-test.log`](./spinon-image-test.log).
- `cargo test --locked -p spinon-style-to-layout c14_`: 11 passed, 0 failed; 161 filtered. 원본은 [`c14-layout-test.log`](./c14-layout-test.log).
- 집중 Clippy, `cargo fmt --all -- --check`, `git diff --check` 통과. 원본은 [`focused-clippy.log`](./focused-clippy.log), [`rustfmt-check.log`](./rustfmt-check.log), [`diff-check.log`](./diff-check.log).
- Android `aarch64-linux-android` target `cargo check` 및 test executable build, API 37 arm64 AVD에서 Rust test 10/10 통과. build·실행·ADB 진단은 [AVD 근거](../u05-1-png-android-avd-2026-10-14/README.md).
- Chrome agent-browser precomparison과 C14.1 layout fixture는 [precomparison](../u05-1-png-precomparison-2026-10-14/README.md) 및 workspace test log에 기록했다.

## 범위

AVD에서는 Rust test executable만 실행했다. Android APK/V8 image runtime adapter, 이미지 화면 표시, GPU surface pixel은 이 코드 변경에 없으며 검증하지 않았다. iOS 실행도 수행하지 않았다. U05 전체와 C14.1 제품 runtime 경계는 미완료다. ADB daemon은 정상이고, 최초 연결 실패는 사용한 `spinon_api37_2.ini`의 누락 AVD 경로 문제였다. 이번에는 이미 설치된 `spinon_api37_1_compat16k` arm64 AVD를 사용했다.

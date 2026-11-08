# C11.4 반복 intrinsic tail 30순서 구현 진행 근거

## 현재 단계

`fit-content(px|%)`를 첫 member에 고정하고 뒤에 `min-content` 2개, `max-content` 2개, `auto` 1개가 오는 30개 고유 순서를 제한 Root HTML profile에 연결한다. PR #1284의 기존 순서도 30개 중 하나다. 공개 C11.4/C11 체크와 완료 `78/214`는 미완료로 유지한다.

첫 RuntimeSession matrix 실행에서 style projection은 새 순서를 받아들였지만 layout의 `supports_repeated_min_max_content_auto_track_list`가 PR #1284의 exact order만 허용해 `unsupported_grid_auto_rows`로 거부했다. layout gate를 같은 고정-prefix·후행 multiset 조건으로 넓혔다. 이 원인을 고친 뒤 30순서 전체의 독립 기대값 검증이 통과했다.

## Rust 검증

| 범위 | 결과 |
| --- | --- |
| Style projection | 2/2 tests; 30순서 × 4 cap form = 120 typed cases |
| Layout | 2/2 tests; 30순서 × px/% × start/stretch = 120 scenes, 독립 row/frame 기대값과 일치 |
| RuntimeSession | 4/4 tests; 120 matrix scenes와 기존 same-session 거부·복구 및 overflow 회귀 |
| Rust format | `cargo fmt --all -- --check` 통과 |

실행 명령은 `cargo test --locked -p spinon-style-to-layout --lib root_html_repeated_intrinsic_auto_row_list`, `cargo test --locked -p spinon-layout --lib six_member_repeated_intrinsic`, `cargo test --locked -p spinon-runtime --lib parsed_root_html_repeated_intrinsic`이다. Changed-crate `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings`도 통과했다. [구현/runtime 실패 관점 20개 검토](./implementation-runtime-review-20.md)와 [PR diff 실패 관점 30개 검토](./pr-diff-review-30.md)를 마쳤다.

## Android API 37 ARM64 AVD

- `emulator-5554`, `ro.kernel.qemu=1`, API 37, `arm64-v8a`에서 새 순서가 든 기본 parsed Root HTML fixture를 실행했다.
- Android APK의 HTML SHA-256은 `f0ad5ac93b87577d04ff0f4960e21c7adade65f1d66a6af6c1ce05f6be63a74f`이고 fixture 원본과 일치한다(3,007 bytes).
- 새 앱 process가 `layout=ready boxes=20` 및 `SPINON_C0410_DRAW status=0 presented boxes=20`를 기록했다.
- [AVD screenshot](./android/pass01/final/screenshot.png), 1080×2400. [process log with trailing whitespace normalized](./android/pass01/final/app-current-process-normalized.log)
- The captured raw process log remains unchanged in the workspace; its SHA-256 is `7d2f395f354bf07328afc9f4ae96087cc556e62ae680e305370058d67b742f26`. The committed readable copy only strips trailing spaces/tabs and ensures a final newline ([normalization record](./android/pass01/final/app-current-process-normalization.json)).
- 이전 설치 APK를 복원한 뒤 기기에 설치된 base APK를 다시 pull해 백업 hash와 동일함을 확인했다([복원 hash](./android/pass01/final/restore-installed-apk.json)).

## iPhone 17 Pro / iOS 26.2 Simulator

- `Spinon-C114-FitContent-Pass04`, UDID `97E99AA8-D144-4DA8-B1BF-7F6B34007F8B`에서 같은 3,007-byte fixture hash를 실행했다.
- 새 앱 process의 Root HTML boot와 environment layout이 `layout=ready boxes=20`이었다.
- [iOS Simulator screenshot](./ios/pass01/final/screenshot.png), 1206×2622. [process log](./ios/pass01/final/app-current-process.log)
- 설치 후 iOS app data를 복원했다. 백업 전후 파일 manifest가 byte-for-byte 일치한다([before](./ios/pass01/final/data-container-before.sha256), [after restore](./ios/pass01/final/data-container-after-restore.sha256)).

두 앱 화면은 OS screenshot visual smoke이고, Android PixelCopy, iOS Metal drawable readback, hardware GPU 또는 compositor 후 pixel parity를 검증하지 않는다. 앱·fixture build hash는 [build artifacts](./built-artifacts.sha256)에 있다. Bootstrap 원본도 [복원 hash](./bootstrap-restored.sha256)와 기존 hash가 일치한다. 기존 앱·build artifact 백업은 `build/spinon/c11-4-root-html-repeated-intrinsic-tail-permutations-20261108-pass01/`에 보존했다.

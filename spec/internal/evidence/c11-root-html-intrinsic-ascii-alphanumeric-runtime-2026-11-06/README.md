# C11.4 Root HTML ASCII 영숫자 단어 intrinsic sizing 실행 근거

이 제한 구현은 기본 parsed Root HTML direct-text source gate에서 token grammar를 `[A-Za-z]+`에서 `[A-Za-z0-9]+`로 넓힌다. whitespace, source/node identity, Grid track sizing 및 paint 경로는 그대로 유지한다. 범위는 [계획](../../../../plan/c11-root-html-intrinsic-ascii-alphanumeric-words.md)과 [Chrome 157 독립 사전 비교](../c11-root-html-intrinsic-ascii-alphanumeric-precomparison-2026-11-06/README.md)에 고정했다.

## Rust 검증

- `cargo fmt --all -- --check` — 통과.
- `cargo test --locked -p spinon-style-to-layout --lib c11_1_root_html_intrinsic_auto_rows_text_tests` — 11/11 통과.
- `cargo test --locked -p spinon-runtime --lib root_html_grid_intrinsic_auto_rows::` — 15/15 통과.
- 같은 RuntimeSession matrix에 `--features c11-4-grid-track-sizing-fixture-test` 적용 — 15/15 통과.
- `cargo clippy --locked -p spinon-style-to-layout --all-targets -- -D warnings` 및 `cargo clippy --locked -p spinon-runtime --all-targets -- -D warnings` — 통과.

두 Rust 행렬은 기존 ASCII letter scenes와 신규 10-scene alphanumeric matrix를 모두 실행한다. Style-to-layout은 independent track/row/line 기대값과 identity를 확인한다. RuntimeSession은 default Root HTML route와 source/revision/owner/track trace를 확인하며 fixture feature가 켜진 경우 diagnostics trace도 확인한다. 이전 reject control의 `A1`은 이제 positive input이므로 punctuation-bearing `A1-B`/`B-1`로 바꿔 계속 거부되는지 확인한다.

## Android 및 iOS Simulator

실행기: [`run-simulators.sh`](./run-simulators.sh), 결과: [`pass01`](./pass01/environment.txt). 후보 source HEAD는 `76e618e3b032f478ed933c21067a88bc25836afb`; source hash 목록과 목록 hash는 [manifest](./pass01/source-inputs.sha256)와 environment에 있다. Android는 API 37 ARM64 AVD `emulator-5554`, iOS는 iPhone 17 Pro/iOS 26.2 Simulator `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`를 사용했으며 Android 실기기는 사용하지 않았다.

앱 fixture에는 단일 text node `12345678`이 있다. 기본 Root HTML 앱에서 root app marker와 `layout=ready boxes=8`, `error=none`, 두 text paint의 draw/present를 확인했다. Android와 iOS 로그는 모두 동일 fixture SHA `abfb318793910b1e63371a31498276a299baccd62cc7b8c75b1a8cd9891386e3`을 기록한다.

Xcode build 원본 로그는 trailing whitespace를 보존한 무손실 gzip 파일 `pass01/ios/build.log.gz`로 둔다.

`pass01`은 commit `76e618e3`에서 실행했다. 이후 최종 Style-to-layout suite에 full-width digits, Arabic-Indic digits 및 비ASCII letter가 source gate를 통과하지 않는 test-only negative controls를 추가하고 11/11을 다시 통과했다. 그 후속 변경은 production predicate와 app fixture를 바꾸지 않았으며, `pass01/source-inputs.sha256`은 실행 당시 입력 hash로 보존한다.

| 플랫폼 | 화면·surface | 40px Grid 외부 표본 |
| --- | --- | --- |
| Android | [화면](./pass01/android/screenshot.png) `1080×2400`, surface `1080×2209`, density `2.625` | CSS `(42,19)` → `(111,51)`, `[91,168,245,255]`, PixelCopy 1/1 exact ([PNG](./pass01/android/pixelcopy.png)) |
| iOS | [화면](./pass01/ios/screenshot.png) `1206×2622`, Metal drawable `1206×2334`, scale `3` | CSS `(42,19)` → `(127,58)`, `[91,168,245,255]`, Metal 1/1 exact |

이 표본은 그 좌표의 overflow item 배경 paint를 증명한다. Android AVD는 실기기 GPU 검증이 아니며, iOS sample은 compositor 제출 전 Metal drawable이다. 모바일 로그는 전체 10-scene geometry matrix를 대신하지 않는다. 전체 geometry는 Rust matrix가 담당한다. 실행기는 임시 app asset overlay 전의 Android bootstrap/iOS app assets를 보존하고 실행 뒤 byte-for-byte 복원했다고 기록했다.

## 기본/fixture Rust artifact 경계

[경계 기록](./artifact-boundary.txt)은 release `spinon-runtime` rlib에서 scene id·sample source·exact quoted token marker가 없는지와 fixture-test positive-control executable에서 같은 marker가 있는지를 확인한다. Unquoted `12345678`는 일반 ASCII hexadecimal lookup string 안에 포함되므로 marker로 세지 않는다. 이 검사는 Rust crate artifact 기준이며 Android APK/iOS app bundle 전체를 다시 검사했다는 주장이 아니다. 이 slice는 플랫폼 adapter/FFI나 fixture payload를 추가하지 않았고, 기존 PR #1208의 fixture-only surface adapter는 수정하지 않았다.

## 범위와 미완료 항목

이 slice는 punctuation/Unicode, arbitrary fonts, 일반 CSS author admission, WPT, Android 실기기, iOS compositor 이후 pixel 및 전체 C11.4/C11 적합성을 검증하지 않는다. C11.4/C11 완료 checkbox와 공식 `78/214`는 그대로 미완료다. PR checks/CI 결과는 이 로컬 검증과 별개다.

구현/runtime의 별도 20관점 검토는 [implementation-runtime-review-20.md](./implementation-runtime-review-20.md)에 기록한다. PR 생성 뒤 실제 PR diff에 대한 30관점 검토를 별도로 남긴다.

증거 파일 hash 확인: `sha256sum -c spec/internal/evidence/c11-root-html-intrinsic-ascii-alphanumeric-runtime-2026-11-06/SHA256SUMS`.

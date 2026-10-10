# C07.2 테두리 폭 레이아웃 구현 및 시뮬레이터 실행

## 상태와 범위

- 작업 브랜치에서 구현·검증을 마쳤다. PR은 아직 만들지 않았으며 공식 병합 완료가 아니다.
- 내부 계약 숫자 버전과 모든 crate 버전은 `0.1.0`이다. `0044`는 명세 문서 ID다.
- 현재 runtime layout profile의 물리 네 면 border used width를 Stylo에서 `LayoutBorder`와 Taffy로 전달한다.
- border 선·색상 페인트, radius, border-image 페인트, Grid·텍스트 intrinsic sizing은 구현하지 않았다.

## Chromium 비교

- Oracle: Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`.
- 고정 HTML inventory 50개 node를 DPR 1·2에서 각각 관찰했다. 파일 digest와 capture provenance는 [사전 비교](./c07-2-border-width-layout-precomparison-2026-10-10.md)와 고정 JSON reference에 기록했다.
- 각 DPR에서 node별 `x/y/width/height`, 네 면의 used border width와 style을 대조했다. 각 geometry field는 최대 허용 오차 `0.5 CSS px`, width는 `0.01 CSS px` 이내이며 양 DPR의 CSS px 출력이 동일해야 한다. 전체 비교 테스트가 통과했다.
- `none`·`hidden`, 다른 표준 border style, shorthand와 longhand cascade, 기본 폭·`thin/medium/thick`, zero·fractional 폭, `calc()`·`var()`, 유효 선언 뒤 음수·percentage 무효 선언, content-/border-box, min/max, flex shrink를 fixture에서 확인했다.
- 색상만 바꾼 별도 계산은 used width DTO와 최종 frame이 그대로임을 확인했다. 현재 전역 `StyleRevision` 때문에 계산 자체는 다시 수행될 수 있으며 속성별 layout 생략은 약속하지 않는다.

## 실제 V8·WGPU 경로

두 앱은 동일한 `runtime-border-width.js` fixture를 실행하고 `status=0`, layout ready, 6 boxes를 보고했다. 아래 로그는 실행 직후 저장한 것이다.

| 플랫폼 | 환경 및 결과 | 근거 |
| --- | --- | --- |
| Android | API 37 ARM64 emulator, Android 17. `SPINON_C072_EVAL`에서 `layout=ready boxes=6`; WGPU는 GLES 경로의 ANGLE/SwiftShader software backend에서 6 boxes를 제출했다. | [Logcat](./c07-2-border-width-layout-android-emulator-2026-10-10.log) · [화면](./c07-2-border-width-layout-android-emulator-2026-10-10.png) |
| iOS | iPhone 17 Pro / iOS 26.2 Simulator. `SPINON_C072_SUMMARY`에서 `status=0 layout=ready boxes=6`; UIKit WGPU surface가 6 boxes를 제출했다. | [실행 로그](./c07-2-border-width-layout-ios-simulator-2026-10-10.log) · [화면](./c07-2-border-width-layout-ios-simulator-2026-10-10.png) |

화면의 색상 사각형은 layout geometry 시각화다. 테두리 선은 그리지 않으므로 border paint가 구현됐다는 증거가 아니다. 시뮬레이터 런타임 로그는 개별 자식 frame을 Chromium과 수치 대조하지 않는다. 수치 비교는 고정 50-node fixture의 Rust·Stylo·Taffy 비교에서만 주장한다. 실기기와 hardware GPU는 이번 검증에 포함하지 않았다.

iOS의 전체 C ABI 보고 문자열은 OSLog 길이 제한으로 잘릴 수 있어 짧은 `SPINON_C072_SUMMARY`를 추가했다. 전체 처리량 수치를 추정하지 않고 summary, 별도 environment layout 로그, draw 제출 로그를 보존했다.

## 실행한 검사

- `mise exec -- cargo fmt --all -- --check`
- `mise exec -- cargo test --locked --workspace --all-features --quiet` — 404 passed, 2 ignored.
- `mise exec -- cargo check --locked --workspace --all-features`
- `mise exec -- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `mise exec -- bun run test:css-reference` — 67 passed.
- `SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:android` 및 API 37 emulator 설치·실행.
- `SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:ios-sim` 및 iOS 26.2 Simulator 설치·실행.

## 화면·로그 digest

| 파일 | SHA-256 |
| --- | --- |
| Android 화면 | `0ab2536b92fb5d54379de1f7ddfc47e2372f43a2d1c430d0accfdf39e73b5f7d` |
| iOS 화면 | `8f3d6699cbecc3d39cde8bab8e5ce235f6977098e008421d50e7180c9691e4f5` |
| Android Logcat | `f4ae23b492373bb680355ac438567ddfb9cbb414fec432b7ffa7e949db6ca27b` |
| iOS 실행 로그 | `634da68270cd8fcc0c87c2488c48bfe1ef942306c425b7695ed51c47bc34e28a` |

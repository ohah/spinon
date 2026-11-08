# C11.4 fit-content/min-content/max-content/auto implicit-row list 구현 근거

## 범위

이 후보는 기존 parsed Root HTML `RootHtmlFitContentAutoRowsV1`에서 `fit-content(<computed finite non-negative px|%>)`, `min-content`, `max-content`, `auto`가 각각 정확히 한 번 있는 네-member 목록 24개 순서를 허용한다. 40×100px Grid, 40px column, 10px explicit row, row-sparse placement, visible overflow, zero edge/gap 및 다섯 직접 item/payload 쌍만 다룬다. public API/FFI는 바꾸지 않고 숫자 계약 `0.1.0`과 C11.4/C11 미완료·공식 `78/214`를 유지한다.

Chrome 157 사전 관측과 독립 기대값은 [96/96 precomparison](../c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/README.md)에 있다. RuntimeSession은 absolute `calc(12px + 8px)`가 cascade에서 `20px`으로 정규화되는 것도 확인했다. 이 computed value는 typed `FitContent(LengthPx(20))`, serialization `fit-content(20px)`로 전달된다. unresolved percentage-mixed calc, indefinite basis, 잘못된 member/child/payload profile은 계속 거부한다. 이 동작 차이와 계획 수정은 [계획 수정 검토 20개](../../../../plan/c11-4-root-html-fit-min-max-content-auto-rows-cap-normalization-review-20.md)에 설명한다.

고정 Root HTML fixture는 [`root-html-grid-fit-min-max-content-auto-implicit-row-list-runtime.html`](../../../../tests/fixtures/css/c11/root-html-grid-fit-min-max-content-auto-implicit-row-list-runtime.html), SHA-256 `abe726bd10f5f7d1f2cbb49a9fa762df7a70fa41078d066e430fb7a78852deb9`다. 기본 화면은 payload heights `[60,10,12,6,60]px`로 cap overflow를 보인다. 96-case 순서 행렬은 `baseline_source()`에서 첫째·다섯째 payload를 각각 20px로 정규화해 기존 독립 기대값과 분리한다. 그 matrix에서 explicit + implicit used rows의 `start` 결과는 `[10,20,10,12,6,20]px`, `stretch`에서는 auto track만 22px free space를 받는다. 추가 양성 overflow 검사는 24개 순서 각각에서 fit-content가 네 순환 위치를 차지하도록 payload를 60px로 만들고, `20px`·`20%` cap으로 Layout/RuntimeSession 각 48 scene을 확인한다. 해당 row는 모두 20px로 유지되고 payload는 item 밖으로 이어진다. 첫 위치에서는 다섯째 row가 y=58px, h=20px이고 payload y=58..118px이어서 Grid 아래 18px까지 보인다. 네 번째 위치도 Grid 아래로 이어지며, 가운데 두 위치는 item overflow를 보이되 Grid bottom을 넘지 않는다.

## Rust 검증

첨부한 text log의 명령 출력과 실패 본문은 보존했고 파일 끝의 빈 줄·줄 끝 공백만 정규화했다.

| 명령 | 결과 |
|---|---:|
| `cargo test --locked -p spinon-layout --lib root_html_fit_content_auto_rows` | 12 passed |
| `cargo test --locked -p spinon-layout --lib four_member_intrinsic` | 4 passed; 48 px/% cap-overflow scenes and 48 zero/over-100% boundary scenes |
| `cargo test --locked -p spinon-style-to-layout --lib root_html_fit_content_min_max_content_auto` | 2 passed |
| `cargo test --locked -p spinon-style-to-layout --lib fit_content_four_member_intrinsic` | 2 passed; 24 orders × 20px/20%/0%/120% typed caps, plus rejection controls |
| `cargo test --locked -p spinon-style-to-layout --lib root_html_fit_content_min_content_auto` | 2 legacy-order regressions passed |
| `cargo test --locked -p spinon-runtime --lib four_member` | 7 passed; 96-scene matrix, 48 cap-overflow positives, 0%/120% runtime boundaries, rejection, calc normalization, and same-session recovery |
| `cargo test --locked -p spinon-runtime --lib absolute_calc_cap` | 1 passed |
| `cargo test --locked -p spinon-runtime --lib parsed_root_html_fit_content_min_content_auto` | 4 legacy-order/overflow/rejection/recovery regressions passed |
| changed-crate Clippy (`spinon-layout`, `spinon-style-to-layout`, `spinon-runtime`, all targets, `-D warnings`) | passed |
| `cargo fmt --all -- --check` · `git diff --check` | passed |

최신 overflow 후보 재검사 결과는 [`overflow-layout-rerun-pass01.log`](./overflow-layout-rerun-pass01.log), [`overflow-style-rerun-pass01.log`](./overflow-style-rerun-pass01.log), [`overflow-style-legacy-rerun-pass01.log`](./overflow-style-legacy-rerun-pass01.log), [`overflow-runtime-all-rerun-pass02.log`](./overflow-runtime-all-rerun-pass02.log), [`cycle-overflow-layout-rerun-pass02.log`](./cycle-overflow-layout-rerun-pass02.log), [`cycle-overflow-runtime-rerun-pass02.log`](./cycle-overflow-runtime-rerun-pass02.log), [`overflow-calc-rerun-pass01.log`](./overflow-calc-rerun-pass01.log), [`overflow-runtime-legacy-rerun-pass01.log`](./overflow-runtime-legacy-rerun-pass01.log), [`overflow-clippy-pass01.log`](./overflow-clippy-pass01.log), [`overflow-rustfmt-pass01.log`](./overflow-rustfmt-pass01.log), [`overflow-diff-check-pass01.log`](./overflow-diff-check-pass01.log)에 둔다. 독립 구현/runtime 실패 관점 20개는 [검토표](./implementation-runtime-review-20.md)에 있다. 이 검토에서 잘못 귀속되던 unsupported `align-content` 오류를 수정했고, 비유한 cap·추가 direct item·overflow payload가 fit-content가 아닌 순환 위치에 놓이는 경우를 거부한다. 모든 순환 위치의 positive cases는 각 20px/20% cap에서 24순서 전체를 돌려 track cap과 payload overflow를 확인한다.

추가 순환 위치 검증의 최종 잠금 재실행은 [`cycle-overflow-layout-rerun-pass03.log`](./cycle-overflow-layout-rerun-pass03.log), [`cycle-overflow-style-rerun-pass02.log`](./cycle-overflow-style-rerun-pass02.log), [`cycle-overflow-runtime-rerun-pass04.log`](./cycle-overflow-runtime-rerun-pass04.log), [`cycle-overflow-runtime-legacy-rerun-pass02.log`](./cycle-overflow-runtime-legacy-rerun-pass02.log), [`cycle-overflow-style-legacy-rerun-pass02.log`](./cycle-overflow-style-legacy-rerun-pass02.log)에 있다. 최종 Clippy는 [`cycle-overflow-clippy-rerun-pass02.log`](./cycle-overflow-clippy-rerun-pass02.log), formatter와 whitespace 검사는 [`cycle-overflow-rustfmt-rerun-pass04.log`](./cycle-overflow-rustfmt-rerun-pass04.log) 및 [`cycle-overflow-diff-check-rerun-pass02.log`](./cycle-overflow-diff-check-rerun-pass02.log)에 있다. Clippy pass01이 새 test helper의 동일 분기를 발견해 조건을 결합한 뒤 pass02를 다시 실행했다. Formatter pass02가 잡은 줄바꿈 차이도 수정했고 pass04가 통과했다.

추가 cap 경계 검증은 [`cap-boundary-layout-rerun-pass01.log`](./cap-boundary-layout-rerun-pass01.log), [`cap-boundary-style-rerun-pass01.log`](./cap-boundary-style-rerun-pass01.log), [`cap-boundary-runtime-rerun-pass02.log`](./cap-boundary-runtime-rerun-pass02.log)에 기록했다. RuntimeSession의 첫 시도 [`cap-boundary-runtime-rerun-pass01.log`](./cap-boundary-runtime-rerun-pass01.log)는 zero-height item을 paint snapshot에서 찾으려다 실패했다. 기존 계약대로 zero-height box는 layout frame에서 확인하고 payload paint는 render snapshot에서 확인하도록 고쳐 최종 실행을 통과시켰다.

cap-boundary 수정 뒤 changed-crate Clippy (`-D warnings`), `cargo fmt --all -- --check`, `git diff --check`는 각각 [`cap-boundary-clippy-rerun-pass01.log`](./cap-boundary-clippy-rerun-pass01.log), [`cap-boundary-rustfmt-rerun-pass01.log`](./cap-boundary-rustfmt-rerun-pass01.log), [`cap-boundary-diff-check-rerun-pass01.log`](./cap-boundary-diff-check-rerun-pass01.log)에 있고 모두 통과했다.

PR 최종 변경을 대상으로 한 별도 [diff 실패 관점 30개 검토](./pr-diff-review-30.md)는 기존 구현/runtime 검토와 구분해 기록했다.

## Simulator 화면 source 범위

아래 초기 Android/iOS 화면은 fixture SHA-256 `9404483c478c1ad4414488f7bad45bdf378d01376930ca6d0fe13ea7e2a3d31e`를 사용한 기본 화면 capture다. 이어지는 overflow pass는 현재 fixture SHA-256 `abe726bd10f5f7d1f2cbb49a9fa762df7a70fa41078d066e430fb7a78852deb9`를 양쪽 simulator에서 설치·실행한 capture이며 PR 본문에 표시할 화면도 그 결과를 사용한다. 둘 다 OS screenshot visual smoke이며 native surface readback은 아니다.

## Android API 37 ARM64 AVD

전용 `emulator-5554`에서 fixture HTML hash `9404483c…a3d31e`와 일치하는 기본 parsed Root HTML 경로를 실행했다. current-process log는 `html_bytes=2375`, `layout=ready boxes=14`, `page_scale=1`, `SPINON_C0410_DRAW status=0 presented boxes=14`를 기록한다. APK hash 검증은 설치 APK와 빌드 산출물을 대조했다. OS screenshot은 [`screenshot-final.png`](./android/final/screenshot-final.png)이며 SHA-256 `3f3d06724edc5b2036e197da7703d5b8fd60f3d60ca41f966c11f445a3bf3a45`다. [current PID log](./android/final/app-current-process.log), [device metadata](./android/final/device.txt)와 [build log](./android/build.log)를 보존한다.

Screenshot에는 explicit row, 네 intrinsic member 색, 늘어난 auto row와 다섯 번째 payload가 보인다. 이는 Simulator AVD의 compositor screenshot visual check이며 PixelCopy 표본이나 hardware GPU 결과가 아니다. baseline item CSS frame은 RuntimeSession 96-scene matrix에서, fit-content cap overflow는 48-scene 순환 위치 matrix에서 비교했다.

두 앱 화면은 unsupported `align-content` 진단 귀속을 고치기 전 build에서 캡처했다. 수정은 거부 경로만 바꾸며 위의 지원 fixture와 계산 경로는 바꾸지 않았다. 최종 source의 positive matrix와 alignment rejection은 RuntimeSession/Layout tests로 다시 검증했다. iOS 컨테이너 연속성 사고 뒤에는 앱을 재설치하지 않았다.

## iOS Simulator

전용 iPhone 17 Pro, iOS 26.2 Simulator (`97E99AA8-D144-4DA8-B1BF-7F6B34007F8B`)에서도 같은 fixture hash·2,375 bytes가 설치되어 기본 Root HTML 경로가 `layout=ready boxes=14`, `page_scale=1`에 도달했다. OS screenshot은 [`screenshot.png`](./ios/final/screenshot.png), SHA-256 `1ef3a4be105395d47fcd0e9da36b8e2fce4f6cefdb9829ca84e0c8519a9cd010`이다. [current PID log](./ios/final/app-current-process.log)와 [device metadata](./ios/final/device-metadata.json)를 보존한다. iOS 로그에 draw-presented marker는 없으므로 layout-ready와 screenshot-visible output까지만 주장한다. Metal drawable readback이나 compositor-after pixel parity는 검사하지 않았다.

`simctl install` 전후 app-data container ID가 바뀌었다. 마지막 설치 전에 있던 `E867C833-4759-4B8F-A937-79954760D58A`와 현재 `EA87A678-9FC7-4838-8DEA-8D8222DEE78E`의 tar snapshot은 로컬 evidence directory에 보존했다. 이 실행 이전의 `A710A909-02B3-47AD-A591-D487A4A75BFE` 경로는 사라졌고 그 당시에는 백업하지 못했다. 앱 데이터 clear·uninstall·Simulator erase를 실행하지 않았지만 이전 container의 연속성은 확인할 수 없다. 원인은 특정되지 않았다. app-data tarball은 저장소/PR에 공개하지 않는다.

## Cap overflow Android/iOS Simulator pass01

새 fixture SHA-256 `abe726bd10f5f7d1f2cbb49a9fa762df7a70fa41078d066e430fb7a78852deb9`로 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 다시 빌드·설치했다. 두 앱 모두 `layout=ready boxes=14`에 도달했고 Android draw-presented 및 두 OS screenshot을 확인했다. [Android 화면](./overflow-screen-pass01/android/final/screenshot.png) · [iOS 화면](./overflow-screen-pass01/ios/final/screenshot.png) · [빌드·설치·캡처 hash](./overflow-screen-pass01/final-capture.sha256).

화면에는 Grid 내부 경계 아래로 이어지는 60px payload가 보인다. RuntimeSession frame assertion은 Grid bottom 100px, payload y=58px/h=60px, overflow 18px를 검사한다. 정확한 입력은 [source manifest](./overflow-screen-pass01/final-source.json), APK·app·screenshot hash는 [capture manifest](./overflow-screen-pass01/final-capture.sha256)에 있다. Simulator 캡처는 OS screenshot visual evidence이며 PixelCopy·Metal drawable readback이 아니다. iOS 설치 직전 container `EA87A678-9FC7-4838-8DEA-8D8222DEE78E`를 tar로 보존했으며 설치 뒤 `2D38305E-1A41-43A1-AE80-E37B12A42D39`로 경로가 바뀌었다. 캐시 clear·앱 data clear·uninstall·Simulator erase는 하지 않았다. 두 container snapshot은 로컬 전용이며 저장소/PR에 공개하지 않는다.

## 재현 스크립트와 한계

- [`build-simulators.sh`](./build-simulators.sh)는 bootstrap source를 기준 hash로 검사하고 fixture HTML을 빌드한 뒤 원본 source를 복원한다. `c04-runtime-gpu-fixture-test` opt-in은 꺼져 있다.
- [`install-capture-simulators.sh`](./install-capture-simulators.sh)는 두 시뮬레이터에 build를 설치하고 screenshot/log를 수집한다. iOS 설치 전에 active app-data container snapshot을 만든다.
- [`capture-android-current-process.sh`](./capture-android-current-process.sh)와 [`extract-ios-current-process.sh`](./extract-ios-current-process.sh)는 현재 앱 PID 로그를 따로 추출한다.
- fixture, APK, iOS executable, screenshot의 hash는 [`final-capture.sha256`](./final-capture.sha256)에 있다.

이 후보는 arbitrary HTML/CSS Grid grammar, content-varying min/max contribution parity, spanning, WPT 적합성, 실기기/hardware GPU, PixelCopy/Metal surface pixels, iOS compositor 후 pixels 또는 C11.4/C11 완료를 증명하지 않는다. PR/CI 상태도 이 근거에 포함되지 않는다.

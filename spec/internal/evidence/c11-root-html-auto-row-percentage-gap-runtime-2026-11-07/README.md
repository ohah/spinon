# C11.4 parsed Root HTML auto-row percentage `row-gap` runtime 근거

## 범위와 기준

- 제한 profile: `RootHtmlAutoRowStretchV1`, 40×40px definite Grid content box, 40px explicit column, 10px explicit row, two implicit `auto` rows, three direct items (fixed empty item, direct text `A`, direct text `B`), `column-gap:0`, `align-items:normal`, existing ten-value `align-content` allowlist.
- 규범은 [CSS Gaps Level 1 §2.1](https://www.w3.org/TR/css-gaps-1/#row-and-column-gutters-the-row-gap-and-column-gap-properties) (`normal`은 Grid에서 used zero) 및 [§2.3](https://www.w3.org/TR/css-gaps-1/#percentages-in-gap-properties) (percentage는 해당 축의 content-box 크기 기준), [CSS Grid Level 1 §10.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters)다.
- [사전 비교](../c11-root-html-auto-row-percentage-gap-precomparison-2026-11-07/README.md)는 Chrome 157 Agent Browser raw observation과 독립 수식 기대값을 분리해 70/70 scene, 1/32 CSS px 허용차로 고정했다. 예상 row gap은 0·2.5·5·7.5·8·40·48px다. 120%/center 마지막 item top은 Grid top보다 73px 아래다.
- 기능 코드는 parsed computed percentage를 `LayoutLengthPercentage::Percentage`로 보존한다. Fixed px `0..=7.5px`, `row-gap:normal` used zero, column gap 0, item/tree shape 및 다른 Grid profiles는 유지한다. 40px basis에서 두 gutters와 30px base row extent가 finite f32를 초과하는 값은 `row-gap` node/property diagnostic으로 거부한다.
- HTML authored `row-gap:-5%`는 현재 Spinon cascade에서 `OtherInvalidValue` fatal diagnostic으로 끝난다. HTML CSS declaration을 무시하고 computed initial로 회복하는 브라우저 동작과 차이가 있으며, 이 parser 정책 변경은 이 bounded layout slice에 포함하지 않는다. Valid `calc(5px)`도 이 profile의 Layout 경계에서 `row-gap` 오류로 거부하며 실패 layout을 publish하지 않는다.

## 검증 결과

- Layout: `root_html_auto_row_stretch` focused filter 6/6 통과. 기존 fixed-px·overflow 행렬, 새 70 percentage × alignment Layout geometry/used-track matrix, finite-f32 gap 경계와 직접 typed Layout의 기준 우회 거부를 포함한다. 기존 `root_html_two_implicit_auto_rows_respect_supported_content_alignment` 및 `root_html_intrinsic_track_sizing_stretches_an_automatic_implicit_row` 회귀도 각 1/1 통과했다.
- Style projection: `row_gap_admits_finite_px_values_and_representable_percentages` 1/1 통과. Fixed px·percentage·`normal`, negative/non-finite/unsupported value와 마지막 finite-f32 경계를 검사한다.
- RuntimeSession: `row_gap` focused filter 4/4, `root_html_grid_intrinsic_auto_rows` 회귀 묶음 28/28 통과. 기본 `RuntimeSession::new_runtime_gpu_with_root_html()`에서 70/70 item frame·alignment, explicit `normal`, authored negative cascade diagnostic, unsupported `calc()`의 원자적 실패 후 recovery를 검사한다. 확장 전용 filter와 별도로 회귀 묶음도 실행해 이전의 "percentage row-gap 계속 거부" 기대가 새 제한 profile에 남아 있던 것을 찾아, percentage `column-gap` 거부로 교체했다.
- Android/iOS screenshots는 기본 Root HTML app route에서 실행했다. C11 fixture-test flag는 모두 껐다. 최종 제품 코드 해시 `252e392adac9a5fb392c429be626b1b7b3cf0393d55db7b8d901a7d63e45d2fa`로 다시 실행한 `pass03`은 0%/normal positive-space case, `pass04`는 120%/center overflow case다. 두 pass 모두 app boot, `layout=ready boxes=6`, draw/present와 두 text records를 확인했다. 전체 실행 로그·기기·hash·원래 앱 복원은 각 pass 폴더에 있다.

## Android API 37 ARM64 AVD / iPhone 17 Pro iOS 26.2 Simulator

- [최종 pass03 Android PNG](./pass03/android/simulator.png)와 [iOS PNG](./pass03/ios/simulator.png)는 0%/normal positive-space scene이다. 두 앱은 같은 source fixture SHA-256 `1fc1c874bbd2cb5cff6fa32af98a06abcbb5de8097d628a8c2ef4edda3227248`을 번들에 포함했다. 화면은 visual smoke이며 정확한 geometry는 Layout/RuntimeSession matrix가 판정한다.
- [최종 pass04 Android PNG](./pass04/android/simulator.png)와 [iOS PNG](./pass04/ios/simulator.png)는 120%/center scene이다. fixture SHA-256은 `5283624e7d78f9145398c12f8be7fea4e083ef0a6b24f624c1054aebfbe1c0a2`; APK와 iOS app asset hash가 모두 동일하다.
- [최종 pass04 PNG pixel analysis](./pass04/screenshot-pixel-analysis.json)는 정확한 opaque Grid·item RGB 픽셀의 bounds를 비교했다. Android scale 2.625에서 Grid background top y=128, 예상 bottom y=233, 마지막 item top y=320 (73.14 CSS px offset)다. iOS scale 3에서 top y=186, bottom y=306, 마지막 item top y=405 (73 CSS px offset)다. 양쪽 PNG에서 마지막 주황 item이 Grid bottom 아래에 실제 표시된다.
- 이는 OS Simulator screenshot의 post-compositor PNG pixel 관찰이다. Android PixelCopy, iOS Metal drawable readback, iOS compositor 이후 GPU surface parity, 실기기·hardware GPU 증거로 확대하지 않는다.
- `pass03/environment.txt`와 `pass04/environment.txt`에서 이전 Android installed APK, iOS installed executable, build APK와 iOS app SHA가 실행 전 값과 복원 후 값으로 일치하며 `asset_and_simulator_restoration=PASS`다. 두 최종 pass는 같은 제품 코드 diff SHA를 기록한다. bootstrap HTML·JS Git diff는 비어 있다. Agent Browser는 simulator 실행에 사용하지 않았고 실행 뒤 `pgrep -fl '^agent-browser'`도 비어 있었다. 기존 다른 iOS simulator process는 종료하지 않았다.
- `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `git diff --check`가 통과했다. Clippy 명령은 세 변경 crate의 all-target gate이며 workspace Clippy 전체 실행으로 확대 해석하지 않는다.
- 별도 [구현/runtime 실패 관점 검토 20개](./implementation-runtime-review-20.md)는 이전 stale rejection assertion을 수정한 뒤 전체 RuntimeSession 회귀 28/28과 범위·oracle·overflow evidence를 재확인한다. PR 실제 diff review는 별도로 수행한다.
- Rust/runtime 입력 fixture와 simulator harness의 현재 SHA-256 목록은 [`source-inputs.sha256`](./source-inputs.sha256), focused test·Clippy·format·diff-check 원본 출력은 [`verification/focused-validation.log`](./verification/focused-validation.log)에 있다.
- `xcodebuild` export와 Android logcat 출력의 줄 끝 공백은 Git whitespace 검사에서 확인됐다. 사람이 읽는 `.log` 파일에서는 줄 끝 공백과 빈 EOF 줄만 정리했고, 정리 전 원본 bytes의 SHA-256은 [`raw-capture-sha256-before-whitespace-normalization.txt`](./raw-capture-sha256-before-whitespace-normalization.txt)에 보존했다. 로그 message·event 값은 수정하지 않았다.

## 남은 한계

이 후보는 제한된 Root HTML profile slice이며 일반 CSS Grid admission, C11.4 또는 C11 완료가 아니다. WPT 적합성, 임의 percentage row-gap, indefinite/cyclic basis, `column-gap` percentage, CSS math, PixelCopy/Metal surface readback 및 실제 표시 시점은 확인하지 않았다. 공식 완료 수 `78/214`와 C11/C11.4 unchecked 상태를 유지한다.

# C11.4 repeated-text row-span six-fit `min-height:auto`

**상태:** 제한된 구현과 RuntimeSession 검증 완료 · 구현 PR [#1365](https://github.com/ohah/spinon/pull/1365) · 공식 `78/214`, C11.4/C11 미완료 유지

## 범위

`RootHtmlFitContentAutoRowsV1`의 고정 repeated-text row-span profile에만 six-fit track list를 추가했다. 여섯 `grid-auto-rows` member는 정확히 `fit-content()`이고 typed cap multiset은 `[0px, 10px, 20%, 30px, 40px, 50px]`이다. 720 authored order를 지원한다. 여섯 positive implicit row에 span text item이 배치되고, definite `40×220px` Grid, `40px` explicit column, `10px` explicit control row, zero gap, LTR sparse row flow, visible overflow, `align-content:start|stretch`와 고정 두 topology만 허용한다.

기존 five-fit row-span의 phase-by-phase 1/64px sizing replay를 공통 span simulator로 일반화했다. six-fit span을 replay할 수 없거나 trace와 대조가 안 되면 일반 Taffy row 결과로 대체하지 않고 fail-closed한다. 기존 five-fit fallback은 유지한다.

## 검증 결과

- Chrome 157 oracle: 720 cap orders × 16 scenes = **11,520 scenes**, 즉 5,760 baseline/candidate pairs. Candidate payload-floor violation **0**; baseline violation **4,800**. 모든 5,760 pair에서 geometry 차이를 확인했다. Forward/reverse DOM order와 `start/stretch` 결과의 row, item frame, payload frame은 일치했다. 최종 실행 로그: [`runtime-matrix-final.log`](runtime-matrix-final.log).
- Matrix metadata 11,520 scenes / 720 orders: 1/1 통과. Style six-fit profile 2/2, layout profile 2/2, RuntimeSession rejection 1/1, same-session rejection/recovery 1/1, typed percentage cap trace 2/2, five-fit non-matrix regression 9/9 통과.
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `git diff --check` 통과. Workspace Clippy 로그는 [`clippy.log`](clippy.log).
- Android APK와 iOS Simulator 앱을 최종 소스에서 다시 빌드했다. SHA-256은 각각 [`Android`](android/candidate-apk-sha256.txt), [`iOS`](ios/candidate-executable-sha256.txt)이며 앞서 찍은 플랫폼 화면을 만든 산출물과 일치한다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 `index.html` route의 light/dark 화면, layout-ready, draw/present를 확인했다. 자세한 범위·화면은 [`platform-smoke.md`](platform-smoke.md).
- `cargo test -p spinon-layout --lib` 전체는 **544 passed / 1 failed**다. 실패는 변경하지 않은 C15 `host_document_collects_direct_text_and_inherited_spans_in_source_order` assertion이다. 이번 작업에서는 baseline tree에서 이 실패를 재실행하지 않았으므로 회귀 여부를 별도 검증하지 않았다. 로그: [`layout-lib-full.log`](layout-lib-full.log).

## 독립 리뷰

- 구현·런타임 실패 관점 20개: [`implementation-runtime-review-20.md`](implementation-runtime-review-20.md)
- 구현 PR diff 실패 관점 30개: [`pr-diff-review-30.md`](pr-diff-review-30.md)

## 남은 범위

이 결과는 arbitrary six-fit list, 임의 Grid topology, WPT, 공개 API/FFI, 일반 CSS Grid 지원, device-side six-fit fixture geometry, PixelCopy/Metal drawable readback, compositor parity, 실기기 또는 hardware GPU를 주장하지 않는다. 플랫폼 screenshot은 기본 app route이며 six-fit fixture를 실행하지 않는다. 이 제한 profile만으로 공식 완료 수를 늘리지 않았다.

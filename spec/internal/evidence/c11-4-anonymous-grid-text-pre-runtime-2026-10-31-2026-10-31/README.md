# C11.4 anonymous Grid text `white-space:pre` runtime 근거

## 판정 범위

이 기록은 C11.4 anonymous Grid text에 `white-space:pre` 의미를 넣은 제한 fixture candidate를 다룬다. CSS Text/Grid의 고정 기대값은 [작업 계획](../../../../plan/c11-4-anonymous-grid-text-pre.md)과 [Chrome 156 사전 비교](../c11-4-anonymous-text-pre-precomparison-2026-10-31/README.md)에 고정했다. 계획 자체의 실패 관점 20개와 별도 구현/runtime 실패 관점 [20개](implementation-runtime-review-20.md)를 남겼다.

구현 PR [#880](https://github.com/ohah/spinon/pull/880)은 2026-10-30T23:06:20Z Rebase and merge됐고 merge SHA는 `c592212bbf858cda0f310bb5885e864687960d06`다. PR check rollup은 비어 있었고 main required status checks도 설정되지 않았다.

신규 장면은 `pre-max-content-spaces`, `pre-min-content-spaces`, `pre-forced-break`, `normal-fixed-width-control`, `pre-fixed-width-nowrap` 다섯 개다. 기존 anonymous text matrix 26개와 합친 31개 장면을 양 플랫폼에서 실행했다. 이 구현은 고정 Ahem ASCII source/style profile와 opt-in fixture allowlist에 한정되며 일반 CSS author admission이나 public CSS 지원이 아니다.

## 최종 source와 simulator 실행

최종 비교 실행은 같은 working-source snapshot에서 이뤄졌다. Base commit은 `7cbebe5532cb0176cb4330ce2ea8e23217e7687a`, tracked diff SHA-256은 `27db7e4ffc1e9c73bc820895d6452beb49e3db26ca28166fda39288d4a1ca579`, source snapshot SHA-256은 `f66f1ae34a61847e643065181b54449c74b5ec486f74877d7b83b422901d2d02`다.

| Target | 기기/OS | 실행 | 결과 |
| --- | --- | --- | --- |
| Android | API 37 ARM64 AVD, `emulator-5554`, ANGLE/Vulkan/SwiftShader | [실행 log](android-matrix-status-sync.log) · [결과 JSON](android-avd-c114-anonymous-grid-text-android-avd-20261030T225732284Z-090ad9bf/result.json) · [simulator receipt](android-avd-c114-anonymous-grid-text-android-avd-20261030T225732284Z-090ad9bf/simulator.json) | 31/31 scene, draw/present 31/31, retry 0 |
| iOS | iPhone 17 Pro, iOS 26.2 Simulator | [실행 log](ios-matrix-status-sync.log) · [결과 JSON](ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T225830613Z-41a55d90/result.json) · [simulator receipt](ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T225830613Z-41a55d90/simulator.json) | 31/31 scene, draw/present 31/31, retry 0 |

신규 장면의 검증 결과는 다음과 같다.

| Scene | Root frame | Columns / Rows | Trace chunks (Android / iOS) |
| --- | --- | --- | ---: |
| `anonymous-pre-max-content-spaces` | 20×10 | 60 / 10 | 7 / 13 |
| `anonymous-pre-min-content-spaces` | 20×10 | 50 / 10 | 7 / 13 |
| `anonymous-pre-forced-break` | 20×20 | 20 / 20 | 7 / 13 |
| `anonymous-normal-fixed-width-control` | 25×20 | 25 / 20 | 5 / 10 |
| `anonymous-pre-fixed-width-nowrap` | 25×10 | 25 / 10 | 5 / 10 |

Fixed-width `pre` 장면에서 Grid item frame은 root 폭 25px이고 line advance는 50px다. Android와 iOS 모두 installed app hash가 local build와 일치한다. Android/iOS build 결과는 각각 [Android log](android-build-final.log), [iOS log](ios-build-final.log)에 있다. Android는 SwiftShader software renderer다.

첫 최종-source AVD 시도는 첫 scene 뒤 새 activity가 Splash Screen에 머물러 두 번째 scene의 draw/present를 내지 않았다. Layout eval과 7 trace chunk는 있었고 SurfaceFlinger dump도 timeout됐다. 이 시도는 [실패 log](android-matrix-final.log)와 [실패 run evidence](android-avd-c114-anonymous-grid-text-android-avd-20261030T225036857Z-32a9ade2/)에 보존했다. 프로젝트 AVD를 cold boot한 뒤 전체 31 scene을 다시 돌려 통과했고, 같은 tracked source hash로 양 플랫폼 행렬을 끝냈다. 이 실패 run의 원인은 AVD SurfaceFlinger/renderer startup 상태로 관찰됐으며, layout 계산 오류로 분류하지 않았다.

## 자동 검사

- Text layout: `cargo test --locked -p spinon-text-layout --lib` — 59/59.
- Anonymous Grid HostDocument: `cargo test --locked -p spinon-layout --features c11-4-grid-track-sizing-fixture-test c11_4_anonymous_text:: -- --nocapture` — 31/31.
- Root HTML `preserve + nowrap` mapping, anonymous scene uniqueness, text/track mutation guard — 각각 1/1; FFI 31-scene source matrix — 1/1.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs` — 24/24; `cargo fmt --all -- --check` — 통과; `cargo clippy --locked --workspace --all-targets -- -D warnings` — 통과.
- `cargo test --locked --workspace` — 262 passed, 1 failed. 실패는 변경 밖 C07.3 computed aspect-ratio 고정 문자열 비교로, Spinon `1.7777778 / 1`, pinned Chromium 기대 `1.77778 / 1`이다. 전체 테스트는 통과로 보고하지 않는다. [전체 실행 log](workspace-test.log).

Focused test의 세부 output은 [focused tests](focused-tests.log), [fixture boundary](boundary.log), [rustfmt](rustfmt.log), [Clippy](clippy.log)에 둔다.

## Fixture archive 경계와 화면 한계

macOS ARM64 Release static archive 검사에서 기본 product는 신규 scene marker 0/5, C11.4 host export 0개였다. fixture-test positive control은 marker 5/5, host export 2개였다. [Archive 검사 근거](artifact-boundary/README.md).

앱 runtime은 draw/present와 layout trace를 확인하지만 현재 path에서 C15.3 glyph paint는 켜져 있지 않다(`runtime_text_count=0`, `glyph_batch_draw_count=0`). 보존한 PNG는 simulator/app 상태 증거이며 glyph 잉크 픽셀의 정확성을 보여주지 않는다. iOS는 compositor 제출 이후 화면을 검사하지 않았다. 실기기, hardware GPU, 전체 WPT, arbitrary font/script/tab, 일반 CSS admission 및 전체 C11.4/C11은 이 검증에 포함하지 않았다.

공식 상태 대장은 C11과 C11.4를 미완료로 유지하며 전체 완료 `72/213`은 변하지 않았다. 이 좁은 slice는 C11 진행 비율로 환산할 수 없다.

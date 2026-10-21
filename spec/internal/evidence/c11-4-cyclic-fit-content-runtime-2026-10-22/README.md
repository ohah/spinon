# C11.4 cyclic percentage `fit-content()` runtime 근거

2026-10-22에 cyclic column·row 및 definite column control 세 장면을 기존 C11.4 opt-in fixture 경로에 연결했다. 구현은 일반 CSS admission이나 새로운 Grid sizing 알고리즘을 추가하지 않는다. 기대값의 독립 계산과 Chrome 156 관측은 [계획](../../../../plan/c11-4-cyclic-percent-fit-content.md) 및 [사전 비교 근거](../c11-4-cyclic-fit-content-precomparison-2026-10-22/README.md)에 있다. 기준 규범은 CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1 및 §11.1이다.

## 고정 입력과 판정값

각 scene은 `div` grid root/item 2개와 Ahem direct text 하나, explicit column·row 각 한 개, margin/padding/border/gap 0이다. item은 `min-width:0;min-height:0`을 명시한다. 런타임 adapter는 세 ID만 허용한다.

| scene | Final Columns / Rows | Root frame | Item frame |
| --- | --- | --- | --- |
| `cyclic-fit-content-column` | `[25]` / `[20]` | `50×20` | `25×20` |
| `cyclic-fit-content-row` | `[20]` / `[20]` | `20×40` | `20×20` |
| `definite-fit-content-column` | `[20]` / `[20]` | `40×20` | `20×20` |

column scene은 Ahem `aa aa`의 min-content 20px과 max-content 50px 사이에서 50% limit 25px을 적용한다. row scene은 width 20px에서 Ahem `a a a a`가 만드는 40px intrinsic root height에 cyclic row percentage를 다시 풀어 20px row를 얻는다. definite control은 width 40px의 50%인 20px을 사용한다. 모든 크기와 두 축 trace는 두 simulator의 `result.json` 및 원본 runtime log에서 일치한다.

## 구현과 검증

- 고정 scene adapter: `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/cyclic_percentage_fit_content.rs`. dispatch와 scene allowlist는 기존 fixture-test feature에만 연결했다.
- Rust projection/layout: `FitContent(Percentage(0.5))` 보존 회귀, Ahem direct-text layout, 최종 Columns·Rows track trace와 root/item frame을 검사한다.
- Runtime/native harness: 각 scene의 source ID·tree·style·track·전체 node frame·draw 상태를 확인하고, 표면 경계에서 item/background 색을 고정 표본한다.
- Android API 37 ARM64 AVD `emulator-5558` (`sdk_gphone64_arm64`, `arm64-v8a`): 세 장면 3/3, draw/present 3/3, PixelCopy item/background 표본 6/6 exact. local 및 설치 APK SHA-256은 `28ce114845a6dc9ab3d5cc2f574cf7084cdaaefa62fc029a8e2ebfa1fdfe9f5c`로 같다. 원본은 [최종 Android 실행](./android-avd-c114-cyclic-fit-content-android-avd-20261021T211550973Z-507a47e9/)이다.
- iPhone 17 Pro / iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`: 세 장면 3/3, draw/present 3/3, Metal drawable 표본 6/6 exact. 표본은 OS compositor 제출 전이다. 앱 binary SHA-256 `e24f792eb0458dc99158a4def2c2c02af1997cfede9b97a5ea12f2a8c67efa32`. 원본은 [최종 iOS 실행](./ios-simulator-c114-cyclic-fit-content-ios-simulator-20261021T211557002Z-ad4d44ff/)이다.
- 대표 화면: [Android column](./android-avd-c114-cyclic-fit-content-android-avd-20261021T211550973Z-507a47e9/cyclic-fit-content-column.png) · [iOS column](./ios-simulator-c114-cyclic-fit-content-ios-simulator-20261021T211557002Z-ad4d44ff/cyclic-fit-content-column.png). 나머지 장면과 JSON/log는 각 실행 폴더에 보존했다.
- 두 최종 `simulator.json`은 source commit `9af5ee8fa29269c589a02d6f1aafe3f72d53d1d0`, tree `3f21957dbd4dfeca6c60dd0f4427cc6a5bce79e7`, 빈 tracked diff SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`, 빈 dirty-path 목록을 기록한다. Android/iOS artifact hash와 screenshot hash는 실행 폴더와 [SHA256SUMS](./SHA256SUMS.txt)에 있다.

### 검사 결과

- `cargo fmt --all -- --check`, Node syntax check, fixture module boundary test 6/6 통과.
- Rust focused tests: style/layout track suite 10/10, runtime fixed-text matrix 1/1, FFI C11.4 fixture suite 5/5 통과.
- feature별 Clippy: `spinon-style-to-layout`, `spinon-runtime`, `spinon-ffi` 모두 `--all-targets -- -D warnings` 통과.
- Android fixture APK build와 iOS Simulator fixture app build 통과. 보존 로그: `android-fixture-build-pass05.log`, `ios-fixture-build-pass04.log`.
- 기본 FFI release `.rlib`에서 신규 scene ID/module marker가 없고, `c04-runtime-gpu-fixture-test` positive control에 포함됨을 `strings`로 확인했다. hash·scan 결과와 제한은 [artifact-boundary README](./artifact-boundary/README.md)에 있다. 이는 `.rlib` 경계 검사이며 최종 APK/IPA release 구성에 대한 주장이 아니다.
- CSS runner inventory는 총 211 scene이며 세 신규 ID가 고유하다. capture command는 Android와 iOS 모두 정확히 세 ID만 실행했다.

초기 시도에서 Android runtime이 새 root ID를 미등록으로 거부했고, `align-items`의 computed `normal`과 기존 validator의 `start` 기대가 달랐으며, text fixture의 명시적 `min-height:0`도 adapter에 없었다. 이를 각각 fixed case registry, 신규 FitContentPercentage scene에 한정한 computed style 판정, min-height assertion으로 수정했다. 첫 표면 실행에서 row inside-item 색 ID도 기대 RGB map에 빠져 있어 추가했고, unit trace는 첫 sizing pass 대신 축의 마지막 pass를 보도록 고쳤다. 초기 로그는 같은 evidence root의 이전 run directory에 삭제 없이 보존했다. 모든 수정 후 commit `9af5ee8f`에서 두 simulator 행렬을 다시 실행했다.

일부 Android runtime 및 Xcode build log에는 도구 출력의 줄끝 공백이 있었다. 링크 가능한 `.log` 사본은 끝 공백과 마지막 빈 줄만 정규화했고, 원본 byte stream은 같은 이름의 `.log.raw.gz`에 보존했다.

## PR 본문 이미지 확인

PR [#474](https://github.com/ohah/spinon/pull/474)의 두 raw screenshot 링크를 Agent Browser `0.31.1`에서 확인했다. Android 이미지는 1080×2400, iOS 이미지는 1206×2622로 모두 `complete=true` 및 `naturalWidth > 0`이었다. [Agent Browser가 렌더링한 PR 화면](./pr-474-agent-browser.png)과 기계 판정 [JSON](./pr-474-browser-verification.json)을 남겼다. 전용 browser session을 닫았고 확인 직후 Agent Browser/Chrome process는 0개였다.

## 한계

이 결과는 세 개의 고정 opt-in fixture에 한정한다. Android 실기기와 hardware GPU, iOS compositor 이후 pixels, glyph paint/equality, border stroke, WPT, 다른 UA, 임의 HTML/CSS, `fit-content(calc(...))`, 여러 track/item, 같은 scene의 양 축 cyclic percentage, nested Grid 및 전체 C11.4는 확인하지 않았다. 일반 `fit-content()` 또는 CSS 지원 완료로 해석하지 않는다. C11.4 공식 checkbox는 미완료이며 공식 완료 수 `72/213`을 유지한다.

## PR 병합 기록

PR [#474](https://github.com/ohah/spinon/pull/474)은 2026-10-21 21:23:35 UTC Rebase and merge되어 merge SHA df0c607f83732081a47ceb8435f3a174289acee7를 남겼다. live 확인 당시 main 필수 status check 설정은 없었고 PR check rollup은 비어 있었다. C11.4 checkbox 및 공식 완료 수 72/213은 계속 유지한다.

병합 상태를 반영한 문서 diff 검토는 [post-merge docs review 30개](./post-merge-docs-review-30.md)에 기록했다.

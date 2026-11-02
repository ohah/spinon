# C11.1 Root HTML named-area percentage track 실행 근거 · 2026-11-02

## 판정 범위

이 후보는 기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 경로에서 기존 `header/header`·`side/content` 2×2 named-area map과 direct `px`/`%` track 두 개씩의 조합만 허용한다. 기존 fixed-px와 양축 `1fr 2fr` named-area 경로는 유지한다. `%`와 `fr` 혼합, 다른 map·track 문법, implicit placement, percentage gap은 지원하지 않는다. C11.1/C11, 공개 CSS 지원 또는 WPT 적합성을 완료로 승격하지 않는다. 공식 집계는 `72/213` 그대로다.

구현 출발 commit은 [`1e73158`](https://github.com/ohah/spinon/commit/1e73158e945f2a9aa6d8b16718435d3a6497cc51)이며 현재 후보는 [PR #1035](https://github.com/ohah/spinon/pull/1035)다. 후속 검토에서 Root HTML named-area의 네 min/max 진단이 모두 `min-width`로 뭉뚱그려지던 기존 오류를 찾았다. 위반한 computed property별 오류를 반환하도록 수정하고 `max-width`, `min-height`, `max-height`, `min-width`를 각각 회귀 검사했다. 이 변경은 거부 입력의 진단만 바꾸며 성공 scene geometry는 바꾸지 않는다. 화면 캡처는 그 진단 수정 전 구현 commit에서 얻었고, 최종 Rust 검사와 PR에는 수정이 포함된다.

## 고정 입력과 독립 기대값

Chrome 156 Agent Browser 사전 비교와 산술 기대값은 [precomparison README](../c11-root-html-grid-area-percentage-precomparison-2026-11-02/README.md), [`expected.json`](../c11-root-html-grid-area-percentage-precomparison-2026-11-02/expected.json), [`chrome-observation.json`](../c11-root-html-grid-area-percentage-precomparison-2026-11-02/chrome-observation.json)에 있다. 두 플랫폼에 같은 HTML SHA-256을 넣었다.

| Scene | HTML SHA-256 | columns / rows (CSS px) | 기대 paint box |
| --- | --- | --- | ---: |
| `both-axes` | `26162acf3d707fc47e91a997451a133ac30300266b757ae914dbecb2cbc48487` | `[30,90]` / `[14,56]` | 6 |
| `mixed-px-percent` | `b55dd31c32a6b0881f25cd6d57a8351496a58584d42c788b57dc2e15f8150898` | `[40,60]` / `[14,50]` | 6 |
| `overflowing-columns` | `0e7996ca52f1462b28ce2b599ec470cfd5111499d2d9acc86f19baa418db9261` | `[144,12]` / `[14,56]` | 6 |
| `zero-track` | `ac563fb0d1576065e44a3f01f1b739f00e8072c2e2f2021660e47ef45bd33950` | `[0,120]` / `[14,56]` | 5 |

각 root grid는 `(0,0,120,70)` CSS px다. semantic item frame은 `expected.json`과 Rust runtime test가 대조한다. Android/iOS app log는 바깥 HTML root frame `320×240`과 revision·layout readiness·draw/present를 기록하지만 item별 layout frame을 내보내지 않는다. 따라서 item geometry 근거는 Rust layout snapshot assertion이고, 모바일 PNG는 화면 배치의 시각 확인이다.

## Android API 37 ARM64 AVD

기기는 `emulator-5556`, `sdk_gphone64_arm64`, Android API 37 / release 17이다. Renderer는 ANGLE → Vulkan → SwiftShader software renderer다. 네 scene 모두 default `index.html` Root HTML 경로에서 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`, `layout=ready`와 `SPINON_C0410_DRAW status=0 presented`를 기록했다. document revision `1`, environment revision `0`, render-tree revision `1`이다. Root `320×240` CSS px, surface `1080×2209` 로그와 1080×2400 screenshot이 있다.

| Scene | 실행 로그 | 화면 |
| --- | --- | --- |
| `both-axes` | [runtime](./android-avd/runtime-both-axes.log) | [PNG](./android-avd/both-axes.png) |
| `mixed-px-percent` | [runtime](./android-avd/runtime-mixed-px-percent.log) | [PNG](./android-avd/mixed-px-percent.png) |
| `overflowing-columns` | [runtime](./android-avd/runtime.log) | [PNG](./android-avd/overflowing-columns.png) |
| `zero-track` | [runtime](./android-avd/runtime-zero-track.log) | [PNG](./android-avd/zero-track.png) |

입력별 APK/source hash는 [`matrix-hashes.txt`](./android-avd/matrix-hashes.txt), build/install/launch 원본은 같은 디렉터리에 있다. 첫 zero-track harness는 6개 paint box를 기대했지만 실제 로그는 5개였다. 폭 0인 side item은 layout frame `(0,14,0,56)`을 유지하면서 paint box를 만들지 않는 것이 올바르다. 기대 box 수를 5로 바꿔 다시 캡처했고 최초 로그는 `runtime-zero-track-first-attempt.log`로 보존했다. 이는 제품 layout 실패가 아니라 harness 기대값 오류였다.

## iOS Simulator

대상은 iPhone 17 Pro / iOS 26.2 Simulator, UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`다. 네 scene 모두 같은 각 HTML SHA로 기본 Root HTML 앱 경로를 열어 `layout=ready` 및 draw/present를 기록했다. document revision `1`, environment revision `0`, render-tree revision `1`이다. 로그의 surface는 `1206×2334`; 캡처 PNG는 1206×2622다.

| Scene | 실행 로그 | 화면 |
| --- | --- | --- |
| `both-axes` | [runtime](./ios-simulator/runtime-both-axes.log) | [PNG](./ios-simulator/both-axes.png) |
| `mixed-px-percent` | [runtime](./ios-simulator/runtime-mixed-px-percent.log) | [PNG](./ios-simulator/mixed-px-percent.png) |
| `overflowing-columns` | [runtime](./ios-simulator/unified-log.txt) | [PNG](./ios-simulator/overflowing-columns.png) |
| `zero-track` | [runtime](./ios-simulator/runtime-zero-track.log) | [PNG](./ios-simulator/zero-track.png) |

입력별 app binary/source hash는 [`matrix-hashes.txt`](./ios-simulator/matrix-hashes.txt), PNG hash는 [`screenshots-sha256.txt`](./screenshots-sha256.txt)다. `xcodebuild`는 성공했다. 로그에 V8 archive 중복 symbol/dSYM object와 AppIntents metadata 관련 경고가 있으나 앱 설치·실행은 성공했다. 이 경고를 해결된 원인으로 추정하지 않는다.

## 검사 결과

최종 소스에서 다음 focused 검사가 통과했다. 전체 CSS suite 또는 workspace test를 의미하지 않는다.

- `cargo test -p spinon-style-to-layout --lib root_html`: 15 passed.
- `cargo test -p spinon-style-to-layout --lib c11_3_3`: 26 passed.
- `cargo test -p spinon-runtime --lib root_html_grid`: 24 passed. Named-area percentage positive·rejection matrix를 포함한다.
- `cargo clippy --locked -p spinon-runtime -p spinon-style-to-layout -p spinon-style --all-targets -- -D warnings`: passed.
- `cargo fmt --all -- --check` 및 `git diff --check`: passed.

명령 출력은 [Root HTML projection 15/15](./rust-style-root-html-final.log), [C11.3.3 projection 26/26](./rust-style-c11-3-3-final.log), [Root HTML runtime 24/24](./rust-runtime-root-html-grid-final.log), [Clippy](./clippy-final.log), [rustfmt](./rustfmt-final.log), [diff 검사](./git-diff-check-final.log)에 있다. Android API 37 AVD build는 성공했다. `build.log` 기준 Android Gradle Plugin 8.13.2의 compileSdk 37 지원 범위 경고를 기록한다.

일반 `.log`는 `git diff --check`가 통과하도록 trailing whitespace와 마지막 빈 줄만 정리했다. 명령/기기 원본 출력은 같은 이름 뒤에 `.raw.gz`를 붙인 압축 파일로 그대로 보존했다.

## 관찰 한계와 정리

- screenshot은 Android `screencap` 및 `simctl screenshot` 시각 자료다. Android PixelCopy, iOS Metal drawable readback, compositor 이후 pixel equality는 실행하지 않았다.
- overflow 장면은 120px grid root보다 넓은 header/content frame을 Rust test에서 직접 확인한다. 이미지는 colored content가 자연스럽게 연장되는 것을 보여주지만 root edge 자체에 border를 추가하지 않았으므로 PNG만으로 root 경계 좌표를 재측정하지 않는다.
- Android는 SwiftShader software renderer이며 Android 실기기·hardware GPU 결과가 아니다. iOS도 Simulator 결과다.
- Chrome precomparison HTML의 shorthand `background:`는 현재 제한 CSS profile이 `background-position-x`로 진단해 거부하므로 동등한 `background-color:`로 정규화했다. Chrome geometry와 PNG hash가 전후 동일했다.
- Android 앱은 force-stop했고 대상 iOS Simulator app은 종료했다. 임시로 바꿨던 `examples/bootstrap/index.html`과 `root-app.js`는 각각 원본 SHA-256 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`, `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`로 복원했다. 테스트 AVD/Simulator는 종료하지 않았다.

## 실패 관점 검토

- [구현/runtime 실패 관점 20개](./implementation-runtime-review-20.md)
- PR 변경 diff를 대상으로 한 독립 관점 검토는 [pr-diff-review-30.md](./pr-diff-review-30.md)에 둔다.

이 근거는 고정 named-area profile의 제한 후보만 설명한다. 전체 C11.1/C11 완료, 일반 CSS Grid 지원, Spinon WPT 적합성, Android 실기기 또는 iOS 최종 compositor 출력으로 확대 해석하지 않는다.

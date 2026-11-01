# C11.1 parsed Root HTML named-area runtime 실행 근거

## 판정 범위

이 검증은 구현 branch `feat/c11-root-html-grid-named-area-runtime-20261102`의 pending changes를 대상으로 한다. 기존 `RuntimeRootHtmlTextPaintV1` 기본 Root HTML GPU route에 구현 전 고정한 HTML fixture를 입력했다. C11.3.3 고정 fixture route를 호출하지 않았다. Android와 iOS 번들에 든 `index.html` SHA-256은 모두 `7c49f066398d51b6ade5ccf842addfc9bf1ec857b21b9e16c7e74d39a6275f3d`로, 기준 파일 `tests/fixtures/css/c11/root-html-grid-template-areas-precomparison.html`과 일치한다.

`root-app.js`는 번들러가 제거하지 않는 `document.documentElement.setAttribute("data-c11-runtime", "ready")`만 실행했다. 이는 DOM과 layout이 같은 세션에서 실행되는지 확인하기 위한 app script marker이며 별도 Grid fixture 또는 CSS/layout 입력을 만들지 않는다. 앱의 bundled `examples/bootstrap/index.html`과 `root-app.js` 원본은 빌드 중 임시 교체 후 복구했고 현재 worktree에서 수정되지 않았다.

기대 geometry는 source의 fixed tracks와 2×2 area matrix를 직접 더해 산출했다. Rust runtime test가 root `(0,0,120,70)`, header `(0,0,120,20)`, side `(0,20,40,50)`, content `(40,20,80,50)` CSS px를 확인한다. semantic frame은 DOM child 순서를 뒤집고 ID·selector 이름을 바꾸어도 유지된다.

## Rust 검증

| Gate | 결과 |
|---|---|
| `mise exec -- cargo fmt --all -- --check` | 통과 |
| `mise exec -- cargo test -p spinon-style --lib` | 258 passed |
| `mise exec -- cargo test -p spinon-style-to-layout --lib template_area` | 35 passed |
| `mise exec -- cargo test -p spinon-style-to-layout --lib root_html_named_area_projection_reports_line_and_area_allocation_failures` | 1 passed; Root HTML line-group, area-vector, area-name reservation failure context 확인 |
| `mise exec -- cargo test -p spinon-runtime --lib root_html_grid` | 7 passed; named-area positive, 21 out-of-scope candidates, CSS-invalid fallback, newer media-query revision failure에서 stale scene 미노출 포함 |
| `mise exec -- cargo test -p spinon-runtime --lib root_html` | 50 passed; viewport/color-scheme, block/text paint, fixed Grid·gap·named-area 회귀 포함 |
| `mise exec -- cargo clippy --locked -p spinon-runtime -p spinon-style-to-layout -p spinon-style --all-targets -- -D warnings` | 통과 |
| `mise exec -- cargo build --locked -p spinon-ffi --lib` | 기본 static archive 빌드 통과. LLVM 22 `llvm-nm`에서 C11 fixture host export가 0개이며 archive SHA-256은 `d641ccb6c2c135d5768d138560a2077cb6a54ff7583c1b6a7f9ce3af1561d093`이다. |
| `mise exec -- cargo build --locked -p spinon-ffi --lib --features c11-grid-metal-surface-pixels-test` | opt-in positive control 빌드 통과. `spinon_runtime_gpu_host_new_c11_3_3_grid_template_areas_fixture` export가 존재한다. 검증 뒤 target archive는 기본 빌드로 복원했다. [archive boundary record](./ffi-archive-boundary.txt). |
| `mise exec -- cargo test -p spinon-style-to-layout` | 279 중 278 passed, 1 failed. `c07_3_aspect_ratio_tests::comparison::aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs`가 computed serialization `1.7777778 / 1` 대 `1.77778 / 1`에서 실패했다. Grid 변경 외의 C07 비교 실패이며, 전체 crate gate는 통과로 처리하지 않는다. |

명령 출력은 `git diff --check`를 통과하도록 로그 끝의 공백과 여분 빈 줄만 정규화해 `rustfmt.log`, `style-full-test.log`, `style-to-layout-template-area-test.log`, `root-html-area-allocation-test.log`, `runtime-focused-test.log`, `runtime-root-html-regressions.log`, `clippy.log`, `default-ffi-build.log`, `default-ffi-c11-fixture-symbols.txt`, `opt-in-ffi-build.log`, `opt-in-ffi-c11-fixture-symbols.txt`, `ffi-archive-boundary.txt`, `style-to-layout-full-test.log` 및 `style-to-layout-full-test.exit`에 보관한다.

## Android API 37 ARM64 AVD

- Device: `emulator-5556`, `sdk_gphone64_arm64`, Android 17 / API 37, `arm64-v8a`, physical screen 1080×2400 px, density 420 dpi.
- `mise exec -- env ANDROID_SERIAL=emulator-5556 bun run build:android`가 성공했다. APK의 `assets/index.html` 해시는 기준 fixture와 일치한다.
- 앱을 launch intent 없이 시작해 로그의 `SPINON_ROOT_HTML_DEFAULT_APP_ROUTE source=index.html`, `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass ... layout=ready`, `SPINON_C0410_DRAW status=0 presented`를 확인했다.
- [ADB logcat](./android-avd/logcat.txt), [APK input hash](./android-avd/hash-verification.txt), [build](./android-avd/build.log), [화면 캡처](./android-avd/c11-root-html-grid.png).

## iOS Simulator

- Device: iPhone 17 Pro / iOS 26.2, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`.
- `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 bun run build:ios-sim`가 성공했다. 앱 번들의 `index.html` 해시는 기준 fixture와 일치한다.
- 앱을 launch argument 없이 기본 Root HTML app route로 시작해 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass ... layout=ready`, `SPINON_C0410_RENDERER ... backend=Metal device=Apple iOS simulator GPU`, `SPINON_C0410_DRAW ... status=0 presented`를 확인했다.
- [Unified log](./ios-simulator/unified-log.txt), [앱 입력 hash](./ios-simulator/hash-verification.txt), [build](./ios-simulator/build.log), [화면 캡처](./ios-simulator/c11-root-html-grid.png).

## 한계와 상태

양 플랫폼 결과는 Simulator/AVD의 기본 Root HTML app route에서 scene 준비와 draw/present를 확인한 실행 및 화면 캡처다. Android PixelCopy 또는 iOS Metal drawable pixel readback은 수행하지 않았으며 실기기, hardware GPU, post-compositor 색상 결과를 주장하지 않는다. 전체 C11.3.3 fixture 행렬과 WPT 적합성을 검증한 결과도 아니다.

구현/runtime 20개와 PR diff 30개 실패 관점 검토는 각각 [구현/runtime review](./implementation-runtime-review-20.md), [PR diff review](./pr-diff-review-30.md)에 기록했다. 구현 PR 생성·GitHub checks/review·merge는 남아 있다. C11.1/C11.3.3/C11.4/C11의 공식 checkbox와 완료 수 `72/213`은 그대로다.

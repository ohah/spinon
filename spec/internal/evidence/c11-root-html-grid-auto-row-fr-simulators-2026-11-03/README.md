# C11.1 parsed Root HTML `grid-auto-rows: fr` 실행 근거

이 근거는 `RuntimeRootHtmlTextPaintV1` 기본 parsed Root HTML 경로의 no-area Grid에서 direct finite non-negative px/%/fr implicit-row 목록을 순환하는 제한 slice 후보에 대한 것이다. 공식 C11 완료를 의미하지 않으며, 프로젝트 집계 `72/213` 및 C11 상위 미완료 상태는 유지한다.

## 입력과 독립 기준

- Runtime fixture: [`root-html-grid-auto-row-fr-runtime.html`](../../../../tests/fixtures/css/c11/root-html-grid-auto-row-fr-runtime.html)
- Fixture SHA-256: `573b76610f648e52913225f9481f854c9ae9c9fedb57ecfe2fa1d2b1a5ad6f51`
- Chrome 156 사전 비교 7 scene, 독립 기대값 및 관찰 원본: [precomparison evidence](../c11-root-html-grid-auto-row-fr-precomparison-2026-11-03/README.md)
- Chrome 기준에서 고정한 used row와 item frame은 layout/runtime 테스트에서 직접 비교했다. 플랫폼 화면은 시각 smoke evidence이며 pixel-readback 비교가 아니다.

## Rust 검증

| 영역 | 명령 필터 | 결과 |
|---|---|---:|
| Style cascade | `cargo test --locked -p spinon-style c11_1_root_html_auto_row_list` | 9 passed |
| Style-to-layout projection | `cargo test --locked -p spinon-style-to-layout root_html_profile_` | 12 passed |
| Layout rows | `cargo test --locked -p spinon-layout root_html_implicit_rows` | 22 passed |
| Layout auto-placement regression | `cargo test --quiet --locked -p spinon-layout grid::auto_placement_tests` | 52 passed |
| Runtime implicit-row integration | `cargo test --locked -p spinon-runtime root_html_grid_implicit_rows` | 11 passed |
| Broader Root HTML runtime regression | `cargo test --locked -p spinon-runtime root_html_grid_` | 41 passed |
| Scoped Clippy | `cargo clippy --locked -p spinon-layout -p spinon-style -p spinon-style-to-layout -p spinon-runtime --all-targets -- -D warnings` | passed |
| Formatting | `cargo fmt --all -- --check` | passed |
| Diff whitespace | `git diff --check` | passed |

`cargo test --quiet --locked -p spinon-style-to-layout root_html_`는 36/37이며, 변경하지 않은 `parsed_root_html_css_invalid_fraction_falls_back_to_unsupported_auto_columns`에서 실패했다. 해당 검사는 `grid-auto-columns: -1fr`의 cascade fallback 결과를 예상 `UnsupportedComputedValue`와 대조한다. 이 변경은 auto-row 경로만 수정하며 auto-column 코드는 건드리지 않았다. 이 넓은 suite는 통과로 계산하지 않는다.

## Android API 37 ARM64 AVD

- AVD: `spinon_api37_1_compat16k`, API 37, `arm64-v8a`, emulator (`ro.kernel.qemu=1`)
- Renderer: ANGLE / SwiftShader software path. 실기기 GPU 근거가 아니다.
- 제품 runtime GPU 빌드: `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 bun run build:android` · **BUILD SUCCESSFUL**
- Android APK 안의 Root HTML asset과 fixture SHA가 일치한다. runtime source log에도 같은 SHA가 나온다.
- runtime: `layout=ready boxes=13`, `SPINON_C0410_DRAW status=0 presented boxes=13`.
- [앱 화면 캡처](./android-api37-screen.png) · [runtime log (gzip)](./android-runtime.log.gz) · [device 정보](./android-device.txt) · [build log (gzip)](./android-build.log.gz)
- Surface `PixelCopy`는 하지 않았다.

## iOS 26.2 Simulator

- Device: iPhone 17 Pro, iOS 26.2, Xcode 26.2. 기존 부팅된 Simulator들을 보존하기 위해 별도 CoreSimulator device set을 사용했다.
- Renderer: iOS Simulator Metal. 실기기 GPU 근거가 아니다.
- 제품 runtime GPU 빌드: `mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=0 bun run build:ios-sim` · **BUILD SUCCEEDED**
- App bundle 안의 Root HTML asset과 fixture SHA가 일치하며 runtime source log에도 같은 SHA가 나온다.
- runtime: `layout=ready boxes=13`, `SPINON_C0410_DRAW status=0 presented boxes=13`.
- [앱 화면 캡처](./ios-26-2-screen.png) · [runtime log (gzip)](./ios-runtime.log.gz) · [device 정보](./ios-device.txt) · [build log (gzip)](./ios-build.log.gz)
- Metal drawable의 직접 pixel readback은 하지 않았다.

## 빌드 입력과 한계

각 플랫폼의 기존 `root-html-default-rendering.html` asset에 fixture를 빌드 때만 복사해 기본 Root HTML runtime route로 실행했다. 빌드 입력 SHA는 committed runtime fixture와 일치하며, 빌드가 끝난 뒤 두 원본 asset을 백업에서 복구했다. 저장소 asset diff는 없다. 해시와 build/install/launch log는 이 디렉터리에 보존한다.

이 결과는 한 Android API 37 software-rendered AVD와 한 iOS 26.2 Simulator에서 같은 HTML 입력의 기본 runtime layout 및 draw/present를 확인한 것이다. WPT 적합성, 실기기·하드웨어 GPU, 전체 CSS Grid 또는 C11/C11.1 완료를 주장하지 않는다. 계획 PR [#1069](https://github.com/ohah/spinon/pull/1069)은 병합됐다. [구현/runtime 실패 관점 검토 20개](./implementation-runtime-review-20.md)와 [구현 PR diff 실패 관점 검토 30개](./implementation-pr-diff-review-30.md)는 별도로 완료했다. 구현 PR 생성·GitHub checks 확인·merge 및 후속 roadmap sync는 남아 있다.

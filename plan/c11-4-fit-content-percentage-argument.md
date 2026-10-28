# C11.4 · definite-height percentage `fit-content()` row WPT slice

**상태:** 제한된 `fit-content-percentage-row-wpt` 구현과 Android API 37 ARM64 AVD / iPhone 17 Pro iOS 26.2 Simulator 실행이 통과했다. Rust runtime 2/2, FFI route 1/1, runtime·FFI fixture-feature Clippy, fixture-module boundary 18/18, capture-script syntax와 `cargo fmt --all -- --check`가 통과했다. Default static FFI archive marker 0개와 fixture-test positive control 4개를 확인했고 구현/runtime 실패 관점 20개 검토도 마쳤다. PR diff 검토 30개와 PR 생성은 남았다. 계획 검토 20개는 [별도 문서](./c11-4-fit-content-percentage-argument-plan-review-20.md)에, 계획 PR diff 검토 30개는 [별도 기록](./c11-4-fit-content-percentage-argument-pr-diff-review-30.md)에 남겼다. 현재 실행 화면과 제한은 [시뮬레이터 근거](../spec/internal/evidence/c11-4-fit-content-percentage-argument-runtime-2026-10-29/README.md)에 기록한다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목표와 한계

Pinned WPT `css/css-grid/grid-definition/fit-content-track-sizing.html`의 percentage argument assertion을 기존 opt-in C11.4 CSS→HostDocument/Stylo→typed Grid input→Taffy→Android/iOS 앱 fixture 경로에서 검증한다. definite 300px block size에 대한 `fit-content(50%)` 행 limit을 확인한다.

기존 coverage는 cyclic row `fit-content(50%)`와 definite column control, fixed-pixel fit-content floor 및 spanning input을 다룬다. 이 WPT assertion의 definite block-axis percentage path는 별도 앱 scene으로 확인되지 않았다. 이 계획은 한 고정 입력의 sizing과 root box 바깥 overflow 관찰로 제한하며 일반 CSS admission 또는 API 지원을 선언하지 않는다.

## 고정 출처와 upstream 관찰

- WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 원본 [`fit-content-track-sizing.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html), SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`.
- 대상 DOM은 원본 48행, assertion은 107행이다. Pinned checkout과 2026-10-29 `wpt.live` 원본의 SHA-256이 일치했다.
- Agent Browser `0.31.1`, HeadlessChrome `156.0.0.0`, viewport `1280×633` CSS px, DPR 1에서 upstream harness `OK`, 13 tests / 13 Pass를 확인했다. Target root height는 `300px`, computed row list는 `150px`, grid item height는 `150px`이다. 전체 결과는 [WPT screenshot](../spec/internal/evidence/c11-4-fit-content-percentage-argument-precomparison-2026-10-29/wpt-fit-content-percentage-argument.png)에 있다.
- 이 결과는 upstream Chrome의 pinned WPT 실행이다. Spinon WPT 실행이나 앱 검증을 의미하지 않는다. 관찰 값과 source hash는 [기계 판독 기준](../spec/internal/evidence/c11-4-fit-content-percentage-argument-precomparison-2026-10-29/expected.json)과 [사전 비교 README](../spec/internal/evidence/c11-4-fit-content-percentage-argument-precomparison-2026-10-29/README.md)에 고정했다.

## 독립 기대값

CSS Grid 2025 CRD [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)는 row track의 percentage basis를 grid container의 inner block size로 정한다. Grid 크기가 track 크기에 의존하면 intrinsic sizing 동안 percentage를 `auto`처럼 취급한 뒤 결과 크기로 resolve하지만, 이 입력의 `height:300px`은 definite다. 따라서 limit은 `300 × 0.5 = 150px`이다. [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms)의 available grid space는 definite container의 content box다.

[§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms)의 fit-content sizing과 [§6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)의 grid item minimum을 함께 적용한다. WPT item은 `min-height:0`을 지정하며 내부 block은 400px 높이다. Contribution vector는 `Minimum 0`, `MinContent 400`, `MaxContent 400`이다. `fit-content(150px)`의 effective base-size cap으로 expected used row size는 150px이다. Raw diagnostic growth limit 400px은 max-content 쪽 상한이며 used row size를 뜻하지 않는다. CSS Grid 2025 CRD §11.5.1은 base-size distribution limit을 growth limit과 fit-content argument 중 작은 값으로 제한한다. 기대값은 Chrome observation에서 역산하지 않았다.

## 제한 runtime 입력

새 allowlisted scene ID는 `fit-content-percentage-row-wpt`다. DOM은 Grid root, direct grid item, fixed-height child의 세 `div`로 고정한다. Root는 `width:60px;height:300px`, `grid-template-columns:60px`, `grid-template-rows:none`, `grid-auto-rows:fit-content(50%)`, zero gap, `align-content:normal`, row auto-flow를 쓴다. WPT root의 auto width는 app viewport 의존성을 없애기 위해 60px로 고정한다. Row percentage basis와 target row value는 유지한다.

Grid item은 `min-height:0`을 사용한다. Computed overflow는 initial `visible`이며 이를 author declaration으로 추가하지 않는다. Child는 `width:30px;height:400px`로 두어 row block-axis input은 유지하면서 item/root 배경이 보일 가로 영역을 남긴다. Root, item, child에는 고정 배경색을 둔다. Root의 CSS border는 추가하지 않는다. Root-relative pixel centers `(15.5,149.5)`, `(45.5,149.5)`, `(45.5,200.5)`, `(15.5,350.5)`에서 각각 row 안 child, row 안 item, row 아래 root background, root box 밖 child를 Android PixelCopy와 iOS 제출 전 Metal drawable에서 sample한다. 가장 아래 표본을 포함하도록 이 scene만 app viewport를 `320×400` CSS px로 설정한다. Root 자체와 percentage basis는 `60×300`으로 유지한다.

Runtime fixture는 scene ID, namespace/tag, root ownership, node topology, computed properties, typed track, dimensions와 document/style/environment revision을 확인한다. Unknown ID와 root size, track, item style 또는 tree를 바꾼 입력은 fail-closed한다. Fixture module·scene payload는 opt-in `c04-runtime-gpu-fixture-test`에만 둔다.

## 수용 기준

1. Rust layout/runtime oracle에서 root `[0,0,60,300]`, Columns `[60]px`, Rows `[150]px`, item `[0,0,60,150]`, child `[0,0,30,400]`을 same-revision track trace와 함께 검사한다. Rows trace는 six-stage sequence/base values와 `Minimum 0`/`MinContent 400`/`MaxContent 400` contribution을 exact 검사한다.
2. Positive input은 정확히 한 implicit row를 만든다. Explicit row, 추가 item, span, cyclic percentage, 다른 percentage basis는 이 slice의 입력으로 허용하지 않는다. Mutated style/tree와 unlisted scene은 거부한다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 V8→HostDocument→Stylo→typed Grid→Taffy→WGPU draw/present 경로를 실행한다. 각 플랫폼에서 track/frame/revision 결과를 보존한다.
4. Surface sample은 root-relative pixel centers `(15.5,149.5)`, `(45.5,149.5)`, `(45.5,200.5)`, `(15.5,350.5)` CSS px를 포함해 row 내부, row 끝, root 안쪽 배경과 root box 바깥 overflow를 구별한다. 앱 surface 실행이 없는 callback/frame log만으로 통과 처리하지 않는다.
5. Default/product FFI archive에는 신규 scene ID·marker·symbols가 없고 fixture-test archive에는 positive control로 포함된다. Android/iOS simulator 앱 build/run, focused Rust/runtime/FFI checks와 artifact boundary 결과를 기록한다.
6. 계획 검토, 구현/runtime 적대 검토와 PR diff 검토는 서로 독립된 관점으로 남긴다. 수정된 실패 관점은 영향 범위를 다시 확인한다.

## 범위 밖과 상태

나머지 WPT assertion 12개, WPT harness의 Spinon 실행, CSS author admission, arbitrary Grid tree, 일반 `fit-content()` support, 실기기·hardware GPU, iOS compositor 이후 pixels와 전체 C11.4 적합성은 이 slice로 판정하지 않는다. `spec/STATUS.md`의 C11.4 상태와 공식 완료 `72/213`은 미완료로 유지한다.

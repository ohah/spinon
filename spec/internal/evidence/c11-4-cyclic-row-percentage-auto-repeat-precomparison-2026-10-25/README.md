# C11.4 cyclic percentage auto-repeat: row-axis precomparison

이 디렉터리는 Chrome 156의 CSS 관측과 CSS Grid 규범으로 계산한 독립 기대값을 비교하는 사전 자료다. Spinon layout 결과, runtime fixture 실행, simulator 검증, artifact 경계를 증명하지 않는다. 이전 `repeat(auto-fill, minmax(0px, 50%))` row-axis 계획의 `[8px, 8px]` 가설은 독립 계산과 Chrome 관측에 맞지 않아 구현 승인 근거로 사용할 수 없다.

## 입력과 관측 환경

- Fixture: [`grid-cyclic-row-percentage-auto-repeat-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-row-percentage-auto-repeat-c114-precomparison.html)
- Scene IDs: `cyclic-max-content-auto-fill-rows`, `definite-40px-auto-fill-control-rows`
- Fixture SHA-256: `fe753b810e9d03dfb7c7386196583934027b1a0c07f9eaa9dbde6aa2fecde027`
- Browser: Agent Browser 0.31.1, macOS ARM64, `HeadlessChrome/156.0.0.0`, viewport `1280×633` CSS px, DPR 1.
- Raw Agent Browser `eval --json` output: [`chrome156-observation.json`](./chrome156-observation.json), SHA-256 `eb0266838a532b461399b5b264b3b0b152a4409f38debcaed5c3378e0429db0b`.
- Screenshot: [`chrome156-agent-browser.png`](./chrome156-agent-browser.png), PNG `1280×633`, SHA-256 `b06f58a88e530af90914cae2e83952631dc3b354697d1b24fb3d0a7b3a6571ff`.
- `CSS.supports()` returned `true` for the row auto-repeat declaration and `height: max-content`; `document.fonts.status` was `loaded` (the fixture uses a fixed-size empty block, not text metrics).

Fixture는 root `min-height`를 선언하지 않는다. CSS Box Sizing Level 3의 초기값은 `auto`이며, Chrome 원본 관측의 `getComputedStyle().minHeight === "0px"`는 브라우저가 직렬화한 CSSOM 결과로 그대로 보존한다. 이 관측을 authored `0px` 또는 Spinon typed `Fixed(0)` 입력으로 해석하지 않는다. 구현 계획의 root typed 값은 `LayoutDimension::Auto`다.

## 독립 규범 계산과 Chrome 결과

기준은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), [§11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-overview), [CSS Sizing Level 3 §2](https://www.w3.org/TR/css-sizing-3/#definite), [§3.1.2](https://www.w3.org/TR/css-sizing-3/#min-width-min-height)다. CSS Sizing은 `max-content` intrinsic keyword를 indefinite size로 정의하고, `min-height` 초기값은 `auto`로 둔다. 따라서 이 입력의 root에는 authored definite minimum size가 없다. Grid §7.2.3.2에 따라 definite preferred/maximum size와 definite minimum size가 모두 없으므로 auto-repeat count는 1회다. Grid §7.2.1과 §11은 cyclic track percentage를 grid container intrinsic size 계산 중 `auto`처럼 취급하고, container 크기를 얻은 뒤 그 크기를 percentage basis로 사용한다.

| Scene | 독립 기대값 | Chrome 156 관측 | 비교 |
| --- | --- | --- | --- |
| `cyclic-max-content-auto-fill-rows` | intrinsic sizing에서 한 row가 유일한 `24×16px` in-flow item으로부터 16px block size를 얻는다. 최종 `50%` maximum은 8px이므로 used rows `[8px]`; root content box `24×16px`; item `24×16px` | `gridTemplateRows: "8px"`; root computed size `24×16px`, rect `(32, 59.59375) 24×16`; item rect 동일; half-height marker `y=67.59375` | 일치. 1 row이며 `[8,8]`이 아님 |
| `definite-40px-auto-fill-control-rows` | definite 40px block size의 50% maximum은 20px. auto-fill은 두 row가 차지하는 40px에서 멈춤: `[20px,20px]`; root `24×40px`; item `24×16px` | `gridTemplateRows: "20px 20px"`; root computed size `24×40px`, rect `(332,59.59375) 24×40`; item rect `(332,59.59375) 24×16`; marker `y=79.59375` | 일치 |

Target의 계산 순서는 다음과 같다. 먼저 `height:max-content`는 indefinite이며 root `min-height:auto`도 definite minimum size가 아니므로 auto-repeat count는 한 번이다. Intrinsic sizing에서 `50%`는 `auto`처럼 동작한다. 유일한 in-flow child의 fixed height 16px와 0 gap으로 root max-content content height는 16px이다. Final layout에서 percentage basis는 그 16px이고 maximum은 8px이다. 단일 row의 used size는 `[8px]`가 된다. Child는 fixed height 16px라 8px row를 8px 넘지만 root content box는 16px이다. absolute `.half-height-marker`는 sizing item이 아니며 표본에서 반높이 좌표만 보여준다.

Chrome CSSOM/DOMRect는 위 기대값과 일치한다. 따라서 `[8px,8px]` 재확장 가설을 반박하는 관측이며 CSS Grid 규범과 Chrome 사이의 divergence가 아니다. 이 자료는 row auto-repeat count를 percentage resolution 뒤 다시 늘리는 구현을 승인하지 않는다. 새 알고리즘 변경을 시작하기 전에 규범/기대값을 다시 검토해야 한다.

Raw observation의 normalized scene JSON SHA-256 (각 scene object를 key-sort, compact UTF-8 JSON으로 직렬화한 바이트):

- `cyclic-max-content-auto-fill-rows`: `1840d5558351d8ea66bde3283681cc4414bf3b7dece75d7e106dabf58b10f774`
- `definite-40px-auto-fill-control-rows`: `8fccc8e618ec79d87a0d2f4c4de29d05125c31c2e93b800933ff86b03c6440bb`

Machine-readable 계산은 [`spec-derived-expectations.json`](./spec-derived-expectations.json)에 있다. 계획의 독립 실패 관점 20개는 [`plan-review-20.md`](./plan-review-20.md)에 있다. Hash manifest는 [`SHA256SUMS`](./SHA256SUMS)다.

## 현재 Spinon 지원 경계 (읽기 전용 코드 조사)

- Definite percentage auto-repeat row의 기반 입력/알고리즘은 이미 테스트된다. [`positive_percentage_auto_repeat.rs`](../../../../crates/spinon-layout/src/grid_track_sizing_tests/positive_percentage_auto_repeat.rs#L165)는 axes `[Columns, Rows]`와 auto-fill/auto-fit을 순회하고, [`#L259`](../../../../crates/spinon-layout/src/grid_track_sizing_tests/positive_percentage_auto_repeat.rs#L259)는 definite percentage maximum의 content-axis 및 gap 계산을 양 축에서 확인한다. [`grid.rs`](../../../../crates/spinon-layout/src/grid.rs#L1355)는 `grid-template-rows` basis로 `height`, `min-height`, `max-height`, vertical insets를 읽는다. 따라서 definite 40px control은 일반 layout input으로 처리할 수 있는 축이다. 이 exact HTML scene의 앱 runtime/Simulator 검증을 뜻하지는 않는다.
- Cyclic max-content row target은 현재 C11.4 opt-in fixture admission에서 허용되지 않는다. [`grid.rs`](../../../../crates/spinon-layout/src/grid.rs#L486)는 row percentage validation context에 `fixture_cyclic_percentage_auto_repeat: false`를 전달한다. 최대값이 percentage일 때 [`#L1332`](../../../../crates/spinon-layout/src/grid.rs#L1332)는 그 opt-in 허용 또는 definite content basis를 요구하므로 이 indefinite target은 validation에서 fail-closed된다. Existing allowlist [`#L851`](../../../../crates/spinon-layout/src/grid.rs#L851)는 `width:max-content`/`width:40px`, `columns_auto_repeat`, row `auto`를 고정하므로 row target/control trace를 포함하지 않는다.
- Taffy에는 indefinite parent height와 percentage rows가 있으면 block-axis track sizing을 다시 실행하는 일반 경로가 있다([`vendor/taffy/src/compute/grid/mod.rs`](../../../../vendor/taffy/src/compute/grid/mod.rs#L640), [`#L765`](../../../../vendor/taffy/src/compute/grid/mod.rs#L765)). 현재 fixture 전용 repeat-count 재확장 코드는 column rerun에서만 horizontal axis를 사용한다([`#L658`](../../../../vendor/taffy/src/compute/grid/mod.rs#L658)); row rerun은 기존 `row_auto_repetition_count`를 전달한다([`#L765`](../../../../vendor/taffy/src/compute/grid/mod.rs#L765)). 이 row 동작은 여기서 실행하지 않았지만, 독립 기대값 `[8px]`에는 재확장이 필요하지 않다.
- 현재 계획 [`c11-4-cyclic-percentage-auto-repeat.md`](../../../../plan/c11-4-cyclic-percentage-auto-repeat.md)는 columns 한 축으로 제한하고 cyclic row axis를 제외한다. 따라서 남은 증거 공백은 max-content row scene의 opt-in runtime/Android·iOS Simulator 검증 및 필요한 exact row fixture admission이다. 이 코드는 명시된 target을 실제 실행하지 않았으므로 예상 출력의 runtime 일치 여부는 미확인이다. Row-axis 재확장을 지원 공백으로 분류하거나 구현 대상으로 선택할 근거는 없다.

이 조사는 source/test 파일을 변경하지 않았다. Spinon tests, build, browser runtime, Android/iOS simulator, artifact-boundary 검사는 실행하지 않았다. 화면 캡처는 Chrome 관측용이며 border paint, compositor pixel equality, 일반 CSS admission, WPT 또는 C11.4 전체 완료를 주장하지 않는다.

## Browser lifecycle

Agent Browser unique namespace/session: `spinon_c114_row_axis_20261025` / `c114-row-axis-20261025`. Session close 및 해당 daemon/Chrome PID 종료 증거는 [`agent-browser-lifecycle.json`](./agent-browser-lifecycle.json)에 기록한다.

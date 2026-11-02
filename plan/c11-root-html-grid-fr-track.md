# C11.1 parsed Root HTML Grid 제한 `fr` explicit track

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid 기본 경로](./c11-root-html-grid-runtime.md) · [fixed px gap](./c11-root-html-grid-gap.md) · [고정 named-area 경로](./c11-root-html-grid-template-areas.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [Agent Browser Chrome 156 관측, 독립 계산과 입력 hash](../spec/internal/evidence/c11-root-html-grid-fr-track-precomparison-2026-11-02/README.md) · [고정 HTML](../tests/fixtures/css/c11/root-html-grid-fr-precomparison.html) · [expected.json](../spec/internal/evidence/c11-root-html-grid-fr-track-precomparison-2026-11-02/expected.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-fr-track-plan-review-20.md) · **계획 PR diff 검토:** [30개](../spec/internal/evidence/c11-root-html-grid-fr-track-plan-pr-2026-11-02/pr-diff-review-30.md).

## 구현 전 확인한 결손

`RuntimeRootHtmlTextPaintV1`의 parsed Root HTML Grid 경로는 [`grid_values.rs`](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)의 `validate_root_html_fixed_grid_tracks()`에서 모든 explicit track을 finite non-negative `LengthPx`로 제한한다. 같은 typed projection의 일반 변환기는 `ComputedCssGridTrackSize::Fraction`을 이미 `GridTrackSize::Fraction`으로 보존하지만, 기본 Root HTML 경로에는 도달하지 못한다. [`validate_root_html_grid_box_inputs()`](../crates/spinon-style-to-layout/src/projection/style_values.rs)는 Grid box edges·gap·position·overflow 등을 제한하지만, `fr` 축이 definite CSS-pixel container size를 갖는지 아직 판정하지 않는다.

CSS Grid Layout Module Level 1, 2025 CRD [§11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)과 [§11.7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-find-fr-size)은 definite free space에서 flex factor를 기준으로 flexible track을 확장한다. §11.7.1은 flex factor 합이 1보다 작으면 1로 취급하고, 어떤 factor의 곱이 track base size보다 작으면 해당 track을 freeze하고 남은 track으로 다시 계산한다. 이 계획의 fixed-size empty items와 `LengthPx`/`Fraction` 직접 track subset은 existing Taffy Grid algorithm을 재사용한다.

## 목표와 제한 범위

기본 `RuntimeSession::new_runtime_gpu_with_root_html()` parsed HTML author-style 경로에서 finite non-negative fractional factor `fr`를 explicit Grid track에 전달한다. 기존 CSS selector·HostDocument·computed style·layout revision과 track/frame provenance는 그대로 유지하며, fixture-test constructor나 고정 scene payload를 호출하지 않는다.

- 한 Grid container, 기본 `grid-auto-flow:row`, definite explicit tracks와 implicit item 없는 visible in-flow empty block items만 허용한다.
- Explicit track list는 finite non-negative `LengthPx`와 finite non-negative `Fraction`의 조합만 받는다. 고정 px와 `fr` 혼합, `0fr`, decimal fraction, factor 합이 1보다 작은 경우, `1e38fr` 세 개와 네 개를 포함한다. 세 factor 합 `3e38`은 f32에서 유한하고, 네 factor 합 `4e38`은 f32 최댓값을 넘는다. [사전 비교](../spec/internal/evidence/c11-root-html-grid-fr-track-precomparison-2026-11-02/README.md)에서 Chrome 156이 네 factor에 0px을 반환하는 명세 차이를 확인했으므로, Taffy `find_size_of_fr`의 누산·나눗셈은 f64를 사용해 유한한 입력 비율이 f32 합 overflow로 0이 되지 않도록 한다. 각 track의 최종 CSS-pixel 결과는 기존 f32 layout 경계에 안전하게 변환할 수 있는지 검사한다.
- `fr`가 포함된 열 축은 같은 computed-style snapshot의 definite finite non-negative CSS `width: <length>`를, 행 축은 `height: <length>`를 가져야 한다. 해당 축의 computed min-size는 `auto`, max-size는 `none`, `aspect-ratio`는 미설정이어야 한다. 이 제한은 flex fraction basis를 CSS pixel로 고정한다.
- 현재 Root HTML px-gap subset은 그대로 사용한다. Gap은 해당 축의 flex free space를 계산할 때 한 번만 제외되며 track이나 item frame에 사후 보정하지 않는다.
- root/node ID, class, DOM child index를 성공 조건으로 삼지 않는다. CSS semantic mapping을 바꾸고 source order를 역전한 positive control에서도 track list는 동일하고 item frame은 현재 source order의 auto-placement를 따라야 한다.
- Existing Root HTML fixed px named-area layout은 그대로 유지한다. 이번 slice의 `fr` + `grid-template-areas` 입력은 거부한다.
- 일반 CSS Grid fixture profile, 다른 Root HTML profile, public CSS admission 또는 내부 계약 version `0.1.0`의 새 ABI를 변경하지 않는다.

## 사전 비교와 판정 기준

고정 입력, Chrome 156 observation과 독립 CSS Grid 2025 CRD 계산은 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-fr-track-precomparison-2026-11-02/README.md)에 저장한다. 사용한 Chromium은 Agent Browser가 실행한 HeadlessChrome 156.0.0.0이다. browser DOMRect의 root-relative frame은 CSS 1/64 quantization을 포함할 수 있으므로 rational 독립 기대값과 CSS frame은 최대 0.02 CSS px 이내에서 비교한다.

| scene | 독립 track 계산 | Root / item 기대 |
|---|---|---|
| `column-1-2` | `(130−10)/(1+2)` → Columns `[40,80]` | root `130×40`; items `(0,0,40,40)`, `(50,0,80,40)` |
| `column-2-1` | `(130−10)/(2+1)` → Columns `[80,40]` | root `130×40`; items `(0,0,80,40)`, `(90,0,40,40)` |
| `mixed-tracks` | `(160−30−2×10)/(1+2)=110/3` → Columns `[30,110/3,220/3]` | root `160×40`; track/frame 합이 gap 20px을 뺀 definite width를 넘지 않음 |
| `row-1-2` | `(130−10)/(1+2)` → Rows `[40,80]` | root `40×130`; items `(0,0,40,40)`, `(0,50,40,80)` |
| `subunit-sum` | factor sum `0.5`는 `1`로 처리 → Columns `[25,25]`; trailing free space 50px | root `110×40`; items `(0,0,25,40)`, `(35,0,25,40)` |
| `zero-factor` | `0fr 1fr` → Columns `[0,120]` | root `120×40`; zero-width first item, second item `(0,0,120,40)` |
| `large-factor` | `1e38fr` 세 개 → Columns `[40,40,40]` | root `120×40`; factor 합 `3e38`은 f32에서 유한한 positive control |
| `renamed-reversed` | `column-1-2`와 동일한 Columns `[40,80]` | selector·semantic source order 변경에도 layout source order를 보존 |

네 개 `1e38fr`의 독립 §11.7.1 기대는 Columns `[30,30,30,30]`이다. Chrome 156의 `[0,0,0,0]` 관측은 [별도 browser divergence](../spec/internal/evidence/c11-root-html-grid-fr-track-precomparison-2026-11-02/README.md)에 기록하며 기능 기대값으로 사용하지 않는다. Chromium `main` 소스의 float accumulator는 가능한 원인을 조사할 단서지만 Chrome 156 바이너리와 동일 revision이라는 증거가 아니다. Valid CSS인 `minmax(0px,1fr) 2fr`, fractional named areas, indefinite row `fr`는 Chrome geometry를 precomparison에 기록하지만 이 제한 slice에서는 layout scene을 publish하기 전에 각각 거부한다. Invalid CSS 선언을 positive fallback으로 오인하지 않도록 computed tracks와 runtime rejection을 별도로 검사한다.

## 구현 및 검증 순서

1. 이 계획 PR을 병합하기 전에 서로 다른 계획 실패 관점 20개와 계획 PR 변경분 별도 검토를 수행한다. 구현은 계획 PR 병합 뒤 최신 `main`에서 시작한다.
2. Root HTML Grid track validation은 finite non-negative px/fraction의 직접 track만 허용한다. `Fraction(NaN/Infinity/negative)`, auto-repeat, authored line names, percentage, `calc()`, `auto`, `min-content`, `max-content`, `minmax()`, `fit-content()`는 fail-closed한다.
3. Fraction이 있는 각 axis에 대해 exact computed grid owner의 `layout_dimensions.width`/`height`가 finite non-negative `LengthPx`인지 확인한다. 해당 축의 min-size, max-size와 aspect ratio 제한 및 style/layout revision을 같은 element snapshot에서 검사한다. Indefinite axis, percentage, math value, stale revision 또는 mismatched root는 layout projection 전에 구체 property diagnostic으로 거부한다.
4. Fixed px named-area validator를 변경하지 않는다. Template areas, track line names, auto-repeat, extra implicit items, authored numeric placement, non-default flow, nested Grid, non-empty text/image, nonzero edges, non-visible overflow, non-static position, RTL과 비-horizontal writing mode는 현재 rejection을 유지한다.
5. Rust tests는 8개 positive precomparison scene의 typed tracks·definite axis·root/item frames, Chrome/spec divergence 값 한 쌍, 3개 fixed negative controls를 확인한다. Taffy track-sizing unit test는 `1e38fr` 세 개의 `[40,40,40]` positive control과 네 개의 규범상 `[30,30,30,30]` 회귀를 검사한다. 추가 negative matrix는 negative/NaN/infinite factor, auto/intrinsic/percentage/calc/minmax/fixed-repeat, named-area+fraction, indefinite size, min/max/aspect-ratio override, stale style/root revision, implicit item overflow 및 allocation/layout failure를 거부하고 scene·partial frame·stale cached result가 발행되지 않음을 검사한다. Identity-independent selectors, reverse source order, fixed px regressions와 #1000/#1002/#1006/#1011 Root HTML regressions도 실행한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 HTML/CSS source revision을 기본 Root HTML entry로 실행한다. Source/build hashes, geometry comparison, draw/present 및 simulator screenshots를 보존한다. PixelCopy 또는 Metal drawable readback을 별도 수행하지 않으면 pixel equality를 주장하지 않는다. Android AVD software rendering과 iOS compositor 이후 미검증 경계를 기록한다.
7. 기본/제품 artifact가 opt-in scene ID·payload·FFI entry를 얻지 않는지 확인하고, scoped focused Rust tests, `cargo fmt --all -- --check`, 관련 Clippy와 두 simulator app build를 실행한다. PR check rollup이 비어 있으면 CI 통과로 기록하지 않는다.
8. 변경 코드와 runtime 결과를 계획 review와 다른 실패 관점 20개로 검토한다. 실제 구현 PR diff도 별도의 적대 관점으로 최소 20회 검토한다. 발견 결함을 수정한 뒤 영향받은 tests·matrix·review를 다시 실행한다.
9. PR에는 CSS source와 Android/iOS simulator 화면을 첨부하고 image URL을 Agent Browser에서 확인한다. Rebase and merge 뒤 `spec/internal/0059-c11-grid-layout.md`, `plan/c11-css-grid.md`, 이 계획, `spec/STATUS.md`의 C11 note를 제한 범위로 동기화한다. C11/C11.1 체크박스와 공식 완료 `72/213`은 유지하고 Tailnet preview는 상태 문서 반영 후 hash와 실제 served response를 확인해 갱신한다.

## 범위 밖과 완료 판정

이 slice는 indefinite container intrinsic flex sizing, item intrinsic contribution, `<flex>` inside `minmax()`, auto-repeat, line-name/area expansion, implicit tracks, nested Grid, general CSS parsing/admission, placement/alignment expansion, WPT runner/conformance, arbitrary writing modes, physical-device/hardware GPU, Android hardware renderer, iOS post-compositor pixel equality 또는 전체 C11.1/C11.4/C11 지원을 완료하지 않는다. 이 bounded route를 구현해도 C11.1 또는 C11 전체를 완료로 표시하지 않는다.

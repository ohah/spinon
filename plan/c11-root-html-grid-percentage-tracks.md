# C11.1 · parsed Root HTML Grid 제한 percentage explicit track

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML Grid 기본 경로](./c11-root-html-grid-runtime.md) · [fixed-px gap](./c11-root-html-grid-gap.md) · [제한 `fr` explicit track](./c11-root-html-grid-fr-track.md) · [fixed named-area path](./c11-root-html-grid-template-areas.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [독립 기대값과 Chrome 156 관측](../spec/internal/evidence/c11-root-html-grid-percentage-precomparison-2026-11-02/README.md) · [다중 case HTML](../tests/fixtures/css/c11/root-html-grid-percentage-precomparison.html) · [simulator 입력 HTML](../tests/fixtures/css/c11/root-html-grid-percentage-runtime.html) · [expected.json](../spec/internal/evidence/c11-root-html-grid-percentage-precomparison-2026-11-02/expected.json) · [Chrome observation](../spec/internal/evidence/c11-root-html-grid-percentage-precomparison-2026-11-02/chrome156-observation.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-percentage-tracks-plan-review-20.md) · **계획 PR diff 검토:** [30개](../spec/internal/evidence/c11-root-html-grid-percentage-precomparison-2026-11-02/plan-pr-diff-review-30.md) · **구현/runtime 실패 관점 검토:** [20개](../spec/internal/evidence/c11-root-html-grid-percentage-runtime-2026-11-02/implementation-runtime-review-20.md) · **구현 PR diff 적대 검토:** [30개](../spec/internal/evidence/c11-root-html-grid-percentage-runtime-2026-11-02/pr-delta-review-30.md).

**상태:** 계획 승인됨 ([PR #1018](https://github.com/ohah/spinon/pull/1018), 2026-11-02 Rebase and merge). 제한 범위의 implementation candidate를 같은 체크아웃에서 검증 중이다. 이 slice가 구현·병합되더라도 C11.1·C11 전체 완료나 공개 CSS Grid 지원으로 승격하지 않는다.

## 구현 전 확인한 결손

- `RuntimeRootHtmlTextPaintV1`의 Root HTML Grid validator [`validate_root_html_grid_tracks()`](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)는 이름 없는 direct explicit track 중 finite non-negative `LengthPx`와 `Fraction`만 받는다. `Percentage`는 아직 unsupported다.
- 같은 typed projection의 generic converter는 [`ComputedCssGridTrackSize::Percentage`](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)를 이미 `GridTrackSize::Percentage`로 변환한다. 공용 Grid engine에는 definite axes의 percentage track을 다루는 [`percentage_tracks_use_the_definite_grid_content_axes()`](../crates/spinon-layout/src/grid_tests.rs) 회귀가 있다. 이 구현은 새 CSS value parser나 새로운 Taffy track representation을 요구하지 않는다.
- Root HTML의 [`validate_fraction_axis_inputs()`](../crates/spinon-style-to-layout/src/projection/style_values/root_html_grid_fr.rs)는 `fr`이 있는 축만 definite finite non-negative CSS-pixel size인지 검사한다. percentage track은 현재 이 gate를 지나가지 않는다.
- `validate_root_html_named_area_template()`는 현재 named-area 예제를 정확한 40px/80px columns와 20px/50px rows로 제한한다. Root HTML gap gate는 기존 fixed-px gap만 받는다. 이 경로에 percentage track을 추가해도 named-area나 percentage gap은 확장하지 않는다.

[CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)은 column percentage를 grid container의 inner inline size, row percentage를 inner block size 기준으로 해석한다. Container 크기가 track에 의존하면 percentage는 intrinsic sizing 동안 `auto`로 취급된 뒤 계산된 container 크기를 기준으로 layout 단계에서 resolve된다. 이번 범위는 그 cyclic/intrinsic 경로를 구현하지 않고 definite CSS-pixel axis만 허용한다.

## 목표와 제한 범위

기본 `RuntimeSession::new_runtime_gpu_with_root_html()` author-style 경로에 finite non-negative percentage explicit track을 연결한다. Stylo computed value → Root HTML 검증 → 기존 typed projection → Taffy → layout/paint의 문서·style·layout revision identity를 유지한다.

- 한 parsed Root HTML document에서 이름 없는 explicit track만 받는다. Track 크기는 `LengthPx`, `Percentage`, `Fraction`을 직접 섞을 수 있으며 각 축의 percentage는 같은 computed-style snapshot의 해당 축 definite CSS-pixel 크기를 사용한다.
- Percentage가 있는 columns 축은 finite non-negative definite `width: <length>`를, rows 축은 `height: <length>`를 요구한다. 해당 axis의 min-size는 `auto`, max-size는 `none`, aspect-ratio는 미설정이어야 한다. `calc()`/percentage 기반 container size, intrinsic/indefinite basis, min/max override 및 aspect-ratio 유도 크기는 명시 진단으로 fail-closed한다.
- Container와 item의 border/padding, item margin, positioned/hidden/clipped item은 기존 Root HTML gate를 그대로 적용한다. 현재 zero edge 제한에서는 CSS-pixel width/height와 inner percentage basis가 같은 값이다.
- 기존 기본 row auto-placement, fixed-size empty block items, visible overflow, item 수로 implicit track을 만들지 않는 조건을 유지한다. 0%, 100%, 120% 이상 값과 definite zero-size axis를 허용하며 퍼센트 크기를 100%로 clamp하지 않는다.
- 고정 px gap은 percentage basis를 줄이지 않는다. `%`와 `fr`이 같은 axis에 있으면 existing gap을 한 번 제외한 나머지를 `fr` 분배가 사용한다. Root HTML의 percentage gap과 math-backed gap은 계속 거부한다.
- 각 percentage track의 resolved size와 해당 axis explicit fixed-track aggregate 및 gap 누산은 유한한 f32 범위여야 한다. Non-finite percentage, f32로 안전하게 표현할 수 없는 resolved track/aggregate는 Taffy layout 전에 해당 property 진단으로 거부해 partial frame을 publish하지 않는다. 유한하지만 container보다 큰 정상 percentage는 허용하고 visible overflow를 보존한다.
- `minmax()`, `fit-content()`, `calc()`, `repeat()`, auto-repeat, named lines, template areas, authored line placement, dense/column auto-flow, implicit track, nested Grid, anonymous text/image intrinsic item은 현재 rejection을 유지한다. Fixed named-area subset은 기존 px dimensions만 허용한다.
- 일반 Grid fixture profile, C11 opt-in constructor, fixture scene ID/payload 또는 내부 ABI version `0.1.0`을 변경하지 않는다.

## 기준 fixture와 expected values

고정 Chrome 입력은 [HTML fixture](../tests/fixtures/css/c11/root-html-grid-percentage-precomparison.html)다. SHA-256, Chrome 156 browser observation, screenshot 및 독립 계산은 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-percentage-precomparison-2026-11-02/README.md)에 고정했다. Chrome은 관측용이며 기대값 oracle은 CSS Grid 규범과 독립 산술 계산이다.

| 장면 | 규범상 expected geometry | 경계 |
|---|---|---|
| `both-axes` | width 240, height 120; columns `[60,120,20]`, rows `[30,60,10]`; col/row gap 10/5; origins x `[0,70,200]`, y `[0,35,100]` | 양 축의 percentage + px, gap은 basis에서 제외하지 않음 |
| `percentage-and-fr` | columns `[60,170]`, gap 10; origins `[0,70]` | 25% 이후 gap을 한 번 빼고 1fr 분배 |
| `overfull-columns` | width 200, columns `[240,20]`, gap 8; second origin 248, track end 268 | 120%와 10% 유지, overflow clipping 없음 |
| `zero-and-full-columns` | columns `[0,200]` | 0%와 100% 경계 |
| `zero-basis-columns` | definite width 0, columns `[0,0]` | 유한 non-negative definite basis의 zero 경계 |
| `indefinite-rows` | Root HTML adapter에서 거부 | CSS Grid auto/cyclic intrinsic sizing으로 fallback하지 않음 |
| `percentage-gap` | Root HTML adapter에서 거부 | 기존 fixed-px-only gap 범위 유지 |
| `percentage-areas` | Root HTML adapter에서 거부 | 기존 fixed named-area 크기 제약 유지 |

Runtime expected frames는 독립 계산값과 CSS 0.02px 이내로 비교한다. Chrome `getComputedStyle()`/`DOMRect`의 1/64 CSS px 정밀도를 감안해 exact string identity를 요구하지 않는다. Simulator와 end-to-end Root HTML 비교는 양성 `both-axes` 및 visible `overfull-columns` case만 담은 [positive-only runtime fixture](../tests/fixtures/css/c11/root-html-grid-percentage-runtime.html)를 사용한다. 두 번째 Grid의 10% track은 200px root 끝을 넘어 x=248px에 놓이며, root에 overflow clipping이 없어 화면 밖 track area 안의 8px marker를 유지한다. 나머지 positive/negative track-list matrix는 고정 multi-case precomparison fixture의 CSS와 같은 선언을 projection/runtime focused tests에 직접 옮겨 검증한다. Browser-only `spacer`와 invalid negative scene은 simulator input에 넣지 않는다.

## 구현 수락 기준

1. Existing Root HTML default session의 selector와 author CSS를 유지해 같은 source revision에서 percentage tracks, definite width/height, resolved tracks와 item frames를 검증한다. Semantic ID/class를 동등한 selector로 재작성해도 tracks가 같고, child source order를 뒤집으면 기본 placement에 따라 frame 순서만 바뀌어야 한다. NodeId/class/child index를 성공 조건으로 삼지 않는다.
2. Columns/rows independently test한다. `both-axes`, `% + px`, `% + fr`, fixed-px gap, over-100%, 0%/100%, definite zero basis를 positive matrix에 둔다.
3. Negative matrix는 indefinite/cyclic percentage axis, computed min/max/aspect-ratio override, math-backed axis size, negative/non-finite/out-of-f32 resolved result, percentage gap, percentage + named areas, `calc()`, `minmax()`, named lines/repeat, item으로 유발되는 implicit track을 확인한다. 각 오류는 property와 원인을 진단하고 아무 Grid frame도 부분 publish하지 않아야 한다.
4. Percentage basis는 CSS content box의 inner axis size를 사용한다. Existing zero edge gate를 유지하고 percentage track 계산에서 row/column gap을 basis에서 빼지 않는다. `% + fr`에서는 existing flex algorithm이 definite free space를 같은 axis에서 계산하고 gap을 두 번 빼지 않는지 확인한다.
5. 기존 Root HTML fixed-px Grid, gap, fixed named-area, anonymous text, viewport/color-scheme, visible-overflow positive/rejection regressions를 실행한다. `RuntimeBlockFlexGridTrackSizingV1` 및 다른 profile의 결과는 그대로여야 한다.
6. `spinon-style-to-layout` projection/runtime focused tests, 관련 `spinon-layout` percentage/fraction tests, `cargo fmt --all -- --check`, scoped Clippy와 diff checks를 실행한다. Default product artifact를 빌드해 새 fixture-only symbol, scene ID 또는 payload가 추가되지 않는 경계를 확인한다.
7. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 committed positive source/hash를 기본 parsed Root HTML entry로 실행한다. Platform source/build hashes, tracks, frames, revision, draw/present와 screenshots를 보존한다. PixelCopy나 Metal drawable을 직접 읽지 않으면 screenshot을 pixel equality 근거로 부르지 않는다. 물리기기/hardware GPU 및 iOS compositor 이후 pixels는 주장하지 않는다.
8. 구현/runtime failure views 20개를 계획 검토와 다른 관점으로 기록한다. 최종 구현 PR diff는 프로젝트 규칙대로 별도 30개 관점으로 검토하고 결함 수정 뒤 영향받은 검증과 review를 반복한다. Required check이 존재하면 통과 후 Rebase and merge한다. Check rollup이 없으면 CI 통과로 보고하지 않는다.
9. 병합 후 `plan/c11-css-grid.md`, 이 계획, `plan/c11-root-html-grid-runtime.md`, `spec/internal/0059-c11-grid-layout.md`, `spec/STATUS.md`를 실제 제한 범위에 맞춰 갱신한다. C11.1/C11 checkbox와 공식 `72/213`은 유지한다. Public/Tailnet roadmap을 갱신할 때 실제 served hash와 HTTP response를 확인한다.

## 범위 밖

Indefinite/cyclic percentage sizing, percentage gap, named-area + percentage combination, auto/min/max content and intrinsic sizing, percentage in `minmax()`/`fit-content()`/`calc()`, repeat/implicit placement, nested Grid, general CSS admission, WPT runner/conformance, arbitrary writing modes, physical-device/hardware GPU, iOS post-compositor pixel equality, C11.1/C11.3/C11.4 completion 및 전체 CSS Grid 지원은 이 slice의 주장이 아니다.

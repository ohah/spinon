# C11.1 parsed Root HTML `grid-auto-rows` definite percentage 목록

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [fixed-px implicit row list](./c11-root-html-grid-auto-row-list.md) · [definite percentage explicit tracks](./c11-root-html-grid-percentage-tracks.md) · [fixed implicit row 조합](./c11-root-html-grid-fr-percent-implicit-rows.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [Chrome 156 Agent Browser observation과 독립 기대값](../spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-precomparison-2026-11-02/README.md) · [다중 case HTML](../tests/fixtures/css/c11/root-html-grid-auto-row-percentage-list-precomparison.html) · [expected.json](../spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-precomparison-2026-11-02/expected.json).

**계획 검토 20개:** [실패 관점](./c11-root-html-grid-auto-row-percentage-list-plan-review-20.md) · **계획 PR diff 검토 30개:** [실제 변경 검토](../spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-precomparison-2026-11-02/plan-pr-diff-review-30.md).

## 문제와 목표

[fixed-px 목록 구현](./c11-root-html-grid-auto-row-list.md)은 기본 parsed Root HTML Grid에서 direct finite non-negative px `grid-auto-rows` 목록을 첫 implicit row부터 순환한다. Root HTML explicit rows에는 이미 definite-axis percentage가 허용되고, 단일 fixed implicit row profile도 explicit percentage/fr track과 조합된다. 현재는 definite root height가 있어도 implicit row list의 percentage entry를 받을 수 없다.

기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 경로에서 non-empty direct px/percentage implicit-row 목록을 제한적으로 허용한다. 각 direct percentage는 Root HTML Grid의 definite inner block size를 basis로 계산한다. 기존 typed CSS cascade → Root HTML validation → `GridContainerInput` → Taffy → layout/paint의 document/style/layout revision identity와 row-sparse placement를 유지한다.

기준 동작은 [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)과 implicit track sizing의 목록 순환 규칙이다. Chrome 156은 독립 관측자다. 독립 expected values는 고정 CSS size·percentage multiplication·row gap·source-order placement로 계산하며 관측 JSON과 대조했다.

## 수락 범위

- `RuntimeRootHtmlTextPaintV1` 기본 parsed Root HTML route에서만 non-empty `grid-auto-rows` TrackList의 direct finite non-negative `LengthPx`와 `Percentage`를 받는다. 단위와 목록 순서는 유지하고 첫 implicit row부터 순환한다. 기존 단일 `LengthPx` 표현을 바꾸지 않는다.
- percentage가 하나 이상 있으면 root height가 finite non-negative definite CSS-pixel size여야 한다. root의 `min-height`, `max-height`, aspect-ratio, math-backed height 및 기존 Root HTML box-edge 제약이 percentage basis를 바꾸는 경우는 거부한다. definite 0px height는 valid zero basis다.
- percentage는 grid container inner block size의 백분율로 계산한다. row-gap은 basis에서 차감하지 않는다. 값이 100%보다 커도 clamp하지 않고 기본 visible overflow frame을 보존한다.
- explicit row track, 기존 fixed-px gap, row-sparse auto-placement, item source order, hidden child exclusion과 기존 fixed-px implicit-row 동작은 유지한다. percentage gap은 계속 거부한다. 기존 explicit px/%/fr tracks와 새 px/% implicit list의 조합도 focused tests에서 확인한다.
- 목록은 모든 direct item과 explicit grid 용량을 적용한 후 필요한 implicit row 수에 맞춰 순환한다. list 길이와 expanded row 수는 `GRID_MAX_TRACKS`를 넘지 않으며 값·resolved track·gap을 포함한 누적 extent가 유한 f32 layout 범위인지 배치와 frame publication 전에 검사한다. allocation failure 또는 unsupported input에서는 partial frame을 publish하지 않고 같은 worker의 다음 valid request가 성공해야 한다.
- CSS syntax-invalid 선언의 cascade fallback과 syntax-valid percentage가 indefinite axis/profile 제약으로 거부되는 경우를 별도로 검증한다.

## 독립 수치 oracle

입력과 Chrome raw computed values/DOMRect는 [precomparison evidence](../spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-precomparison-2026-11-02/README.md)에 고정했다. Root HTML runtime fixture는 구현 PR에서 단일 root 입력으로 추가하며 이 다중 root browser fixture를 제품 runtime input으로 재사용하지 않는다.

| Case | Definite root height | Explicit row | `grid-auto-rows` | Expanded implicit rows | Row starts (root-relative) | 판정 |
|---|---:|---:|---:|---|---|---|
| `cycle` | 100px | 20px | `10% 20%` | `[10,20,10,20]px` | `[0,25,40,65,80]px` | 2-column source order, root height 안에서 네 implicit row 순환 |
| `mixed` | 120px | 20px | `15px 25%` | `[15,30,15,30]px` | `[0,25,45,80,100]px` | percentage basis는 120px; gap을 빼지 않아 content가 root보다 10px 넘침 |
| `bounds` | 100px | 20px | `0% 120%` | `[0,120]px` | `[0,25,30]px` | 0px row 및 root 아래까지의 visible overflow 50px 보존 |
| `zero` | 0px | 0px | `50%` | `[0]px` | `[0,5]px` | definite zero basis도 0px implicit row로 resolve; fixed-size item은 track 밖으로 visible overflow |

Geometry 비교 허용치는 0.02 CSS px다. `getComputedStyle().gridAutoRows`, used `gridTemplateRows`, root frame와 item frame을 별도로 저장한다. Chrome 캡처는 visual observation으로만 사용한다.

## 구현 전/구현 후 검증

1. 이 계획 PR에는 위 고정 input, raw observation, independent expected values와 별도 계획 review 20개를 포함한다. PR diff에는 별도의 30관점 검토를 기록한다.
2. 구현 projection tests는 px-only scalar/list regression, percentage 단독/혼합 순환, explicit percentage/fr 조합, zero/120% 경계, definite zero axis, indefinite/minmax/aspect-ratio/math-backed axis, invalid/negative/non-finite/f32-overflow entry, area/gap/auto-flow unsupported, resource ceiling과 allocation recovery를 다룬다.
3. `spinon-layout` tests는 fixed/percentage list resolution, first implicit index, explicit rows 비순환, gap basis 분리, expanded track cap, aggregate f32 extent overflow 및 no-partial-frame을 확인한다. CSS syntax-invalid fallback과 typed-but-unsupported percentage rejection을 구분한다.
4. 기본 parsed Root HTML runtime은 same-source single-grid positive fixture를 실행한다. `cycle`, 혼합 목록, over-100% visible-overflow, semantic ID/class rename과 reversed source order, style/environment/layout revision identity, draw/present와 platform source/build/input hashes를 확인한다. 여러 grid case가 있는 precomparison fixture는 사용하지 않는다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 committed positive input hash를 기본 route로 실행한다. 화면은 visual capture다. PixelCopy/Metal readback을 실제 수집한 경우에만 표면 evidence를 별도 보고하고 screenshot을 pixel equality로 부르지 않는다. physical device, hardware GPU, iOS post-compositor pixels, WPT 및 전체 CSS conformance는 별도다.
6. 변경된 `spinon-style-to-layout` projection/runtime, `spinon-layout` Grid, Root HTML runtime focused suites와 scoped Clippy, `cargo fmt --all -- --check`, diff check를 실행한다. Default product artifact에서 fixture-only ABI/payload가 나오지 않는 경계도 확인한다. 실행하지 않은 검사나 CI는 성공으로 보고하지 않는다.
7. 구현/runtime 실패 관점 20개는 계획 관점과 다른 목록으로 검토한다. 실제 구현 PR diff는 프로젝트 규칙대로 별도의 30개 관점으로 검토한다. 결함 수정 시 영향 검증과 해당 리뷰 관점을 다시 실행한다.
8. 병합 후 C11 plan, Root HTML Grid plan, contract 0059 `0.1.0`, `spec/STATUS.md`, public roadmap 및 Tailnet preview를 확인한다. 제한 slice만 기록하고 C11.1/C11 checkbox와 공식 `72/213`은 유지한다.

## 구현 결과 · 병합된 제한 slice · PR #1047 · 2026-11-02

구현 PR [#1047](https://github.com/ohah/spinon/pull/1047)은 2026-11-02T09:10:09Z Rebase and merge됐고 merge SHA는 `bc3430ac664e76a2f246bcbfc387f61121017bb6`다. GitHub PR check rollup은 비어 있었고 `main` required status checks도 설정되지 않아 CI 통과로 주장하지 않는다. 자세한 내용은 [병합 기록](../spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/merge-PR1047.md)을 참조한다.

기본 parsed Root HTML `grid-auto-rows`에서 direct px/percentage scalar·list를 허용하는 구현을 추가했다. CSS typed cascade → Root HTML projection → `GridContainerInput` → Taffy used geometry까지 연결했다. fixed px 표현과 기존 gap/placement 경계를 유지하며 definite 0px, 100% 초과 visible overflow, mixed list, explicit `%`/`fr` 조합을 확인했다. indefinite/math-backed height, non-auto min/max·aspect ratio, percentage gap, unsupported track, resource/f32 overflow는 실패한다.

- [Android/iOS build·runtime·artifact hash·한계](../spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/README.md)
- [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-root-html-grid-auto-row-percentage-list-runtime-2026-11-02/implementation-runtime-review-20.md)
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML app route에서 같은 입력 hash가 `layout=ready boxes=13` 및 13-box draw/present로 끝났다. Screenshots는 visual capture이고 surface pixel readback은 하지 않았다.
- cascade 7/7, layout 14/14, style-to-layout Root HTML 26/26, parsed Root HTML integration 4/4, 관련 crate scoped all-target Clippy, rustfmt 및 diff 검사 통과.
- 이 제한 candidate는 C11.1/C11 완료, 일반 CSS admission, WPT 적합성 또는 공식 완료 수를 변경하지 않는다. 공식 `72/213`과 체크박스는 그대로 둔다.

## 제외 범위

Indefinite/cyclic percentage row sizing, intrinsic/min/max-content, `auto`/`fr`/`minmax()`/`fit-content()`/`calc()`/auto-repeat, named lines, named-area + implicit row 결합, percentage gap, implicit columns, column/dense flow, authored placement, nested Grid, intrinsic text/image contribution, general CSS admission, WPT runner/conformance, arbitrary writing modes, physical hardware GPU 및 C11 전체 완료는 이번 계획 범위가 아니다.

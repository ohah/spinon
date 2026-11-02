# C11.1 parsed Root HTML named areas × percentage tracks

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [parsed Root HTML percentage tracks](./c11-root-html-grid-percentage-tracks.md) · [parsed Root HTML named areas](./c11-root-html-grid-template-areas.md) · [named areas × fractional tracks](./c11-root-html-grid-fr-named-areas.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**현재 상태:** bounded projection/runtime 구현과 Chrome 156 사전 비교를 마치고 [PR #1035](https://github.com/ohah/spinon/pull/1035)로 2026-11-02T05:24:52Z Rebase and merge됐다(merge SHA `4b921f3e8ccd86e7e156562bb7e5b4a464f082f2`). Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 네 percentage named-area source를 각각 실행해 4/4 scene `layout=ready`와 draw/present, 화면 캡처를 확인했다. 구현 commit [`1e73158`](https://github.com/ohah/spinon/commit/1e73158e945f2a9aa6d8b16718435d3a6497cc51) 이후 검토에서 min/max 위반 진단을 실제 속성으로 고치는 수정이 더해졌다. 이 제한 slice로 C11.1/C11 완료, 일반 CSS Grid 지원 또는 공식 `72/213` 집계를 바꾸지 않는다. [실행 근거](../spec/internal/evidence/c11-root-html-grid-area-percentage-runtime-2026-11-02/README.md) · [구현/runtime 검토 20개](../spec/internal/evidence/c11-root-html-grid-area-percentage-runtime-2026-11-02/implementation-runtime-review-20.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-root-html-grid-area-percentage-runtime-2026-11-02/pr-diff-review-30.md).

**사전 비교:** [Chrome observation과 독립 계산](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/README.md) · [expected.json](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/expected.json) · [SHA-256 manifest](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/hashes.txt)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-area-percentage-plan-review-20.md) · **계획 PR diff 검토:** [20개](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/plan-pr-diff-review-20.md)

## 해결한 결손과 남은 제한

기본 parsed Root HTML named-area route는 이제 정확한 2-track columns/rows, 기존 `header/header`·`side/content` rectangle map 및 세 semantic item을 유지하면서 finite non-negative direct `px`/`%` 조합을 받는다. 각 축에 percentage basis가 필요할 때 definite CSS-pixel width/height를 검증하고, `min-size:auto`, `max-size:none`, `aspect-ratio` 미지정과 finite f32 track/aggregate 범위를 유지한다. `fr`+`%`, 다른 area map, 계산식·intrinsic·implicit track은 계속 거부한다. 네 positive scene과 제한 rejection matrix는 [runtime evidence](../spec/internal/evidence/c11-root-html-grid-area-percentage-runtime-2026-11-02/README.md)에 있다.

후속 검토에서 named-area candidate의 네 min/max 조건이 모두 `min-width` 오류로 보고되던 진단 결함을 수정했다. 네 조건은 각각 실제 속성을 반환하고, runtime test는 cascade diagnostic 또는 computed-value diagnostic에서 위반 속성이 드러나는지 검사한다.

## 독립 기준과 고정 입력

기준 명세는 [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes) 및 [§7.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-template-areas)다. 사전 비교는 Agent Browser HeadlessChrome 156에서 직접 실행했으며, computed track 문자열·DOMRect와 screenshot을 독립 산술에 대조했다.

모든 scene은 definite `120×70px` content box, zero gap, area map `"header header" "side content"`, 세 semantic item을 사용한다. DOM child source order는 `content`, `header`, `side` 순서라 class/node order를 area 위치로 혼동하지 않는지도 본다.

| Scene | Explicit columns | Explicit rows | 기대 columns / rows (px) | 기대 item frame (px) |
|---|---|---|---|---|
| `both-axes` | `25% 75%` | `20% 80%` | `[30,90]` / `[14,56]` | header `[0,0,120,14]`; side `[0,14,30,56]`; content `[30,14,90,56]` |
| `mixed-px-percent` | `40px 50%` | `20% 50px` | `[40,60]` / `[14,50]` | header `[0,0,100,14]`; side `[0,14,40,50]`; content `[40,14,60,50]` |
| `overflowing-columns` | `120% 10%` | `20% 80%` | `[144,12]` / `[14,56]` | header `[0,0,156,14]`; side `[0,14,144,56]`; content `[144,14,12,56]` |
| `zero-track` | `0% 100%` | `20% 80%` | `[0,120]` / `[14,56]` | header `[0,0,120,14]`; side `[0,14,0,56]`; content `[0,14,120,56]` |

The independent values are recorded in [`expected.json`](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/expected.json); Chrome results are separately recorded in [`chrome-observation.json`](../spec/internal/evidence/c11-root-html-grid-area-percentage-precomparison-2026-11-02/chrome-observation.json). The existing fixed-px named-area and both-axis `fr` pages are regression controls, not output from this new matrix.

## 구현 경계

- Extend only the default parsed `RuntimeSession::new_runtime_gpu_with_root_html()` route. Do not route through C11.3.3/C11.4 fixture constructors, add a fixture scene ID, or change product ABI.
- Preserve the current exact rectangular 2×2 area map and the three direct empty block items selected by computed `grid-area`. Keep zero gap, default sparse row flow, LTR/horizontal writing mode, visible overflow, zero edges, and definite `120×70px` root.
- Add a named-area combination that permits exactly two direct explicit tracks on each axis. In the new percentage path, tracks may be finite non-negative direct `px` and `%` values, mixed per axis. When the map uses no percentage, preserve the existing fixed-px and exact `1fr 2fr` paths. Do not admit `fr` and `%` in the same named-area input in this slice.
- Reuse the existing root percentage-axis checks: columns require a definite finite non-negative width basis, rows require a definite finite non-negative height basis; min-size stays `auto`, max-size `none`, aspect-ratio unset. Resolved track lengths and track aggregates must remain finite and within the existing f32 boundary.
- Keep fixed-px gap behavior unchanged and use `gap:0` in the new matrix so percentage basis, named-area placement, and gap distribution are not conflated. Keep `grid-auto-rows`/`grid-auto-columns:auto` and no implicit tracks.
- A negative authored percentage track is CSS-invalid and must not be treated as a computed zero or as evidence that negative values are supported. The parser/cascade and runtime admission must still fail closed when invalid declarations leave this route without the exact explicit tracks.
- Do not clamp finite percentages to 100%. The overflow case must retain the `[144,12]` columns and place the blue content item at x=144px outside the 120px root. Do not clip or clamp that frame in layout or screenshot processing.
- Keep invalid/unlisted values, indefinite/cyclic percentage axes, computed min/max/aspect-ratio overrides, `calc()`, `minmax()`, `fit-content()`, `repeat()`, named lines, unsupported area maps, missing/duplicate area items, child mutation and stale revisions fail-closed before publishing a partial scene.

## 수락 기준과 구현 결과

1. Four fixed source pages and independent expected geometry are in the Root HTML projection/runtime matrix. Tests assert area map, typed tracks, definite basis, root/item frames and same revision; tolerance is 0.02 CSS px.
2. Rejection coverage includes indefinite axes, `%` plus named-area `fr`, `calc()`/`minmax()`, percentage gap, all four min/max overrides, aspect-ratio, negative percentage and no-partial-publication. Existing Root HTML percentage tests cover f32 range overflow. Existing area topology and stale revision regressions remain enabled.
3. named-area percentage runtime positive 행렬은 4/4이고 제한 rejection matrix도 통과했다. fixed-px 및 양축 `1fr 2fr` named-area 회귀, Root HTML Grid 24/24, Root HTML projection 15/15, C11.3.3 projection 26/26, scoped Clippy와 rustfmt가 통과했다. geometry test와 명령 출력은 runtime evidence에서 확인할 수 있다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 source commit의 네 positive scene을 실행했고 scene별 HTML hash가 일치한다. 두 환경 모두 layout ready와 draw/present를 기록했다. root/item frame은 app log나 screenshot 추정이 아니라 Rust snapshot 검사로 확인했다. Android는 SwiftShader이며 PixelCopy, iOS Metal/compositor pixel readback은 실행하지 않았다. Android overflow 장면의 APK hash는 당시 저장하지 못한 점을 기록했다.
5. Android/iOS Simulator 빌드가 성공했다. FFI 심볼, fixture adapter, fixture scene ID, 제품 ABI 또는 fixture payload를 바꾸지 않아 fixture archive-boundary 재빌드는 해당되지 않는다. Android Gradle compileSdk 및 iOS duplicate-symbol/dSYM/AppIntents 경고는 build log에 보존했다.
6. [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-root-html-grid-area-percentage-runtime-2026-11-02/implementation-runtime-review-20.md) 검토에서 발견한 min/max 진단 오류를 수정하고 관련 property assertions를 재검사했다. 실제 PR diff의 독립 검토는 PR 준비 후 별도 30개로 기록한다.
7. 이 계획, 상위 C11 계획, Root HTML runtime 계획, 내부 계약 0059와 상태 대장을 갱신했다. 구현 PR #1035는 `4b921f3e8ccd86e7e156562bb7e5b4a464f082f2`로 병합됐다. C11.1/C11 완료 체크와 공식 `72/213`은 그대로다.

## 이 slice로 완료를 주장하지 않는 범위

This does not support arbitrary CSS Grid syntax, named-area auto-placement beyond the fixed three items, named areas combined with `fr`+`%`, intrinsic or indefinite percentage sizing, percentage gap, general implicit tracks, nested/text/image content, WPT runner/conformance, public CSS admission, full C11.1/C11.3.3/C11.4, Android physical devices/hardware GPU, or iOS post-compositor pixel equality.

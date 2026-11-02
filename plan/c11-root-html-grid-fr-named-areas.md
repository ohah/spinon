# C11.1 parsed Root HTML Grid named areas × fractional tracks

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [parsed Root HTML Grid 기본 경로](./c11-root-html-grid-runtime.md) · [named-area 경로](./c11-root-html-grid-template-areas.md) · [fractional-track 경로](./c11-root-html-grid-fr-track.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**현재 상태:** 계획 단계. 현재 Root HTML named-area validator는 고정 `40px 80px` columns와 `20px 50px` rows만 허용한다. 별도 Root HTML `fr` slice는 definite axis에서 direct finite non-negative fraction을 지원하지만 template areas와 함께 쓰면 거부한다. 재현과 기준은 [Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-root-html-grid-fr-named-areas-precomparison-2026-11-02/README.md)다.

## 확인한 차이와 목표

`RuntimeSession::new_runtime_gpu_with_root_html()`의 기본 parsed HTML 경로는 기존 제한 named-area 배치와 definite-axis fractional tracks를 각각 처리한다. 현재 두 기능의 조합은 `validate_root_html_named_area_template()`가 named-area tracks를 고정 px 쌍에 묶어서 `grid-template-areas` 진단으로 거부한다. 기존 fractional-axis 검증은 `grid-template-rows|columns`의 definite CSS-pixel basis, min/max override와 `aspect-ratio`를 별도로 검증한다.

이 계획은 기존 2×2 `header/side/content` area map에 direct `1fr 2fr` track 쌍을 양 축에서 추가한다. CSS 문자열 재파싱·Taffy 결과 사후 보정·새 FFI/scene ID는 추가하지 않는다. root와 세 빈 direct block item, 기본 sparse row flow, `gap:0`, `120×70px` definite root, visible overflow, LTR/horizontal writing mode를 유지한다. 기존 fixed-px area 입력은 같은 결과로 계속 통과해야 한다.

## 기준과 독립 기대값

CSS Grid Layout Module Level 1 2025 CRD의 [named areas §7.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-template-areas), [fractional tracks §11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks), [find the size of an fr §11.7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-find-fr-size)를 기준으로 한다. 두 definite axis의 free space를 각 axis의 `1fr:2fr` 비율로 나눈다. 독립 기대값은 열 `[40,80]`, 행 `[70/3,140/3]`이다.

고정 입력과 Agent Browser가 실행한 HeadlessChrome 156 observation은 [사전 비교 README](../spec/internal/evidence/c11-root-html-grid-fr-named-areas-precomparison-2026-11-02/README.md), [expected.json](../spec/internal/evidence/c11-root-html-grid-fr-named-areas-precomparison-2026-11-02/expected.json), [원본 HTML](../spec/internal/evidence/c11-root-html-grid-fr-named-areas-precomparison-2026-11-02/source.html), [화면](../spec/internal/evidence/c11-root-html-grid-fr-named-areas-precomparison-2026-11-02/chrome-agent-browser.png)에 고정한다. Chrome의 `DOMRect`는 1/64 CSS px 격자에 양자화되어 각 frame 성분은 독립 기대값과 0.02 CSS px까지 비교한다.

| 요소 | 독립 geometry (CSS px) | Chrome 156 관측 (CSS px) |
|---|---|---|
| root | `[0,0,120,70]`, columns `[40,80]`, rows `[70/3,140/3]` | `[0,0,120,70]`, `40px 80px`, `23.328125px 46.671875px` |
| header | `[0,0,120,70/3]` | `[0,0,120,23.328125]` |
| side | `[0,70/3,40,140/3]` | `[0,23.328125,40,46.671875]` |
| content | `[40,70/3,80,140/3]` | `[40,23.328125,80,46.671875]` |

## 구현 수락 조건

1. Default Root HTML projection에서 현재 고정-px named-area 입력과 이 계획의 named-area + two-axis `1fr 2fr` 입력만 허용한다. Existing fixed-area exactness를 단순히 삭제하거나 모든 named-area track syntax로 넓히지 않는다.
2. Fraction-bearing axes는 기존 `validate_fraction_axis_inputs()`를 계속 거쳐 같은 computed container의 finite definite `width`/`height`, initial min/max constraints, unset aspect ratio를 만족해야 한다. 값이 비례 배분을 보장하지 않는 indefinite, percentage/math basis, min/max override, aspect-ratio는 구체 computed property로 fail-closed한다.
3. 2×2 area map은 `"header header" "side content"`이며 semantic item은 각 area 하나씩이다. 기존 area resolver/projection이 computed `grid-area`를 사용한다. ID·class·child index에 의존하지 않고 renamed selector와 source reorder control을 검사한다.
4. Fixed px baseline은 기존 expected frames를 유지한다. 신규 positive source는 `[1fr,2fr]` 두 축의 track metadata·root/item frames·document/style/layout revision·draw/present를 검사한다. 각 결과는 Chrome observation과 독립 분수 계산에 0.02 CSS px 이내로 일치한다.
5. Negative controls는 wrong track factor/count, percentage or mixed unsupported basis, indefinite axis, min/max/aspect-ratio override, `minmax()`, named-area dimension/shape mismatch, missing/duplicate/extra area item, unsupported placement/flow, nonzero gap/edges, nested/text/replaced items, non-visible overflow 및 failed layout의 partial scene 부재를 검사한다.
6. 동일 parsed HTML fixture를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 앱 runtime으로 실행한다. Geometry와 draw/present를 기록한다. PixelCopy·Metal drawable readback을 수행한 경우에만 pixel 결과를 별도 주장하고, simulator screenshot만으로 surface equality를 말하지 않는다.
7. Root HTML fixed-px named-area, bounded `fr`, percentage, fixed-px gap, implicit rows, anonymous-text, viewport/color-scheme, C11.3.3 placement regressions를 실행한다. Package/default artifact에 fixture-only symbol/payload, 새로운 ABI 또는 고정 scene ID가 없는지 확인한다. 숫자 contract `0059` 버전은 `0.1.0`으로 유지한다.
8. 구현/runtime 결과를 독립 실패 관점 20개, 구현 PR diff를 별도 30개 관점으로 기록한다. 관련 형식 검사·focused tests·scoped Clippy·Android/iOS simulator build를 실행하고, PR check run이 비어 있으면 CI 통과로 표현하지 않는다.
9. 병합 후 실제 범위와 한계를 `spec/STATUS.md`, `plan/c11-css-grid.md`, 내부 README 및 공개 roadmap에 반영한다. C11.1/C11.3.3/C11 상위 checkbox와 공식 완료 `72/213`은 바꾸지 않는다.

## 제외와 중단 조건

Percentage + area, mixed px/fr + area, arbitrary factors/track count, named lines/repeat, implicit tracks, auto-placement 확장, intrinsic `auto`/`minmax()`/`fit-content()`, image/text contributions, arbitrary area templates, general CSS admission, WPT conformance, RTL/writing modes, nested Grid, surface-pixel equality, physical-device/hardware GPU validation은 이 계획 범위에 없다. Taffy 또는 Chrome geometry가 독립 §11.7 계산과 허용 오차 밖으로 다르면 validator를 느슨하게 하지 않고 구현을 중단해 최소 재현을 보강한다.

## 계획 전 검토

- [계획 실패 관점 20개](./c11-root-html-grid-fr-named-areas-plan-review-20.md)
- [계획 PR diff 적대 검토 30개](../spec/internal/evidence/c11-root-html-grid-fr-named-areas-plan-pr-2026-11-02/pr-diff-review-30.md)

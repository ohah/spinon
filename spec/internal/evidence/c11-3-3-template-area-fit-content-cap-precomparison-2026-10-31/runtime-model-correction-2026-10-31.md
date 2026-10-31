# C11.3.3 area fit-content track 분류 정정

## 발견

Chrome 156 precomparison은 `grid-template-areas: "fixed fit"`, authored `grid-template-columns: 20px`, `grid-auto-columns: fit-content(40px)`를 고정했다. 초기 구현 계획은 두 번째 area-created column을 implicit track으로 잘못 분류했다.

CSS Grid Level 1 §§7.1, 7.5–7.6은 explicit grid의 크기를 `grid-template-areas` 행렬과 row/column template track 수 중 큰 값으로 정한다. Area가 만든 explicit track 중 `grid-template-columns` 또는 `grid-template-rows`로 size가 지정되지 않은 track은 해당 `grid-auto-*` 값으로 sizing한다. 이 fixture의 2-column explicit grid 안에 두 item을 배치하므로 grid 밖 placement나 positive implicit track은 없다. [공식 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/).

## 런타임에서 관찰한 값

Android API 37 ARM64 AVD에서 새 scene은 MainActivity route를 통과해 Rust runtime까지 도달했다. 첫 검증은 실패했으며, 보존한 runtime diagnostic은 다음을 보여준다.

- `GridPlacementMode::TemplateGrid`
- typed columns `[LengthPx(20), FitContent(LengthPx(40))]`
- typed rows `[LengthPx(60)]`
- typed `grid-auto-columns: fit-content(40px)`와 1×2 template areas
- track trace `20px 40px`, explicit 2, positive implicit 0

이 구조는 위 CSS Grid 정의에 맞는다. 당시 assertion이 `TemplateGridImplicitTracks`, explicit 1 / positive implicit 1을 요구했기 때문에 실패했으며, 이는 구현 pass가 아니다. 전체 layout geometry, item intrinsic source와 surface pixels은 그 실행에서 검증되지 않았다.

- 실패 실행: [Android API 37 AVD runtime diagnostic](../c11-3-3-template-area-fit-content-cap-runtime-2026-10-31/android-avd-c1133-fitcap-android-avd-20261031T092319339Z-c55eda2a/runtime.log)
- Chrome 기준: [관찰 JSON](chrome/observation.json), [독립 기대값](expected-values.json)

## 수정한 기대값

Fixture assertion은 `TemplateGrid`, columns `[20px, fit-content(40px)]`, explicit track 2개와 positive implicit track 0개를 검사한다. Chrome used columns `20px 40px`, Ahem intrinsic 20px/110px, root `[0,0,200,60]`, fit item `[20,0,40,60]`, 세 경계 픽셀 표본을 계속 검증한다. 이 fixture는 CSS Grid의 일반 implicit placement나 전체 `fit-content()` 지원을 검증하지 않는다.

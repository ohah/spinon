# C11.1 parsed Root HTML intrinsic Grid track과 template area

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [기본 Root HTML intrinsic track sizing](./c11-root-html-intrinsic-grid-tracks.md) · [template area](./c11-3-template-repeat-named-lines-areas.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 비교:** [Chrome 156 Agent Browser 행렬](../spec/internal/evidence/c11-root-html-intrinsic-areas-precomparison-2026-11-05/README.md) · [고정 fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-areas-precomparison.html) · [browser observation](../spec/internal/evidence/c11-root-html-intrinsic-areas-precomparison-2026-11-05/chrome156-observation.json) · [독립 구조 기대값](../spec/internal/evidence/c11-root-html-intrinsic-areas-precomparison-2026-11-05/expected-invariants.json)

## 결손

기본 `RuntimeRootHtmlTextPaintV1`에는 explicit intrinsic track과 template area를 각각 처리하는 제한 경로가 있지만, 두 기능을 조합하는 경로는 닫혀 있다. intrinsic-track 계획은 area 조합을 명시적으로 거부한다. Cascade의 typed auto-track 선택도 area가 있을 때 intrinsic 값을 허용하지 않고, style-to-layout은 area profile을 먼저 고르며, layout intrinsic-track validator도 `template_areas`를 거부한다. 각 계층이 같은 지원 경계를 사용해야 named-area placement를 기존 Grid sizing 알고리즘까지 전달하면서 area map을 보존하고 partial layout을 막을 수 있다.

고정 fixture는 HeadlessChrome 156에서 네 조합을 관찰한다: min-content 열, intrinsic `minmax()`, 두 intrinsic 열에 걸친 named area, min/max-content 행. 텍스트 폭은 browser/font 측정값이다. area 구조와 span 기대값은 독립 불변 조건으로 두고, 런타임 geometry는 0.5 CSS px 허용 오차로 비교한다.

## 구현 계약

- 별도 fixture-only 성공 mode나 일반 CSS admission을 만들지 않고, 기존 기본 Root HTML intrinsic-track 경로가 직사각형 `grid-template-areas` map을 유지하고 검증하게 한다.
- 기존 area projection으로 각 named `grid-area`를 해석하고, 계산된 명시 row/column line과 span을 보존한다. intrinsic sizing 알고리즘에는 Chrome 기준의 placement가 전달되어야 한다.
- 이미 허용된 intrinsic track 문법과 definite 크기 basis를 유지하고, 두 축 및 여러 track에 걸친 item 기여를 확인한다.
- parsed Root HTML 허용 범위는 visible Grid 하나, positive definite CSS px 크기, 두 explicit track 목록과 일치하는 고유·비중첩 area map, named area마다 text-only child를 가진 직계 HTML `div` 하나로 제한한다. 알 수 없는 area 이름과 잘못된 area 선언 뒤의 named-line fallback은 원자적으로 거부한다.
- invalid/non-rectangular area map, unknown area, intrinsic auto-repeat/implicit track, named-line metadata, column/dense flow, direct-text aspect-ratio mode, overflow 및 revision mismatch는 계속 fail-closed한다. 기존 제한상 `minmax(auto, px|%)` named-area 조합도 지원하지 않는다.
- area-only/fixed-track 및 no-area/intrinsic 경로의 기존 동작을 보존한다. 기본 artifact에는 fixture-only ID나 payload를 추가하지 않는다.

## 검증 계획

1. 고정 matrix와 독립 area/span 불변 조건을 cascade, projection, layout, 기본 Root HTML runtime 회귀에 연결한다. spanning 장면은 각 track 경계가 드러나는 빈 area item을 둬 총 span 폭만 같고 track 기여가 잘못되는 경우를 잡는다.
2. malformed area 직사각형, unresolved area 이름, 지원 밖 track/profile 조합, quota 초과 입력의 거부를 확인하고 partial scene이 publish되지 않는지 검사한다.
3. focused Rust tests, 기존 Root HTML Grid 회귀, `cargo fmt --all -- --check`, scoped Clippy, 기본 artifact 경계 검사, Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 앱 빌드·실행을 기록한다.
4. geometry를 Chrome과 0.5 CSS px 이내에서 비교한다. draw/present와 PixelCopy/Metal readback을 구분하고, 시뮬레이터 근거를 hardware GPU나 compositor 이후 픽셀 근거로 표현하지 않는다.
5. 구현/runtime 실패 관점 20개와 실제 구현 PR diff의 별도 공격 관점 30개를 기록하고, 수정 후 관련 항목을 다시 확인한다.
6. 병합된 범위만 [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md), [공식 STATUS](../spec/STATUS.md), 공개 roadmap에 반영한다. 이 slice는 C11.3.3, C11.4, C11, WPT 적합성 또는 일반 CSS Grid 지원을 완료하지 않는다.

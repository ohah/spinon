# C11.1 · parsed root HTML author CSS Grid runtime integration

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [공식 상태](../spec/STATUS.md#c11-css-grid) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [레이아웃 계약 0009](../spec/internal/0009-layout-engine.md)

**구현 전 비교:** [고정 HTML source와 독립 기대값](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/README.md) · [Chrome 156 관측](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/chrome-observation.json) · [expected.json](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/expected.json)

**상태:** 사전 비교를 끝냈다. 아직 구현, Rust test, app build, Android/iOS 실행은 하지 않았다. 이것은 root HTML author CSS가 기존 C11.1 Grid input을 실제 default runtime entry에서 사용하는 계획이다. C11/C11.4 공식 완료 또는 `72/213` 수를 바꾸지 않는다.

**계획 검토:** [계획 실패 관점 20개](./c11-root-html-grid-runtime-plan-review-20.md) · [이번 계획 PR diff 적대 검토 20개](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/plan-pr-diff-review-20.md)

## 확인된 결손

- Parsed root HTML의 default runtime은 `RuntimeCalculationProfile::RootHtmlDefaultRender`에서 `RuntimeRootHtmlTextPaintV1` computed-style profile을 선택한다. 관련 경로는 [calculation.rs](../crates/spinon-runtime/src/session/ua_cascade/calculation.rs)다.
- [root HTML renderer](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/root_html_default_render.rs)는 visible tree의 `display:block`·`display:inline`만 받아 Grid container를 거부한다.
- C11.1 typed Grid style/layout 경로는 존재하지만 session에선 [runtime session](../crates/spinon-runtime/src/session.rs)의 `RuntimeCssProfile::BlockFlexGridFixture` 같은 profile별 entry로 호출된다. C11.4 track sizing entry도 별도 fixture profile로 노출된다. 이로 인해 일반 parsed root HTML의 `<style>` 또는 inline author declaration은 현재 Grid layout 경로까지 연결되지 않는다.

## 목표와 수락 기준

실제 root HTML runtime entry에서 author CSS `display:grid`를 Stylo cascade, typed Grid projection, Taffy layout과 runtime paint까지 전달한다. Grid root나 child를 NodeId, scene ID, class 이름, DOM 위치로 성공 선택하지 않는다. 일반 문서의 root id, child id, class 및 source order를 바꾼 입력도 같은 CSS 결과여야 한다.

고정 precomparison source는 [fixture](../tests/fixtures/css/c11/root-html-grid-author-precomparison.html)다. 독립 산술과 Chrome 156 관측은 다음과 같다.

| 대상 | 기대 geometry |
|---|---|
| `main` | `(0, 0, 120, 70)` CSS px; columns `[40, 80]`; rows `[20, 50]` |
| `.item-a` | `(40, 0, 80, 20)` CSS px; column 2, row 1 |
| `.item-b` | `(0, 20, 40, 50)` CSS px; column 1, row 2 |

### 구현 수락 기준

1. root HTML parser가 `<meta name="viewport">`·inline `<style>`·body children을 같은 document/style/layout revision으로 전달한다. 새 code path는 기존 RootHtml runtime entry에서 호출되며 C11 fixture-test constructor에 종속되지 않는다.
2. positive scene source는 `NodeId`와 분리된 HTML/CSS fixture에서 구성한다. root/child HostNode IDs, id/class attribute, child insertion sequence를 바꾼 positive control이 같은 used tracks와 CSS frame을 내야 한다.
3. runtime은 root columns `[40,80]`, rows `[20,50]`과 모든 node frame을 `0.02 CSS px` 허용 오차로 대조한다. Track source, placement, layout revision, draw/present를 함께 검사한다.
4. width/height, Grid track list, explicit placement, gap, margin/border/padding, writing mode 중 허용 계약 밖의 입력은 명시 오류로 fail-closed한다. Grid를 Block으로 낮추거나 첫 scene이 이미 만들어둔 style/frame을 재사용해 통과시키지 않는다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 parsed source commit을 실행한다. 플랫폼별 frame·used tracks·runtime log·app/source hashes를 보존한다. Surface samples를 별도 대조할 경우 Android PixelCopy와 iOS compositor 제출 전 Metal drawable 결과를 구분한다.
6. 기존 RootHtml block/text/background, root viewport/color-scheme, author-style diagnostic 경로 및 C11.1 explicit Grid fixture의 회귀를 실행한다. 기본/product artifact의 새 Grid route는 test-only scene ID나 fixed payload를 포함하지 않아야 한다.
7. Android hardware GPU, iOS 실기기, compositor 이후 iOS pixels, 전체 WPT 또는 전체 CSS Grid support를 이 slice의 결과로 주장하지 않는다. `spec/internal/0059-c11-grid-layout.md` 숫자 버전은 `0.1.0`으로 유지한다.

## 구현 전 필수 확인

- RootHtml default render의 production session·FFI entry와 기존 `RuntimeBlockFlexGridV1` pipeline 사이에서 CSS environment, author declaration admission, computed-style properties, layout snapshot, paint builder가 어느 경계에서 분리되는지 소스 호출 그래프로 고정한다.
- `RuntimeRootHtmlTextPaintV1`의 system-color, light/dark media, root `color-scheme`, viewport meta 결과가 Grid projection을 추가해도 같은 snapshot revision과 CSS viewport로 유지되는지 기존 root HTML fixtures와 실제 code path로 입증한다.
- Grid descendant의 intrinsic text/image collection이 필요해지는 순간을 확인한다. 이번 고정 scene은 빈 box만 쓰지만, 다음 단계의 intrinsic track sizing은 같은 일반 root HTML route에서 별도 계획·oracle·review를 거쳐야 한다.
- 성공 path와 negative controls를 구현하기 전 focused Rust fixture, default-vs-opt-in archive, Android app, iOS app 및 runtime source hash를 결정한다.

## 제외 범위

이 제한 통합은 auto-placement, repeat/named lines/areas, intrinsic text/image sizing, implicit track, span/fraction sizing, overflow/clipping/scroll, arbitrary unsupported CSS를 새로 지원하지 않는다. 이 목록은 C11 전체의 완료 정의가 아니다. Public CSS coverage, WPT runner, C11.3, C11.4 및 parent C11은 계속 열린다.

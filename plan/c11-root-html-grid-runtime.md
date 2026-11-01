# C11.1 · parsed root HTML author CSS Grid runtime integration

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [공식 상태](../spec/STATUS.md#c11-css-grid) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [레이아웃 계약 0009](../spec/internal/0009-layout-engine.md)

**구현 전 비교:** [고정 HTML source와 독립 기대값](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/README.md) · [Chrome 156 관측](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/chrome-observation.json) · [expected.json](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/expected.json)

**상태:** 현재 작업 트리 후보가 C11.1 focused runtime 2/2, Root HTML regression 17/17, formatting, scoped Clippy, Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 base/visible-overflow 실행에서 통과했다. simulator artifact와 미커밋 source hashes는 [evidence README](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/README.md) 및 [`candidate-source.sha256`](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/candidate-source.sha256)에 묶었다. implementation/runtime adversarial review 20개를 기록했다. 별도 PR diff review 30개는 통과했다. 구현 PR 생성·CI 확인·리베이스 병합은 아직 남았다. 이 제한 통합은 C11/C11.4 공식 완료 또는 `72/213` 수를 바꾸지 않는다.

**2026-11-02 fixed-px gap follow-up:** [별도 gap 계획](./c11-root-html-grid-gap.md)의 후보 구현·runtime 20관점 검토와 Android API 37 AVD/iPhone 17 Pro Simulator 실행이 끝났다. Root HTML 회귀는 18/18이며 percentage·math-backed gap은 계속 fail-closed한다. 이 follow-up은 같은 root HTML integration 경계를 좁혀 확장한 별도 PR 대기 slice다. 구현/runtime 근거는 [gap evidence](../spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/README.md); PR 생성·30관점 diff 검토·merge 전에는 공식 완료 또는 `72/213`을 바꾸지 않는다.

**계획 검토:** [계획 실패 관점 20개](./c11-root-html-grid-runtime-plan-review-20.md) · [이번 계획 PR diff 적대 검토 20개](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/plan-pr-diff-review-20.md) · [구현 중 visible overflow 범위 조정 검토 20개](./c11-root-html-grid-runtime-overflow-scope-review-20.md)

**구현 검토:** [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/implementation-runtime-review-20.md) · [실행·화면·hash](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/README.md)

## 구현 전 확인한 결손

- Parsed root HTML의 default runtime은 `RuntimeCalculationProfile::RootHtmlDefaultRender`에서 `RuntimeRootHtmlTextPaintV1` computed-style profile을 선택한다. 관련 경로는 [calculation.rs](../crates/spinon-runtime/src/session/ua_cascade/calculation.rs)다.
- [root HTML renderer](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/root_html_default_render.rs)는 visible tree의 `display:block`·`display:inline`만 받아 Grid container를 거부한다.
- C11.1 typed Grid style/layout 경로는 존재하지만 session에선 [runtime session](../crates/spinon-runtime/src/session.rs)의 `RuntimeCssProfile::BlockFlexGridFixture` 같은 profile별 entry로 호출된다. C11.4 track sizing entry도 별도 fixture profile로 노출된다. 이로 인해 일반 parsed root HTML의 `<style>` 또는 inline author declaration은 현재 Grid layout 경로까지 연결되지 않는다.

## 목표와 수락 기준

실제 root HTML runtime entry에서 author CSS `display:grid`를 Stylo cascade, typed Grid projection, Taffy layout과 runtime paint까지 전달한다. Grid root나 child를 NodeId, scene ID, class 이름, DOM 위치로 성공 선택하지 않는다. 일반 문서의 root id, child id, class 및 source order를 바꾼 입력도 같은 CSS 결과여야 한다.

고정 precomparison source는 [fixture](../tests/fixtures/css/c11/root-html-grid-author-precomparison.html)다. 독립 산술과 Chrome 156 관측은 다음과 같다.

| 대상 | 기대 geometry |
|---|---|
| `main` | `(0, 0, 120, 70)` CSS px; columns `[40, 80]`; rows `[70]`; initial gap `0` |
| `.item-a` | fixed size `40×70`; `(0, 0, 40, 70)` CSS px; default row auto-placement, first column |
| `.item-b` | fixed size `80×70`; `(40, 0, 80, 70)` CSS px; default row auto-placement, second column |

### 구현 수락 기준

1. root HTML parser가 `<meta name="viewport">`·inline `<style>`·body children을 같은 document/style/layout revision으로 전달한다. 새 code path는 기존 RootHtml runtime entry에서 호출되며 C11 fixture-test constructor에 종속되지 않는다.
2. positive scene source는 `NodeId`와 분리된 HTML/CSS fixture에서 구성한다. HostNode IDs를 바꾸고 CSS selector/attribute 이름을 동등하게 다시 쓴 positive control은 같은 computed style, used tracks와 semantic frames를 내야 한다. source-order reverse는 같은 root/tracks와 default row placement 규칙에 따른 item frame을 내야 한다.
3. runtime은 root columns `[40,80]`, row `[70]`과 모든 node frame을 `0.02 CSS px` 허용 오차로 대조한다. Root/style/layout revision과 기본 row auto-placement 결과를 함께 검사한다.
4. Root HTML Grid는 고정 px explicit track, 기본 row auto-placement, implicit track 없는 item 수만 받는다. Inline item은 Stylo에서 Grid 규칙에 따라 blockified된 computed style을 사용한다. 이외 제한되지 않은 item display와 nested Grid는 명시 오류로 fail-closed한다. `1fr`, percentage/calc/intrinsic/repeat/named line/area track, 추가 item으로 생기는 implicit track, authored explicit placement, non-default auto-flow, percentage/calc gap, nonzero margin·border·padding, non-static `position`, non-visible `overflow`, `direction:rtl` 및 비-horizontal writing mode도 명시 오류로 fail-closed한다. fixed non-negative px gap은 별도 [C11.1 follow-up plan](./c11-root-html-grid-gap.md)에서 한정한다. Item의 authored fixed width/height가 cell보다 크면 기본 `overflow: visible`에 따라 frame이 cell/root 바깥까지 유지되어야 한다. 41×71px item과 81px item의 120×70px root 밖 frame을 별도 fixture·Chrome oracle·runtime test로 확인한다. CSS overflow clipping·scroll은 이 slice 범위에 넣지 않는다. Grid를 Block으로 낮추거나 첫 scene이 이미 만들어둔 style/frame을 재사용해 통과시키지 않는다.
   Grid의 직접 자식 텍스트 시퀀스는 ASCII CSS whitespace만 포함할 때 `white-space: pre`를 포함한 computed `white-space` 값과 무관하게 item으로 세지 않는다. non-whitespace 직접 텍스트는 익명 Grid item layout이 이 slice 범위 밖이므로 어떤 `white-space` 값에서도 scene publish 전에 거부한다 ([CSS Grid §6](https://www.w3.org/TR/css-grid-2/#grid-items)).
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 parsed source working tree를 실행한다. `baseHead`와 변경 source/fixture hash manifest를 나눠 기록하고, 플랫폼별 runtime log·app/source hashes를 보존한다. Surface samples를 별도 대조할 경우 Android PixelCopy와 iOS compositor 제출 전 Metal drawable 결과를 구분한다.
6. 기존 RootHtml block/text/background, root viewport/color-scheme, author-style diagnostic 경로 및 C11.1 explicit Grid fixture의 회귀를 실행한다. 기본/product artifact의 새 Grid route는 test-only scene ID나 fixed payload를 포함하지 않아야 한다.
7. Android hardware GPU, iOS 실기기, compositor 이후 iOS pixels, 전체 WPT 또는 전체 CSS Grid support를 이 slice의 결과로 주장하지 않는다. `spec/internal/0059-c11-grid-layout.md` 숫자 버전은 `0.1.0`으로 유지한다.

## 구현 전 필수 확인과 결과

- source call graph에서 RootHtml default session의 CSS environment·cascade profile과 기존 typed Grid projection, layout snapshot, paint builder 경계를 확인한 뒤 실제 `RuntimeSession::new_runtime_gpu_with_root_html` path에 연결했다.
- 기존 viewport·color-scheme·block/text/background regression 17/17을 통과시켰고 Grid positive/negative runtime tests 2/2에서 같은 document/layout revision과 fail-closed 입력 경계를 확인했다.
- 고정 장면에 필요한 빈 Grid item 경로만 연결했다. intrinsic text/image contribution과 track sizing은 이 slice에서 추가하지 않고 별도 계획·oracle·review 범위로 남겼다.
- Android/iOS 앱 빌드에서 runtime GPU를 켜고 fixture-test feature는 껐다. 두 플랫폼의 base/overflow 실행·화면·로그와 source hashes를 보존했다. 최신 기록은 evidence README와 두 platform observation JSON을 따른다.

## 제외 범위

이 제한 통합은 authored non-default auto-placement, repeat/named lines/areas, intrinsic text/image sizing, implicit tracks, span/fraction sizing, CSS overflow clipping/scroll, arbitrary unsupported CSS를 새로 지원하지 않는다. 기본 visible overflow는 고정-size Grid item frame에 대해 유지하고 별도 경계 fixture로 검사한다. 고정 장면은 `grid-auto-flow`와 item placement를 author CSS에서 생략한 기본 row placement만 검사한다. 이 목록은 C11 전체의 완료 정의가 아니다. Public CSS coverage, WPT runner, C11.3, C11.4 및 parent C11은 계속 열린다.

# C11.1 · parsed root HTML author CSS Grid runtime integration

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [공식 상태](../spec/STATUS.md#c11-css-grid) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [레이아웃 계약 0009](../spec/internal/0009-layout-engine.md)

**Root HTML percentage-track 후속:** 계획 PR [#1018](https://github.com/ohah/spinon/pull/1018)에 이어 제한 구현 PR [#1019](https://github.com/ohah/spinon/pull/1019)이 2026-11-02T01:42:07Z Rebase and merge됐다(merge SHA `8e549ed55ac6f7e9611a4f7e1e571fca3a1e5684`). 범위와 한계는 [전용 계획](./c11-root-html-grid-percentage-tracks.md)에 기록한다. 기존 `px`/`fr` 동작과 공식 C11.1/C11 완료 상태는 그대로이며 공식 완료는 `72/213`이다.

**구현 전 비교:** [고정 HTML source와 독립 기대값](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/README.md) · [Chrome 156 관측](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/chrome-observation.json) · [expected.json](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/expected.json)

**상태:** Root HTML 기본 runtime 통합 PR [#1000](https://github.com/ohah/spinon/pull/1000)은 2026-11-01T20:04:43Z Rebase and merge로 병합됐다(merge SHA `1971edf0ad9e11ec32f08b83728220f2161b17f5`). 제한된 base/visible-overflow 경로의 Root HTML regression 17/17, formatting·scoped Clippy와 플랫폼 실행 근거가 기록돼 있다. 이어 고정 px gap 후속 PR [#1002](https://github.com/ohah/spinon/pull/1002)도 2026-11-01T20:53:41Z 병합됐다(merge SHA `12b554c5e1a1946e7267545d5d5092ba33211af9`); Root HTML regression은 18/18이고 percent·math-backed gap은 계속 거부한다. [기본 경로 실행 근거](../spec/internal/evidence/c11-root-html-grid-runtime-precomparison-2026-11-02/README.md) · [gap 후속 실행 근거](../spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/README.md). PR check run은 보고되지 않아 CI 통과로 주장하지 않는다. 이 두 제한 slice는 전체 CSS Grid, C11.3/C11.4 또는 C11 공식 완료를 뜻하지 않으며 `72/213`을 바꾸지 않는다.

**2026-11-02 fixed-px gap follow-up:** [별도 gap 계획](./c11-root-html-grid-gap.md)의 제한 구현 PR [#1002](https://github.com/ohah/spinon/pull/1002)이 2026-11-01T20:53:41Z Rebase and merge로 병합됐다(merge SHA `12b554c5e1a1946e7267545d5d5092ba33211af9`). Root HTML 회귀 18/18, percentage·math-backed gap fail-closed, 구현/runtime 검토 20개와 PR diff 검토 30개를 기록했다. Android API 37 AVD/iPhone 17 Pro iOS 26.2 Simulator의 캡처는 draw/present 보조 증거이며 PixelCopy·Metal drawable readback은 아니다. [gap 실행 근거](../spec/internal/evidence/c11-root-html-grid-gap-runtime-2026-11-02/README.md). GitHub check run은 보고되지 않았다. 이 제한 후속은 전체 C11 또는 공개 CSS Grid 지원 완료를 뜻하지 않으며 공식 `72/213`을 바꾸지 않는다.

**2026-11-02 percentage-track follow-up:** direct finite non-negative percentage tracks는 definite width/height 축으로 제한한다. Chrome 156 사전 관측과 Android API 37 ARM64 AVD/iPhone 17 Pro iOS 26.2 Simulator의 동일 HTML SHA, `layout=ready boxes=15`, draw/present 및 visible-overflow screenshots를 기록했다. Projection 4/4, layout 1/1, Root HTML runtime 16/16, scoped Clippy와 rustfmt가 통과했다. 구현 PR [#1019](https://github.com/ohah/spinon/pull/1019)은 2026-11-02T01:42:07Z Rebase and merge됐다. PR check rollup은 비었고 필수 status check가 없어 CI 통과를 주장하지 않는다. 상세 제한·실행 근거는 [percentage plan](./c11-root-html-grid-percentage-tracks.md)과 [runtime evidence](../spec/internal/evidence/c11-root-html-grid-percentage-runtime-2026-11-02/README.md)에 있다. 이 제한 slice는 C11.1/C11 완료 또는 공개 CSS 지원으로 승격하지 않는다.
**2026-11-02 fixed `grid-auto-rows` implicit-row plan:** 계획 PR [#1023](https://github.com/ohah/spinon/pull/1023)은 단일 computed px row, 기본 row flow와 empty fixed-size Root HTML items 범위만 제안한다. 현재 cascade는 single computed px length를 전달하고 projection이 거부한다. 계획·Chrome 156 geometry precomparison만 준비했으며 구현·테스트·simulator 검증은 하지 않았다. [계획](./c11-root-html-grid-implicit-rows.md) · [사전 비교](../spec/internal/evidence/c11-root-html-grid-implicit-rows-precomparison-2026-11-02/README.md). C11.1/C11 및 공식 `72/213`은 미완료다.

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
4. Root HTML Grid base slice는 고정 px explicit track, 기본 row auto-placement, implicit track 없는 item 수만 받았다. Inline item은 Stylo에서 Grid 규칙에 따라 blockified된 computed style을 사용한다. 후속 [bounded direct `fr` slice](./c11-root-html-grid-fr-track.md)는 PR #1015로 병합됐고, [definite-axis percentage slice](./c11-root-html-grid-percentage-tracks.md)는 PR #1019로 병합됐다. 두 후속 모두 fixed named-area combination, implicit sizing, nested Grid 및 일반 CSS admission을 추가하지 않는다. 추가 item으로 생기는 implicit track, authored explicit placement, non-default auto-flow, percentage/calc gap, nonzero margin·border·padding, non-static `position`, non-visible `overflow`, `direction:rtl` 및 비-horizontal writing mode는 계속 fail-closed한다. fixed non-negative px gap은 별도 [C11.1 follow-up plan](./c11-root-html-grid-gap.md)에서 한정한다. Item의 authored fixed width/height가 cell보다 크면 기본 `overflow: visible`에 따라 frame이 cell/root 바깥까지 유지되어야 한다. 41×71px item과 81px item의 120×70px root 밖 frame을 별도 fixture·Chrome oracle·runtime test로 확인한다. CSS overflow clipping·scroll은 이 slice 범위에 넣지 않는다. Grid를 Block으로 낮추거나 첫 scene이 이미 만들어둔 style/frame을 재사용해 통과시키지 않는다.
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

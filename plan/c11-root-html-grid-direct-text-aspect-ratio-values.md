# C11.1 parsed Root HTML direct-text Grid `aspect-ratio` 값 확장

**계획 실패 관점 검토:** [구현 전 20개](./c11-root-html-grid-direct-text-aspect-ratio-values-plan-review-20.md) · **구현 전 oracle:** [Chrome 156 사전 비교와 14개 scene](../spec/internal/evidence/c11-root-html-grid-text-aspect-ratio-values-precomparison-2026-11-05/README.md)

## 목적과 현재 경계

기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1`의 direct-text Grid item sizing에서 현재 숫자형 `aspect-ratio:2/1`만 통과하는 제한을 확장한다. 폭 또는 높이가 고정된 같은 두-column·한-auto-row Grid의 direct block-text target 하나에 대해 finite positive preferred ratio를 typed cascade 값으로 layout에 전달한다. 이는 C11.4의 별도 opt-in fixture profile과 일반 CSS 지원을 확장하지 않는다.

현행 기준은 구현 PR [#1161](https://github.com/ohah/spinon/pull/1161)에서 병합한 narrow slice다. 그 코드의 ratio `2/1` gate, 허용 target size pair, text/Grid topology는 이번 작업의 시작 경계다. 이 계획은 값을 넓히되 topology나 소유권 범위를 넓히지 않는다.

## 기준값과 독립 판정

- 기준 구현은 Agent Browser 0.31.1 / HeadlessChrome 156.0.0.0이다. 입력은 [고정 HTML fixture](../tests/fixtures/css/c11/root-html-grid-direct-text-aspect-ratio-values-precomparison.html)이며 fixture hash, 14개 장면의 computed style/frame, 전체 화면과 독립 기대값은 [증거 디렉터리](../spec/internal/evidence/c11-root-html-grid-text-aspect-ratio-values-precomparison-2026-11-05/README.md)에 기록했다.
- CSS ratio pair는 width/height 비율이다. 숫자 하나 `1.5`는 computed `1.5 / 1`, pair `3/2`는 `3 / 2`로 직렬화되지만 typed ratio와 기하는 같다. 규범 근거는 [CSS Values and Units](https://www.w3.org/TR/css-values/)의 `<ratio>` 정의, [CSS Grid Layout](https://www.w3.org/TR/css-grid/)의 Grid item sizing, [CSS Sizing Level 4](https://www.w3.org/TR/2026/WD-css-sizing-4-20260904/)의 preferred ratio sizing이다. CSS Sizing 문서는 Working Draft이므로 이 고정 Chrome 행렬과 함께 비교 입력으로 사용하며 상호운용성 완료를 주장하지 않는다.
- 고정 topology는 content-box 70px root Grid, columns `[10px,60px]`, single auto row, zero gap, row flow, visible overflow, start alignment다. 두 in-flow direct HTML block text item은 `A`와 `AAAA`이며, 한 target만 ratio를 가질 수 있다. width-locked target은 `width:60px;height:auto`; height-locked target은 `width:auto;height:30px`이다. 나머지 고정값은 [기존 계획](./c11-root-html-grid-text-aspect-ratio.md)의 경계를 따른다.

| 장면 | typed preferred ratio | 독립 기대값 |
|---|---:|---|
| width fixed 60, `1/2` | 0.5 | target `60×120`, auto row `120` |
| width fixed 60, `4/3` | 1.333… | target `60×45`, auto row `45` |
| width fixed 60, `1.5` 및 `3/2` | 1.5 | 두 syntax 모두 target `60×40`, auto row `40` |
| width fixed 60, `5/2` | 2.5 | target `60×24`, auto row `24` |
| width fixed 60, `10/1` | 10 | transferred height `6`; text automatic minimum이 우선해 target/row height `10` |
| height fixed 80, `1/2` | 0.5 | target `40×80`; 지정 height는 Ahem direct-registration 최소 폭과 맞춤 |
| height fixed 30, `4/3`, `3/2`, `5/2` | 각각 1.333…, 1.5, 2.5 | target 폭 `40`, `45`, `75`; 마지막은 70px Grid 바깥으로 15px visible overflow |
| 양 축 기존 `2/1` 회귀 | 2 | 두 방향 모두 기존 `60×30` |
| `auto` 대조 | numeric ratio 없음 | 기존 ratio transfer가 없고 장면별 text/minimum sizing 유지 |

앱/Chrome 기본 font는 OS에 따른 intrinsic width 차이가 있으므로 height-locked `auto` control의 monospace 폭은 플랫폼 간 exact 비교하지 않는다. Rust layout 단위 검사는 고정 Ahem을 직접 등록해 intrinsic control을 결정적으로 만든다. Chrome DOM geometry의 고정 입력 값은 독립 기대값과 정확히 일치한다. Rust 내부 ratio 연산은 f32이므로 ratio-derived frame/auto row의 허용 오차를 0.01 CSS px로 둔다. authored fixed tracks와 overflow 계산은 정확한 입력값으로 별도 확인한다. Simulator node geometry 허용 오차는 기존 runtime harness 기준인 0.5 CSS px다. Geometry oracle는 glyph raster, WGPU surface, Android PixelCopy 또는 iOS compositor pixel 비교가 아니다.

## 구현 계약

- cascade allowlist는 기본 `RuntimeRootHtmlTextPaintV1` profile에 이미 존재하는 `aspect-ratio` 항목을 유지한다. 다른 style profile이나 generic projection allowlist를 넓히지 않는다. cascade가 만든 `ComputedLayoutAspectRatio`를 sizing의 숫자 출처로 쓰며 CSSOM serialization을 다시 파싱하지 않는다.
- projection은 direct item 하나와 owner identity를 그대로 검사한다. 성공 조건은 matching `NodeId`, computed serialization이 numeric preferred ratio임, typed ratio 존재, `ComputedAspectRatioSizingBox::BoxSizing`, finite positive ratio다. `auto <ratio>`의 `ContentBox`, computed `auto`, degenerate/unsupported 값, 누락되거나 불일치한 typed state는 기존 contextual error 경계에서 거부한다.
- ratio transfer는 width/height 방향을 유지한다: width fixed일 때 auto height는 `width / ratio`; height fixed일 때 auto width는 `height × ratio`다. target size·track·text minimum에 대한 결과가 layout engine에서 유한하게 표현되지 않으면 partial frame을 반환하지 않고 실패해야 한다. typed ratio는 CSS computed serialization string에서 가져오지 않는다.
- layout admission은 기존 owner/item/topology/style 검사와 두 size pair를 보존한다. 한 owner의 ratio item이 여러 개면 owner 전체를 거부한다. root Grid/block root ratio, 다른 owner가 섞인 상태, multiple target, nested/replaced/anonymous item, 비기본 placement/alignment, wrapping, min/max constraints와 box edge 변경은 이 slice 밖이다.
- 자동 최소 크기는 ratio transfer 뒤에도 적용된다. `10/1` width-locked 장면의 6px transferred height를 그대로 frame으로 강제하지 않고 Ahem text 최소 높이 10px을 보존해야 한다. `5/2` height-locked 장면의 target frame은 Grid track 경계로 clamp하지 않고 visible overflow를 유지한다.
- CSS-invalid 선언의 cascade fallback과 valid-but-unsupported computed ratio는 분리한다. `aspect-ratio:auto`, `auto <ratio>`, `0/1`, typed serialization 누락/불일치, min/max, Grid-root ratio, 복수 ratio item을 negative controls로 둔다. 이미 유효한 CSS computed ratio의 오류를 `auto` fallback으로 바꾸지 않는다.

## 구현·검증 순서와 수락 기준

1. 이 계획을 서로 다른 실패 관점 20개로 검토하고 필요한 보정을 반영한다. 계획 검토 완료 전에 제품 코드를 수정하지 않는다.
2. [기존 layout admission](../crates/spinon-layout/src/grid/root_html_direct_text_aspect_ratio.rs), [owner-scoped projection](../crates/spinon-style-to-layout/src/projection/style_values/root_html_direct_text_aspect_ratio.rs), cascade typed conversion 및 [기존 regression matrix](../crates/spinon-style-to-layout/src/c11_1_root_html_direct_text_aspect_ratio_tests.rs)를 대조한다. 적합한 영역의 정확한 수치 gate만 일반 finite-positive typed ratio gate로 바꾼다.
3. Rust layout와 projection 테스트는 14 Chrome scene의 ratio-derived frame/auto row를 독립 기대값과 `0.01 CSS px` 이내로 확인하고 authored fixed tracks는 exact로 단언한다. Ahem direct registration을 유지하고 unsupported/invalid/multiple-owner/owner identity/size constraint controls에서 전체 장면이 fail closed인지 확인한다. `ratio=10` automatic-minimum 및 height-locked `5/2` unclamped overflow는 명시적으로 단언한다.
4. 기본 parsed Root HTML runtime fixture를 같은 HTML source/hash로 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 실행한다. run별 source/hash, build/run 환경, root frame, `layout-ready`, draw/present 경계를 기록한다. 런타임 harness가 NodeId frame을 내보내지 않으면 앱이 그 frame을 직접 측정했다고 쓰지 않는다. Surface readback은 실제 수행할 때만 별도 증거로 기록한다.
5. 바뀐 crate의 focused tests, 해당 범위 Clippy/rustfmt 및 두 simulator runtime을 각각 기록한다. 실행하지 않은 gate를 통과했다고 쓰지 않는다. implementation/runtime 변경에는 새 실패 관점 20개를, 구현 PR diff에는 별도의 20개 적대 검토를 둔다. 각 PR마다 사용자 지시에 따라 20개 PR 검토도 별도로 남긴다.
6. 구현 병합 후 내부 contract `0059`, `spec/STATUS.md`, 기존 C11.1 작업 기록과 사용자가 지정한 Tailnet roadmap을 실제 merge state에 맞춰 동기화한다. 이 bounded slice는 C11.1, C11 또는 공식 완료 수를 올리지 않는다. 전체 WPT, 일반 CSS 지원, C11.4와 실기기 GPU 증거는 이 변경의 완료로 표기하지 않는다.

## 제외 범위

일반/임의 CSS `aspect-ratio` 지원, `auto <ratio>`, replaced/image item, nested/anonymous items, 여러 ratio item/owner, min/max transfer, content/border-box edges, wrapping/shaping, 임의 Grid topology와 placement, WPT 적합성, 실제 GPU, iOS compositor 이후 pixels, C11.4 전체 또는 C11 완료 판정.

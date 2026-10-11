# C12.4 · 쌓임 맥락과 `z-index` 구현 계획

**상위:** [C12 위치 지정 계획](c12-positioning.md) · [공식 상태 대장](../spec/STATUS.md)
**계약:** [내부 계약 0058](../spec/internal/0058-c12-4-stacking-context.md)
**계획 검토:** [서로 다른 20개 실패 관점](../spec/internal/evidence/c12-4-stacking-plan-review-2026-10-11.md)
**현재 상태:** 계획·내부 계약 초안 · 기능 구현 전
**목표:** CSS 쌓임 맥락의 원자성, `z-index` 단계, Flex paint 예외를 고정된 Chromium 결과와 GPU paint-list 양쪽에서 확인한다.

## 범위

C12.4는 기존 layout frame 위에 CSS paint 순서를 적용한다. 레이아웃 좌표를 다시 계산하거나 Taffy의 노드 순서를 바꾸지 않는다. 첫 구현은 내부 runtime profile 하나에서 섞인 Block/Flex box, C12.1–C12.3 위치 지정, `z-index`, C10.3 `order`를 같은 document/style revision으로 계산해야 한다. 기존 Block-only 또는 Flex-only snapshot을 이어 붙여 혼합 장면을 만드는 방법은 허용하지 않는다.

계획한 제한 profile 이름은 `RuntimeBlockFlexStackingV1`이다. 이 이름은 내부 타입 후보이며 공개 API나 출시 버전이 아니다. 실제 코드를 시작하기 전에 현재 cascade·layout profile의 지원값을 다시 대조한다. profile은 다음 의미를 한 번의 cascade 및 layout 입력 안에서 보존해야 한다.

- `display:block|flex|none`, 기존 지원 크기·간격·배경 paint, 물리 inset과 `position:static|relative|absolute|fixed`.
- computed `z-index:auto|<integer>`와 Flex item의 computed `order`.
- CSS author stylesheet와 inline style의 기존 cascade 우선순위. `z-index:var(...)`처럼 Stylo가 계산할 수 있는 값은 raw CSS 문자열을 다시 파싱하지 않고 computed typed value로 사용한다. 이미 지원하는 사용자 지정 속성 및 등록 사용자 지정 속성 동작을 혼합 profile에서도 유지한다.
- C12.2/C12.3 containing-block 검증과 C10.3.5 Flex absolute-child static position 및 paint-order 규칙.
- transparent 또는 기존 runtime profile이 paint할 수 있는 완전 불투명 단색 배경. 기존 frame·paint·revision 오류는 계속 원자적으로 거부한다.

텍스트/inline fragment, float, table, Grid, replaced element, `display:contents`, sticky, pseudo-element, top layer는 이 단계의 성공 입력이 아니다. `visibility`는 computed `visible`만 허용하며 `hidden`·`collapse`와 그 cascade/source 표현은 현재 paint/layout 경계에서 구체적인 진단으로 거부한다. 요소-only layout/paint 경계 밖의 장면은 node·source·property를 식별해 실패해야 한다. 이는 최종 웹 호환 목표를 줄이지 않으며, 미구현 범위를 지원처럼 표시하지 않기 위한 단계 경계다.

## 페인트 순서 계약

기준 순서는 CSS2.2 Appendix E의 stacking-context paint 단계이며, CSS Positioned Layout 3의 `auto` 의미 및 Flexbox의 z-axis 규칙을 함께 적용한다.

1. Host root element가 항상 만드는 root stacking context를 시작한다. 이 단계는 기존 C08 root background paint 동작을 보존하고 canvas·body background 전파를 새로 구현하지 않는다.
2. 각 실제 stacking context 안에서는 음수 stack level context를 오름차순으로 그리고, 같은 level은 해당 context의 CSS tree order로 안정적으로 정렬한다.
3. 지원되는 in-flow Block-level box는 기존 Block paint 순서를 따른다. Flex container 자체는 자기 formatting context에서 block-level box로 취급하고, 그 직접 in-flow Flex item은 Flexbox가 정한 order-modified document order의 paint participant로 처리한다. 중첩 container의 `order`를 조상 전체의 단일 정렬 키로 합치지 않는다. `row-reverse`/`column-reverse`는 order-modified document order 자체를 뒤집지 않는다.
4. Flex item은 inline-block 유사 paint 규칙과 z-axis 예외를 따른다. static Flex item의 non-auto `z-index`는 실제 stacking context를 만든다. 해당 Flex item이 실제 context가 아니면 자손 context는 이를 건너뛰어 상위 context에 참여할 수 있다. 자식 stacking context를 무조건 Flex item 아래로 묶는 구현은 금지한다.
5. positioned descendant의 `z-index:auto` pseudo-context 및 stack level 0의 context를 같은 paint 단계에서 CSS tree order로 처리한다. `auto` pseudo-context는 자손 stacking context를 격리하지 않으며, 실제 `z-index:0` context는 자손을 원자적으로 격리한다.
6. 양수 stacking context를 stack level 오름차순으로 그리고, 같은 level은 context 안의 유효 CSS tree order로 안정적으로 정렬한다. 음수·0·양수 값은 전역 비교하지 않고 현재 context의 자식끼리만 비교한다.
7. 각 실제 자식 stacking context는 완전히 paint한 뒤 하나의 원자적 participant로 부모 순서에 합친다. 자손의 큰 숫자가 context 바깥 sibling 위로 빠져나오면 안 된다. 비-context positioned/Flex box의 자손은 규격의 pseudo-context/tree-order 규칙을 따라 해당 부모 context phase에 참여한다.

구현 전 Chrome 사전 비교에서 위 participant phase와 tie-break를 supported box 유형별로 확인한다. 특히 Flex item subtree가 non-context item을 넘어 참여하는 경우, fixed/absolute Flex child의 `order`, Flex container와 같은 단계의 Block sibling 간 tie를 별도 overlap case로 고정한다. 이 관측값을 확인하기 전에는 paint builder의 세부 순서를 더 좁혀 구현하지 않는다.

실제 context 생성 규칙은 다음과 같이 한정한다.

| box 종류 | `z-index:auto` | 정수 `z-index` |
| --- | --- | --- |
| root element | root context | root context의 기존 위치 유지; 자식 간 전역 rank를 만들지 않음 |
| `position:relative|absolute` | pseudo-context; 실제 자손 context는 부모 context에 참여 | 실제 stacking context |
| `position:fixed` | 실제 stacking context | 해당 정수 stack level의 실제 context |
| static Flex item | Flex item paint participant | 실제 stacking context, Flexbox 규칙 적용 |
| 그 밖의 static Block box | 일반 flow paint | stack level 효과 없음 |

`position:sticky`는 CSS상 stacking context를 만들더라도 C12.5의 scrollport·clip 입력이 없으므로 이 profile에서 거부한다. 모든 `z-index` 값의 효력은 computed box 종류와 formatting context에 따라 정한다. `z-index`를 장면 전체에서 숫자로 정렬하는 구현은 금지한다.

Flex `order`는 Flex item의 layout/paint 순서에만 영향을 준다. HostDocument parent/child 순서, DOM 유사 조회, NodeId, 수명 및 접근성 traversal을 바꾸지 않는다. absolute Flex child는 Flex item이 아니며 C10.3.5의 `order:0` paint 규칙과 source-tree 순서를 따른다. fixed Flex child 역시 in-flow Flex item이 아니므로 authored `order`를 flex layout 순서로 적용하지 않는다. fixed child의 정확한 paint phase는 고정 Chrome reference로 구현 전에 확인한다. 기존 S04.9 static hit-test fixture나 미래 입력 event target 정책을 이 문서만으로 완료 처리하지 않는다. 향후 scene 기반 hit-test가 붙을 때에는 painted topmost box 선택과 동일한 `paint_order`를 소비하는지 별도 검증한다.

## 데이터 흐름과 책임

1. Stylo cascade는 같은 style revision에서 position, z-index, display, order와 paint 입력을 computed typed snapshot으로 만든다. `auto`와 정수는 별도 enum variant여야 하며 문자열 비교나 CSS 값 재파싱을 사용하지 않는다.
2. Style-to-layout은 현재 HostDocument parent와 Flex item 관계에서 paint용 box/context 관계를 만든다. Taffy 계산 parent, absolute containing-block owner, HostDocument parent를 서로 바꾸어 쓰지 않는다.
3. Paint builder는 임시 stacking-context tree 또는 그와 동등한 typed staging 구조를 구성한다. Rust call stack에 의존하는 무제한 재귀는 피한다. 깊은 트리, sibling 대량 정렬, 중복/누락 NodeId와 할당 실패를 명시적으로 처리한다.
4. staging 구조를 back-to-front 단일 `RuntimeRenderSnapshot`으로 한 번 flatten한다. 기존 `paint_order`는 CSS paint rank를 뜻하고 `0..N-1` 연속 불변식을 유지한다. 진단용 context 경로와 NodeId는 테스트·evidence에서만 노출하며 public JS API에 추가하지 않는다.
5. WGPU는 이미 정렬된 box 목록을 순서대로 그린다. CSS 해석, z-index sort, stacking-context 정책을 WGPU backend에 복제하지 않는다. 이 계약은 paint order를 고정하며 C22 border/gradient/shadow 및 C23 opacity/transform/offscreen compositing을 구현하지 않는다.

스타일 변경으로 z-index·position·order 또는 display가 달라지면 새 `StyleRevision` 장면의 rank를 다시 계산해야 한다. layout frame이 동일하다는 이유로 이전 paint list를 재사용하지 않는다. 문서·스타일·환경 revision 중 하나라도 맞지 않거나 새 scene 계산이 실패하면 부분 scene을 게시하지 않는다.

## 닫힌 실패 경계

다음 선언은 첫 profile에서 stacking/compositing 의미를 보존하지 못하므로 inline 및 author stylesheet 입력에서 감지되면 fail-closed한다. stylesheet selector의 현재 match 여부나 cascade winner 여부로 미지원 효과를 조용히 숨기지 않는다.

- computed `opacity < 1`, `transform`·개별 `translate`/`rotate`/`scale`·`perspective`의 비초기 효과, `transform-style:preserve-3d`.
- `filter`, `backdrop-filter`, `mix-blend-mode`, `isolation`.
- `contain`의 layout/paint 효과, `will-change`가 예고하는 stack/context 효과.
- top layer 진입 상태와 이를 요청하는 현재 미지원 dialog/popover API·속성, animated stacking trigger와 현재 typed profile이 표현하지 않는 기타 paint participant.

위 목록은 allowlist가 아니다. 전체 입력 검사가 기본이고 위 속성은 회귀용 negative control이다. context trigger의 속성 이름만 찾는 것으로 끝내지 말고, source declaration preflight와 computed typed value 검사에서 어떤 값까지 보수적으로 거부하는지 구현 전 계약으로 고정한다. `@media`/`@supports` 및 중첩 규칙, inline declarations, custom-property substitution, stylesheet parse diagnostic이 검사를 우회하지 않는지 확인한다. selector/cascade winner를 판정하지 않는 기존 source preflight의 한계와 그에 따른 과잉 거부는 지원 범위에 적는다. 전체 C22/C23 기능을 끌어오거나 비지원 속성의 CSS 의미를 이 계약에서 재구현하지 않는다.

## 비교 기준과 fixture

**주 oracle:** Chromium `154.0.8037.98`, revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33`. existing CSS reference tools와 고정 Chromium launcher를 사용하며 HTML, capture code, Chromium binary, inventory, output JSON의 SHA-256을 보관한다. C12.2/C12.3/C10.3.5 geometry fixture의 frame과 C10.3.2/C10.3.5 paint fixture를 regression으로 함께 사용한다.

새 `c12-4-stacking-context-v1` fixture에는 겹치는 불투명 색 box와 `NodeId` 대응을 넣는다. geometry capture, computed-style capture, paint-order inference, pixel oracle를 서로 다른 관찰치로 저장하고 paint 순서를 screenshot 하나에서 추론하지 않는다. 적어도 다음 장면을 포함한다.

- 음수·0·양수, 동률, `auto`, `z-index` 정수 경계와 root background.
- 중첩 실제 context의 자손이 외부 sibling 위로 빠져나오지 않는 경우.
- relative/absolute `auto` 아래 자손 context가 부모 context에 참여하는 경우와 같은 위치의 `z-index:0` 원자 격리 비교.
- fixed `auto`가 stacking context를 생성하는 경우, static Block의 z-index 비효력, static Flex item의 `z-index` 예외.
- Flex `order` 동률/source order, in-flow subtree, 중첩 Flex, absolute/fixed Flex child의 authored nonzero `order`가 paint rank를 바꾸지 않는 경우.
- inline style·author stylesheet·cascade layer·`!important`·`var()`·registered custom property의 computed z-index, `display:none`, `visibility:hidden|collapse`, C12.2/3 containing-block owner와 paint context가 다른 경우.
- 미지원 context trigger, 누락 style/frame, 중복 NodeId, 잘못된 revision, 비유한/범위 밖 z-index 입력 및 깊고 넓은 트리의 오류 경계.

Chrome reference는 node별 bounding rect/computed `position`, `z-index`, `order`를 저장한다. Chrome에서 paint-order 배열을 직접 읽었다고 주장하지 않는다. 보이지 않는 CSS stacking 결과는 고정 겹침 지점의 Chrome screenshot pixel과 WGPU offscreen readback pixel을 서로 다른 증거로 비교한다. 양 oracle 출력 공간을 sRGB로 고정·기록하고 box 내부의 edge가 아닌 표본점을 비교한다. WGPU offscreen 결과의 채널당 최대 1/255 차이는 보조 확인 기준이며, Android 기기 screenshot을 동일 픽셀 오라클로 섞지 않는다. NodeId order·context isolation·frame geometry는 색 오차로 상쇄하지 않는다. Chrome의 `elementsFromPoint()`는 보조 관측이며 screenshot pixel을 대신하지 않는다.

WPT 의미 후보는 upstream commit `c999c58338ee1d223df5ad62e48be1202ced0d37`에 고정한다. 관련 경로는 [CSS2 z-index stack](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/CSS2/zindex/z-index-stack-001.xht), [CSS2 absolute z-index](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/CSS2/zindex/z-index-abspos-003.xht), [positioned auto pseudo-context](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-position/position-absolute-under-non-containing-stacking-context.html), [Flex stacking-context isolation](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-flexbox/flexbox-items-as-stacking-contexts-001.xhtml), [Flex paint ordering with order/z-index](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-flexbox/flexbox-paint-ordering-002.xhtml)다. 이들은 test selection 참고이며 아직 실행 결과가 아니다. 텍스트·float·border·blend가 섞인 WPT case는 현재 범위와 분리해 실행 전 지원 subset 또는 예상 미지원으로 분류한다.

판정 기준은 다음과 같다.

- 모든 지원 box의 Chrome frame은 node ID 기준 각 축 최대 절대 오차 `0.5 CSS px` 이내이며 paint 구현 전후의 geometry는 바뀌지 않는다.
- fixture에서 기대한 NodeId paint rank가 매 revision에서 정확히 같고 모든 지원 visible node가 한 번만 나타난다. `display:none` subtree는 paint list에서 빠진다.
- context 격리, auto pseudo-context, stack-level phase, Flex order exception의 각 overlap 표본에서 Chrome과 WGPU 결과가 지정 색상 허용치 이내다. 평균값으로 개별 실패를 숨기지 않는다.
- 지원하지 않는 stack trigger·display·position·cascade 입력은 NodeId/source/property를 식별해 전체 계산을 거부한다. stale·부분 snapshot을 WGPU에 제출하지 않는다.
- baseline `RuntimeRenderSnapshot`, C12.1–C12.3, C10.3.2/C10.3.5 회귀가 유지된다. `u32` paint-order overflow, reserve 실패, 아주 깊은 context graph가 panic·중복·부분 scene을 만들지 않는다.

## 작업 목록과 순서

- [ ] **계약·profile:** position/block/flex를 결합하는 단일 computed-style/layout profile, allowlist, Stylo typed z-index snapshot, custom-property/registered-property 경계와 diagnostic을 확정한다.
- [ ] **독립 oracle:** Chrome 154 fixture, viewport/DPR, screenshot sample coordinate, geometry/paint rank inventory와 WPT 고정 revision/hash를 생성한다. expected stacking order는 input CSS/fixture metadata 또는 별도 규격 판정표에 저장하고, 구현 후 생성된 paint list를 정답으로 재사용하지 않는다. 비교 전 기대값을 기록하고 fixture 변경으로 실패를 없애지 않는다.
- [ ] **paint builder:** HostDocument/Flex 순서를 보존한 stacking-context staging, stack-level별 안정 정렬, atomic child-context flattening, overflow/할당 오류의 원자성을 구현한다.
- [ ] **GPU/runtime:** 변경된 flat paint list를 revision key로 검증하고 WGPU에 같은 순서로 제출한다. scene consumer가 향후 hit-test를 지원할 때 GPU paint와 topmost target이 같은 순서를 쓰는지 별도 관문으로 둔다. Android 실기기에서 동일 JS fixture를 실행하고 iOS Simulator에서 보완 실행한다. 로그, screenshot, offscreen pixel readback, device/OS/GPU/backend를 저장한다.
- [ ] **검증:** 새 Rust unit/integration/reference tests, 기존 C10/C12 regression, 실패 주입, Clippy/rustfmt/workspace 검증을 실행한다. 기능 구현 뒤 계획 검토와 겹치지 않는 코드·런타임 실패 관점 20개를 따로 기록한다.
- [ ] **동기화:** 내부 계약, `spec/STATUS.md`, internal index, C12 master plan, public roadmap, evidence index 및 Tailnet preview를 PR 결과와 같은 상태로 갱신한다. GitHub Pages는 실행하지 않는다.

구현 시작 전 Chrome 사전 비교 기준과 fixture hash가 준비되지 않으면 기능 코드를 작성하지 않는다. Android 실기기가 작업 시점에 연결되지 않으면 이를 platform gate로 기록하고, 시뮬레이터 결과만으로 C12.4를 완료 표시하지 않는다.

## 표준 및 고정 비교 모델

- [CSS Positioned Layout Level 3 · 2025-10-07 Working Draft](https://www.w3.org/TR/2025/WD-css-position-3-20251007/) — position, `z-index:auto`, fixed/relative/absolute와 stacking context 의미.
- [CSS2.2 Appendix E · Elaborate description of Stacking Contexts](https://www.w3.org/TR/CSS22/zindex.html) — stacking-context paint 단계와 auto/zero/positive/negative level.
- [CSS Flexbox Level 1 · 2025-10-14 CRD](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/) — Flex item z-index 예외, `order`-modified paint order.
- [CSS Display Level 3 · 2026-06-05 CRD](https://www.w3.org/TR/2026/CRD-css-display-3-20260605/) — box tree와 display/order 개념. CRD는 작업 중인 규격이며 무조건 안정화된 표준으로 부르지 않는다.
- context 생성 negative control 근거: [CSS Transforms 1](https://www.w3.org/TR/css-transforms-1/), [CSS Color 4](https://www.w3.org/TR/css-color-4/), [CSS Containment 2](https://www.w3.org/TR/css-contain-2/), [Compositing and Blending 1](https://www.w3.org/TR/compositing-1/).
- renderer baseline: [RuntimeRenderSnapshot flat paint order](../spec/internal/0051-c10-3-2-flex-order.md), [C10.3.5 positioned Flex paint contract](../spec/internal/0056-c10-3-5-positioned-flex.md), [C12.3 fixed contract](../spec/internal/0057-c12-3-fixed-viewport.md), [Chromium 154 fixed build metadata](../spec/internal/evidence/c12-3-fixed-precomparison-2026-10-11.md).

## 완료 판정

이 문서는 계획 산출물이며 기능 완료를 뜻하지 않는다. C12.4를 체크하려면 구현 PR이 병합되고, pinned Chrome·WPT candidate·NodeId rank·GPU pixel·Android 실기기·iOS Simulator·실패 경계 근거를 모두 갖춰야 한다. 미지원 stacking effect나 미검증 platform은 상태 설명에 남긴다. 내부 계약·crate 숫자 버전은 계속 `0.1.0`이며 이 계획으로 올리지 않는다.

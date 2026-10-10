# 0047 · C09 Block formatting

**문서 ID:** `0047` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** C09.1 구현·검증 완료 · [PR #107 리베이스 병합](https://github.com/ohah/spinon/pull/107); C09.2–C09.4 미구현 · **공개 API:** 아님

`0047`은 문서 ID다. 구현·검증·문서 개정만으로 앱·crate·내부 계약의 숫자 버전을 올리지 않는다. C08.1 PR #105와 C09 계획·reference가 병합됐고, C09.1 runtime 구현은 PR #107로 병합됐다. 내부 fixture profile의 완료이며 전체 C09 또는 공개 CSS 지원 완료를 뜻하지 않는다.

## 1. 입력과 profile

- 신규 profile 이름은 `RuntimeBlockFormattingV1`로 제안한다. 기존 `RuntimeBlockPaintV1` 또는 Flex profile의 입력·오류 의미를 바꾸지 않는다.
- 입력은 같은 HostDocument snapshot, layout root, viewport/containing block, cascade result와 source/document/style/environment revision tuple이어야 한다. 반환 frame과 진단도 동일 tuple을 echo한다. tuple 일부가 달라지면 전체 계산을 거부한다.
- CSS computed value는 Stylo가 소유하고 레이아웃 adapter는 typed 값을 projection한다. CSS 문자열 재파싱, 기본값 대체, 오류 뒤 부분 frame 공개를 하지 않는다.
- 이 계약의 초기 성공 범위는 `horizontal-tb`, `direction:ltr`, in-flow non-replaced Block box와 `display:none` subtree다. visible text, line box, float/clear, positioned, inline/table/flex/grid, replaced intrinsic sizing은 별도 소유 항목이 준비되기 전까지 profile 밖이다.
- C04.8 의미상 HostRoot 직속 Element는 CSS `:root`/`documentElement`가 아니라 독립 fragment cascade root다. 초기 성공 범위는 HostRoot 직속 Element 하나이며 여러 root와 `body`·`documentElement` 의미는 제외한다. viewport는 containing block 입력으로 제공하고 DOM·paint node로 노출하지 않는다. Chromium 기준 fixture는 `html, body { margin:0; padding:0 }`로 초기 여백을 없앤 뒤 viewport 크기의 `display:flow-root` wrapper 안에 일반 앱 Block 하나를 둔다. case별 wrapper는 테스트 문서에서 viewport 원점에 절대 배치해 겹친다. 비교기는 inventory의 case 하나를 HostRoot 직속 Element로 materialize하고 wrapper와 다른 case는 제품 트리에서 제외하며, viewport를 별도 containing-block 입력으로 준다. wrapper는 HTML root/body 특례, wrapper-child margin collapse, 숨김 box의 문서상 위치에 따른 좌표 오염을 차단하는 fixture harness이며 Spinon node/API가 아니다. 앱 Block 자신의 margin과 viewport 기준 frame만 비교한다. CSS root margin propagation은 이 profile에 적용하지 않는다.

## 2. C09.1 normal Block geometry

- 상위 Block의 content box가 일반 자식의 containing block이다. viewport containing block, nested definite·indefinite size, 기존 C06 percentage basis, C07 box-sizing·padding·border·min/max를 함께 대조한다.
- 일반 Block width 방정식의 `auto` width 및 LTR 좌우 `auto` margin 분배, 모든 값 지정 시 over-constrained LTR의 우측 margin 처리, 수직 auto margin 사용값, negative/fractional margins와 자식 document order를 다룬다. C06/C07의 단위와 box model 계약을 복사해 새 환산 규칙을 만들지 않는다.
- `display:none` subtree는 margin strut·배치·paint에 참여하지 않는다. DOM preorder frame 기록은 C04.9 계약대로 유지하고 해당 node와 자손의 frame은 모두 0이어야 한다. 이전 revision frame으로 대체하지 않는다.
- RTL, vertical writing, inline line box, replaced element content measurement는 이 계약의 성공 조건이 아니다.

### C09.1 구현의 지원 경계

| 입력 | 현재 동작 |
| --- | --- |
| root·flow | HostRoot 직속 HTML element 하나를 수평 쓰기·LTR normal-flow Block으로 계산한다. viewport는 합성 Taffy containing block이고 DOM/frame 결과에 포함하지 않는다. |
| display | `block`, `none`을 계산한다. inline·flow-root·flex·grid·table·contents와 CSS-wide 초기/상속 값은 계산 결과가 `block`으로 정규화되더라도 원문 선언을 검사해 실패한다. |
| 크기 | `width:auto`와 기존 C06 typed length·percentage·CSS math, C07 box-sizing·padding·used border·min/max를 이어 쓴다. 일반 Block width 방정식과 수직 normal flow로 auto height를 계산한다. |
| margin | 네 면의 기존 length·percentage·음수 margin과 C09.1 좌우 `auto` 분배를 계산한다. 세로 `auto` 사용값은 0으로 처리한다. 수직 margin collapse는 아직 하지 않는다. |
| 스타일 선언 | C09.1 profile의 제한 author-property 목록만 받는다. `display`의 알려진 지원 밖 값은 inline·`<style>` 원문을 CSS token parser로 검사한다. 사전 검사기는 `@media`·`@supports`와 CSS 중첩 규칙 안의 style rule도 깊이 64까지 순회한다. 다만 at-rule 자체는 현재 cascade profile의 지원 범위 밖이며 실패한다. 그보다 깊은 규칙도 fail-closed로 거부한다. |
| 실패 경계 | rtl 방향 지정, position, float, clear, overflow, transform, writing-mode, visible text와 여러 root는 성공 layout이 아니다. inline display 오류는 `unsupported_block_display`와 node/value를, author stylesheet 사전 검사 오류는 `unsupported_block_stylesheet`와 stylesheet ID/원인을 담는다. |

이 표는 PR #107로 병합된 C09.1의 테스트된 내부 범위다. `border-width`는 geometry에만 반영되며 border 선 paint는 포함하지 않는다. 전체 HTML/CSS 지원, C09.2–C09.4 또는 공개 API 지원을 뜻하지 않는다.

현재 fail-closed 검사는 author input 전체를 대상으로 한다. 따라서 선택되지 않는 stylesheet selector 안의 `display:grid`나 뒤에서 `display:block`으로 덮는 fallback 선언도 거부한다. 적용 요소·cascade 승자만 지원 판정하는 웹과 같은 conditional matching은 아직 없다. 중첩 at-rule과 CSS 중첩 규칙은 64단계까지만 순회하고 더 깊으면 실패한다. 이 제약을 완화하려면 selector match 및 cascade winner를 보존하는 값 검사를 별도 작업으로 다뤄야 한다.

## 3. C09.2 margin collapse

- 세로 margin만 같은 BFC의 in-flow Block 사이에서 collapse한다. horizontal margin은 collapse하지 않는다.
- 대상 adjacency는 형제, 부모-첫 자식, auto-height 부모-마지막 자식, empty/self-collapsing Block이다. positive·negative·zero·fractional·세 개 이상 margin의 strut 결합을 각각 검증한다.
- 여러 양수 margin의 결합은 양수 최댓값, 음수의 결합은 절댓값 최댓값을 양수 최댓값에서 뺀 결과를 대조한다. 양수 없이 음수만 있는 경우도 별도 oracle로 둔다.
- collapse 경계는 선언 문자열이 아니라 computed style과 C07.2의 면별 **used** border width/padding을 입력으로 판단한다. used border가 0인 경우와 nonzero인 경우를 구분한다. border paint는 이 계약의 결과물이 아니다.
- 중첩된 부모·자식 사이의 margin collapse 여부는 면별 border/padding, used height, min-height와 in-flow child 조건별 fixture로 검증한다. HostRoot fragment Element에는 CSS parent가 없으므로 root 경계에서 부모-자식 collapse는 발생하지 않는다. 여러 HostRoot root는 이 profile 밖이다. Clearance·float interaction은 C26 전까지 미지원이다.

## 4. C09.3 BFC와 flow-root

- C09가 추가하는 명시적 BFC 생성 값은 `display:flow-root`다. computed `block`과 `flow-root`를 구분하고, flow-root 내부 자식 margin은 그 box의 외부 margin과 collapse하지 않는다.
- flow-root의 내부 BFC 경계와 box 자체의 외부 margin을 분리한다. 내부 자식과는 collapse하지 않지만 box 자체의 위·아래 margin은 비-BFC 부모의 첫째/마지막 자식, 상위 BFC의 이전/다음 형제와 각각 collapse할 수 있으므로 중첩 fixture로 검증한다. fragment root에는 CSS parent가 없으며 root boundary에서 parent-child collapse를 만들지 않는다.
- `overflow`, position, float, flex/grid, table, inline-block 등이 만드는 formatting context는 이 profile에서 추정하거나 `block`으로 치환하지 않는다. 각 기능의 C12/C13/C10/C11/C15/C26 구현 단계에서 C09 fixture로 교차 검증한다.
- `flow-root` 하나를 구현했다고 모든 BFC 생성 조건, float containment 또는 외부 float 상호작용을 지원한다고 표시하지 않는다.

## 5. C09.4 shrink-to-fit 의존 계약

- 사용처를 float non-replaced auto width, inline-block non-replaced auto width, absolute non-replaced 중 CSS 2.1 §10.3.7 cases 1·3처럼 inset/width 조합이 shrink-to-fit을 선택하는 경우로 나누고 각각 C26, C15, C12 구현과 연결한다. absolute의 모든 `width:auto`가 shrink-to-fit인 것은 아니다.
- preferred minimum/min-content와 preferred/max-content 측정은 C14/C15에서 소유한다. CSS 2.1 설명식 `min(max(preferred minimum width, available width), preferred width)`를 사용할 때도 실제 결과는 pinned Chromium의 사용처별 geometry와 대조한다. CSS 2.1은 세부 algorithm 전체를 규정하지 않으므로 해당 식만으로 pass를 정하지 않는다.
- shrink-to-fit은 `width:fit-content`의 CSS computed value와 같은 기능이 아니다. 줄바꿈, available inline size, padding/border, min/max와 containing block을 사용처별로 검증한다.
- Taffy 0.14.0 float layout feature는 현재 Spinon dependency에서 비활성이다. feature 변경이나 Taffy `fit-content` 매핑은 동등성 증거가 아니다. 선행 intrinsic measurement가 없거나 basis가 indefinite하면 node/property 오류로 실패해야 한다.
- 해당 owner 작업과 Chromium/Rust/runtime 비교가 준비될 때까지 C09.4는 미구현, C09 상위도 미완료다.

## 6. 비교·오류·검증

- 구현 전 기준 artifact는 [`c09-block-formatting-v1.json`](../../tests/fixtures/css/references/c09-block-formatting-v1.json)이다. 고정 Chromium `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, viewport `320×240` CSS px, DPR 1·2와 23개 case·76개 node를 담는다. 실행 파일 hash, HTML·inventory·capture digest, locale/timezone/color scheme/pointer와 capture provenance가 맞지 않으면 비교를 중지한다. 재생성 명령은 `bun run css:reference:c09-block-formatting`이며 기존 JSON을 덮어쓰지 않는다.
- authored/computed margin 값은 computed value oracle로, collapse가 반영된 결과는 node별 `getBoundingClientRect()` geometry로 따로 대조한다. 각 지원 node의 x/y/width/height 필드마다 최대 오차 `0.5 CSS px`, DPR 간 CSS geometry 일치를 요구한다. screenshot이나 평균 오차로 실패를 숨기지 않는다.
- Chromium 원본 HTML·inventory를 Rust 비교기 및 실제 runtime fixture의 공통 입력으로 쓴다. runtime은 case 하나를 별도 HostDocument root로 materialize하며 fixture wrapper를 제품 트리에 복제하지 않는다. node ID·부모·document order를 보존하며 각 platform V8 실행은 예상 node별 CSS frame과 revision을 수집해 같은 reference에 대조한다.
- unsupported declaration, Stylo parser 진단, 불명확한 basis, 잘못된/non-finite geometry, stale revision, Taffy 오류는 node/property가 식별되는 전체 실패다. 이전 partial tree나 이전 revision scene을 새 계산의 성공 결과로 보이지 않는다.
- 계획 검토 20개 관점은 기능 구현 검토와 별도다. 구현 뒤에는 새 20개 failure perspective를 작성한다. 둘 중 하나도 다른 문서의 표를 재사용하지 않는다.

## 7. 공식 참고

- [C09 계획·하위 작업·판정 기준](../../plan/c09-block-formatting.md)
- [계획의 fixture 통합 재검토](./evidence/c09-block-formatting-plan-review-precomparison-followup-2026-10-10.md)
- [Chromium 사전 비교 결과·도구 검토](./evidence/c09-block-formatting-precomparison-2026-10-10.md)
- [C09.1 구현 실패 관점 검토·Android/iOS Simulator 근거](./evidence/c09-block-formatting-implementation-review-2026-10-10.md)
- [고정 Chromium HTML fixture](../../tests/fixtures/css/c09/block-formatting.html) · [inventory](../../tests/fixtures/css/c09/block-formatting-inventory.json) · [capture 도구](../../tools/css-reference/capture-c09-block-formatting.mjs) · [reference 테스트](../../tools/css-reference/c09-block-formatting.test.mjs)
- [CSS 2.1 margin collapse](https://www.w3.org/TR/CSS21/box.html#collapsing-margins), [Block formatting context](https://www.w3.org/TR/CSS21/visuren.html#block-formatting), [containing block와 width](https://www.w3.org/TR/CSS21/visudet.html#containing-block-details)
- [CSS 2.1 §10.3.5 float](https://www.w3.org/TR/CSS21/visudet.html#float-width), [§10.3.7 absolute non-replaced](https://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width), [§10.3.9 inline-block](https://www.w3.org/TR/CSS21/visudet.html#inlineblock-width) shrink-to-fit 문맥. 정확한 알고리즘을 정의하지 않은 부분은 pinned Chromium과 대조한다.
- [CSS Display Level 3 `flow-root`](https://www.w3.org/TR/css-display-3/#flow-root) — Editor’s Draft; Chromium 고정 reference가 구현 비교 oracle이다.
- [Taffy 0.14.0 문서](https://docs.rs/taffy/0.14.0/taffy/) — Spinon은 `=0.14.0`을 pin하고 `default-features=false`로 `float_layout`을 끈다.

이 계약은 미출시 내부 계약이다. RuntimeBlockFormattingV1은 C09.1 내부 fixture에서 구현·검증했지만 공개 API나 일반 CSS 지원으로 게시하지 않는다.

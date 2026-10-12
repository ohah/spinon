# C11.2.2 · 행 우선 sparse 자동 배치와 암시 행

**상태:** 계획 검토 완료 · Chrome 154 사전 비교 고정(16 scene·69 node, DPR 1·2) · 고정 WPT 후보 Chrome 기준 2/2 통과(Spinon 적합성 아님) · C11.2.1 통합 관문 대기 · runtime 구현 미시작  
**상위:** [C11 CSS Grid](c11-css-grid.md) · [C11.2.1 명시 배치](c11-2-1-explicit-grid-placement.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)  
**계획 실패 관점 검토:** [초기 계획 검토](../spec/internal/evidence/c11-2-2-sparse-row-placement-plan-review-2026-10-12.md) · [커서 되감기 장면 보강 후 재검토](../spec/internal/evidence/c11-2-2-sparse-row-placement-plan-review-2026-10-12-v2.md)  
**사전 비교:** [초기 Chrome 154 기준 v3](../tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v3.json) · [보강한 Chrome 154 기준 v4](../tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json) · [v4 장면·실행·캡처 근거](../spec/internal/evidence/c11-2-2-sparse-row-placement-precomparison-2026-10-12-v4/README.md) · [v4 도구·fixture 검토](../spec/internal/evidence/c11-2-2-sparse-row-placement-precomparison-implementation-review-2026-10-12-v2.md)  
**표준 기준:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §7.5–7.7, §8.3–8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)  
**기준 엔진:** 기존 고정 Chromium 154 환경 · 현재 잠금된 Taffy `0.14.0`

## 선행 조건

C11.2.2 runtime 코드는 C11.2.1 구현이 별도 변경으로 통합되고 Rust·Chrome fixture 검사가 통과한 뒤 시작한다. Android 실기기 10-scene PixelCopy 행렬은 2026-10-13 SM-S731N에서 통과했다. Spinon WPT runner·전체 적합성은 X16에서 별도 추적하며 C11.2.2 runtime 시작 조건으로 두지 않는다. 이 계획 문서와 기준 fixture 작성은 선행 통합 전에 준비할 수 있다.

## 목표

C11.2.1의 explicit-only placement 다음으로 한 Grid 안에서 definite item과 auto item을 함께 배치하고, CSS 기본값인 행 우선 sparse 배치가 필요로 하는 양의 암시 행을 만든다. 지원 값은 Stylo computed longhand에서 typed 입력으로 옮기고, 검증된 배치 입력만 기존 Taffy custom tree에 전달한다. 기존 C11.1·C11.2.1 profile의 성공 범위는 바꾸지 않는다.

이번 단계는 Grid Level 1 전체 구현이 아니다. 다음 고정 subset의 Chromium 동작을 맞추고 그 밖의 입력은 계산 전체를 구체적인 오류로 거부한다.

## 지원 계약

### 컨테이너

- `grid-template-columns`와 `grid-template-rows`가 각각 하나 이상의 기존 C11.1 track으로 계산되어야 한다. definite container size, `px`·definite `%`·양의 유한 `fr`, 기존 gap 제한을 유지한다.
- `grid-auto-flow`는 computed `row`만 허용한다. 기본값 `row`를 포함한다. `row dense`, `column`, `column dense`는 지원하지 않고 거부한다.
- computed `grid-auto-rows`는 초기값 `auto` 또는 단일 비음수 `px` 길이만 허용한다. `auto`는 암시 행이 0개일 때만 미사용 값으로 허용하며, 암시 행이 필요하면 거부한다. 여러 track, `%`, `fr`, `minmax()` 등은 암시 행 유무와 관계없이 거부한다.
- 암시 track은 auto-placement가 명시 Grid의 끝을 넘어서 만들게 되는 양의 행으로 제한한다. 음의 암시 행과 암시 열은 이번 단계에서 거부한다.
- 음의 암시 track을 만들지 않는 이번 profile은 행·열 각각 `explicit tracks + positive implicit tracks ≤ 10,000`을 지킨다. Taffy `0.14.0`의 `MAX_GRID_TRACKS`가 positive 방향 explicit+implicit track에 적용되므로, 두 수의 합을 checked arithmetic으로 Taffy 호출 전에 검사한다. 상한에 닿으면 잘라 성공시키지 않고 계산 전체를 거부한다.

### Grid item

- 배치 분류는 각 Grid container의 보이는 직계 in-flow 자식만 대상으로 한다. `display:none` subtree는 기존과 같이 제외한다. 중첩 Grid container는 각자 독립적인 배치 모드를 가진다.
- 기존 C11.2.1의 definite numeric line/span 배치를 유지하면서 같은 container의 auto item과 섞을 수 있다. 양수·음수 line은 explicit track 기준으로 정규화하고, explicit line 범위 밖의 선언은 이번 단계에서도 거부한다.
- Auto-placement에서 사용할 longhand 조합은 `auto/auto`, line과 `auto`, line과 positive span, `auto`와 positive span 및 이미 구현된 두 definite endpoint다. CSS conflict handling(역순 line 교환, 동일 line의 end 제거, 두 span 중 end span 제거)을 먼저 적용한다. named line/span과 계산 결과를 표현할 수 없는 조합은 거부한다.
- definite row와 auto column인 item은 해당 explicit row에서 빈 explicit column 범위를 찾는다. 맞는 위치가 없으면 CSS상 암시 열이 필요하므로 전체 계산을 거부한다.
- 모든 definite item을 먼저 occupancy에 등록한 뒤 definite row/auto column 항목을 처리하고, 남은 항목을 order-modified document order로 처리한다. 이번 범위는 `order:0`만 허용하므로 이 순서는 DOM/source order와 같다.
- 남은 단계에서 definite column/auto row 항목은 cursor의 column을 해당 항목의 definite column으로 옮긴다. 새 column이 직전 cursor column보다 작으면 row를 먼저 한 칸 증가시킨 뒤 겹치지 않는 첫 row를 찾는다. 따라서 앞서 지나간 빈칸을 다시 채우지 않는다. 공간이 없으면 양의 암시 행을 추가한다. 두 축 모두 auto인 항목은 같은 cursor에서 row-major sparse 검색을 계속한다.
- definite row/auto column 항목은 해당 explicit row 안에서 이전 row-locked 항목의 cursor보다 뒤에 있는 첫 빈 explicit column 범위를 찾는다. 필요한 column이 explicit grid 밖이면 암시 열을 만들지 않고 전체 계산을 거부한다.
- auto-placement item의 width·height는 고정 길이만 허용한다. text/replaced/intrinsic sizing, auto min-size, baseline, auto item sizing은 C14/C15 의존 단계로 남긴다.
- `order`는 계속 `0`만 허용한다. 따라서 이번 단계의 order-modified document order는 DOM/source order와 같다. HostDocument 자식 배열을 재정렬하지 않는다.

### 구현 경계

- C11.2.1의 `RuntimeBlockFlexGridPlacementV1`에 성공 의미를 추가하지 않고 새 내부 profile `RuntimeBlockFlexGridAutoPlacementV1`과 전용 FFI fixture 진입점을 둔다. 이름과 JSON/schema 식별자는 구현 전 계약·fixture inventory에 고정한다.
- Stylo는 placement longhand와 `grid-auto-flow`·`grid-auto-rows`의 computed typed 값을 제공한다. 자체 CSS 문자열 parser를 만들지 않는다.
- layout adapter는 CSS origin-zero line/span을 검증한 뒤 Taffy `0.14.0`의 placement·auto-row 입력으로 변환한다. Taffy의 clamp·implicit sizing이 unsupported CSS를 성공처럼 보이게 하지 않도록 범위 검사를 Taffy 호출 전에 수행한다.
- 검증·배치·track 확장이 모두 성공한 revision만 publish한다. 부분 frame, 잘린 track, 이전 revision 혼합 결과는 반환하지 않는다. 실패 뒤 같은 worker에서 얕은 유효 입력의 회복도 확인한다.
- 내부 계약 및 crate 숫자 버전은 `0.1.0`으로 유지한다. 내부 fixture 결과는 공개 CSS 지원표에서 지원으로 승격하지 않는다.

## 비교 fixture와 검증 설계

### 구현 전 고정 기준

1. C11.2.1의 reference와 inventory를 덮어쓰지 않는 새 C11.2.2 fixture를 만든다. Chrome 154 binary/revision, capture tool, helper, HTML, inventory, screenshot, DPR 1·2의 SHA-256을 저장한다.
2. Chrome oracle에는 최소 다음 13개 지원·거부 scene을 둔다.
   - 지원 후보: explicit item 뒤의 auto item, auto item만으로 explicit grid 채우기, implicit row 생성, definite row/auto column, definite column/auto row, **definite column이 이전 cursor보다 작아 row가 증가하는 경우**, span된 auto item, explicit overlap이 있는 sparse 배치, gap과 track sizing, hidden item 제외, 중첩 container의 서로 다른 placement mode.
   - 거부 후보: 암시 열이 필요한 경우, 암시 행 `auto`·복수 track sizing, dense/column flow.
3. scene마다 preorder NodeId/parent, computed placement longhand, explicit/implicit row·column 수, item rect, viewport, track/gap 값을 기록한다. 모든 지원 frame field의 절대 오차는 `0.5 CSS px` 이하로 판정하고 누락·중복 ID나 예상과 다른 rejection은 별도 실패다.
4. WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 기존 후보를 다시 hash 검사한다: `css/css-grid/placement/grid-layout-grid-span.html` (`1be809e54f1b7398112923acd60ef423735934b528ff42c1a8f3b326adbebef7`) 및 `css/css-grid/placement/grid-negative-line-placement-001.html` (`e5d17120d87170282d2522ec483d751c6f7db9ac23ec09d2a540cce305d81d31`). 이 원문 전체를 C11.2.2 subset과 동일시하지 않는다. 지원 동작만 개별 fixture에 대응시키고, 원문 Chrome 실행 결과와 Spinon 제한 fixture 결과를 분리한다. subset을 정확히 덮는 후보가 없으면 새 WPT를 임의로 주장하지 않고 미선정 이유를 기록한다.

### 완료 관문

| 관문 | 통과 조건 | 실패 시 처리 |
| --- | --- | --- |
| Typed cascade | shorthand·longhand cascade와 computed placement 값이 Chrome fixture와 일치하고 값·node·property 진단을 보존한다. | 부정확한 변환이나 무관한 child의 값으로 container 전체를 잘못 거부하면 실패. |
| 배치 알고리즘 | definite-first, row-locked, definite-column, fully-auto 순서와 sparse cursor가 reference의 cell occupancy·implicit row 수와 같다. | DOM order 재배열, hole backfill, overlap 오판, 암시 열 생성은 실패. |
| Track/layout | 지원 `grid-auto-rows`와 row/column gap 및 fixed-size item의 모든 rect가 Chrome 허용 오차 안이다. | implicit auto sizing 또는 unsupported track을 Taffy 기본값으로 성공 처리하면 실패. |
| 실패 원자성 | 잘못된 placement, line/span overflow, 10,000 track 초과, 예약 실패에서 새 revision 결과가 publish되지 않고 이전 scene이 잘못된 새 revision으로 보이지 않는다. 유효 후속 계산은 같은 worker에서 회복한다. | 부분 frame·stale snapshot·panic을 정상 결과로 분류하면 실패. |
| Runtime | Android와 iOS에서 동일 고정 V8 fixture가 FFI→Rust→WGPU 경로를 통과하고 node ID·revision·frame·draw marker를 수집한다. | AVD/Simulator 결과와 실기기 결과를 합치지 않는다. draw marker를 pixel 검증으로 부르지 않는다. |
| Surface | 지원 scene의 track/gap/item interior sample을 Android PixelCopy와 iOS Metal drawable에서 Chrome 색상과 node별 비교한다. | screenshot을 surface readback 대신 쓰지 않고, compositor 이후 색상이나 hardware 성능으로 과장하지 않는다. |
| WPT | Chrome 공식 runner에서 고정한 자동 WPT 후보를 실행하고, Spinon 결과는 공식 runner 통합 전까지 자체 fixture 결과로만 보고한다. | Chrome 통과를 Spinon 적합성으로 표기하지 않는다. |

## 구현 순서

- [x] 초기 15-scene·66-node 기준을 보존하고, definite-column cursor가 뒤로 이동할 때 row를 증가시키는 누락 장면을 추가해 Chrome 154의 16-scene·69-node reference v4를 DPR 1·2로 고정했다. 16개 DPR 1 screenshot, 각 scene의 explicit/computed/implicit track 수, WPT source hash 및 negative inventory를 저장했다. 신규 장면에서 Grid 콘텐츠 기준 column 2 item은 `(44, 0)`, 뒤이어 column 1에 고정한 item은 cursor row-wrap으로 `(0, 24)`에 놓이고 explicit row 하나 뒤에 implicit row 하나가 생성됐다. 고정 WPT 후보 두 개는 기존 WPT runner에서 Chrome `154.0.8037.98` 기준 각각 PASS, negative-line 7/7 subtest PASS다. 이는 Chrome oracle 결과이며 Spinon 적합성이나 현재 profile 전체 통과를 뜻하지 않는다. [기준 v3](../tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v3.json) · [기준 v4](../tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v4.json) · [v4 실행 근거](../spec/internal/evidence/c11-2-2-sparse-row-placement-precomparison-2026-10-12-v4/README.md).
- [ ] `GridPlacementInput` 분류·CSS conflict handling·범위 검증을 unit test-first로 확장한다.
- [ ] typed `grid-auto-rows`/`grid-auto-flow` 투영과 새 runtime profile을 연결한다.
- [ ] Taffy adapter의 sparse placement·양의 암시 행 sizing을 연결하고 track 상한·예약 실패·원자성 회귀를 추가한다.
- [ ] Rust→FFI→V8 단위 장면을 먼저 확인한 뒤 Android AVD/실기기와 iOS Simulator에서 전체 지원·거부 scene을 실행한다. 연결되지 않은 기기는 별도 미완료로 남긴다.
- [ ] 실제 surface PixelCopy/Metal readback을 수집하고 Chrome 표본과 비교한다.
- [ ] 구현 diff와 로그를 사용한 별도 20개 실패 관점 검토를 끝낸다. 수정이 있으면 영향을 받는 검증을 다시 실행한다.
- [ ] `plan/c11-css-grid.md`, `spec/STATUS.md`, `spec/internal/README.md`, public roadmap 및 Tailnet preview를 같은 근거로 동기화한다. GitHub Pages 배포는 하지 않는다.

## 이번 단계에서 제외

`grid-auto-flow:dense`, column flow, implicit columns, negative implicit rows, explicit line으로 implicit track 생성, 복수 `grid-auto-rows` 반복 pattern, `auto`·`%`·`fr`·`minmax()` implicit sizing, named lines/areas, `repeat()`·auto-repeat, `order`, text/replaced/intrinsic content, auto min-size, baseline/alignment 확장, absolute-positioned child, scroll/overflow, writing mode·RTL, subgrid, masonry, full WPT runner 및 성능 우위는 포함하지 않는다. dense/column/implicit column은 다음 C11.2.3 계획에서 별도 범위로 다룬다.

이 단계의 완료는 C11.2.1·C11.2·C11 상위 항목, 전체 CSS 지원 또는 웹/모바일 전체 동등성을 완료로 표시하지 않는다.

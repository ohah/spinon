# C11.1 교차축 부분 Grid 배치 계획 실패 관점 검토 20개

대상 계획: [`c11-root-html-grid-mixed-cross-axis.md`](./c11-root-html-grid-mixed-cross-axis.md)

기준: [CSS Grid 2025 CRD §8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-placement-algo), [Chrome 156 precomparison](../spec/internal/evidence/c11-root-html-grid-mixed-cross-axis-precomparison-2026-11-04/README.md), 기존 one-axis/mixed-explicit 계약.

| # | 실패 관점 | 판정과 계획상 방어 |
|---:|---|---|
| 1 | DOM order interleave가 row-locked phase를 column-locked보다 먼저/나중으로 바꾸는가 | CSS Grid phase 순서를 고정하고 `dom-cross-axis` scene에 column-locked item을 먼저 둔다. PASS. |
| 2 | fully definite item이 partial item 뒤에 선언되면 occupancy가 비는가 | definite placement는 먼저 점유하며 마지막 DOM의 `d-last` 대조가 있다. PASS. |
| 3 | column-locked cursor가 row-locked cursor 위치를 잘못 되감는가 | mixed-span과 DOM-interleaved expected frame을 별도 고정한다. PASS. |
| 4 | row-locked와 column-locked를 같은 auto axis로 오분류하는가 | 분류 조건과 selector 요구사항에 각 유형이 같은 owner에 하나 이상 존재해야 한다고 적었다. PASS. |
| 5 | fully auto item이 교차축 profile에서 누락되는가 | 남은 placement 단계에서 허용하고 `auto-last` 기대 frame을 고정한다. PASS. |
| 6 | all-auto-only grid가 새 mode에 끌려 들어오는가 | selector는 두 방향 partial 유형이 모두 있을 때만 선택한다. PASS. |
| 7 | row-only/column-only 기존 route가 바뀌는가 | 새 selector의 cross-axis prerequisite와 기존 profile 회귀 조건이 있다. PASS. |
| 8 | `display:none` item이 분류 수나 occupancy를 바꾸는가 | visibility filtering을 분류 이전에 하고 별도 Chrome control을 둔다. PASS. |
| 9 | `visibility:hidden`이 occupancy에서 빠지는가 | hidden row/column scene이 visible successor 위치를 검사한다. PASS. |
| 10 | row span이 여러 cell을 점유하지 않아 겹침이 생기는가 | row-span과 후속 placement가 2행 occupancy를 구분한다. PASS. |
| 11 | column span이 cursor collision을 놓치는가 | col-span과 col-after frame을 고정하고 item 영역 크기를 span에 맞췄다. PASS. |
| 12 | negative numeric line이 explicit grid 밖으로 잘못 정규화되는가 | numeric line 허용은 explicit grid 내부로 제한하고 out-of-range를 rejection matrix에 둔다. PASS. |
| 13 | `grid-row:4/5`가 implicit row를 만들고 성공으로 오인되는가 | Chrome은 유효 결과를 관찰하되 Spinon slice는 atomic rejection으로 명시한다. PASS. |
| 14 | implicit `grid-auto-*` value가 implicit extent 허용으로 해석되는가 | auto-track value와 implicit extent support를 구분해 문서화했다. PASS. |
| 15 | dense 또는 column flow가 row-sparse algorithm으로 우회하는가 | 새 selector에서 `grid-auto-flow:row`만 허용하고 dense/column을 거부한다. PASS. |
| 16 | named line/area가 numeric path로 추측 변환되는가 | 이름 기반 placement와 area는 fail-closed 범위에 명시했다. PASS. |
| 17 | `order`로 order-modified order가 달라지는데 source order로 처리되는가 | `order:0` 고정과 별도 order 지원 제외를 명시했다. PASS. |
| 18 | 여러 Grid owner 간 item/status가 섞이는가 | selector/validator는 owner 단위로 분류하고 cross-owner 공유 금지. PASS. |
| 19 | rejection 후 일부 새 frame 또는 이전 revision scene이 publish되는가 | failure 시 전체 layout/render output atomic rejection과 stale revision control을 요구한다. PASS. |
| 20 | 시뮬레이터 근거를 실기기/hardware GPU/WPT로 확대하는가 | AVD, iOS drawable 단계, WPT와 physical/hardware 한계를 명시한다. PASS. |

**결과:** 서로 다른 20개 실패 관점을 검토해 현재 계획에서 blocker를 찾지 못했다. 구현 후 새 runtime/code 관점 검토는 별도로 수행한다.

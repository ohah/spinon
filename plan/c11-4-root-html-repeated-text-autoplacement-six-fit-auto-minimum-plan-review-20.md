# C11.4 six-fit auto-placement `min-height:auto` 계획 검토 20개

**검토 대상:** [계획](./c11-4-root-html-repeated-text-autoplacement-six-fit-auto-minimum.md)와 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-six-fit-auto-min-precomparison-2026-11-11/README.md)

계획 PR 전의 적대 검토다. 각 항목은 서로 다른 입력 축, CSS Grid 불변 조건, revision 경계 또는 주장의 범위를 확인했다. 계획 검토는 구현/runtime 검토 및 실제 계획 PR diff 검토와 분리한다.

| # | 실패 관점 | 계획의 방어 | 판정 |
|---:|---|---|---|
| 1 | six-fit auto-placement 조건이 임의 Root HTML Grid로 넓어짐 | 단일 Root HTML Grid owner, 고정 geometry/text/topology만 admission한다. | 통과 |
| 2 | `fit-content()`의 minimum track sizing function을 `auto`로 보지 않아 eligibility를 놓침 | W3C 2025 CRD §11.2의 track minimum과 §6.6의 automatic minimum 기준을 함께 고정했다. | 통과 |
| 3 | fit cap이 automatic minimum content floor를 자른다고 오해 | §6.6의 cap 한계를 계획에 적고 Chrome baseline/candidate payload floor를 각각 비교한다. | 통과 |
| 4 | scroll container에도 non-scrollable grid item의 automatic minimum을 적용 | candidate는 non-scrollable item만 허용하고 scroll overflow 입력은 fail-closed 대상이다. | 통과 |
| 5 | fit track이 아닌 explicit row를 six-member implicit list의 첫 track으로 셈 | 첫 control은 explicit row이며 auto-placement된 여덟 item은 implicit list position 1부터 시작한다. | 통과 |
| 6 | list의 여덟 item에서 여섯 track 이후 cycle wrapping을 잘못 계산 | 여덟 direct text item을 forward/reverse source order로 모두 유지해 7·8번째 배치를 비교한다. | 통과 |
| 7 | authored cap을 다른 track 위치로 옮기거나 track order를 정규화하면서 잃음 | CSS typed cap과 authored index/order를 same-revision trace 및 Chrome computed/used values와 대조한다. | 통과 |
| 8 | `%` cap을 indefinite basis에서 계산하거나 임의 basis로 재사용 | definite 220px block basis를 고정하고 indefinite percentage는 거부한다. | 통과 |
| 9 | 사전 비교의 여섯 cap 순열에서 특정 assignment가 빠짐 | 여섯 distinct cap 값의 `6! = 720` authored assignments를 모두 관측한다. | 통과 |
| 10 | 사전 비교 cap corpus를 넘어 중복 또는 다른 finite cap이 admission됨 | 지원 cap multiset을 정확히 `{0px, 10px, 20%, 30px, 40px, 50px}`로 제한하고 duplicate/missing/unlisted 값을 거부한다. | 통과 |
| 11 | baseline과 candidate 사이에 alignment·source order 외 값이 바뀌어 잘못 pair됨 | pair key에는 track list/alignment/DOM order를 두고 minimum mode만 비교 축으로 변경한다. | 통과 |
| 12 | 8개 text item의 payload line box와 outer grid item frame을 같은 값으로 간주 | 각 row, item frame, payload frame, line count/top을 따로 수집하고 검증한다. | 통과 |
| 13 | `align-content:stretch`에서 item 위치 변화가 없다는 결론을 row 높이만 보고 냄 | row height뿐 아니라 item/payload frame과 line top의 전체 geometry를 start와 비교한다. | 통과 |
| 14 | forward/reverse DOM order를 CSS visual order와 혼동 | DOM item ID 순서, auto-placement 위치와 payload line box를 별도로 관측한다. | 통과 |
| 15 | mixed `min-height` 또는 빈 explicit control의 `auto`가 candidate corpus로 누출 | 여덟 item mode는 전체 `0` 또는 전체 `auto`, control은 항상 `0`으로 고정한다. | 통과 |
| 16 | computed `calc()` 결과와 unresolved calc를 같은 admission으로 처리 | computed absolute px는 resolved cap으로 판단하고 unresolved calc는 거부한다. | 통과 |
| 17 | nested/inline/replaced content 또는 다른 Grid owner가 고정 direct-text 결과를 바꿈 | 해당 topology는 support 밖으로 두고 rejection matrix와 추가 owner check를 둔다. | 통과 |
| 18 | stale layout이나 불완전 trace가 성공 결과처럼 publish되고 rejection 후 복구 실패 | owner/style/layout revision과 trace를 결합하고 atomic fail-closed 뒤 동일 session recovery를 요구한다. | 통과 |
| 19 | 기존 1–5-fit, row-span 및 일반 Grid 경로가 새 six-fit 판정에 흡수되어 회귀 | 신규 predicate를 별도로 유지하고 모든 이전 profile 및 unrelated Grid 회귀를 별도 gate로 둔다. | 통과 |
| 20 | Chrome headless 결과를 앱, simulator pixel parity, WPT 또는 C11.4 완료로 표현 | Browser oracle, RuntimeSession matrix, iOS/Android visual smoke, 미포함 범위를 분리하며 공식 `78/214`와 C11.4 상태를 유지한다. | 통과 |

## 판정

20개 관점에서 계획 범위를 바꿀 결함을 찾지 못했다. 사전 비교는 Chrome CSS 동작이고 Spinon 구현 증거가 아니다. 실제 계획 PR diff는 이 표와 별개의 20관점으로 다시 검토한다. 구현 뒤에는 새로운 runtime 검토 20관점과 implementation PR diff 20관점을 별도로 수행한다.

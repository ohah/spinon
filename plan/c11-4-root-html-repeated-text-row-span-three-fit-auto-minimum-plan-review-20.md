# C11.4 three-fit row-span automatic minimum 계획 · 실패 관점 20개

검토 대상은 [계획](./c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum.md), [W3C 2025 CRD 기준](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-three-fit-auto-minimum-precomparison-2026-11-10/README.md), 11,520개 Chrome 157 사전 비교와 현재 `RootHtmlFitContentAutoRowsV1` admission 경계다. 다음 항목은 서로 다른 입력 축·Grid sizing·RuntimeSession·검증·정리 실패를 겨냥한다. 계획 검토는 제품 구현/PR diff 검토와 별도다.

| # | 적대 관점 | 실패 가능성 | 계획의 방어·근거 | 판정 |
|---:|---|---|---|---|
| 1 | fit multiplicity | 0/1/2/4 fit이나 중복 `fit-content()`가 새 positive profile에 진입 | 새 multiset을 정확히 3 fit + min-content/max-content/auto 각 1개로 제한하고 주변 multiplicity를 negative로 둔다 | 통과 |
| 2 | fit-position 조합 | 세 위치의 20개 조합 중 누락·중복·정렬 오류 | 가능한 6개 위치에서 모든 조합 20개를 정하고 strictly increasing positions로 저장한다 | 통과 |
| 3 | cap과 위치 결합 | cap 값은 유지되지만 authored index가 뒤섞임 | 각 위치별 typed cap을 scene key·computed list·track trace와 함께 검사한다 | 통과 |
| 4 | cap 단위 의미 | px와 percentage를 같은 숫자로 비교하거나 percentage를 잘못된 basis로 resolve | definite 220px basis를 고정하고 `0px`, `10px`, `20%`의 여섯 assignment 및 computed authored order를 비교한다 | 통과 |
| 5 | cap이 min-content floor를 자름 | 0px cap을 최대 상한처럼 적용해 content-based minimum을 제거 | Grid 명세상 fit-content cap이 item의 content-based minimum을 fixed maximum처럼 clamp하지 않는 관계를 §6.6/§7.2/§11.5에서 검사한다 | 통과 |
| 6 | tail member 순서 | min/max/auto tail order가 새 triple profile에서 누락·중복 | 남은 세 member의 여섯 permutation을 모두 fixture matrix와 typed projection 검증에 넣는다 | 통과 |
| 7 | implicit list phase | explicit row가 auto-row list 위치를 밀어 fit cap과 used row가 어긋남 | explicit row 하나 뒤 여섯 implicit row를 만들고 track trace의 used row index를 authored list phase와 대응시킨다 | 통과 |
| 8 | automatic-minimum eligibility | 모든 span에 무조건 content minimum을 부여 | visible/non-scrollable, auto-minimum track 포함, flexible track 미포함을 CSS §6.6 조건대로 각 item별 계산한다 | 통과 |
| 9 | `fit-content()` eligibility | fit track의 min sizing function을 cap 또는 max-content로 취급 | §7.2가 정의하는 `auto` minimum function과 Chrome profile을 쓴다 | 통과 |
| 10 | 명시 minimum과 혼합 값 | item별 auto/zero 혼합이 지원된 것처럼 확장 | text span item 전체의 zero 또는 전체 auto만 허용하고 explicit-row control은 늘 zero다 | 통과 |
| 11 | scrollable overflow | scroll container가 된 item까지 content-based minimum 적용 | 이 고정 profile은 visible overflow만 허용하고 scrollable overflow negative를 둔다 | 통과 |
| 12 | flexible track 간섭 | flexible track을 포함하는 multi-track span에도 auto minimum 계산 | 신규 track multiset에는 flexible member가 없고 추가/변형 track list는 fail-closed한다 | 통과 |
| 13 | span topology 폭발 | invalid placement가 7번째 implicit row나 implicit column 생성 | 여섯-row 두 topology와 column 1/2를 고정하고 다른 span/column/flow 및 7개 이상 row를 거부한다 | 통과 |
| 14 | DOM source-order 의존 | 같은 semantic placement가 순서에 따라 cap/track sizing을 바꿈 | 두 topology의 forward/reverse pair 전체에서 tracks·item frames를 0.01 CSS px 안에서 비교한다 | 통과 |
| 15 | stretch의 범위 | stretch가 fit/min/max row를 성장시키거나 over-constrained grid를 줄임 | start/stretch pair에서 auto row만 남는 공간을 받는지 확인하고, overflow된 grid는 축소되지 않음을 expected total로 검사한다 | 통과 |
| 16 | text wrapping 기저 변경 | font/payload/wrapping 변화가 automatic minimum 결과에 섞임 | 기존 ASCII text·10px monospace·10px line-height와 line count를 고정한다 | 통과 |
| 17 | used-row 관측 오인 | CSS computed string 또는 span frame 하나만으로 모든 row를 유추 | CSSOM seven-row serialization, typed track list, frame와 직접 row-sum을 함께 검사하고 독립 verifier가 원본 NDJSON을 재검증한다 | 통과 |
| 18 | remainder/rounding | 세 fit cap 때문에 1/64px 분배 규칙을 1/2-fit에서 무근거 재사용 | 실제 Chrome used rows와 Layout trace를 대조하며 필요 시 검증된 triple profile만 제한된 rounding rule을 추가한다 | 통과 |
| 19 | 실패 후 세션 오염 | malformed/stale trace가 partial output을 게시하거나 다음 valid scene을 깨뜨림 | owner/revision/trace negative, atomic rejection과 동일 RuntimeSession valid recovery를 구현 gate로 둔다 | 통과 |
| 20 | 검증·정리 범위 과장 | Chrome 관측을 제품 구현으로 보거나 simulator/사용자 프로필을 삭제 | Chrome oracle와 RuntimeSession·Android/iOS evidence를 분리하고, 재사용 profile 보존·task-owned 앱/세션만 종료·공식 78/214 유지 조건을 명시한다 | 통과 |

**결과:** 20개의 서로 다른 실패 관점에서 계획 누락은 발견되지 않았다. 구현 전 남는 실질 불확실성은 세 fit track의 track-sizing stage와 1/64px remainder 동작이므로 RuntimeSession의 typed order/cap trace, row-size pair와 atomic recovery를 별도 완료 관문으로 둔다. 이 사전 비교만으로 제품 지원을 주장하지 않는다.

# C11.4 later fit-content row-span 계획 · 실패 관점 검토 20개

검토 대상: [계획](./c11-4-root-html-repeated-text-later-fit-content-row-span.md), [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-later-fit-content-row-span-precomparison-2026-11-09/README.md), 전체 [1,920-scene observation](../spec/internal/evidence/c11-4-root-html-repeated-text-later-fit-content-row-span-precomparison-2026-11-09/chrome157-matrix-observation.ndjson), 그리고 독립 관계 검증기. 계획 검토는 구현 검토를 대체하지 않는다.

| # | 공격 관점 | 깨질 수 있는 계약 | 확인한 방어·근거 | 판정 |
|---:|---|---|---|---|
| 1 | CSS position과 Rust index 혼동 | position 2를 세 번째 track에 삽입 | fixture는 1-based `fitPosition`과 `splice(position - 1)`를 저장하고 verifier가 authored list를 재구성 | 통과 |
| 2 | track 교체와 삽입 혼동 | fit-content 삽입 시 keyword 하나가 사라짐 | 각 scene에서 six-member list 길이 6 및 다섯 keyword multiset을 exact 검사 | 통과 |
| 3 | keyword multiplicity 누수 | min/max/auto 개수가 바뀐 invalid list가 긍정 대조에 들어감 | 30개 tail order마다 `min-content×2`, `max-content×2`, `auto×1`을 검증 | 통과 |
| 4 | 중복 permutation | 순서가 같은 tail을 여러 scene으로 세어 누락을 숨김 | five-member multiset의 unique order 30개와 full scene key uniqueness 확인 | 통과 |
| 5 | 위치별 matrix 누락 | 두 번째 또는 세 번째 위치 전체가 실행되지 않음 | 각 fit position별 960개, 합계 1,920개 및 모든 cap/alignment/topology 곱집합 확인 | 통과 |
| 6 | percentage basis 오용 | 20% 또는 120%를 viewport나 intrinsic row에 대한 값으로 계산 | definite 220px content block size를 고정하고 44/0/264px cap 기대를 문서화 | 통과 |
| 7 | 120% clamp | authored 120%를 100%로 제한 | fixed input에서 `fit-content(120%)`를 별도 cap으로 보존; 계획은 clamp하지 않음 | 통과 |
| 8 | zero cap floor | 0%를 min-content floor까지 자르는 상한으로 오해 | used size가 min-content floor를 넘을 수 있음을 명시하고 `size <= cap` 판정을 금지 | 통과 |
| 9 | explicit row의 cycle 포함 | 10px explicit row 뒤 implicit member 위치가 한 칸 밀림 | explicit row는 row 0, list cycle은 첫 implicit row부터 시작; 4 used rows와 line range 합을 점검 | 통과 |
| 10 | position 2가 span과 만나지 않음 | 뒤쪽 fit-content가 positive matrix에 있지만 sizing에는 쓰이지 않음 | row 2/span 2, row 3/span 2, row 2/span 3이 첫 세 implicit row를 모두 덮음 | 통과 |
| 11 | position 3과 span-3 경계 | 세 번째 implicit row의 cap/floor 영향을 빠뜨림 | span-3 C item과 세 번째 row를 포함해 fit position 3 scene을 960개 관측 | 통과 |
| 12 | implicit column 발생 | 겹치는 row-span이 별도 column을 만들어 축 결과를 오염 | 모든 item의 `grid-column:1 / 2`, frame x=0/width=40 확인 | 통과 |
| 13 | source-order sizing 의존 | DOM 순서만 바꾼 장면에서 semantic frame이 달라짐 | 두 forward/reversed pair의 rows와 semantic frame 최대 차이 0 CSS px | 통과 |
| 14 | span frame과 row trace 불일치 | item height가 span track 합과 다른데 성공으로 취급 | 모든 item frame height/top을 점유 rows 합과 비교; 최대 차이 0 CSS px | 통과 |
| 15 | text wrapping 환경 drift | 브라우저 글꼴·DPR 변화가 intrinsic row 결과를 바꿈 | Chrome 157, MacIntel, DPR 1을 기록; A/B/C line count 2/3/4 exact | 통과 |
| 16 | stretch 시 intrinsic track 변경 | `align-content:stretch`가 fit-content/min/max row를 늘림 | start/stretch 대조에서 `auto`가 아닌 사용 track의 크기 불변 확인 | 통과 |
| 17 | fit-content stretch 해석 | fit-content track을 `auto`처럼 늘림 | 독립 기대 관계는 auto track만 stretch; computed list type와 row deltas를 따로 보존 | 통과 |
| 18 | computed order 정규화 누락 | parser가 값 의미만 보존하고 순서를 바꿈 | Browser `gridAutoRows` serialization을 authored six-member order와 scene별 비교 | 통과 |
| 19 | 관측과 기대 혼동 | Chrome 값을 독립 CSS Grid 기대값으로 설명 | span-sum, line count, source order, stretch 관계를 별도로 검증하고 Chrome은 교차 확인으로 분리 | 통과 |
| 20 | 범위·완료 주장 누수 | 이 matrix를 general CSS, image, arbitrary row-span 또는 C11.4 완료로 확대 | 계획은 position 4–6, arbitrary content와 image를 거부 경계에 남기고 `78/214` 및 C11.4/C11 미완료를 고정 | 통과 |

**결과:** 서로 다른 20개 실패 관점을 검토했다. 1,920개 Chrome scene은 구조와 독립 관계 검사에 모두 통과했다. 구현 전 추가로 막아야 할 계획 결함은 발견되지 않았다. 구현 중 code path가 달라지면 관련 관점을 새 구현/runtime 20개 검토에서 다시 확인한다.

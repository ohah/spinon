# C11.4 five-fit row-span 계획 PR diff 검토 30개

**검토 대상:** 계획·계획 검토·Chrome 사전 비교 자료·`spec/STATUS.md`·공개 roadmap의 staged PR 변경분. 제품 구현 코드는 이 PR 범위에 없다.

아래 관점은 계획 자체의 [20개 실패 관점](../../../../plan/c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-plan-review-20.md)과 구분해 실제 계획 PR diff를 검토한다. 구현/runtime 검토는 구현 PR 단계에서 별도로 수행한다.

| # | 변경 실패 관점 | 확인 | 판정 |
|---:|---|---|---|
| 1 | 계획 PR이 C11.4/C11 checkbox를 완료 처리 | 상태 변경은 진행 기록만 추가하며 체크박스는 그대로다. | 통과 |
| 2 | 공식 완료 수가 `78/214`에서 바뀜 | 상태 본문과 계획은 `78/214`를 유지한다. | 통과 |
| 3 | 내부 숫자 계약이 조용히 올라감 | 명세 버전은 `0.1.0` 그대로라고 기록했다. | 통과 |
| 4 | 구현 전 계획을 runtime 구현으로 표시 | 계획 상태가 implementation 미착수라고 한정한다. | 통과 |
| 5 | 현재 활성 후보 카드가 오래된 미착수 단계로 남음 | `detailNext`를 Chrome 비교·계획 검토 완료 상태로 갱신한다. | 통과 |
| 6 | 마지막 병합 implementation을 후보 계획으로 덮음 | 최근 bounded implementation 헤더는 four-fit PR #1357을 계속 가리킨다. | 통과 |
| 7 | five-fit row-span이 다섯-fit auto-placement와 범위를 섞음 | 별도 owner·placement profile로 분리하고 auto-placement 코드를 교체하지 않는다. | 통과 |
| 8 | six-member invariant 없이 5-fit 문자열만 검색 | 정확히 다섯 fit와 singleton tail의 여섯-member list를 요구한다. | 통과 |
| 9 | 허용 tail 값 일부가 빠지거나 합쳐짐 | `min-content`, `max-content`, `auto`를 각각 관측하고 보존한다. | 통과 |
| 10 | 같은 cap 집합이라도 typed value가 달라짐 | 다섯 exact cap type/value를 고정하고 assignment 순서를 검사한다. | 통과 |
| 11 | fit-position quintuple 일부가 matrix에 빠짐 | 여섯 5-of-6 position 조합 전부를 verifier가 요구한다. | 통과 |
| 12 | cap authored order 일부가 누락 | 각 position/tail grouping에서 120 permutation을 모두 요구한다. | 통과 |
| 13 | `%` cap의 sizing basis가 달라짐 | Grid height 220px를 definite로 고정하고 indefinite basis를 거부한다. | 통과 |
| 14 | 미해결 `calc()`를 수치 추측으로 허용 | 기존 projection이 exact accepted type으로 정규화한 계산만 허용한다. | 통과 |
| 15 | five-item span-2 topology가 omitted 또는 count가 다름 | 5-item 가족과 각 start/span을 fixture와 verifier가 검사한다. | 통과 |
| 16 | four-item span-2/3 topology가 omitted 또는 span이 다름 | 4-item 가족의 span-2와 span-3 위치를 고정해 검증한다. | 통과 |
| 17 | DOM forward/reverse에서 semantic frame이 바뀜 | item ID별 row/frame을 pair하고 observed delta는 0 CSS px다. | 통과 |
| 18 | baseline과 candidate가 다른 CSS input을 사용 | 같은 track list·alignment·topology의 `min-height:0|auto` 17,280 pair를 확인한다. | 통과 |
| 19 | eligibility denominator가 모든 item으로 무차별 계산 | span의 fit/auto minimum track 조건과 non-scrollable 경계를 검사한다. | 통과 |
| 20 | baseline floor 위반과 candidate 결과가 뒤집힘 | verifier는 baseline 13,248, candidate 0을 각각 독립 계산했다. | 통과 |
| 21 | tail 종류별 `align-content:stretch` 차이를 합침 | auto tail free-space 변화와 min/max tail 불변을 별도로 검사한다. | 통과 |
| 22 | seven used rows에서 explicit row를 생략 | verifier는 explicit 1개와 implicit 6개의 serialization을 검사한다. | 통과 |
| 23 | geometry tolerance가 오차를 과도하게 숨김 | 0.01 CSS px tolerance를 쓰고 관측된 최대 span 합 오차 0.00025를 기록한다. | 통과 |
| 24 | screenshot이 다른 profile이나 baseline을 보여줌 | 대표 장면은 고정 sample selector가 고른 five-fit/auto-minimum scene이다. | 통과 |
| 25 | fixture 또는 collector가 evidence manifest와 연결되지 않음 | fixture SHA와 collector SHA를 capture manifest에 기록했다. | 통과 |
| 26 | observation 수가 generator나 paired denominator와 다름 | gzip 해제 후 34,560 NDJSON records, 17,280 paired keys, 2,160 track lists를 확인했다. | 통과 |
| 27 | 독립 verifier에 이전 four-fit 가정이 남음 | 첫 실행에서 stale four-cap assertion이 드러나 수정했고, 최종 verifier와 matrix는 통과했다. | 수정 후 통과 |
| 28 | raw observation이 바뀌거나 잘릴 때 발견하지 못함 | SHA-256 manifest의 9개 산출물 검증이 모두 통과했다. | 통과 |
| 29 | 전용 Agent Browser 또는 simulator가 계획 PR 준비 뒤 남음 | Agent Browser collector는 close를 기록했고 session list는 비었다. iOS Simulator는 모두 shutdown이다. | 통과 |
| 30 | PR이 범위 밖 파일을 끌어오거나 platform result를 과장 | staged 경로는 plan·fixture·evidence·status·roadmap에 한정한다. 앱 runtime/simulator 결과나 일반 CSS 지원을 주장하지 않는다. | 통과 |

## 판정

최종 staged diff에서 계획 경계, official status, 관측 파일 정합성에 blocking discrepancy를 찾지 못했다. 검증기 첫 실행에서 남아 있던 four-fit cap 개수 조건은 수정한 뒤 전체 matrix와 checksum 검증을 다시 통과했다. Merge 전에는 원격 PR의 changed-file set과 patch를 local branch와 다시 대조한다. 이 계획 PR은 제품 구현을 병합하지 않는다.

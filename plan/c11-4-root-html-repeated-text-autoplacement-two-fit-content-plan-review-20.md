# C11.4 반복 텍스트 자동 배치 두-fit 계획 · 실패 관점 20개

검토 대상은 [계획](./c11-4-root-html-repeated-text-autoplacement-two-fit-content.md)과 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-two-fit-content-precomparison-2026-11-10/README.md)다. 구현/runtime 검토와 PR 전체 diff 검토는 별도 관문으로 남겼다.

| # | 실패 관점 | 누락될 수 있는 결함 | 계획의 차단 방법 | 판정 |
|---:|---|---|---|---|
| 1 | two-fit auto와 row-span admission 혼합 | explicit span topology가 auto profile에 새어 들어감 | 기존 two-fit span predicate·topology를 그대로 두고 auto profile별 입력과 회귀를 분리 | 통과 |
| 2 | fit 위치 pair coverage | 15개 pair 중 조합이 중복되거나 빠짐 | 1–6의 모든 오름차순 pair를 고정하고 2,880 matrix와 15쌍 확인 | 통과 |
| 3 | cap assignment 방향 | 두 cap이 서로 교환되거나 한 위치에 붙음 | 여덟 ordered cap assignment와 typed projection 순서를 별도로 비교 | 통과 |
| 4 | 같은 numeric cap의 타입 보존 | `0px`와 `0%`가 같은 geometry라 타입 유실을 감춤 | CSSOM과 typed projection에서 authored 값/위치 보존을 확인 | 통과 |
| 5 | tail permutation 누락 | 나머지 track의 한 순서에서만 통과 | `[min,min,max,auto]`의 독립 12개 permutation을 전부 생성 | 통과 |
| 6 | list multiplicity 변경 | `min/max/auto`가 추가되거나 빠져 다른 profile을 통과 | exact `2 fit + 2 min + 1 max + 1 auto` multiset을 매 scene에서 확인 | 통과 |
| 7 | explicit row를 implicit cycle에 포함 | implicit row 첫 cap이 position 2로 밀림 | control item의 explicit row 10px와 implicit row 첫 index를 개별 검사 | 통과 |
| 8 | 6-member cycle off-by-one | 반복 row 7·8에 잘못된 cap이 적용 | 여덟 implicit row의 track position `[1…6,1,2]` 관계를 확인 | 통과 |
| 9 | 두 fit 위치의 repeated occurrence | position 1/2 cap이 두 번째 사용 row에서 달라짐 | 위치 1·2의 두 cycle occurrence를 7,680 fit row check에 포함 | 통과 |
| 10 | auto track 반복과 stretch | 반복 등장한 auto track 한 곳에 free space 전부 배정 | 실제 auto row occurrence 수로 free space 균등 분배를 검사 | 통과 |
| 11 | non-auto track stretch | `stretch`가 fit/min/max track의 base size를 바꿈 | 1,440 start/stretch pair에서 non-auto row delta 0을 확인 | 통과 |
| 12 | percentage cap basis | percentage를 viewport나 width 기준으로 계산 | definite 220px block basis와 `20%=44px`, `120%=264px`를 계획에 고정 | 통과 |
| 13 | cap-zero 처리 | 0 cap을 음수/미지원 값처럼 거부하거나 0보다 큰 row를 생성 | `0px`, `0%` cap observations와 독립 min-contribution 관계를 포함 | 통과 |
| 14 | min-content floor 과장 | `min-height:0` 입력에서 지원하지 않는 auto minimum을 약속 | 계획이 해당 profile의 floor 확장을 명시적으로 제외 | 통과 |
| 15 | text wrapping 변동 | font, width 또는 payload 변화가 row sizing 관측을 오염 | 고정 `40px` column, payload 8개, `10px/10px monospace`와 line counts를 검사 | 통과 |
| 16 | item frame과 row trace 분리 | frame이 track list와 어긋나도 높이 총합만 통과 | item/frame-to-used-row와 payload/frame-to-lines를 각각 대조 | 통과 |
| 17 | overflow 장면 제거 | root 아래로 넘친 text가 자동 clip되거나 evidence에서 빠짐 | visible overflow를 scene별 검사하고 기존 288 overflow scene 관계를 대조 | 통과 |
| 18 | invalid basis/topology admission | indefinite percentage, implicit column, dense flow가 성공 | 고정 shape 밖의 cap basis/column/flow를 fail-closed negative matrix에 둠 | 통과 |
| 19 | trace atomicity·회복 | 잘못된 trace가 frame 일부를 발행하거나 같은 session을 오염 | owner/revision/order failure와 no-partial-output, same-session recovery를 요구 | 통과 |
| 20 | 정리·증거 범위 과장 | 공유 simulator를 삭제하거나 Chrome screenshot을 앱 증거로 표시 | 이번 run 소유 브라우저/앱만 종료, 공용 profile 보존, Chrome 사전 비교와 native smoke를 분리 | 통과 |

**결과:** 서로 다른 20개 계획 실패 관점을 모두 반영했다. 기존 two-fit span 지원과 auto-placement 미지원 경계를 분리했고, 구현은 아직 시작하지 않았다. Chrome 결과는 사전 비교만 증명한다.

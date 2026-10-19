# C11.4 auto-repeat explicit neighbor 계획 검토 · 20개 실패 관점

**검토 대상:** `plan/c11-4-grid-track-sizing.md`의 2026-10-20 auto-repeat explicit-neighbor 후속 계획, Chrome fixture와 WPT source 경계. 제품 계산 코드 수정 전에 검토했다. 각 관점은 별도 실패 경로를 확인하며 이번 표를 구현 또는 PR diff 검토에 재사용하지 않는다.

초기 초안에서 반복 위치가 중간에 있는 경우만 다뤘다. 검토 중 track list 시작·끝 위치, 2-track repeat pattern, definite max/min size, span으로 점유되는 auto-fit track이 빠진 것을 찾아 계획과 13-case fixture에 추가했다.

| # | 실패 관점 | 계획 검토와 판정 기준 | 결과 |
| ---: | --- | --- | --- |
| 1 | auto-repeat이 track list 첫 위치에 오는 경우 | suffix-only case로 repeat insertion index 0의 component 순서와 suffix placement를 고정한다. | PASS |
| 2 | auto-repeat이 track list 마지막 위치에 오는 경우 | prefix-only case로 prefix 다음에 반복 track이 생성되는지 고정한다. | PASS |
| 3 | prefix와 suffix가 모두 있는 중간 위치 | both-neighbor case에서 prefix, expanded repeat, suffix의 순서를 비교한다. | PASS |
| 4 | prefix 사용량을 repeat count에서 빠뜨림 | 20px prefix의 존재에 따른 반복 수를 Chrome oracle·규범 계산과 비교한다. | PASS |
| 5 | suffix 사용량을 repeat count에서 빠뜨림 | 10px suffix 때문에 사용 가능한 반복 수가 달라지는 경우를 분리해 기록한다. | PASS |
| 6 | 전체 track 수와 gap 수의 off-by-one | track `N`개일 때 `N-1` gap을 기준으로 46px 사례의 합계가 200px인지 확인한다. | PASS |
| 7 | auto-repeat fragment 자체가 2개 track인 경우 | `[minmax(30px,1fr),20px]` pattern의 repetition 수와 component 순서를 별도로 고정한다. | PASS |
| 8 | 한 번 반복해도 available size를 초과하는 경우 | 40px width control에서 CSS Grid의 최소 1회 반복과 외부 overflow를 유지한다. | PASS |
| 9 | auto-fill 빈 repeated track을 누락 | empty auto-fill track은 보존하며 collapse하지 않는다고 명시한다. | PASS |
| 10 | auto-fit collapse를 placement보다 먼저 실행 | 반복 item 배치 뒤 empty 여부를 판단하는 순서와 fixture를 고정한다. | PASS |
| 11 | 두 repeated track을 span한 item을 빈 track으로 오판 | span 내부 repeated track 둘은 점유로 남고 나머지만 collapse하는 case를 추가했다. | PASS |
| 12 | 모든 repeated track이 비어 있을 때 | all-empty auto-fit으로 0px repeated track과 suffix 경계를 고정한다. | PASS |
| 13 | collapsed track 주변 gutter 처리 | computed track list뿐 아니라 suffix item 좌표를 기록해 gutter-collapse 위치도 비교한다. | PASS |
| 14 | column repeat count에 height를 사용 | columns case는 definite inline size와 column gap을 독립적으로 고정한다. | PASS |
| 15 | row repeat count에 width를 사용 | rows case는 definite block size와 row gap을 독립적으로 고정한다. | PASS |
| 16 | preferred/max/min/indefinite basis 혼동 | preferred 200px, max-width 200px, min-width 200px, max-content indefinite를 각각 별도 case로 둔다. | PASS |
| 17 | fractional 1fr serialization을 exact integer로 강제 | suffix-only 55.3281px을 ideal 55.333…px와 1/64 CSS px 기준으로 대조한다. | PASS |
| 18 | neighbor의 비-px·음수·비유한 값을 조용히 수용하거나 버림 | 유한·0 이상 fixed px만 positive scope에 두고 나머지는 unsupported 입력으로 fail-closed한다. | PASS |
| 19 | 이름/areas를 잃고 성공처럼 보이는 projection | surrounding/repeated line names와 template areas를 명시적으로 지원 밖에 두고 오류를 유지한다. | PASS |
| 20 | 중복 repeat·양 축 repeat·한도 또는 실패 원자성 회귀 | list당 한 repeat·한 축 제한, 기존 track cap, 계산/예약 실패 시 partial output 부재를 implementation gate에 남긴다. | PASS |

계획 결과는 구현 승인이나 CSS Grid 전체 지원 판정이 아니다. 구현 뒤에는 실제 code/runtime 실패 관점 20개를 새로 수행하고, PR diff도 이 표와 분리해 별도 검토한다.

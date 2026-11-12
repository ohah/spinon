# C11.4 implementation PR diff 적대적 검토 30개

실제 구현과 테스트 변경을 실패 경로별로 검토했다. 계획 PR의 30 관점 검토와 계획 검토 20개를 재사용하지 않았다. 발견 사항은 assertion/test fixture 수정으로 반영한 뒤 영향받는 실행을 재확인했다.

| # | 검토 관점 | 결과 |
|---:|---|---|
| 1 | six-fit predicate가 정확히 여섯 track만 받는가 | 확인: list length가 6이어야 한다. |
| 2 | cap 집합이 0px, 10px, 20%, 30px, 40px, 50px와 정확히 같은가 | 확인: typed enum 값별 index를 구분하고 전체 seen set을 요구한다. |
| 3 | 20%가 길이 20px로 오인되지 않는가 | 확인: percentage와 LengthPx가 서로 다른 match arm이다. |
| 4 | duplicate cap이 허용되는가 | 거부: 이미 seen인 cap index를 실패 처리한다. |
| 5 | authored order를 고정해 정상 순열을 놓치는가 | 거부: multiset 기반이므로 720 순열을 모두 받으며 전부 실행했다. |
| 6 | style projection이 unrelated six-fit auto-placement predicate를 넓히는가 | 확인: row-span helper는 분리된 별도 predicate이고 기존 auto-placement helper를 교체하지 않는다. |
| 7 | layout과 projection이 서로 다른 cap 집합을 허용하는가 | 확인: 두 계층이 동일 six-fit span helper를 사용한다. |
| 8 | cap 목록만 맞고 repeated-text span profile은 아닌 scene이 허용되는가 | 거부: mixed mask predicate는 기존 repeated_text_span_profile도 요구한다. |
| 9 | unsupported writing context가 들어오는가 | 거부: 기존 grid/item writing-context profile 검사도 유지된다. |
| 10 | visible Grid owner가 여러 개인가 | 거부: projection은 정확히 하나의 visible owner를 요구한다. |
| 11 | 숨김 item이 visible item 수를 우회하는가 | 확인: projection/layout의 기존 visible child 및 ownership 검사를 유지한다. |
| 12 | explicit row/control item이 자동 placement로 바뀌는가 | 거부: row-span control은 row 1 / 2, column 1 / 2 placement를 요구한다. |
| 13 | row-span child가 임의 row나 span을 요청하는가 | 거부: 기존 bounded placement topology 목록만 허용한다. |
| 14 | control item이 mixed profile에 auto minimum을 받는가 | 거부: control min-height는 0px여야 하며 auto control negative test를 추가했다. |
| 15 | 다른 item에 1px/28px 같은 미지정 minimum이 섞이는가 | 거부: six-fit mixed gate는 span items를 Auto 또는 Fixed(0)로만 허용한다. |
| 16 | uniform all-auto 기존 slice가 계속 유효한가 | 확인: 기존 uniform candidate path와 11,520-scene regression을 보존했다. |
| 17 | uniform all-zero 기존 slice가 mixed profile에서 처리되는가 | 확인: zero mask control은 기존 geometry와 함께 통과했다. |
| 18 | allow_auto_minimum이 unrelated profile에 켜지는가 | 확인: layout/projection의 기존 gate에 새 bounded span candidate만 추가했다. |
| 19 | item max-height나 other dimensions가 새로 넓어지는가 | 확인: max/width/overflow/spacing/position profile은 그대로 고정이다. |
| 20 | item에 여러 payload child가 들어오는가 | 거부: row item은 정확히 하나의 text payload를 요구한다. |
| 21 | text payload의 font, flow, white-space 조건이 바뀌는가 | 확인: 기존 repeated_text payload validator를 그대로 사용한다. |
| 22 | nested grid, scroll overflow 또는 positioned child가 통과하는가 | 거부: 기존 no-nested-grid, visible-overflow, static-position 조건을 유지한다. |
| 23 | grid-auto-placement 범위가 span row profile로 확대되는가 | 거부: span placement는 explicit row/column position을 요구하고 auto-placement 별도 지원은 변경하지 않았다. |
| 24 | style/layout에 알려진 6 cap 순열이 빠지는가 | 확인: projection/layout helper tests는 720 accepted permutations를 모두 확인한다. |
| 25 | mixed matrix가 Chrome record metadata를 잘못 해석하는가 | 확인: scene adapter가 ID로 mask를 다시 계산하고 모든 record 값과 대조한다. |
| 26 | mixed test가 Chrome row를 runtime trace라고 표시하는가 | 수정: SceneResult.rows 설명과 trace assertion을 분리했고 runtime item/payload frames가 geometry oracle다. |
| 27 | legacy test의 computed min-height 기대값이 글로벌 stylesheet와 충돌하는가 | 수정: autoItems record에서는 item별, legacy scene에서는 scene minimum으로 기대값을 계산했다. 관련 3-fit 11,520 matrix 재실행 통과. |
| 28 | 4-fit rejection이 실은 지원되는 다섯-fit cap set인가 | 수정: 유효 40px 대신 unsupported 41px cap을 사용하고 rejection 재실행 통과. |
| 29 | fixture-test 증거가 제품 지원 또는 C11.4 전체 완료로 홍보되는가 | 확인: README/PR 범위를 bounded RuntimeSession slice로 제한하고 공식 78/214와 C11.4/C11 상태를 유지한다. |
| 30 | platform smoke가 mixed grid pixel parity를 증명한다고 오인되는가 | 확인: Android/iOS 이미지는 기본 Root HTML boot smoke로만 표시하고 C11.4 scene/pixel 검증과 분리한다. |

## 검토 중 수정한 확인 문제

- Mixed trace의 per-track raw allocation은 Blink 최종 row placement와 다를 수 있음을 확인했다. 테스트는 raw trace를 Chrome row array로 오인하지 않으며 각 item/payload frame과 covered-row 결과를 비교한다.
- Legacy 3-fit test helper는 CSS가 globally auto를 계산하는 구간에서도 이전 scene의 0px를 기대했다. item-specific autoItems와 scene-wide legacy expectation을 나눈 뒤 11,520-scene regression을 통과했다.
- 4-fit rejection mutation의 40px cap은 이미 지원되는 five-fit set을 만들었다. 잘못된 rejection 입력을 41px로 바꾸고 해당 test를 통과시켰다.
- Clippy strict run의 여섯 warning은 이번 diff에 없는 기존 helper에 있다. 원본 failure와 기존 lint 두 종류를 제외한 candidate pass를 각각 보존했고 warning source는 변경하지 않았다.


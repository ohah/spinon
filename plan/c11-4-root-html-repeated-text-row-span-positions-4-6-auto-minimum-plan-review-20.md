# C11.4 positions 4–6 automatic minimum · 계획 실패 관점 검토 20개

계획과 Chrome 157 사전 비교를 2026-11-10에 구현 전 검토했다. 각 항목은 서로 다른 failure surface와 acceptance condition을 본다. 제품 코드는 아직 변경하지 않았다.

| # | 실패 관점 | 검토 |
| ---: | --- | --- |
| 1 | 이미 완료된 single-fit positions 4–6 또는 two-fit row-span과 중복되는 범위 | PASS — 기준 구현 #1306의 동일 positions에 automatic minimum만 추가하며, two-fit row-span은 별도 profile로 남긴다. |
| 2 | 목록 순서와 fit 위치를 1-index/0-index 혼동 | PASS — authored positions 4·5·6을 fixture builder와 관측에서 분리해 기록하고 track list insertion index는 `position - 1`이다. |
| 3 | six-row topology가 실제로 여섯 implicit row를 만들지 않음 | PASS — explicit row 1개와 six implicit row를 합친 used rows 7개를 각 observation과 독립 검증기가 확인한다. |
| 4 | span2 family에서 끝 row가 여섯 번째 implicit row가 되지 않음 | PASS — E item은 row 6 / span 2이며 마지막 used track을 점유한다. |
| 5 | mixed span2/span3 family의 서로 다른 점유 범위 누락 | PASS — A/B는 span 2, C/D는 span 3이고 forward/reverse 배열을 각각 고정한다. |
| 6 | tail list에서 중복 순열을 서로 다른 장면으로 셈 | PASS — `min-content ×2`, `max-content ×2`, `auto ×1` multiset의 30개 고유 순서를 사용하고 독립 검증기가 uniqueness를 확인한다. |
| 7 | `0px`와 `0%`를 같은 typed cap으로 합침 | PASS — 두 authored 값 모두 장면과 computed `grid-auto-rows`에서 문자열 유형 그대로 보존한다. |
| 8 | percentage cap을 indefinite axis에 적용 | PASS — Grid block size는 definite 220px이며 percentage cap의 기준 축을 고정한다. |
| 9 | fit-content cap이 content-based automatic minimum을 낮춘다고 가정 | PASS — CSS Grid §6.6·§7.2를 독립 기준으로 두고 cap 초과 content floor를 판정한다. |
| 10 | overflow scrollability가 달라 automatic minimum eligibility가 바뀜 | PASS — 모든 scene에서 item overflow는 `visible`로 고정하고 범위를 scroll/clipping 지원으로 확대하지 않는다. |
| 11 | 해당 span에 auto minimum track이 없는 item을 eligible로 셈 | PASS — 독립 verifier는 item이 span한 실제 implicit track의 `auto`/`fit-content()` minimum을 하나씩 조회하고 explicit row는 제외한다. |
| 12 | explicit empty control까지 자동 최소 크기 후보로 변함 | PASS — 첫 row control은 두 cohort 모두 computed `min-height:0`이며 고정 40×10 frame을 유지한다. |
| 13 | baseline/candidate paired key에서 minimum mode가 빠지거나 다른 축을 바꿈 | PASS — 동일 authored key로 4,320쌍을 만들고 fixture, 수집기, 독립 검증기가 track/topology/alignment 일치를 확인한다. |
| 14 | pair 순서나 source order가 track sizing을 바꾸는 결함을 숨김 | PASS — 두 topology family 각각 forward/reverse를 포함하고 paired row/frame 차이 허용치를 0.01px로 고정한다. |
| 15 | `align-content:stretch`가 non-auto track을 바꾸거나 auto track을 줄임 | PASS — 독립 verifier는 auto track만 non-decreasing이어야 하며 나머지 track은 start와 같음을 확인한다. |
| 16 | item frame을 점유 track 합이 아닌 총 grid height와 비교 | PASS — 각 item y/height를 실제 시작 row와 span의 used row 합으로 계산한다. |
| 17 | `min-height:auto`가 text wrapping이나 font input을 바꿔 sizing 변화와 섞임 | PASS — 모든 paired scene에서 line count를 비교하고 고정 monospace/direct-text 입력을 사용한다. |
| 18 | 혼합 item minimum이나 control mutation을 허용 범위로 오인 | PASS — plan acceptance에 mixed `auto`/`0`, empty control 변경과 unsupported payload를 fail-closed rejection으로 적었다. |
| 19 | 거부 뒤 같은 RuntimeSession의 복구가 sizing state를 오염 | PASS — implementation gate에 atomic rejection 후 유효 revision 재실행을 별도 요구한다. |
| 20 | Chrome precomparison을 Spinon, mobile 또는 전체 CSS Grid 적합성으로 확대 | PASS — 범위를 browser oracle로 제한하고 implementation/runtime, simulator, public CSS admission과 C11.4 completion을 별도 gate로 둔다. |

최종 계획 판정: blockers 없음. 구현 전 Chrome 비교 8,640/8,640과 별도 독립 verifier가 고정 입력, pair coverage, content floor/overflow, source order, span-frame 및 alignment 관계를 확인했다. 구현은 이 계획의 명시적 acceptance gate를 만족한 뒤 별도 검토로 진행한다.

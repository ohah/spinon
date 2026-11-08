# C11.4 Root HTML 반복 intrinsic row text span 계획 · 실패 관점 20개

검토 대상은 [계획](./c11-4-root-html-repeated-text-row-span.md)과 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-precomparison-2026-11-09/README.md)다. 각 항목은 서로 다른 입력·알고리즘·publish·검증 실패를 겨냥한다. 처음 실행한 browser fixture에서는 row만 고정하고 column을 auto로 두자 implicit column이 생기는 결함이 확인됐다. fixture의 모든 item에 `grid-column:1 / 2`를 추가하고 960 scene matrix를 재생성한 뒤 아래 검토를 수행했다.

| # | 공격 관점 | 확인할 실패 | 계획의 방어·근거 | 판정 |
|---:|---|---|---|---|
| 1 | 열 auto-placement 충돌 | 겹친 row span이 다른 열로 밀려 열 축 결과를 섞음 | 모든 item에 column `1 / 2`; Chrome 모든 frame x=0, width=40 | 통과 |
| 2 | CSS line number의 1-based 오프셋 | span 검증이 explicit row와 첫 implicit row를 한 칸 잘못 셈 | 기대 row를 CSS line과 used row index 양쪽에서 고정하고 explicit row를 분리 | 통과 |
| 3 | auto row 반복 시작 위치 | 첫 implicit row가 explicit row 뒤가 아닌 track-list 시작으로 계산됨 | explicit row 10px와 첫 implicit fit-content row를 별도 비교 | 통과 |
| 4 | span-2 겹침 순서 | A/B가 row 3을 공유하지 않아 span contribution batch를 안 만듦 | `A:2/span2`, `B:3/span2`; 실제 Chrome item 영역 교차 확인 | 통과 |
| 5 | span-3 batch 순서 | span-3이 span-2와 겹치지 않거나 source order가 sizing 순서를 바꿈 | 두 span-2 뒤에 `C:2/span3`; 역순 control 포함 | 통과 |
| 6 | DOM order 의존성 | 같은 semantic placement의 item 순서 변경이 frame을 바꿈 | 4 pair의 semantic item frame 최대 차이 0px | 통과 |
| 7 | repeated list 순열 누락 | 30 tail order 중 중복 또는 빠진 조합 | unique permutation count=30, total 960 unique key 검증 | 통과 |
| 8 | cap 단위 오류 | `%`가 px처럼 처리되거나 definite basis가 없음 | 220px definite block-size, 20px/20%/0%/120% 모두 포함 | 통과 |
| 9 | fit-content zero cap | zero cap이 minimum contribution을 잘못 삭제하거나 invalid로 거부 | 0% scene를 독립 matrix input으로 유지; runtime에서 min floor와 cap을 별도로 검사 | 통과 |
| 10 | 과대 percentage cap | 120%가 100%로 clamp되거나 overflow로 오인 | 120% scene와 item/track geometry를 각각 저장 | 통과 |
| 11 | `align-content:stretch` 순서 | sizing 때 stretch가 적용되거나 모든 intrinsic track을 늘림 | spec상 sizing 후 alignment; verifier가 auto track만 바뀌는지 비교 | 통과 |
| 12 | line wrapping drift | browser text가 예상 줄 수와 다르거나 DPR이 바뀜 | 40px width, monospace 10/10, DPR 1; A/B/C line count 2/3/4 exact | 통과 |
| 13 | text grammar 확장 누출 | 이 slice가 일반 Unicode/font/shaping 지원으로 오인됨 | #1292의 1–32 byte ASCII 대문자 문법과 고정 style만 재사용 | 통과 |
| 14 | wrapper topology 변형 | sibling/descendant text ownership이 다른 item으로 연결됨 | 각 item당 wrapper 1개, payload 1개, 고정 DOM child 수 | 통과 |
| 15 | span 길이·line 경계 | span sum이 인접 item 간 y차로 계산됨 | final track trace와 resolved line range 요구; sibling delta 추정 금지 | 통과 |
| 16 | track trace 유실·불일치 | trace 없는 layout을 성공 처리하거나 부분 frame을 publish | trace/span mapping 누락·모순은 원자 실패, output revision·recovery 검증 | 통과 |
| 17 | implicit track 폭발 | 임의 큰 span이 수천 개 implicit row를 생성 | span 2/3와 네 used rows만 허용, 기존 resource bound 유지 | 통과 |
| 18 | unsupported placement 별칭 | shorthand/negative line/auto placement가 허용 범위를 우회 | resolved numeric lines를 exact match; 기타 변형은 diagnostic 거부 | 통과 |
| 19 | stale owner/revision | text/style/placement가 서로 다른 snapshot에서 합쳐짐 | 동일 owner/style/layout revision 검사 후 layout; stale negative와 recovery 포함 | 통과 |
| 20 | 검증 주장 과대화 | Chrome geometry를 Spinon/native pixel 또는 C11.4 완료로 기록 | Chrome·Rust·Android·iOS·surface readback을 구분하고 공식 78/214 유지 | 통과 |

**결과:** 20개 관점을 검토했고 최종 계획 및 사전 비교에서 미해결 결함은 없다. #1의 fixture 결함은 측정 초기 단계에서 발견해 최종 기준 입력과 960 scene 전체를 다시 생성했다. 구현 중 새 결함이 발견되면 영향받은 관점을 다시 확인한다.

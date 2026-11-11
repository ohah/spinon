# C11.4 four-fit row-span 계획 PR diff 검토 30개

**검토 대상:** `codex/c11-4-row-span-four-fit-plan-20261111` 계획 PR diff · plan/evidence/official roadmap 변경

실제 계획 PR 변경분을 아래 30개 서로 다른 관점으로 점검했다. 이는 [계획 실패 관점 20개](../../../../plan/c11-4-root-html-repeated-text-row-span-four-fit-auto-minimum-plan-review-20.md)와 다르며, 구현/runtime 검토를 대신하지 않는다.

| # | 변경 실패 관점 | 확인 | 판정 |
|---:|---|---|---|
| 1 | 계획 추가가 C11.4 checkbox를 완료로 바꿈 | `spec/STATUS.md`의 C11.4와 C11은 계속 `[ ]`다. | 통과 |
| 2 | 구현 전 계획을 implementation progress로 기록 | 새 상태 줄은 “구현은 시작하지 않았다”고 명시한다. | 통과 |
| 3 | official completion counter를 올림 | `78/214` 문자열과 기존 체크 상태를 유지한다. | 통과 |
| 4 | public roadmap이 계획을 진행 중으로 오표기 | `detailNext`에 `계획 PR`로만 넣고 active/inProgress는 비운다. | 통과 |
| 5 | 최근 merged slice 포인터를 미완료 후보로 덮음 | latest merged heading은 #1354 six-fit으로 유지한다. | 통과 |
| 6 | 새 slice가 auto-placement 여섯-fit을 다시 구현 | plan은 explicit row-span과 기존 row-span gap만 다룬다. | 통과 |
| 7 | 여섯-member 조건 없이 track list 범위 확대 | exact six implicit members와 four-fit multiset을 요구한다. | 통과 |
| 8 | `min-content` tail이 빠지거나 다른 intrinsic tail로 교체 | 허용 multiset에 `min-content` 한 개만 기록한다. | 통과 |
| 9 | `auto` tail이 빠져 stretch/eligibility 조건이 바뀜 | `auto` 한 개를 고정하고 Chrome matrix에서 실제 stretch를 측정했다. | 통과 |
| 10 | cap assignment 일부가 matrix에서 누락 | 24 cap 순열과 verifier group별 전체 coverage를 확인했다. | 통과 |
| 11 | fit-position coverage가 일부 track index를 빠뜨림 | 6개 위치 중 4개 조합 15개를 각각 검증한다. | 통과 |
| 12 | percentage cap의 basis가 임의 환경값 | definite 220px Grid block basis를 고정한다. | 통과 |
| 13 | resolved/unresolved `calc()` admission을 추측으로 확정 | 기존 typed projection 정규화 결과에 따른 허용/거부를 계획에 구분한다. | 통과 |
| 14 | track order를 정규화해 authored 위치를 잃음 | CSSOM track list와 fit index/cap mapping을 독립 비교한다. | 통과 |
| 15 | test topology가 여섯 implicit row를 실제로 채우지 못함 | span2×5와 span2/span3×4 가족의 used-row 합을 검증한다. | 통과 |
| 16 | DOM reverse를 다른 placement로 오해 | topology별 forward/reverse semantic item frame을 별도 pair한다. | 통과 |
| 17 | total scene 수가 곱셈과 실제 generator에서 불일치 | 15×24×2×2×4×2 = 11,520을 관측·독립 검산했다. | 통과 |
| 18 | baseline/candidate key가 서로 다른 scene | track list, cap, topology, source order, alignment를 고정한 5,760 pair를 만든다. | 통과 |
| 19 | 기준선과 candidate floor 결과를 뒤바꿈 | baseline 3,648 floor violation과 candidate 0을 verifier가 각각 계산한다. | 통과 |
| 20 | automatic-minimum eligible denominator를 모든 item으로 계산 | fit/auto minimum track을 실제 span한 item만 25,920 occurrence로 센다. | 통과 |
| 21 | fit cap을 content-based floor 상한으로 취급 | 계획에 fit-content cap과 automatic minimum floor를 구분한다. | 통과 |
| 22 | `stretch`가 auto가 아닌 track을 바꾸어 결과를 가림 | Chrome matrix의 non-auto 최대 변화 0px, auto 최대 130px를 독립 확인했다. | 통과 |
| 23 | float 오차를 숨기기 위해 평균 오차만 보고 | 각 span frame/row sum 허용치는 0.01px이며 실제 최대 오차를 기록한다. | 통과 |
| 24 | scrollable item에 non-scrollable automatic minimum을 적용 | scrollable overflow는 거부 matrix에 포함한다. | 통과 |
| 25 | mixed minimum 또는 empty control의 `auto`가 허용됨 | all-zero/all-auto profile과 control zero를 분리해 검사한다. | 통과 |
| 26 | invalid layout이 부분 publish되고 session이 오염 | atomic rejection 후 same-session recovery를 별도 관문으로 둔다. | 통과 |
| 27 | 관련 기존 profile 회귀 누락 | one-/two-/three-fit row-span과 1–6 auto-placement 및 track-list 회귀를 명시한다. | 통과 |
| 28 | simulator smoke를 feature geometry 또는 pixel parity로 확대 | 기본 route smoke는 시작 회귀만 확인한다고 제한한다. | 통과 |
| 29 | evidence가 HTML/실행기/결과/검증기 중 하나에만 의존 | fixture, capture, NDJSON, independent verifier, screenshot, lifecycle와 SHA manifest를 연결한다. | 통과 |
| 30 | Agent Browser가 전용 session을 남기거나 무관 HWP session을 종료 | 전용 session close와 목록/process 사후 확인을 기록하고 HWP session은 보존한다. | 통과 |

## 판정

계획 PR 변경분에서 계획 범위·공식 상태·증거 경계 불일치를 찾지 못했다. Merge 전에 PR remote head diff가 이 검토 대상과 같은지 SHA-256으로 대조하고, GitHub PR 상태·check rollup을 별도 확인한다. 계획 PR merge는 제품 구현 완료를 뜻하지 않는다.

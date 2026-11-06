# C11.4 two implicit auto-row stretch 계획 PR diff 실패 관점 검토 30개

검토 범위는 전용 계획, parent C11.4 계획에 추가된 링크, 사전 비교 fixture/expected JSON/Chrome screenshot, 계획 20점 review다. 각 검사는 별개의 오해·오염·경계 실패를 다룬다.

| # | PR diff에서 확인한 실패 경로 | 결과 |
|---:|---|---|
| 1 | CSS 규범과 oracle observation을 같은 권위로 취급 | PASS — 규범 산술을 기대값으로 두고 Chrome은 교차 확인 |
| 2 | 이전 2025-03-26 CRD 아닌 최신 mutable spec 참조 | PASS — 날짜 고정 URL 사용 |
| 3 | §11.8의 적용 조건 `normal|stretch` 누락 | PASS — 계획에 `normal`/`stretch` 규칙과 profile `normal` 고정 명시 |
| 4 | free space 10px 계산에서 explicit row 누락 | PASS — `40−10−10−10=10` 산술 기록 |
| 5 | auto rows 2개가 아니라 explicit rows 3개인 입력 | PASS — explicit row 1개와 implicit 2개 분리 |
| 6 | two text items가 같은 row에 놓여 auto row count가 1이 됨 | PASS — row-flow order와 exact item/frame sequence 확인 |
| 7 | positive fixture item count가 2로 남음 | PASS — fixed + `A` + `B`, 정확히 3개 |
| 8 | fourth child가 silently accepted | PASS — 거부 입력으로 명시 |
| 9 | `A`/`B` 외 source를 broad grammar로 허용 | PASS — exact token source scope 명시 |
| 10 | line-height와 minimum contribution이 다름 | PASS — font 10px/10px, 한 줄씩, 각 10px로 고정 |
| 11 | positive free space가 indefinite/negative가 됨 | PASS — definite 40px content height와 양수 free space 고정 |
| 12 | row gap 또는 item spacing이 free-space equation을 바꿈 | PASS — gap 0 고정 및 기타 gap 거부 |
| 13 | auto row 값 반복이 implicit track별로 다름 | PASS — singleton `auto` pattern을 두 implicit rows에 적용 |
| 14 | `align-content:start`가 default acceptance처럼 보임 | PASS — Chrome control only, 제품 profile 밖 |
| 15 | min-content/fixed row를 auto stretch track으로 잘못 처리 | PASS — 별도 non-stretch controls |
| 16 | single-auto-row existing behavior 회귀 누락 | PASS — `[10,30]px` one-row control 유지 |
| 17 | Taffy explicit template을 늘려 implicit trace 경계를 제거 | PASS — template expansion 금지 명시 |
| 18 | track trace가 contribution, stretch step, final used size를 분리하지 않음 | PASS — trace 요구와 independent arithmetic을 구별 |
| 19 | implementation hardcodes 10px/5px instead of measuring | PASS — existing text and Grid sizing engine 재사용 요구 |
| 20 | stale NodeId/revision/source order가 frames와 섞임 | PASS — ids, order, style/layout revisions 검사 |
| 21 | reject 이후 partial layout output이 publish | PASS — atomic reject와 same-session recovery 포함 |
| 22 | previously accepted single row rejected after gate extension | PASS — existing route regression 유지 |
| 23 | default runtime fixture marker or FFI ABI added | PASS — existing opt-in boundary reuse, no new FFI/API |
| 24 | arbitrary implicit row count/size를 이번 task scope로 확대 | PASS — exactly two implicit rows only |
| 25 | browser screenshot을 app screenshot/pixel evidence로 표기 | PASS — Chrome fixture image only, explicit limitation |
| 26 | Android AVD result를 physical device claim으로 확대 | PASS — plan labels API 37 ARM64 AVD |
| 27 | iOS Simulator result를 compositor-after pixel equality로 확대 | PASS — visual smoke limitation |
| 28 | plan update toggles official implementation checkbox/count | PASS — no status checkbox/count edit |
| 29 | internal contract numeric version increments for this plan | PASS — version remains `0.1.0` |
| 30 | plan 20-review reused as PR 30-review | PASS — separate 20 and 30 review artifacts |

계획 diff에서 필요한 수정은 발견되지 않았다. Implementation/runtime 20-point review와 구현 PR diff review 30개는 구현 결과를 대상으로 새로 작성해야 한다.

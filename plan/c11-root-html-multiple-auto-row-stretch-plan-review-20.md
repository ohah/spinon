# C11.4 two implicit auto-row stretch 계획 실패 관점 검토 20개

| # | 별도 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 두 implicit row를 두 explicit row로 잘못 계산 | PASS — template row 1개, auto-generated row 2개로 고정 |
| 2 | 균등 분배 전에 남는 공간 산술 오류 | PASS — 40−10−10−10=10px, 각 auto row에 5px |
| 3 | 텍스트 line height와 min-content contribution 혼동 | PASS — 두 direct item 각각 1 line·10px로 고정 |
| 4 | free space가 음수이거나 indefinite일 때 같은 기대값 적용 | PASS — definite 40px content box와 양의 free space만 허용 |
| 5 | auto rows 사이에 gap이 들어가 분배량 변경 | PASS — row/column gap 0 고정, gap 변경 거부 |
| 6 | implicit row 반복 패턴을 한 번만 생성 | PASS — singleton `auto`가 두 implicit rows 모두에 적용 |
| 7 | source order와 row auto-placement 뒤바뀜 | PASS — fixed, A, B 순서 및 row flow 고정 |
| 8 | visible child 수가 트랙 수와 다르게 되는 경계 | PASS — 정확히 세 visible direct items만 허용 |
| 9 | item 하나가 두 implicit rows를 span해 분배를 왜곡 | PASS — 모든 placement override/span 거부 |
| 10 | `align-content:start` control을 제품 positive로 승격 | PASS — 비교 control이며 profile 밖으로 명시 |
| 11 | `min-content` 또는 fixed row를 stretchable auto로 취급 | PASS — 각각 non-stretch control로 분리 |
| 12 | text content source change가 기존 grammar보다 넓어짐 | PASS — source `A`와 `B`만 허용 |
| 13 | nested inline, whitespace, hidden item이 count에 영향 | PASS — nested/whitespace/hidden/추가 item 거부 |
| 14 | 10px contribution이나 5px allocation을 제품 코드에 상수화 | PASS — measurement와 Grid algorithm을 사용하도록 명시 |
| 15 | explicit/implicit boundary를 template expansion이 지움 | PASS — auto tracks를 implicit 상태로 유지하도록 계획 |
| 16 | trace의 aggregate 10px만 확인해 track별 분배를 놓침 | PASS — 두 track 각각 +5px 및 15px used size 검사 |
| 17 | failure 뒤 partial revision/state가 노출 | PASS — atomic reject 및 same-session recovery 포함 |
| 18 | old single-auto-row behavior regress | PASS — `[10,30]px` 기존 scene을 focused regression으로 유지 |
| 19 | simulator screenshot을 pixel parity로 과장 | PASS — visual smoke만 인정, readback은 별도 근거로 한정 |
| 20 | bounded task를 C11.4/C11 completion으로 표시 | PASS — C11.4/C11와 `78/214` 유지 명시 |

이번 계획에서 추가로 필요한 contract/API 결정은 발견하지 못했다. 구현 전에는 이 계획 PR을 먼저 병합한다.

# C11.4 auto-repeat 양축 분류 정정 PR diff 적대 검토 30개

**검토 대상:** 수정 계획과 상위 계획 링크, 양축 Rust draft repro, 이전 검토 무효화, 고정 Chrome oracle 및 SHA 목록. 이 검토는 단일 축과 양축의 서로 다른 line path를 구분해 확인했다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 현재 계획이 양축 negative-line 오류를 한 축 Taffy 오류로 설명하는가 | selected repeat axis raw path와 secondary-axis `resolve_axis` 경로를 분리했다. 통과. |
| 2 | `repeat_columns` 우선 선택이 rows case를 빠뜨리는가 | 양 축 방향 대칭 control과 별도 regression을 요구한다. 통과. |
| 3 | source error의 원인을 Taffy 내부로 돌리는가 | 오류는 `normalize_auto_repeat_grid_items`의 pre-Taffy `resolve_axis`에서 반환된다고 명시한다. 통과. |
| 4 | authored count가 explicit expanded count인 것처럼 기술되는가 | authored vector length와 Taffy expanded explicit count를 구분한다. 통과. |
| 5 | 한 축 negative line이 현재도 실패한다고 잘못 선언하는가 | 한 축 Rust 대조의 통과를 기록하고 제한을 양축 case로 한정한다. 통과. |
| 6 | second axis resolve 제거가 non-repeat 축 검증까지 비활성화하는가 | non-repeating fixed axis validation은 보존하도록 implementation plan을 한정한다. 통과. |
| 7 | line numeric 좌표가 필요 없는 definite classification을 잘못 계산하는가 | Line/Span/Auto 상태만 구분할 별도 regression과 raw placement 보존을 요구한다. 통과. |
| 8 | `auto_placed_items` 상태가 definite axis item의 intrinsic size 허용을 바꾸는가 | 양축 definite placement와 일부 자동 배치의 profile 분류를 검사한다. 통과. |
| 9 | reversed endpoints가 Spinon과 Taffy에서 서로 다르게 정규화되는가 | source values를 그대로 전달하고 Taffy placement의 reversed behavior를 oracle과 비교한다. 통과. |
| 10 | equal endpoints가 zero span 또는 automatic item으로 변하는가 | equal endpoint case를 별도 regression에 유지하고 Taffy semantics로 판정한다. 통과. |
| 11 | secondary axis negative line에서 leading implicit count를 explicit count에 합산하는가 | expanded explicit mapping과 leading origin shift를 별도로 검증한다. 통과. |
| 12 | positive line이 authored 수를 넘지만 expanded grid 안에 놓일 때 잘못 거부되는가 | 이 경우와 expanded grid 바깥의 trailing implicit control, cap/fail-closed 경계를 구분해 남긴다. 통과. |
| 13 | 양축 count가 track pattern 또는 gap을 공유하는가 | 독립 count 식과 축별 기대 track을 유지하도록 계획에 명시한다. 통과. |
| 14 | auto-fit occupancy 계산 전에 repeated count가 바뀌는가 | auto-fill count와 동일성, post-placement collapse를 각각 검증한다. 통과. |
| 15 | empty auto-fit repeated slots가 trace와 computed vector에서 사라지는가 | collapsed slots와 gutters를 per-axis trace로 확인한다. 통과. |
| 16 | leading auto-list cycle을 뒤집어 `[31,47]`이 `[47,31]`로 되는가 | 1/2/3 leading track expectations와 frame 원점을 고정한다. 통과. |
| 17 | trailing implicit cycle에 leading reverse pattern을 적용하는가 | positive/trailing control은 forward cycle로 유지한다. 통과. |
| 18 | empty, invalid, negative, over-cap list가 px validator를 우회하는가 | current plan에 fail-closed input tests가 있다. 통과. |
| 19 | unsupported intrinsic auto track function이 Taffy adapter로 전달되는가 | 지원 subset을 finite non-negative px만으로 제한한다. 통과. |
| 20 | `LengthPx` scalar 또는 `Auto` 기본값이 목록 경로에서 깨지는가 | scalar와 Auto 대조를 계획에서 독립적으로 다룬다. 통과. |
| 21 | adapter fields가 일반 non-repeat track expansion을 중복 생성하는가 | auto-repeat branch에 한해 Taffy auto track fields를 활성화하도록 요구한다. 통과. |
| 22 | Rust draft 실행의 8/4 집계가 명령 출력과 다르게 적혔는가 | 재현 기록은 정확한 `auto_repeat` 필터 명령과 통과/실패 수를 보존한다. 통과. |
| 23 | 오류 property가 columns와 rows 중 잘못된 축으로 표시되는가 | draft failure `grid-row-start`는 columns repeat 선택 시 secondary rows pre-resolution과 일치한다. 통과. |
| 24 | Chrome 155 gutter 편차를 규범값으로 채택하는가 | 기존 auto-fit Blink divergence와 validator exclusion을 유지한다. 통과. |
| 25 | Chrome screenshot이 Spinon runtime/app screenshot으로 표시되는가 | oracle-only label은 PR body와 README에서 보존한다. 통과. |
| 26 | 초기 plan 20 review가 유효 승인으로 남는가 | initial review superseded note 및 current 20-case review를 별도 링크한다. 통과. |
| 27 | 초기 PR diff 30 review의 incorrect pass verdict가 남는가 | first diff review를 invalidated로 표시하고 current PR review 30을 추가한다. 통과. |
| 28 | 1차 정정 review의 Taffy order 확인이 양축 결함까지 승인한 것으로 남는가 | correction review 20/30도 superseded로 표시하고 새 축 분류 검토로 대체한다. 통과. |
| 29 | test evidence나 review 파일 해시가 README/index에서 누락되거나 오래된 값인가 | draft 재현과 두 새 review를 SHA 목록에 넣고 `shasum -a 256 -c`로 전부 재검증한다. 원본 Chrome artifact 해시는 유지한다. 통과. |
| 30 | plan PR이 simulator/test 통과 또는 C11.4 completion을 과장하는가 | plan은 draft Rust observation을 분리하고 simulator/official status 미완료를 명시한다. 통과. |

## 판정

이번 계획 diff는 양축 secondary-axis 사전 분류와 auto-track 값 전달이라는 두 구현 경계를 명확히 분리한다. 초기와 1차 정정의 틀린 판정은 기록상 무효화되며, simulator 동작은 미검증이다.

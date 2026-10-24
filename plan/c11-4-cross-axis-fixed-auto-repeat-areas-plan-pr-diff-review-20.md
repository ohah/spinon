# C11.4 교차축 fixed-repeat + auto-repeat + areas 계획 PR diff 검토 20개

2026-10-25 기준 계획·fixture·evidence·STATUS·roadmap 변경 13개 staged 경로를 계획 검토와 별도로 서로 다른 실패 관점 20개로 검토했다. 계획·fixture·CRD 기대값·Chrome 관측·validator·로드맵 상태를 함께 대조했다. 이 파일과 최종 STATUS/roadmap review link는 결과를 보존하려고 검토 뒤 추가했으며, 20행 수·링크 경로를 따로 확인했다. 구현 PR의 runtime 검토를 대신하지 않는다.

| # | 독립 실패 관점 | 최종 판정 및 조치 |
|---:|---|---|
| 1 | 계획-only 상태와 공식 완료 집계 | **PASS.** Cascade/runtime은 미착수, C11.4 미완료, 공식 `72/213` 유지로 STATUS와 공개 로드맵을 맞췄다. |
| 2 | W3C CRD 버전과 해석 표시 | **PASS.** Level 1 2025 CRD로 표기했고, cross-axis 허용은 조항을 결합한 해석이라고 명시했다. Chrome 관측이 규범 기대값을 대체하지 않는다. |
| 3 | fixed-repeat 1-item pattern | **PASS.** `[20px]` 반복 장면과 authored order track vector를 기대값·관측에 보존했다. |
| 4 | fixed-repeat 2-item pattern | **PASS.** 초기 oracle 누락을 수정하기 위해 `[16px 20px]` pattern 장면을 추가했다. Chrome resolved columns와 CRD 계산 모두 `[16,20,16,20]`이다. |
| 5 | auto-axis grammar 누출 | **PASS.** auto-axis를 prefix/suffix 없는 single-breadth auto-repeat 하나로 제한하고 fixed-repeat, multi-track pattern, ordinary neighbor의 거부 경로를 적었다. |
| 6 | auto-repeat 산식과 축 매핑 | **PASS.** 44px·76px의 count를 독립 계산하고 `autoRepeatAxis`를 root width/height 및 관측과 연결한다. |
| 7 | fixed-axis area dimension | **PASS.** validator가 fixed-axis expanded track vector 길이와 named-area dimension의 동등성을 검사한다. 계획은 양 방향 mismatch 거부를 별도 구현 조건으로 둔다. |
| 8 | exact named-area matrix | **PASS.** 모든 기대 scene에 행/cell matrix를 보존하고 Chrome CSSOM 행렬과 셀 단위로 대조한다. |
| 9 | auto-fit 점유와 gutter | **PASS.** 빈 area, 한 행 점유, 양쪽 점유 사이 가운데 빈 행을 구별하며 `[20,0,20]`과 마지막 item y=24를 기록했다. |
| 10 | 전치와 shorthand | **PASS.** 첫 장면과 rows/columns 전치 장면의 root·area·track·item geometry가 일치하고, shorthand 범위를 track-list slash로 한정했다. |
| 11 | overflow 증거 범위 | **PASS.** root-relative frame 초과량만 기록하고 PixelCopy/Metal 및 실제 painted pixel 증거로 확대하지 않는다. |
| 12 | 모든 child frame·identity 설명 | **PASS.** 예상값은 모든 child key/frame을 기록한다. capture가 DOM `id`가 아닌 마지막 class token을 key로 사용함을 반영해 “child ID”를 “child key”로 바로잡았다. |
| 13 | 반복 track과 area-added track 경계 | **PASS.** expected `autoRepeatTrackIndices`를 별도 저장하고 단일 group의 정확한 index 구간, 반복 수, area 확장 후 explicit track 수를 validator로 비교한다. |
| 14 | fixed-axis track/area 동등성 | **PASS.** 앞선 검토에서 직접 assert가 빠진 점을 찾아 validator에 추가하고, 11/11 비교를 재실행했다. |
| 15 | observation·screenshot 정합 | **PASS.** Agent Browser 캡처에는 11 scene 전체가 보이며, 새 two-track pattern의 CSSOM/frame 관측이 기대값과 일치한다. viewport, DPR, browser build를 기록했다. |
| 16 | SHA-256 및 JSON integrity | **PASS.** fixture, expected, observation, screenshot, validator, capture code, comparison result, plan의 8개 hash를 metadata와 맞췄다. 중복 JSON key와 scene ID 누락은 없다. |
| 17 | Agent Browser 및 server cleanup | **PASS.** session 목록은 비었고 local HTTP listener, Agent Browser daemon 및 headless/remote-debugging browser process를 종료 확인했다. 별도 사용자 Google Chrome은 그대로 두었다. 확인되지 않은 고정 CDP port 종료 주장은 제거했다. |
| 18 | STATUS·roadmap scene 수와 완료 수 | **PASS.** 10→11 scene 변경을 STATUS와 roadmap에 반영하고 공식 `72/213`, C11.4 미완료 및 plan-only 상태를 유지했다. |
| 19 | 후속 runtime corpus·artifact 경계 | **PASS.** 향후 template-area JSON `29→40`, 전체 fixture `473→484`, 0/1/2-item schema, simulator ID 선택 및 default/opt-in artifact 경계를 구현 작업으로 명시했다. 지금 구현됐다고 쓰지 않는다. |
| 20 | PR 검토 절차와 unrelated 작업 격리 | **PASS.** 계획 검토 20개와 이 계획 PR diff 20개를 분리했다. PR diff 횟수는 사용자가 지정한 20회를 따랐으며, unrelated untracked 작업은 stage하지 않았다. |

최종 staged diff에서 차단 이슈가 남지 않았다. 발견된 validator·fixture·count·cleanup 설명 문제는 수정한 뒤 해당 관점을 재확인했다. `validate-comparison.py` 결과는 11/11 PASS이며, 이 계획 PR 자체는 runtime 구현이나 C11.4 완료 근거가 아니다.

# C11.1 혼합 교차축 자동 배치 PR diff 적대 검토 30개

대상은 PR [#1152](https://github.com/ohah/spinon/pull/1152)의 최초 구현 diff다. base `2b7bb7bc2f70dcff6be782a82a881cb6dcfb5351`, 검토한 구현 head `fb4135c3a1c8052d30bef83bf773c50dc677c9c8`이다. 기존 계획 검토와 구현/runtime 및 source diff 20개 검토는 이 문서에 합산하지 않았다. 30개 항목은 각각 다른 실패 경로 또는 경계 조건을 대상으로 확인했다.

| # | 적대 관점 | 확인 결과 |
|---:|---|---|
| 1 | Root HTML 외 computed-style profile이 새 mode로 유입되는가 | selector와 projection mode 전환이 `RuntimeRootHtmlTextPaintV1`에 한정된다. 통과. |
| 2 | dense placement와 새 sparse route가 동시에 선택되는가 | `dense_route_nodes`에 속하면 후보에서 제외한다. 통과. |
| 3 | template area가 혼합 교차축 경로로 잘못 들어오는가 | `template_areas`가 있으면 후보에서 제외하고 layout validator도 거부한다. 통과. |
| 4 | column/dense auto-flow가 row sparse 알고리즘에 들어오는가 | selector와 layout validator가 `Row`만 허용한다. 통과. |
| 5 | repeat metadata가 실제 track list와 다르게 해석되는가 | fixed/auto repeat metadata가 남은 Grid는 후보에서 제외한다. 통과. |
| 6 | named line metadata가 numeric line처럼 처리되는가 | line-name 그룹이 있으면 selector가 거부하고 layout validator도 거부한다. 통과. |
| 7 | named endpoint 또는 named span이 numeric route에 섞이는가 | typed selector와 layout endpoint validator가 named variants를 거부한다. 통과. |
| 8 | 두 축 모두 auto 또는 양 축 모두 partial인 item이 부분 고정 item으로 오인되는가 | endpoint 조합 분류가 row-locked와 column-locked의 정확한 모양만 받는다. 통과. |
| 9 | fully definite item이 row/column 잠금 집계에서 누락되어 점유 순서가 틀리는가 | fully definite phase는 기존 sparse normalizer로 전달되며 phase 순서 테스트가 프레임을 확인한다. 통과. |
| 10 | fully auto item이 locked item보다 먼저 배치되는가 | 새 route에서만 fully auto를 허용하고 공용 sparse phase가 locked items 뒤에 배치한다. 통과. |
| 11 | row-locked만 있거나 column-locked만 있을 때 새 mode가 과잉 선택되는가 | typed selector와 layout validator 양쪽에서 두 축의 존재를 요구한다. 통과. |
| 12 | `order` 값이 0이 아닌 항목이 source-order sparse placement로 위장하는가 | layout validator가 visible item마다 `order != 0`을 거부한다. 통과. |
| 13 | positioned item이 일반 Grid occupancy로 처리되는가 | static 외 positioning을 거부한다. 통과. |
| 14 | nested, text-bearing, 또는 대체 Grid item이 제한된 empty block item으로 취급되는가 | item validator가 child, nested Grid, text content를 검사하고 제한된 block만 허용한다. 통과. |
| 15 | `display:none` 항목 또는 숨긴 조상이 occupancy를 차지하는가 | RuntimeGridNodes 수집이 `display:none` subtree를 후보에서 제외하고 layout은 hidden item을 건너뛴다. 전용 fixture도 통과한다. |
| 16 | `visibility:hidden` item을 `display:none`처럼 빼서 후속 배치가 앞으로 당겨지는가 | visibility-hidden 항목을 Grid item으로 유지하며 fixture에서 뒤 항목 위치를 확인한다. 통과. |
| 17 | 음수 line 번호가 explicit grid 경계 안에서 잘못 정규화되는가 | layout regression이 양 축 음수 line의 최종 프레임을 확인한다. 통과. |
| 18 | row-locked placement가 explicit row 밖으로 implicit row를 요구해도 조용히 확장되는가 | shared normalizer의 expanded rows를 거부하고 style integration negative scene도 실패를 기대한다. 통과. |
| 19 | column-locked placement가 explicit column 밖으로 implicit extent를 요구하는가 | layout regression이 implicit column 경계를 거부한다. 통과. |
| 20 | 양수 span만 허용한다는 계약에서 0/음수 span이 빠져나오는가 | endpoint validator가 양수 span만 받으며 0 span 회귀 사례는 error를 확인한다. 통과. |
| 21 | 잘못된 endpoint 조합이 partially locked axis로 분류되는가 | row/column line·auto 조합 외에는 `InvalidGridPlacement`로 fail closed한다. 통과. |
| 22 | 빈 track list 또는 0/음수·비유한 track이 배치 계산에 들어오는가 | container validator가 explicit list와 finite non-negative px track을 요구한다. 통과. |
| 23 | track 개수 한도를 넘겨 자원 사용이 제한 없이 증가하는가 | `GRID_MAX_TRACKS`를 초과하면 명시적 오류를 반환한다. 통과. |
| 24 | percentage, flex, intrinsic 또는 repeat track이 fixed-px 범위로 오인되는가 | fixed-px 검증과 repeat metadata 검사가 이를 거부한다. 통과. |
| 25 | 비영 gap, border, padding, margin, overflow 또는 지원 밖 정렬이 결과에 섞이는가 | 제한 container validator가 해당 값을 property별로 거부한다. 통과. |
| 26 | `grid-auto-rows/columns` 값이 implicit track 생성을 유발하는가 | auto/유한 non-negative px만 받고 track-list 기반 implicit sizing은 거부한다. 통과. |
| 27 | 새 placement mode가 공용 normalization dispatch의 일부 검사를 건너뛰는가 | validation, normalization, auto-placement 분기에 mode 등록을 확인했다. 통과. |
| 28 | 기존 same-axis one-axis route가 교차축 변경으로 회귀하는가 | 기존 layout focused regression은 13/13, Root HTML integration은 10/10 통과했다. |
| 29 | default iOS app에 opt-in scene marker가 포함되거나 source guard가 새는가 | Swift compile guard, source regression, flag-off/opt-in executable scans를 확인했다. 기본 marker 없음, opt-in positive control 있음. |
| 30 | PR이 후보 결과를 전체 C11 완료, 실기기 또는 compositor 검증으로 과장하는가 | STATUS와 PR 본문 모두 `72/213`, C11 미완료를 유지하고 AVD software renderer·iOS compositor 전 경계를 밝힌다. 통과. |

결과: 이 검토에서 수정이 필요한 결함은 발견하지 못했다. 스크린샷과 실행 근거는 [runtime README](./README.md)에 연결했다. 범위 밖인 일반 CSS Grid, WPT 적합성, Android 실기기/hardware GPU, iOS compositor 이후 pixel parity는 통과로 간주하지 않았다.

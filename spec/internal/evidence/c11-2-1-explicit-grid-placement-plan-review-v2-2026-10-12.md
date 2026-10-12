# C11.2.1 명시 Grid 배치 계획 보완 검토

**검토일:** 2026-10-12  
**대상:** [`plan/c11-2-1-explicit-grid-placement.md`](../../../plan/c11-2-1-explicit-grid-placement.md)  
**기준:** Chrome 154 사전 비교 v2, Stylo 0.20.0, Taffy 0.14.0  
**결과:** 계획 불일치 1건을 고쳐 재검토했습니다. 구현·모바일 완료 판정은 별도입니다.

| # | 실패 관점 | 검토 결과와 계획상 방어 |
| ---: | --- | --- |
| 1 | `RuntimeBlockFlexGridV1`에 placement를 추가해 기존 C11.1의 성공 범위를 바꿀 수 있다. | 새 `RuntimeBlockFlexGridPlacementV1`을 사용하고 기존 C11.1 inline whitelist와 snapshot을 유지합니다. |
| 2 | `width:auto`·`height:auto` item이 기존 fixed-size 검증에서 거부되어 Chrome scene 자체가 구현 불가능할 수 있다. | 검토 중 실제 충돌을 발견했습니다. 새 profile만 Auto 또는 Fixed dimension을 허용하고 C11.1 gate는 유지하도록 계획을 수정했습니다. |
| 3 | Auto dimension을 열며 text·intrinsic content sizing까지 암묵적으로 지원한다고 해석될 수 있다. | 빈 fixture의 Grid stretch만 포함합니다. min/max·intrinsic content·text measurement는 명시적으로 제외했습니다. |
| 4 | `grid-row` shorthand 일부가 Auto end를 남긴 채 성공할 수 있다. | 4개 computed longhand가 모두 definite line/span으로 정규화될 때만 지원합니다. |
| 5 | `grid-area`와 뒤따르는 longhand cascade 우선순위가 source 문자열 재해석 때문에 바뀔 수 있다. | Stylo computed typed longhand를 기준으로 하고 shorthand/cascade scene을 Chrome과 비교합니다. |
| 6 | 음수 line이 track 개수 기준이 아니라 0-based Taffy index에서 해석될 수 있다. | explicit track count 기준으로 양수 line으로 먼저 정규화합니다. |
| 7 | line 0이 Taffy `Auto` 변환을 통해 오류 없이 배치될 수 있다. | 0은 adapter에서 거부하고 독립 negative test에 둡니다. |
| 8 | 숫자 line이 Taffy의 `i16` 범위에서 잘리거나 wrap될 수 있다. | track 범위를 확인한 뒤 checked conversion만 수행합니다. |
| 9 | span 0 또는 범위 밖 span이 Taffy clamp로 정상처럼 보일 수 있다. | positive span을 요구하고 계산된 양 끝 line이 explicit range를 벗어나면 실패합니다. |
| 10 | `line / span`과 `span / line`의 기준 방향이 뒤바뀔 수 있다. | 시작 anchored·끝 anchored 각 장면과 정규화 단위 테스트를 분리합니다. |
| 11 | 역순 숫자 line이 Taffy와 CSS에서 다르게 처리될 수 있다. | 두 computed line을 먼저 정규화하고 순서를 교환하며 Chrome geometry와 비교합니다. |
| 12 | 동일 line이 0 track 크기나 유효한 자동 배치로 오인될 수 있다. | 동일 line의 CSS auto-placement 전환은 이번 profile에서 명시 거부합니다. |
| 13 | 명시 line이 explicit boundary를 넘어 implicit track을 만들 수 있다. | 모든 양 끝 line을 `1..=track_count + 1`에서 확인합니다. |
| 14 | 두 span 또는 Auto/explicit 혼합이 Grid 자동 배치로 조용히 들어갈 수 있다. | 두 축의 definite line/span을 만들 수 없는 item은 명시 오류로 거부합니다. 한 container 내 all-auto와 all-explicit 혼합도 거부합니다. |
| 15 | 형제·중첩 Grid가 placement mode를 공유해 하나의 scene 전체가 과하게 거부될 수 있다. | 배치 분류는 각 Grid container 직계 자식으로 독립 수행합니다. |
| 16 | `display:none` descendant의 stale placement가 계산을 실패시키거나 화면에 남을 수 있다. | 숨긴 subtree는 분류와 배치에서 제외하고 최종 frame은 기존 0-size 보정 경로를 유지합니다. |
| 17 | 허용된 overlap을 중복 셀 오류로 보거나 item source order를 정렬할 수 있다. | 명시 배치만으로 겹침을 거부하지 않고 DOM/source order를 보존합니다. |
| 18 | geometry 일치를 paint/hit-test 일치로 확대 해석할 수 있다. | fixture 비교는 geometry와 computed values를 고정합니다. draw order·hit-test 검증은 별도이며 C12 stacking 완성 근거로 쓰지 않습니다. |
| 19 | profile을 추가하며 runtime paint, FFI dispatch, Android/iOS runner 중 일부만 기존 profile을 유지할 수 있다. | exhaustive match, paint 선택, FFI entry와 각 플랫폼 실행을 구현 완료 관문에 포함합니다. 아직 통과했다고 간주하지 않습니다. |
| 20 | fixture 또는 reference가 변경되어 이전 결과가 조용히 다른 입력을 가리킬 수 있다. | v2 fixture, inventory, capture tool, helper, Chrome binary, WPT source, screenshot의 SHA-256을 연결합니다. v1은 변경하지 않고 별도 보존합니다. |

## 남은 구현 관문

계획 보완 검토는 끝났습니다. 실제 Stylo cascade부터 Taffy까지의 node identity, 오류 원자성, Android 실기기, iOS Simulator, WGPU surface readback은 구현 및 실행 근거가 있어야 별도로 완료 처리합니다.

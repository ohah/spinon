# C11.1 zero-minimum auto-repeat · PR diff 적대 검토 30개

**대상:** [PR #1216](https://github.com/ohah/spinon/pull/1216)의 최종 diff. 기준 구현 커밋은 `d7c48966def7586e193a187d8a5c492909a8c065`이며, 이 기록과 상태 문서 수정도 diff에 포함했다. 코드 검토는 Rust cascade/projection/layout/runtime, Android/iOS fixture route와 build gate, fixture 및 evidence 문서를 실제 변경 기준으로 대조했다.

| # | 독립 실패 관점 | 실제 diff 대조 및 판정 |
|---:|---|---|
| 1 | 0px 경계 허용 | Stylo cascade, projection grammar, layout contract 모두 finite `minimum >= 0`를 사용한다. zero-minimum만 새로 허용한다. 통과. |
| 2 | 음수·비유한 minimum | Projection 테스트는 음수와 NaN을 거부하고 typed gates도 infinity/NaN을 거부한다. 통과. |
| 3 | 다른 track grammar가 섞이는가 | 지원 타입은 `minmax(LengthPx, 1fr)`로 제한된다. `auto`, percentage, 계산식 및 다른 max breadth는 계속 거부된다. 통과. |
| 4 | repeat pattern 크기 증가 | 기존 1~2개 track 제한을 validator와 count helper 양쪽에서 유지한다. 통과. |
| 5 | line name·area·implicit track 우회 | 기존 unnamed line metadata와 명시 축 제한이 남아 있다. 일반 Grid admission을 열지 않는다. 통과. |
| 6 | indefinite·0·음수 컨테이너 크기 | Count helper는 finite positive fixed axis size만 받는다. zero-size 컨테이너를 1회 반복으로 잘못 수용하지 않는다. 통과. |
| 7 | 음수 또는 비유한 gap | 두 count 경로가 finite non-negative px gap만 허용한다. 통과. |
| 8 | row/column 축 불일치 | Layout capacity가 repeat 축의 height/row gap과 width/column gap을 각각 고른다. row regression이 포함됐다. 통과. |
| 9 | 1px floor가 actual minimum에 섞임 | floor는 count용 값에만 적용되고 projected `GridTrackSize::MinMax`에는 0px가 유지된다. 전용 projection 테스트가 확인한다. 통과. |
| 10 | pattern 내부 gutter 계산 누락 | 분모가 floored minima 합과 `P × gap`을 더하고 분자는 `axis + gap`이다. pattern 내부 및 반복 경계 gutter를 모두 반영한다. 통과. |
| 11 | 첫 반복이 들어가지 않는 좁은 축 | `max(1, floor(...))`로 최소 반복 1회를 유지한다. 테스트/fixture가 독립 count와 대조한다. 통과. |
| 12 | f32 경계에서 count 반올림 | Count arithmetic은 finite f32 computed values를 f64로 변환해 계산한다. 소수 gap 표본이 포함됐다. 통과. |
| 13 | Taffy f32 누적 overflow | Pattern과 gap의 Taffy f32 공간도 별도로 합산하고 finite 여부를 확인한다. 비유한 결과는 거부된다. 통과. |
| 14 | quota 검사 전에 usize 변환 | f64 count를 최대 반복 수와 비교한 다음 usize로 변환한다. 변환 전 상한 검사가 있다. 통과. |
| 15 | quota 정확 경계 | 단일 track 10,000개 및 2-track pattern 10,000개가 layout/projection 테스트에서 허용된다. 통과. |
| 16 | quota 초과를 조용히 잘라냄 | 초과 count는 `GridTrackExpansionOverflow` 또는 projection rejection으로 반환된다. `min(GRID_MAX_TRACKS)` clamping은 제거됐다. 통과. |
| 17 | 축 간 capacity 곱 overflow | Layout 경계에서 expanded track count와 반대 축 track 수의 곱에 `checked_mul`을 사용한다. 통과. |
| 18 | projection/layout count 규칙 불일치 | 두 함수 모두 같은 1px floor, denominator, max-one 및 quota 규칙을 가진다. 동일 경계 사례가 각각 테스트된다. 통과. |
| 19 | computed style이 아닌 authored text 판정 | Root HTML 경로는 기존 typed computed-value converter와 profile validator를 사용한다. fixture ID나 선언 원문 검색으로 허용 여부를 바꾸지 않는다. 통과. |
| 20 | positive-minimum 동작 회귀 | 기존 positive-minimum reference/runtime 테스트를 유지하고 C11.4 sizing regressions도 다시 실행해 기록했다. 통과. |
| 21 | column auto-fill/auto-fit sizing | 고정 fixture와 runtime 테스트가 fill, fit, gap, fractional size 및 empty-fit를 포함한다. 기대 frames가 검증된다. 통과. |
| 22 | row 반복 sizing | Runtime fixture의 row 단일·mixed pattern과 layout row-axis capacity가 실행·검증됐다. 통과. |
| 23 | auto-fit collapse 시점 | 사용 item 배치 후 비어 있는 반복 track이 collapse되는 기존 경계를 유지하며 empty-fit 및 mixed-fit 사례를 포함한다. 통과. |
| 24 | 실제 flex sizing이 count floor에 끌려감 | 0px minimum은 Taffy에 그대로 전달되고 final frame 검사가 0px single track의 flex sizing 결과를 확인한다. 통과. |
| 25 | mixed pattern의 반복 순서 손실 | `[0px, 2px]`의 네 반복에서 item x/width 순서를 확인해 반복 track 순서를 보존한다. 통과. |
| 26 | 초과 반복에서 부분 scene 게시 | Runtime negative matrix에 10,001-track 입력이 추가됐고 기존 no-partial-scene assertion을 통과했다. 통과. |
| 27 | runtime fixture가 10개 scene을 모두 포함하는가 | Fixture에는 10개 grid가 있고 9개 red item frame과 빈 auto-fit 장면을 별도로 검증한다. source SHA-256은 Android/iOS artifact와 일치한다. 통과. |
| 28 | Android opt-in asset 누출·stale copy | 기본 flag 0은 generated asset을 삭제하며 opt-in만 source를 복사한다. 기본 APK asset 부재와 opt-in APK source hash 일치를 fresh install/build 로그로 확인했다. 통과. |
| 29 | Android/iOS flag와 runtime route 우회 | 두 빌드 경로 모두 0/1 이외 값과 C04 GPU 미사용 조합을 거부한다. Android BuildConfig와 iOS bundle lookup이 opt-in fixture를 gate하고 page-scale 조합도 거부한다. 각 Simulator runtime에서 draw/present를 기록했다. 통과. |
| 30 | PR evidence 및 상태 설명의 stale/broken link | PR 본문 raw image 두 개를 Agent Browser에서 확인했다: Android `1080×2400`, iOS `1206×2622`, 모두 complete/naturalWidth>0로 실제 렌더링됐다. 검토 중 plan/README의 “PR 전” 표기가 stale임을 발견해 #1216 open 및 검토 완료 상태로 수정했다. C11 및 공식 `78/214`는 미완료로 명시했다. 수정 완료. |

**결론:** 30개 서로 다른 diff 실패 관점을 모두 확인했다. 발견된 문제는 상태 문서의 PR 전 표기였고 이 PR 안에서 수정했다. 코드 결함은 발견하지 못했다. Android 검증은 ANGLE/SwiftShader AVD, iOS 검증은 Simulator Metal drawable이며 hardware GPU·실기기 또는 compositor 이후 pixel parity로 확대 주장하지 않는다. C11.1/C11 및 공식 완료 수는 바꾸지 않는다.

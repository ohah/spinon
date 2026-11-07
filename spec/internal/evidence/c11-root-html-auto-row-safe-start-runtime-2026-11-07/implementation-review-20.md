# C11.4 `safe|unsafe start` 구현/runtime 실패 관점 20개

후보 source, Rust layout/style/runtime 경로와 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 실행을 별도로 검토했다. 계획 검토표는 재사용하지 않았다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 지원 범위가 safety modifier가 붙은 모든 content-position으로 넓어지는가 | Layout과 computed-value gate에는 정확 `safe start`·`unsafe start`만 추가했다. `safe flex-start` negative control은 거부된다. 통과. |
| 2 | Stylo serialization에서 `safe`와 `unsafe`가 plain `start`로 축약되는가 | 새 projection test가 computed `align-content` 문자열을 각 scene별로 그대로 검사한다. 2/2 통과. |
| 3 | unsupported 값이 다른 속성의 오류로 보고되는가 | `safe flex-start`는 projection과 RuntimeSession에서 `align-content`로 귀속된다. 통과. |
| 4 | positive space에서 `safe start`가 가운데나 끝에 배치되는가 | Layout/style/runtime oracle의 `[0,10,20]px`과 Simulator screenshot 경계를 확인했다. 통과. |
| 5 | positive space에서 `unsafe start`가 다른 좌표를 내는가 | Rust six-scene matrix와 style serialization이 plain start와 같은 시작 좌표를 확인한다. 통과. |
| 6 | plain `start`가 새 safety 값 추가로 회귀하는가 | Chrome·독립 기대값의 positive/overflow control과 기존 Root HTML profile 회귀가 통과한다. |
| 7 | overflow에서 `safe start`가 잘못된 safety fallback을 하는가 | Expected `[0,56,112]px`; Android/iOS screenshot bounds 모두 허용 오차 안이다. 통과. |
| 8 | overflow에서 `unsafe start`가 negative offset으로 바뀌는가 | Start placement expected `[0,56,112]px`; 양 플랫폼 screenshot 경계도 동일하다. 통과. |
| 9 | 정렬값이 Grid track sizing을 바꾸는가 | Track trace가 used rows `[10,10,10]px`를 검사한다. 통과. |
| 10 | `calc()` percentage가 viewport 또는 잘못된 축에서 resolve되는가 | definite 40px row basis에서 46px gutter 두 개를 확인한다. 통과. |
| 11 | computed alignment이 `justify-content`로 잘못 투영되는가 | 새 allowlist는 content alignment profile에만 추가됐고 fixed fixture가 `justify-content` 기본값을 유지한다. 통과. |
| 12 | 일반 author CSS admission이 전역으로 넓어지는가 | 기존 parsed Root HTML computed-value 및 `RootHtmlAutoRowStretchV1` gate만 수정했다. parser/public CSS admission은 변경하지 않았다. 통과. |
| 13 | legacy intrinsic-row profile이 새 값까지 허용하는가 | 지원 mode를 RootHtmlAutoRowStretchV1로 한정했다. legacy gate는 변경하지 않았다. 통과. |
| 14 | 다른 row/column topology가 지원 profile로 잘못 분류되는가 | exact track, definite box, item order, text style 및 gap 조건을 유지했다. baseline negative control도 통과한다. |
| 15 | 원본 direct-text item 순서나 node ownership이 바뀌는가 | fixed empty item 뒤의 `A`, `B` 순서를 runtime frame과 paint color로 검사했다. 통과. |
| 16 | overflow negative control에서 미완료 frame이 publish되는가 | 지원 scene의 complete snapshot/cascade revision 일치와 unsupported failure의 snapshot 부재를 검사한다. 통과. |
| 17 | 실패 뒤 같은 RuntimeSession에서 회복하지 못하는가 | dark media의 unsupported alignment 이후 light media의 지원값으로 `Ready` 복구를 확인했다. 통과. |
| 18 | Simulator 캡처가 fixture-only ABI에 기대는가 | Android/iOS 기본 Root HTML route를 C11 fixture flags 비활성으로 실행했다. 통과. |
| 19 | OS screenshot analyzer가 Grid background occlusion 때문에 y=0을 잘못 추정하는가 | Positive start의 fixed item top을 root origin으로 사용하고 1 CSS px 허용 오차로 검증한다. 통과. |
| 20 | 검사 뒤 앱·빌드·bootstrap 상태를 덮어쓴 채 남기는가 | 네 pass 모두 설치 앱과 기존 build/bootstrap hash를 복원하고 restoration PASS를 기록했다. 통과. |

발견된 구현 결함은 없었다. Simulator 이미지 분석에서 background-first origin 가정이 positive start에 맞지 않는 점을 찾아 fixed-item anchor로 수정하고, 세 scene을 재측정했다. Screenshot은 PixelCopy/Metal drawable readback이 아니며 이 검토는 physical device, WPT 또는 C11 완료를 주장하지 않는다.

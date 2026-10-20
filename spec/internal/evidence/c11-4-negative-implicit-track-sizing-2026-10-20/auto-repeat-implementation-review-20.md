# C11.4 auto-repeat negative-line implementation review

2026-10-20 구현 diff를 계획 검토와 분리해 20개 실패 관점으로 확인했다. 현재 구현은 반복 횟수와 negative line을 독립 기대값에 대조하고, auto-fit은 빈 반복 track 및 양쪽 gutter를 CSS Grid 규범대로 접는다. Chrome 155가 일부 경계 gutter를 보존하는 관찰은 기대값으로 사용하지 않는다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 신규 18개 runtime ID와 Chrome/규범 기대 case가 서로 빠지거나 바뀌는가 | `autoRepeatNegativeLineScenes()`가 세 JSON의 18개 ID 집합을 각각 비교한다. 최종 장면 ID는 중복 없이 124개다. |
| 2 | 기존 auto-repeat 33개 scene이 같은 개수지만 다른 ID로 교체되는가 | 첫 33개 ID를 고정 배열과 순서대로 `deepEqual`한다. 검토 중 추가한 회귀 검증이 현재 `--list-scenes`에서 통과했다. |
| 3 | 기존 C11.4 matrix를 덮어쓰거나 새 행렬만 실행하는가 | 53·73·106개 기존 단계 assertion 뒤에 새 18개를 붙이고 124개 총합과 고유 ID를 검증한다. |
| 4 | auto-repeat 횟수가 content-box와 prefix/suffix, gap을 반영하지 않는가 | 18개 독립 expectation의 definite available size, repeat count, prefix/suffix, leading/trailing implicit track 수와 Chrome observation을 각각 비교한다. |
| 5 | 열과 행의 repeat 경계에서 반복 수가 대칭적이지 않은가 | 134·180·200·206·230px 경계와 repeat-only axis가 기대 목록 및 양 플랫폼 matrix에 포함된다. |
| 6 | negative line이 repeated explicit grid가 아니라 authored track 수를 기준으로 해석되는가 | auto-repeat case의 `-8`, `-7`, `-6` line을 독립 모델의 expanded repeat boundary와 비교한다. 최종 track list에는 leading implicit auto track 크기도 포함된다. |
| 7 | reversed line, span, prefix를 가로지르는 placement가 경계 정규화를 놓치는가 | reversed negative line과 `-8 / -5` spanning case에서 resolved lines, item frame, track trace를 비교한다. |
| 8 | expanded repeat grid 뒤 positive line이 거부되거나 trailing implicit track 주기가 틀리는가 | `columns-fill-negative-positive-cycle`은 x=284의 trailing item과 뒤쪽 implicit track 크기 31px을 검사한다. 화면 표본도 이 item과 tail을 확인한다. |
| 9 | leading `grid-auto-columns`/`grid-auto-rows` 주기가 경계에서 역방향으로 시작하지 않는가 | asymmetric `31px 47px` / `17px 29px` pattern과 복수 leading track case의 track trace를 대조한다. |
| 10 | repeat-only axis에서 prefix/suffix가 없는 경우 line 원점이 잘못 이동하는가 | `columns-fill-repeat-only-leading` 및 rows counterpart의 used track, item 위치와 leading pattern을 양 시뮬레이터에서 확인했다. |
| 11 | 양축 repeat 수와 auto track list가 한 축 값으로 덮이는가 | `both-axes-fit-negative-leading`와 `both-axes-fill-independent-cycles`에서 columns/rows의 독립 track 수와 크기를 각각 검증한다. |
| 12 | `auto-fill`이 항목 부재만으로 반복 track을 접는가 | `auto-fill` cases는 모든 repeated track의 `collapsed=false`와 최종 track 크기를 검사한다. |
| 13 | `auto-fit`이 item placement 이후 점유된 반복 track까지 접는가 | 단일·span·복수 점유 case가 occupied repeat를 유지하고 빈 반복만 collapse하는지 frame 및 trace로 검사한다. |
| 14 | 빈 auto-fit track 양옆 또는 explicit prefix/suffix 경계의 gutter가 남는가 | vendored Taffy와 Rust 회귀는 빈 track 양쪽 gutter를 0으로 확인한다. Chrome 155 boundary 차이는 별도 관찰로 남기고 CSS 규범 기대를 적용한다. |
| 15 | 활성 반복 track이 첫 번째·중간·마지막일 때 공간 분배와 원점이 달라지는가 | style-to-layout matrix는 첫 반복 162px/x=28, 가운데 반복 170px/x=20, 마지막 반복 162px/x=20을 각각 대조한다. span 두 track은 77px씩 확인한다. |
| 16 | span·prefix·suffix item frame이 track 크기만 맞고 배치는 틀리는가 | 각 scene에서 예상 node frame 전체와 track stage를 독립 비교한다. row/column과 prefix/suffix 결합 case가 포함된다. |
| 17 | simulator 화면에 표시되지 않은 계산 결과가 통과로 기록되는가 | 캡처 검증은 runtime summary, 완전한 trace chunk, node frame, draw/present marker를 scene마다 요구한다. 최종 124개 모두 통과했다. |
| 18 | Android PixelCopy가 새 overflow scene을 허용하지 않거나 다른 노드/색을 읽는가 | scene allowlist, 표본 ID·root/item node ID·좌표·색을 함께 검증한다. 최종 AVD 로그는 `columns-fill-negative-positive-cycle` 5/5 exact다. |
| 19 | iOS Metal readback이 빠진 feature flag 또는 Android와 다른 좌표/색을 사용해 통과하는가 | iOS sample allowlist와 정적 기대값을 대조하고 `c11_grid_metal_surface_pixels=1` 빌드로 실행했다. 최종 drawable 로그는 같은 scene 5/5 exact다. |
| 20 | 통합 결과가 물리 기기·compositor·전체 CSS Grid 적합성으로 과장되는가 | API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 각각 124/124, 전체 trace/draw 결과, 세 overflow scene의 5/5 표본을 보존했다. AVD는 software renderer이고 iOS 표본은 OS compositor 제출 전이다. |

## 검토 중 수정하고 다시 확인한 항목

- 이전 style-to-layout matrix는 Chrome 경계 gutter에 맞춰 `154px`와 `73px`를 기대했다. 규범에 맞는 `162px`, 위치별 `170px`/`162px`, `77px`로 바꾸고 첫/중간/마지막 track 위치 focused test와 workspace 전체 테스트를 통과시켰다.
- Android PixelCopy 허용목록에 새 overflow scene ID가 없어 `scene-index-mismatch`가 났다. 허용목록과 표본 색 계약을 추가한 뒤 최종 scene을 포함한 124/124 캡처에서 확인했다.
- 첫 iOS capture는 Metal pixel feature 없이 빌드한 앱으로 시작해 10번째 scene에서 멈췄다. feature flag를 켜서 다시 빌드하고 전체 124/124를 통과했다.
- 최종 `cargo test --locked --workspace`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, FFI opt-in scene test 1/1, fixture module boundary test 6/6이 통과했다. 구현 blocking finding은 남지 않았다.

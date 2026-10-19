# C11.4 양쪽 축 auto-repeat 구현 후 20개 실패 관점 검토

**검토 대상:** cascade·typed projection·layout validation·track sizing·simulator fixture/capture 변경과 suffix geometry 회귀. 계획 자체의 20개 검토는 재사용하지 않았다.

| # | 적대적 실패 경로 | 확인과 결과 |
| --- | --- | --- |
| 1 | columns와 rows를 모두 선언했을 때 두 번째 축 metadata가 cascade에서 덮이거나 거부된다. | `track_sizing_profile_preserves_independent_auto_repeats_on_both_axes`가 columns `auto-fit`, rows `auto-fill`의 mode와 half-open range를 각각 보존한다. |
| 2 | 기존 column-only `auto-fill`/`auto-fit` 경로가 dual-axis 조건 변경에 따라 거부된다. | `track_sizing_profile_preserves_fixed_minimum_auto_repeat_modes`가 두 keyword를 각각 검사하고 rows metadata가 없음을 확인한다. |
| 3 | typed projection이 둘 중 한 axis의 repeat mode/range를 지운다. | `track_sizing_profile_projects_independent_auto_repeat_modes_on_both_axes`가 양쪽 `GridAutoRepeat` 값을 독립 비교한다. |
| 4 | 새 양축 회귀가 과거 column-only projection 동작을 대체해 회귀를 숨긴다. | `track_sizing_profile_keeps_single_axis_auto_repeat_projection`을 별도로 복구해 column metadata, 반대 축 `None`, line-name N+1 입력을 검사한다. |
| 5 | layout validator가 두 유효한 axis를 여전히 서로 배타적으로 취급한다. | `auto_repeat_accepts_independent_repeats_on_both_axes`가 Columns와 Rows track-sizing trace 생성을 확인한다. |
| 6 | rows가 유효하면 잘못된 columns repeat range가 우연히 허용된다. | `auto_repeat_rejects_an_invalid_first_axis_independently`가 `grid-template-columns` 오류와 node identity를 확인한다. |
| 7 | columns가 유효하면 잘못된 rows repeat range가 우연히 허용된다. | `auto_repeat_rejects_an_invalid_second_axis_independently`가 `grid-template-rows` 오류와 node identity를 확인한다. |
| 8 | 빈 range 또는 track 수를 넘는 range에서 slice panic/부분 결과가 발생한다. | `validate_auto_repeat_axis`가 slice 전에 `start >= end || end > tracks.len()`을 거부한다. 두 axis의 out-of-range 경로는 별도 regression으로 실행했다. 같은 경계의 `start == end`는 코드 guard로 확인했으며 전용 runtime scene으로 주장하지 않는다. |
| 9 | repeat이 있는 축의 named line이 확장 구간을 잘못 이동시킨다. | `auto_repeat_rejects_named_lines_through_the_layout_api`가 columns와 rows 각각의 반복축 line names를 거부한다. |
| 10 | 두 축 지원을 켠 뒤 `grid-template-areas`와 auto-repeat 조합이 새로 허용된다. | cascade는 두 axis metadata를 검사한 뒤에도 `template_areas.is_some()`을 fail-closed 처리한다. 기존 malformed/profile fixture가 이 경계를 유지한다. |
| 11 | 한 track list 안의 두 번째 auto-repeat 또는 integer-repeat 혼합이 단일 repeat인 것처럼 투영된다. | `auto_repeat_rejects_profiles_and_shapes_outside_the_supported_subset`에 복수 repeat·integer/auto-repeat 혼합 사례가 남아 있으며 거부된다. |
| 12 | repeated fragment의 definite fixed minimum이 없는 grammar가 layout까지 통과한다. | cascade의 fixed-minimum track parser와 layout의 `auto_repeat_track_has_fixed_minimum` 검증을 함께 대조했다. 현재 subset 외 문법은 계속 거부된다. |
| 13 | explicit neighbor가 `%`, 음수 또는 비유한 `px`여도 repeat 계산의 고정 사용량으로 처리된다. | `auto_repeat_neighbor_track_is_fixed_px`는 finite 0 이상 `LengthPx`만 허용한다. 사전 거부 fixture에 비-`px` neighbor가 유지된다. |
| 14 | 두 축이 같은 repeat count를 공유해 200px과 266px 축의 차이가 사라진다. | `both-fill-square`와 `both-fill-asymmetric`의 simulator 기대값은 각각 2회/2회와 2회/3회다. 각 axis track vector가 별도로 비교된다. |
| 15 | `auto-fill`에서 빈 반복 track을 `auto-fit`처럼 collapse한다. | one-axis controls와 `columns-fit-rows-fill-empty`/`columns-fill-rows-fit-empty`에서 fill 축 track은 유지되고 fit 축만 접힌다. |
| 16 | 두 축 `auto-fit`에서 item이 없을 때 축 중 하나의 반복 track이나 gap이 남는다. | `both-fit-empty`는 양쪽 `[20, 0, 0, 0, 0, 10]` track과 collapse flag 및 내부 gutter를 비교한다. |
| 17 | 한 repeated track을 차지한 item 때문에 다른 빈 반복 track도 active 상태로 남는다. | `both-fit-single`의 양축 trace는 occupied 154px track 하나와 나머지 zero/collapsed tracks를 확인한다. |
| 18 | item이 두 repeated track을 span해도 첫 track만 점유로 계산된다. | `both-fit-span-two`는 양쪽 `[20, 126, 20, 0, 0, 10]`와 뒤 반복의 collapse 상태를 검사한다. |
| 19 | 두 번째 repetition에만 놓인 item이 첫 repeat로 이동하거나 mixed mode 반대 축을 변경한다. | `both-fit-second-repeat` 및 두 mixed second-repeat scene에서 axis별 track vector와 `(28,109)`/`(109,28)` item frame을 비교한다. |
| 20 | collapsed track의 겹치는 gutter를 두 번 더하거나 모두 제거해 explicit suffix line이 이동한다. | CSS Grid 규범의 gutter overlap 해석을 대조하고, Agent Browser Chrome 155 suffix probe와 `auto_fit_collapsed_gutters_preserve_explicit_suffix_line_geometry`가 양쪽 suffix item `(190,190)`을 확인한다. 관찰 원본은 [Chrome probe JSON](./chrome155-autofit-suffix-probe.json)이다. |

## 교차 실행 근거

- [Android API 37 ARM64 AVD](../c11-4-auto-repeat-two-track-min-size-2026-10-20/android-avd-c114-neighbors-android-avd-20261019T221920350Z-cf547c61/result.json): 33개 고유 scene, 모두 `drawPresented: true`; overflow 장면의 root 안/밖과 root 바깥 item 표본도 exact다. AVD serial·API·ABI·APK SHA는 같은 폴더의 `simulator.json`에 있다.
- [iPhone 17 Pro / iOS 26.2 Simulator](../c11-4-auto-repeat-two-track-min-size-2026-10-20/ios-simulator-c114-neighbors-ios-simulator-20261019T220311971Z-4cde0c2a/result.json): 33개 고유 scene, 모두 `drawPresented: true`; Metal surface overflow 표본은 exact다. Simulator UDID와 app SHA는 `simulator.json`에 있다.
- `capture-c11-4-auto-repeat-neighbors-simulator.mjs`는 33개 scene ID의 중복·누락, scene/run ID, 모든 trace chunk 복원, axis별 track/collapse/gutter, draw/present 및 pixel error/mismatch marker를 확인한다. Android 실행은 지정 AVD의 `ro.kernel.qemu=1`, API 37, ARM64를 확인한다. iOS 실행은 고정 Simulator 이름과 iOS 26.2 runtime을 확인한다.
- 기본 `spinon-ffi` rlib에서 fixture sentinel이 빠지고 opt-in fixture-test rlib에 포함되는 경계는 [artifact 기록](./fixture-artifact-boundary.json)으로 남겼다. 이는 static rlib 결과이며 최종 app 크기 검증이 아니다.
- workspace 테스트와 Clippy 결과는 `rust-workspace-test.log`, `rust-workspace-clippy.log`에 있다. `cargo fmt --all -- --check`와 `git diff --check`도 통과했다.

## 검토 결론과 미검증 범위

위 20개 경로에서 차단 결함은 남지 않았다. 이 검토와 두 Simulator 행렬은 WPT, 실기기, hardware GPU, iOS compositor 이후 pixel equality 또는 전체 C11.4 지원을 증명하지 않는다. 공식 C11.4 완료 체크와 roadmap 완료 수는 바뀌지 않는다.

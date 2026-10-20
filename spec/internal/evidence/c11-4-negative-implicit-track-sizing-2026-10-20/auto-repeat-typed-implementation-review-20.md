# C11.4 auto-repeat typed implicit track sizing 구현 후 20개 실패 관점 검토

**검토 대상:** typed `grid-auto-rows`·`grid-auto-columns` projection, Taffy conversion, Calc binding, runtime fixture/text input, Android/iOS capture path와 최신 138-scene evidence.
**검토 기준:** 구현 코드와 실행 결과를 서로 다른 실패 경로로 대조했다. 구현 전 계획의 20개 표는 재사용하지 않았다.
**결과:** 20개 관점을 확인했고 확인 과정에서 찾은 검증 결함은 수정 후 관련 검사를 다시 통과했다.

| # | 적대적 실패 경로 | 확인과 결과 |
| ---: | --- | --- |
| 1 | typed auto-track 허용이 C11.2.2/C11.2.3 또는 TemplateGrid profile로 새어 기존 fail-closed 계약을 바꾼다. | `typed_auto_track_lists_remain_rejected_outside_track_sizing_profile`은 `grid-auto-rows`와 `grid-auto-columns` rejection을 각각 검사한다. focused layout 13/13 통과. |
| 2 | `Auto` 또는 scalar `LengthPx` auto sizing이 새 list converter에 끌려가 기본 동작이 바뀐다. | `to_taffy_grid_auto_tracks`의 기존 Auto/LengthPx 분기와 `auto_repeat_scalar_auto_track_sizes_cover_leading_implicit_tracks`를 확인했다. |
| 3 | percentage·fraction·auto·intrinsic·minmax·fit-content typed list가 여전히 fixed-px 전용 converter에서 거부되거나 일부 variant가 누락된다. | `auto_repeat_implicit_lists_convert_supported_track_sizing_functions`가 양 축의 supported function set을 실행하고 최종 sizes가 유한한지 확인한다. |
| 4 | `grid-auto-columns` Calc가 rows identity를 쓰거나 `grid-auto-rows` Calc가 columns identity를 사용한다. | `grid_math_property` mapping과 `auto_repeat_calc_implicit_tracks_use_axis_math_bindings`의 25px/23px axis-specific 결과를 대조했다. |
| 5 | `minmax()` 또는 `fit-content()` 내부 Calc ID 검증을 우회해 다른 node/축의 expression이 연결된다. | `validate_grid_track_math_references`는 top-level과 nested Calc ID를 같은 node 및 axis math property에 연결한다. `auto_repeat_nested_calc_tracks_validate_axis_and_node_binding` 통과. |
| 6 | 없는 ID, 존재하지만 잘못 바인딩된 ID가 Taffy conversion에서 누락되거나 panic한다. | `auto_repeat_calc_implicit_track_rejects_missing_axis_binding`와 mismatched node test가 각각 `MissingCssMath`와 `CssMathBindingMismatch`를 요구한다. |
| 7 | negative/non-finite track 값이나 empty list가 유효한 computed style로 통과하거나 `0px`를 잘못 거부한다. | 공용 `validate_tracks`의 제한을 확인했고, `zero_px_auto_track_size_remains_zero_on_leading_implicit_tracks` 및 전체 workspace 테스트가 통과했다. |
| 8 | negative line을 repeat expansion 전 authored track 수에 맞춰 leading implicit extent를 한 track 어긋나게 만든다. | `numeric_negative_line_uses_expanded_auto_repeat_track_count`와 138-scene 결과의 `nodeFrames`·used tracks를 확인했다. |
| 9 | explicit-grid 앞과 뒤의 auto-track list cycle 방향이 같아져 leading list authored order가 뒤집힌다. | `auto_repeat_leading_auto_track_lists_cycle_backwards_on_both_axes`와 mixed typed-cycle trace에서 양·음 cycle 방향을 비교했다. |
| 10 | columns/rows가 한 axis의 Calc basis, list 길이 또는 repeat count를 공유해 비대칭 fixture에서 서로 덮는다. | axis별 math ID test와 `cross-platform-comparison-20261020.json`의 1,019개 track 값 및 138 scene IDs를 비교했다. Android/iOS 최대 track delta는 0 CSS px다. |
| 11 | `auto-fit`이 occupied track까지 접거나 empty track을 유지한다. | occupied/empty typed case, 기존 `auto_fit_keeps_repeat_count_and_collapses_empty_repeats_after_leading_placement`, 양 플랫폼 trace의 collapse flag를 확인했다. |
| 12 | collapsed track 주변 gutter를 남기거나 중복 제거해 후속 item 좌표를 이동시킨다. | `typed-auto-fit-gutter` 및 overflow/mixed-axis surface case에서 frame, gutter trace, Android PixelCopy와 iOS Metal sample을 검사했다. |
| 13 | typed text fixture가 `textContent` 값만 두고 HostDocument text leaf를 만들지 않아 intrinsic input이 비어 있다. | item 생성 코드를 `createTextNode` 삽입으로 바꿨다. typed text cases의 trace에는 Ahem min/max-content input이 나타나며 runtime test package가 통과했다. |
| 14 | text item 또는 root의 예상 외 child/tag/id/style가 allowlist를 통과해 고정 Ahem 측정값을 다른 입력에 적용한다. | `prepare_typed_auto_repeat_text`는 root/item identity, parent-child order, text literal과 computed text style을 확인하고 다른 형태를 구체 오류로 거부한다. |
| 15 | fixed-minimum `minmax()` text case가 CSS automatic minimum 때문에 track을 과도하게 키운다. | `typed-mixed-cycle`과 `typed-minmax-max-content`에서 `min-height:0`을 고정 검증하고 나머지 case는 min-size-auto 경로를 유지했다. 14개 typed simulator scene 모두 통과. |
| 16 | Android JNI result buffer가 긴 trace를 잘라 실제 FFI 오류를 `status=-3`로 가린다. | 32 KiB stack array를 heap `std::vector<char>` 1 MiB로 바꾸고 오류 상세를 700-byte log chunk로 분리했다. Android 전체 matrix 138/138 통과. |
| 17 | iOS eval/environment 경로가 같은 긴 report를 자르거나 대형 stack array를 쓴다. | C11.4 및 environment result storage를 heap vector 1 MiB로 바꾸고 NUL-terminated `const char*` report converter를 사용했다. iOS Simulator build와 전체 138/138 통과. |
| 18 | iOS runtime summary는 성공하지만 node-level frame 로그가 없어 capture가 실제 좌표를 검증하지 못한다. | 첫 iOS 실행에서 이를 발견해 Swift C11.4 completion 경로에 frame logger를 추가했다. 이후 full matrix에서 960 scalar가 Android와 0 CSS px 차이로 비교됐다. |
| 19 | capture script가 소수점 f32 frame을 문자열 exact 비교하거나 전체 matrix 대신 선택 장면만 보고 성공 처리한다. | node frame을 구조로 파싱해 1/64 CSS px tolerance로 비교하고 기본 실행의 runScenes 138개 모두를 순서대로 검사한다. 138 draw/present 및 scene별 screenshot을 확인했다. |
| 20 | opt-in fixture FFI가 기본 product artifact에 들어가거나 surface sample을 CSS border pixel·compositor 결과로 과장한다. | default staticlib에는 C11.4 FFI symbol 두 개가 없고 fixture feature artifact에는 둘 다 있다. overflow 5개 scene의 24/24 sample만 기록하며 edge 값은 root background, iOS는 compositor 이전 drawable이다. |

## 검토 중 발견해 수정한 사항

- Android 및 iOS C11.4/environment report가 32 KiB를 넘어 잘릴 수 있어 heap-backed 1 MiB output buffer로 바꾸고 Android 오류 상세를 chunk 단위로 남겼다.
- iOS Swift fixture completion 경로에 실제 node frame 로그가 없어 capture checker가 좌표 수를 0으로 읽었다. frame logger를 추가한 뒤 단일 장면과 전체 matrix를 다시 통과시켰다.
- 새 typed text scene은 `textContent` property 설정만으로 HostDocument text node를 만들지 않았다. 직접 text leaf를 삽입하도록 fixture builder를 고쳤다.
- runtime image test fixture가 validator가 요구하는 `align-items:start`를 생략했다. 입력 style을 명시해 fixture-only test를 통과시켰다.
- fixture boundary test는 `c04-runtime-gpu` feature가 renderer 외에 optional core dependency도 가진 현재 manifest를 반영하지 못했다. assertion을 renderer 의존성 포함 여부 검사로 바꿔 6/6 통과시켰다.

## 실행 근거

- `cargo test --locked --workspace --quiet`: 1,182 passed, 0 failed, 4 ignored.
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test`: 209 passed, 0 failed, 3 ignored.
- focused layout auto-repeat 13/13, style-to-layout projection 1/1, FFI scene registry 1/1, fixture module boundary 6/6, workspace Clippy, rustfmt와 opt-in artifact negative/positive control 통과.
- Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 각각 138/138 scene. 전체 summary와 한계는 [실행 evidence](./README.md), frame/track 교차비교는 [JSON](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/cross-platform-comparison-20261020.json)을 따른다.

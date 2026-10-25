# C11.4 orthogonal row intrinsic runtime 구현 검토 20개

## 범위

네 개의 orthogonal row 장면(min/max-content × forward/reversed)에 대한 실제 구현을 계획 검토와 별도로 정적 검토했다. 확인 범위는 CSS cascade와 Style→Taffy projection, Taffy alignment 동작, logical/physical intrinsic 값, Runtime 장면 준비와 출력 검증, trace/JSON, FFI 장면 라우팅, Android/iOS adapter다.

아래 표의 근거는 현재 코드와 테스트 assertion이다. 이번 검토에서는 테스트·빌드·시뮬레이터를 실행하지 않았다. 따라서 assertion이 있다는 사실을 동적 통과 결과로 해석하지 않는다.

## 실패 관점 20개

| # | 실패 관점 | 코드·테스트 근거 | 정적 판정 |
|---:|---|---|---|
| 1 | cascade가 부모와 자식의 writing mode를 뒤바꾸거나 상속하지 않는가 | `crates/spinon-style-to-layout/src/c11_4_track_sizing_areas_tests/orthogonal_writing_mode.rs:107-160`에서 부모 `HorizontalTb`, 텍스트 `VerticalRl`, 이미지 `HorizontalTb` 및 계산된 property를 각각 확인한다. | 각 노드의 계산 결과를 따로 확인한다. |
| 2 | CSS `align-items: normal`이 cascade 뒤 `stretch`로 정규화되는가 | 같은 테스트 `:126-140`은 root의 계산 속성이 `normal`이고 아이템의 `align-self`가 `auto`인지 확인한다. | `normal` keyword가 projection 입력까지 유지되는지 직접 다룬다. |
| 3 | Style→Taffy 변환이 `normal`과 `stretch`를 같은 값으로 합치는가 | `crates/spinon-layout/src/taffy_style.rs:395-409`에서 별도 매핑하고 `crates/spinon-layout/src/taffy_style/tests.rs:7-17`에서 두 Taffy 값이 각각 다른지 검사한다. | mapping 차이를 직접 고정한다. |
| 4 | Grid의 `normal`이 replaced image까지 늘리는가 | 고정한 Taffy 코드 `vendor/taffy/src/compute/grid/types/grid_item.rs:357-364,438-488`에서 `normal`은 non-replaced 항목에만 stretch fallback으로 쓰고 replaced 항목은 구분한다. | 장면에 필요한 replaced 구분을 의존성 코드에서 확인했다. |
| 5 | 공용 매핑 변경이 Flex의 `normal` 동작을 바꾸는가 | `vendor/taffy/src/compute/flexbox.rs:647-654,696-699`는 Flex 계산에서 `normal`을 stretch로 해석한다. | `normal`을 보존해 Taffy로 전달해도 Flex 내부 fallback은 남아 있다. 다만 이 리뷰에서는 Flex 실행 검증을 하지 않았다. |
| 6 | 텍스트의 min/max logical intrinsic 값이 writing mode에서 잘못 측정되는가 | 같은 Style→Layout 테스트 `:172-189,239-267`은 Ahem `XX XXXXX`의 min `(inline 50, block 20)` 및 max `(inline 80, block 10)`을 확인한다. | 네 장면 모두 해당 텍스트를 고정 입력으로 사용한다. |
| 7 | logical→physical 축 전환에서 inline/block 축이 바뀌는가 | `crates/spinon-layout/src/intrinsic_measurement.rs:223-253`는 지원 조합을 제한한 뒤 physical width에 logical block, height에 logical inline을 넣는다. 테스트 `orthogonal_writing_mode.rs:263-267`도 `(20,10)` width와 `(50,80)` height 값을 검사한다. | row 높이에 해당하는 physical height가 inline 값을 받는지 확인한다. |
| 8 | 지원하지 않는 writing-mode 조합 또는 NaN/음수 intrinsic 값이 조용히 계산되는가 | `intrinsic_measurement.rs:231-246`은 다른 parent/child/orientation 조합과 잘못된 logical size를 오류로 처리한다. `orthogonal_writing_mode.rs:386-418`에는 author writing-style 불일치 음성 검사가 있다. | 적용 범위 밖 조합은 이 경로에서 fail closed 한다. |
| 9 | Style→Taffy 장면이 기대한 row·column·정렬 입력과 다르게 구성되는가 | `orthogonal_writing_mode.rs:47-76,126-169`는 50/24 column, min/max row, 1번 row의 두 column 배치, `align-items:normal`, `align-content:start` 및 computed style을 확인한다. | 실제 cascade 결과를 layout 계산에 넣는 장면이다. |
| 10 | min-content와 max-content row가 같은 크기로 계산되거나 반대로 매핑되는가 | `orthogonal_writing_mode.rs:27-35,214-237` 및 Runtime 테스트의 scene table `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/scene.rs:10-50`와 frame assertions `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/assertions.rs:50-67`는 min 50, max 80 root/text 높이를 각각 기대한다. | min/max 축을 분리해 관찰한다. |
| 11 | 이미지 natural size가 텍스트 intrinsic 값이나 row stretch에 의해 변하는가 | Runtime 준비 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/prepare.rs:100-150`는 고정 PNG를 decode하고 24×16 metadata를 확인한다. Style→Layout 테스트 `orthogonal_writing_mode.rs:187-201,268-279`도 natural size와 replaced source를 확인한다. | 이미지의 natural replaced input과 최종 프레임을 별도로 검사한다. |
| 12 | DOM source order를 reversal에서 grid column 위치와 혼동하는가 | Runtime 테스트 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/assertions.rs:72-88`은 item input의 `source_order`와 node를 비교한다. FFI scene 생성 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/orthogonal_direct_text_image_intrinsic_rows.rs:82-109,174-189`은 DOM 생성 순서를 뒤집되 아이템 ID·column을 타입별로 고정한다. | source order와 grid placement가 서로 독립적으로 검사된다. |
| 13 | 다른 ID, profile, 부모 또는 직접 자식 구성이 orthogonal scene으로 오인되는가 | Runtime 준비 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/prepare.rs:31-71`과 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/scene.rs:124-129`는 allowlist ID, profile, root 소유권, 두 direct child, 순서, ID, `data-node`를 확인한다. 알려지지 않은 `c114-orthogonal-…-grid`는 오류로 반환한다. | 등록 장면 외의 같은 이름 공간 변형을 제한한다. |
| 14 | 텍스트 하위 트리나 이미지 resource가 고정 scene 계약과 달라도 계속되는가 | 같은 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/prepare.rs:74-150`는 한 개의 정확한 직접 텍스트 leaf, 고정 Ahem 문자열, leaf image와 URL, PNG decode, natural dimensions, Ahem 등록 실패를 검사한다. | 잘못된 content/resource는 고정된 입력으로 대체하지 않고 오류로 처리한다. |
| 15 | Runtime root/item의 author style이 장면 계약을 벗어나도 예상 프레임으로 통과하는가 | Runtime validator `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/style.rs:13-197`는 box sizing, dimensions, gap, border, Grid tracks/placement, writing style, text/font 속성을 확인한다. | 현재 fixture에서 지원하는 스타일 경계를 구체적으로 제한한다. |
| 16 | 출력 frame이 누락·중복되거나 stale revision의 intrinsic 결과와 섞이는가 | Runtime validator `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/validate.rs:64-107,127-145` 및 frame helper `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/helpers.rs:5-28`는 root/text/image frame과 item frame, host/style/environment/intrinsic revision을 검사한다. 테스트 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/assertions.rs:319-380`는 Runtime 오류 때 원본 measurements·item inputs·traces Debug를 함께 생성한다. | 기대 노드별 frame과 revision 결속을 검증한다. 실제 오류 출력은 실행하지 않았다. |
| 17 | 같은 node의 일반 `Text` measurement 때문에 필요한 `OrthogonalText` provenance가 사라지거나 query 중복이 잘못 거부되는가 | Runtime validator `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/validate.rs:210-245`은 일반 Text 측정은 허용하면서 orthogonal 측정이 최소 하나 있고 해당 결과 전부가 기대 logical 값·writing mode·font revision과 맞는지 확인한다. Runtime 테스트 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/assertions.rs:140-184`도 같은 규칙이다. | 일반/orthogonal 측정의 공존을 허용한다. 측정 중복 수는 고정하지 않는다. |
| 18 | 열 sizing보다 row sizing이 먼저 오거나, 허용된 Min/Max query 중 선택된 kind가 누락되는가 | Runtime trace validator `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/validate.rs:250-351`는 마지막 Columns와 첫 Rows 순서, 최종 track 크기, text/image별 유효 Min/Max 값, span 1, 장면 선택 kind 양쪽을 확인한다. 테스트 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/assertions.rs:189-251`은 같은 selected-kind 규칙을 사용한다. | 중복 query와 반대 kind의 유효 항목은 허용하면서 잘못된 item/value/kind는 거부한다. |
| 19 | flexible-track 여부, JSON provenance 또는 unlisted FFI scene가 trace 경계에서 누락되는가 | FFI JSON `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs:63-120,263-287`은 논리 source, writing mode, font revision, span 및 `crossesFlexibleTrack`를 serialize한다. FFI dispatcher 테스트 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs:2120-2135`는 네 장면과 unlisted scene을 구분한다. Runtime feature는 `crates/spinon-runtime/Cargo.toml:12-16`, FFI fixture 모듈은 `crates/spinon-ffi/src/runtime_gpu.rs:29-45`에서 opt-in 된다. | source gate와 JSON 필드가 있다. 기본 artifact에서 바이너리 심볼/payload가 제외되는지는 빌드를 금지했으므로 확인하지 않았다. |
| 20 | Android/iOS가 JSON 입력·frames·row trace를 서로 다른 기준으로 허용하는가 | Android `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java:4669-4680,4765-4808,4926-4975`는 scene allowlist, source order, logical source, frame, selected Min/Max를 확인한다. iOS `platforms/ios/Sources/C0410RuntimeGpuDemo.swift:1682-1730,1733-1757,3102-3198`은 같은 항목과 contribution `crossesFlexibleTrack == false`까지 확인한다. 현재 Android도 `:4955-4958`에서 해당 flag를 false로 요구한다. | 코드상 네 장면과 주요 trace 경계가 연결되어 있다. 두 앱 adapter의 실행 결과는 이번 범위 밖이며 주장하지 않는다. |

## 발견 사항과 현재 상태

### F1. Runtime trace validator의 flexible-track flag 검사 누락

초기 검토 시 `validate_track_trace`는 row contribution에서 item/kind/value/span과 선택된 Min/Max kind만 검사하고 `crosses_flexible_track`를 보지 않았다. 이 장면의 row는 고정 min/max-content track이므로 `true`로 바뀐 trace도 Runtime validator가 받아들일 수 있었다. FFI JSON은 이미 해당 값을 노출했고 플랫폼은 이를 구별할 수 있었다.

검토 중 현재 작업 트리에는 Runtime validator `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/validate.rs:295-329`에서 flag가 true인 경우 오류를 반환하는 검사가 추가됐다. Runtime 테스트 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/assertions.rs:218-240`에도 모든 row contribution에서 false를 요구하는 assertion을 추가했다. Android `C0410RuntimeGpuDemo.java:4955-4958` 및 iOS `C0410RuntimeGpuDemo.swift:3162-3164`도 false를 요구한다. 최초 정적 검토 시점에는 실행하지 않았으며, 후속 Runtime suite 및 simulator 결과는 아래 실행 follow-up을 참조한다.

### F2. font registry revision을 Runtime validator에서 고정 값과 대조하지 않음

새 Runtime engine은 기본 revision에서 시작해 Ahem font를 한 번 등록한다(`crates/spinon-text-layout/src/engine/fonts.rs:42-74`). 따라서 이 장면의 기대 revision은 1이다. Android `C0410RuntimeGpuDemo.java:4835-4846`, iOS `C0410RuntimeGpuDemo.swift:1706-1715`, Runtime validator `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/validate.rs:150-176,210-238`는 모두 source revision을 1로 고정한다. Runtime measurement의 revision도 item source revision과 일치해야 하며 테스트 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows/assertions.rs:90-139,140-184`는 기대 source와 revision 1을 확인한다.

수정은 코드와 assertion에서 확인했다. 최초 정적 검토 시점의 실행 대기는 후속 Runtime suite 및 simulator 검증으로 해소했다.

### F3. Style-only track-sizing entry가 writing-mode를 layout 입력인 것처럼 받아들임

`runtime_block_flex_grid_track_sizing_author_properties()`가 `writing-mode`를 허용하지만, 공통 projection은 text profile이 있을 때에만 orthogonal computed style과 paragraph input을 대조했다. 그래서 public style-only `compute_runtime_block_flex_grid_track_sizing_style_layout` 경로는 text 입력 없이 `vertical-rl`을 cascade한 뒤 Taffy의 물리축 projection으로 전달할 수 있었다. 회귀 테스트 `crates/spinon-style-to-layout/src/c11_4_track_sizing_areas_tests/orthogonal_writing_mode.rs::style_only_track_sizing_rejects_vertical_writing_without_text_input`은 해당 entry를 통해 `UnsupportedComputedValue { property: "writing-mode", value: "vertical-rl" }`를 기대한다.

공통 projection의 C11.4/no-text-profile 분기에서 비-horizontal writing mode를 같은 `UnsupportedComputedValue`로 거부한다. ParagraphStyle이 있는 text-enabled path는 기존 exact orthogonal validation을 유지한다. Style→Layout feature suite 261/261과 feature/default Clippy가 수정 후 통과했다.

## 실행 범위

- 정적 코드와 테스트 assertion만 확인했다.
- 요청에 따라 테스트, build, simulator 및 device 명령은 실행하지 않았다.
- 따라서 네 장면의 실행 통과, Android/iOS 시각 결과, 기본 artifact의 fixture payload 부재를 확인했다고 주장하지 않는다.

## 후속 실행 및 수정 결과 · 2026-10-25

위 표와 정적 검토 범위는 작성 당시 실행 전 상태를 나타낸다. 이후 F1의 `crosses_flexible_track=false` 검사와 F2의 `font_registry_revision=1` 검사를 Runtime validator/test에 반영했고, F3의 style-only vertical writing mode 오수락을 공통 projection에서 거부하도록 수정했다. Runtime feature suite는 227 passed·3 ignored, 0 failed였고 Style→Layout feature suite는 261 passed·0 failed였다. 관련 focused test, platform matrix와 archive scan의 최종 결과는 [통합 evidence README](./README.md)에 기록했다.

후속 Clippy에서 FFI orthogonal JSON helper의 인자 수, test module 뒤에 둔 test-only helper, 기본 feature에서만 드러나는 조건부 unused import를 발견해 named fields, 선언 순서와 feature-gated import로 수정했다. Feature-enabled 관련 package Clippy와 default workspace all-target Clippy 모두 `-D warnings`로 통과했다. 이 코드는 실행 동작을 바꾸지 않는 lint/구조 수정이다.

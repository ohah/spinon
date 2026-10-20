# C11.4 auto-repeat typed implicit track sizing PR #391 변경분 30개 실패 관점 검토

**검토 대상:** PR #391의 layout/style/runtime 변경, Android·iOS fixture adapter, capture/assertion 코드, 최신 simulator evidence 및 상태 대장·명세 인덱스 갱신.
**검토 기준:** 각 행은 별도 실패 조건을 정해 실제 diff와 focused/workspace/simulator 근거를 대조했다. 계획 검토 30개와 구현 검토 20개는 재사용하지 않았다.
**결과:** 30개 관점을 확인했다. PR #391은 Draft이며, 구현 범위와 C11.4 미완료 상태가 문서에 유지돼 있다.

| # | 적대적 실패 경로 | 변경 diff에서 확인한 결과 |
| ---: | --- | --- |
| 1 | 계획·내부 계약·상태 대장이 typed list slice를 C11.4 전체 지원 완료로 승격한다. | 계획, 계약 0059, `spec/STATUS.md`, 내부 인덱스가 범위를 `RuntimeBlockFlexGridTrackSizingV1` 및 현재 typed forms로 한정한다. C11.4 checkbox와 완료 수는 그대로다. |
| 2 | auto-repeat 확장이 C11.2.2·C11.2.3·TemplateGrid 등 기존 profile의 허용 규칙을 바꾼다. | profile-specific validation 경계를 확인했다. layout regression `typed_auto_track_lists_remain_rejected_outside_track_sizing_profile`은 rows와 columns 양쪽 거부를 각각 확인한다. |
| 3 | 이 변경이 parser 또는 repeat/line-name 문법까지 새로 허용한다. | 변경은 기존 computed `GridTrackSize` 전달만 확장한다. 새 parser, 복수 repeat, line name, arbitrary grammar는 변경하지 않았다고 계약·계획에 기록했다. |
| 4 | 빈 목록·상한 초과 목록이 track converter까지 도달해 빈 grid 또는 과도한 할당을 만든다. | converter 전 공용 `validate_tracks`가 비어 있지 않은 목록 및 길이 상한을 검증한다. 관련 workspace와 focused layout 테스트가 통과했다. |
| 5 | 음수·비유한 길이는 수용되거나 유효한 `0px`는 거부된다. | 공용 유효성 검사와 zero-size regression을 확인했다. workspace 테스트 1,182/1,182 통과다. |
| 6 | percentage·fraction·auto·intrinsic·minmax·fit-content list 중 일부가 빠지거나 서로 다른 축 변환을 탄다. | `auto_repeat_implicit_lists_convert_supported_track_sizing_functions`가 양 축의 기존 converter 경로를 확인한다. typed 14-case simulator matrix도 통과했다. |
| 7 | scalar `Auto` 또는 scalar `LengthPx`가 list conversion 변경으로 의미를 바꾼다. | scalar와 list match arm을 분리해 확인했고 기존 scalar implicit-track regression을 유지한다. |
| 8 | `grid-auto-columns` Calc가 rows math identity 또는 그 반대를 사용한다. | `grid_math_property`는 columns/rows axis identity를 유지하고, 25px·23px 비대칭 기대값 테스트가 통과했다. |
| 9 | `minmax()`·`fit-content()` 내부 Calc handle이 axis/node 검사를 우회한다. | 중첩 참조도 공통 validator로 보내며 `auto_repeat_nested_calc_tracks_validate_axis_and_node_binding`을 실행했다. |
| 10 | 존재하지 않거나 다른 node/property에 결합된 Calc ID가 panic·기본값 변환으로 숨는다. | missing handle은 `MissingCssMath`, node/property 불일치는 `CssMathBindingMismatch`를 요구하는 회귀가 통과했다. |
| 11 | 오류가 발생한 layout이 부분 `LayoutOutput` 또는 일부 scene submission을 반환한다. | converter와 기존 layout error propagation의 `Result` 경계를 확인했다. 단위/fixture 검증은 오류를 성공 프레임으로 정규화하지 않는다. |
| 12 | percentage/Calc basis가 한 축 또는 auto-repeat count 계산과 섞여 used track이 바뀐다. | axis별 scene, trace 및 교차 비교를 확인했다. 1,019 used track 값이 Android와 iOS에서 모두 일치했다. |
| 13 | negative line이 auto-repeat expansion 전의 explicit track 수를 기준 삼아 implicit extent를 한 칸 틀린다. | expanded-repeat-count negative-line 테스트와 138-scene 각 ID의 frame/used-track assertion이 통과했다. |
| 14 | explicit grid 앞쪽의 implicit list가 authored order 순환 방향과 반대로 배치된다. | 양 축 leading cycle regression 및 `typed-fixed-cycle`·mixed-cycle trace에서 authored index 방향을 확인했다. |
| 15 | explicit grid 뒤쪽의 list cycle이 leading cycle과 같은 reverse rule을 잘못 사용한다. | positive/trailing cycle 기대값과 `typed-fixed-cycle-three-leading-trailing`의 frame/track 결과를 확인했다. |
| 16 | columns와 rows가 repeat count, auto-track 길이 또는 Calc ID 저장을 공유한다. | axis별 비대칭 typed scene과 two-axis repeat case를 확인했다. cross-platform JSON의 frame·track 비교 최대 차이는 0 CSS px다. |
| 17 | `auto-fill` repeat count가 adjacent track·definite basis·gutter 처리 때문에 기존 subset에서 달라진다. | 고정 neighbor, 8px gutter 및 repeat boundary scene을 기존 124 IDs에 포함해 양 플랫폼 138/138 통과했다. |
| 18 | `auto-fit`이 실제 item이 차지한 repeated track까지 접는다. | occupied repeat의 collapse flags와 node frames를 simulator trace 및 fixture expectation과 대조했다. |
| 19 | `auto-fit`이 빈 repeated track을 유지하거나 repeat 수 자체를 줄인다. | empty scene은 pre-collapse repeat count와 collapsed used track을 분리해 검사한다. 양 플랫폼 scene assertion이 통과했다. |
| 20 | 접힌 track 경계에 gutter가 남거나 인접한 non-collapsed gutter까지 제거된다. | `typed-auto-fit-gutter`의 axis trace, 후속 item frame, 5-sample 표면 표본을 확인했다. |
| 21 | `min-height:0` 예외가 다른 typed text case의 automatic minimum을 꺼 버린다. | 고정 최소값이 필요한 두 scene만 allowlist하고 나머지는 auto minimum 경로를 유지한다. runtime fixture package 209 passed다. |
| 22 | JS fixture의 `textContent`가 HostDocument text node를 만들지 않아 intrinsic contribution이 빈 값이 된다. | builder가 실제 `createTextNode`를 붙인다. Ahem intrinsic input trace와 typed text scenes가 확인됐다. |
| 23 | fixture에서 Ahem 로드 실패 후 system font가 사용돼 simulator 간 font metrics가 달라진다. | system font 없는 text engine에 고정 Ahem asset을 등록하며 실패는 구체 runtime error로 반환한다. |
| 24 | 다른 root/item/tag/text/style가 text allowlist를 통과해 고정 intrinsic 값을 잘못 재사용한다. | scene ID, element identity, parent/child order, literal text, font 및 box/text computed style을 검사해 다른 형태를 fail-closed 한다. |
| 25 | Rust FFI·Android·iOS scene registry가 서로 달라 일부 scene이 검증되지 않는다. | 세 registry의 typed scene 14개를 대조했다. FFI registry test 1/1 및 양 플랫폼 각각 138 scene이 통과했다. |
| 26 | Android JNI 32 KiB result buffer가 전체 trace를 잘라 실제 오류나 frame을 숨긴다. | JNI output을 heap-backed 1 MiB vector로 늘렸다. Android full matrix에서 전체 trace chunk·node frames·draw marker를 확인했다. |
| 27 | Android 오류 로그 chunking이 긴 report를 빠뜨리거나 다른 run/scene에 붙인다. | chunk는 run ID·scene ID·part 번호를 남기며, 성공/실패 report와 반환 byte array는 유지한다. simulator logs에서 scene별 분리를 확인했다. |
| 28 | iOS ObjC++/Swift 경로가 큰 report를 자르거나 layout 성공 뒤 실제 node frame을 출력하지 않는다. | 1 MiB heap output과 NUL 종료 report 변환, Swift node-frame logger를 확인했다. iOS 138/138 및 1,659 trace chunk가 통과했다. |
| 29 | capture validator가 선택된 scene만 검사하거나 float 문자열 exact match·프레임 누락을 허용한다. | runScenes 전체 반복, node ID/order/count 검사, 1/64 CSS px tolerance, draw/present assertion 및 scene별 screenshot을 확인했다. |
| 30 | 교차 비교·화면·계약 인덱스가 실행 hash/범위 한계를 누락하거나 border/compositor·실기기 보증으로 과장한다. | README는 APK/app hash, 24/24 표본, 두 최신 run link와 sample 한계를 기록한다. Android software renderer, iOS compositor 전 readback, border stroke 미검증을 분명히 한다. |

## 실행 근거

- `mise exec -- cargo test --locked --workspace --quiet` — 1,182 passed, 0 failed, 4 ignored.
- `mise exec -- cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test` — 209 passed, 0 failed, 3 ignored.
- Workspace all-target Clippy, rustfmt, fixture-module-boundary 6/6, capture/compare script syntax, `git diff --check` 통과.
- Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 각각 138/138 scene, 24/24 surface sample. 비교는 frame scalar 960개·track size 1,019개 최대 차이 0 CSS px.
- docs build는 저장소의 기존 dead-link 묶음으로 실패한다. 추가한 plan/evidence 링크는 GitHub blob/tree 또는 현재 유효한 상대 문서 경로로 연결했다.

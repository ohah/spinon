# C11.4 양 축 auto-repeat + template areas 구현/runtime 실패 관점 20개

이 검토는 계획 검토와 별도로 최종 변경 코드·고정 fixture·Android/iOS 실행 결과를 대상으로 했다. 각 항목은 다른 실패 경로 또는 계약 경계를 확인한다. 새로 남은 결함은 찾지 못했다. 실행 중 발견한 layout validator의 양축 거부, 빈 item 목록의 node-frame 구분자, longhand CSS 이름 생성 오류는 최종 matrix 전에 수정했다.

| # | 실패 관점 | 확인 결과와 근거 |
| --- | --- | --- |
| 1 | 잘못된 computed-style profile에 양축 repeat와 areas가 새어 들어가는가 | cascade는 `RuntimeBlockFlexGridTrackSizingV1`만 허용하고 `RuntimeBlockFlexGridTemplateV1`은 거부한다. `runtime_grid_tests/template_repeat.rs`의 성공·실패 회귀. |
| 2 | typed projection이 cascade보다 넓은 profile을 허용하는가 | target profile은 두 축과 area metadata를 보존하고 `RuntimeBlockFlexGridTemplateV1`은 거부한다. `grid_values/tests.rs`. |
| 3 | 둘 다 repeat인 경우 validator의 XOR 조건이 유효 입력을 거부하는가 | `grid.rs` 조건을 “적어도 한 축”으로 바꿨다. 양축 metadata 회귀와 `TaffyLayoutEngine` 3×3 area frame 회귀가 통과했다. |
| 4 | repeat이 하나도 없는데 area metadata만 주면 통과하는가 | `validate_template_areas`가 repeat axis가 없으면 `InvalidGridTrackList`를 반환한다. `grid_template_areas_tests.rs`. |
| 5 | 양축 repeat 경계가 서로 바뀌거나 한 축의 범위 오류를 놓치는가 | columns와 rows를 독립적으로 `validate_auto_repeat_axis`에 전달한다. 두 축의 repeat/area 조합은 34-scene matrix에 포함된다. |
| 6 | 0 track 또는 최대 track 수 초과 차원이 Taffy에 도달하는가 | 0 dimension과 `GRID_MAX_TRACKS` 초과를 validator가 거부한다. `grid_template_areas_tests.rs`. |
| 7 | repeat 없는 축의 template-area dimension이 authored track 수와 불일치해도 허용되는가 | 해당 축 dimension과 track list 길이가 다르면 거부한다. fixed-axis mismatch 회귀가 있다. |
| 8 | 빈/NUL/중복 area 이름이나 비어 있는 rectangle이 named-line alias를 모호하게 만드는가 | validator가 빈 이름, NUL, 중복 이름, 빈 row/column range와 상한 밖 bounds를 거부한다. |
| 9 | 잘못된 metadata 크기 또는 메모리 예약 실패가 부분 상태로 통과하는가 | area count 곱셈·누적 bytes를 checked arithmetic으로 제한하고 이름 집합 예약은 `try_reserve` 실패를 오류로 반환한다. 1 MiB 초과 회귀가 있다. |
| 10 | 0-based half-open area bounds가 Taffy line 변환에서 한 칸 어긋나는가 | 3×3 양축 확장 layout 회귀와 `area-added-both`가 item `(44,40,8,8)` 및 columns `[20,16,16]`·rows `[20,12,12]`를 확인한다. |
| 11 | area로 추가된 explicit track이 지정한 `grid-auto-*` 크기를 잃는가 | fixture의 fixed 16px column·12px row를 두 축에서 각각 확인한다. `area-added-columns`, `area-added-rows`, `area-added-both`. |
| 12 | area가 repeat count보다 작은 경우 authored repeat track을 잘라내는가 | `area-short` scene에서 4 columns·3 rows가 유지되고 area item은 `(24,0)`이다. Android/iOS 결과 JSON. |
| 13 | columns만 area dimension으로 늘 때 row placement·scroll extent가 달라지는가 | `area-added-columns`는 columns `[20,16,16]`, rows `[20,20,20]`, item x=44와 scroll width=52를 확인한다. |
| 14 | rows만 늘 때 column placement·scroll extent가 달라지는가 | `area-added-rows`는 columns `[20,20,20]`, rows `[20,12,12]`, item y=40과 scroll height=48을 확인한다. |
| 15 | 양축 확장 item이 root 경계에서 clip되거나 overflow extent가 누락되는가 | root 40×40, item `(44,40,8,8)`, scroll 52×48을 양 플랫폼에서 확인했다. |
| 16 | `auto-fit`/`auto-fill` collapse가 다른 축의 repeat mode를 덮어쓰는가 | `fill-fit`과 `fit-fill`이 각각 axis별 track·collapse·frame 기대값을 만족한다. |
| 17 | area에 item이 없어도 `auto-fit` track이 공간을 차지하는가 | `fit-fit-empty`는 item 0개, 양축 used tracks 모두 0, scroll 100×76이다. 빈 목록의 serialization도 이 scene으로 검사했다. |
| 18 | spanning item이 차지하는 `auto-fit` track까지 잘못 collapse되는가 | `fit-fit-span`은 item 44×44와 각 축의 두 occupied 20px tracks, 나머지 collapsed tracks를 확인한다. |
| 19 | `gridArea` shorthand, `gridColumn`/`grid-row` longhand가 잘못된 CSS 속성 이름으로 생성되는가 | property 변환 분기를 보완했고 `area-longhand` runtime scene과 Rust fixture source 검사에 포함했다. |
| 20 | geometry가 맞아도 양 플랫폼에서 root 밖 item/host surface의 색이 다르거나 draw가 누락되는가 | Android PixelCopy와 iOS 제출 전 Metal drawable 표본이 각각 4/4 exact이며 두 최종 result JSON은 모두 34/34 `drawPresented=true`다. iOS compositor 이후·실기기·hardware GPU 결과로 확대하지 않는다. |

## 근거

- Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator 최종 matrix, screenshots, build/test 한계: [runtime README](README.md)
- Chrome 156 oracle와 예상값 판정: [precomparison README](../c11-4-two-axis-template-areas-precomparison-2026-10-23/README.md)
- 구현 diff의 PR 전 30개 검토는 같은 evidence 폴더의 `pr-diff-review-30.md`에 별도로 기록한다.

## 남은 경계

Current-source default/product FFI archive negative control과 최종 APK/IPA fixture exclusion은 디스크 여유 문제로 실행하지 않았다. WPT, 실기기, Android hardware GPU, iOS compositor 이후 surface, border stroke, 일반 CSS admission 및 전체 C11.4는 검증하지 않았다. 이 제한은 성공 판정에 포함하지 않는다.

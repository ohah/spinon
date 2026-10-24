# C11.4 auto-repeat `calc()` neighbor 구현/runtime 실패 관점 검토 20개

계획 검토 20개와 별도로 현재 cascade, typed projection, layout, Taffy adapter, runtime 진단, fixture 경계와 기록된 simulator 실행을 대조했다. 계획 검토나 뒤이을 PR diff 검토 항목을 재사용하지 않는다.

| # | 실패 관점 | 소스·증거 대조와 판정 |
| --- | --- | --- |
| 1 | Cascade에서 계산식 neighbor가 projection 전에 사라진다. | `computed_auto_repeat_tracks`가 prefix/suffix에서 representable track을 보존하고 제한은 projection으로 넘긴다. Cascade test에서 두 항 순서와 signed length를 typed AST로 확인한다. 통과. |
| 2 | auto-repeat range 오류가 이웃 검사에서 slice panic을 낸다. | Projection은 `start >= end` 및 `end > tracks.len()`을 slicing 전에 거부한다. `track_sizing_projection_rejects_invalid_auto_repeat_ranges_before_slicing`을 대조했다. 통과. |
| 3 | 반복 fragment 내부 track까지 neighbor whitelist로 오판한다. | 검사는 `tracks[..start]`와 `tracks[end..]`만 순회한다. 반복 fragment의 fixed-minimum 검증은 기존 layout 경로에 별도로 남아 있다. 통과. |
| 4 | 한쪽 축 calc가 다른 축의 track list에 붙는다. | Projector는 전체 cascade에서 공유되는 calc ID를 할당하고 각 value에 node/property를 보관한다. 축별 projection test와 layout의 property 대조가 이를 막는다. 통과. |
| 5 | Sum의 항 순서가 바뀌면 유효 calc가 거부된다. | Projection과 layout은 `LengthPx`/`Percentage` 두 순서를 모두 처리한다. 계산·projection test에서 순서를 바꾸어 검사한다. 통과. |
| 6 | 다른 수학식 AST를 우연히 2-term calc로 허용한다. | 지원 AST는 `Sum` 정확히 두 항이며 signed `LengthPx` 하나와 `Percentage` 하나뿐이다. `MinMax`, fraction, 중첩 `Clamp`, 추가 항을 projection에서 거부한다. 통과. |
| 7 | 음수 percentage를 음수 length처럼 처리한다. | Percentage는 유한하고 0 이상이어야 한다. CSS-valid negative-percentage control은 node/property가 있는 unsupported 결과로 남는다. 통과. |
| 8 | 비유한 coefficient가 계산 count에 전달된다. | Projection에서 양 계수의 finite 여부를 확인하고 layout에서 계산식의 raw 결과와 resolved 값도 finite/representable인지 검사한다. NaN·과대 percentage negative test가 있다. 통과. |
| 9 | 동일 calc ID 중복 또는 다른 node/property 식을 잘못 재사용한다. | 입력 전역 중복 ID 검증과 neighbor의 unique match, node/property 확인이 모두 있다. missing ID, wrong node, wrong property 경로가 별도 assertion으로 거부된다. 통과. |
| 10 | indefinite percentage basis를 임의의 0 또는 viewport로 대체한다. | Neighbor에 percentage가 있으면 definite percentage basis와 양수 content axis가 필요하다. indefinite columns control은 layout 결과를 거부한다. 통과. |
| 11 | columns에 height, rows에 width를 사용한다. | 축 property로 width/height, min/max, padding, border를 선택한다. 양축 결과 test와 Android/iOS scene matrix가 각각의 track 배열을 확인한다. 통과. |
| 12 | border-box의 declared size를 content basis로 사용한다. | Border-box는 고정 padding과 border를 뺀 content size를 계산하고 결과가 양수·유한한지 검사한다. `col-fill-border-box`의 기대 basis 116px 및 simulator scene을 대조했다. 통과. |
| 13 | 최종 계산값이 음수이면 CSS range clamp 대신 선언 전체를 거부한다. | 제한 shape 안에서는 `LayoutCssMathValue::resolve`가 grid-template의 nonnegative range clamp를 적용한다. `-20px + 5%`는 0px track이고, intermediate signed length는 허용된다. 통과. |
| 14 | 큰 개별 neighbor 또는 여러 neighbor 합이 f32 범위를 넘는다. | Percentage/calc의 resolve 전 표현 가능성과 prefix+suffix aggregate를 각각 확인한다. 비표현식과 `f32::MAX` fixed neighbor 합 테스트가 거부된다. 통과. |
| 15 | calc 결과를 auto-repeat count의 non-repeating 사용량에 포함하지 않거나 gap 계산과 분리한다. | Taffy explicit-grid count 경로가 definite non-repeating track, repeated fragment, gap을 같은 count 산식에 넣고 비유한 중간값은 보수적 1회로 처리한다. 고정 overflow scene의 Chrome·layout·simulator 기대값을 대조했다. 통과. |
| 16 | auto-fit에서 점유된 repeat까지 collapse하거나 빈 repeat를 남긴다. | occupied-first와 all-empty scene에서 각 track의 collapse flag·used size를 확인한다. Android/iOS 14-scene matrix에서 draw/present 결과도 통과했다. 통과. |
| 17 | 연속 collapsed track의 경계 gutter를 모두 제거하거나 중복시켜 suffix 위치를 옮긴다. | Taffy 변경은 collapsed track 뒤 gutter를 접고 run 앞의 shared boundary gap을 남긴다. [CSS Grid §11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters) 및 `auto_fit_calc_neighbors_keep_the_collapsed_run_boundary_gutter`의 suffix item x=26을 대조했다. 통과. |
| 18 | 기대 rejection이 generic error로 축약되거나 실패 뒤 부분 frame을 게시한다. | `InvalidGridTrackList`는 runtime `invalid_grid_track_list`와 node/property를 보존한다. 세 rejection control은 정확한 diagnostic과 frame 부재를 Android/iOS에서 각각 확인했다. 통과. |
| 19 | 임의 fixture 입력이 실행되거나 opt-in scene 목록과 기대값이 어긋난다. | 14개 정적 scene ID allowlist, 미등록 ID 거부, dispatcher 연결 및 scene/track/collapse 수 검사를 대조했다. Catalog test가 수·중복·미등록 ID를 검사한다. 통과. |
| 20 | fixture 지원을 제품 지원·공식 완료로 과장한다. | product release FFI archive는 0/14 scene ID와 fixture symbol 부재, fixture-test positive control은 14/14 및 symbol 존재다. 이는 macOS ARM64 archive 한정이다. APK/IPA 제외, 일반 CSS admission, WPT, 실기기·hardware GPU와 iOS compositor 이후 pixel은 입증하지 않는다. 공식 C11.4는 미완료이며 완료 수 72/213을 유지한다. 통과. |

## 근거와 결론

- 구현 소스: `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs`, `crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs`, `crates/spinon-layout/src/grid.rs`, `crates/spinon-layout/src/css_math.rs`, `vendor/taffy/src/compute/grid/explicit_grid.rs`, `crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs`.
- 기존 focused test 기록: layout 51/51, style 19/19, style-to-layout 15/15, runtime diagnostic regression 1/1. [검증된 Android/iOS simulator 실행과 입력 hash](./c11-4-auto-repeat-calc-neighbors-projection-pass03-2026-10-24/README.md) 및 [FFI archive feature 경계](./c11-4-auto-repeat-calc-neighbors-artifact-2026-10-24/README.md)를 현재 source/status와 대조했다.
- 20개 관점에서 병합을 막는 구현 결함은 찾지 못했다. 이 검토 시점에는 새 테스트 실행이나 PR diff 검토 30개를 대신하지 않았으며, 이후 [PR #583 diff 30개 검토](./c11-4-auto-repeat-calc-neighbors-pr-diff-review-30-2026-10-24.md)를 별도로 기록했다. C11.4 상위 완료와 공식 집계 승격은 남아 있는 CSS 지원·WPT·최종 앱 artifact 등 범위가 해결되기 전까지 하지 않는다.

# C11.4 contribution input 구현 적대 검토 20개

**대상:** `LayoutGridItemIntrinsicInput::contribution_input()`, 축별 `outer_size()`, 연결 테스트와 내부 계약 문서.
**범위 한계:** Rust layout 출력의 다음 단계 입력을 추가한다. Grid track sizing, 앱 runtime, 공개 CSS 지원은 구현하지 않는다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | Text min-content 폭을 callback의 마지막 응답에서 잘못 가져오는가 | raw source의 Parley `min_content()`를 쓴다. definite callback과 독립된 기존 fixture가 70 CSS px를 확인한다. |
| 2 | Text max-content 폭을 min-content와 혼동하는가 | `max_content()`를 별도로 복사하며 fixture가 100 CSS px를 확인한다. |
| 3 | Text 높이 후보가 폭 후보에 덮어쓰이는가 | `min_content_height()`와 `max_content_height()`를 높이 축에 분리한다. fixture는 20·10 CSS px를 확인한다. |
| 4 | Text font registry revision이 후보 변환에서 유실되는가 | contribution input이 원래 source를 함께 복사해 font registry identity/revision을 보존한다. |
| 5 | Replaced natural width·height를 서로 바꾸거나 한 축만 사용하는가 | natural width와 height를 각각 해당 축의 min/max 후보로 보낸다. `24×16` test가 두 축을 확인한다. |
| 6 | Replaced metadata가 빠졌을 때 임의 기본 크기로 대체하는가 | `Option`을 그대로 복사해 알 수 없는 자연 축을 `None`으로 남긴다. |
| 7 | Content-box 후보에 padding/border를 미리 섞어 이중 적용하는가 | 후보는 content-box로 유지하고 edge는 별도 필드에 둔다. 고정 edge 변환은 한 번만 합산한다. |
| 8 | `box-sizing` 값이 candidate에 적용되며 원본 style과 달라지는가 | 전체 `item_style`을 보존한다. content-box text test가 전체 style equality를 확인한다. |
| 9 | `min-width`·`max-width`를 현재 Grid profile이 지원한다고 과장하는가 | typed style은 입력에 보존하지만 계산은 하지 않는다. `min-width:90px` 실행은 기존 `UnsupportedGridItemConstraint`로 거부되어 검토 후 test 입력에서 제외했다. |
| 10 | CSS preferred width/height를 intrinsic max-content로 대체하는가 | `preferred_size`는 style의 typed width/height를 그대로 복사한다. image test는 `min-content` 값도 보존됨을 확인한다. |
| 11 | aspect ratio 축 전달이 이미 구현된 것으로 오인되는가 | 전체 style과 replaced natural metadata에는 보존되지만 이 변경은 비율 전파를 계산하지 않는다. 계획·계약에 미구현으로 남겼다. |
| 12 | overflow 또는 `order` 같은 후속 입력을 좁은 필드 투영에서 버리는가 | 전체 `item_style` 복사로 두 입력과 다른 미사용 style 값도 보존한다. |
| 13 | width 시작·끝 edge가 위·아래 edge로 매핑되는가 | width는 left/right, height는 top/bottom으로 매핑한다. text fixture의 좌우 padding/border를 검증한다. |
| 14 | auto margin을 외곽 크기에 더해 무한·미정 값으로 만드는가 | `outer_size()`에서 auto margin을 0으로 처리하며 단위 검증으로 확인한다. |
| 15 | 고정 음수 margin을 padding처럼 금지하는가 | 유한한 고정 margin은 음수도 보존·합산한다. 음수의 CSS contribution 후속 보정은 이 단계에서 하지 않는다. |
| 16 | percentage/calc margin 또는 padding을 근거 없이 0이나 px로 취급하는가 | 고정 basis가 없는 `outer_size()` 호출은 `UnsupportedIntrinsicSizing`으로 거부한다. percentage 경로를 test했다. |
| 17 | 음수/비유한 padding이나 border를 유효한 길이로 받는가 | padding과 border는 0 이상·유한 값만 합산하고 나머지는 오류로 처리한다. |
| 18 | 비유한 content 후보 또는 합산 overflow가 결과에 들어가는가 | 입력과 최종 합을 finite 검사하고 `InvalidIntrinsicSize`로 실패시킨다. |
| 19 | min/max 후보의 독립 outer size를 final contribution으로 오인하는가 | helper는 outer-size만 반환한다. 음수 margin으로 max-content가 min-content보다 작을 때 적용하는 floor, preferred/min/max contribution 선택은 미구현임을 코드·문서에 구분했다. |
| 20 | 후보 projection이 기존 output에 불필요한 allocation/failure path를 추가하는가 | 후보는 호출 시 값으로 생성되므로 layout-time allocation이 없다. raw input vector의 기존 fallible reserve와 recovery test는 그대로 유지한다. |

## 발견 사항

- 검토 중 `min-width:90px`를 붙인 Grid text test가 현재 validator의 `UnsupportedGridItemConstraint`에 걸렸다. 이 slice가 그 제약을 넓히지 않도록 test를 고치고 제한을 계획·상태에 명시한다.
- outer-size와 후보 입력은 CSS contribution algorithm의 일부 입력일 뿐이다. 자동 최소, sizing suggestion, aspect-ratio transfer, percentage basis, span, track stages가 남아 있어 C11.4 완료 판정에는 사용할 수 없다.

**실행 근거:** `cargo test --locked -p spinon-layout` 161 passed, `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings` 통과. Workspace Clippy는 변경하지 않은 `crates/spinon-style/src/stylo_dom/cascade/incremental/compute.rs:43`의 기존 `too_many_arguments`에서 중단했고, `cargo check --locked --workspace`는 통과했다.

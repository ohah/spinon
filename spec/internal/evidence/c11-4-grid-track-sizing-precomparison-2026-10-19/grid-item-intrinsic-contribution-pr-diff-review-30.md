# C11.4 contribution input PR 변경 검토 30개

**대상:** 이번 PR의 Rust API·테스트·계약·계획·상태·구현 검토 자료 전체 diff.
**판정:** 아래 30개 경계를 확인했다. 현 변경에서 수정이 필요한 코드 결함은 남지 않았다.

| # | 적대 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | `LayoutOutput`을 새 필드 추가로 불필요하게 깨뜨리는가 | 별도 contribution vector를 두지 않고 원래 raw input의 변환 메서드로 제공한다. 기존 output 구조는 바꾸지 않는다. |
| 2 | 기존 layout 계산에 새로운 allocation 또는 OOM failure path를 추가하는가 | `contribution_input()`과 `outer_size()`는 값과 스택 산술만 사용한다. output 수집은 기존 두 vector 그대로다. |
| 3 | 후보 변환이 raw input 자체나 font registry token을 변경하는가 | `&self`에서 새 값만 만들며 source/style 원본은 바꾸지 않는다. |
| 4 | input revision이 callback·intrinsic-size snapshot revision과 달라지는가 | 원본 `LayoutInputRevision`을 그대로 전달하며 계산 후 revision-stamping 순서는 기존 collection 뒤다. |
| 5 | 여러 Grid/container의 결과 순서가 변하는가 | 변환은 이미 정렬된 raw vector 항목별로 호출자가 수행한다. 정렬이나 필터를 새로 수행하지 않는다. |
| 6 | text min-content width가 definite callback 크기로 대체되는가 | 원본 Parley `min_content()`를 사용하고 기존 test의 70px를 확인한다. |
| 7 | text max-content width가 min-content와 같아지는가 | Parley `max_content()`를 별도 보존하고 100px를 확인한다. |
| 8 | text height 후보를 width와 혼동하는가 | min/max content height를 height 축에 따로 둔다. 20px·10px을 확인한다. |
| 9 | text shaping/font identity가 raw source projection에서 분리되는가 | source 전체를 값으로 전달해 font registry identity와 revision을 유지한다. |
| 10 | replaced natural width·height를 한 축으로 합치거나 추정값으로 만드는가 | natural width·height를 각각 같은 값의 min/max 후보로 투영한다. `24×16` 입력을 검증했다. |
| 11 | natural dimension이 없는 image에 기본값을 생성하는가 | 각 `Option`을 유지한다. source natural metadata도 결과에 남는다. |
| 12 | CSS preferred/min/max 크기 typed value가 해석 과정에서 바뀌는가 | width/height와 min/max를 `LayoutDimension` 값 그대로 복사한다. percent·calc basis 계산을 하지 않는다. |
| 13 | `box-sizing`이 삭제되거나 content 후보에 잘못 선적용되는가 | 전체 `LayoutStyle`에 보존한다. 후보는 content-box로 남고 edge 변환은 호출형 helper에 둔다. |
| 14 | `aspect-ratio`가 후보 구조에 없어 후속 계산에서 유실되는가 | 전체 `item_style`과 replaced natural source가 보존된다. 비율 전파를 지원한다고 표시하지 않는다. |
| 15 | overflow 상태가 후속 automatic minimum 경로로 전달되지 않는가 | 전체 `item_style` 복사에 `overflow_x/y`가 포함된다. automatic minimum 자체는 미구현이다. |
| 16 | CSS `order`와 원본 source order 중 하나가 사라지는가 | `source_order`와 `item_style.order`를 둘 다 유지한다. |
| 17 | width의 physical edge가 height edge와 섞이는가 | width는 left/right를 읽고 height는 top/bottom을 읽는다. text edge test가 width pair를 확인한다. |
| 18 | padding/border가 intrinsic content size에 이중 합산되는가 | 축 후보와 edge 입력이 분리된다. `outer_size()`만 fixed edge를 한 차례 합산한다. |
| 19 | auto margin이 margin-box 결과에 포함되어 무한값을 만드는가 | auto margin은 0으로 처리한다. CSS Sizing §2.2 참조와 test를 연결했다. |
| 20 | negative fixed margin이 padding처럼 잘못 거부되는가 | finite `LengthPx` margin은 음수도 합산한다. CSS max-content floor는 다음 contribution 선택 단계에 남긴다. |
| 21 | percentage/calc margin·padding을 추정 basis로 조용히 계산하는가 | 호출자 basis가 없으면 `UnsupportedIntrinsicSizing`으로 명시 거부한다. percentage test가 있다. |
| 22 | negative padding/border가 외곽 후보를 줄이는 데 사용되는가 | padding은 0 이상 fixed length만 받고 border도 유한·0 이상인지 확인한다. |
| 23 | 음수·NaN content candidate가 합산되는가 | content 후보를 유한·0 이상으로 검증한 뒤 산술한다. |
| 24 | 유한 입력 산술 overflow가 `inf` 결과로 노출되는가 | 합산 뒤 finite 검사를 하고 node/axis가 있는 `InvalidIntrinsicSize`를 반환한다. |
| 25 | natural candidate가 없는 축에서 불필요한 edge error가 발생하는가 | `None`은 바로 `None`으로 돌려보내 후보 없는 축을 계산하지 않는다. |
| 26 | raw outer size를 max-content contribution 최종값이라고 과장하는가 | helper는 outer-size만 계산한다. min/max/preferred 선택과 max-content floor는 문서에 미완료로 명시한다. |
| 27 | 새 method가 anonymous, hidden, out-of-flow, non-leaf 항목까지 Grid leaf로 만든다고 오해하는가 | method는 기존 raw item에만 적용된다. 그 입력의 수집 filter는 변경하지 않았다. |
| 28 | test의 80·110px가 Spinon Grid track 또는 앱 화면 통과라고 표시되는가 | 테스트는 Ahem content candidate와 fixed edge 합산만 확인한다. Chrome은 독립 사전 oracle이며 track sizing 미검증임을 유지한다. |
| 29 | min-width/max-width의 미지원 상태를 가리는가 | `min-width:90px`는 기존 `UnsupportedGridItemConstraint`로 거부됨을 기록하고 validator를 넓히지 않았다. |
| 30 | 상태 대장·계획이 PR을 C11.4 완료나 mobile 검증으로 승격하는가 | C11.4 미완료 유지, percentage/calc·ratio·automatic minimum·tracks·auto-repeat·Android/iOS runtime 미검증을 각각 명시한다. 앱 동작 변경이 없어 simulator 실행은 이 PR 범위에 없다. |

## 발견 및 수정

- 초기 설계는 raw source/style을 복사한 별도 `LayoutOutput` vector를 추가했다. 추가 memory와 OOM 지점만 늘리는 문제를 발견해 on-demand `contribution_input()`으로 변경했다. 기존 public Rust output shape와 layout-time allocation은 바뀌지 않는다.
- 검토 중 `min-width:90px` test가 현재 Grid validator에서 실패하는 점을 확인했다. 이 PR은 validator를 우회하지 않고 scope 및 plan/status에 남은 제약을 기록한다.

## 실행 근거

- `cargo fmt --all -- --check`
- `cargo test --locked -p spinon-layout`: 161 passed
- `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings`: 통과
- `cargo check --locked --workspace`: 통과
- Workspace Clippy: 변경하지 않은 `crates/spinon-style/src/stylo_dom/cascade/incremental/compute.rs:43`의 `too_many_arguments`에서 중단
- Android/iOS Simulator: 이 변경은 앱 runtime·paint·FFI에 연결되지 않아 실행하지 않음

참조: [CSS Box Sizing Module Level 3 §2.2](https://www.w3.org/TR/css-sizing-3/#intrinsic-contribution)는 intrinsic size contribution을 outer size 기준으로 정의하고 auto margin을 0으로 취급한다. max-content가 min-content보다 작을 때의 floor는 이 helper가 아닌 이후 contribution 단계의 책임이다.

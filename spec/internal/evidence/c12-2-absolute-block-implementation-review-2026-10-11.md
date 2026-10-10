# C12.2 Block absolute 구현 변경 실패 관점 검토

**범위:** Rust layout 계산, Stylo→layout 투영, V8/FFI와 Android·iOS 표시 경로, 기준 비교기 및 현재 지원 계약. 구현 전 계획 검토는 [별도 기록](./c12-2-absolute-block-plan-review-2026-10-11.md)에 둔다. 이 기록은 전체 CSS/WPT 적합성 선언이 아니다.

## 변경 중 발견해 수정한 결함

- iOS fixture는 처음에 CSS viewport 360×800으로 계산됐지만 UIKit의 `viewDidLayoutSubviews`가 320×240 canvas bounds를 다시 환경 입력으로 넣었다. 계산 시점 frame만 비교하면 최종 표시 장면의 viewport 불일치를 놓친다. fixture viewport를 플랫폼 입력과 함께 고정했고 비교기는 최종 환경·WGPU `presented` 로그를 확인하도록 바꿨다. 재빌드·재설치 뒤 360×800 장면으로 다시 비교했다.
- 계산 결과에 author `<style>` 노드가 포함되며 ID 83이다. 이전의 암묵적 연속 ID 가정은 스타일 노드를 건너뛸 때 이후 DOM node를 잘못 대응시킬 수 있었다. 비교기는 숫자 NodeId와 fixture source order를 별도로 검증하고 0×0 style node만 명시적으로 제외한다.
- Block absolute 성공 범위에서 빠졌던 Flex source parent, Flex containing-block owner, layout root 자체의 absolute 입력을 fail-closed 검사와 회귀 테스트에 추가했다.

## 독립 실패 경로

| # | 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | absolute 배치 과정이 원본 HostDocument의 부모·자식 관계를 바꾸는가 | `CalcLayoutTree`는 source child map을 복사해 별도 `layout_children`을 만든다. NodeId와 HostDocument source order는 계산 parent 재연결에 사용하지 않는다. Chrome inventory가 source parent를 각 관찰에서 고정하고 Rust 결과도 source tree 입력을 읽기 전용으로 받는다. |
| 2 | `static` 중간 ancestor에서 containing block 탐색을 멈추는가 | owner 전파는 상대·절대 위치 ancestor에서만 변경된다. `skip-absolute` 기준은 static wrapper를 건너뛰어 `skip-owner`를 선택한다. |
| 3 | 중첩 absolute child가 가장 가까운 absolute ancestor 대신 바깥 relative owner를 쓰는가 | positioned owner 전파는 `relative`와 `absolute`를 모두 owner로 취급한다. `absolute_ancestor_establishes_the_nested_containing_block` 및 `nested-absolute` 기준으로 확인한다. |
| 4 | 적격 owner가 없을 때 viewport 좌표와 크기 대신 source parent를 암묵 사용하거나 surface 좌표를 섞는가 | viewport synthetic root는 계산 트리에만 있다. `viewport_absolute_box_leaves_normal_flow_unchanged` 및 `viewport-absolute`가 CSS viewport 원점을 확인한다. surface pixel·point·safe area는 runtime 비교기에서 CSS viewport와 별도 필드다. |
| 5 | `display:none` ancestor 아래 절대 자식을 viewport에 되살리는가 | owner 수집은 숨김 상태를 subtree에 전파하고 `NoBox`를 유지한다. Rust hidden subtree 테스트와 Chrome `hidden-absolute`의 `hasLayoutBox=false`가 이를 확인한다. |
| 6 | Flex source parent를 일반 Block static-position처럼 처리하는가 | `absolute_child_of_flex_source_parent_fails_closed_until_c1035`는 C10.3.5 전까지 명시 오류를 기대한다. |
| 7 | source parent는 Block이지만 containing-block owner가 Flex일 때 잘못 성공하는가 | `absolute_child_with_flex_containing_block_fails_closed_until_c1035`가 owner의 display를 따로 검사한다. source parent 검사만으로 통과시키지 않는다. |
| 8 | document root 자체의 absolute 상태에서 일부 frame을 만든 뒤 실패하는가 | `absolute_layout_root_fails_closed_before_a_partial_frame_map_exists`는 계산 진입 전 거부를 확인한다. |
| 9 | RTL 값을 LTR 기준으로 조용히 해석하는가 | `validate_positioning`은 non-static RTL positioning을 거부한다. CSS reference에 LTR만 성공 범위라고 적고 RTL·논리 inset은 미지원으로 둔다. |
| 10 | 잘못된 inset 수치가 Taffy나 GPU로 유입되는가 | finite 수치 검사와 percentage-basis 검증이 layout 실행 전 수행된다. 기존 malformed-position 검사 및 absolute percentage indefinite-height 검사가 오류 경계를 확인한다. |
| 11 | padding edge 대신 border/content edge로 containing block 원점을 잡는가 | Rust `positioned_owner_padding_edge_is_the_absolute_origin`, Chrome `edge-absolute`와 frame 비교가 border·padding을 포함한 owner를 사용한다. |
| 12 | percentage inset·size를 source parent나 viewport에 대해 푸는가 | owner별 padding-box 축 basis를 수집하고 `absolute_percentages_use_positioned_owner_instead_of_source_parent`와 `percent-absolute` 기준으로 검사한다. 세로 basis가 indefinite인 경우는 성공값으로 추정하지 않고 거부한다. |
| 13 | out-of-flow child가 auto Block 부모 높이나 뒤쪽 형제 좌표에 영향을 주는가 | normal-flow용 별도 Taffy pass에서 absolute를 flow 상태로 투영한다. `flow-row`, `flow-before`, `flow-after` 및 `viewport_absolute_box_leaves_normal_flow_unchanged`가 부모·형제 불변 조건을 확인한다. |
| 14 | 두 inset이 모두 auto일 때 static position을 일반 containing-block 좌표와 혼동하는가 | flow와 visual 결과를 분리하고 source parent/owner의 offset 차이를 변환한다. `automatic_insets_retain_static_position_when_source_parent_differs_from_owner` 및 `static-diff-absolute` 기준이 서로 다른 두 owner를 사용한다. |
| 15 | 한쪽 inset만 auto거나 반대 inset·auto size가 주어진 경우 CSS의 used geometry가 틀리는가 | 고정 기준의 `single-auto-absolute`, `opposite-absolute`, `stretch-absolute`, `over-absolute`를 비교한다. over-constrained 동작은 LTR 한정으로 문서화한다. |
| 16 | automatic margin, box sizing, min/max, padding/border, aspect ratio가 positioned size 계산에서 누락되거나 두 번 더해지는가 | `margin-absolute`, `minmax-absolute`, `border-box-absolute`, `content-box-absolute`, `ratio-absolute` reference와 Rust geometry가 각각 비교된다. intrinsic text/replaced sizing 및 일부 min/max 조합은 지원으로 추정하지 않고 거부한다. |
| 17 | absolute inline의 used display가 inline fragmentation으로 남거나 BFC 생성이 빠지는가 | Stylo projection 테스트는 absolute inline의 blockification을 typed 값으로 검사한다. `inline-absolute`와 `bfc-absolute` Chrome 기준은 block geometry를 고정한다. inline fragmentation 자체는 입력 topology 부재로 제외한다. |
| 18 | shorthand/layer/variable cascade를 adapter가 다시 해석해 Stylo 결과와 달라지는가 | `block_positioning_author_profile_accepts_layers_vars_and_border_resets_only`와 `cascade-absolute` 기준이 layer, custom property, shorthand winner를 확인한다. 미지원 author style은 fallback하지 않고 profile 오류다. |
| 19 | 오래된 source/style/environment revision 또는 일부 node만의 frame이 새 snapshot처럼 공개되는가 | layout 진입의 기존 `LayoutInputRevision` 검증과 실패 시 `Result` 경로가 사용된다. C12.2는 별도 owner graph를 결과에 반환하고 누락 owner/frame은 오류로 처리한다. 전체 runtime 비동기 stale-result 경합은 이 fixture로 증명하지 않으며 C04 runtime gate의 범위다. |
| 20 | Android/iOS 첫 계산 로그만 맞고 최종 WGPU 화면·viewport·node 대응은 다른가 | 실제 Android SM-S731N과 iPhone 17 Pro/iOS 26.2 Simulator에서 82/82 비교 대상이 고정 Chrome과 일치했고 최대 오차는 0.0125 CSS px였다. comparator는 최종 `presented boxes=80`, 360×800 CSS viewport, 숫자 NodeId 순서를 확인한다. iOS의 초기 320×240 덮어쓰기 결함은 위에서 수정 후 재실행했다. |

## 실행 증거와 남은 경계

- [Android 실기기 및 iOS Simulator 로그·화면](./c12-2-absolute-block-2026-10-11/README.md)
- [Chrome 고정 사전 비교와 23-case inventory](./c12-2-absolute-block-precomparison-2026-10-11.md)
- focused Rust: `cargo test --locked -p spinon-layout absolute_position_tests --lib`, `cargo test --locked -p spinon-layout positioning_tests --lib`
- 전체 Rust workspace test·Clippy·rustfmt 및 CSS reference suite 결과는 실행 근거 문서에 적는다. C12.2 V8 host 단위 검사는 macOS에서 host 초기화가 불가능해 ignored이며, 두 모바일 앱 runtime의 실제 V8 경로를 대신 확인했다. WPT suite는 미실행이다.
- 여기서 확인한 제한 Block/LTR fixture를 전체 CSS Position, WPT, 모든 앱 상태·비동기 resize, paint order, hit-test, clipping, 성능 또는 하드웨어 GPU 성능으로 확대하지 않는다. Flex static-position 교차는 C10.3.5, fixed·stacking·sticky는 C12 후속 단계다.

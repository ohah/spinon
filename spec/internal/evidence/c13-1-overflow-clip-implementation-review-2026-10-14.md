# C13.1 overflow clip 구현 실패 관점 20개 검토 · 2026-10-14

검토 대상은 `feat/c13-1-overflow-clip`의 제한 `RuntimeBlockFlexOverflowClipV1` 구현과 Android fixture 경로다. 이 검토는 C13.1 전체 완료 판정이 아니다. 코드, 고정 Chrome fixture, Android API 37 ARM64 AVD log/PixelCopy, Rust 테스트 및 기본/opt-in artifact 경계를 대조했다.

## 관점별 결과

| # | 실패 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | 새 overflow 값이 이전 profile에 새어 들어감 | `supports_overflow_clip()`은 overflow profile만 활성화하고, 다른 profile projection은 `Visible` 기본값을 유지한다. `cargo test --locked --workspace` 통과. |
| 2 | `overflow` shorthand가 축 값을 잃음 | Stylo computed `overflow-x/y`를 각각 typed enum으로 snapshot에 복사한다. `overflow_profile_keeps_computed_axes_and_clip_margin_typed`에서 shorthand와 축 longhand를 확인했다. |
| 3 | mixed-axis computed normalization을 authored 값으로 오인 | `visible/hidden`, `clip/auto`의 computed 축 결과를 따로 보존하고 `overflow_profile_records_mixed_axis_computed_normalization`에서 auto/hidden 변환을 확인했다. |
| 4 | 지원하지 않는 stylesheet 효과가 clip 결과에 조용히 빠짐 | overflow profile stylesheet allowlist가 `transform`과 logical `overflow-block` 입력을 거부한다. `overflow_profile_limits_inline_css_and_author_stylesheet_features` 통과. |
| 5 | 지원하지 않는 inline declaration이 무시됨 | inline property preflight는 허용 목록 밖 property를 반환하고 fixture 호출 경계에서 거부한다. `overflow-block` 거부와 지원된 overflow declaration 허용을 테스트했다. |
| 6 | `clip`이 hidden scroll-container layout 효과를 가짐 | 고정 Taffy 0.14 `Overflow::Clip`은 `is_scroll_container() == false`다. 어댑터가 `Clip`을 그대로 전달한다. float/`flow-root`의 Chrome 대조는 아직 남아 있다. |
| 7 | clip edge가 authored padding 대신 잘못된 box에서 계산됨 | `LayoutOutput.used_box_edges`가 계산 후 각 면 padding/border를 보존하고, paint 단계는 그 값과 CSS px frame을 사용한다. |
| 8 | `overflow-clip-margin` 기준 box가 틀림 | content/padding/border box edge를 나누고 clip 축만 offset을 확장한다. `overflow_clip_margin_uses_padding_edge_for_paint_and_hit_test`, `content_box_clip_margin_changes_paint_without_narrowing_chromium_hit_bounds` 통과. |
| 9 | 음수·비유한 clip offset/좌표가 snapshot에 들어감 | clip ancestry 검증은 offset이 finite이고 0 이상인지 검사하며 edge 연산 결과와 `CssAxisClip` 경계를 다시 검증한다. 잘못된 값은 `StyleRenderError`로 종료한다. Chrome의 음수 margin 관측과 명세 차이는 사전 비교에 남겨 두었다. |
| 10 | x축 제한이 y축을 함께 자름 | paint/hit clip은 두 축을 독립 보유한다. Chrome fixture의 x-only nested clip, `nested-y-visible`, `nested-y-clip-outside` 표본이 구분된다. |
| 11 | clip 조상 여러 개 중 하나만 적용되거나 순서가 바뀜 | 각 box의 DOM lineage를 확인하고 축별 interval 교집합으로 누적한다. Android 13개 box geometry 및 nested clip inside/outside 표본이 일치했다. |
| 12 | 요소 자신의 overflow가 자기 background를 자름 | ancestry iteration은 target node를 제외하고 부모부터 적용한다(`skip(1)`). 이 규칙은 own border/background와 descendant clip의 책임을 분리한다. |
| 13 | absolute child가 containing block 바깥 clip에 잘림 | `positioned_owners`가 현재 clip ancestor 안에 있는지 판정한다. `absolute_descendant_escapes_static_clip_when_its_containing_block_is_outside`와 PixelCopy `positioned-escape`가 통과했다. |
| 14 | fixed child가 element clip에 잘못 묶임 | 현재 C12 fixed profile은 viewport containing block만 만들므로 element clip lineage에서 제외한다. transform/contain이 containing block을 만드는 경우는 author allowlist가 거부하며 별도 교차 검증은 남아 있다. |
| 15 | paint clip과 hit-test clip이 같은 경계로 강제됨 | `RuntimeRenderBox`에 두 clip을 분리 저장하고 `.with_hit_test_clip()`으로 차이를 표현한다. 두 경계가 다른 runtime box 테스트가 통과했다. |
| 16 | hit-test 경계점, 음수점, viewport 밖 좌표가 target을 반환 | half-open `[min,max)` 좌표, frame, viewport를 모두 확인한다. x/y 경계와 음수 point 회귀 테스트가 통과했다. agent-browser Chrome 155 elementFromPoint() 15개 고정 좌표와 host runtime query 및 Android 앱 snapshot query가 각각 15/15 일치했다. viewport 밖 null control 네 점도 모두 통과했다. 전체 matrix와 DOM event dispatch는 남아 있다. 스타일 원본과 다른 fixture drift도 실패하도록 Rust 회귀가 공유 JS source의 node·parent·declaration을 대조한다. |
| 17 | WGPU가 hit clip을 paint에 쓰거나 빈 quad를 그려 냄 | vertex 생성은 paint clip만 frame에 교차하고 폭/높이가 0 이하인 결과를 건너뛴다. clip 교차 vertex 테스트와 PixelCopy paint controls가 통과했다. |
| 18 | clip lineage 예약 실패 뒤 partial snapshot을 반환하거나 재시도 상태가 오염됨 | `overflow_clip_ancestry_reservation_failure_returns_error_and_recovers_on_same_reserve`가 예약 실패의 stage를 확인하고 같은 reserve의 다음 snapshot 생성이 baseline과 같음을 확인했다. 실제 앱 worker에서 실패 후 새 요청 복구는 아직 입증하지 않았다. |
| 19 | stale revision 또는 잘못된 node set이 clip 계산을 우회함 | 기존 document/style/environment revision 및 node-set admission을 clip 생성보다 앞에서 수행하고 snapshot을 마지막에 만든다. 관련 기존 stale/missing-node 실패 테스트와 workspace suite 통과. |
| 20 | AVD 표본 선택·DPR 또는 fixture payload가 실제 결과와 다름 | agent-browser Chrome 155의 13 frame과 11 CSS sample을 고정하고 AVD SurfaceView PixelCopy에서 density `2.625`를 적용했다. run `a101139308e`에서 13/13 geometry, Chrome·Android PNG·log 11/11 RGBA, opt-in/default FFI symbol·payload 격리를 확인했다. |

## 발견 및 잔여 경계

PixelCopy의 첫 y축 바깥 표본은 `y=95`에서 border 장식 색을 기대해 현재 background-only renderer 범위를 벗어났다. 표본을 clip 바깥의 root 배경인 `y=97`로 옮겨 장식 미구현과 clip 누수를 구분했고 최종 11/11 exact를 다시 저장했다. 내부 계약의 “공개 API 아님” 표현은 Rust crate 간 공개 타입 전달과 혼동될 수 있어 “제품 CSS 지원 선언 아님”으로 바꿨다.

전체 C13.1 matrix, runtime worker의 같은 세션 복구, clip 밖 draw와 hit/event negative-control breadth, `clip` 대 `hidden`/`flow-root` float, iOS Simulator, WPT 분류, border radius/transform 교차 사례는 여전히 미완료다. AVD는 ANGLE/SwiftShader software renderer이며 이 결과는 실기기 또는 compositor scanout 근거가 아니다.

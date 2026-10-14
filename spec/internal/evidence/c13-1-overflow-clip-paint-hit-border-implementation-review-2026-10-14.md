# C13.1 paint/hit border-strip 구현 실패 관점 20개 · 2026-10-14

검토 대상은 static/absolute clip 경계 여섯 점, `hit_test_order` 분리, C13.1 profile 격리다. 각 항목은 다른 실패 경로를 확인한다. Chrome 표본은 고정 Chrome 155이며, 앱 Simulator fixture에는 새 border 좌표를 추가하지 않았다.

1. **Static child 내부 hit** — paint clip 안 `(125,40)`에서 child가 부모보다 먼저 hit되는가? Chrome oracle와 Rust focused test에서 `static-target`으로 일치.
2. **Static 오른쪽 border strip** — child rect 안이지만 paint clip 밖 `(128,40)`에서 child가 새어 나오지 않는가? Chrome과 Rust 모두 `static-clip`.
3. **Static border 바깥** — parent frame 밖 `(131,40)`에서 clip parent를 과잉 hit하지 않는가? Chrome과 Rust 모두 `root`.
4. **Absolute child 내부 hit** — parent padding 안 `(315,40)`에서 absolute clip 자식이 paint 순서의 parent에 가려지지 않는가? Chrome과 Rust 모두 `positioned-target`.
5. **Absolute border의 paint/hit 분리** — `(318,40)`에서 border pixel은 gold지만 hit target은 child인가? Chrome pixel `(240,198,116,255)`, Chrome/Rust target `positioned-target`.
6. **Absolute border 바깥** — frame 오른쪽 `(321,40)`에서 확장된 hit clip이 parent border box 밖으로 잘못 나가지 않는가? Chrome과 Rust 모두 `root`.
7. **다른 child pixel negative control** — `(125,40)`과 `(315,40)`의 color가 child blue인가? screenshot RGBA `(87,166,217,255)`로 확인.
8. **Paint clip 불변** — hit 수정이 static/absolute paint clip을 확장하거나 줄이지 않는가? runtime interval은 각각 `[14,126]`, `[204,316]` 그대로.
9. **Painter 우선순위 유지** — absolute parent border가 child 위에 그려지는 paint 순서를 바꾸지 않는가? fixture assertion에서 target `paint_order < parent.paint_order`.
10. **Hit 우선순위 독립** — 같은 쌍의 hit-test는 child를 우선하는가? fixture assertion에서 target `hit_test_order > parent.hit_test_order`; render crate unit test도 paint/hit 순서 분리 확인.
11. **Clip parent 자체 hit 영역** — child가 clip edge 밖인 strip에서 parent가 여전히 event target이 되는가? static `(128,40)` browser/runtime 결과가 parent.
12. **Mixed axis 보존** — `overflow-x:clip; overflow-y:visible`의 y축까지 clip으로 좁히지 않는가? 15점 기존 Chrome 비교와 축별 clip tests 통과; 새 fixture y=40 표본도 허용.
13. **Relative positioned regression** — `position:relative` clip margin hit 확장을 잃지 않는가? `overflow_clip_margin_uses_padding_edge_for_paint_and_hit_test` 통과.
14. **Content-box clip margin** — paint visual box는 content edge지만 positioned hit bounds는 border까지 유지하는가? `content_box_clip_margin_changes_paint_without_narrowing_chromium_hit_bounds` 통과.
15. **외부 containing block 탈출** — static clip 아래 absolute 자식의 containing block이 바깥이면 clip chain이 계속 자식을 차단하지 않는가? `absolute_descendant_escapes_static_clip_when_its_containing_block_is_outside` 통과.
16. **C12 stacking profile 격리** — 변경하지 않은 CSS profile에 새 preorder가 적용되지 않는가? `negative_zero_and_positive_contexts_follow_css_paint_phases`에서 모든 box의 hit 순서가 paint 순서와 동일함을 확인.
17. **형제 painter order** — overlap sibling의 query target이 document insertion만으로 덮이지 않는가? 기존 Chrome runtime hit matrix, flex order, reverse painter matrix 통과.
18. **Stacking context 경계** — descendant z-index가 parent context 밖으로 새지 않는가? 기존 nested positive/zero context와 WPT 유래 4개 stacking 테스트 통과.
19. **순서 metadata fail-closed** — hit order 중복과 장면 범위 밖 값이 snapshot으로 승인되지 않는가? `spinon-render` 테스트가 두 경우 `InvalidHitTestOrder`로 거부.
20. **예약·깊이·viewport 경계** — 새 순서 구성의 Vec 예약 실패가 지정 stage로 반환되고, 기존 깊은 tree·viewport/non-finite/half-open 검증이 살아 있는가? 세 신규 reservation stage, 기존 exact-stage injection, 1,024-node, viewport/non-finite/half-open tests 통과.

## 검증 한계

실제 iOS Simulator 앱은 13/13 frame, 기존 15/15 hit query와 viewport null controls를 통과했다. 그 앱 query에는 새 여섯 border-strip 좌표가 없으며 Android app route도 이번 delta에서는 실행하지 않았다. Chrome six-point fixture와 workspace Rust 테스트 외의 DOM event dispatch, WPT 전체 또는 drawable readback은 입증하지 않는다. C13.1 전체와 상위 C13은 미완료다.

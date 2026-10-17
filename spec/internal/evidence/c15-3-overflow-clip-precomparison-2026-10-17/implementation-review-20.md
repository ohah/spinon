# C15.3 owner text clip 구현 실패 관점 검토 20개

## 검토 범위

- 작업 기준 commit: `073c74f7bd3140f9cdefbac70a344dd2db3d240c`와 현재 C15.3 owner-clip 변경.
- 비교 기준: text는 자기 owner의 paint clip을 적용하고, `overflow:visible`은 부모 경계 밖 잉크를 보존하며, 지원하지 않는 scroll transform은 명시 거부한다.
- 재현 근거: [Android PixelCopy·로그](./README.md), [iOS Metal drawable·로그](./README.md), 관련 Rust/WGPU/FFI tests.
- 전체 DOM 텍스트 연결, 다른 overflow 값, non-zero scroll transform, hit-test, 하드웨어 GPU와 실기기 동작은 이 검토 범위 밖이다.

## 독립 실패 관점

1. **Text owner 누락** — 존재하지 않는 `NodeId`를 text owner로 주면 snapshot 생성을 거부하는지 확인했다. `text_snapshot_checks_bitmap_geometry_source_owner_and_unique_node`가 owner 누락을 검사한다. 통과.
2. **중복 Text owner** — 한 NodeId에 두 text snapshot을 연결해 paint command 소유가 모호해지지 않는지 확인했다. 같은 renderer test가 중복을 거부한다. 통과.
3. **오래된 text source revision** — text source가 scene key와 다르면 clip 계산 전에 거부한다. 위 renderer test의 source 검증과 `validate_texts_and_apply_owner_clips`의 선행 검사를 확인했다. 통과.
4. **잘못된 draw index/order** — 유효하지 않거나 순서가 바뀐 DrawText를 통과시키지 않는다. `text_paint_command_indices_must_be_complete_and_in_order`가 검사한다. 통과.
5. **scroll metadata 배열 길이 불일치** — owner index가 metadata 범위를 벗어나 panic하지 않고 `InvalidScrollState`를 반환한다. snapshot constructor가 text validation 전에 길이를 대조한다. 통과.
6. **metadata와 render-box frame 불일치** — 같은 index의 다른 frame metadata를 owner clip으로 오인하지 않도록 constructor가 먼저 거부한다. snapshot frame 대조를 확인했다. 통과.
7. **누락된 scroll ancestor container** — metadata가 가리키지만 scroll container가 없는 ancestor를 조용히 건너뛰지 않는다. `text_rejects_scroll_ancestor_metadata_without_a_matching_container`가 회귀를 막는다. 통과.
8. **중복 scroll owner** — 하나의 owner에 두 scroll container가 등록되면 text 검증에 도달하기 전에 거부한다. constructor의 `DuplicateScrollOwner` 검사를 확인했다. 통과.
9. **Text owner 자신이 scroll container인 경우** — 현재 transform 지원으로 오인해 clip만 적용하지 않고 `UnsupportedTextScroll`을 반환한다. `text_owner_that_is_itself_a_scroll_container_is_rejected`가 검사한다. 통과.
10. **가로 non-zero scroll** — x offset을 빼먹거나 clip을 옮기지 않은 채 그리지 않는다. `text_inside_a_scrolled_overflow_owner_still_requires_scroll_transform_support`가 x offset 1 CSS px를 거부한다. 통과.
11. **세로 non-zero scroll** — x 검사만 있고 y transform은 통과하는 경로가 없도록 별도 회귀 test `text_inside_a_vertically_scrolled_overflow_owner_still_requires_scroll_transform_support`를 추가했다. y offset 1 CSS px를 거부한다. 통과.
12. **zero-offset scroll ancestor** — scroll ancestor가 있다는 이유만으로 정적 clip까지 버리지 않는다. `text_inside_an_unscrolled_overflow_owner_keeps_its_paint_clip`와 snapshot 로직에서 0 offset clip 보존을 확인했다. 통과.
13. **Text clip과 owner clip의 교집합** — 입력 text clip보다 owner clip이 넓어져 text를 풀어주지 않으며 x/y 각각 `[5,8]`, `[2,6]` 교집합을 유지한다. `text_clip_intersects_the_owner_boxes_effective_paint_clip`가 검사한다. 통과.
14. **visible owner 확장** — owner에 별도 paint clip이 없을 때 text clip이 owner frame으로 임의 축소되지 않는다. `text_can_overflow_a_visible_owner_when_neither_clip_is_constrained`가 검사한다. 통과.
15. **실제 WGPU 경계 sample** — owner frame을 1×1로 줄여도 unclipped text glyph가 바깥 sample 두 점에 남는다. `wgpu_readback_keeps_glyph_ink_outside_an_unclipped_owner`가 읽는다. 통과.
16. **glyph mask 안의 sample 선택** — 좌표를 임의 하드코딩하거나 antialiased edge를 고르지 않고 고정 Ahem raster에서 coverage가 255인 pixel을 찾아 hidden clip 내부와 parent 바깥 점을 고른다. viewport·x/y clip 유효성 검사와 Android/iOS 관측값을 확인했다. 통과.
17. **hidden 내부의 부모 배경 오인** — 초기에 “stage 배경색이 아니면 잉크”라고 분류해 부모 배경색만으로 hidden 내부가 통과할 수 있었다. Android와 iOS 모두 정확한 고정 ink `[9,33,112,255]`를 요구하도록 수정했고 새 AVD/Simulator run에서 통과했다.
18. **hidden 외부의 잔류 잉크** — hidden owner 바깥 sample은 정확한 stage 배경 `[247,249,252,255]`이어야 한다. Android PixelCopy와 iOS Metal 표면 모두 해당 RGBA를 반환했다. 통과.
19. **stale 또는 다른 Metal drawable의 표본** — iOS readback report를 요청 generation, drawable size, format과 맞추고 CSS pixel center를 drawable pixel 좌표로 바꾼다. mismatch는 실패 처리하며 새 실행에서 1080×720/3× mapping과 정확 좌표를 기록했다. 통과.
20. **fixture 없는 기본 ABI** — 검증 helper가 기본 runtime에 노출되지 않도록 default와 opt-in FFI artifact를 각각 빌드했다. [artifact 경계 기록](./artifact-boundary.txt)에서 기본 artifact의 symbol/payload marker 부재와 opt-in positive control을 확인했다. 통과. Apple `nm`은 LLVM 22.1 object attribute를 읽지 못해 해당 기록은 `strings` marker scan을 사용한다.

## 판정

hidden parent 배경을 잉크로 오인할 수 있는 판정 결함을 수정하고 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 다시 확인했다. 고정 fixture의 owner clip 제한 slice는 통과하지만 일반 DOM/style 연결과 C15.3 전체 완료를 뜻하지 않는다.

# C11.1 Stylo→layout 연결 구현 검토 · 2026-10-12

**검토 대상:** `RuntimeBlockFlexGridV1` cascade, typed Grid value extraction, `compute_runtime_block_flex_grid_style_layout`, Grid layout projection. 기존 layout-core 전용 검토와 분리한다.

## 독립 실패 관점 20개

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 다른 `HostDocumentSnapshot`에서 만든 Stylo view를 함께 쓰는가 | snapshot identity 검사 뒤 cascade에 진입한다. 전체 workspace 회귀 통과. |
| 2 | 같은 snapshot이어도 adapter root와 Stylo mount root가 달라질 수 있는가 | 불일치를 발견해 `SnapshotMismatch { field: "root" }` guard와 `style_layout_rejects_a_root_different_from_the_stylo_mount_root` 회귀를 추가했다. 통과. |
| 3 | root가 아닌 element를 mount root로 바꾸어 부분 트리 계산이 되는가 | 현재 계약은 view mount root와 layout root가 동일해야 한다. 별도 subtree API는 제공하지 않는다. |
| 4 | 새 Grid profile이 기존 Block/Flex profile에 Grid 허용을 섞는가 | 새 `RuntimeBlockFlexGridV1` 분기로 격리한다. 기존 profile의 grid 거부 회귀와 workspace 통과. |
| 5 | display가 `grid`가 아닌 노드에 track 값을 강제로 요구하는가 | computed Grid payload는 Grid display에만 만든다. `display:none` subtree 회귀 통과. |
| 6 | author stylesheet cascade 대신 inline 문자열만 직접 해석하는가 | Stylo author stylesheet 입력을 별도 fixture로 계산하고 typed track 결과를 확인한다. 통과. |
| 7 | cascade된 값과 author 원문이 다를 때 원문을 computed 값으로 오인하는가 | `ComputedValues`의 typed track/flow 값을 읽고 unsupported diagnostic에는 serialized property context를 남긴다. |
| 8 | 외부 CSS 문법을 Grid track parser가 다시 해석하는가 | 별도 문자열 parser는 만들지 않았다. Stylo typed CSS 값을 허용 enum으로 변환한다. |
| 9 | `grid-auto-flow:column` 또는 `dense`가 기본 row로 조용히 치환되는가 | cascade는 flow variant를 보존하고 C11.1 layout은 지원 밖 flow를 명시 거부한다. 회귀 통과. |
| 10 | `none`·빈 track list·auto-repeat가 빈 Grid 성공으로 바뀌는가 | 빈 list, auto-repeat, named-line list는 computed extraction에서 거부한다. |
| 11 | `auto`·`min-content`·`max-content`·`minmax()`·`fit-content()`·calc track이 무시되는가 | 현재 typed mapping에 없는 track syntax는 node/property/value diagnostic으로 거부한다. 회귀 통과. |
| 12 | 음수·NaN·무한 length/percentage 또는 0·음수 `fr`을 layout으로 전달하는가 | cascade와 layout projection이 각각 finite/range 조건을 검사한다. Rust 회귀 통과. |
| 13 | percentage track이 indefinite axis에서 임의의 0% 기준을 쓰는가 | layout core가 해당 축의 definite basis를 요구한다. `indefinite_grid_axis_fails_with_specific_basis` 통과. |
| 14 | `fr` 분배에서 gap을 두 번 빼거나 빼지 않는가 | 고정 track·gap scene과 `fr` layout core geometry를 검사한다. Rust 회귀 통과. |
| 15 | unsupported inline placement가 source-order auto-placement로 조용히 내려가는가 | inline grid placement preflight가 layout 전에 실패한다. 전용 regression 통과. |
| 16 | 비기본 item `order` 또는 명시 placement가 지원된 것처럼 재정렬되는가 | 현재 C11.1은 기본 source order와 auto placement만 허용하며 그 외는 fail-closed한다. |
| 17 | 명시 track 용량이 모자랄 때 implicit track을 생성하는가 | 셀 부족은 explicit error다. `insufficient_explicit_cells_fail_instead_of_creating_an_implicit_track` 통과. |
| 18 | Grid 안팎 mixed Block/Flex/Grid가 source tree를 재배치하는가 | HostDocument order를 유지한 mixed dispatch regression 및 고정 Chrome scene 비교가 통과한다. |
| 19 | style/environment/document revision이 서로 섞이거나 오래된 scene이 재사용되는가 | 결과 computed snapshot과 layout revision의 document/style/environment 값이 같은 입력을 가리키는지 검사한다. regression 통과. |
| 20 | track reservation 실패가 일부 값을 게시하거나 node/property 문맥을 잃는가 | cascade 및 adapter의 각 track reservation 단계에서 오류 주입; vector는 부분 게시되지 않고 축 property 문맥을 보존한다. 회귀 통과. |

## 발견 사항과 수정

- **수정:** adapter 호출 root와 Stylo view mount root가 달라도 같은 document snapshot만으로 통과할 수 있었다. root identity guard를 추가하고 회귀를 넣었다.
- **범위 정정:** 구현 전 계획은 Stylo cascade 및 Rust adapter를 미구현으로 적고 있었다. 현재 코드와 테스트에 맞춰 계획·내부 계약·상태 대장을 갱신했다.
- **비교 범위 정정:** Rust adapter가 직접 대조하는 pinned Chromium 장면은 `fixed-tracks-gap` 1개, 5 node다. Chrome 사전 reference 10 scene·40 node 전체가 Spinon 구현으로 검증된 것은 아니다.
- **남은 구현:** Android/iOS FFI와 V8 runtime은 `RuntimeBlockFlexGridV1` 진입점을 아직 제공하지 않는다. 기존 runtime이 `display:grid`를 거부하는 상태이므로 연결된 Android 실기기에서도 C11 Grid 화면·WGPU pixel 검증은 아직 실행할 수 없다. 이는 기기 연결 문제로 분류하지 않는다.

## 재실행 근거

- `mise exec -- cargo test --locked --workspace` — 592 passed, 0 failed, 3 ignored.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` — 통과.
- `mise exec -- cargo fmt --all -- --check` — 통과.
- `mise exec -- bun run test:css-reference` — 현재 실행 결과는 [검증 기록](./c11-style-grid-projection-2026-10-12/README.md)에 기록한다.
- `mise exec -- bun run test:js` 및 `mise exec -- bun run test:css-c11-grid-precomparison` — 실행 결과는 같은 검증 기록에 둔다.

이 검토는 C11.1 mobile runtime·WGPU 경로, C11 전체, WPT 적합성 또는 공개 CSS 지원을 승인하지 않는다.

# C12.4 · 구현 변경 적대 검토

**대상:** `feat/c12-4-stacking-context-runtime`의 Rust paint builder, Stylo profile, FFI, Android/iOS 연결, WGPU 제출 및 실행 근거  
**계약 숫자 버전:** `0.1.0` 고정  
**결론:** 10-node 검증 범위에서 발견한 런타임 경로·증거 판정 결함은 수정했다. C12.4 전체 완료는 아니다. 전체 26-scene mobile runtime, WPT candidate 실행, iOS surface pixel readback, PR 검토·병합은 별도 미완료 관문이다.

## 서로 다른 실패 관점 20개

1. **Android 화면 진입 분기** — `MainActivity`의 GPU demo 실행 조건에 C12.4 extra가 빠져 있던 것을 찾았다. 조건을 추가하고 회귀 검사를 만들었다. 수정 APK를 SM-S731N에서 실행해 `SPINON_C124_EVAL status=0`을 확인했다.
2. **Android Surface 준비 뒤 초기화** — Surface가 준비된 뒤 실행하는 positioning 초기화가 C12.1–C12.3만 검사해 C12.4가 대기 상태에 머무르는 경로를 찾았다. C12.4 조건을 추가했고, 재실행 로그에서 `layout=ready boxes=10`과 실제 WGPU 제출을 확인했다.
3. **iOS 앱 진입 인자** — AppDelegate와 demo 진입부가 같은 `--spinon-c124-stacking-context` 인자를 전달하는지 검사했다. iPhone 17 Pro Simulator에서 호스트·fixture·Metal surface 경로를 다시 빌드해 확인했다. iOS 실기기 결과로 확대하지 않는다.
4. **FFI 함수·fixture 연결** — iOS C ABI와 Android JNI가 서로 다른 fixture 또는 오래된 C12.3 API를 부르지 않는지 검사했다. 소스 회귀 검사와 양 플랫폼 V8 log의 `document_nodes=10`으로 연결을 확인했다.
5. **profile 혼합·revision freshness** — Block/Flex/C12 위치 결과를 다른 profile snapshot에서 합치거나 stale style/environment revision으로 admission하는지 확인했다. 전용 profile 검사와 기존 revision 실패 테스트가 이를 거부한다.
6. **root·document graph 무결성** — 잘못된 root, text root, 중복 방문 노드가 불완전 scene을 만들 수 있는지 확인했다. element root 검사, visited set, style/layout node-set 검증이 전체 오류로 막는다.
7. **text와 숨겨진 subtree 경계** — 표시되는 text node가 box처럼 조용히 paint되는지 확인했다. 표시 subtree의 text는 거부하고, `display:none` ancestor 아래는 paint traversal에서 제외한다. inline/text 지원으로 확대하지 않는다.
8. **누락·추가 style/frame** — HostDocument와 computed style/layout frame의 node 집합이 다를 때 일부만 제출되는지 확인했다. 세 집합 검증이 `RuntimeRenderSnapshot` 생성 전에 오류를 반환한다.
9. **typed z-index 누락·잘못된 값** — `auto`를 임의의 `0`으로 대체하거나 문자열을 다시 해석하는지 확인했다. `ComputedCssZIndex`만 사용하고 누락 typed 값은 거부한다.
10. **visibility 상속·collapse** — `hidden`/`collapse`가 자식 상속이나 CSS cascade 뒤 통과하는지 확인했다. Stylo typed computed value를 admission에서 확인하며 hidden/collapse는 scene 전체를 거부한다.
11. **author CSS 우회** — inline declaration, `@media`, `@supports`, nested rule 또는 미지원 declaration이 제한 profile을 우회하는지 확인했다. inline allowlist·stylesheet preflight·cascade diagnostic 테스트가 거부한다. source selector 승자를 계산하지 않는 데 따른 보수적 과잉 거부는 계약에 남겨둔다.
12. **context 생성자 분류** — relative/absolute integer, positioned `auto`, fixed `auto`, static Flex item integer가 각각 다른 context 규칙을 쓰는지 확인했다. typed decision table 및 전용 Rust ordering 테스트로 확인했다.
13. **음수·0·양수 stack phase** — 값들을 전역 숫자로 비교하거나 동률 순서를 비결정적으로 두는지 확인했다. context-local phase와 고유 tree-order tie-break를 구현했고 phase 테스트가 기대 순서를 고정한다.
14. **실제 nested context 격리** — 큰 자손 `z-index`가 상위 context 밖 sibling을 덮는지 확인했다. 중첩 positive 및 zero context 테스트에서 child subtree가 한 parent participant로 flatten된다.
15. **`z-index:auto` pseudo-context** — positioned auto box가 자손 정수 context를 잘못 가두는지 확인했다. 자손 context가 상위 context 단계에 참여하는 fixture를 고정했다.
16. **Flex `order`와 positioned child** — `order`가 HostDocument를 변경하거나 absolute/fixed 자식까지 in-flow 정렬에 넣는지 확인했다. Flex 직계 in-flow child만 `(order, source order)`로 정렬하며 absolute/fixed child는 재배치하지 않는 테스트가 있다.
17. **`display:none`·0-area paint admission** — 숨김 subtree 또는 0 크기 box가 paint rank와 box 배열에 섞이는지 확인했다. hidden node는 scene에서 생략되고 유효 box만 0부터 연속 `paint_order`를 받는다.
18. **깊은 graph·allocation·overflow 실패** — traversal이 Rust 재귀 stack을 쓰는지, 확인된 `Vec::try_reserve` 실패와 `u32` 변환 오류가 부분 scene을 반환하는지 코드 경로를 검사했다. traversal은 반복형이고 reserve/checked conversion 오류를 반환한다. allocator 실패 주입과 매우 깊은 graph stress test는 아직 추가하지 않았다.
19. **Renderer가 CSS 정렬을 재구현하는지** — WGPU가 별도 z-index sort를 하거나 계산된 순서를 무시할 가능성을 확인했다. renderer에는 flat snapshot 순서로 전달하고 macOS offscreen WGPU가 cyan·green·red·root 배경 4 pixel을 정확히 읽었다.
20. **플랫폼 근거 과장·누락** — Android 실기기와 iOS Simulator 결과가 실제 runtime인지, 캡처를 pixel oracle로 오인하지 않는지 확인했다. 양쪽에서 같은 10 frame·paint rank·10 box 제출과 화면을 확인했다. 전체 26-scene 실행, WPT, iOS surface readback은 아직 없으며 evidence에 미완료로 남겼다.

## 검증 명령

- `cargo test --locked --workspace`
- `cargo clippy --locked --workspace --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo test --locked -p spinon-render-wgpu --test runtime_stacking -- --nocapture`
- `bun run test:css-reference` — 168개 통과
- Android Debug APK 빌드 후 SM-S731N/Android 16/Vulkan 직접 실행
- iOS Simulator 빌드 후 iPhone 17 Pro/iOS 26.2/Metal 직접 실행
- [플랫폼 실행 로그·캡처·한계](./c12-4-stacking-runtime-2026-10-11/README.md)

## 남은 merge 관문

이 검토는 계획 검토·profile 검토·PR 통합 검토를 대신하지 않는다. WPT 미실행 범위와 mobile subset을 유지한 상태에서 PR 본문을 검토하고, 별도의 PR 변경 검토 후에만 병합할 수 있다. public support 상태와 C12 상위 작업은 미완료로 유지한다.

## 후속 실행 정정

위 20개 검토는 당시 10-node fixture 기준 상태를 기록했다. 그 뒤 Android AVD에서 26-scene runtime matrix 및 `PixelCopy` 21-scene/22-sample surface 비교가 완료됐고, iOS Simulator의 Metal surface에서 변경된 reverse Flex 방향·order 8개 표본도 exact match했다. iOS Simulator 전체 26-scene geometry matrix 역시 26/26으로 확인했다. Android AVD·iOS Simulator 동시 session process memory도 0·1·2·4·8단계별 세 번 수집했다. 상세 근거는 [최신 런타임 요약](./c12-4-stacking-runtime-2026-10-11/README.md), [iOS runtime matrix 결과](./c12-4-ios-runtime-matrix-implementation-review-2026-10-11.md), [iOS Metal 결과](./c12-4-ios-metal-surface-pixels-2026-10-11/), [동시 session 결과](./c12-4-concurrent-session-stack-cost-2026-10-11.md)에 연결했다. 이 후속 결과는 전체 iOS scene의 surface pixel, hardware GPU, 실기기 메모리 귀속, Spinon WPT 적합성 또는 C12.4 전체 완료를 증명하지 않는다.

추가로 실제 HostDocument→Stylo→Taffy worker에서 Android API 37 AVD와 iOS Simulator 각각 1,024 depth 성공·1,025 거부·새 revision 복구를 확인했다. painter reservation 실패 주입·50,000 단계/폭 synthetic graph와 worker별 pthread bounds/mincore도 후속 실행으로 완료했다. 현재 CSS reference 회귀는 189/189이며 전체 검증과 남은 경계는 [후속 검증 결과](./c12-4-follow-up-verification-2026-10-11/README.md), [worker 표본](./c12-4-worker-stack-residency-2026-10-11/README.md)에 있다. 기존 20개 관점의 원래 관찰 시점은 유지하며, 위 결과를 초기 검토 때 이미 실행한 것처럼 소급하지 않는다.

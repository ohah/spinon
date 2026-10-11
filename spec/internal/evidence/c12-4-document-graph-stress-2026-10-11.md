# C12.4 실제 문서 그래프 깊이 스트레스

**계획:** [C12.4 stacking context](../../../plan/c12-4-stacking-context.md) · [내부 계약 0058](../0058-c12-4-stacking-context.md)  
**구현:** [`deep_graph_tests.rs`](../../../crates/spinon-style-to-render/src/stacking/deep_graph_tests.rs)  
**플랫폼 근거:** [Android AVD 실행 기록](./c12-4-stacking-runtime-2026-10-11/README.md)

## 실행 결과

- 일반 Rust test thread에서 128개의 실제 `HostDocument` 요소를 한 번의 변경 묶음으로 만들고 snapshot → Stylo view/cascade → Taffy layout → paint snapshot까지 통과했다. 각 단계의 node/frame/box 수가 128이고 paint order가 `NodeId` 순서와 일치한다. 가장 깊은 frame은 `(0, 0, 1, 1)`이다.
- 별도 ignored stress test는 1,024개 요소를 같은 전체 경로에 넣어 통과했다. `RUST_MIN_STACK=33554432`로 test thread stack을 32 MiB로 키운 실행이며 일반 앱 thread 지원 증거가 아니다. 단일 Debug 실행에서 문서 생성 124.432 ms, snapshot 0.521 ms, cascade 62.559 ms, layout 35.168 ms, paint 6.609 ms, 전체 248.475 ms였다. 성능 벤치마크가 아니다.
- 일반 test thread에서 깊이 256을 시험했을 때 cascade 완료 로그 다음 `compute_runtime_style_layout` 호출 중 stack overflow로 프로세스가 종료됐다. 깊이 254는 같은 환경에서 통과했지만 안전한 최대 깊이나 이식 가능한 한계로 보지 않는다. 정확한 내부 frame은 crash stack trace로 심볼화하지 않았다.
- 고정 비교 Chrome은 `Google Chrome 154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, binary SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`다. 같은 `createElement`/`appendChild` DOM API로 1,024개 요소를 만들었고, 최심부 부모가 `node-1022`, geometry가 `(0, 0, 1, 1)`, `display:block`임을 확인했다. pixel paint 비교는 하지 않았다.
- 2026-10-11 17:15 KST Android API 37 AVD(`emulator-5554`, Android 17, arm64-v8a, 16 KiB page size)에서 26-scene runtime matrix를 다시 실행해 `26/26`을 확인했다. 이번 Android 실행은 scene matrix이며 1,024 깊이 그래프를 Android에서 돌린 결과가 아니다. AVD 그래픽은 ANGLE/SwiftShader software GL이다. 당시 `adb devices -l`에는 이 AVD만 연결되어 있었다.
- HTML 문자열 1,024단계를 직접 파싱한 첫 비교는 폐기했다. HTML parser의 open-element stack 제한으로 실제 parent chain이 재구성됐기 때문이다. 제품 HostDocument와 같은 DOM API 생성으로 바꿔 비교했다.

## 실패 관점 재검토

1. **root를 깊이에서 제외했는가?** root를 포함해 128 또는 1,024개 Element의 정확한 개수를 검사한다.
2. **HTML parser의 깊이 제한이 결과를 오염시키는가?** HTML 문자열 nesting은 사용하지 않고 DOM API로 부모·자식을 연결했다. parser가 parent chain을 재구성한 초기 입력은 버렸다.
3. **한 갈래만으로 넓은 tree도 검증했다고 말하는가?** 이 fixture는 단일 깊이 chain이다. 폭은 별도의 synthetic painter fanout 50,000개 검증에만 해당한다.
4. **모든 요소가 동일해 stacking context가 생기지 않는 경우인가?** 각 요소에 `position:relative; z-index:0`을 지정해 각 level이 실제 context 경로를 통과한다.
5. **중복 NodeId나 순서 교환을 놓치는가?** 각 output box의 NodeId가 입력 handle index와 같고 paint order가 연속 index인지 전부 비교한다.
6. **부분 변경 묶음 때문에 수명 경로가 빠지는가?** 전체 node 생성·style 지정·삽입을 단일 `DocumentChangeBatch` commit으로 만든다. 반복 incremental update는 이 검증의 범위가 아니다.
7. **서로 다른 revision을 섞는가?** snapshot, cascade, layout, paint에 동일 snapshot과 revision을 계속 전달한다.
8. **Stylo view가 실제 HostDocument subtree를 순회하는가?** root fragment child view를 만들고 그 아래 실제 HostDocument 자식 관계를 사용한다.
9. **cascade 결과가 끝까지 생성됐는가?** computed style element 개수가 전체 graph node 수와 같은지 확인한다.
10. **layout이 일부 node만 처리했는가?** layout frame 개수가 전체 node 수인지 확인한다.
11. **box가 0 크기여서 paint 결과에서 빠졌는가?** 각 요소에 양의 고정 width/height 및 불투명 배경을 주고 render box 전체 개수를 확인한다.
12. **깊은 끝의 frame이 틀려도 개수만 맞는가?** 최심부 frame의 x/y/width/height를 직접 검사한다.
13. **snapshot 생성 중 paint node를 누락·중복하는가?** 모든 box의 NodeId 및 순차 paint order를 원본 handle과 대조한다.
14. **CI에서 위험한 stack crash를 기본 실행하는가?** 일반 test는 128 깊이만 사용한다. 1,024 깊이는 명시적으로 ignored 처리했다.
15. **256 깊이 stack overflow를 숨기고 있는가?** 실패를 기록했고 테스트 결과로 취급하지 않는다. 기본 test thread에서 cascade 뒤 layout 호출 중 프로세스가 종료됐다.
16. **254 성공을 최대 허용 깊이로 오해하는가?** 경계 근처 표본 한 건은 플랫폼·빌드·thread stack에 따라 달라지므로 max-depth 정책으로 일반화하지 않는다.
17. **32 MiB stack 결과를 모바일 기본 thread에 일반화하는가?** 그렇게 하지 않는다. Android/iOS runtime thread의 stack 크기와 깊이 한계는 미측정이다.
18. **시간 한 표본을 성능 결론으로 쓰는가?** 단계 시간은 Debug 단일 실행의 진단 로그이며 반복 분포나 비교 벤치마크로 표현하지 않는다.
19. **Android 실기기 또는 GPU 검증으로 표현하는가?** 아니다. 실기기는 사용하지 않았다. AVD에서는 기존 26-scene matrix만 재실행했고 deep graph는 host Rust test에서 실행했다. scene pixel도 읽지 않았다.
20. **Chrome에서 다른 tree shape를 비교하는가?** Chrome 요소도 HTML parser가 아닌 DOM API로 만들었다. pinned browser의 node 수·부모·geometry만 확인했으며 paint pixel, Android AVD deep graph 및 CSS 전체 호환은 주장하지 않는다.

## 남은 조건

128-node 전체 경로와 1,024-node 대형 stack 실행은 통과했지만, 일반 stack에서 256 depth가 프로세스를 종료한다. 따라서 전체 깊은 문서 그래프 검증은 완료 처리하지 않는다. 다음 단계는 Android AVD runtime thread와 iOS Simulator runtime thread에서 실제 깊이를 확인하고, stack-safe layout 경계나 안전한 명시적 거부 계약을 정하는 것이다. Android scene별 surface pixel 및 iOS Metal readback은 별도 미완료 항목이다.

## 후속 모바일 worker 깊이 검증

이 문서의 256-depth default test thread overflow 관측은 유지한다. 뒤이어 Android API 37 AVD와 iPhone 17 Pro / iOS 26.2 Simulator 앱의 실제 8 MiB CSS worker에서 HostDocument depth 1,024 성공, 1,025를 `tree_depth_exceeded`로 거부, 같은 worker의 새 revision 복구를 각각 확인했다. 세션이 동시에 여럿일 때의 process memory는 [별도 측정](./c12-4-concurrent-session-stack-cost-2026-10-11.md)에서 0·1·2·4·8단계별로 수집했지만, 그 증가분을 특정 CSS thread stack에 귀속하지 않는다. 따라서 당시 기록의 “실제 runtime thread 검증 미완료”는 후속 실행으로 닫혔고, 기본 test thread의 재귀 깊이 위험과 실기기 stack/RSS 미확정은 남는다. [플랫폼 실행 결과](./c12-4-layout-depth-safety-platforms-2026-10-11.md)를 참조한다.

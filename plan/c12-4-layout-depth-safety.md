# C12.4 후속 · 실제 런타임 트리 깊이 안전성 계획

**상위 계획:** [C12.4 쌓임 맥락과 `z-index`](c12-4-stacking-context.md)  
**구현 상태:** 코드와 Android/iOS Simulator 관문 통과 · 메모리 비용·실기기 범위는 미확정 · 이 계획은 C12.4 완료를 뜻하지 않음  
**계획 실패 관점 검토:** [서로 다른 20개 검토](../spec/internal/evidence/c12-4-layout-depth-safety-plan-review-2026-10-11.md)
**구현 실패 관점 검토:** [별도 20개 검토](../spec/internal/evidence/c12-4-layout-depth-safety-implementation-review-2026-10-11.md)  
**플랫폼 근거:** [Android API 37 AVD·iOS 26.2 Simulator 실행 결과](../spec/internal/evidence/c12-4-layout-depth-safety-platforms-2026-10-11.md)

## 문제와 현재 근거

`TaffyLayoutEngine`의 Block layout 경로는 각 자식의 layout 계산을 재귀 호출한다. worker의 panic 포착은 Rust stack overflow로 인한 프로세스 abort를 복구하지 못한다. 현재 일반 테스트 스레드의 기본 설정에서 256-depth graph가 layout 호출 중 프로세스를 종료한 실행이 있다. 실제 runtime의 CSS cascade/layout은 `spinon-css-cascade` worker에서 실행한다.

C12.4 deep-graph fixture를 제품 runtime과 같은 `new_html_runtime_mount_shared` 경로로 바로잡았다. 이 경로에는 Stylo cascade용 합성 `html`·`body`가 포함된다. Android API 37 AVD와 iPhone 17 Pro / iOS 26.2 Simulator 양쪽에서 8 MiB `spinon-css-cascade` worker의 실제 V8→HostDocument→Stylo→Taffy 경로를 실행했다. HostDocument depth 1,024에서 cascade style·layout frame·paint box 1,024개가 생성됐고, 같은 runtime에 1,025번째 node를 붙인 요청은 `tree_depth_exceeded`로 거부됐다. 초과 subtree를 제거해 새 document revision을 제출한 뒤 기존 worker가 얕은 장면을 다시 계산했다. 4 MiB 지정은 이전 fragment-view 실행에서 stack overflow가 났다. 현재 증거는 두 시뮬레이터의 앱 실행 결과다. Android 실기기, 여러 runtime session의 누적 메모리 비용, 일반적인 1,024-depth 성능은 확인하지 않았다.

## 구현 결정과 범위

- 현재 Rust Taffy 재귀 경계의 프로세스 안전을 위해 CSS runtime worker에 8 MiB stack을 지정한다. 이 크기는 host와 두 시뮬레이터에서 1,024-depth 실제 runtime-mount cascade/layout 경로가 통과한 현재 확인값이며, 플랫폼 최소값이나 메모리 비용이 승인된 최종값으로 간주하지 않는다.
- runtime layout 입력은 각 HostRoot 직속 subtree에서 실제 `HostDocument` node depth를 root 포함 1,024까지 허용한다. HTML runtime mount의 합성 `html`·`body`는 고정된 cascade wrapper이며 HostDocument depth에는 포함하지 않는다. 실제 runtime-mount test에서 wrapper 경로까지 검증한다.
- 깊이 제한 확인은 author CSS 파싱·Stylo cascade 전에 반복 순회로 수행한다. 1,025번째 node를 처음 발견하면 cascade/layout을 시작하지 않고 해당 node를 식별해 `tree_depth_exceeded`로 실패시킨다. 직접 `TaffyLayoutEngine` 입력도 같은 한계에서 Taffy 재귀 호출 전에 실패시킨다.
- 제한은 CSS 의미가 아니라 현재 recursive layout 구현의 임시 자원 한계다. 1,024를 넘는 트리를 성공 또는 전체 호환으로 표시하지 않는다. 제한 완화는 stack 측정과 재귀 제거/의존성 경로 변경을 별도 근거로 검토한다.
- 깊이 초과 요청은 이전 revision 결과를 남기지 않는다. cascade와 layout 상태를 모두 실패로 만들고, layout 오류 JSON의 기존 `{code,nodeId,property}` 필드를 유지한다. `code`는 `tree_depth_exceeded`, `nodeId`는 첫 초과 node, `property`는 `null`이다. 새 JSON 필드를 추가하지 않는다.
- 오류 뒤 새 shallow revision을 등록하면 기존 runtime worker가 다시 계산할 수 있어야 한다. worker 종료·복구 계약이나 JS API를 추가하지 않는다.

## 검증 게이트

1. [x] 단위 경계: HostDocument depth 1, 1,024 성공, 1,025 명시 거부. 넓고 얕은 1,025-node graph는 depth 제한에서 거부되지 않는다.
2. [x] direct layout 경계: 수동 `LayoutInput`의 1,025-depth graph가 `TreeDepthExceeded`를 반환한다. 같은 node 수의 넓은 graph는 layout 성공한다.
3. [x] 실제 runtime worker: 8 MiB `spinon-css-cascade` worker에서 1,024-depth runtime-mount document의 computed style·layout frame·paint box 전체 개수와 최심부 node를 확인한다. 1,025 요청 거부 뒤 HostDocument에서 깊은 subtree를 제거하고 증가한 document revision을 같은 worker에 제출해 2-node shallow 결과를 다시 계산한다.
4. [x] 플랫폼 시뮬레이터: Android API 37 AVD(`emulator-5554`, Android 17, arm64-v8a)와 iPhone 17 Pro / iOS 26.2 Simulator에서 실제 앱 V8 경로의 1,024 성공·1,025 거부·후속 회복을 확인했다. Android 실기기에서는 실행하지 않았다.
5. [x] 회귀: 새 Android API 37 AVD 앱에서 기존 C12.4 26-scene matrix가 26/26 일치했다. 전체 Rust workspace, Clippy, rustfmt, CSS reference 170개를 재실행했다.
6. [x] Simulator session 비용: iOS 26.2 Simulator과 Android API 37 AVD에서 0·1·2·4·8 C12.4 session을 누적 생성·해제하고 stage별 process memory를 세 번 수집했다. CSS worker의 8 MiB/session stack 요청과 process-wide PSS/RSS/VmSize/footprint를 분리했다. [측정 근거](../spec/internal/evidence/c12-4-concurrent-session-stack-cost-2026-10-11.md).
7. [ ] 실기기·thread별 귀속: Android 실기기 RSS, thread별 stack page commitment, 8 MiB 요청의 실제 mapping/commit 비용은 미측정이다. 8 MiB·1,024 경계를 모든 기기의 최종 보장으로 승격하지 않는다.

## 계획 실패 검토

계획 검토에서 합성 runtime wrapper 누락, direct-layout 우회, 깊이와 넓이의 혼동, stale 결과 잔류, JSON schema 변경, 모든 profile로 인한 범위 확대, stack 메모리 비용, 플랫폼 stack 차이, text node 포함 방식 및 recovery 누락을 확인했다. deep graph 테스트는 실제 runtime mount로 바꿨고 depth/node-count 대조, 기존 JSON 필드 유지, 동일 HostDocument의 새 revision 복구를 추가했다. Android·iOS Simulator 경계는 통과했지만 실제 기기와 누적 stack 메모리 비용이 남아 있어 8 MiB와 1,024는 잠정적인 제한값이다.

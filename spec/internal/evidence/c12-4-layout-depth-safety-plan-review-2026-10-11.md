# C12.4 runtime tree depth 안전성 계획 검토

**대상 계획:** [실제 runtime 트리 깊이 안전성](../../../plan/c12-4-layout-depth-safety.md)  
**기준 코드:** `crates/spinon-layout/src/compute.rs`, `crates/spinon-layout/src/calc_tree.rs`, `crates/spinon-runtime/src/session/ua_cascade.rs`  
**비교 입력:** C12.4 deep graph fixture, Android API 37 AVD, iOS Simulator  
**상태:** 계획 검토 완료 · 구현 후 별도의 코드·runtime 검토 필요

## 서로 다른 실패 관점 20개

1. **stack overflow 복구 착각:** `catch_unwind`가 Rust stack overflow abort를 복구한다고 오해할 수 있다. stack 안전성은 panic 변환에 의존하지 않고 제한 전 검증한다.
2. **재귀 호출 위치 오인:** HostDocument 변환만 반복형이라 layout도 stack-safe하다고 오해할 수 있다. 고정 Taffy 소스의 `compute_block_layout → perform_child_layout → compute_cached_layout` 경계를 근거로 둔다.
3. **테스트와 제품 view 불일치:** fragment view는 실제 앱의 합성 `html`·`body` cascade wrapper 두 개를 생략한다. deep graph fixture를 `new_html_runtime_mount_shared`로 수정했고 128/default 및 1,024/8 MiB 경로를 재실행했다.
4. **기본 stack 크기 단정:** host test runner의 기본 stack을 모든 OS thread의 고정 크기로 일반화할 수 없다. 플랫폼 값은 Android/iOS Simulator에서 별도 계측한다.
5. **`RUST_MIN_STACK` 과잉 일반화:** test thread 설정이 생성자 worker와 동일한 플랫폼 설정이라는 증거는 없다. 실제 coordinator worker에서 계산하는 integration test를 둔다.
6. **8 MiB 최소값 주장:** 8 MiB에서 한 host fixture가 통과한 사실은 최소 요구량이 아니다. 계획은 이를 가장 작은 현재 성공 측정값으로만 설명하며, 플랫폼 확인 후 재평가한다.
7. **per-session 비용 누락:** CSS worker마다 stack 설정이 예약되면 동시 session 수에 따라 비용이 늘어난다. 플랫폼별 worker 수와 메모리 변화를 기록한다.
8. **무제한 재귀 잔존:** 1,024 이후에도 Taffy 재귀가 무제한이면 프로세스 abort가 재발할 수 있다. 1,025는 Taffy 진입 전 거부한다.
9. **가짜 지원 표기:** depth cap은 CSS 규격 의미가 아니라 구현 자원 한계다. 명세에 임시 제한으로 표시하고 C12.4/전체 CSS 호환 완료와 분리한다.
10. **검증 순서 문제:** depth 검사를 Stylo cascade 뒤에 하면 과도하게 깊은 입력에서 cascade가 먼저 실패할 수 있다. runtime admission 검사 순서를 author CSS 수집과 cascade보다 앞에 둔다.
11. **직접 엔진 우회:** runtime preflight만 두면 `TaffyLayoutEngine` 직접 호출이 재귀 위험을 남긴다. `LayoutInput`/engine 검증에서도 동일 제한을 적용한다.
12. **root 포함 오차:** 1,024를 root 제외 자식 수로 해석하면 경계가 한 node 어긋난다. root를 depth 1로 정의하고 1,024/1,025 테스트를 둔다.
13. **node 수와 depth 혼동:** 1,025개의 형제는 depth 2인데 거부될 수 있다. 넓고 얕은 대조를 두고 depth만 검사한다.
14. **여러 HostRoot 누락:** 첫 root만 검사하면 다른 root의 과도한 subtree가 계산 경계에 도달할 수 있다. 모든 root를 document 순서로 검사하고 첫 초과 node를 결정적으로 반환한다.
15. **text node 의미 혼동:** 텍스트를 제외하면 Stylo DOM 깊이를 실제보다 작게 셀 수 있다. runtime preflight는 HostDocument node kind와 무관하게 subtree 깊이를 센다. layout 단계의 text 지원 여부는 별도 계약을 따른다.
16. **합성 node 비용 누락:** HostDocument depth와 Stylo effective depth는 같지 않다. 고정 합성 wrapper 수를 문서화하고 실제 runtime-mount fixture에서 cascade stack을 검증한다.
17. **오류 JSON 파손:** 새 `message`나 `depth` 필드를 추가하면 기존 `spinon.runtime.layout` 소비자를 바꾼다. 기존 3개 오류 필드를 유지하고 안정적인 새 code/nodeId만 사용한다.
18. **부분·stale 결과 노출:** 초과 거부 시 직전 revision의 frame/paint snapshot이 남을 수 있다. 요청 key 교체 때 이전 결과 제거와 새 `tree_depth_exceeded` 결과를 확인한다.
19. **오류 뒤 영구 정지:** 과도한 요청이 worker를 죽이면 이후의 작은 document도 멈출 수 있다. 초과 거부 뒤 shallow revision 성공을 회복 검증으로 둔다.
20. **플랫폼 일반화:** Android AVD software renderer 및 iOS Simulator는 실기기 stack·메모리·GPU 성능을 증명하지 않는다. Android는 AVD만 사용하고, 결론은 simulator 범위로 한정한다.

## 계획에 반영한 수정

- 실제 runtime mount를 쓰지 않던 deep graph fixture를 수정했다.
- Taffy 직접 입력과 runtime cascade 전 admission 양쪽에 경계 검증을 둔다.
- 1,024/1,025 경계, 넓은 tree 대조, 오류 뒤 shallow recovery, 양 simulator 및 메모리 계측을 검증 게이트에 추가했다.
- JSON schema와 공식 지원 범위를 유지하고, 8 MiB·1,024 경계를 모바일 검증 전 잠정값으로 명시했다.

## 관찰된 기준 실행

- 128 HostDocument node, 실제 runtime mount, default test thread: HostDocument→snapshot→Stylo cascade→Taffy layout→paint snapshot 성공.
- 1,024 HostDocument node, 실제 runtime mount, `RUST_MIN_STACK=8388608`: 전체 경로 성공. 단일 Debug test이며 benchmark가 아니다.
- 1,024 node, 이전 fragment-view fixture, `RUST_MIN_STACK=4194304`: process stack overflow. 이는 runtime worker 또는 모바일 stack 측정이 아니다.

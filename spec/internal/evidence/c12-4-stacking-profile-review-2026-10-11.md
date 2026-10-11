# C12.4 혼합 profile 계약 검토

검토 대상은 구현 전 `RuntimeBlockFlexStackingV1` 계약과 그 연결 경계다. 코드를 구현하거나 iOS/Android runtime을 실행한 검증이 아니다. 각 항목은 서로 다른 입력·소유권·실패 경로를 확인한다.

| # | 실패 관점 | 검토 결과와 계약 조치 |
|---:|---|---|
| 1 | Block-only와 Flex-only 계산물을 합쳐 혼합 document처럼 보이게 하는가 | 안 된다. 새 profile은 HostDocument 전체의 단일 Stylo cascade와 같은 revision의 Taffy layout을 입력으로 받는다. |
| 2 | 새 profile이 기존 profile의 결과나 cache를 재사용해 이전 cascade 의미를 섞는가 | 초기 구현은 full cascade만 쓴다. 새 profile을 incremental reuse whitelist에 추가하지 않는다. |
| 3 | layout 결과의 style/environment revision이 computed snapshot과 달라진 채 painter에 도달하는가 | cascade, layout projection, style-to-render가 같은 document/style/environment key를 각각 검증하며 불일치는 전체 오류다. |
| 4 | profile tag를 잘못 붙인 Block 또는 Flex snapshot을 runtime renderer가 받는가 | cascade에서 renderer admission까지 `RuntimeBlockFlexStackingV1`을 검사하고 이전 profile은 거부한다. |
| 5 | z-index 문자열을 재파싱하거나 값 오류를 0으로 바꾸는가 | Stylo 0.22.0의 `ComputedValues::clone_z_index()` typed `Auto | Integer(i32)` 결과만 복사한다. 잠금된 source의 `CSSInteger = i32`를 사용하고 raw CSS 정수 parser를 새로 만들지 않는다. |
| 6 | `z-index:var(...)`·등록 사용자 지정 속성 결과가 layer 처리에서 사라지거나 미해결 문자열로 painter에 전달되는가 | `@property`와 custom property를 Stylo computed cascade로 계산하고, `@layer` statement/block은 열되 nested declarations를 같은 allowlist로 검사한다. |
| 7 | `@media`, `@supports`, nested style rule, 외부 stylesheet 또는 알 수 없는 at-rule이 허용목록 확장으로 통과하는가 | 전용 policy는 layer 및 property registration만 허용한다. 나머지 규칙 형태는 계속 거부한다. |
| 8 | selector에서 이기지 않은 미지원 declaration이 source 검사 우회로가 되는가 | source preflight는 cascade winner를 추정하지 않고 미지원 declaration을 보수적으로 거부한다. |
| 9 | inline style이 author stylesheet와 다른 allowlist를 사용해 unsupported property를 넣는가 | inline과 author declaration은 같은 profile property allowlist 및 Stylo diagnostics gate를 거친다. |
| 10 | `visibility:hidden`이 부모에서 상속되거나 `var()`로 생겨 보이지 않는 box가 scene에 포함되는가 | 모든 computed element의 typed visibility가 `Visible`인지 확인하며 `Hidden`·`Collapse`면 scene 전체를 거부한다. |
| 11 | `visibility:hidden`을 `display:none`과 동일 처리해 자식 box 계산 계약을 섞는가 | `display:none`은 기존 zero-frame/subtree omission 규칙을 유지하고 hidden/collapse는 새 profile에서 미지원 오류로 구분한다. |
| 12 | `display:contents`, inline, Grid 같은 box tree를 요소 node 하나와 동일하게 painter에 넣는가 | `block|flow-root|flex|none`만 허용하고 inline fragment, Grid, `display:contents` 등은 diagnostic으로 거부한다. |
| 13 | static Flex item의 non-auto z-index를 일반 static Block처럼 무시하는가 | box가 direct in-flow Flex item인지 실제 formatting parent로 판정해 CSS Flex stacking 예외를 적용한다. |
| 14 | static Block의 non-auto z-index가 실제 stacking context 또는 stack level을 잘못 만드는가 | static Block에는 stack-level 효과를 주지 않는다. 실제 context 여부는 typed position, box role 및 z-index 조합으로 결정한다. |
| 15 | `position:fixed; z-index:auto`가 auto pseudo-context로 잘못 분류되는가 | fixed는 auto에서도 actual context로 분류한다. C12.3 geometry owner는 같은 profile input으로 유지한다. |
| 16 | `position:relative|absolute; z-index:auto`가 실제 context처럼 자손 context를 격리하는가 | auto는 pseudo-context로 기록하고 자손 stacking context가 부모 context phase에 참여하도록 한다. |
| 17 | transform·opacity·filter·contain·blend 같은 context trigger가 allowlist 외부에서 computed effect로 살아남는가 | author declaration allowlist와 computed diagnostics를 함께 적용한다. trigger를 무시하거나 scene 생성 뒤 일부 box만 제거하지 않는다. |
| 18 | root margin이 Flex 전용 profile과 Block positioning profile 사이에서 조용히 의미가 바뀌는가 | 새 혼합 profile은 Block positioning의 viewport-root margin 동작을 유지하고 nonzero root margin을 새로 금지하지 않는다. Flex item auto margin은 기존 Flex 규칙대로 별도 계산한다. |
| 19 | 깊은 tree, rank overflow, 명시적 reserve 실패가 panic 또는 부분 `RuntimeRenderSnapshot`을 만드는가 | traversal은 반복형이며 `try_reserve`·`u32` 변환 오류는 snapshot 생성 전에 반환한다. 전역 OOM 복구는 보장하지 않는다고 계약 범위를 한정한다. |
| 20 | ephemeral context staging을 renderer가 다시 정렬하거나 scene에 누출해 snapshot consumer마다 의미가 달라지는가 | staging은 style-to-render 내부에서만 사용하고 flat contiguous paint list를 내보낸다. WGPU는 순서만 소비하며 CSS 정렬은 복제하지 않는다. |

## 판정

문서 계약에서 막을 수 있는 20개 경계를 반영했다. Stylo 0.22.0 고정 소스의 typed z-index/visibility accessor, cascade allowlist와 기존 runtime 연결점을 확인했다. `@layer`·`@property` 결합 policy 및 새 profile은 아직 코드로 검증하지 않았고, Android 실기기/iOS Simulator 실행도 아직 시작하지 않았다. 다음 구현 검토는 실제 변경 코드와 실행 결과를 대상으로 별도의 20개 실패 관점으로 작성한다.

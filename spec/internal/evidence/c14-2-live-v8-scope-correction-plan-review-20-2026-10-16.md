# C14.2 live V8 plan 분모 정정 재검토 20개 — 2026-10-16

검토 대상은 [live V8 Simulator 검증 계획](../../../plan/c14-2-live-v8-simulator-verification.md)과 Block inventory의 구현 범위 표시다. 첫 계획 검토 뒤 실제 inventory·C14.2 원 계획·Rust oracle을 다시 대조해 발견한 분모 불일치를 수정한 재검토다. 이전 20개 표의 결론을 재사용하지 않는다.

| # | 독립 실패 관점 | 재검토 기준과 결과 |
|---:|---|---|
| 1 | 27개 Block browser row와 21개 구현 case를 같은 집합으로 혼동한다 | inventory는 natural PNG row 22개, C14.2 구현/Rust oracle 집합은 21개라고 계획에 분리했다. 통과 |
| 2 | PNG가 있다는 이유만으로 C14.2 미지원 `aspect-ratio:auto <ratio>`를 실행 분모에 넣는다 | `aspect-ratio-auto-override`를 제외하고 `layoutStage=unsupported_aspect_ratio_auto_with_fallback`으로 표시했다. 통과 |
| 3 | `aspect-ratio:auto`와 `aspect-ratio:auto <ratio>`를 같은 문법으로 취급한다 | natural ratio만 쓰는 행렬과 fallback ratio가 붙은 미지원 입력을 계획·inventory에서 구분했다. 통과 |
| 4 | 계획의 분모와 실제 C14.2 Rust oracle 테스트 case 목록이 어긋난다 | 구현 전 fixture는 기존 21개 Rust/Chrome supported case와 ID·순서를 맞추고 Flex 8개를 별도 유지한다. 통과 |
| 5 | excluded row의 Chrome 관측 결과를 삭제하거나 바꿔 기존 oracle provenance를 훼손한다 | `expectedCssPx`와 Chrome observation은 그대로 두고 inventory에 제외 사유만 추가했다. 통과 |
| 6 | 하나의 제외 표식을 이용해 지원 대상의 기존 21개 행까지 제거한다 | 현재 inventory에서 scope marker가 붙은 ID는 `aspect-ratio-auto-override` 하나이며 21개 배열과 직접 대조한다. 통과 |
| 7 | case ID 중복 또는 순서 변경이 있어도 21개로만 계산한다 | 구현은 mapping ID/order를 inventory의 21개 미표시 case와 정확히 비교하고 observation 중복을 거부한다. 통과 |
| 8 | Flex 8개를 Block 분모 보정 과정에서 누락하거나 중복한다 | Flex inventory와 관측은 그대로 8개이며 총 실행 분모는 29개로 유지한다. 통과 |
| 9 | 원래 C14.2 계획의 명시적 fail-closed 계약과 후속 runtime gate가 충돌한다 | 원 계획의 `aspect-ratio:auto <ratio>` 거부를 후속 gate의 제외 근거로 연결했다. 통과 |
| 10 | 22개 PNG row로 분모를 올리면서 계산기 재구현이 범위에 몰래 추가된다 | 후속 작업은 이미 구현된 21+8 live V8 앱 경로 검증만 유지한다. 통과 |
| 11 | `aspect-ratio-auto-override` 행을 21개 행 중 하나로 대체해 CSS 2.2 분기를 누락한다 | 기존 21개 ID 순서를 oracle/test 집합과 일치시켜 row substitution을 거부한다. 통과 |
| 12 | Chrome observation에 `complete`인 실패 대조와 SVG를 양성 집합에 넣는다 | 구현 대상 21개 natural PNG는 기존 test IDs에 고정하고 SVG/실패 대조는 별도로 제외한다. 통과 |
| 13 | natural size `24×16`을 oracle이 아닌 synthetic 값만으로 확인한다 | 각 선택된 Chrome observation natural size와 고정 PNG metadata를 대조하도록 계획에 유지했다. 통과 |
| 14 | 19.984375 subpixel expected를 정수 20으로 바꾼다 | 원시 oracle 값을 보존하며 기존 0.05 CSS px tolerance를 유지한다. 통과 |
| 15 | Block frame의 x/y가 기준에 없어도 width/height만 비교한다 | 선택된 21개 image의 x/y/width/height 전체를 비교한다. 통과 |
| 16 | parent frame이 없는 Block observation에 가상의 containing-block oracle을 만든다 | Block image frame과 Flex 관측 parent size/image frame만 비교한다. 통과 |
| 17 | inventory 수정 때문에 고정 Chrome observation 버전/viewport/DPR가 바뀐다 | Chrome 155.0.8059.40, 1280×633 CSS px, DPR 1 기준을 유지한다. 통과 |
| 18 | 미지원 row의 이름이 inventory·계획·실행 ID 집합에서 달라진다 | 모든 문서에서 `aspect-ratio-auto-override`를 같은 제외 row로 연결했다. 통과 |
| 19 | C14.2, C14.1 또는 상위 C14의 체크 상태를 잘못 갱신한다 | 계획/상태의 C14.2와 상위 C14 체크는 미완료로 유지한다. 통과 |
| 20 | 이전 계획 검토가 정정 뒤 검증으로 잘못 재사용된다 | 이 표를 별도 20개 계획 재검토로 추가하고 PR diff 검토는 프로젝트 규칙에 따라 별도 30개로 기록한다. 통과 |

**판정:** 21+8의 기존 구현 범위와 22번째 inventory PNG row의 fail-closed 경계를 일치시켰다. 새로 측정한 Chrome oracle이나 기능 구현은 없다. 미해결 P1/P2가 없다.

# C13.1 전체 surface pixel-center 구현 실패 관점 검토 · 2026-10-14

구현 후 renderer 변경과 실제 Android/iOS 실행 결과를 대상으로 계획 검토와 별개의 20개 실패 경로를 점검했다. 수정 전 860개 mismatch를 고정 oracle에서 재현했고, 중간 적대 점검에서 `position:relative; z-index:auto`도 같은 순서 결함을 드러내 별도 회귀를 추가했다. 이 검토는 C13.1 전체 지원 판정이 아니다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | positioned escape sibling이 존재하면 부모 배경까지 root에 덮인다 | 수정 전 AVD capture에서 30,000점 중 860 mismatch를 재현했다. static child와 escape sibling을 함께 가진 Rust 회귀 및 최종 0 mismatch로 확인했다. **통과.** |
| 2 | `relative; z-index:auto`가 paint 후보에서 누락된다 | 이전 구현으로 새 Rust 회귀가 실패하는 것을 재현한 뒤 후보에 `Relative`를 포함했다. 수정 후 부모 배경보다 static child 순서가 뒤다. **통과.** |
| 3 | absolute auto 후보에서 자기 static subtree가 빠진다 | positioned-auto parent fixture가 static box의 배경/테두리 순서를 직접 단언하고 30,000점 oracle에서 수정 결과를 확인했다. **통과.** |
| 4 | fixed auto 후보만 기존 absolute 경로에 남아 회귀한다 | 후보 매치가 `Relative | Absolute | Fixed`를 포함한다. 같은 함수에서 fixed를 처리하는 코드 경로를 확인했다. fixture는 별도 fixed viewport 동작을 주장하지 않는다. **코드 경로 통과, fixed 독립 표면 fixture는 범위 밖.** |
| 5 | `z-index:0` 또는 숫자 stacking context까지 auto 그룹에 끌려간다 | 후보는 `ComputedCssZIndex::Auto`만 받는다. 숫자 stacking context가 별도 paint phase에 남는 Rust 회귀를 통과했다. **통과.** |
| 6 | static descendant를 부모보다 앞에 넣어 부모 배경이 자식을 지운다 | `paint_nodes`에서 descendant를 제거한 뒤 candidate 바로 뒤에 삽입한다. 부모→static child 순서를 검사한다. **통과.** |
| 7 | positioned sibling escape target이 static 자식과 함께 부모 뒤로 이동한다 | traversal은 static만 수집하고 positioned node에서 해당 subtree 탐색을 멈춘다. escape target이 static child보다 뒤인 회귀가 통과했다. **통과.** |
| 8 | escaped positioned child의 static 후손을 바깥 auto parent가 잘못 이동한다 | 바깥 후보는 positioned child에서 중단하고, 해당 positioned child가 자기 후보로 후손 group을 처리한다. nested fixture가 outer→static→escape→escape child 순서를 검사한다. **통과.** |
| 9 | positioned subtree 안의 non-auto stacking context가 static group에 섞인다 | 숫자 z-index subtree는 수집 경계에서 제외된다. 부모 static child보다 뒤의 별도 phase에 남는 assertion이 통과했다. **통과.** |
| 10 | `display:none` 아래 노드가 재배치돼 존재하지 않는 paint item을 만든다 | `display:none` node에서 traversal을 중단한다. 숨김 하위 트리를 포함한 실물 paint oracle의 별도 색 표본은 없으므로 코드 경계만 확인했다. **코드 경로 통과.** |
| 11 | parent map의 정렬 순서가 DOM child 순서를 대체한다 | child 목록을 `parent_by_node` map iteration이 아니라 `document_nodes` 순회로 구성한다. **통과.** |
| 12 | `NodeId` 숫자 정렬이 `InsertBefore`로 바뀐 tree order를 대체한다 | 테스트가 생성 후 node 3을 node 2 앞에 삽입해 ID 순서와 tree 순서를 다르게 만든다. 실제 `paint_nodes` 상대 순서 assertion이 통과했다. **통과.** |
| 13 | descendant 목록 정렬이 기존 paint order 자체를 정렬한다 | 정렬 목록은 membership 검색에만 쓰고 이동 목록은 현재 `paint_nodes` 순서로 구성한다. 역전된 tree order fixture에서 순서가 보존됐다. **통과.** |
| 14 | 대상 candidate가 paint list에 없는데도 잘못 splice한다 | descendant 이동 전 candidate 존재를 확인하고, 제거 이후 candidate를 다시 찾아 rank를 계산한다. 관련 `InvalidRoot` 경계를 코드에서 확인했다. **코드 경로 통과.** |
| 15 | 앞 candidate의 제거로 다음 candidate의 저장 rank가 stale해진다 | 고정 rank를 보관하지 않고 각 candidate 처리 뒤 남은 paint list에서 위치를 다시 찾는다. nested positioned/static group 순서 회귀를 통과했다. **통과.** |
| 16 | paint 순서 재배치가 hit-test topmost 순서를 바꾼다 | `hit_test_order`를 별도 산출하며 복합 회귀에서 positioned escape와 static child의 hit order를 각각 단언한다. Android 52/52 identity도 통과했다. **통과.** |
| 17 | static sibling overflow clip이 positioned escape sibling에 전파된다 | 복합 회귀가 escape sibling의 clip x가 비어 있는지 확인한다. 기존 visible/clip/hidden/scroll 네 overflow 모드 회귀도 유지됐다. **통과.** |
| 18 | child/style 입력 손상이 조용히 잘못된 순서를 만든다 | child의 parent mapping이 없으면 `InvalidRoot`, 계산 style이 없으면 `MissingComputedStyle`를 반환한다. 이 오류 분기는 직접 fault injection하지 않고 코드 경로로 확인했다. **코드 경로 통과.** |
| 19 | traversal·move vector 할당 실패가 panic 또는 부분 결과로 노출된다 | 확장되는 child/pending/descendant/moved vector가 `FallibleReserve` 경로를 사용하며 오류를 전파한다. allocator fault injection은 실행하지 않았다. **코드 경로 통과, 주입 시험 미실행.** |
| 20 | 고정 52점 또는 simulator 화면만으로 결함이 끝났다고 판단한다 | 수정 전/후 전체 30,000 CSS pixel-center RGBA를 전수 비교해 860→0을 확인했다. AVD는 SwiftShader, iOS는 Simulator 화면·query뿐이며 physical-pixel 전체, hardware, drawable readback은 미검증으로 남겼다. **범위를 제한해 통과.** |

증거: [수정 전/후 비교와 환경](README.md), [renderer 회귀](../../../../crates/spinon-style-to-render/src/stacking/tests/c13_1_paint_hit_border.rs), [paint ordering](../../../../crates/spinon-style-to-render/src/stacking/ordering.rs). 확장 지원, WPT, DOM event/CSSOM 또는 C13.1 완료를 주장하지 않는다.

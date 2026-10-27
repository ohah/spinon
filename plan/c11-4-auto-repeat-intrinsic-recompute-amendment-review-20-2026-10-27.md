# C11.4 auto-repeat intrinsic 재계산 범위 보정 · 계획 실패 관점 20개

검토 대상은 [집중 계획](./c11-4-auto-repeat-intrinsic-recompute.md)의 WPT neighbor grammar 보정이다. 구현 중 발견한 최초 실패는 runtime cascade 이전의 `grid-template-columns` style projection 거부였다. 아래 항목은 보정 범위만 독립적으로 검토한다. 기존 계획의 20개 검토를 대체하지 않는다.

| # | 별도 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 기준 WPT에 없는 문법을 근거 없이 추가할 수 있음 | **PASS.** 추가 허용은 고정 source의 trailing `minmax(min-content, 40px)`에서 직접 나온다. |
| 2 | 실패 지점이 style parser·cascade·layout으로 잘못 설명될 수 있음 | **PASS.** 재현은 runtime style projection의 `grid-template-columns` 거부에서 멈췄으며, 아직 track sizing 결과로 부르지 않는다. |
| 3 | 반복 fragment 내부 track까지 허용 범위가 바뀔 수 있음 | **PASS.** 추가 허용은 auto-repeat fragment 바깥의 trailing neighbor에만 한정한다. |
| 4 | 같은 intrinsic 형태를 prefix에 허용해 검증 입력이 넓어질 수 있음 | **PASS.** 반복 fragment 직후의 단일 trailing track만 허용하고 prefix 위치는 거부한다. |
| 5 | trailing 고정 track 뒤에 더 많은 track이 있어도 허용될 수 있음 | **PASS.** 전체 suffix 길이가 정확히 1이어야 한다. |
| 6 | row axis 또는 auto-fit까지 의도치 않게 활성화될 수 있음 | **PASS.** WPT가 쓰는 `grid-template-columns`의 auto-fill만 허용한다. |
| 7 | `minmax(auto, px)`를 같은 형식으로 취급할 수 있음 | **PASS.** minimum은 computed `MinContent`로 정확히 제한한다. |
| 8 | `minmax(max-content, px)` 또는 독립 max-content neighbor가 포함될 수 있음 | **PASS.** max-content minimum은 negative control로 거부한다. |
| 9 | 고정 px 상한 대신 percentage·calc·flex를 임의로 받아들일 수 있음 | **PASS.** maximum은 finite non-negative px만 받는다. |
| 10 | 음수·NaN·무한대 max 값이 repeat count 산술에 들어갈 수 있음 | **PASS.** 상한 검증은 유한성과 0 이상 조건을 함께 요구한다. |
| 11 | repeat count에서 max를 반영하지 않아 반복 개수가 과도해질 수 있음 | **PASS.** 사전 검증은 고정 max px를 neighbor 사용 공간으로 계산한다. |
| 12 | track sizing에서도 minimum이 max로 바뀌어 min-content 결과가 손실될 수 있음 | **PASS.** Taffy 입력에는 기존 computed min-content minimum과 fixed maximum을 보존한다. |
| 13 | max 고정값을 minimum으로 잘못 세어 count를 낮출 수 있음 | **PASS.** 허용 형식의 count bound는 명시적인 fixed maximum에서 가져온다. |
| 14 | 지원 grammar가 column property 오류 context를 잃을 수 있음 | **PASS.** projection 거부는 계속 `grid-template-columns`와 해당 node를 보고해야 한다. |
| 15 | projection만 고치고 layout 사전 검증은 계속 거부할 수 있음 | **PASS.** 계획의 acceptance gate가 projection과 layout preflight 양쪽을 요구한다. |
| 16 | layout validator의 기존 prefix/suffix 총합 overflow bound를 우회할 수 있음 | **PASS.** 추가 track도 기존 finite neighbor sum 및 track-list limit에 포함한다. |
| 17 | 정상 고정 neighbor 형태까지 새 조건 때문에 거부될 수 있음 | **PASS.** 기존 length, percentage, 제한 calc와 provenance 기반 minmax positive controls를 유지한다. |
| 18 | 오류 입력이 성공 입력으로 바뀌어 범위가 조용히 확대될 수 있음 | **PASS.** prefix, row, 다중 suffix, 다른 intrinsic breadth와 비유한 max의 negative controls를 추가한다. |
| 19 | WPT Chrome oracle나 simulator matrix가 dynamic runtime mutation 증거로 섞일 수 있음 | **PASS.** oracle, 같은-document runtime sequence, 독립 simulator scenes의 증거 경계를 기존 계획대로 분리한다. |
| 20 | 범위 보정이 공식 C11.4 완료나 `72/213` 승격으로 이어질 수 있음 | **PASS.** API/명세 완료 조건은 변하지 않으며 체크와 공식 수를 그대로 유지한다. |

**판정:** 범위 보정 관점 20/20 PASS. 제품 구현은 위의 단일 열 축 trailing shape와 negative controls를 함께 넣을 때만 진행한다. 구현 후에는 보정 검토와 별개인 새 runtime/implementation 20개 및 PR diff 30개 검토가 필요하다.

## Runtime 발견 뒤 revision 기준 재확인

첫 same-document 실행에서 `HostDocument::commit`의 render projection은 style attribute를 포함해 `render_tree_revision`을 증가시키는 것이 확인됐다. 계획과 regression은 document·render-tree·style revision이 각 mutation에서 모두 증가하는 조건으로 수정했다. 이에 따라 #7–#9의 revision·invalidation 경계를 targeted recheck했고, generation과 node handles는 동일하게 유지하는 요구를 보존했다.

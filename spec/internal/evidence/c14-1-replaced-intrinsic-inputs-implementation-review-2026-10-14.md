# C14.1 대체 이미지 자연 크기 구현 실패 경로 검토

## 검토 범위

2026-10-14 기준 `feat/c14-intrinsic-sizing`의 snapshot 생성, layout leaf 전달, Stylo preflight, revision 전파와 stale admission을 검토했다. 계획 검토 표를 재사용하지 않고 입력 손상·CSS 경계·캐시 revision 실패 경로를 각각 대조했다. 비교 oracle은 [사전 비교 기록](c14-1-replaced-intrinsic-inputs-precomparison-2026-10-14.md)의 agent-browser Chrome 155 관측이다.

## 20개 구현 실패 관점

| # | 공격 입력·불변 조건 | 확인 결과 |
| --- | --- | --- |
| 1 | 측정 owner의 `u64` revision이 최대값에 도달한다. | `checked_next()`는 `None`을 반환하고 wrap하지 않는다. `owner_revisions_increment_independently_and_fail_closed_at_capacity` 통과. |
| 2 | 다른 Tree/HostDocument source의 측정값을 현재 문서에 재사용한다. | 전체 `LayoutSourceRevision` 비교가 layout 진입과 Stylo preflight 양쪽에서 mismatch를 반환한다. 두 source mismatch 회귀 검사 통과. |
| 3 | 한 snapshot에 같은 NodeId가 두 번 들어온다. | iterator를 받는 fallible constructor가 `DuplicateIntrinsicSizeNode`로 거부한다. 회귀 검사 통과. |
| 4 | map key가 현재 layout tree에 없는 NodeId다. | generic engine과 선택 subtree preflight가 `UnknownIntrinsicSizeNode`로 거부한다. 회귀 검사 통과. |
| 5 | 측정 map이 자식이 있는 layout node를 가리킨다. | Taffy tree를 만들기 전에 leaf가 아닌 입력을 거부한다. 회귀 검사 통과. |
| 6 | 자연 너비·높이가 0, 음수, NaN 또는 무한대다. | 축별 값 검증이 `InvalidIntrinsicSize`를 반환한다. 네 경계값 회귀 검사 통과. |
| 7 | visible HTML image 양 축이 auto이고 크기가 `24×16 CSS px`다. | leaf 측정값이 frame `24×16`과 measurement revision stamp에 반영된다. HostDocument→Stylo→Taffy 비교 통과. |
| 8 | block-level replaced image의 auto 폭이 일반 Block stretch로 부모 폭에 덮인다. | 첫 실행의 `120×16` 실패를 재현했다. 측정 entry가 있을 때 Taffy `item_is_replaced`를 켜고, 재검사에서 `24×16`으로 수정됐다. |
| 9 | visible auto/auto image에서 decoder 측정값이 누락된다. | 기존 runtime projection 경로도 0 크기 성공으로 숨기지 않고 `MissingIntrinsicSize`를 반환한다. 회귀 검사 통과. |
| 10 | CSS width는 definite, height만 auto인 image다. | natural ratio 처리를 가장하지 않고 `UnsupportedIntrinsicSizing`으로 거부한다. 회귀 검사 통과. |
| 11 | CSS width만 auto이고 height는 definite인 image다. | 같은 partial-auto 경계를 별도 CSS 입력으로 거부한다. 회귀 검사 통과. |
| 12 | 양 축 auto와 author `aspect-ratio`가 함께 들어온다. | ratio precedence가 구현된 것처럼 처리하지 않고 node가 포함된 오류를 반환한다. 회귀 검사 통과. |
| 13 | cached natural size가 있어도 CSS 양 축이 모두 definite다. | 측정 map entry를 Taffy leaf/`item_is_replaced`에 전달하지 않아 고정 `48×32`가 유지된다. 회귀 검사 통과. |
| 14 | 이미지 자신 또는 조상이 `display:none`이며 측정값이 없다. | 누락 크기 오류 없이 계산하고 hidden frame을 0으로 처리한다. 회귀 검사 통과. |
| 15 | 자연 콘텐츠 크기에 padding·border가 이미 포함됐거나 두 번 더해진다. | 측정값은 콘텐츠 크기로 전달하고 Taffy가 edge를 한 번 더한다. `24×16` 콘텐츠와 edge 합계 `30×22` 회귀 검사 통과. |
| 16 | min/max 제약이 natural size와 충돌한다. | 기존 Taffy clamp 경로를 유지해 `24×16`이 지정된 `20×20` 제약 결과가 된다. 회귀 검사 통과. |
| 17 | Flex item의 auto basis가 natural size를 잃는다. | 같은 measurement closure가 Flex sizing 단계에 전달돼 결과가 `24×16`이다. 회귀 검사 통과. |
| 18 | snapshot이 HTML image가 아닌 일반 요소의 ID를 포함한다. | Stylo adapter가 `UnexpectedIntrinsicSizeNode`로 거부한다. 회귀 검사 통과. |
| 19 | snapshot entry는 문서에는 있지만 선택한 root subtree 밖에 있다. | 선택 layout node 집합과 대조해 `UnknownIntrinsicSizeNode`로 거부한다. 회귀 검사 통과. |
| 20 | 새 measurement revision이 왔는데 이전 layout output을 현재 render로 승인하거나 일반·stacking runtime key에서 잃는다. | `CurrentLayoutInputs` mismatch가 scene 생성을 거부한다. 두 runtime key 경로가 measurement revision을 보존하고 문서 revision 재바인딩도 값을 유지한다. 회귀 검사 통과. |

## 검토 중 발견해 수정한 결함

- Taffy `0.14.0` Block 경로가 auto-width block child에 부모의 known width를 전달해 첫 결과가 `120×16`이었다. 고정된 Taffy의 replaced-item 경로를 확인하고 측정된 leaf에만 `item_is_replaced`를 설정해 natural width를 보존했다.
- `BTreeMap`을 받는 생성 API에서는 중복 ID가 constructor에 오기 전에 사라질 수 있었다. iterator 기반 fallible constructor와 중복 ID 오류를 추가했다.
- inventory 초안은 DPR 2와 DPR 불변성을 주장했지만 agent-browser 증거는 DPR 1뿐이었다. device scale factor를 `[1]`, `dprInvariant`을 `false`로 좁히고 reference의 inventory SHA-256과 ID를 갱신했다.
- 첫 구현 시 generic `LayoutInput`에 측정 필드를 붙여 직접 struct literal 다수가 깨졌다. generic input 모양을 되돌리고 immutable measurement snapshot을 계산 인자로 분리했다.
- Stylo preflight가 필요한 image 측정 누락을 먼저 반환해 잘못된 non-image map entry를 가릴 수 있었다. map node/type 검증 후 필요한 image key를 확인하도록 순서를 바꿨다.

## 판정과 남은 범위

이 20개 관점에서 남은 P1/P2 결함은 없다. C14.1의 내부 입력 경계와 Chrome 155 단일 fixture는 확인했지만 decoder/U05 owner, resource decode/cache, 앱 runtime 공급 경로, simulator 앱 경로, Chrome 154 비교, DPR 2, WPT 및 GPU queue stale 폐기는 검증하지 않았다. 따라서 C14.1·C14·U05·C15·X16 완료를 주장하지 않는다.

# C05.4 구현 계획의 적대적 검토

검토 대상은 [`c05-runtime-incremental-restyle.md`](../../../plan/c05-runtime-incremental-restyle.md)다. 각 관점에서 cache 적중을 잘못 허용하는 경우, 최신 revision을 놓치는 경우, 성능 범위를 부풀리는 경우를 찾고 계획에 차단 조건을 반영했다.

| # | 공격 관점 | 발견 가능성 | 계획 반영 |
|---:|---|---|---|
| 1 | author stylesheet만 확인하고 늘 등록되는 내장 UA 규칙을 놓치는가 | 계획 초안은 `compute_cascade`가 매번 추가하는 UA stylesheet를 빠뜨렸다. | 내장 UA profile을 명시하고 selector가 변하지 않은 element namespace/tag만 참조하는 invariant test를 요구했다. attr·state·관계 selector나 다른 profile은 부분 계산을 거부한다. |
| 2 | `<style>` 요소 속성 변경을 일반 inline style 변경으로 잘못 분류하는가 | 연결 `<style>` 입력은 별도 스타일 source에 영향을 준다. | 연결 stylesheet source가 있는 snapshot은 무조건 전체 cascade로 되돌린다. |
| 3 | namespace가 있는 `style` 속성도 HTML style로 취급하는가 | namespace가 다른 속성은 별도 입력이다. | namespace 없는 속성만 후보로 두고 namespace 있는 속성 변경은 전체 무효화한다. |
| 4 | HTML 대소문자 표기와 중복 `STYLE` 속성에서 다른 값을 누락하는가 | HTML 이름은 ASCII 대소문자를 구분하지 않고 중복 표기는 모호하다. | 후보 이름은 ASCII 대소문자 무시로 찾으며 중복·모호한 이름은 부분 계산을 거부한다. |
| 5 | `class`, `id`, `hidden`, 기타 속성 변경으로 selector 결과가 달라져도 cache hit하는가 | 임의 author selector가 영향을 받을 수 있다. | `style` 이외 속성 변경은 모두 전체 cascade fallback이다. |
| 6 | `:nth-child`, `:empty`, sibling, ancestor selector가 구조 변화에 따라 바뀌는가 | 연결·분리·이동·순서 변화가 선택자 집합과 inherited context를 바꾼다. | NodeId 집합, 부모·자식 순서 또는 root가 달라지면 전체 계산한다. |
| 7 | text 변경으로 `:empty` 결과가 바뀌는가 | 텍스트 변화는 선택자와 실제 렌더 내용에 영향을 줄 수 있다. | text node data 변경은 전체 cascade fallback이다. |
| 8 | `@property` 등록이나 `inherits` 변화가 stylesheet 없는 가정 밖에서 누락되는가 | 등록 속성의 상속·초깃값이 branch 전체에 영향을 준다. | 모든 stylesheet source 부재와 동일 profile을 함께 요구한다. |
| 9 | viewport 수치는 같아도 media/environment revision이 달라진 요청을 재사용하는가 | device scale·환경 입력이 스타일 결과를 바꿀 수 있다. | 전체 revision tuple과 viewport/media 값이 모두 일치하지 않으면 전체 계산한다. |
| 10 | generation 변경 뒤 재사용된 NodeId가 이전 문서 값과 섞이는가 | 숫자 NodeId만 비교하면 새 문서 노드와 충돌할 수 있다. | generation을 cache key와 invalidation source 양쪽에서 검증한다. |
| 11 | NodeId 출력은 재사용하면서 이전 `DocumentRevision`을 새 완료 결과에 남기는가 | 값이 같아도 상위 결과 stamp는 최신이어야 한다. | 이전 `ComputedElementStyle` 항목만 재사용하고 snapshot revision stamp는 새 요청에서 만든다. |
| 12 | 부모의 inherited custom property가 바뀌었는데 자손만 갱신하는가 | `var()` 결과가 임의 깊이로 전파된다. | 변경 요소 전체 하위 트리를 dirty로 두고 ancestor context를 root부터 다시 계산한다. |
| 13 | 겹치는 dirty root가 중복 계산을 일으키거나 빠뜨리는가 | 부모와 자손이 함께 바뀔 수 있다. | 현재 snapshot에서 조상 관계를 확인해 가장 바깥 dirty root만 남긴다. |
| 14 | pending request가 덮일 때 중간 dirty root를 잃는가 | worker 실행 중 여러 request가 병합될 수 있다. | revision 경계가 이어질 때만 dirty root를 합치며 단절·full mutation은 전체 무효화로 승격한다. |
| 15 | worker cache base와 pending change source가 다른데 부분 재사용하는가 | 실패·superseded request로 cache 기준이 요청 이력보다 뒤처질 수 있다. | source revision이 cache base와 다르면 부분 계산을 거부한다. |
| 16 | 오래된 in-flight 계산이 최신 worker style cache를 덮어쓰는가 | 계산 완료 순서와 등록 순서가 다를 수 있다. | 최신 결과 검사를 통과하지 못한 계산은 partial cache를 갱신하지 않는다. |
| 17 | cascade 중 실패·panic이 일부 cache를 덮어 이후 결과를 오염시키는가 | dirty subtree 중간 상태가 다음 요청의 기준이 될 수 있다. | 임시 계산이 완전히 성공한 뒤에만 cache를 교체하고 실패·panic이면 cache를 무효화한다. |
| 18 | layout 실패 뒤 cascade 성공 출력이 보관되어 오류 결과가 성공처럼 보이는가 | CSS 결과와 Taffy 결과는 성공 범위가 다를 수 있다. | cascade 출력 cache와 layout 상태를 분리하고 layout 오류를 유지한다. |
| 19 | 재사용된 node의 이전 CSS 진단이 남거나 새 진단 순서가 틀리는가 | 이전 revision diagnostics를 그대로 복사하면 stale 오류가 남는다. | 현재 view에서 현재 traversal 순서대로 diagnostics를 다시 수집한다. |
| 20 | cascade 계산 횟수 감소를 frame 성능이나 전체 앱 속도로 부풀리는가 | DOM view, Taffy, paint는 전체 입력을 계속 처리한다. | 계산 호출 수와 cascade/layout/scene/요청 시간을 분리하고 frame 성능을 단정하지 않는다. |

## 반영 뒤 판단

부분 계산을 author stylesheet가 없는 단일 연결 tree의 namespace 없는 HTML `style` 속성 변경으로 한정했다. 고정 UA profile의 type/namespace selector 조건도 검사한다. 조건이 입증되지 않으면 전체 cascade를 사용한다. 스타일 출력 cache는 최신 결과에만 갱신하고 revision 출처가 어긋나면 전체 계산한다. 최종 구현 검토는 이 계획 검토와 다른 20개 코드·실패 경로를 검사한다.

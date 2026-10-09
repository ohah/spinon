# C04.8 · fragment root 의미 계획 보정 검토

**대상:** [C04.8 계획](../../../plan/c04-runtime-ua-cascade.md), [내부 계약](../0029-c04-runtime-ua-cascade.md)
**계기:** 최초 runtime cascade 비교 테스트에서 앱의 HostRoot 직속 `<span>`이 Stylo 가상 document element로 처리되어 `display: block`이 반환됨. 같은 root의 inline `display` 선언도 blockified된 결과를 보임. Chromium C01 UA 기준은 `<span>`에 `display: inline`을 반환한다.
**범위:** 구현 전에 fragment-root 의미, 기존 문서-root adapter 호환, root별 UA cascade와 상속 경계 점검. 이 기록은 아직 기능 구현 검토가 아니다.

| # | 독립 실패 관점 | 검토 결과와 반영 |
|---:|---|---|
| 1 | HostRoot 직속 `<span>`이 `documentElement`로 인식돼 computed `display`가 block으로 바뀌는가 | 재현 테스트에서 불일치 확인. C04.8은 fragment-root Stylo adapter를 써 앱 root에 CSS root blockification을 적용하지 않는다. |
| 2 | root의 inline `style="display:inline"`도 root blockification으로 UA 값에 덮이는가 | 재현 테스트에서 같은 불일치 확인. fragment root는 일반 element computed value를 반환하고 inline author origin을 유지한다. |
| 3 | 앱 root가 CSS `:root` 선택자에 잘못 매치되는가 | fragment-root 모드에서는 선택한 HostNode가 CSS root가 아니며 `:root`에 매치되지 않도록 계약했다. |
| 4 | virtual document wrapper가 CSS cascade parent처럼 기본 style이나 inheritance를 추가하는가 | 가상 wrapper는 traversal 경계로만 남기고 UA 요소·선택자·추가 상속 값을 만들지 않는다. |
| 5 | HostRoot 아래 서로 다른 top-level Element 사이 descendant/sibling selector 관계가 생기는가 | 각 Element를 격리된 fragment scope로 계산한다. 다른 root를 selector 관계에 포함하지 않는다. |
| 6 | 한 top-level root의 상속 값이 옆 root에 전파되는가 | 각 root가 별도 계산 view를 갖고 동일 snapshot만 공유한다. 계산 state와 상속은 view 사이에 공유하지 않는다. |
| 7 | root 아래 실제 자손이 상위 앱 root에서 정상적으로 상속받는가 | fragment 모드도 기존 HostDocument 자손 연결을 그대로 사용한다. root 내부 상속 동작을 회귀 테스트에 포함한다. |
| 8 | fragment 모드가 기존 C03 `StyloDocumentView::new` 의미를 바꿔 기존 `:root` 동작을 깨는가 | 기존 생성자는 문서-root semantics를 유지하고 새 생성자만 fragment semantics를 선택한다. 기존 C03·C04 fixture를 다시 실행한다. |
| 9 | `div`, `button`, `p` 등 다른 root도 이전 값과 다른 방향으로 blockification 영향을 받는가 | `span` 외 기존 Chromium fixture에 있는 모든 root tag의 property string을 동일 비교한다. |
| 10 | fragment root가 document element용 기본 style·상속 경계를 받는가 | synthetic HTML/body HostNode를 만들지 않는다. adapter의 fragment wrapper 외에 브라우저 document 요소 의미를 추가하지 않는다. |
| 11 | root element가 실제 HostDocument에서 사라지거나 다른 NodeId로 치환되는가 | wrapper는 HostDocument에 삽입하지 않는다. 결과 node ID는 기존 `HostNodeHandle::id()` 그대로 유지한다. |
| 12 | wrapper를 만들려고 HostDocument generation/document revision을 인위적으로 증가시키는가 | 문서 snapshot은 읽기 전용으로 공유하고 HostDocument 변경·revision 증가를 하지 않는다. |
| 13 | fragment root가 HostRoot 직속이 아닌 detached node 또는 다른 세대 node로 우회되는가 | 기존 root generation·부모·Element 검증을 그대로 적용하고 invalid root는 전체 실패한다. |
| 14 | top-level Text가 fragment wrapper 밑에 들어가 조용히 무시되는가 | top-level Text는 이전 계약대로 전체 요청 오류이며 앞서 계산한 root 결과를 공개하지 않는다. |
| 15 | root 하나의 adapter/cascade 실패 뒤 앞선 root의 부분 result가 남는가 | 모든 root 계산 성공 뒤 한 번에 publish하는 atomic result 계약을 유지한다. |
| 16 | `:root`를 제외하면서 root 내부의 `html`, `body` 같은 실제 HostNode를 가짜 wrapper로 추가하는가 | 별도 HTML tag나 hidden host node를 만들지 않는다. 사용자가 실제로 만든 tag만 snapshot에 존재한다. |
| 17 | 각 root마다 HostDocument 전체를 deep clone해 다중 root에서 비용이 root 수만큼 증가하는가 | `Arc<HostDocumentSnapshot>` 공유 view를 사용한다. 기존 전체 snapshot clone은 JS owner가 revision 변경 시 한 번 만든다. |
| 18 | 비교가 동일한 입력 subtree를 기준으로 하지 않고 기준 Chromium fixture를 수정해 맞추는가 | 기존 C01/C04 reference를 변경하지 않는다. runtime HostNode/property 결과를 같은 원소 reference와 정확 비교한다. |
| 19 | HTML `style` 속성 파서가 fragment 모드에서 비활성화되어 author inline 값을 잃는가 | fragment view에서도 HTML namespace·attribute 검사와 inline-style parser를 사용하고 값·진단을 별도로 확인한다. |
| 20 | 이 계획 보정만으로 제품 DOM의 `Document`·`documentElement`·body·전체 CSS 지원을 주장하게 되는가 | 내부 Stylo adapter의 cascade 입력 semantics만 고정한다. 제품 JS DOM/CSSOM 및 전체 브라우저 문서 모델은 미지원으로 남긴다. |

## 판정

초기 계획의 “각 direct root를 독립 cascade root로 계산” 표현은 Stylo adapter의 `documentElement` 의미와 충돌했다. 이를 “CSS root가 아닌 fragment 자식”으로 바꾸고, 기존 문서-root adapter는 유지한다. `<span>` 기준 값이 불일치한다는 실행 관찰을 구현 결함으로 기록했으며, 이 검토는 보정 계획 전용이다. 코드 변경 뒤에는 별도의 새로운 20 관점 구현 검토를 수행한다.

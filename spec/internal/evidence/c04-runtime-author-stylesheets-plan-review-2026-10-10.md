# C04.11 구현 계획 검토

검토 대상: [`plan/c04-runtime-author-stylesheets.md`](../../../plan/c04-runtime-author-stylesheets.md)

## 독립 실패 관점

| # | 검토 관점 | 검증과 수정된 계획 결정 |
|---:|---|---|
| 1 | 하위 범위가 C04·C05 전체 완료로 읽히는가 | 하위 ID C04.11만 추가하며 C04·C05 상위는 미완료로 유지하도록 범위를 고정했다. |
| 2 | HTML namespace를 잘못 판정하는가 | namespace와 ASCII 대소문자 무관 local name을 함께 검사하고 HTML 외 style은 오류로 보인다. |
| 3 | style 텍스트에 일반 자손 텍스트 순서를 잃는가 | HostDocument의 textContent 순서로 CSS 본문을 구성하며 UTF-16 decode 실패를 숨기지 않는다. |
| 4 | 연결되지 않은 노드가 stylesheet로 남는가 | root children에서만 순회하고 매 요청마다 source 목록을 새로 구성한다. |
| 5 | 재삽입 순서가 node ID 생성 순서로 계산되는가 | source order를 node ID가 아닌 최신 snapshot 전위 순회 순서로 정했다. |
| 6 | stylesheet가 서로 다른 root에 적용되지 않는가 | 전체 문서의 source 목록을 각 root cascade에 전달한다. root 간 selector는 fragment 경계로 보장하지 않는다고 명시했다. |
| 7 | `<style type>` 해석을 과도하게 지원한다고 표시하는가 | 비 CSS MIME essence는 무시하고 CSS essence 후보만 stylesheet로 처리한다. |
| 8 | media query를 무조건 평가하는가 | 누락·빈 값·all·screen만 처리하고 나머지는 명시 오류로 지정했다. |
| 9 | `<link>`가 fetch를 호출하거나 오류 없이 빠지는가 | `rel` token 검사 후 미지원 오류를 반환하며 network client를 추가하지 않는다. |
| 10 | `@import`가 외부 fetch를 호출하거나 layout-only 경로에서 diagnostic 뒤 부분 성공하는가 | 초기 계획은 GPU 경로만 diagnostic 실패를 요구해 layout-only 요청이 부분 성공할 수 있었다. 계획을 고쳐 C04.11 author stylesheet parse diagnostic은 양 runtime 경로 모두 source ID·위치와 함께 전체 요청 실패로 고정했다. |
| 11 | source order·specificity·importance·inline origin이 잘못 연결되는가 | Chromium 계산값 fixture에 네 우선순위 축을 각각 포함한다. selector fixture는 type·ID·class·descendant로 제한해 전체 selector 지원을 암시하지 않는다. |
| 12 | C05 custom properties가 C04 profiles로 새어 들어가는가 | C05.1 profile에만 stylesheet custom properties를 허용하도록 구현 순서를 제한했다. |
| 13 | author background color가 잘못 파싱되거나 GPU가 색을 생략하는가 | paint profile에서만 허용하고 existing typed opaque/transparent 변환과 computed reference를 유지한다. |
| 14 | parser 진단이 있어도 조용히 일부 layout/scene을 게시하는가 | 초기 계획이 GPU 경로만 고려한 점을 수정했다. author stylesheet diagnostic은 layout-only 및 GPU 계산 모두 실패하며 inline style diagnostic 기존 동작은 유지한다. |
| 15 | HostDocument의 16M UTF-16 한도를 넘는 별도 입력이 생기는가 | 새로운 별도 총량 상한 없이 이미 강제되는 document budget을 사용하도록 했다. 구현은 매 revision 재파싱 비용을 성능 보장으로 해석하지 않는다. |
| 16 | lossy UTF-16 decode가 CSS 입력을 바꾸는가 | `String::from_utf16` strict decode와 잘못된 단위 오류 테스트를 완료 기준에 포함했다. |
| 17 | style CSS text가 일반 visible text처럼 layout 실패를 일으키는가 | `display:none` 조상 내 text만 layout tree/frame/render preorder에서 제외하도록 지정했다. |
| 18 | style을 보이게 한 author rule이 지원하지 않는 text를 숨기는가 | visibility override 시 layout은 실패해야 하며 성공한 빈 box로 처리하지 않는다. |
| 19 | hidden style이 frame 또는 GPU paint를 만드는가 | Chromium display/zero geometry, frame set, render box 부재를 함께 확인하도록 했다. |
| 20 | 오래된 worker가 제거된 sheet를 다시 게시하는가 | source list는 request-local이며 기존 full revision tuple·latest-wins publish를 유지하고 removal/move burst를 검증한다. |

## 재검토 결과

검토에서 발견한 layout-only partial success와 selector 범위 표기를 계획 본문에 반영했다. 외부 CSS loader·CSSOM·일반 stylesheet media·SVG style·visible text layout·incremental cache는 이 기능에 포함하지 않는다. 구현 시작 전 Chromium 입력·환경·오차 기준을 고정하며, 계획 검토 기록은 구현 후 코드 검토와 별도로 둔다.

# C08 Block·기본 페인트 계획 검토

검토 대상은 [C08 구현 계획](../../../plan/c08-first-screen-block-paint.md)과 [Chromium 사전 비교](./c08-block-flow-precomparison-2026-10-10.md)다. 이 표는 계획만 검토하며, 뒤의 코드 구현 검증을 대신하지 않는다.

| # | 실패 관점 | 적대적 질문·반례 | 확인 및 계획 반영 |
| ---: | --- | --- | --- |
| 1 | Oracle 실행 파일 고정 | 다른 Chrome 154 patch/build가 같은 결과라고 간주되는가? | capture가 product와 revision, 실행 파일 hash를 고정하고 mismatch에서 중단한다. |
| 2 | DPR 측정 혼합 | DPR 2의 기기 픽셀을 CSS px로 읽어 오차를 숨기지 않는가? | browser rect는 CSS px이고 DPR 1/2 각각 관찰한다. 각 좌표·크기를 비교하며 평균은 허용하지 않는다. |
| 3 | 기본 글꼴 OS 종속 | macOS의 Apple SD Gothic Neo를 Android/iOS의 필수 값처럼 강제하지 않는가? | 계획을 수정했다. OS 기본 family는 macOS oracle 진단값이며, 명시 family와 상속은 비교하고 기본 generic family는 cross-OS exact assertion에서 제외한다. |
| 4 | UA 기본 display | root에 display:block을 작성해 UA cascade가 빠져도 통과하지 않는가? | fixture root는 작성자 display가 없는 main이며 computed block 값을 확인한다. |
| 5 | auto width 채우기 | 모든 box에 width를 고정해 block auto-width가 빠져도 통과하는가? | root width만 고정하고 auto-width 자식의 280px frame을 비교한다. |
| 6 | auto height·자식 흐름 | 모든 부모 height를 명시해 자식 흐름을 계산하지 않아도 통과하는가? | group height를 auto로 두고 두 block child의 높이 합에 따른 38px used height를 oracle에 둔다. text/intrinsic sizing은 제외한다. |
| 7 | 문서 순서 | sibling 정렬이 바뀌어도 node ID 집합만 같으면 통과하는가? | root preorder와 각 frame y를 순서대로 비교하고 두 child·후속 sibling의 상대 위치를 고정한다. |
| 8 | display:none 조상 | 숨긴 node만 제외하고 보이는 descendant가 새 위치에 그려지지 않는가? | 숨긴 parent와 green descendant의 computed display는 유지하되 zero frame·scene 제외·뒤 sibling 위치를 함께 확인한다. |
| 9 | 숨긴 root와 빈 장면 | root 자체가 none이면 잘못된 Empty scene을 성공으로 보고하는가? | contract에 hidden root scene의 빈 결과를 유효로 정의하고 status/report로 성공과 layout 실패를 구분한다. 부정·성공 두 경로를 시험한다. |
| 10 | 텍스트 노드 유실 | 텍스트가 조용히 빠져 빈 색 상자 화면이 성공처럼 보이는가? | runtime render 단계가 visible text node를 명시 오류로 거부한다. glyph 지원 완료로 오인하지 않도록 C15/S07을 유지한다. |
| 11 | block 외 display | inline, contents, flex, grid, table을 Block처럼 해석하지 않는가? | C08 profile은 block/none만 허용하고 나머지는 node 식별 오류로 거부한다. 기존 Flex profile은 분리해 보존한다. |
| 12 | shorthand 우회 | 지원하지 않는 margin/border/background shorthand가 expanded longhand 검사에서 빠지는가? | allowlist는 parser가 확장한 declarations를 검사하고 모든 non-initial 미지원 longhand를 fail closed한다. shorthand negative fixture를 둔다. |
| 13 | 알파 손실 | 반투명 색을 불투명 RGB로 양자화해 잘못된 화면을 승인하지 않는가? | partial alpha는 style/runtime error이고 부분 장면을 공개하지 않는다. opaque와 transparent만 지원한다. |
| 14 | transparent underpaint | 투명 child가 parent color를 덮어 검정/초기 clear color를 만드는가? | scene에서 transparent를 no-paint로 보존하고 native readback에서 root 색상이 드러나는 pixel을 확인한다. |
| 15 | scene paint order | preorder index가 숨긴 node 때문에 건너뛰거나 중복되지 않는가? | scene은 hidden subtree 제거 뒤 연속 paint order를 발급하고 sibling 순서 및 parent underpaint를 검사한다. stacking context는 이번 범위 밖이다. |
| 16 | 0 크기 상자 | zero width/height box를 GPU에 보내 division/invalid geometry를 일으키지 않는가? | zero extent box는 scene에서 생략하며 음수·비유한 frame은 오류로 막는다. 별도 unit test를 추가한다. |
| 17 | foreground style 상속 | color/font property를 child별 default로 초기화해 inherited value를 덮지 않는가? | fixture에 hero-child를 추가해 명시된 color, size, family 상속값을 Chrome과 비교한다. 실제 글자 paint는 주장하지 않는다. |
| 18 | revision 혼합 | 이전 document/style/environment 결과가 새 box scene과 결합되지 않는가? | 내부 계약은 전체 source/style/environment revision tuple을 scene admission까지 확인하고 stale 하나라도 있으면 전체 거부한다. |
| 19 | runtime worker·incremental 재계산 | 새 profile이 첫 cascade에만 추가되고 style 변경 뒤 이전 Flex profile로 다시 계산되지 않는가? | 계획을 보완했다. runtime coordinator/profile selector, incremental reuse identity, worker 재계산의 모든 분기에 C08 profile을 연결하고 cache hit/miss와 profile mismatch를 시험한다. |
| 20 | 플랫폼 검증·preview 혼동 | Rust test만으로 Android/iOS WGPU 화면을 완료 처리하거나 terminal-browser가 제품 화면을 대신하지 않는가? | 실제 V8→Stylo→Taffy→WGPU 경로를 각 Simulator에서 실행한다. backend·화면·로그를 구분하고 Tailscale 페이지는 agent-browser로 QA한다. 실기기와 hardware GPU 성능은 주장하지 않는다. |

## 검토 후 수정

- div fixture root에서 작성자 display:block을 제거해 내장 UA 기본 block 결과를 검사하도록 바꾸고, hero-child를 추가해 color·font-size·font-family 상속을 보게 했다.
- 기본 generic font-family의 OS 종속성을 비교 기준에서 분리했다. 명시된 family와 inherited value만 exact 비교한다.
- profile 선택이 runtime cascade worker와 incremental reuse까지 관통해야 한다는 조건, hidden root·shorthand 확장·zero-area scene 동작을 계획에 명시했다.
- 화면 확인은 agent-browser를 사용한다. 코드 구현과 테스트 결과는 이 계획 검토에서 완료로 취급하지 않는다.

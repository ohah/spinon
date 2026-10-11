# C12.4 쌓임 맥락 계획 실패 관점 검토

검토 대상은 C12.4 전용 계획·제안 계약 0058·C12 상위 계획이다. 기능 코드, Chrome fixture, 모바일 실행 결과를 승인하는 문서가 아니다. 아래 항목은 서로 다른 규격 경계·데이터 경로·실패 조건을 하나씩 검토했다.

| # | 실패 관점 | 검토 결과와 조치 |
|---:|---|---|
| 1 | 전체 C22 painter가 C12.4의 선행 조건으로 남는가 | 기존 flat `RuntimeRenderSnapshot`은 연속 `paint_order`를 이미 보유한다. C12.4는 box 배경 순서만 소비하므로 전체 C22를 선행으로 요구하지 않게 계획과 상위 문서를 바꿨다. |
| 2 | 서로 다른 Block-only/Flex-only 계산 결과를 이어 붙여 혼합 문서인 것처럼 처리하는가 | 현재 profile 경계를 대조했다. Block·Flex·C12 position을 같은 style/layout revision에서 계산하는 단일 제한 profile을 구현 선행 계약으로 적었다. |
| 3 | flat 배열 하나만으로 nested context의 구조를 복원하려 하는가 | painter 안에서 임시 typed context staging을 만들고 결과만 flat scene으로 내보내도록 했다. scene에 context tree나 public API를 추가하지 않는다. |
| 4 | `z-index`를 scene 전체의 숫자 기준으로 정렬하는가 | 음수·0·양수 stack level 비교를 현재 stacking context 자식에 한정했다. context 바깥 숫자 비교를 금지했다. |
| 5 | `z-index:auto`와 `z-index:0`을 같은 격리 의미로 취급하는가 | positioned `auto` pseudo-context는 자손 context를 격리하지 않고 실제 level-0 context는 원자적으로 paint한다고 분리했다. |
| 6 | `position:fixed; z-index:auto`가 context를 만들지 않는가 | context 생성 표·fixture 요구에 fixed auto를 별도로 넣었다. |
| 7 | static Flex item의 정수 `z-index`를 무시하거나 static Block에도 잘못 적용하는가 | static Flex item은 실제 context를 만들 수 있고 일반 static Block에는 stack-level 효과가 없도록 box 종류를 분리했다. |
| 8 | Flex item의 `order`를 DOM/HostDocument 순서 변경으로 구현하는가 | 원본 tree·NodeId·수명 순서를 유지하고 직접 Flex item paint participant만 order-modified order에 두도록 했다. |
| 9 | `row-reverse`를 order-modified document order와 혼동하는가 | reverse direction은 paint rank 자체를 뒤집지 않는다고 명시하고 별도 Chrome overlap case로 남겼다. |
| 10 | 중첩 Flex의 local `order`를 전역 sort key로 합치는가 | item의 순서 키는 부모 Flex container 안에서만 유효하며 중첩 container 간 global sort를 금지했다. |
| 11 | absolute Flex child의 authored `order`가 item처럼 적용되는가 | C10.3.5의 `order:0`·source-tree paint 규칙을 참조하고 absolute child를 in-flow Flex item과 분리했다. |
| 12 | fixed Flex child를 absolute child 규칙의 증거 없이 같은 세부 순서로 단정하는가 | fixed child의 authored `order`는 flex item 재배치에 쓰지 않되, 정확한 paint phase/tie는 새 pinned Chrome case에서 고정하기 전까지 구현 차단 조건으로 남겼다. |
| 13 | non-context Flex item의 descendant context가 item의 무조건적 원자 상자 안에 갇히는가 | Flex item이 실제 context가 아니면 자손 context가 상위 context에 참여할 수 있음을 추가했다. atomic 처리 여부는 typed context 생성 규칙으로 정한다. |
| 14 | Stylo computed `z-index`를 다시 문자열 파싱하거나 실패값을 0으로 바꾸는가 | `auto`와 정수 typed value를 별도 표현으로 보존하고 raw CSS 재파싱·0 fallback을 금지했다. Stylo 0.22 accessor와 실제 Rust integer는 source 확인 전 추정하지 않는다. |
| 15 | mixed profile에서 기존 custom property·registered property·cascade layer 결과가 사라지는가 | `var()`, registered custom property, layer·inline cascade를 같은 revision의 Stylo computed result로 보존하고 fixture에 포함했다. |
| 16 | context 생성자 또는 stylesheet parser diagnostic이 source preflight를 우회하는가 | property 이름만 나열하는 체크로 끝내지 않도록 computed value와 source preflight 경계를 분리했다. selector winner를 모르는 보수적 거부 범위도 문서에 공개했다. |
| 17 | opacity·transform·contain 등 미지원 stacking 효과가 기본값처럼 조용히 렌더되는가 | context·compositing trigger를 fail-closed 회귀 입력으로 유지하고, 값별 source/computed 거부 규칙을 Chrome 비교 전에 확정하도록 했다. |
| 18 | `visibility:hidden|collapse`가 `display:none`처럼 생략되거나 보이게 그려지는가 | visibility를 누락된 paint 입력으로 확인해 `visible`만 성공 조건으로 추가하고 `hidden`·`collapse`를 미지원 오류 경계로 기록했다. |
| 19 | 새로운 paint rank가 이미 지원된 hit-test/event target까지 완료 처리하는가 | 현재 C12.4는 painter 순서만 정하고 S04.9·event dispatch를 완료 처리하지 않게 수정했다. 향후 scene hit-test가 생기면 동일 paint order의 topmost 선택을 별도로 검증한다. |
| 20 | screenshot 한 장이나 Android 화면 색을 독립 paint oracle로 쓰는가 | Chrome geometry/computed-style, 고정 overlap pixel, WGPU offscreen readback, 기기 화면 증거를 분리했다. output color space를 기록하고 Android screenshot은 픽셀 동등성 oracle로 혼합하지 않는다. |

## 남은 구현 전 관문

- Chromium 154 fixture와 독립 기대 paint rank를 만들고, Flex/non-context subtree·fixed/absolute Flex child·Block/Flex sibling tie를 캡처한다.
- WPT SHA는 후보 파일 위치만 고정했다. 목록 파일을 분류하고 실행하지 않은 WPT를 통과 결과로 표시하지 않는다.
- Android 실기기와 iOS Simulator 검증은 구현 후 관문이다. 이 계획 검토는 플랫폼 실행을 대신하지 않는다.
- 공개 상태 체크는 미완료이며 내부 계약 버전은 `0.1.0` 고정이다.

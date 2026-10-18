# C04.12 파싱 루트 HTML Block scene 계획 검토

계획 자체를 구현 전에 검토했다. 각 항목은 별도의 입력·실패 경로·불변 조건이며 같은 결론을 표현만 바꾸어 세지 않았다. 비교 입력은 [Chrome 155 사전 비교](c04-12-root-html-block-paint-precomparison-2026-10-18/README.md), 계획은 [`plan/c04-12-root-html-block-paint.md`](../../../plan/c04-12-root-html-block-paint.md), 관련 기존 계약은 C04.12/0077 및 C08.1이다. **구현 코드는 아직 시작하지 않았다.**

| # | 적대 입력·실패 경로 | 확인한 위험 | 계획의 판정·보호 장치 |
| --- | --- | --- | --- |
| 1 | 같은 source를 parser와 renderer용 DOM으로 각각 다시 파싱 | V8 DOM과 scene tree가 서로 다른 node/order를 가질 수 있음 | 같은 HostDocument snapshot을 cascade·layout·scene 전체에서 재사용하고 재파싱하지 않도록 고정했다. |
| 2 | 1 MiB 초과 source, 과다 node/depth/attribute 또는 quirks 문서 | 새 profile이 기존 parser budget을 우회하거나 과도한 입력을 허용할 수 있음 | 기존 `html5ever` 경로와 source/node/depth/attribute budget, `NoQuirks` 거부를 그대로 재사용한다. |
| 3 | `<head><style>`의 scheme media 규칙과 `<title>` text | `head`가 숨겨졌다는 이유로 stylesheet를 잃거나 head text를 paint할 수 있음 | Chrome 기준에서 head box는 0이며 style의 dark rule은 body 자식 computed color를 바꾼다. 기존 stylesheet collector를 유지하고 둘 다 따로 비교한다. |
| 4 | `<meta viewport>`/`<meta color-scheme>`를 visible element로 취급 | metadata가 layout·paint 또는 hit target에 나타날 수 있음 | 허용 visible node 목록에 metadata가 없으며 head 하위 metadata/title/style이 box를 만들지 않는지 통합 검증한다. |
| 5 | 입력에 재배치되는 head/body, 누락/반복 root 구문 | HTML tree recovery 뒤 잘못된 subtree를 root로 고를 수 있음 | HTML5 parser 출력의 단일 `<html>` 및 body 구조를 검증하고 unknown/duplicate rendering root는 fail-closed한다. |
| 6 | SVG namespace 또는 `foreignObject`를 HTML block처럼 전달 | 다른 namespace를 일반 HTML element로 오인할 수 있음 | visible node는 HTML namespace의 `html/body/main/div`만 허용하고 비 HTML namespace를 오류로 처리한다. |
| 7 | visible body text 또는 auto-height block 안의 text | text line box를 생략하고 잘못된 auto height/scene을 성공으로 게시할 수 있음 | visible text는 renderer 진입 전에 거부한다. 이 slice는 빈 block과 block 자식만 성공 경로다. |
| 8 | `display:none` 아래 text 또는 style-only text | 숨김 text 때문에 전체 문서가 불필요하게 실패하거나 숨김 여부 확인 전에 무시할 수 있음 | 실제 hidden ancestor 계산이 확인될 때만 hidden text를 paint 입력에서 제외한다. head의 stylesheet source 추출은 별도 단계로 유지한다. |
| 9 | 숨긴 ancestor 아래 `display:block` child, 빨강/초록 강한 배경 | descendant가 조상 숨김을 뚫고 높이·색을 남길 수 있음 | Chrome 기준의 hidden/hidden-child는 모두 0 rect이고 샘플에는 root 배경만 남는다. layout 및 scene 양쪽에서 subtree를 제거한다. |
| 10 | inline/table/flex/grid/contents display와 지원하지 않는 CSS 속성 | 새 root profile이 C08 whitelist를 우회해 일부 레이아웃만 잘못 그릴 수 있음 | `block`/`none` 외 display와 allowlist 밖 속성은 node/property 오류로 전체 계산을 거부한다. |
| 11 | stylesheet parse error와 shorthand expansion 뒤의 unsupported longhand | 선언 일부만 적용된 부분 style을 정상 결과처럼 게시할 수 있음 | cascade diagnostic 또는 unsupported expanded property가 하나라도 있으면 scene을 게시하지 않는다. |
| 12 | light↔dark 전환에서 `#child` media rule 변경 | CSS environment가 아닌 OS 기본값 또는 이전 scheme 결과를 재사용할 수 있음 | 사전 기준의 light/dark pixel을 고정하고 `prefers-color-scheme` 선택과 environment revision을 함께 검증한다. |
| 13 | layout viewport를 visual viewport·native pixel과 혼용 | 320 CSS px 좌표가 DPR/page-scale과 중복 곱해질 수 있음 | CSS frame과 device surface를 분리하고 DPR을 한 번만 적용한다. page-scale과 input transform은 이 slice에서 사용하지 않는다. |
| 14 | 투명 root, body 배경 전파와 일반 element box paint 혼합 | Canvas 배경과 body box를 중복 또는 다른 색으로 덮을 수 있음 | root Canvas resolver 의미를 그대로 두고 fixture는 opaque html root + transparent body를 사용한다. body-background propagation은 명시적으로 제외했다. |
| 15 | partial alpha, HWB/wide-gamut 또는 system color | WGPU의 제한 paint 색을 비슷한 sRGB로 반올림해 Chrome과 다르게 보일 수 있음 | 투명·완전 불투명 sRGB만 받고 나머지는 fail-closed한다. C04.12 기존 색상 제한과 C08 renderer 한계를 재사용한다. |
| 16 | 중첩 block의 auto height, root/child width 및 document order | flattening·잘못된 부모·source 순서 손실로 frame이 달라질 수 있음 | Chrome의 root/body/main/group/child 각 x/y/width/height를 고정하고 field별 0.5 CSS px 한도로 비교한다. |
| 17 | UA body 기본 margin 또는 인접 nonzero margin | 브라우저 기본 margin이나 margin collapse가 C08 subset 바깥으로 침투할 수 있음 | fixture에서 body margin을 0으로 고정한다. nonzero margin 및 margin-collapse를 성공 범위에서 제외한다. |
| 18 | style/environment revision이 layout 계산 중 갱신 | 이전 scheme 또는 document의 frame/scene이 최신 key를 달고 게시될 수 있음 | 같은 document/style/environment revision tuple을 scene까지 유지하고 stale revision은 거부한다. |
| 19 | unsupported node 하나 뒤에 지원 node가 여러 개 있는 tree | 앞부분만 그린 scene이 성공처럼 노출될 수 있음 | 실패 시 부분 scene을 게시하지 않고 이전 revision을 현재 결과로 재사용하지 않는다. Canvas resolver는 기존 결과와 분리한다. |
| 20 | simulator PASS 문자열만 있고 실제 drawable/readback 또는 artifact 경계가 없음 | runtime 진입·WGPU 표시·기본 바이너리 제외를 잘못 주장할 수 있음 | Android API 37 AVD와 iOS 26.2 Simulator의 WGPU 표본/화면을 요구하고, fixture FFI는 전용 feature/build flag 아래에서 기본·opt-in artifact를 각각 비교한다. 실기기·hardware GPU 주장은 금지한다. |

검토 결과 **구현 진입에 막히는 계획 결함은 발견하지 않았다.** 기존 parser·C08 제한을 재사용하고, visible text/unsupported CSS에서 부분 scene을 만들지 않는 경계가 고정되었다. 이 결과는 계획 검토이며 이후 구현 코드나 PR 변경의 검토를 대신하지 않는다.

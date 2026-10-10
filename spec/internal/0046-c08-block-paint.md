# 0046 · C08 Block 흐름·기본 페인트

**문서 ID:** 0046 · **내부 계약 숫자 버전:** 0.1.0 고정 · **상태:** 구현·기준 비교·시뮬레이터 검증 완료, PR 전 검토 · **공개 API:** 아님

0046은 문서 ID다. 출시 전 내부 계약과 Spinon crate 숫자 버전은 0.1.0으로 유지한다.

## 1. 입력과 소유권

- 계산 입력은 같은 HostDocumentSnapshot, root handle, CssViewport, StyleRevision에서 생성한다.
- Stylo computed style의 profile은 RuntimeBlockPaintV1이어야 한다. 이전 Flex runtime profile의 입력·출력 의미를 바꾸지 않는다.
- source DocumentGeneration, DocumentRevision, RenderTreeRevision, StyleRevision, EnvironmentRevision, viewport는 계산·layout·render scene에 그대로 전달한다. 현재 tuple 중 하나라도 다르면 전체 scene을 거부한다.
- RuntimeBlockPaintV1은 한 HostRoot element subtree만 계산한다. top-level text node, dangling/duplicate node, 해당 subtree 밖의 style/frame은 오류다.
- CSS가 제공하는 computed value는 Stylo가 소유한다. Taffy는 제한 Block geometry를 계산하며, renderer는 입력 CSS를 재해석하지 않는다.

## 2. C08 author style 허용 범위

성공 입력의 author property는 다음으로 제한한다.

| 속성 | 계산·사용 |
| --- | --- |
| display | block과 none만 허용. root는 UA computed block도 허용 |
| box-sizing | content-box / border-box |
| width, height | CSS auto 또는 현재 layout adapter가 지원하는 length·percentage·CSS math |
| background-color | 완전 투명 또는 불투명 sRGB 단색 |
| color, font-size, font-family | computed string 및 상속 확인용; GPU text paint로 넘기지 않음 |

위 목록 밖의 inline declaration과 stylesheet author declaration은 property/stylesheet 출처가 식별되는 오류로 거부한다. shorthand는 파서가 확장한 longhand를 기준으로 검사한다. 무효 CSS declaration은 cascade 진단으로 보존되며 진단이 있는 결과를 장면으로 공개하지 않는다. 지원되는 display는 author 값을 조용히 다른 값으로 바꾸지 않는다.

유효한 computed display:none root는 layout-ready 상태의 빈 scene을 만든다. 숨긴 node와 모든 descendant는 zero frame이며 scene box에 포함되지 않는다. visible text node가 있으면 UnsupportedTextNode를 반환한다. 텍스트를 버리고 배경 box만 성공시킬 수 없다.

## 3. Layout projection

- root와 자식의 computed display:block은 Taffy Display::Block에 전달한다.
- auto-width child는 root content width를 채운다. 고정 높이·빈 자식의 기본 Block 흐름, 자식으로 계산되는 auto-height parent, DOM 순서를 C08 fixture에서 비교한다.
- width 또는 height가 0인 node는 layout 결과에 남지만 runtime paint scene에서는 생략한다. 음수·NaN·무한 frame은 전체 계산 오류다.
- inline, contents, flow-root, flex, grid, table 등 Block/none 외 display는 C08 profile에서 오류다. flex를 허용하는 기존 profile과 교차하지 않는다.
- margin collapse, padding·border 상호작용, text/intrinsic sizing, replaced elements, formatting context, positioned layout은 이 계약에 없다.

## 4. Paint scene

- scene은 HostDocument preorder에서 visible positive-size element box만 만든다.
- paint order는 scene에 실제 포함된 box에 0부터 연속 발급한다. hidden/zero-size node는 순번을 소비하지 않는다.
- background-color: transparent는 RuntimePaint::None이고 기존 장면 아래의 paint를 보존한다. 완전 불투명 sRGB만 RuntimePaint::Opaque로 보낸다.
- 부분 alpha와 현재 color conversion으로 보존할 수 없는 computed color는 전체 오류다. 색 채널을 임의로 반올림해 단색으로 가장하지 않는다.
- color, font family, font size는 현재 computed style에 기록되지만 GPU scene의 box painter는 사용하지 않는다. C08의 실제 화면에는 텍스트가 없다.
- border geometry가 있더라도 C08 painter는 선을 그리지 않는다. border decoration은 C22 계약 전까지 미지원이다.
- scene key는 source/style/environment revision 전체와 CSS viewport를 포함한다. 오류 중 일부 box를 이전 scene에 덧붙이지 않는다.

## 5. 실행·오류 계약

실제 runtime은 기존 CSS cascade worker에서 profile을 명시적으로 선택하고, cache 및 incremental restyle key도 profile identity를 보존한다. 새 profile을 Flex 결과에 캐스팅하지 않는다. Android/iOS 경로는 같은 fixture JS를 실제 V8에서 평가하고 같은 RuntimeRenderSnapshot을 wgpu에 제출한다.

실패 코드는 기존 runtime layout/render 오류 모델을 따른다. 적어도 unsupported style/display, partial alpha, visible text, stale snapshot, node/style/frame 누락, duplicate node, invalid geometry는 실패 결과로 구분해야 한다. 실패 때 새 부분 scene은 공개하지 않으며 직전 표시 scene의 수명을 훼손하지 않는다.

## 6. 판정과 한계

[C08 고정 Chromium 기준](evidence/c08-block-flow-precomparison-2026-10-10.md)과 [작업 계획](../../plan/c08-first-screen-block-paint.md)을 따른다. computed display, background color, color, font-size는 고정 환경에서 정확 비교한다. author 지정 font-family는 exact 비교하고 UA 기본 generic family는 OS preference 차이가 있으므로 cross-platform diagnostic으로만 둔다. layout field별 최대 절대 오차는 0.5 CSS px, DPR 1/2 geometry는 불변이어야 한다.

Android API 37 emulator 및 iOS 26.2 Simulator에서는 layout=ready, 동일 fixture node/box 수, scene revision, wgpu presented, 화면을 확인한다. 실제 기기·hardware GPU 성능, glyph·text color, OS 글꼴 fallback, border line, opacity blending을 이 검증으로 주장하지 않는다.

구현 후 실패 경로는 [별도 구현 검토](evidence/c08-block-flow-implementation-review-2026-10-10.md), 플랫폼 실행과 캡처는 [Simulator 근거](evidence/c08-block-flow-simulators-2026-10-10.md)에 기록한다. 계약 숫자 버전은 계속 `0.1.0`이다.

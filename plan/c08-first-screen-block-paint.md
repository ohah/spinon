# C08 · 첫 화면 Block 흐름과 기본 페인트 계획

## 목표와 완료 경계

현재 runtime CSS→GPU 장면을 Flex root 전용 예제로만 묶지 않고, 일반적인 block root와 block 자식의 첫 정적 화면까지 연결한다. display:block의 기본 수직 흐름, auto width, display:none의 조상 숨김, 투명·완전 불투명 background-color를 Stylo→Taffy→runtime scene→Android/iOS WGPU 경로에서 검증한다. CSS color, font-size, font-family의 초기·상속 computed value도 snapshot에서 비교한다.

이 항목은 CSS Block formatting 전체나 텍스트 표시 완료가 아니다. 현재 renderer는 box 배경을 칠하고 텍스트 노드는 거부한다. C08에서는 글자색·글꼴 computed 값을 확인해도 glyph를 렌더했다고 주장하지 않는다. 텍스트 shaping·줄 배치·glyph paint는 C15/S07의 소유다. 테두리 paint는 C22의 소유라 border-width가 있어도 선은 나타나지 않는다. 부분 alpha, gradient, image, radius, shadow도 지원하지 않는다.

기존 RuntimeFlexPaintV1 이름을 Block 경로의 계약으로 재해석하지 않는다. C08 전용 내부 RuntimeBlockPaintV1을 추가하고, 허용 CSS와 출력 node set을 좁게 정한다. 공개 element API나 사용자 CSS 호환 완료로 표시하지 않는다. 0.1.0 내부 계약 및 crate 숫자 버전은 올리지 않는다.

## 사전 기준과 oracle

- 기준: [C08 고정 Chromium 154 비교](../spec/internal/evidence/c08-block-flow-precomparison-2026-10-10.md), [기준 JSON](../tests/fixtures/css/references/c08-block-flow-v1.json), 320×240 CSS px / DPR 1·2.
- 고정값: Chrome 154.0.8037.98, revision @b859317bf11f6be47f9b7799ec690a0a42a1fb33, macOS 26.5.1 (25F80), en-US, UTC, light, coarse pointer.
- computed display, background-color, color, font-size를 inventory 순서로 exact string 비교한다. font-family는 명시한 Arial, sans-serif의 계산 문자열과 상속을 검증하고, UA 기본 generic family 문자열은 macOS capture 환경 진단값으로만 보존한다. Stylo platform preference와 OS font mapping이 확정되기 전에는 기본 family를 다른 플랫폼과 exact 비교하지 않는다.
- Used geometry는 getBoundingClientRect() 대 Rust layout frame으로 각 node의 x/y/width/height를 개별 비교한다. field별 허용치는 최대 절대 오차 0.5 CSS px; DPR 1·2 결과도 일치해야 한다.
- 실제 GPU 장면은 node order, frame, transparent/opaque paint를 검사하고 중앙 offscreen pixel 또는 동등한 native readback을 기준 색과 비교한다. screenshot 화면만으로 layout 수치가 같다고 판정하지 않는다.
- 캡처 실패, 실행 파일·revision·hash·fixture ID·node 목록 불일치, 누락 frame은 실패다. reference를 테스트 도중 자동 재생성하지 않는다.

## 지원하는 C08 subset

| 입력·동작 | C08 처리 |
| --- | --- |
| root 및 자식의 computed display:block | 지원. block 자식의 DOM 순서 세로 배치와 auto width를 Taffy Block으로 계산 |
| display:none | 지원. 해당 노드와 모든 descendant의 layout frame은 zero 처리하고 scene에서 제외 |
| CSS display 초기값 | Stylo UA cascade 결과를 사용하되 root·fixture의 실제 computed value를 비교 |
| 배경색 transparent 또는 완전 불투명 sRGB 색상 | 지원. 기존 GPU 단색 배경 표현과 paint order에 연결 |
| 부분 alpha·색상 공간/색 함수가 scene 표현 범위를 넘는 값 | 진단/오류로 전체 계산을 거부. 불투명 색으로 반올림하지 않음 |
| color, font-size 초기값·상속 및 명시 font-family 상속 | cascade snapshot computed string 비교. 글리프 paint·font fallback 동등성 근거로 사용하지 않음 |
| 텍스트 노드 | runtime scene 진입을 거부. 무시하거나 box로 바꾸지 않음 |
| inline/flex/grid/table/contents 등 Block 외 display, unsupported style property | node/property 식별 가능한 오류로 거부 |
| width 또는 height가 0인 block | 유효 layout이지만 paint scene에서는 생략. 음수·비유한 frame은 전체 오류 |
| 숨김 root와 descendant | 유효한 빈 paint scene. layout 실패나 누락 결과와 구분되는 ready report를 반환 |
| margin collapse, padding·border geometry 상호작용, intrinsic content sizing, inline formatting | 이번 성공 경로에서 제외하고 C09/C14/C15 등 후속 CSS 계획으로 남김 |

fixture에는 고정 크기의 빈 block만 둔다. 따라서 auto height는 자식 block을 포함하는 범위로만 확인하며 텍스트·이미지 기반 intrinsic height는 포함하지 않는다. auto width는 root의 content width 안에서 비교한다. margin collapse를 우연히 밟지 않도록 fixture에 margin을 넣지 않는다.

## 구현 단계

1. C08 fixture·inventory·capture tool·고정 reference를 생성하고 SHA-256과 Chromium 환경을 기록한다. HTML div root는 작성자 display 없이 포함된 UA 기본 block을 사용한다. 현재 사전 기준은 DPR 1/2에서 block auto-width, nested auto-height, source order, none subtree, 기본·상속 computed style을 보존한다.
2. 이 계획을 코드 구현과 별도로 20개의 서로 다른 실패 관점에서 검토한다. 각 관점은 구체 입력/불변 조건과 disposition을 남기고, 누락된 경계는 계획 수정으로 반영한다.
3. 내부 계약 0046을 추가하고 RuntimeBlockPaintV1의 profile, 허용 속성, 텍스트/투명도/오류 경계, source revision, GPU scene 출력 계약을 고정한다. runtime cascade worker와 incremental reuse에서도 이 profile을 명시적으로 선택하고, 예전 Flex profile의 동작은 유지한다. internal index와 공식 상태 대장에는 계획 진행만 기록한다.
4. Stylo runtime block profile이 whitelist longhand와 typed background paint를 같은 style revision에서 출력하도록 연결한다. foreground/font defaults는 computed string으로 보존하고 GPU glyph path로 위장하지 않는다.
5. layout adapter에서 C08 profile을 Taffy Block으로 project한다. 두 개 이상의 block 자식, auto width/height, nested flow, hidden ancestor에 대해 fixture ID별 geometry를 비교한다.
6. render adapter가 같은 revision의 preorder만으로 scene을 만들고 hidden subtree를 생략하는지, 투명·불투명 background 및 paint order가 맞는지 확인한다. 오류/진단이 있는 snapshot은 부분 장면으로 공개하지 않는다.
7. CSS reference test·cascade/layout/render/runtime 통합 시험을 추가한다. invalid display, shorthand가 확장된 unsupported property, author stylesheet 성공·거부, parse diagnostic, partial alpha, visible/hidden text, stale revision, hidden descendants와 zero-area box를 확인한다. 숨김 root는 empty scene 성공으로, zero-area box는 paint 생략으로 구분한다.
8. 실제 V8 fixture를 Android API 37 에뮬레이터와 iOS 26.2 Simulator에서 실행한다. layout=ready, 예상 box 수, scene key/revision, presented 및 화면을 저장한다. Tailscale 웹 미리보기의 화면 확인은 agent-browser로 하고, terminal-browser를 사용하지 않는다. emulator 소프트웨어 renderer는 hardware GPU로 표현하지 않는다.
9. 구현 뒤 계획 검토와 중복하지 않는 새 20개 관점으로 코드·런타임·모바일 경로를 공격 검토한다. 발견한 결함은 고친 뒤 해당 oracle·회귀·플랫폼 확인을 다시 실행한다.
10. 구현 계약·STATUS·internal README·fixture/evidence·Tailscale preview source를 함께 동기화하고, PR은 한국어 본문과 Android/iOS 화면 첨부를 포함한다. 문서 사이트 자동 배포는 하지 않는다.

## 현재 구현 결과

- RuntimeBlockPaintV1, Taffy Block 투영, 제한 background scene, Android/iOS V8→WGPU fixture를 구현했다. 고정 Chromium reference의 9개 node와 DPR 1·2를 비교한다.
- 실패 경로 검토에서 빠져 있던 supported stylesheet, unsupported stylesheet, parse diagnostic, visible/hidden text, zero-area box, stale style revision 검사를 추가하고 재실행했다.
- Rust crate 회귀, C08 reference 검사, Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator 검증은 통과했다. 로그·화면·hash 및 한계는 [구현 검토](../spec/internal/evidence/c08-block-flow-implementation-review-2026-10-10.md)와 [Simulator 근거](../spec/internal/evidence/c08-block-flow-simulators-2026-10-10.md)에 있다.
- PR 전이라 `spec/STATUS.md` 체크박스는 미완료로 유지한다. C09 Block semantics, C15/S07 glyph, C22 border paint는 이 하위 항목의 완료 범위에 포함하지 않는다.

## 통과 기준과 남는 항목

- Chromium C08 inventory/reference에서 모든 supported computed style과 node geometry를 field별 비교하고 DPR 1·2가 일치한다.
- Rust tests는 supported block tree의 geometry 및 scene paint 순서를 검사하며, 각 unsupported 입력의 fail-closed 결과와 stale snapshot 거부를 검사한다.
- Android/iOS Simulator 각각에서 고정 V8 fixture를 실제 wgpu scene으로 제출하고 presented/화면 근거를 남긴다. OS 간 픽셀 래스터 동일성을 주장하지 않는다.
- 외부 CSS 지원, inline/table/grid/flex 상호작용, margin collapsing, text node/glyph, color text paint, border line, radius, opacity blend, transforms, clipping, images, hit-test/IME/accessibility, performance ranking은 별도 항목으로 유지한다.
- C08 전체 체크는 지원 범위와 구현 계약이 STATUS에 정확히 반영될 때만 완료한다. C08 subset가 머지되어도 C09 이후 Block semantics 및 C15/S07 텍스트는 미완료다.

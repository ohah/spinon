# C12.3 · viewport fixed 위치 계획

## 목표

제한 runtime layout profile에서 CSS `position: fixed`의 기본 fixed containing block을 `CssViewport`로 선택하고, 물리 inset과 C12.2에서 이미 지원하는 측정 가능한 박스 크기 조합을 CSS viewport 기준으로 계산한다. viewport 크기나 device scale factor가 바뀌면 이전 environment revision의 fixed frame을 새 화면으로 게시하지 않는다.

이 단계는 fixed 요소의 초기 viewport frame과 환경 변경 재계산만 다룬다. Scroll API가 없으므로 스크롤 중 화면 좌표 불변을 검증하거나 전체 fixed 동작을 지원한다고 주장하지 않는다. C12.3은 CSS 전체 fixed positioning 지원이 아니다.

## 선행 조건과 상태

- C12.1 static/relative, C12.2 Block absolute, C10.3.5 positioned Flex 교차 구현이 main에 병합되었다.
- `CssViewport`는 CSS px 폭·높이, device scale factor, media environment, `EnvironmentRevision`을 가진다. layout→render adapter는 viewport·environment·document/style revision이 맞지 않는 snapshot을 거부한다.
- Taffy `0.14.0`의 position 값은 `Relative`와 `Absolute`뿐이다. CSS `Fixed` 의미는 Spinon layout 입력·owner 계산에 보존하고, Taffy 경계에서만 absolute positioning으로 투영한다.
- 이 계획과 별도인 구현 전 비교 기준을 고정한 뒤 기능 구현을 시작한다. 구현 시 내부 계약 문서 `0057`을 추가하며 출시 전 내부 숫자 버전 `0.1.0`은 유지한다.

## 이번 단계에서 고정할 동작

### 포함 범위

- 연속 미디어의 앱 CSS viewport를 initial fixed containing block으로 사용한다. 원점은 `(0, 0)` CSS px이며 크기는 같은 계산의 `CssViewport.width_css_px`·`height_css_px`다.
- `horizontal-tb`·LTR, Block root, 현재 runtime CSS whitelist가 표현하는 box 입력을 대상으로 한다. C12.2의 fixed-size 박스와 inset·min/max로 측정 가능한 조합을 재사용하되, 고정 comparator에서 개별 동작을 먼저 확정한다.
- fixed box에는 기존 C12.2가 지원하는 물리 `top/right/bottom/left`, `inset`, 길이·percentage·지원 CSS math, margin, padding, border, box-sizing, min/max를 viewport basis로 적용한다. top/bottom percentage는 viewport 높이, left/right percentage는 viewport 너비를 사용한다.
- 두 축 모두에서 static-position 계산을 요구하지 않는 입력만 허용한다. 한 축의 두 inset이 모두 `auto`여서 static position에 의존하는 입력은 이번 성공 profile에서 명시 오류로 거부한다. definite 양쪽 inset과 auto 크기·margin은 C12.2가 지원한다고 고정한 측정 가능 박스에 한해 comparator로 확인한다.
- 일반 `static`·`relative`·`absolute` 조상은 fixed containing block이 아니다. 따라서 viewport-only subset에서는 fixed box를 그 조상들의 크기·position·padding에 붙이지 않는다.
- `display:none` 자신 또는 조상 아래의 fixed box는 viewport synthetic root로 올리지 않는다. 숨겨진 subtree는 layout frame과 render entry를 만들지 않는다.
- fixed box는 absolute positioned descendant를 위한 absolute containing block이 된다. 반면 fixed descendant는 viewport fixed containing block을 계속 사용한다. 두 owner 관계를 별도 계산한다.
- Stylo computed snapshot에는 현재 Chromium fixed-CB trigger에 해당하는 computed effect 표식을 별도 보존한다. 최소한 transform 관련 속성/`transform-style: preserve-3d`, non-initial `filter`·`backdrop-filter`, layout/paint containment, `content-visibility`가 활성화한 containment, 이에 해당하는 `will-change`와 적용되는 containment shorthand를 추적한다. author/inline preflight가 이 입력을 먼저 거부하더라도 표식은 style-to-layout 경계에서의 두 번째 검증으로 유지한다. CSS `position:fixed` 자체는 fixed-CB effect 표식으로 취급하지 않는다.
- CSS source parent, DOM parent, event/lifetime 소유 관계는 유지한다. viewport synthetic root는 Taffy 계산 내부 전용이며 HostDocument·NodeId·render tree에 노출하지 않는다.
- viewport 폭·높이, device scale factor, media environment 중 유효 값이 바뀌면 새 `EnvironmentRevision`과 그 revision에 맞는 cascade/layout/render snapshot만 게시한다. 동일 환경 입력은 불필요한 revision 증가를 만들지 않는다.

### 명시적 제외·실패 경계

- CSS transform 계열, `perspective`, `filter`, `backdrop-filter`, `contain`, `content-visibility`, fixed-CB 효과를 내는 `will-change` 등 ancestor 효과를 이번 profile은 지원하지 않는다. 해당 선언을 author stylesheet와 inline style 양쪽에서 조용히 무시하거나 viewport 기준으로 잘못 계산하지 말고, 기존 preflight/cascade 오류 계약으로 계산 전체를 거부한다.
- view transition root, canvas-for-drawing, SVG `foreignObject`, UA text control 내부 트리, anonymous fieldset box 같은 Chromium 전용 box topology는 이 단계에서 모델링하지 않는다. 지원 DOM·box topology가 fixed owner를 정확히 분류하지 못하는 입력은 성공으로 처리하지 않는다.
- `position: fixed`인 layout root, 두 inset이 모두 auto인 축의 static position, Flex/Grid static-position, Grid, inline fragmentation, text/replaced intrinsic sizing, RTL·논리 inset, transform/contain fixed containing block, top layer/dialog/popover, paged media를 제외한다.
- scroll offset, scroll containers, clipping, safe-area·viewport segment, pinch zoom의 visual viewport, 키보드에 따른 platform viewport 정책, hit testing, stacking context·`z-index`, fixed box의 paint order를 구현 완료로 주장하지 않는다. scroll은 C13, stacking은 C12.4가 소유한다.
- border stroke·radius·shadow 같은 paint는 이 단계의 geometry 증거에 포함하지 않는다.

## owner와 계산 트리 규칙

- layout snapshot은 position enum에서 `Fixed`를 보존한다. Taffy에 넘길 때만 `Position::Absolute`로 매핑하며, 최종 frame은 CSS viewport origin 기준 CSS px로 환산한다.
- out-of-flow child 계산은 source tree를 재부모화하지 않는 별도 graph를 유지한다. `absolute_owner`와 `fixed_owner`를 같은 상속 경로로 합치지 않는다.
- `absolute_owner`는 nearest positioned ancestor를 따른다. `relative`, `absolute`, `fixed` box는 absolute descendant의 owner가 될 수 있다.
- `fixed_owner`는 이 단계에서 viewport다. 일반 positioned ancestor와 fixed ancestor는 fixed descendant의 owner가 되지 않는다. fixed-CB 효과를 낼 ancestor는 성공 입력에 존재하지 않으며 preflight에서 차단한다.
- 효과 표식이 있는 조상 아래 fixed 노드는 viewport로 재부모화하지 않고 layout 전체를 `UnsupportedPositioning` 오류로 닫는다. author source preflight뿐 아니라 computed snapshot→layout projection 경계에서도 검사하므로 한 계층이 우회되어도 viewport owner를 잘못 채택하지 않는다.
- fixed box를 viewport synthetic Taffy root의 직접 계산 자식으로 두더라도 `display:none`·누락 node·불일치 source parent·중복 owner를 필터링/검증한다. hidden child를 viewport 자식으로 승격하지 않는다.
- layout 계산은 C12.2처럼 전체 입력에 대해 실패 원자성을 유지한다. 일부 fixed box만 새 frame으로 노출하거나 다른 revision의 normal-flow frame과 섞지 않는다.

## 비교 모델과 판정 기준

- 비교 oracle은 저장소가 고정한 Chrome `154.0.8037.98`, Chromium revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33`다. 기준은 [CSS Position 3 · 2025-10-07 WD](https://www.w3.org/TR/2025/WD-css-position-3-20251007/)의 fixed containing block과 inset 계산, [CSS Transforms 1](https://www.w3.org/TR/css-transforms-1/), [CSS Containment 2](https://www.w3.org/TR/css-contain-2/) 및 pinned Chromium의 `LayoutObject::ComputeIsFixedContainer`와 대조한다.
- Chromium oracle은 viewport `360×800` 및 `390×844` CSS px, LTR, `horizontal-tb`, light, `en-US`, UTC에서 캡처한다. 각 viewport는 DPR 1·2·2.625·3으로 관찰한다. same CSS viewport에서 DPR만 바뀌면 computed CSS px geometry는 같아야 한다.
- 고정 inventory는 최소 다음을 각각 구분한다: top/left 및 right/bottom 고정 inset, 축별 percentage와 `calc()`, negative·zero inset, auto 반대편 inset, 양쪽 inset 기반의 측정 가능한 stretch, margin·box-sizing·border·padding·min/max, static/relative/absolute 중첩 ancestor, fixed ancestor, fixed 내부 absolute child, fixed descendant, root frame 크기·원점이 viewport와 다른 경우, `display:none`, 360×800→390×844→360×800 resize, DPR-only 변경, 같은 입력의 no-op.
- 실패 inventory는 양 stylesheet/inline의 unsupported fixed-CB ancestor 효과, root fixed, unsupported static-position 입력, invalid viewport·non-finite 값, overflow/indefinite percentage, stale style/environment revision, hidden ancestor, node·owner 불일치와 계산 실패 뒤 이전 scene 잔존을 포함한다.
- 각 지원 node의 `x/y/width/height` 최대 절대 오차는 `0.5 CSS px` 이하여야 한다. 개별 node·field identity, computed inset, owner 종류, source parent 불변, hidden 상태를 따로 판정하며 평균 오차로 실패를 숨기지 않는다.
- viewport가 같은 경우 DPR 변경은 CSS geometry를 바꾸지 않아야 한다. CSS viewport 변경은 Chrome frame과 일치해야 하고, 실제 runtime에서는 새 environment revision의 frame만 제출되어야 한다. resize 전후 프레임 제출 순서를 고정하며 예전 revision이 뒤늦게 게시되면 실패다.
- HTML, inventory, capture tool, Chrome binary 및 reference JSON의 SHA-256을 precomparison에 기록한다. hash, Chrome revision, viewport/DPR 조건이 달라지면 pass/fail 비교를 중지한다. WPT 의미 대조 subset은 commit `9ec154ff43db468923997c08bb08f905ceab62a5`의 `css/css-position/position-fixed-at-bottom-right-on-viewport.html`(viewport 기준), `css/css-position/absolute-pos-box-inside-fixed-pos-box-with-changing-height.html`(fixed box의 absolute 자손 owner), `css/css-position/position-fixed-dynamic-transformed-sibling.html`(transform fixed-CB 효과의 negative 경계)로 고정한다. 이 subset은 fixture 선택 근거이며 전체 WPT suite나 해당 동적 CSS mutation을 구현·실행했다고 주장하지 않는다. fixed root의 Flex/Grid cases는 이번 범위 밖이다.

## 구현·검증 순서

1. 이 계획과 계획 실패 관점 기록을 먼저 확정한다. 계획 검토 지적을 반영하기 전에는 런타임 기능 코드를 수정하지 않는다.
2. [구현 전 비교 완료](../spec/internal/evidence/c12-3-fixed-precomparison-2026-10-11.md). 전용 HTML·inventory·Chrome capture에서 viewport owner, ancestor owner와 resize/DPR 환경을 관찰하고 지원·거부 경계 및 tolerance를 고정했다. runtime 구현은 이 고정 기준을 바꾸지 않는다.
3. Stylo author stylesheet source preflight와 inline style preflight를 각각 확인한다. computed snapshot→layout projection에도 fixed-CB effect 표식과 ancestor validation을 전달한다. effect 속성이 preflight를 우회해 cascade/layout에 도달하면 해당 입력을 reject하는 회귀 테스트를 추가한다.
4. layout DTO에 `Fixed` 의미를 추가하고 Taffy absolute 매핑, 별도 absolute/fixed owner, viewport synthetic root 연결을 구현한다. source tree와 viewport root 사이의 graph consistency를 검증한다.
5. C12.2 absolute fixture 전부를 회귀 실행하고, C12.3 지원/실패 fixture의 Chrome geometry를 node별 비교한다. resize·DPR-only·stale completion·invalid viewport 테스트를 추가한다.
6. Rust workspace test, Clippy, rustfmt, CSS reference suite와 앱 빌드를 실행한다. Android 실기기에서 실제 V8→Stylo→Taffy→WGPU를 먼저 확인하고 iOS Simulator에서 같은 fixture를 확인한다. 플랫폼은 CSS px viewport, device scale factor, EnvironmentRevision, 최종 제출 revision을 별도 로그로 보존한다.
7. 화면 캡처는 런타임과 revision 로그를 확인한 후 얻는다. 화면으로 geometry의 Chrome 수치 일치, scroll 불변, hardware GPU 성능을 주장하지 않는다.
8. 구현 뒤 계획 검토와 겹치지 않는 실제 변경 코드·실행 실패 관점 20개를 별도 기록하고 발견 결함을 고친 뒤 영향 검사를 다시 실행한다.
9. 내부 계약 ID `0057`, `spec/STATUS.md`, `spec/internal/README.md`, 계획, roadmap preview와 실행 evidence를 동기화한다. C12.3을 완료로 바꿔도 C12 상위·C12.4/C12.5는 미완료로 둔다.

## 완료 판정

- 고정 Chrome 지원 fixture에서 모든 지원 node의 field별 오차와 owner/source-tree invariant가 사전 기준을 만족한다.
- 지원하지 않는 ancestor 효과·static position·invalid input·stale revision이 오류로 닫히고 이전/부분 frame이 성공처럼 게시되지 않는다.
- C12.2 절대 배치·C10.3.5 Flex positioned regression이 유지된다.
- Android 실기기와 iOS Simulator에서 동일 V8 fixture가 새 environment revision을 사용해 resize 전후 화면을 표시하고, 각 시점에 제출된 revision을 로그로 확인한다.
- 내부 인터페이스 명세, 코드 예제, 오류 경계, evidence, 공식 상태와 preview가 변경과 일치한다. 위 조건을 충족하기 전에는 C12.3 완료를 체크하지 않는다.

## 근거 자료

- [CSS Positioned Layout Module Level 3 · 2025-10-07 Working Draft](https://www.w3.org/TR/2025/WD-css-position-3-20251007/)
- [CSS Transforms Module Level 1](https://www.w3.org/TR/css-transforms-1/)
- [CSS Containment Module Level 2](https://www.w3.org/TR/css-contain-2/)
- [Pinned Chromium `LayoutObject::ComputeIsFixedContainer`](https://chromium.googlesource.com/chromium/src/+/b859317bf11f6be47f9b7799ec690a0a42a1fb33/third_party/blink/renderer/core/layout/layout_object.cc)
- [C12.2 계약 0055](../spec/internal/0055-c12-2-absolute-block.md), [C10.3.5 계약 0056](../spec/internal/0056-c10-3-5-positioned-flex.md), [C04.10 resize 근거](../spec/internal/evidence/c04-runtime-css-to-gpu-resize-simulators-2026-10-09.md)

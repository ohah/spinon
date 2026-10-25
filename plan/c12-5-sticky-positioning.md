# C12.5 · 제한된 sticky positioning 계획

## 목표와 경계

C12.5는 이미 연결된 C13.2 scroll snapshot에서 가장 가까운 scrollport의 물리 세로 offset을 읽어, 제한된 Block fixture의 `position: sticky; top: <px>` box를 in-flow frame과 sticky 제약 사이에서 배치한다. sticky box는 layout flow에 남고 CSS px geometry·paint·hit-test가 같은 immutable scroll revision을 사용해야 한다. scroll gesture, public CSSOM, 공개 CSS 지원 완료 또는 C12 전체 완료를 뜻하지 않는다.

이번 수직 slice의 고정 profile은 다음으로 제한한다.

- `horizontal-tb`, LTR, Block root와 요소 전용 tree를 사용한다. 텍스트, inline fragment, Grid, Flex item, replaced content, anonymous/pseudo box는 허용하지 않는다.
- `position: sticky` element에는 finite nonnegative absolute `top` 길이를 요구한다. `right`, `bottom`, `left`는 computed `auto`여야 한다. percentage, CSS math, logical inset, 반대편 inset, negative inset, margin, sticky box 자체의 padding/border는 아직 허용하지 않는다.
- sticky box는 leaf element여야 한다. child box/text가 있으면 sticky displacement를 subtree 전체에 보존하는 후속 범위로 식별해 fail closed한다.
- sticky element는 현재 HostDocument parent를 containing block으로 사용한다. containing block은 static/relative Block element이며 부모의 used top/bottom padding과 border를 반영한 content box 안에 sticky box를 유지한다. 부모 border-box frame만 clamp에 쓰지 않는다. edge가 누락·음수·non-finite이거나 content height가 0 이하 또는 sticky box보다 작으면 fail closed한다.
- nearest ancestor scroll container 하나가 `overflow-y:auto|scroll`이고 양의 세로 scroll range를 가진다. 그보다 더 먼 scroll container의 이동은 sticky box와 nearest scrollport를 함께 이동시키며 sticky threshold 계산에는 nearest owner의 offset만 적용한다.
- scrollport/containing block geometry, sticky inset, scroll offsets, ancestor transforms 및 모든 결과는 finite CSS px여야 한다. tree·style·layout·scroll revision이 섞이면 전체 snapshot을 거부한다.
- sticky는 C12.4 paint model에서 `z-index:auto`라도 stacking context를 만든다. 제한 profile은 CSS stacking 전체를 구현하지 않으며 supported/unsupported trigger를 computed style과 source preflight 양쪽에서 판정한다.

모든 미지원 sticky 조합은 `position` 또는 해당 inset/ancestor property와 NodeId를 보이는 오류로 거부한다. static/relative fallback, viewport 고정 fallback, 0 offset, 이전 revision 재사용은 금지한다.

## 선행 계약 검증

- C12.1–C12.4의 position snapshot·Block geometry·paint list는 `origin/main`에서 사용할 수 있다. 현재 code는 `ComputedCssPosition::Sticky`를 Stylo snapshot에 보존하지만 style-to-layout projection과 `crates/spinon-style-to-render/src/stacking.rs`에서 sticky를 거부한다. 그 rejection을 성공 기본값으로 바꾸지 않고 C12.5 검증 경로에서만 좁혀야 한다.
- C13.2 내부 계약 0062는 `RuntimeScrollContainer`의 client scrollport, `ScrollStateRevision`, immutable snapshot, scroll offset request, frame/clip의 scroll ancestor transform을 제공한다. `RuntimeScrollRenderMetadata`에는 원 frame, scroll ancestor와 움직이는 clip layer가 보존된다. 따라서 C12.5는 C13 scroll plumbing을 복제하지 않고 sticky constraint metadata만 추가한다.
- C13.2의 `clip`은 offset을 만들지 않으며 C12.5는 C13.2의 clip/hit-test 결과를 유지해야 한다. ink-only overflow, C13.3 platform gesture, public `scrollTop`/`scroll` event는 sticky 계산의 전제나 이번 구현 scope로 끌어오지 않는다.
- C12.4 내부 계약 0058은 현재 sticky를 금지하고 있다. 구현 시 `position:sticky`의 auto stacking context 규칙을 같은 paint builder에서 명시적으로 표현하고 C12.4의 다른 trigger·order 동작은 보존한다.

## 구현 전 oracle

기준은 macOS의 Agent Browser가 띄운 HeadlessChrome 156에서 읽은 [고정 fixture](../tests/fixtures/css/c12/position-sticky.html)다. Agent Browser viewport는 1280×633 CSS px, DPR 1이고, fixture의 `html`/`body` content extent는 360×260px로 고정했다. body는 `overflow:hidden`이라 document scroll은 없고, 328×228 outer scroller 아래에 260×148 inner scroller, 36px leading block, 176px relative containing block, 28px sticky child가 있다. sticky child는 `top:8px`이다. viewport 크기에 의존하는 media query나 percentage sizing은 없다. fixture bytes SHA-256은 precomparison README에 고정한다.

Agent Browser에서 먼저 `outer.scrollTop=0`, `inner.scrollTop=0`의 frame을 읽고, 이후 outer 또는 inner offset 하나씩 바꾸어 두 scrollport의 frame, containing block frame, sticky frame, client top/height를 관찰했다. 고정된 observation은 [Chrome JSON](../spec/internal/evidence/c12-5-sticky-precomparison-2026-10-26/chrome156-observation.json), 실행 조건·hash·캡처 출처·한계는 [precomparison README](../spec/internal/evidence/c12-5-sticky-precomparison-2026-10-26/README.md)에 둔다.

고정 Chrome 결과의 필수 geometry는 다음과 같다.

| 상태 | outer / inner scrollTop | section top | sticky top | 판정 |
| --- | ---: | ---: | ---: | --- |
| 초기 | 0 / 0 | 118 | 118 | natural in-flow frame |
| outer 이동 | 24 / 0 | 94 | 94 | inner offset이 0이므로 inner 기준으로 pin하지 않음 |
| inner 이동 | 0 / 20 | 98 | 98 | threshold 전에는 content와 함께 이동 |
| inner 경계 | 0 / 36 | 82 | 90 | inner top constraint 도달 |
| inner pinned | 0 / 48, 96, 128, 160 | 70, 22, -10, -42 | 90 | nearest scrollport의 top에 유지 |
| containing-block 끝 | 0 / 212 | -94 | 54 | border/padding이 0인 기본 fixture의 content bottom 82에서 sticky height 28을 뺌 |

이 표는 자동화된 CSS geometry observation이며 CSS 의미 전체나 WPT 통과 결과가 아니다. 초기 browser capture를 임의로 다시 구성하지 않으며 재실행된 offset 시퀀스가 저장 artifact와 다르면 해당 artifact를 재생성하고 hash를 갱신한다. Chrome의 고정 frame 네 축은 지원 장면에서 `0.5 CSS px` 이내, 지정된 scrollport·NodeId·revision은 exact 비교한다. Unsupported controls는 정확한 property/node rejection이 pass 조건이다.

이 fixture에서 `#inner` frame top은 `74px`, border top은 `2px`, padding top은 `6px`다. Chrome의 pinned top은 `74 + 2 + 6 + top(8) = 90px`이다. C13.2 `scrollport_css_px`는 border를 제외한 client area 시작(`76px`)을 저장하므로 sticky 계산은 computed padding top을 더해야 한다. 이 차이를 무시하고 `scrollport_y + top`을 사용하면 Chrome보다 6px 위에서 stick해 틀린다. containing-block lower bound는 current containing-block visual content bottom에서 sticky border-box height를 뺀 값이며, 기본 fixture 마지막 sample은 `82 - 28 = 54px`이다.

Chrome 비교에서 부모 경계도 확인한다. 동일한 fixture의 relative Block `#section`에 `padding-bottom:12px; border-bottom:2px solid #333`를 넣으면 border-box bottom은 계속 `82px`지만 content edge는 `68px`이다. [추가 고정 fixture](../tests/fixtures/css/c12/position-sticky-parent-content-edge.html)는 `inner.scrollTop=212`에서 sticky top `40px`, bottom `68px`를 요구한다. 이 sticky box는 inner client clip top `76px` 위에 있어 마지막 offset에서는 paint가 잘린다. Chrome에서 `(200,78)`의 hit target/pixel은 `section` / `(245,245,245,255)`이고 `(240,100)`은 `section-after` / `(204,221,238,255)`다. 두 RGBA는 Agent Browser screenshot에서 직접 sample했다. geometry·hit target·screenshot은 [추가 Chrome observation](../spec/internal/evidence/c12-5-sticky-precomparison-2026-10-26/chrome156-observation.json)에 고정한다.

## 런타임 및 fixture 판정

1. 계획 실패 관점 20개를 코드 전에 독립 기록하고 발견 결함을 반영한다.
2. Chrome fixture·관찰 JSON·hash가 저장소와 맞는지 확인한다. fixture의 case ID와 예상값은 Chrome observation과 독립 계산으로 고정하고, Spinon 결과로 재생성하지 않는다.
3. `RuntimeBlockFlexStackingV1` C12.4 전체 범위를 넓히지 않는 opt-in `c12-5-sticky-fixture-test` route에서 target/control을 실행한다. default/product feature에는 fixture entry point와 scene payload가 없어야 하고 fixture-test build에는 positive control이 있어야 한다.
4. static flow control, top threshold 직전/도달/이후, nearest-vs-outer nested scroller, border-box와 padded/bordered content-edge containing-block 종료, reverse offset, no-scroll-owner, scroll owner removal/reparent, stale scroll revision, hidden ancestor, unsupported inset/writing mode/ancestor effect, invalid/non-finite/inverted parent edges를 포함한다. Reverse offset 기대값은 현재 scroll offset만으로 판정하는 runtime state invariant이며 Chrome에서 왕복을 관찰한 값이라고 주장하지 않는다. 왕복 뒤 같은 final offset의 Chrome 관찰값과 다르면 runtime 비교를 중지하고 Agent Browser precomparison을 먼저 추가한다.
5. 각 supported frame은 CSS px node별로 Chrome과 비교한다. render frame, paint clip, hit-test clip 및 fixed `NodeId`는 같은 `ScrollStateRevision`을 가리켜야 한다. old revision frame/hit-test 결과를 새 scroll revision으로 허용하지 않는다. 한 장면의 frame 오차는 `0.5 CSS px` 이하이고 판정 대상 position/inset/owner/revision identity는 exact다.
6. Android API 37 ARM64 AVD와 iOS 26.2 Simulator에서 실제 V8→Stylo→layout→render snapshot 경로를 각각 확인한다. Android PixelCopy·iOS Metal drawable 표본은 사전 고정 색 box의 내부 좌표를 읽어 Chrome RGBA와 비교한다. AVD software renderer, Simulator 결과는 실기기/hardware GPU/compositor 이후 검증으로 확대하지 않는다.
7. 기존 C12.1–C12.4와 C13.2 scroll geometry/offset/clip 회귀, Rust focused/workspace tests, Clippy, rustfmt와 Android/iOS simulator builds를 실행한다. 기본/fixture artifact 경계를 확인한다. parent edge runtime은 inner=212에서 `(200,78)` section `(245,245,245,255)`와 `(240,100)` section-after `(204,221,238,255)` PixelCopy/Metal 표본을 확인한다. 실행하지 않은 WPT·실기기·제품 CSS admission은 미검증으로 적는다.
8. 코드/runtime용 새 실패 관점 20개와 별도 PR 변경 실패 관점 30개를 수행한다. 계획 검토 표를 구현/PR 리뷰로 재사용하지 않는다.

## 제외 항목

양 축 sticky, `bottom`/`left`/`right`, percentage/`calc()` inset, negative inset, oversized sticky box, sticky box child, sticky box margin/border/padding, 여러 sticky sibling의 상호 제약, 서로 다른 scrollport axis, hidden/clip scroll ancestor의 sticky behavior, root viewport sticky, nested writing mode/RTL, transform/contain/filter fixed containing effect, fragmentainer, table/Grid/Flex/replaced/text, actual user touch/wheel, CSSOM, public DOM event, WPT suite 및 전체 C12.5/C12/CSS 완료는 제외한다. 성공적으로 구현된 고정 profile은 일반 author CSS 경로로 승격하지 않는다.

## 표준 및 고정 비교 모델

- [CSS Positioned Layout Module Level 3, 2025-10-07 Working Draft](https://www.w3.org/TR/2025/WD-css-position-3-20251007/)의 sticky positioning 규칙을 normative input으로 읽고 pinned Chrome 156 관찰과 별도 기록한다. 이 문서의 고정 date는 실행 기준 재현을 위한 snapshot이지 최신 Working Draft를 대표한다고 주장하지 않는다.

## 단계와 상태

- [x] Chrome 사전 비교·fixture hash를 고정한다.
- [x] 계획 실패 관점 20개를 검토하고 차단 결함을 반영한다.
- [x] 내부 계약 0082를 추가한다(숫자 버전 `0.1.0` 고정).
- [x] 계획 검토 뒤 제한 runtime slice를 구현하고 fixture 경계를 확인한다.
- [x] Android API 37 ARM64 AVD PixelCopy와 iOS 26.2 Simulator Metal drawable에서 source-bound 실행 근거를 수집한다.
- [x] 최종 source fingerprint `c31f9f53…`에 별도 implementation/runtime review 20개를 새로 고정했다. 이전 `5dbd898c…` review는 이전 동작 검토 기록으로 보존한다.
- [ ] 최종 main base에서 변경 관점 PR diff review 30개를 완료한다.
- [ ] C12.5 parent checkbox와 전체 C12/CSS 공식 count는 더 넓은 계약이 닫히기 전 미완료로 유지한다.

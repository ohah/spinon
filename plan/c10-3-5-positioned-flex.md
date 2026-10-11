# C10.3.5 · 위치 지정 Flex 자식 계획

**상위:** [C10.3 Flex 순서·정렬](c10-3-flex-order-alignment.md) · [C12 위치 지정](c12-positioning.md) · [공식 상태 대장](../spec/STATUS.md)
**현재 상태:** 제한된 계산·paint 경로 구현 및 모바일 smoke 완료 · [PR #132 리뷰 중](https://github.com/ohah/spinon/pull/132)
**내부 계약 숫자 버전:** 출시 전 `0.1.0` 고정

고정 Chrome 비교는 20개 case·70개 source node 전체를 DPR 1·2로 수행했다. Android 실기기와 iOS Simulator는 같은 Chrome 입력에서 직접 Flex child, paint 겹침, Block wrapper를 포함한 3개 case·15개 node를 실행했다. 두 앱의 V8 frame은 Chrome과 최대 오차 0 CSS px이고 각 GPU 표면에서 16개 상자를 제출했다. WPT suite, 전체 Chrome 행렬의 모바일 재실행, GPU 픽셀 동일성은 아직 검증하지 않았다.

## 목표

지원 runtime Flex 경로에서 absolute 자식의 static-position 계산을 Flex 규칙에 맞추고, in-flow Flex item과 paint 순서를 합친다. 이 단계는 C12.2의 Block absolute 구현을 확장하지만, Block static-position을 Flex 정렬로 바꾸지 않는다. 구현 완료나 공개 CSS 지원으로 간주하지 않는다.

## 소유 관계

세 관계를 별도 입력으로 유지한다.

1. **HostDocument source parent:** DOM 유사 트리에서 자식이 실제로 속한 노드다. 재부모화하지 않는다.
2. **static-position formatting owner:** absolute 자식의 CSS source box parent다. Flex container의 직접 box 자식이면 Flex static-position 규칙을 적용한다. 그 사이에 Block wrapper가 있으면 wrapper의 Block formatting context 규칙을 유지한다.
3. **absolute containing-block owner:** CSS 위치 지정 규칙에서 선택한 가장 가까운 containing block이다. static-position formatting owner와 같다고 가정하지 않는다.

현재 `positioning.rs`의 `flex_source_parent || flex_owner` 거부 조건은 두 번째와 세 번째 관계를 합친다. 이 단계는 해당 분기를 고치되 다음을 별도 관찰한다.

- **직접 Flex 자식:** source box parent가 Flex container다. Flex static-position 규칙을 적용한다. containing block이 그 Flex container인지, 더 바깥의 positioned ancestor인지 각각 비교한다. 다른 owner인 입력에서는 Flex 좌표를 containing-block 좌표로 명시적으로 옮긴다.
- **Block wrapper 후손:** source box parent는 Block wrapper이고 containing block만 Flex ancestor일 수 있다. 이 경우 Flex의 `justify-content`, `align-self`, `order: 0`을 상속하지 않는다. C12.2의 Block static-position 의미와 owner 좌표 변환을 보존한다. Chrome 기준과 현재 adapter가 일치하지 않으면 성공을 가장하지 않고 오류를 유지한다.
- `display:contents`, anonymous item, pseudo-element 또는 box parent를 확정할 수 없는 입력은 별도 CSS box-generation 계약 전까지 fail-closed한다.

## 계산 계약

- absolute 자식은 Flex line collection, wrapping, gap, flex grow/shrink, 부모의 used size 및 in-flow 형제 frame을 바꾸지 않는다. 추가·삭제 전후에 이 값들의 불변성을 비교한다.
- 직접 Flex 자식의 static-position rectangle은 Flex container content box에 맞춘다. 교차축의 양 끝은 content edges다. 주축 위치는 자식이 사용 크기가 고정된 유일한 Flex item인 것처럼 계산한다. 이 static-position 계산에서 자식의 auto margin은 0으로 취급한다.
- `justify-content`와 자식의 `align-self`는 auto inset이 있는 축의 위치를 정한다. 각 축에서 두 inset이 모두 auto일 때만 그 축의 static position을 사용한다. 한쪽 inset이 definite이면 기존 C12.2의 absolute inset 해법을 적용한다.
- 실제 자식의 사용 크기, authored/computed inset, static-position offset, 최종 containing-block 기준 frame을 별도 값으로 기록한다. Flex 정렬을 auto width/height의 stretch 계산이나 intrinsic sizing으로 오해하지 않는다.
- 첫 지원 slice는 fixed-size Flex container와 fixed-size element child로 한정한다. `horizontal-tb`·LTR, 물리 inset, 유한 CSS px 값만 성공 입력으로 둔다. percentage/intrinsic size가 기존 C12.2 지원 범위 밖이면 함께 지원한다고 추정하지 않는다.
- `align-self:auto`, `stretch`, baseline, safe/unsafe overflow, `justify-content:normal` 및 분배 값의 구체 동작은 열거형 매핑으로 추정하지 않는다. pinned Chrome reference로 관찰한 지원 값만 구현 계약에 고정하고, 지원하지 못하는 값은 해당 노드·속성의 구체 오류로 닫는다.

## Flex item 순서와 paint phase 계약

- 각 Flex container의 **in-flow Flex item**은 computed `order` 오름차순, 같은 값이면 source order로 페인트한다. 이 시각 정렬은 HostDocument child order, DOM 유사 조회, 이벤트·접근성 순서를 바꾸지 않는다.
- absolute 자식은 out-of-flow이므로 Flex item이 아니다. computed `order`는 보존하지만 paint 순서 계산에는 사용하지 않는다. 고정 Chrome 154 관측과 WPT `flexbox-paint-ordering-003.html`은 authored `order`가 서로 다른 absolute Flex 자식 사이에서도 source tree order가 유지됨을 확인한다.
- 현재 제한 profile의 `z-index:auto` positioned box는 in-flow paint 뒤 positioned phase에서 그린다. positioned 형제는 같은 paint context의 source tree preorder를 따른다. 겹치는 opaque-box fixture와 node별 scene order로 검증하며, 별도 stacking context·`z-index`는 이 계약에 포함하지 않는다.
- in-flow Flex item의 non-positioned descendant subtree는 해당 item의 order-modified 순서 안에 둔다. 그 subtree의 absolute descendant는 in-flow subtree에서 분리해 positioned phase에 모으고 source tree preorder를 보존한다. nested Flex의 `order`를 전역 정렬하지 않는다.
- `display:none` 노드는 layout frame과 paint list에서 제외하고 source tree에는 보존한다. absolute 자식이 Flex 계산에서 제외되더라도 필요한 CSS·paint 정보와 NodeId/revision은 잃지 않는다.
- 이 단계는 positioned `z-index`, stacking context, transform/opacity/clip, scroll clipping, hit-test·pointer target, 접근성 순서를 구현 완료로 주장하지 않는다. 겹침 fixture는 기존 불투명 단색 box만 사용한다.

## 구현 경계

- 현재 HostDocument source tree와 C12.2 layout graph를 분리한다. Flex layout에 넣는 자식 목록과 absolute box 배치 목록을 구분한다.
- absolute direct Flex child가 Flex layout input에 잘못 들어가면 Flex line 수·gap·형제 frame 회귀로 실패한다. 반대로 paint에서 빠지면 node-set 검증과 GPU 캡처가 실패한다.
- static-position owner, containing-block owner, source parent, ancestor frame, style revision이 누락되거나 서로 다른 revision이면 부분 frame을 내보내지 않는다. 기존 오류·commit atomicity 계약을 보존한다.
- `order`의 typed profile 범위는 C10.3.2 계약을 따른다. absolute child의 authored `order`가 paint에 영향을 주지 않는다는 것과 CSS `order` computed value 보존은 구분한다.

## 비교 기준과 fixture

### 고정 환경

- Oracle: Chromium `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`. 실행 파일과 fixture SHA-256을 저장하고 기준을 자동 갱신하지 않는다.
- viewport `320×240 CSS px`, DPR 1·2, `horizontal-tb`, LTR, `en-US`, UTC. geometry의 최대 node별 절대 오차는 `0.5 CSS px`; 누락·중복 NodeId와 잘못된 owner는 별도 실패다.
- WPT source reference는 저장소의 C10.3.1–C10.3.4 inventory와 같은 commit `d5a765f1089ce6d3f72300281481edf3dddff7f3`로 고정한다. suite 실행 여부는 reference 경로 확인과 별도로 기록한다.
- runtime profile은 C10.3의 여섯 profile을 모두 검사한다: `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1`. 기존 profile 중 절대 위치를 허용하지 않는 경로는 명시적인 거부를 검증하며 성공 범위로 승격하지 않는다.

### WPT 대응 후보

아래 경로는 WPT input inventory에 매핑하며 이 계획에서 suite 전체를 실행했다고 주장하지 않는다.

- static position 기본·축: [`abspos/abspos-autopos-htb-ltr.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/abspos/abspos-autopos-htb-ltr.html), [`abspos/dynamic-align-self-001.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/abspos/dynamic-align-self-001.html)
- `align-self`·`justify-content`·margin: [`abspos/flex-abspos-staticpos-align-self-001.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/abspos/flex-abspos-staticpos-align-self-001.html), [`abspos/flex-abspos-staticpos-justify-content-001.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/abspos/flex-abspos-staticpos-justify-content-001.html), [`abspos/flex-abspos-staticpos-margin-001.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/abspos/flex-abspos-staticpos-margin-001.html)
- descendant 경계: [`abspos/abspos-descendent-001.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/abspos/abspos-descendent-001.html)
- paint 순서: [`flexbox-paint-ordering-001.xhtml`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/flexbox-paint-ordering-001.xhtml), [`flexbox-paint-ordering-003.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/flexbox-paint-ordering-003.html)

### 고정 Chromium fixture

1. `row`, `row-reverse`, `column`, `column-reverse`에서 main/cross axis의 `justify-content`·`align-self` static position.
2. wrap/nowrap, 한 줄/두 줄, `align-content` 차이에서 absolute child가 line 계산에 기여하지 않는지 확인.
3. `align-self:auto|flex-start|center|flex-end|stretch`, pinned Chrome이 수용하는 safe/unsafe·baseline 및 overflow 위치의 cascade 결과.
4. inset 양 축 auto, 한 축 auto/반대 축 definite, 네 inset definite를 나눠 static-position 사용 여부를 확인.
5. fixed used child size, signed margin, auto margin, border/padding을 둬 static rectangle과 containing block geometry를 분리.
6. source parent가 Flex인 direct child, Flex ancestor 아래 Block wrapper descendant, Flex source parent와 별도 positioned containing block, ancestor Flex owner와 Block source parent 조합.
7. `order` 음수·0·양수 in-flow 형제와 authored order가 다른 absolute 형제를 겹친다. in-flow 형제는 order-modified 순서, absolute 형제는 source tree 순서로 페인트되는지 각각 확인한다. positioned descendant가 in-flow Flex item subtree에 섞이지 않는지도 둔다.
8. `display:none`, invalid cascade/fallback, missing style/owner, stale revision 및 일부 frame만 만들어지는 실패 입력.
9. absolute child 유무·order 변경 전후 in-flow node frames와 Flex line 결과 불변, Chrome node geometry 및 paint phase 일치.

fixture 수와 node 수는 HTML/CSS를 고정한 다음 확정한다. Chromium oracle, 손계산 가능한 불변식, WPT path 존재 확인은 각각 별도 필드로 보존한다. WPT pass로 보고하지 않는다.

고정 inventory는 20개 case·70개 고유 node다. 입력 HTML/CSS, Chrome 실행 파일, capture 도구 digest는 reference JSON에서 고정한다. 모바일 앱에는 전체 fixture 대신 아래 3개 case를 source style 그대로 연결한다.

- `direct-row-center-end`: 직접 absolute Flex child의 static position
- `paint-order`: in-flow `order`와 authored `order`가 다른 absolute 형제의 겹침
- `block-wrapper-static-position`: Flex ancestor 아래 Block wrapper 후손의 Block static position

runtime host가 허용하는 단색 paint 선언을 위해 이 subset의 `background` shorthand만 같은 색의 `background-color`로 정규화한다. 이 smoke는 background shorthand parser 자체를 검증하지 않는다. viewport는 CSS `320×240`이고 root 포함 16개 runtime box의 NodeId/frame을 비교한다. 15개 fixture node의 기대 좌표는 pinned Chrome observation에 case별 viewport offset을 더해 만들며 비교 도구는 최대 오차 `0.5 CSS px`를 적용한다.

## 모바일 실행 관문

같은 고정 fixture subset을 실제 V8→Stylo→typed style/layout→Taffy→WGPU 경로로 실행한다. Android SM-S731N / Android 16 API 36 실기기는 Samsung Xclipse 940 / Vulkan을 선택했다. iPhone 17 Pro / iOS 26.2 Simulator는 Metal backend를 선택했다. 각 플랫폼의 15개 fixture node는 pinned Chrome과 최대 오차 0 CSS px였고, root 포함 `presented boxes=16`을 기록했다. [로그·비교 JSON·캡처](../spec/internal/evidence/c10-3-5-positioned-flex-2026-10-11/README.md). 전체 Chrome fixture 행렬, 전체 WPT suite, 실기기 iOS, 실기기 성능, GPU pixel equality를 별도 측정 없이 추정하지 않는다.

## 미지원·오류 경계

이 단계는 Grid child, RTL·다른 writing mode, text/anonymous/replaced item, intrinsic 또는 불명확한 used size, `display:contents`, fixed/sticky, positioned `z-index`·stacking context, transform·contain·filter, opacity/clip/scroll clipping, hit-test와 접근성 order를 지원하지 않는다. 필수 owner, style 또는 revision이 없거나 좌표 변환을 보존할 수 없으면 해당 노드의 position/layout 오류로 실패하고 이전 유효 frame을 원자적으로 유지한다.

## 완료 조건과 순서

1. 구현 전에 이 계획의 20개 실패 관점 검토를 완료하고, 고정 Chrome reference와 WPT path inventory를 만든다.
2. fixture 기준이 독립 불변식과 일치한 뒤 `0056 · C10.3.5 positioned Flex child` 내부 계약 초안을 작성한다. 입출력, profile, 오류, revision, source/owner 관계와 예제를 포함하고 숫자 버전 `0.1.0`을 유지한다.
3. 계산 tree·paint preorder·runtime handoff를 각각 수정하고, Rust·CSS reference·V8 report·Android 실기기·iOS Simulator를 검증한다.
4. 구현 PR은 코드·fixture·오류·실제 runtime을 대상으로 계획 검토와 겹치지 않는 새 실패 관점 20개를 검토한다. 계획 문서 표를 구현 검토로 재사용하지 않는다.
5. `spec/STATUS.md`·내부 계약 index·roadmap의 구현 완료 표시는 근거가 생긴 뒤 갱신한다. C10.3 parent는 C10.3.5와 C15 텍스트 baseline 연결 전까지 미완료다.

## 기준 자료

- [CSS Flexbox Level 1 §4.1 absolutely-positioned flex children](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#abspos-items): absolute 자식은 Flex layout에 참여하지 않고, static-position rectangle은 content edge와 단일 used-size Flex item 동작으로 정의한다.
- [CSS Flexbox Level 1 §4.3 Flex item z-order](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#painting): `order`-modified document order는 Flex item paint에 적용되며 absolute 자식은 out-of-flow라 Flex item이 아니다.
- pinned [WPT `flexbox-paint-ordering-003.html`](https://github.com/web-platform-tests/wpt/blob/d5a765f1089ce6d3f72300281481edf3dddff7f3/css/css-flexbox/flexbox-paint-ordering-003.html): authored `order`가 서로 다른 overlapping absolute Flex 자식의 paint가 `order`를 무시하고 source order를 따른다는 기준이다. suite 실행은 별도 상태로 기록한다.
- [CSS 2 stacking-context painting order §E.2](https://www.w3.org/TR/CSS2/zindex.html#painting-order): 제한 profile의 `z-index:auto` positioned descendants paint phase를 구분하는 기준이다.
- [CSS Flexbox Level 1 §5.4 `order`](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#order-property): source order를 바꾸지 않는 visual reordering 경계를 따른다.
- 저장소의 C12.2 [내부 계약 0055](../spec/internal/0055-c12-2-absolute-block.md), [C10.3.2 내부 계약 0051](../spec/internal/0051-c10-3-2-flex-order.md), 잠긴 Taffy `0.14.0` 구현을 대조한다. Taffy 결과를 CSS oracle로 간주하지 않는다.

# 0074 · C15.3 runtime glyph scroll ancestor transform

**버전:** `0.1.0` · 내부 초안<br>
**상태:** 고정 C15.3 fixture의 runtime text snapshot에서 ancestor scroll 이동을 연결 · 일반 DOM text 지원 아님<br>
**연결:** [C15 계획](https://github.com/ohah/spinon/blob/main/plan/c15-inline-text-layout.md) · [공식 상태 대장](../STATUS.md#css-구현-체크리스트) · [scroll transform Chrome 기준](./evidence/c15-3-text-scroll-transform-precomparison-2026-10-17/README.md) · [runtime glyph paint](./0072-c15-3-wgpu-glyph-paint.md) · [owner overflow hit-test](./0073-c15-3-overflow-text-owner-hit-test.md)

## 범위

이 계약은 이미 admission된 `RuntimeRenderText`가 하나 이상의 non-zero scroll ancestor를 가질 때 immutable `RuntimeRenderSnapshot`에 적용하는 glyph origin 이동과 owner paint clip 교차를 정의한다. text 자체가 scroll container인 경우는 계속 거부한다. DOM text admission, CSS style-to-glyph projection, scrollbar paint, glyph별 text hit target, DOM event dispatch를 정의하지 않는다.

## 입력과 invariant

- 입력은 기존 text owner `NodeId`, 같은 snapshot key를 가진 runtime text source, base CSS origin, base text clip, ancestor scroll owner 목록, 그 owner를 참조하는 scroll metadata와 고유 owner를 가진 `RuntimeScrollContainer` 집합이다.
- `RuntimeRenderSnapshot`은 base render box에서 현재 offset을 먼저 적용한다. 각 text node의 metadata ancestor 목록은 모두 scroll container에 존재해야 하며 owner 자체가 scroll container이면 admission을 거부한다.
- 현재 text origin은 항상 `base_origin - sum(ancestor_offsets)`로 계산한다. 이전 snapshot의 활성 origin에 다음 delta를 누적하지 않는다. repeated offset 교체·reset은 base origin을 보존해야 한다.
- text paint clip은 `base_text_clip.intersect(active_owner_box.paint_clip)`이다. active owner clip은 해당 scroll layer의 offset을 반영한 현재 scrollport 좌표를 따른다. text translation은 glyph origin에만 적용해 scrollport를 함께 움직이지 않는다.
- glyph bitmap, glyph order, source key와 device scale은 offset 변경으로 변하지 않는다. 새 snapshot은 동일한 paint command order와 text owner를 유지한다.
- ancestor id가 scroll container 집합에 없거나 ancestor offset 합이 non-finite이면 `InvalidScrollState`로 닫는다. transformed origin이 non-finite이면 `InvalidTextGeometry`로 닫는다. revision이 다른 text source는 기존 `InvalidTextSource` 경계를 따른다.

## 업데이트와 ownership

snapshot은 `base_boxes`, scroll metadata와 immutable glyph bitmap을 유지하고, offset이 바뀔 때 현재 render boxes와 text origin/clip을 다시 만든다. `with_scroll_offsets`는 같은 key의 scroll revision을 검사하고 새 `RuntimeScrollContainer` 상태로 snapshot을 재구성한다. 이 경로는 외부 surface나 GPU 자원의 소유권을 바꾸지 않으며, 이전 snapshot은 변경하지 않는다.

`scroll_ancestors` 목록의 유효성은 render snapshot 생성 시 각 항목이 등록된 scroll owner인지 확인한다. 실제 조상 관계와 clip-layer 구성은 layout adapter가 제공하는 metadata의 책임이다. 이 계약은 metadata가 잘못된 DOM tree 조상을 선언하지 않았는지 별도 증명하지 않는다.

## 고정 검증 사례

- horizontal: `scrollLeft 3→6→0`, text origin x는 base에서 `3→6→0` CSS px 이동한다.
- vertical: `scrollTop 18→7→0`, text origin y는 base에서 `18→7→0` CSS px 이동한다.
- nested: outer/inner x offset `4+5→2+3→0+0`, text origin x는 합계 `9→5→0` CSS px 이동한다.
- 매 상태에서 runtime owner hit-test는 보이는 Ahem `XX` owner를 반환한다. text paint clip은 각각의 effective scrollport에 고정되어 clip 밖 glyph paint와 hit을 허용하지 않는다.
- opt-in Android API 37 ARM64 AVD PixelCopy 및 iPhone 17 Pro/iOS Simulator Metal drawable은 같은 runtime report의 CSS sample 6개를 읽는다. overflow visible/hidden 표본 3개와 scrolled glyph ink 표본 3개를 고정 RGBA로 대조한다.

이 검증은 해당 고정 fixture 및 simulator surface에만 적용한다. C15.3, C15, C16 또는 C19 완료를 뜻하지 않는다.

## 제외 범위

text owner 자체의 scroll offset, 임의 DOM text, browser line box/layout과의 일반 동등성, glyph별 hit geometry, CSSOM `elementFromPoint()`, DOM event, scrollbar, font fallback, iOS compositor 제출 뒤 또는 Android 실기기 GPU 표면은 별도 계약과 검증 대상이다.

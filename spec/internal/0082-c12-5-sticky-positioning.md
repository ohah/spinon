# 0082 · C12.5 제한 sticky positioning

**문서 ID:** `0082` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 계획 검토 완료, 구현 중 · **공개 API:** 아님

`0082`는 문서 ID이며 제품·호환성 버전이 아니다. 이 문서는 `position: sticky` 전체 CSS 지원 또는 C12 전체 완료를 선언하지 않는다. 동작 범위, precomparison, 제한 profile과 남은 경계는 [구현 계획](../../plan/c12-5-sticky-positioning.md), [계획 검토 20개](./evidence/c12-5-sticky-plan-review-20-2026-10-26.md), [Chrome 사전 비교](./evidence/c12-5-sticky-precomparison-2026-10-26/README.md) 및 같은 evidence directory의 browser observation JSON에 결합한다.

## 범위와 선행 조건

이 계약은 C13.2의 immutable `RuntimeRenderSnapshot`, `RuntimeScrollContainer`, `ScrollStateRevision`, scroll offset request 및 paint/hit-test clip ancestry를 재사용해 한 개의 leaf sticky Block box를 세로 방향으로 조정하는 내부 fixture profile을 정의한다. C13.2 scroll state/clip의 의미를 다시 정의하거나 공개 CSSOM/API를 추가하지 않는다.

선행 입력은 C12.1–C12.4의 computed position, Block layout frame, `RuntimeBlockFlexStackingV1` paint order와 C13.2의 scroll geometry다. C13.2는 nearest scroll owner의 border-inside client scrollport와 outer scroll transform을 저장한다. 현재 main은 `ComputedCssPosition::Sticky`를 snapshot에서 보존하지만 style-to-layout projection 및 C12.4 style-to-render에서 거부한다. 해당 rejection은 아래 fixture 조건에서만 좁게 허용하며 이전 profile에 일반 fallback을 추가하지 않는다.

## 허용 profile

- `horizontal-tb`, LTR, Block root, 요소-only layout/paint tree.
- 정확히 한 leaf sticky box: computed `position:sticky`, finite nonnegative absolute `top` length; `right/bottom/left:auto`.
- sticky parent는 static 또는 relative Block containing block이며 leaf sticky box는 in-flow layout frame을 유지한다. 부모의 used border/padding은 허용하며 하한은 부모 border-box가 아닌 content edge다. top/bottom used edge, 합, content height는 finite·nonnegative여야 하고 content box가 sticky box보다 작거나 뒤집히면 fail closed한다.
- 가장 가까운 scroll ancestor는 `overflow-y:auto|scroll` 및 양의 세로 range를 가진다. farther ancestor scroll offset은 sticky와 scrollport의 공통 좌표 변환에 반영하고 nearest owner offset만 pin threshold를 바꾼다.
- sticky box는 `margin`, `padding`, border가 0이고 containing block과 scrollport 안에 들어갈 수 있는 크기여야 한다. 모든 coordinate, offset, used edge, sum과 result는 finite CSS px여야 한다.
- sticky box는 child box/text가 없어야 한다. subtree displacement, multiple sticky sibling, stack overlap contract는 이 버전에 포함하지 않는다.
- normal frame, containing-block frame, nearest scrollport, current `ScrollStateRevision`이 하나의 동일한 snapshot에 속해야 한다. 일부 box·clip·hit-test만 새 revision으로 게시하지 않는다.

다른 writing mode/direction, inline/text/Grid/Flex/replaced/table/anonymous content, unsupported ancestor containing-block effect, transforms/contain/filter, unsupported CSS source/at-rule, `bottom` 또는 양 축 inset, percentage/`calc()`/negative inset, zero-range/no scroll owner, hidden/clip-only nearest ancestor는 성공 profile에서 거부한다. 입력을 거부할 때는 가능하면 `NodeId`, source, property와 actual computed value를 전달한다.

## 계산 및 scroll snapshot

Layout은 sticky box를 normal-flow box로 계산해 line/Block sizing과 sibling placement를 유지한다. 성공 profile에서 computed `Sticky`를 임의로 `Static` 성공 결과로 소비하지 않는다. C12.5 adapter는 sticky identity와 top inset을 style/layout boundary를 넘어 보존한다. Renderer는 원본 frame, nearest scroll owner, containing-block border-box frame, content top/bottom inset, parent lineage 및 sticky inset을 `RuntimeScrollRenderMetadata` 또는 이에 준하는 immutable metadata에 저장한다.

각 scroll state update에서 renderer는 base layout frame에서 nearest·outer scroll offsets를 합산해 정상 visual frame과 containing-block visual bound를 구한다. nearest scrollport의 sticky top threshold는 border-inside scrollport origin을 farther scroll offsets만큼 이동한 뒤, pinned Chrome fixture에서 관찰한 padding 시작을 반영한다:

```text
sticky_threshold_y = scrollport_y + padding_top + computed_top_inset
```

여기서 `scrollport_y`는 C13.2 border-inside client scrollport origin이다. 실제 `position:sticky` box는 nearest owner의 자기 scroll offset을 threshold origin에 적용하지 않으며, farther scrolling ancestor의 이동은 scrollport와 box 양쪽에 공통으로 적용한다.

제한된 top-only, leaf, fitting-box profile의 visual top은 normal top이 threshold보다 위로 올라갈 때 threshold까지 내려가며, containing block의 visual content bottom에 box가 닿을 때 bottom bound가 우선해 다시 위로 밀린다. 계산은 `border_box.bottom - border_bottom - padding_bottom - ancestor_scroll_y`를 사용한다. content top/bottom edge, box size, scroll offset이 누락되거나 non-finite이거나 content height가 0 이하/box보다 작으면 전체 snapshot을 거부한다. target frame, clip과 hit-test geometry는 같은 adjusted sticky frame 및 scroll revision을 사용한다. `position:sticky`는 `z-index:auto`에서도 stacking context를 생성하므로 C12.4 context tree에 별도 node를 만든다.

구체 fixture 기준은 [Chrome JSON](./evidence/c12-5-sticky-precomparison-2026-10-26/chrome156-observation.json)에 둔다. 고정 scene의 기본 parent는 initial sticky top 118px, outer scroll 24/inner 0일 때 top 94px, inner scroll 20일 때 98px, inner scroll 36–160px일 때 top 90px, inner scroll 212px일 때 border-box parent의 content edge와 일치하는 top 54px다. 추가 [parent padding/border fixture](../../tests/fixtures/css/c12/position-sticky-parent-content-edge.html)는 부모 `padding-bottom:12px; border-bottom:2px`에서 section border box bottom 82px, content edge 68px, sticky top 40px를 요구하며 `(200,78)`의 hit target은 section이다. section CSS background에 따른 픽셀 기대값은 `(245,245,245,255)`다. 이 수치는 해당 고정 fixture에만 적용되며 일반 공식 값 목록이 아니다.

## revision·오류·격리

- `ScrollStateRevision` mismatch, stale document/style/environment/layout revision, duplicate/missing scroll owner, reparented/removed NodeId, unsupported sticky style 또는 non-finite geometry에서는 새 snapshot 전체를 오류로 반환한다.
- offset update는 C13.2와 동일하게 immutable snapshot으로 계산한다. 입력 `scrollTop`/`scrollLeft`, 공개 API, gesture callback이나 DOM event를 직접 발생시키지 않는다.
- `display:none` 자체/ancestor 아래 sticky는 paint tree와 scroll constraint에서 제외되며 viewport owner로 승격하지 않는다.
- C12.4의 computed z-index, visibility, paint ordering, source preflight 및 stacking trigger validation을 사용한다. sticky auto stacking context는 positive rule이며 다른 미지원 context trigger는 계속 거부한다.
- 이 profile의 FFI/scene/payload는 전용 `c12-5-sticky-fixture-test` build에서만 존재한다. 기본/product artifact는 fixture-free negative control, opt-in artifact는 positive control로 확인한다.

## 비교 및 검증 규칙

- 비교 원본은 [2026-10-26 Chrome precomparison](./evidence/c12-5-sticky-precomparison-2026-10-26/README.md)이다. 이 observation은 Agent Browser HeadlessChrome 156의 고정 input과 geometric output이며 WPT result나 Spinon 구현 output이 아니다.
- supported node frame의 axis 최대 차이는 `0.5 CSS px` 이하, computed position/top, nearest owner identity, revision tuple, node count와 clip/hit-test ownership는 exact다.
- reverse offset은 동일한 final scroll offset에서 동일 state를 복원하는 runtime invariant다. 현재 precomparison에서 Chrome reverse gesture를 직접 관찰하지 않았으므로 browser comparison이 필요해지는 결과가 있으면 추가 관찰 전까지 pass하지 않는다.
- Android 검증은 API 37 ARM64 AVD, iOS 검증은 iPhone 17 Pro / iOS 26.2 Simulator만 사용한다. AVD software renderer, iOS compositor 제출 전 Metal drawable sample은 실기기/hardware GPU/scanout 근거가 아니다.
- parent content-edge runtime sample은 `inner.scrollTop=212`에서 `(200,78)` section `(245,245,245,255)` / hit `section`, `(240,100)` section-after `(204,221,238,255)` / hit `section-after`를 요구한다. sticky top `40`은 client clip top `76` 위에 있어 paint가 clipping 된다.
- 구현 PR에는 별도 신규 implementation/runtime review 20개와 변경 관점 PR review 30개, C12.1–C12.4 및 C13.2 regression, rustfmt/Clippy, relevant Rust tests와 Android/iOS simulator build/runtime, default-vs-opt-in artifact boundary를 연결한다.
- Runtime fixture는 기본 parent와 content-edge variant를 각각 검증한다. content-edge variant에서 `inner.scrollTop=212`일 때 sticky border box는 `[40,68]`이고 inner client clip은 `y=76`에서 시작하므로 `(200,42)`는 clip 밖 hit-test null이어야 한다. `(200,78)`은 section `#f5f5f5`, `(240,100)`은 section-after `#cde`여야 하며 두 표본의 hit target과 RGBA는 Android PixelCopy 및 iOS Metal drawable에서 Chrome 156과 exact 비교한다.
- spinon WPT runner, 실기기, hardware GPU, 일반 author CSS admission, iOS compositor 이후 픽셀, CSSOM/public DOM event와 전체 C12/CSS 적합성은 이 계약의 검증 결과가 아니다.

## 관련 자료

- [C12.5 implementation plan](../../plan/c12-5-sticky-positioning.md)
- [Plan review · 20 perspectives](./evidence/c12-5-sticky-plan-review-20-2026-10-26.md)
- [Chrome precomparison](./evidence/c12-5-sticky-precomparison-2026-10-26/README.md)
- [C13.2 scroll state contract 0062](./0062-c13-2-scroll-state.md)
- [C12.4 stacking contract 0058](./0058-c12-4-stacking-context.md)

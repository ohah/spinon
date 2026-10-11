# 0057 · C12.3 viewport fixed positioning

**문서 ID:** `0057` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 제한 구현·플랫폼 검증 완료 · [PR #136 검토 중](https://github.com/ohah/spinon/pull/136) · **공개 API:** 아님

`0057`은 문서 ID이며 호환성 버전이 아니다. 이 계약은 CSS `position: fixed` 전체 지원이나 CSS 지원 목록의 완료를 뜻하지 않는다.

## 범위

`RuntimeBlockPositioningV1` profile에서 제한된 Block box의 fixed containing block을 CSS viewport로 계산하고, viewport 크기·device scale factor 변경에 따라 새 environment revision의 layout/render 결과를 만든다. 구현 경로는 V8 fixture → Stylo computed-style snapshot → Rust layout projection → Taffy → 기존 WGPU runtime이다. 이 fixture만으로 앱 전체 DOM/CSS 또는 성능을 주장하지 않는다.

성공 입력은 `horizontal-tb`·LTR와 Block root를 사용한다. pinned Chromium inventory에서 뽑은 parent-closed 28-node fixture에는 viewport inset, percentage·`calc()`, 음수·zero inset, stretch, box sizing·margin·padding·border, min/max, static/relative/absolute/fixed ancestor, fixed 안의 absolute 자식, fixed descendant, `display:none`과 `aspect-ratio:2 / 1` 한 조합이 들어 있다. aspect ratio fixture는 definite width `40px`, `height:auto`에서 `40×20 CSS px`가 되는 경우만 포함한다.

## 계산과 소유 관계

- CSS `position: fixed` 의미는 `LayoutPosition::Fixed`로 보존한다. Taffy는 fixed 전용 position 값이 없으므로 계산 parent를 구성할 때만 Absolute 방식으로 투영한다. 실제 owner 선택은 이 변환과 독립적이다.
- fixed containing block은 viewport origin `(0, 0)`과 `CssViewport.width_css_px`·`height_css_px`를 사용한다. top/bottom percentage는 viewport 높이, left/right percentage는 viewport 너비를 basis로 한다.
- 일반 `static`·`relative`·`absolute` ancestor 및 fixed ancestor는 fixed descendant의 owner를 바꾸지 않는다. fixed box는 absolute 자손의 containing block이 되지만 fixed 자손은 계속 viewport를 사용한다.
- HostDocument source parent, sibling 순서, NodeId, 이벤트·수명 소유권은 유지한다. synthetic viewport root와 owner graph는 계산 중에만 존재한다.
- `display:none` 자신 또는 그 조상 아래의 fixed box는 viewport root로 승격하지 않는다. 숨겨진 box는 frame을 만들지 않고 성공 제출 장면에서도 제외한다.
- 동일 viewport와 device scale factor의 중복 입력은 environment revision을 올리지 않는다. resize 또는 scale 변경은 새 revision을 사용하며 최종 GPU 제출도 해당 revision이어야 한다.

## 성공·실패 경계

- 물리 `top`·`right`·`bottom`·`left`, `inset`, 제한 길이·percentage·typed CSS math와 C12.2에서 측정 가능한 box size 조합만 허용한다. axis에 따라 두 inset이 모두 `auto`여서 static position 계산이 필요한 입력, fixed layout root, Flex/Grid static position, text/replaced intrinsic sizing은 성공 처리하지 않는다.
- 계산 가능한 fixed containing-block ancestor 효과는 typed computed snapshot에서 다시 검사한다. 지원 profile에서 `transform`, individual transform, `perspective`, `filter`, `transform-style: preserve-3d`, 관련 `will-change` 등 효과가 발견되면 전체 layout을 거부한다.
- 현재 고정된 Stylo 빌드에서 computed API가 없는 일부 속성이나 `UnknownProperty` cascade 진단은 computed effect marker로 간주하지 않는다. author/inline source 진단을 보존하고 C12.3 projection 경계에서 전체 계산을 거부한다. 진단을 무시해 viewport owner를 선택하지 않는다.
- 지원되지 않는 CSS 속성·단위, 잘못된/non-finite viewport, stale document/style/environment revision, 누락·중복·다른 owner의 node/frame, partial graph는 성공 결과나 이전 frame과 섞지 않고 오류로 종료한다.
- 이 계약은 scroll offset 고정, visual viewport/zoom, safe area, 키보드 viewport, clipping·hit test, stacking·`z-index`, fixed paint order, border stroke, WPT 전체 통과, iOS 실기기, hardware GPU 성능을 정의하지 않는다. scroll은 C13, stacking은 C12.4 범위다.

## 비교 기준과 증거

- oracle은 Chromium `154.0.8037.98`, revision `b859317bf11f6be47f9b7799ec690a0a42a1fb33`이다. 고정 전체 inventory는 22 case·79 node, `360×800`/`390×844` CSS px와 DPR 1·2·2.625·3 조건을 갖는다. 앱 runtime은 parent-closed 28-node subset만 실행한다.
- 각 플랫폼에서 8개 viewport×DPR 상태와 5개 resize/no-op/device-final 상태, 총 13개 상태·364개 frame을 comparator가 node ID로 대응한다. frame index와 HostDocument source 순서는 별도로 검증한다. 모든 `x/y/width/height`의 최대 허용 절대 오차는 `0.5 CSS px`다.
- Android 실기기 SM-S731N / Android 16 API 36 / 1080×2340 / density 450 / Samsung Xclipse 940 Vulkan에서 364 frame이 통과했고 최대 차이는 `0.009375 CSS px`였다.
- iPhone 17 Pro / iOS 26.2 Simulator / Metal에서 같은 364 frame이 통과했고 최대 차이는 `0.009375 CSS px`였다.
- 양쪽 마지막 제출은 `layout=ready`, `boxes=26`, `environment_revision=11`, viewport `360×800 CSS px`다. 각 platform 실행 근거와 화면은 [C12.3 실행 근거](./evidence/c12-3-fixed-positioning/README.md)에 있다.
- 전체 Rust workspace test·Clippy·rustfmt와 163개 CSS reference test, Android/iOS 앱 빌드는 저장소의 해당 PR 검증 결과와 함께 판정한다. 별도 C12.3 runtime comparator negative-control tests는 frame 누락·잘못된 revision·이전 실패 marker를 통과시키지 않는다.

## 관련 자료

- 구현 계획: `plan/c12-3-fixed-positioning.md`
- [고정 Chrome precomparison](./evidence/c12-3-fixed-precomparison-2026-10-11.md)
- C12.3 runtime fixture: `tests/fixtures/css/c12/runtime-position-fixed.js`
- Chromium reference: `tests/fixtures/css/references/c12-3-position-fixed-v1.json`
- [실제 V8 platform logs·screenshots·frame 비교](./evidence/c12-3-fixed-positioning/README.md)
- [구현 변경 실패 관점 검토](./evidence/c12-3-fixed-positioning-implementation-review-2026-10-11.md)
- [C12.2 absolute positioning 계약](./0055-c12-2-absolute-block.md), [C10.3.5 positioned Flex 계약](./0056-c10-3-5-positioned-flex.md)

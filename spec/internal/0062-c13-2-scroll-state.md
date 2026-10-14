# 0062 · C13.2 scroll state runtime 내부 계약

- **계약:** `0.1.0-draft` · **상태:** 계약·oracle 확정, 제한 구현·Simulator 검증 진행 중
- **제품 CSS 지원 선언:** 아님
- **기준:** [C13.2 계획](../../plan/c13-2-scroll-container-state.md) · [Chrome 사전 비교](evidence/c13-2-scroll-container-precomparison-2026-10-15/README.md)

## 범위

이 계약은 C13.1의 제한 block/flow-root/flex/grid box, 사각 clip, physical `horizontal-tb`·LTR profile에 내부 scroll state를 연결한다. `hidden`·`scroll`·`auto`의 scroll container, scrollport, 양의 CSS px offset 및 range를 계산해 불변 render snapshot과 hit test에 전달한다.

입력 profile은 유한한 CSS px geometry, nonnegative scroll origin, 현재 지원하는 DOM node 종류와 C13.1 clip 경계로 제한한다. 미지원 geometry나 효과는 부분 snapshot으로 통과시키지 않는다.

## Scroll owner와 range

- 연결된 `overflow-x/y:hidden`, `scroll`, `auto` element는 내부 scroll owner다. `scroll`·`auto`는 현재 range가 0이어도 owner이며, `hidden`은 사용자가 아닌 programmatic/internal 입력으로 움직일 수 있다.
- `overflow:visible`과 `clip`은 scroll state를 소유하지 않는다. 이들의 `scrollWidth`가 client box보다 클 수 있어도 내부 offset은 항상 0이다. `clip`의 CSSOM scroll size가 scroll state로 오인되지 않게 한다.
- scrollport는 border를 제외한 client/padding 영역이다. C13.4가 정할 classic scrollbar gutter와 thumb geometry는 이 계약이 소유하지 않는다. 각 실행은 실제 client size를 기록하고 Chrome Headless의 scrollbar 숨김 결과를 플랫폼 기준으로 일반화하지 않는다.
- `scrollable overflow area`는 같은 scrollport에 속하는 descendant border box의 양의 방향 overflow를 반영한다. positioned containing block이 해당 scrollport 안에 있는 absolute descendant는 포함하고, `box-shadow` 같은 ink-only extents는 제외한다. fixed descendant, transform, negative/RTL origin은 선행 owner 계약을 참조하거나 명시적으로 fail-closed한다.
- 단축/축별 overflow computed value는 C13.1 typed cascade 결과를 사용한다. 한 축이 넘쳐도 다른 축의 range를 만들지 않는다. offset 범위는 각 축 `[0, max(0, scrollableSize - clientSize)]`이며 CSS px로 저장한다.
- 초기 offset은 `(0,0)`이다. 동일한 연결 scroll owner의 layout/style 재계산 뒤 현재 offset이 새 range에 들어가면 유지하고, 벗어나면 새 max로 clamp한다. content 축소로 scroll range가 사라지면 해당 축 offset은 0이다. scroll owner 자체가 삭제되거나 연결 트리에서 분리된 뒤 재연결되면 이전 state를 재사용하지 않고 `(0,0)`에서 시작한다. descendant 이동은 owner를 바꾸지 않는 한 새 range에 맞춰 보존/clamp한다.

## Root viewport와 body

- `document.documentElement`와 `body`의 overflow 전파는 element scrollport와 별도 owner 선택 규칙으로 처리한다. CSS에서 `html` overflow가 `visible`이면 body의 overflow가 viewport로 전파될 수 있다.
- 표준 mode Chrome oracle에서 `document.scrollingElement`는 네 관찰 조합 모두 `HTML`이다. `html:visible; body:auto`일 때 body computed overflow는 auto여도 실제 viewport offset은 `window`/HTML에 있으며 `body.scrollTop`은 0이다.
- `html:hidden; body:auto` 대조에서는 viewport/HTML offset을 0으로 유지하면서 body box의 독립 scroll range와 body offset이 존재한다. `document.scrollingElement` 반환값과 현재 움직이는 내부 scroll owner를 같은 값으로 간주하지 않는다.
- viewport scroll은 일반 document content에만 적용한다. 현재 지원되는 fixed-position child는 viewport 기준 위치를 유지한다.
- 공개 `document.scrollingElement`, `Window.scroll*`, `Element.scrollTop/scrollLeft`, `scroll()/scrollTo()/scrollBy()/scrollIntoView()`와 scroll event는 C29/S05 owner다. C13.2 내부 fixture adapter는 offset/state 관찰용일 뿐 공개 CSSOM 지원을 뜻하지 않는다.
- Chrome Headless `Element.scrollLeft/scrollTop` setter의 `55.25→55`, `55.5→56` 반올림은 사전 oracle에만 기록한다. C13.2 내부 state/range는 CSS px coordinate를 보존하고, DOM CSSOM 입력의 normalization은 C29/S05 계약에서 정한다. 이 plan의 runtime offset parity case는 정수 offset으로 고정한다.

## Revision과 snapshot

- `ScrollStateRevision`은 문서 generation 안에서 단조 증가하며 scroll owner 집합·scrollport range·effective offset의 변경만 식별한다. document, style, environment revision을 재사용하거나 대체하지 않는다.
- immutable runtime render key와 hit-test input은 같은 `ScrollStateRevision`을 포함한다. scroll state가 달라진 뒤 이전 key의 paint/hit query는 stale로 거부한다.
- snapshot 생성은 layout의 scroll range 계산, 기존 offset 보존/clamp, root/body owner 선택, scroll revision 할당, paint/hit 변환을 하나의 후보 상태에서 수행한다. 모든 검증과 allocation이 성공한 후 한 번에 publish한다. 중간 오류는 이전 유효 snapshot과 offset map을 유지하고, 다음 유효 revision에서 전체 재계산으로 복구한다.
- nested owner의 offset은 각각 저장하고, descendant의 paint와 hit-test에는 같은 ancestor scroll chain을 적용한다. paint-only offset 또는 hit-only offset 상태를 만들지 않는다.
- 실제 offset/range/owner mapping이 동일하면 scroll revision은 유지한다. 값이 clamp된 뒤 결과가 이전 값과 같아도 range 또는 owner mapping이 바뀌었다면 scroll revision을 올린다.

## Oracle와 실패 처리

[C13.2 사전 비교](evidence/c13-2-scroll-container-precomparison-2026-10-15/README.md)는 다음 경계를 고정한다.

- overflow visible paint가 border를 넘어 보이는 장면과 `visible`의 비-scroll offset
- positive out-of-flow descendant의 scrollable extent와 ink-only shadow extent
- no-overflow `auto`/`scroll`, one-axis x/y, nested owner
- hidden programmatic offset, clip no-offset
- content growth 보존, shrink clamp, owner reparent reset
- root/body propagation, `document.scrollingElement`, viewport와 독립 body scroll offset, fixed-position negative control

입력 revision mismatch, non-finite/negative range, duplicate owner, overflow arithmetic, allocation failure, owner removed/moved, root/body ambiguity, stale paint/hit key는 오류 코드와 미공개 후보 상태로 남긴다. 부분 owner map, 일부 축만 clamp된 snapshot, 이전 style과 새 offset의 혼합을 허용하지 않는다.

## Build 경계와 완료 판정

C13.2 runtime fixture route, FFI, payload는 별도 opt-in fixture-test feature로 한정한다. default artifact에는 전용 symbol/source payload가 없고 opt-in positive control에만 존재하는지 각 플랫폼 archive에서 확인한다. 내부 fixture route는 공개 CSSOM, gesture input, scrollbar 또는 C13 전체 구현으로 보고하지 않는다.

완료 근거는 고정 Chromium observation에 대한 runtime oracle 비교, range/offset/revision/error 복구 단위 검증, 기본/opt-in artifact negative/positive control, Android API 37 ARM64 AVD 및 지정 iOS Simulator에서 같은 실제 runtime fixture 실행, 실행 뒤 회귀 workspace/Clippy/rustfmt를 포함한다. 실기기는 요구하지 않는다. Chrome/WPT 결과, AVD software renderer, iOS Simulator 표시는 각각 다른 증거로 기록한다.

이 계약의 통과는 C13.2 제한 fixture 범위만 가리킨다. C13.1 전체 완료, public CSSOM/events, gestures, scrollbar, scroll snap, RTL/vertical writing modes, WPT 또는 제품 전체 CSS 지원을 뜻하지 않는다.

## 현재 검증 범위 · 2026-10-15

Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 26-frame·9-owner initial matrix 및 growth/shrink/detach/reparent lifecycle을 실행했다. 화면에서 visible child의 border escape가 보이며, 내부 snapshot은 parent 외부 frame과 child hit identity를 검사한다. 양쪽 Simulator 실행 결과와 screenshot SHA는 [실행 근거](evidence/c13-2-scroll-container-implementation-2026-10-15/README.md)에 있다. workspace/Clippy와 default/opt-in artifact isolation은 통과했다. Rust tests에는 root/body geometry, stale scroll revision rejection, reserve failure의 atomic recovery가 있다. 후속 앱 검증은 [C13.2 app failure-control 근거](evidence/c13-2-app-negative-controls-2026-10-15/README.md)에 기록했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 앱 FFI 경로에서 예약 failpoint 오류 marker, 이전 scroll-state snapshot의 exact equality, stale presentation sequence rejection과 같은 host recovery를 확인했다. 이 검증은 OS OOM과 concurrent scheduler race를 재현하지 않는다. 픽셀별 표면 대조가 아니며, ink-only scene과 simulator pixel readback은 아직 닫히지 않았다.

구현 실패 경로 20개와 구현 PR 변경 관점 30개는 각각 [구현 검토 기록](evidence/c13-2-scroll-implementation-review-2026-10-15.md), [PR 변경 검토 기록](evidence/c13-2-scroll-implementation-pr-review-2026-10-15.md)에 있다. 앱 오류 원자성 후속 작업은 별도 [구현 검토 20개](evidence/c13-2-app-negative-controls-implementation-review-20-2026-10-15.md)와 [PR 변경 검토 30개](evidence/c13-2-app-negative-controls-pr-review-30-2026-10-15.md)를 거친다. 리뷰 중 찾은 `display:none` subtree scroll owner 누락은 수정했고 전체 workspace와 두 Simulator를 다시 확인했다.

# C11.4 parsed Root HTML repeated intrinsic implicit-row list

**상태:** Chrome 157 사전 비교와 독립 계산 4/4를 고정했다. 구현 전 계획 실패 관점 검토를 마친 뒤 같은 기본 parsed Root HTML route의 제한 profile을 확장한다. C11.4/C11 상태 체크와 공식 `78/214` 집계는 유지한다.

**상위:** [공식 Grid 상태](../spec/STATUS.md#c11-css-grid) · [C11 CSS Grid 계획](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 명세 0059](../spec/internal/0059-c11-grid-layout.md)

## 결손과 목표

현재 `RootHtmlFitContentAutoRowsV1`은 `fit-content()`·`min-content`·`max-content`·`auto`를 각각 한 번 포함하는 네-member 목록 24순서를 지원한다. `min-content`와 `max-content`를 한 목록 안에서 반복하는 값은 CSS Grid의 `<track-size>+` 문법상 유효하지만 현재 profile은 거부한다. 현재 runtime topology도 이 네-member 목록에 여섯 visible child까지만 허용해, 여덟 implicit row가 같은 여섯-member pattern을 순환하는 경로는 받지 않는다.

이 slice는 다음 **하나의 정확한 여섯-member 패턴**과 해당 패턴의 첫 반복을 기본 parsed Root HTML GPU route에 연결한다.

```css
fit-content(<finite non-negative px | direct percentage>)
  min-content max-content auto min-content max-content
```

`grid-auto-rows`의 첫 implicit row부터 이 여섯 track-size가 순서대로 적용되고, 일곱째 implicit row는 첫 `fit-content()` member, 여덟째는 반복된 첫 `min-content` member를 사용한다. 두 `min-content` track과 두 `max-content` track은 각자의 row item contribution으로 계산한다. 이 결과는 이 exact order와 topology의 제한 구현이다. 임의 순서·임의 반복·일반 CSS admission을 뜻하지 않는다.

## 규범 및 사전 비교 모델

- 규범은 [CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks)이다. `grid-auto-rows`의 값은 `<track-size>+`이고, 여러 값을 지정하면 implicit track을 채울 때 패턴을 필요한 만큼 반복한다. 첫 implicit track은 첫 member를 쓴다.
- 고정 입력은 definite 40×100px border-box Grid, 40px column, 10px explicit row, zero gap·edge, row sparse auto-placement, visible overflow다. 직접 자식은 explicit 10px item 하나와 intrinsic implicit item 여덟 개이며 각 intrinsic item은 fixed-px payload `div` 하나를 가진다.
- implicit payload heights는 `[20, 10, 12, 6, 6, 6, 60, 6]px`다. 서로 다른 높이는 반복되는 `min-content`·`max-content` track의 개별 contribution, 여섯-member pattern 순환, 반복된 fit-content cap overflow를 확인한다.
- `start`의 독립 used rows는 `[10, 20, 10, 12, 6, 6, 6, 20, 6]px`로 합계 96px이며 4px free space가 남는다. `stretch`에서는 `auto` track만 4px 늘어 `[10, 20, 10, 12, 10, 6, 6, 20, 6]px`가 된다.
- `fit-content(20px)`와 `fit-content(20%)`는 definite 100px block-size basis에서 같은 20px cap이다. 일곱째 implicit row의 60px payload는 20px row를 유지하며 visible overflow로 grid border 바깥까지 이어진다. `start`에서 30px, `stretch`에서 34px 바깥으로 나온다.
- Agent Browser HeadlessChrome 157.0.0.0 / macOS arm64 / 1280×633 / DPR 1에서 네 scene을 관찰했다. 모든 used track, item 및 payload 좌표는 독립 기대값과 4/4 exact로 일치했다. [비교 근거](../spec/internal/evidence/c11-4-root-html-repeated-intrinsic-auto-rows-precomparison-2026-11-08/README.md) · [HTML fixture](../tests/fixtures/css/c11/root-html-grid-repeated-intrinsic-auto-rows-precomparison.html).

## 제한 지원 및 거부 경계

- 기존 `RootHtmlFitContentAutoRowsV1`의 단일 visible parsed Root HTML Grid owner를 사용한다. width 40px·height 100px·border-box, 40px column·10px explicit row, row sparse flow, zero spacing/gaps, static positioning과 visible overflow 조건을 보존한다.
- `grid-auto-rows`는 정확히 여섯 member여야 한다. 첫 member는 finite non-negative direct `px` 또는 definite block-size basis에 대한 direct percentage `fit-content()`이고, 나머지는 정확히 `[min-content, max-content, auto, min-content, max-content]` 순서여야 한다. arbitrary ordering, 다른 중복/누락, extra track function, unresolved `calc()` 및 10,000 track 경계 초과는 계속 거부한다.
- visible direct child는 정확히 아홉 개다. 첫 번째는 빈 fixed 10px block이고, 뒤 여덟 개는 auto-height Grid item이며 각 item에는 허용된 fixed-px payload `div` 하나만 둔다. item 순서·placement는 자동 row flow에 따른다.
- intrinsic payload profile은 `[20, 10, 12, 6, 6, 6, 60, 6]px`의 고정 높이만 허용한다. row 7의 `60px` payload만 `fit-content()` cap overflow를 검사하며 payload는 row/grid clipping 없이 그려진다. 일반 content, text, image와 nested Grid는 거부한다.
- `align-content:start` 및 `stretch`만 지원한다. stretch는 positive free space가 있을 때 이 profile의 `auto` track 하나에만 배분한다. center/end/baseline/unsafe 등 다른 값은 거부한다.
- 두 번째 Grid owner, 추가 visible item, 누락 payload, 다른 explicit track shape, 비정상 revision, unsupported sibling/profile은 property/node가 있는 오류로 fail-closed한다. 실패 뒤 partial layout/draw publication이 없어야 하고 동일 RuntimeSession의 다음 유효 revision은 회복해야 한다.
- 공개 CSS 지원, WPT, full CSS grammar, arbitrary intrinsic contribution, C11.4/C11 완료를 주장하지 않는다. 공개 API·FFI surface는 추가하지 않고 내부 contract `0.1.0`을 유지한다.

## 구현 및 검증 관문

1. 이 계획의 서로 다른 20개 실패 관점을 구현 전에 검토한다. Chrome 관측과 independent expected values를 분리한 4-scene precomparison을 변경하지 않는다.
2. Stylo→typed style projection은 exact six-member order와 각 typed member를 보존한다. 변경되지 않은 3-member·4-member 목록의 기존 허용/거부와 singleton `RootHtmlIntrinsicTrackSizingV1` admission을 회귀 검사한다.
3. Layout profile은 정확한 여섯-member pattern과 아홉-child topology만 허용한다. eight implicit rows에 대한 pattern cycling, duplicate intrinsic track별 item contribution, fit-content row/payload overflow와 total geometry를 확인한다.
4. `align-content:start|stretch`에서 독립된 used rows를 검사한다. 4px stretch는 `auto` track 하나에만 들어가며 fit-content/min/max tracks는 바뀌지 않아야 한다.
5. RuntimeSession은 computed track identity, 모든 item/payload frame, border 밖 overflow, 오류 귀속, 실패 atomicity와 same-session recovery를 확인한다. 관련 focused Layout·Style projection·RuntimeSession 검사, 변경 crate Clippy와 `cargo fmt --all -- --check`를 실행한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 HTML/source hash를 기본 Root HTML route로 실행한다. layout-ready, 9 direct items, 8 payloads, draw/present와 화면을 확인한다. item 및 payload는 grid border 바깥까지 비교한다. OS screenshot만 있으면 surface pixel readback으로 표현하지 않는다.
7. 구현/runtime 자체를 별도의 실패 관점 20개로 검토한다. PR diff에는 구현·문서·evidence의 독립 실패 경로 20개를 별도로 검토하고 발견 항목을 고친 뒤 해당 관점을 다시 실행한다.
8. PR에는 Android/iOS 화면 원본 링크, 지원·거부 계약, merge 상태와 실제 checks를 기록한다. Rebase and merge 뒤 `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, 이 C11.4 계획과 roadmap을 동기화한다. Tailnet roadmap은 Agent Browser desktop/mobile로 확인한 뒤 browser process를 종료한다.

## 범위 밖

다른 six-member ordering, arbitrary duplicate sequence, arbitrary HTML/CSS admission, content-varying min/max contribution parity, spanning, auto-repeat interaction, named line/area interaction, implicit columns, full WPT, hardware GPU, post-compositor pixel parity 및 전체 C11.4/C11 완료는 이 slice에 포함하지 않는다. 고정 payload와 exact pattern의 결과를 일반 min/max-content algorithm correctness로 확대 해석하지 않는다.

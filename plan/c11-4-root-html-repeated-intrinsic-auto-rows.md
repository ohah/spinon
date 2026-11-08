# C11.4 parsed Root HTML repeated intrinsic implicit-row list

**상태:** 반복 intrinsic row 제한 구현 PR #1284, 30 tail 순서 PR #1286, 고정 text payload PR #1290이 모두 merged됐다. 최신 merge SHA는 `b9440dcde6d3c3724baa1fa81d105b6c08cb10c6`다. 후속으로 [제한 텍스트 content grammar](./c11-4-root-html-repeated-intrinsic-text-content.md)를 선택해 Chrome 157 사전 비교 810 scenes와 계획 실패 관점 20개를 마쳤다. 계획 PR 병합 전이며 제품 구현은 시작하지 않았다. 이 bounded slice들도 C11.4/C11 전체 지원이 아니며 공식 `78/214`와 두 체크는 유지한다.

**이전 slice 검증:** Chrome 157 사전 비교와 독립 계산 4/4, 계획 실패 관점 검토 20/20을 마쳤다. Layout 2개, Style projection 2개, RuntimeSession 3개 focused test가 통과했고, fit-content 회귀 79개, 전체 workspace Clippy 및 rustfmt도 통과했다. Android API 37 ARM64 AVD는 20 boxes와 draw-presented, iPhone 17 Pro/iOS 26.2 Simulator는 같은 fixture로 layout-ready 20 boxes를 확인했다. 두 OS 캡처는 visual smoke이며 surface pixel readback은 아니다. 기존 iOS app data는 복원 후 11개 파일 hash가 설치 전과 같다. 구현/runtime와 별도 PR diff 실패 관점 검토가 각각 20/20 통과했다. 구현 PR [#1284](https://github.com/ohah/spinon/pull/1284)은 2026-11-08T06:43:38Z Rebase and merge됐고 merge SHA는 `f2f9312e39ddf6d7ca8e703c28d33bf26e5325b8`이다. PR workflow와 status check는 보고되지 않았다. Tailnet 게시본은 [동기화 근거](../spec/internal/evidence/roadmap-tailnet-sync-pr1284-c11-4-2026-11-08/README.md)에 기록한다. C11.4/C11 상태 체크와 공식 `78/214` 집계는 유지한다.

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
4. `align-content:start|stretch`에서 독립된 used rows를 검사한다. 4px stretch는 `auto` track 하나에만 들어가며 fit-content/min/max tracks는 바뀌지 않아야 한다. 구현은 `start` rows `[10,20,10,12,6,6,6,20,6]px`와 `stretch` rows `[10,20,10,12,10,6,6,20,6]px`를 검사한다.
5. RuntimeSession은 computed track identity, 모든 item/payload frame, border 밖 overflow, 오류 귀속, 실패 atomicity와 same-session recovery를 확인한다. Layout 2/2, Style projection 2/2, RuntimeSession 3/3의 repeated-intrinsic 테스트가 통과했고, 세 crate의 fit-content 회귀 79개도 통과했다. changed-crate Clippy와 `cargo fmt --all -- --check`도 통과했다. 전체 workspace Clippy는 아직 실행 전이다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 HTML/source hash `77874483d57af866e9215118d559e3140c4fbf61cba568493288bf94a62d231f`를 기본 Root HTML route로 실행했다. 둘 다 `layout=ready boxes=20`, Android는 draw-presented까지 확인했다. OS screenshot은 visual smoke이며 PixelCopy/Metal surface pixel readback은 아니다. 기존 iOS app container는 11개 파일 hash가 복원 전후 일치했다.
7. 구현/runtime 자체를 계획 검토와 다른 실패 관점 20개로 검토했고 PR diff도 별도 20개 실패 관점 검토를 마쳤다. 실제 staged diff와 PR 본문 링크를 생성 직전 대조한다.
8. PR에는 Android/iOS 화면 원본 링크, 지원·거부 계약, merge 상태와 실제 checks를 기록한다. Rebase and merge 뒤 `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, 이 C11.4 계획과 roadmap을 동기화한다. Tailnet roadmap은 Agent Browser desktop/mobile로 확인한 뒤 browser process를 종료한다.

## 범위 밖

다른 six-member ordering, arbitrary duplicate sequence, arbitrary HTML/CSS admission, content-varying min/max contribution parity, spanning, auto-repeat interaction, named line/area interaction, implicit columns, full WPT, hardware GPU, post-compositor pixel parity 및 전체 C11.4/C11 완료는 이 slice에 포함하지 않는다. 고정 payload와 exact pattern의 결과를 일반 min/max-content algorithm correctness로 확대 해석하지 않는다.

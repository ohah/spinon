# C11.4 parsed Root HTML four-member intrinsic implicit-row list

**상태:** Chrome 157 사전 비교 96/96과 계획 실패 관점 20개 검토를 마쳤다. 구현은 아직 시작하지 않았다. 직전 [`fit-content()`/`min-content`/`auto` 나머지 순서](./c11-4-root-html-fit-content-min-content-auto-remaining-order.md)는 PR [#1277](https://github.com/ohah/spinon/pull/1277)로 병합됐다.

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**사전 비교:** [Chrome 157 관측·독립 기대값](../spec/internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/README.md) · [고정 fixture](../tests/fixtures/css/c11/root-html-grid-fit-min-max-content-auto-implicit-row-list-precomparison.html) · [계획 실패 관점 20개](./c11-4-root-html-fit-min-max-content-auto-rows-plan-review-20.md)

## 결손과 목표

기본 parsed Root HTML의 `RootHtmlFitContentAutoRowsV1`은 `fit-content()`가 있는 track list를 허용한다. 목록에 `min-content`가 있으면 `fit-content()`, `min-content`, `auto`를 각각 한 번 포함하는 여섯 exact 3-member 순서만 지원한다. 기존 singleton `max-content`는 별도 `RootHtmlIntrinsicTrackSizingV1` profile에서 처리하지만, 현재 3-member list 및 이 두 profile의 혼합은 거부한다.

이 slice는 같은 `RootHtmlFitContentAutoRowsV1` owner/topology를 유지하면서 다음 네 값을 각각 한 번 포함하는 24개 순서를 허용한다.

```css
fit-content(<finite non-negative px | %>) min-content max-content auto
```

순서는 24가지 permutation이며 CSS computed style과 typed `GridAutoRows::TrackList`에서 그대로 보존한다. `fit-content()` cap은 기존처럼 direct finite non-negative `px` 또는 definite block-size basis를 갖는 direct percentage만 허용한다. `calc()`, 음수·비유한 값, indefinite percentage basis, 중복·누락 member와 다른 track function은 계속 거부한다.

이 변경은 grid track-list admission의 제한을 넓힌다. 이 고정 높이 payload fixture에서 min-content와 max-content contribution은 같으므로 used geometry만으로 두 track type을 구별하지 않는다. 검증은 authored/computed list와 typed projection에서 두 identity가 보존되는지 별도로 확인한다. 이 결과를 임의 content의 min/max 알고리즘 동등성이나 일반 CSS 지원으로 확대 해석하지 않는다.

## 규범과 독립 기준

- [CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks)는 `grid-auto-rows`를 하나 이상의 `<track-size>`를 갖는 track list로 정의한다.
- [§12.5](https://www.w3.org/TR/css-grid-2/#algo-content)에서 min-content와 max-content track function은 서로 다른 contribution을 요구한다. 이 fixture의 고정 높이 payload는 두 contribution이 같은 입력만 사용하므로 type identity는 CSS computed value와 typed projection으로 구분한다.
- [§12.8](https://www.w3.org/TR/css-grid-2/#algo-stretch)는 `normal` 또는 `stretch` content distribution에서 남은 positive definite free space를 `auto` max track function을 가진 track에 균등 배분한다. `fit-content()`와 `min-content`/`max-content` track에는 이 확장을 적용하지 않는다.
- Chrome oracle은 Agent Browser HeadlessChrome 157.0.0.0, macOS arm64, viewport 1280×633, DPR 1이다. 고정 Grid는 40×100 CSS px, 40px column, 10px explicit row, zero gap/edges, visible overflow다. 다섯 implicit item의 payload heights는 `[20,10,12,6,20]px`다.
- Matrix는 24 순서 × `20px`/`20%` cap × `start`/`stretch`로 96 scene이다. Percentage basis는 definite 100px block size이므로 두 cap form은 같은 used geometry를 가져야 한다. 첫 implicit row에서 list 순환을 시작하고 다섯 번째 implicit row에서 첫 member가 반복되는 것을 검사한다.
- 독립 기대값은 [`verify-chrome-observation.mjs`](../spec/internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/verify-chrome-observation.mjs)가 CSS 규칙과 고정 payloads에서 따로 계산한다. 모든 `start` scene의 used rows는 `[10,20,10,12,6,20]px`, 합계 78px이며 definite free space는 22px다. `stretch`에서는 `auto` track만 그 22px을 받는다. 첫 member가 `auto`면 다섯 번째 implicit row도 `auto`라 각 track이 11px씩 받는다.

## 지원·거부 경계

- 기존 단일 Root HTML owner, 40×100px border-box Grid, 40px column·10px explicit row, row sparse auto-placement, zero edge/gap, visible overflow, single root/document/revision 및 direct item/payload topology를 유지한다.
- Track list는 길이 4이며 `fit-content()`, `min-content`, `max-content`, `auto`가 각각 정확히 한 번 있어야 한다. 모든 24개 순서를 허용한다. 다섯 implicit rows를 만들고 다섯 번째 row에서 네-member list가 다시 순환한다.
- `align-content:start`는 기존 계산을 보존한다. `align-content:stretch`는 이 exact 24-list profile에서만 허용하며 `auto` max-sizing track에만 남은 positive free space를 균등 분배한다. 첫 member가 `auto`인 경우 두 auto rows에 같은 추가 길이를 준다.
- 다른 Root HTML owner, arbitrary markup, intrinsic explicit column, named line/area, auto-repeat, dense/column flow, spanning/authored placement, nested Grid, extra children, invalid cap, invalid track list와 malformed revision은 계속 fail-closed한다. 오류는 전체 layout/render publication을 막고 property/node에 연결하며 같은 RuntimeSession의 다음 유효 revision으로 복구한다.
- 공개 API/FFI selector를 추가하지 않는다. 내부 계약 숫자 버전 `0.1.0`, C11.4/C11 미완료 표시 및 공식 `78/214` 집계를 유지한다.

## 구현·검증 관문

1. 구현 전에 이 계획과 96-scene Chrome 관측·독립 기대값을 고정한다. 계획 리뷰 20개는 구현/runtime 리뷰와 분리한다.
2. Style projection에서 24개 exact 순서를 `[FitContent, MinContent, MaxContent, Auto]`의 typed identity로 보존한다. 빈/중복/누락/다른 멤버·길이·cap은 property diagnostic으로 거부한다. 기존 singleton min/max 및 기존 여섯 3-member 순서를 회귀 검사한다.
3. Layout admission은 `RootHtmlFitContentAutoRowsV1` 단일 owner의 24개 순서만 받아들이고, implicit row 순환·computed track list 순서·item/payload frame을 검사한다. 다른 profile의 admission이 우연히 넓어지지 않는 negative controls를 둔다.
4. `align-content:stretch`는 positive free space 22px, 단일 auto track의 전량 분배, 첫 member 반복으로 두 auto track이 되는 경우의 균등 분배를 확인한다. non-auto rows가 stretch로 변하지 않고 `start`가 free space를 남기는지 검사한다.
5. RuntimeSession에서 root/document/style/environment revision 결합, 오류 attribution, partial layout/draw 부재 및 같은 세션 recovery를 확인한다. focused Layout·Style projection·RuntimeSession 검증, changed-crate Clippy, `cargo fmt --all -- --check`를 실행한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 parsed Root HTML route에서 동일 HTML hash를 실행해 layout-ready, track/item frame, draw/present와 화면을 기록한다. 시뮬레이터 결과를 실기기·hardware GPU·post-compositor pixel parity로 확대하지 않는다.
7. 계획과 겹치지 않는 실패 관점 20개로 구현/runtime을 검토한다. PR diff는 별도 관점 30개로 검토한다. 발견된 결함은 수정하고 관련 관점을 다시 실행한다. PR은 기존 label과 `Rebase and merge`를 사용한다.
8. 병합 뒤 `spec/STATUS.md`, 내부 계약 0059의 제한 범위, C11 계획과 공개 roadmap을 실제 결과에 맞춘다. Tailnet 제공본은 HTTP/hash로 확인하고 Agent Browser desktop/mobile 화면을 검증한 뒤 브라우저 프로세스를 종료한다.

## 범위 밖

이 slice는 일반 CSS author admission, arbitrary track grammar, content-varying min/max contribution correctness, spanning contribution algorithm, implicit columns, named-line/area, WPT 적합성, 실기기/hardware GPU, iOS compositor-after pixel equality 또는 C11.4/C11 완료를 뜻하지 않는다. Chrome의 min/max computed identity와 fixed-height fixture geometry가 일치해도 track contribution의 전체 알고리즘 검증으로 주장하지 않는다.

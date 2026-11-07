# C11.4 parsed Root HTML `grid-auto-rows` fit-content list의 `auto` track

**상태:** 제한 구현, focused Rust 검증, Android·iOS Simulator visual smoke와 계획/구현-runtime 실패 관점 20개 검토를 마쳤다. Layout 2/2, style projection 1/1, 신규 RuntimeSession 2/2, 이전 singleton 회귀 2/2, changed-crate Clippy/rustfmt가 통과했다. 두 simulator는 기본 Root HTML route에서 `layout=ready boxes=12`를 보고했고 package의 HTML hash가 고정 fixture와 일치한다. PR diff 실패 관점 30개 검토를 완료했다. PR 생성·CI/check 결과 확인·merge는 남아 있다. 이 미병합 조각은 `auto` 한 종류만 pattern 안에서 순환 허용하며, C11.4/C11 완료와 공식 `78/214`는 그대로 유지한다.

**상위:** [공식 C11 상태](../spec/STATUS.md#c11-css-grid) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [fit-content 목록 구현](./c11-4-root-html-fit-content-auto-rows-list.md) · [내부 명세 0059](../spec/internal/0059-c11-grid-layout.md)

## 결손과 목표

CSS Grid의 `grid-auto-rows` grammar는 하나 이상의 `<track-size>`를 받으며, `auto`는 유효한 track-size다. 기존 `RootHtmlFitContentAutoRowsV1` list validator는 `LengthPx`, `Percentage`, `FitContent(px|%)`만 허용한다. 따라서 `fit-content(20%) auto 8px`는 computed style에 유효한 mixed pattern인데도 Root HTML runtime에서 fail-closed한다.

기존 제한 profile을 넓혀 mixed list member로 finite non-negative px, direct percentage, finite non-negative `fit-content(px|%)`와 `auto`를 허용한다. 목록은 반드시 `fit-content()`를 하나 이상 포함하고 비어 있지 않으며 기존 track-count cap 이하여야 한다. `auto` 단독 목록과 fit-content가 없는 list, `min-content`, `max-content`, `minmax()`, `fr`, `calc()` 및 arbitrary values는 이 slice에서 계속 거부한다.

`auto` track은 `align-content:start`를 유지해 free-space stretch와 분리한다. 각 implicit auto row item은 `min-height:0`이고 하나의 fixed-height nested block을 가진다. 이로써 auto track의 max-content contribution과 fit-content cap을 별도 row에서 비교하고 list 순환 순서를 네 implicit row로 관찰한다.

## 규범·비교 모델

- 규범은 [CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid/#implicit-track-sizing), [§12.2](https://www.w3.org/TR/css-grid/#track-sizing-terminology), [§12.3](https://www.w3.org/TR/css-grid/#algo-track-sizing), [§12.8](https://www.w3.org/TR/css-grid/#algo-stretch)다. `<track-size>+`의 list 순서는 첫 implicit row부터 순환한다. `auto` max track sizing function은 item의 max-content contribution을 growth limit로 쓰고, `normal|stretch` content distribution일 때만 §12.8에서 남은 positive definite free space를 추가로 받는다.
- 고정 입력은 definite 40×100px Grid content box, explicit 40px column과 10px row, row flow, sparse auto-placement, zero gap/edge, `align-content:start`, visible overflow다. Percentage basis는 100px content-box block-size다.
- independent expected values는 track function과 fixed nested-block contributions에서 직접 계산한다. `fit-content(L)`은 `max(auto minimum, min(L, max-content))`로 계산하고, `min-height:0` item에서 auto minimum을 0으로 고정한다. 세 scene 각각에서 growth-limit 합계가 available space보다 작거나 같으므로 `auto`는 해당 item의 max-content contribution까지 커진다. `align-content:start`는 growth limits에 도달한 뒤 남는 free space를 `auto` tracks에 추가 배분하지 않는다.
- 기준 fixture는 [고정 HTML](../tests/fixtures/css/c11/root-html-grid-fit-content-auto-track-list-precomparison.html)이다. Chrome 관측은 Agent Browser HeadlessChrome 157에서 별도로 저장하며 규범 기대값 대신 사용하지 않는다.

| 장면 | `grid-auto-rows` | payload heights | 독립 used rows (explicit 포함) |
| --- | --- | --- | --- |
| fit, auto, fixed | `fit-content(20%) auto 8px` | 34, 30, 6, 25px | `[10, 20, 30, 8, 20]px` |
| auto, fit, fixed | `auto fit-content(20%) 8px` | 30, 34, 6, 25px | `[10, 30, 20, 8, 25]px` |
| zero fit, auto, fixed | `fit-content(0%) auto 8px` | 34, 30, 6, 25px | `[10, 0, 30, 8, 0]px` |

네 번째 implicit row는 list 첫 member로 돌아오는지 확인한다. 모든 row는 `align-content:start`에서 계산하며 Grid의 남은 definite height를 auto row에 배분하지 않는다. zero cap case에서는 item height가 0이어도 nested fixed-height payload가 visible overflow로 남는다.

## 지원·거부 경계

- 기본 parsed Root HTML GPU runtime route의 기존 `RootHtmlFitContentAutoRowsV1` profile만 적용한다. owner, explicit tracks, child topology, revisions, default style, row-flow placement, direct HTML `div`와 nested fixed-px payload 조건을 유지한다.
- `grid-auto-rows`는 non-empty typed list이고 `fit-content(px|%)` member를 하나 이상 포함해야 한다. 다른 member는 finite non-negative fixed-px, finite non-negative direct percentage 또는 `auto`만 허용한다. percentages는 definite 100px content-box block-size에 대해 해석한다.
- `min-content`, `max-content`, `minmax()`, `fr`, `calc()`, unsupported percentage basis, negative/non-finite values, no-fit list, empty list, auto columns, column/dense flow, named areas/lines, auto-repeat, arbitrary DOM/text/image, spanning와 nested Grid는 계속 fail-closed한다.
- 오류는 partial layout/render output을 게시하지 않으며 같은 RuntimeSession에서 다음 유효 revision이 회복되는지 검사한다. 일반 CSS admission, WPT runner 통과, 실기기/hardware GPU, surface pixel parity 또는 C11.4/C11 완료를 주장하지 않는다. 내부 contract version `0.1.0`과 official `78/214`를 유지한다.

## 구현 및 검증 관문

1. 이 계획과 fixture의 서로 다른 실패 경로 20개 검토를 구현 전에 끝내고 Chrome 관측과 independent expected values를 분리해 고정한다.
2. Style projection은 exact default Root HTML profile에서만 typed `Auto` list member를 보존하고, fixed-px/percentage/fit-content ordering도 그대로 유지한다. 다른 profile의 admission을 넓히지 않는다.
3. Layout validator는 `Auto`를 기존 supported member에 추가하되 fit-content 포함, length cap, owner/track/child contract를 유지한다. Taffy에 list를 순서대로 전달하고 first implicit row부터 cycling한다.
4. Layout, style projection, RuntimeSession focused tests는 세 oracle scenes의 rows, all item/payload frames, track identity/order, overflow, same revision, successful rendering을 검사한다. Negative controls는 no-fit/empty/unsupported/non-finite/negative/indefinite-basis/changed-owner/changed-child-shape를 거부하고 failure atomicity 및 same-session recovery를 확인한다.
5. 관련 focused Rust tests, changed-crate Clippy와 `cargo fmt --all -- --check`를 실행한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 input/source hash로 기본 Root HTML route를 실행한다. screenshot은 visual smoke; readback이 수행되지 않으면 surface pixel 증거로 부르지 않는다.
6. implementation/runtime 변경을 계획 검토와 별도의 실패 관점 20개로 검토하고, PR diff를 서로 다른 관점 30개로 검토한다. 검토에서 발견된 오류는 수정한 뒤 관련 관점을 다시 실행한다. PR에는 가능한 Android/iOS 화면을 pinned raw-image link로 붙인다.
7. PR 병합 뒤 merge SHA, PR state와 checks를 확인하고 `spec/STATUS.md`, 내부 명세, C11.4 계획과 public/Tailnet roadmap을 동기화한다. Agent Browser로 desktop/mobile 표시를 확인하고 browser process를 종료한다.

## 계획 검토

구현 전에 [계획 실패 관점 20개](./c11-4-root-html-fit-content-auto-track-list-plan-review-20.md)를 작성하고 각각 별도 입력·계산·admission·publication 조건으로 판정한다. 이 표는 이후 implementation/runtime review나 PR diff review를 대신하지 않는다.


## 구현/runtime 검증 결과

Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 최신 source 실행, focused test logs, screenshots 및 복원 hash는 [실행 근거](../spec/internal/evidence/c11-4-root-html-fit-content-auto-track-list-runtime-2026-11-08/README.md)에 기록했다. 별도 [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-4-root-html-fit-content-auto-track-list-runtime-2026-11-08/implementation-runtime-review-20.md)를 마쳤다. Screenshot은 visual smoke이며 surface readback이 아니다. PR 변경 실패 관점 30개 검토를 완료했다.

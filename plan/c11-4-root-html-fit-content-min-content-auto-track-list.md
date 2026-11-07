# C11.4 parsed Root HTML fit-content/min-content/auto implicit-row list

**Status:** 제한 구현 PR [#1274](https://github.com/ohah/spinon/pull/1274)가 `2026-11-07T22:12:47Z` Rebase and merge됐다(merge SHA `b361097257f0fb6a9739de139724bef24abadb04`). Chrome 157 사전 비교, focused Rust 검증, Android/iOS Simulator 기본 Root HTML 실행, 구현/runtime 20개와 PR diff 30개 실패 관점 검토를 기록했다. 병합 후 계약·상태·공개/Tailnet 로드맵 동기화는 후속 문서 변경에서 수행한다. 이 slice는 C11.4 또는 C11 완료를 뜻하지 않으며 공식 `78/214`를 바꾸지 않는다.

**상위:** [공식 C11 상태](../spec/STATUS.md#c11-css-grid) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [선행 fit-content auto 목록](./c11-4-root-html-fit-content-auto-track-list.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 결손과 목표

기본 parsed Root HTML `RootHtmlFitContentAutoRowsV1`은 mixed `grid-auto-rows` list에서 `fit-content(px|%)`, fixed-px, direct percentage, `auto`를 순환할 수 있다. 현재 profile은 `min-content` member와 해당 profile의 `align-content:stretch`를 거부한다. 별도 C11.4 profile에는 singleton `grid-auto-rows:min-content`가 있으나 두 profile을 결합한 mixed track list는 아직 지원하지 않는다.

이 제한 slice는 기존 한 Grid/한 문서/고정 DOM profile에서 아래 두 list만 추가한다.

- `fit-content(<finite non-negative px|%>) min-content auto`
- `fit-content(<finite non-negative px|%>) auto min-content`

`fit-content()`는 반드시 하나, `min-content`와 `auto`는 각각 정확히 하나 있어야 한다. 두 순서 모두 첫 implicit row부터 적용하고 다음 implicit row들에서 순환한다. 같은 source fixture에서 auto track만 positive free space를 받아 `min-content` track과 구분되는 값을 보인다.

## 규범·고정 oracle

- 규범은 [CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks), [§12.3](https://www.w3.org/TR/css-grid-2/#algo-track-sizing), [§12.8](https://www.w3.org/TR/css-grid-2/#algo-stretch)다. `grid-auto-rows` list는 첫 implicit row에서 시작해 반복한다. `min-content` track은 item의 min-content contribution을 사용하며 `auto` max track sizing function은 positive definite free space를 stretch 받을 수 있다.
- 고정 사전 비교 fixture는 [`root-html-grid-fit-content-min-content-auto-track-list-precomparison.html`](../tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-track-list-precomparison.html)이다. Chrome 157 Agent Browser capture, computed tracks와 모든 frame은 [사전 비교 근거](../spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-track-list-precomparison-2026-11-08/README.md)에 고정했다. fixture SHA-256은 `2daedf8cb1a755258e51b634c8907168ac27cef3106a1571b9c71cdbdf4292f0`이다.
- 현재 기본 Root HTML RuntimeSession에 연결한 scene fixture는 [`root-html-grid-fit-content-min-content-auto-track-list-runtime.html`](../tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-track-list-runtime.html)이며 SHA-256은 `18f684f61be1484823919fa2895ae7e0e568592808a4d47d243216517935c08c`이다.
- 독립 계산은 Chrome 관측에서 복사하지 않는다. 40×100px definite content box, explicit 10px row, zero edges/gaps, `align-content:stretch`, item `min-height:0`을 사용한다. `fit-content(20%)` cap은 20px이고 fixed nested block이 각각 min/max-content contribution을 제공한다.

| 장면 | `grid-auto-rows` | payload 높이 | independent used rows (explicit 포함) |
| --- | --- | --- | --- |
| 첫 순서 | `fit-content(20%) min-content auto` | 34, 34, 10, 6px | `[10, 20, 34, 30, 6]px` |
| 순서 대조 | `fit-content(20%) auto min-content` | 34, 34, 10, 6px | `[10, 20, 54, 10, 6]px` |
| visible overflow | `fit-content(120%) min-content auto` | 6, 34, 10, 50px | `[10, 6, 34, 10, 40]px` |

Finite pixel caps are covered by a separate pinned Chrome 157 scene: [`fit-content(20px) min-content auto`](../spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-track-list-precomparison-2026-11-08/px-cap/README.md) has the independently calculated `[10, 20, 34, 30, 6]px` rows, matching the equivalent 20% cap at a 100px definite block size.

첫 두 scene은 전체 초기 track contribution 80px에서 남는 20px를 단 하나의 `auto` max track에 배분한다. `min-content` track 높이는 각 payload contribution인 34px 또는 10px로 유지된다. 세 번째 scene은 네 번째 implicit row가 fit-content 항목으로 순환하며 남은 공간 40px만 차지한다. 50px nested payload는 visible overflow로 Grid 아래 10px까지 나온다. Chrome 및 독립 기대값은 grid 3개·item frame 15개·payload frame 12개에서 모두 0 CSS px 차이다.

## 정확한 지원 경계

- 기본 parsed Root HTML runtime의 기존 `RootHtmlFitContentAutoRowsV1`만 확장한다. 하나의 visible Grid owner와 같은 document/root/revision identity를 요구한다.
- Grid는 border-box 40×100px, `grid-template-columns:40px`, `grid-template-rows:10px`, row sparse auto-flow, `grid-auto-columns:auto`, no named lines/areas/repeat, zero gap/edges, static positioning, visible overflow, `align-items:normal`, `justify-content:start`, `justify-items:normal`이어야 한다.
- `align-content:start` 기존 동작을 보존하고, `stretch`는 두 exact min-content/auto list에만 허용한다. 다른 list와 content-distribution 값은 계속 거부한다.
- Grid는 direct `div` 5개를 가진다. 첫 child는 fixed 10px explicit-row item이며 다음 네 auto item은 각각 한 개의 nested fixed-px `div` payload와 `min-height:0`을 가진다. text, image, replaced element, spanning, nested Grid, hidden/positioned child와 다른 topology는 거부한다.
- `grid-auto-rows`는 세 entry이며 위 표의 두 order만 허용한다. Fit-content cap은 유한한 비음수 px/percentage다. Percentage basis는 definite 100px content-box height다. Pattern은 첫 implicit row에 적용하고 4번째 implicit row에서 fit-content entry로 돌아오는 것을 확인한다.
- missing/duplicate `fit-content`, `min-content`, `auto`, 빈 목록, singleton `min-content`, `max-content`, `minmax()`, `fr`, `calc()`, 음수·NaN·무한 cap, 초과 track 수, indefinite basis와 다른 owner/style/tree shape는 계속 fail-closed한다.
- 실패 revision은 layout/render 결과를 부분 게시하지 않는다. 같은 `RuntimeSession`에서 다음 유효 revision의 복구를 검사한다. 일반 CSS admission, arbitrary Grid, WPT, 실기기/hardware GPU, surface pixel parity 또는 full C11.4/C11 지원을 주장하지 않는다. Public API/ABI와 contract version `0.1.0`은 바꾸지 않는다.

## 구현 및 검증 관문

1. 계획 자체의 20개 실패 관점 검토를 먼저 마친다. Chrome observation과 independent expected values를 별도로 고정한다.
2. Style projection은 위 두 exact order의 typed `min-content`를 보존하고 그 외 profile·list에는 성공 의미를 추가하지 않는다. generic `allow_intrinsic` switch로 `calc()`/`minmax()` 등 다른 track을 함께 열지 않는다.
3. Layout allowlist는 FitContent·MinContent·Auto가 각각 하나인 두 pattern만 허용한다. layout validator의 generic intrinsic 허용을 profile 전체에 켜지 않는다. 이 profile에서만 `align-content:stretch`를 허용한다.
4. Layout tests는 세 oracle scene의 computed typed order, repeated pattern position, used row sizes, every item/payload frame, positive free-space 분배, visible overflow를 검사한다.
5. Style projection tests는 양쪽 order를 허용하고 singleton/duplicate/missing/unmodeled members, unsupported align-content와 다른 Grid profile을 거부한다. RuntimeSession tests는 기본 parsed Root HTML 경로, root/document/style/layout revision identity, full frame publication, unsupported-shape failure atomicity와 same-session recovery를 확인한다.
6. Focused tests, 변경 crate Clippy, `cargo fmt --all -- --check`를 실행한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 최종 source/HTML hash로 기본 Root HTML runtime을 실행하고 screenshots를 저장한다. Android current-process log는 `layout=ready boxes=12`와 draw-presented를 기록한다. iOS current-process log는 fixture source와 `layout=ready boxes=12`를 기록하고 screenshot에서 표시를 확인한다. 양쪽 screenshots는 visual smoke와 bounds 확인이며 PixelCopy/Metal surface readback으로 부르지 않는다. iOS 설치 전후 data-container 경로가 달라져 기존 app-data continuity를 확인할 수 없었다. 설치·erase·clear 명령은 실행하지 않았고 원인은 특정하지 못했다. [실행 결과](../spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-track-list-runtime-2026-11-08/README.md) · [구현/runtime 실패 경로 20개 검토](../spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-track-list-runtime-2026-11-08/implementation-runtime-review-20.md)
7. 구현/runtime 변경을 별도의 20개 failure-path review로 검토한다. PR diff를 30개 서로 다른 관점으로 검토하고 가능한 Simulator 화면을 PR 본문에 commit-pinned raw-image link로 넣는다. [구현/runtime 20개 검토](../spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-track-list-runtime-2026-11-08/implementation-runtime-review-20.md) · [PR diff 30개 검토](../spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-track-list-runtime-2026-11-08/pr-diff-review-30.md). PR checks/merge result를 사실대로 기록하고 Rebase and merge만 사용한다.
8. Merge 뒤 official status checkbox와 `78/214`를 유지하고 내부 문서·C11.4 plan·public/Tailnet roadmap을 동기화한다. Tailnet source/served SHA 및 Agent Browser desktop/mobile 표시를 확인한 뒤 Agent Browser process를 닫는다. PR #1274는 merge됐으며 후속 sync 근거는 [roadmap 동기화 evidence](../spec/internal/evidence/roadmap-tailnet-sync-pr1274-c11-4-2026-11-08/README.md)에 둔다.

## 계획 실패 관점 검토

[20개 독립 실패 경로](./c11-4-root-html-fit-content-min-content-auto-track-list-plan-review-20.md)를 구현 전에 검토했다. 이 결과는 구현/runtime review 20개 또는 PR diff review 30개를 대신하지 않는다.

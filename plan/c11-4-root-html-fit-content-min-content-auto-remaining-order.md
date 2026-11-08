# C11.4 parsed Root HTML fit-content/min-content/auto remaining implicit-row orders

**상태:** 제한 구현 PR [#1277](https://github.com/ohah/spinon/pull/1277) 병합, post-merge 문서·Tailnet 동기화 중. PR은 `2026-11-07T23:31:42Z` Rebase and merge됐고 merge SHA는 `8aa7570d34ac7c0ac9f93f3aac4fe555428741fa`다. Chrome 157 사전 비교 8/8 장면과 독립 계산, 계획 실패 관점 검토 20개, 별도 구현/runtime 검토 20개와 PR diff 검토 30개를 마쳤다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 Root HTML fixture hash의 12-box scene이 ready/present에 도달했다. [실행·화면·hash·한계](../spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-runtime-2026-11-08/README.md). PR status/workflow rollup은 비어 있어 CI green으로 기록하지 않는다. 공식 C11.4/C11 체크와 `78/214` 집계는 미완료로 유지한다.

**상위:** [공식 C11 상태](../spec/STATUS.md#c11-css-grid) · [C11.4 track sizing 계획](./c11-4-grid-track-sizing.md) · [선행 fit/min/auto 목록 계획](./c11-4-root-html-fit-content-min-content-auto-track-list.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 결손과 목표

기본 parsed Root HTML `RootHtmlFitContentAutoRowsV1`은 세 항목 `fit-content()`, `min-content`, `auto`가 정확히 한 번씩 나오는 목록 중 `fit-content()`가 첫 항목인 두 순서만 허용한다. 나머지 네 순서를 같은 제한 profile에 추가한다.

- `min-content fit-content(<finite non-negative px|%>) auto`
- `min-content auto fit-content(<finite non-negative px|%>)`
- `auto fit-content(<finite non-negative px|%>) min-content`
- `auto min-content fit-content(<finite non-negative px|%>)`

기존 두 순서의 동작과 `align-content:stretch` 예외를 유지한다. 목록은 첫 implicit row부터 적용하고 세 implicit row마다 반복한다. 새 임의의 track-size 조합이나 일반 CSS admission은 열지 않는다.

## 규범과 구현 전 비교

- 규범은 [CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks), [§12.3](https://www.w3.org/TR/css-grid-2/#algo-track-sizing), [§12.8](https://www.w3.org/TR/css-grid-2/#algo-stretch)다. `grid-auto-rows` 반복 목록은 implicit track에 적용되며 `auto` max sizing function은 `align-content:stretch`에서 positive free space를 받는다.
- Chrome 157 Agent Browser 기준 fixture는 [`root-html-grid-fit-content-min-content-auto-remaining-order-precomparison.html`](../tests/fixtures/css/c11/root-html-grid-fit-content-min-content-auto-remaining-order-precomparison.html)이다. SHA-256은 `e927dcbec971bba640d3cb7c90906850c6a4fe780226bf31b0f7dc854b8d0b89`다. 전체 computed used rows, Grid item·payload frame capture와 screenshot은 [사전 비교 근거](../spec/internal/evidence/c11-4-root-html-fit-content-min-content-auto-remaining-order-precomparison-2026-11-08/README.md)에 보존했다.
- 비교는 definite `40×100px` content box, explicit 첫 `10px` row, zero edge/gap, `align-content:stretch`, 네 auto-placed item과 nested fixed-height payload `[34,34,10,6]px`를 쓴다. `fit-content(20%)`의 definite percentage basis는 100px이므로 `fit-content(20px)`와 같은 20px cap이 된다.

| 순서 | used rows (explicit 포함) | Grid item height | payload height | stretch 분배 |
| --- | --- | --- | --- | --- |
| `min-content fit-content(20%) auto` | `[10,34,20,30,6]px` | `[34,20,30,6]px` | `[34,34,10,6]px` | single `auto`에 20px |
| `min-content auto fit-content(20%)` | `[10,34,40,10,6]px` | `[34,40,10,6]px` | `[34,34,10,6]px` | single `auto`에 6px |
| `auto fit-content(20%) min-content` | `[10,44,20,10,16]px` | `[44,20,10,16]px` | `[34,34,10,6]px` | 두 `auto`에 각각 10px |
| `auto min-content fit-content(20%)` | `[10,37,34,10,9]px` | `[37,34,10,9]px` | `[34,34,10,6]px` | 두 `auto`에 각각 3px |

독립 기대값은 Chrome 관측 좌표를 재사용하지 않고 각 track의 base size와 free space에서 계산한다. 첫 패턴은 `10 + 34 + 20 + 10 + 6 = 80px`이므로 유일한 `auto`에 20px을 더한다. 둘째는 `10 + 34 + 34 + 10 + 6 = 94px`이므로 유일한 `auto`에 6px을 더한다. 셋째는 base 합 80px, free space 20px을 두 `auto`에 같은 양으로 나눈다. 넷째는 base 합 94px, free space 6px을 두 `auto`에 같은 양으로 나눈다. Px와 percentage cap 각각 4장면의 Chrome 결과는 동일했고, 전체 used rows·item frame·payload frame이 독립값과 일치했다.

고정 높이 payload는 이 순서 확장의 sizing을 검사하지만, 각 payload의 min-content와 max-content 높이가 같으므로 intrinsic contribution의 두 종류를 서로 구별하는 증거로 해석하지 않는다. 그 구별은 이 제한 slice의 목표가 아니다.

## 정확한 지원 경계

- 기존 단일 parsed Root HTML Grid owner, root/document/revision identity, static DOM, row sparse auto-placement, `40×100px` border-box Grid, explicit `40px` column과 `10px` row, no named line/area/repeat, zero gap/edges, visible overflow, `align-items:normal`, `justify-content:start`, `justify-items:normal`을 유지한다.
- `grid-auto-rows`는 길이 3이며 `fit-content`, `min-content`, `auto`가 각각 하나 있어야 한다. cap은 유한한 비음수 direct px 또는 percentage다. percentage는 definite 100px content-box height를 쓴다.
- 네 새 순서 모두 첫 implicit row부터 시작하고 세 implicit row마다 반복해 네 번째 implicit row에서 첫 항목으로 돌아오는 것을 검사한다. `auto` 1개와 2개인 경우의 stretch 분배를 별도로 검증한다.
- `align-content:stretch`는 여섯 exact 순서에만 허용한다. 다른 track list, Grid profile, alignment 값에는 허용하지 않는다.
- missing/duplicate member, 다른 list length, `max-content`, `minmax()`, `fr`, `calc()`, 음수·NaN·무한 cap, 불명확한 percentage basis, nested/hidden/positioned/spanning items, 다른 owner/tree/revision은 계속 fail-closed한다.
- 실패한 revision은 부분 layout/render 결과를 게시하지 않고 동일 RuntimeSession의 다음 유효 revision으로 복구해야 한다. 일반 CSS admission, 임의 DOM/Grid, WPT 적합성, 실기기/hardware GPU, surface pixel parity 및 전체 C11.4는 범위 밖이다. 내부 계약/API 버전 `0.1.0`은 바꾸지 않는다.

## 구현 및 검증 관문

1. 계획 검토 20개와 Chrome 157 사전 비교·독립값을 먼저 고정한다. [계획 실패 관점 20개](./c11-4-root-html-fit-content-min-content-auto-remaining-order-plan-review-20.md).
2. Style projection과 layout admission은 위 여섯 exact 순서만 허용한다. 기존 `supports_min_content_auto_track_list`와 auto-placement의 track gate를 일치시키고, generic intrinsic allowlist를 넓히지 않는다.
3. Layout tests에서 여섯 순서의 typed order와 신규 네 순서의 used row, implicit-list wrap, positive free-space 분배, 네 item·payload frame을 검사한다. Px/% cap, cap이 payload보다 작은 row, 동일 결과를 포함한다.
4. Projection tests에서 신규 네 순서의 px/% 값을 보존하고 duplicate/missing/unmodeled member, invalid cap, 다른 profile을 거부한다. RuntimeSession 테스트에서 기본 parsed Root HTML route, 불일치 revision/root identity, atomic rejection 및 same-session recovery를 확인한다.
5. Focused tests, 변경 crate Clippy, `cargo fmt --all -- --check`를 실행한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일한 최종 source/HTML hash의 기본 Root HTML route를 확인한다. Screenshot과 log의 검증 한계는 결과에 구분해 기록한다.
6. 변경 코드/runtime에 대해 계획과 별개의 새 실패 관점 20개를 검토한다. PR diff는 서로 다른 30개 관점으로 확인하고, 가능한 Simulator 동작 화면을 PR 본문에 올린다. PR은 기존 라벨과 변경·근거·한계를 기록하고 Rebase and merge한다.
7. 병합 후 `spec/STATUS.md`, 내부 계약, 공개/Tailnet roadmap을 코드 상태와 대조해 갱신한다. 상위/공식 완료 체크는 실제 API와 전체 범위가 완료되지 않는 한 그대로 둔다. Agent Browser로 Tailnet을 확인하고 브라우저 세션을 종료한다.

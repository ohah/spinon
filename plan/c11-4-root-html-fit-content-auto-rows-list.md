# C11.4 parsed Root HTML `grid-auto-rows` fit-content track-list pattern

**상태:** Chrome 157 사전 관측과 독립 기대값 6 scene, 계획 실패 관점 20개 검토를 고정했다. 현재 branch에서 layout·projection·Root HTML runtime 연결을 구현했고, 6개 geometry scene, typed list order, floor·zero·120% 경계를 focused tests로 확인했다. Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route에서 모두 `layout=ready boxes=12`를 확인했다. 화면은 visual smoke이고 geometry 판정은 focused RuntimeSession/Layout 테스트에 둔다. 구현/runtime 검토 20개는 완료했으며 PR diff 실패 관점 30개 검토와 PR이 남아 있다. 상위 C11.4/C11 및 공식 완료 `78/214`는 미완료다.

**상위:** [C11 CSS Grid](../spec/STATUS.md#c11-css-grid) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**선행 제한 구현:** parsed Root HTML의 singleton `grid-auto-rows:fit-content(px)`는 [PR #1265](https://github.com/ohah/spinon/pull/1265), singleton direct percentage cap은 [PR #1268](https://github.com/ohah/spinon/pull/1268)에서 병합됐다. 현재 profile은 한 항목만 받아 유효한 implicit track-size 목록과 순환을 거부한다.

**구현 전 비교:** [Chrome 관측·독립 기대값·화면](../spec/internal/evidence/c11-4-root-html-fit-content-auto-rows-list-precomparison-2026-11-08/README.md) · [고정 입력](../tests/fixtures/css/c11/root-html-grid-fit-content-auto-rows-list-precomparison.html) · [계획 실패 관점 검토 20개](./c11-4-root-html-fit-content-auto-rows-list-plan-review-20.md)

## 결손과 목표

CSS Grid Layout Module Level 2 §7.6에서 `grid-auto-rows`는 `<track-size>+`를 받고, 값이 여러 개면 implicit track에 그 목록을 반복 적용한다. 첫 implicit row에는 explicit grid의 마지막 row 바로 다음 값이 적용된다. 현재 Spinon의 `RootHtmlFitContentAutoRowsV1`은 singleton `fit-content()`만 허용해 두 개 이상의 valid track-size pattern을 전부 거부한다.

이 slice는 기본 parsed Root HTML GPU runtime에서 fit-content를 하나 이상 포함한 `grid-auto-rows` 목록을 기존 제한 profile에 연결한다. 각 entry는 finite non-negative `fit-content(<length-percentage>)`, fixed-px track, 또는 definite block size에 대한 direct percentage track만 허용한다. 기존 auto-row list limit을 그대로 사용하며 목록을 임의로 잘라내거나 값의 순서를 바꾸지 않는다.

## 규범과 비교 모델

- [CSS Grid Layout Module Level 2 §7.6](https://www.w3.org/TR/css-grid-2/#auto-tracks)는 여러 `grid-auto-rows` 값이 implicit track을 따라 순환하며 첫 implicit track이 목록의 첫 값을 받는다고 정의한다. [§12.3](https://www.w3.org/TR/css-grid-2/#algo-track-sizing)은 각 track의 base size, growth limit, item contribution 계산을 정의한다.
- fixture는 definite 40×100px Grid content box, 40px explicit column, 10px explicit row, sparse row flow, zero gaps, `align-content:start`와 visible overflow를 쓴다. Percentage basis는 100px content-box block size다.
- `fit-content(cap)` track은 minimum contribution을 유지하고 max-content contribution이 cap을 넘을 때 growth limit에서 제한한다. Direct percentage와 fixed-px entries는 각자의 track sizing function을 유지한다. Chrome 결과를 독립 기대값의 입력으로 사용하지 않는다.
- 비교 환경은 Agent Browser HeadlessChrome 157.0.0.0, macOS arm64, viewport 1280×633, DPR 1이다. 고정 fixture SHA-256은 precomparison README와 `expected-values.json`에 기록한다.

## 고정 입력과 기대값

사전 비교는 두 값/세 값의 fit-content list, 3종 혼합 list, item minimum이 cap보다 큰 경우, `0%`와 `120%` cap 경계를 각각 대조한다. 6 scene의 computed value, used rows, item frames 및 nested payload overflow가 [expected values](../spec/internal/evidence/c11-4-root-html-fit-content-auto-rows-list-precomparison-2026-11-08/expected-values.json)에 있다. Chrome과 독립 계산은 모든 frame에서 0 CSS px 차이다.

| 장면 | Auto-row pattern | implicit used rows |
| --- | --- | --- |
| two caps | `fit-content(20%) fit-content(40%)` | `[20, 34, 20, 6]px` |
| three caps | `fit-content(10%) fit-content(20%) fit-content(70%)` | `[10, 20, 50, 10]px` |
| mixed caps | `fit-content(20%) 12px fit-content(80%)` | `[20, 12, 50, 6]px` |
| mixed size types | `fit-content(20%) 12px 10%` | `[20, 12, 10, 20]px` |
| minimum floor | `fit-content(20%) fit-content(40%)` | `[28, 34]px` |
| zero/above max | `fit-content(0%) fit-content(120%)` | `[0, 34]px` |

모든 값은 explicit 10px row 뒤의 implicit rows만 나열한다. 목록의 첫 항목부터 반복해 배치하고, `fit-content` minimum floor와 max-content cap을 구분한다.

## 지원·거부 경계

- Grid owner 하나, parsed Root HTML 기본 runtime 경로, definite 40×100px content box, explicit 40px column·10px row, `grid-auto-flow:row`, `grid-auto-columns:auto`, no areas/repeat/named lines, zero gap/edge, static positioning, start alignment 및 visible overflow를 유지한다.
- 기존 singleton source shape(첫 explicit 10px item, intrinsic item, 6px probe), 두 implicit auto row를 쓰는 3-item list shape, 새 4-row pattern shape(첫 explicit item 뒤 직접 배치된 auto-sized item 네 개)를 허용한다. Auto-sized implicit item은 직접 HTML `div`이고 각각 단일 fixed-px nested `div`를 가진다. 기존 singleton의 첫 row floor와 2-row list의 첫 implicit item floor만 허용하며 다른 min/max sizing, text item, replaced item 및 nested Grid는 거부한다.
- `grid-auto-rows` 목록은 비어 있지 않아야 한다. `fit-content()`를 포함하고, 모든 entry가 finite non-negative `fit-content(px|%)`, fixed-px 또는 direct percentage여야 한다. direct percentage와 fit-content percentage는 definite 100px content-box 높이를 basis로 사용한다.
- `fr`, `auto`, `min-content`, `max-content`, `minmax()`, percentage-bearing `Calc` AST, 음수·비유한 typed 값, 지원 밖 track-size와 부적격 DOM/style shape는 fail-closed한다. Length-only constant `calc()`가 기존 cascade에서 `LengthPx`로 정규화되는 경우 singleton 회귀는 유지한다.
- unsupported 값 또는 layout 실패는 partial layout/draw snapshot을 내보내지 않으며, 같은 `RuntimeSession`의 다음 유효 revision이 복구되는지 확인한다.
- 일반 author CSS admission, 임의 HTML/track grammar, dense·column flow, implicit columns, explicit placement, spanning, auto-repeat, WPT 통과 또는 full C11.4/C11 완료를 선언하지 않는다. 공개 API/ABI를 추가하지 않고 내부 계약 숫자 `0.1.0`을 유지한다.

## 구현과 검증 관문

1. 이 계획의 20개 실패 관점 검토를 구현 전에 완료한다. Chrome 관측과 별도 계산을 바꾸지 않은 상태에서 implementation review는 새로운 관점으로 작성한다.
2. Style projection에서 computed auto-row pattern의 entry 종류·순서·finite/non-negative 값을 보존한다. 목록 길이 제한은 기존 grid track resource limit을 사용한다.
3. Layout은 fit-content implicit-row profile에서 위 allowlist의 track-size 목록을 전달하고 첫 implicit row부터 CSS pattern 순서로 반복한다. 기존 singleton px/percentage behavior와 다른 Grid placement mode에 회귀가 없어야 한다.
4. Layout tests는 6개 사전 비교 장면의 final row sizes, item frames, nested overflow, auto placement 및 pattern cycling을 확인한다. Typed track 종류와 order가 projection 후에도 보존되는 assertion을 둔다.
5. Default RuntimeSession tests는 computed style, source/document/style/layout revision identity, layout/render publication을 확인한다. Invalid list, unsupported member, indefinite height, wrong explicit track, row gap, wrong flow/placement를 거부하고 실패 revision의 partial output 부재 및 same-session recovery를 검사한다.
6. Focused tests, changed-crate Clippy와 `cargo fmt --all -- --check`를 실행한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최종 source hash의 기본 Root HTML route를 빌드·실행해 `layout=ready`, item bounds와 screenshot을 기록한다. 화면은 visual smoke로만 분류하고 surface readback 증거와 혼동하지 않는다.
7. 실제 변경 코드/runtime에 대한 별도 적대 검증 20개와 PR diff 적대 검증 30개를 기록한다. PR 본문에는 두 Simulator 화면을 commit-pinned image로 붙인다. Merge 후 repository status, check rollup, rebase merge SHA를 확인한다.
8. Merge 후 `spec/STATUS.md`, 내부 계약 0059, 이 계획과 public roadmap을 동기화한다. 공식 C11/C11.4 checkbox와 `78/214` aggregate는 미완료로 유지하고, private Tailnet preview source/served hash를 대조한다. Agent Browser desktop/mobile 표시를 확인한 뒤 해당 브라우저 프로세스를 종료한다.

## 계획 자체 검토

[서로 다른 실패 경로 20개](./c11-4-root-html-fit-content-auto-rows-list-plan-review-20.md)를 확인했다. 계획 검토에서 정한 경계와 수락 기준은 implementation review에 재사용하지 않는다.

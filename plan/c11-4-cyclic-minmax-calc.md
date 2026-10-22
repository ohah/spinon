# C11.4 · cyclic percentage `minmax(calc())` track sizing 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [cyclic `calc()` track 계획](./c11-4-cyclic-calc-track-sizing.md) · [cyclic `minmax(percentage)` 계획](./c11-4-cyclic-percent-minmax.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 네 고정 scene 구현과 Android AVD/iOS Simulator 검증을 마쳤다. Chrome 156 Agent Browser 사전 관측과 CSS Grid 2025 CRD 기반 독립 기대값을 먼저 고정했다. 이 작업은 opt-in fixture 범위이며 C11.4 전체 완료가 아니다. 공식 checkbox와 완료 수 `72/213`은 유지한다. 실행 결과와 한계는 [런타임 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-runtime-2026-10-22/README.md)에 기록했다.

## 목적과 경계

직전 cyclic calc 계획은 calc를 track sizing function으로 직접 쓴 경우를 확인했고 `minmax(calc())`를 남은 범위로 명시했다. 이번 slice는 `minmax(0px, calc(50% + length))`의 최대 breadth가 intrinsic sizing과 최종 sizing에서 처리되는지 검증한다. `minmax(percentage)`와 bare `calc()`의 기존 결과를 회귀 control로 대신하지 않는다.

대상은 고정된 Ahem text, 단일 Grid item, 단일 explicit track과 네 allowlist scene이다. cyclic column·row scene은 intrinsic container size를 먼저 정하고 final `calc()` maximum을 다시 푼다. definite column과 small-limit control은 percentage basis가 각각 40px·10px일 때 최종 24px·9px을 확인한다. 일반 CSS author admission, 범용 nested Grid, WPT 및 내부 계약 버전 변경은 포함하지 않는다.

## 기준 입력과 독립 기대값

고정 HTML은 [사전 비교 fixture](../tests/fixtures/css/c11/grid-cyclic-minmax-calc-c114-precomparison.html), Chrome 156 observation과 screenshot은 [사전 비교 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-precomparison-2026-10-22/README.md)에 둔다. 모든 root/item은 margin·padding·border·gap이 0이고 Ahem은 `10px/10px`이다. item의 `min-width`와 `min-height`는 0으로 고정한다.

| scene | root sizing | track | 독립 기대 Columns / Rows | root / item |
| --- | --- | --- | --- | --- |
| `cyclic-minmax-calc-column` | width `max-content` | Columns `minmax(0px, calc(50% + 4px))` | `[29]` / `[20]` | `50×20` / `29×20` |
| `cyclic-minmax-calc-row` | width `20px`, height `max-content` | Rows `minmax(0px, calc(50% + 2px))` | `[20]` / `[22]` | `20×40` / `20×22` |
| `definite-minmax-calc-column` | width `40px` | Columns `minmax(0px, calc(50% + 4px))` | `[24]` / `[20]` | `40×20` / `24×20` |
| `definite-minmax-calc-small-limit` | width `10px` | Columns `minmax(0px, calc(50% + 4px))` | `[9]` / `[20]` | `10×20` / `9×20` |

CSS Grid Layout Module Level 1, 2025 CRD §7.2.1은 Grid container size가 해당 track percentage에 의존할 때 intrinsic size 계산에서는 percentage를 `auto`처럼 다루고, container size가 정해진 뒤 layout에서 결과 size에 대해 다시 resolve한다. `minmax()`의 두 breadth는 range의 minimum과 maximum으로 유지된다. column Ahem `aa aa`의 min-/max-content widths는 20/50px이다. intrinsic root width 50px에서 최종 maximum은 `50% × 50 + 4 = 29px`이다. row 입력 `a a a a`는 20px inline size에서 네 줄·40px intrinsic height를 만들고 final maximum은 `50% × 40 + 2 = 22px`이다. definite controls는 같은 계산식에 각각 40px·10px basis를 넣어 24px·9px을 얻는다. 이 기대값은 Chrome 결과를 역산하지 않았다.

small-limit control은 root 10px와 item 9px 바깥의 root-relative `(15,5)` 표본이 Ahem overflow color `#e5484d`이고, text 바깥 `(25,5)`는 흰 page background인지 확인한다. 이는 text paint overflow 표본이며 CSS border stroke acceptance가 아니다.

## 구현과 검증 관문

### 구현 결과

- 계획 사전 검토 20개 관점을 기록하고 독립 CSS Grid 기대값, Chrome 156 observation, fixture와 screenshot hash를 고정했다.
- 기존 style projection이 typed MinMax maximum Calc를 보존하는 경로를 직접 layout regression으로 검사했고 네 scene의 최종 track·frame 기대값이 통과했다.
- 네 scene만 허용하는 opt-in C11.4 adapter를 추가했다. 변경된 ID·DOM shape·style·track expression은 거부한다.
- 기본/fixture-test static archive 경계를 비교했다. 기본 archive marker는 absent, fixture-test positive control은 present다. Rust focused tests, fixture module boundary, rustfmt, Node syntax, workspace Clippy 및 세 feature 대상 Clippy가 통과했다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene을 실행했다. Android PixelCopy와 iOS 제출 전 Metal drawable 표본이 각각 9/9 exact다. Android 실기기와 iOS compositor 이후 값은 확인하지 않았다.
- 구현·런타임 적대적 검토 20개 관점은 런타임 근거 디렉터리의 implementation-review-20.md에 기록했다. 검토 중 발견한 Android sample default fallback은 fail-closed로 고치고 최종 APK와 AVD 실행을 다시 확인했다.

### 계획한 구현·병합 관문

1. 구현 전에 이 계획을 기존 계획 검토와 다른 실패 관점 20개로 검토하고 지적을 입력·판정·범위에 반영한다. 기준 fixture, Ahem 준비 상태, 기대값 JSON과 Chrome observation의 hash를 함께 고정한다.
2. style projection이 `MinMax.max`의 `Calc`를 `GridTrackMaxSizingFunction::Calc`와 동일한 `LayoutCalcId` expression으로 보존하는지 확인한다. 직접 layout regression에서 track trace의 최종 axis pass, root/item frame, calc percentage basis 및 실패 원자성을 검사한다. 제품 sizing algorithm을 바꾸기 전에 기존 출력과 independent expectation을 비교한다.
3. `c04-runtime-gpu-fixture-test` opt-in C11.4 adapter에 정확히 네 scene ID·DOM shape·Ahem text·track expression만 등록한다. 누락/추가 node, 변경 expression/style와 미등록 ID는 partial layout 없이 거부한다.
4. 기본 product archive에서 신규 fixture symbol, scene ID와 source 문자열이 빠지고 fixture-test positive control에는 들어가는지 비교한다. Rust focused tests, fixture module 경계 검사, `cargo fmt --all -- --check`, workspace all-target Clippy `-D warnings`, Android/iOS fixture build를 실행하고 결과를 분리 기록한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene의 Columns/Rows track, root/item frame, trace와 draw/present를 확인한다. Android PixelCopy와 iOS 제출 전 Metal drawable에서 scene별 유효한 표본을 비교한다. 실기기와 iOS compositor-after 결과를 주장하지 않는다.
6. 구현/runtime 변경은 계획 리뷰와 다른 실패 관점 20개로 새로 검토한다. 최종 PR diff는 프로젝트 규칙의 별도 30개 관점으로 검토해 사용자가 요청한 PR별 최소 20개를 충족한다. 발견한 문제는 수정하고 영향받은 관점을 재확인한다.
7. PR은 C11.4를 완료로 표시하지 않고 고정 fixture·남은 범위·화면·실행근거를 적는다. 현재 required checks와 merge state를 live 확인하고 Rebase and merge를 사용한다. PR screenshot 링크는 Agent Browser로 실제 로드 확인 후 해당 작업 session만 닫는다.

## 범위 밖

`minmax()`의 calc minimum, negative calc term, 여러 calc track, 두 축 동시 `minmax(calc())`, gap/border 변경, nested/anonymous/replaced item, 일반 CSS admission, WPT, Android 실기기, hardware GPU와 iOS compositor-after pixels는 검증하지 않는다. 네 fixture의 통과는 C11.4 또는 전체 CSS Grid 적합성 완료가 아니다.

## 근거

- [계획 자체 실패 관점 검토 20개](./c11-4-cyclic-minmax-calc-plan-review-20.md)
- [독립 기대값·Chrome 156 observation·screenshot](../spec/internal/evidence/c11-4-cyclic-minmax-calc-precomparison-2026-10-22/README.md)
- [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)

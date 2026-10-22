# C11.4 · cyclic percentage `fit-content(calc())` 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [직전 cyclic `calc()` 계획](./c11-4-cyclic-calc-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 고정 입력, 독립 기대값, Chrome 156 비교와 계획 실패 관점 검토 20개를 기록했다. 네 고정 scene의 runtime fixture 구현과 Android/iOS Simulator 4/4 검증을 마쳤고, 구현/runtime 및 PR diff 검토를 마치고 [PR #485](https://github.com/ohah/spinon/pull/485)를 열었다. live required checks와 merge state를 확인한 뒤 Rebase and merge한다. C11.4 공식 checkbox와 완료 수 `72/213`은 그대로다.

## 목적과 범위

직전 [cyclic `calc(percentage + length)` track slice](./c11-4-cyclic-calc-track-sizing.md)는 `calc()`가 직접 track sizing function인 네 scene을 확인했다. 이번 slice는 `fit-content(calc(percentage + length))`의 cyclic intrinsic basis와 최종 limit resolve를 기존 C11.4 opt-in fixture-test 경로에서 확인한다. 일반 CSS author admission이나 새 public API를 추가하지 않는다.

고정 입력은 별도 grid root와 direct text item `div` 한 개, 단일 explicit track, Ahem 10px text, zero gap·border·padding이다. 허용 scene ID는 네 개뿐이다. 다른 DOM shape, style, expression, scene ID는 기존 fail-closed 경계가 유지되어야 한다.

## 독립 기대값과 Chrome 비교

입력 fixture와 Chrome 156 observation은 [사전 비교 자료](../spec/internal/evidence/c11-4-cyclic-fit-content-calc-precomparison-2026-10-22/README.md)에 기록했다. CSS Grid Layout Module Level 1, 2025 CRD §7.2.1에서 cyclic percentage는 intrinsic container size를 구할 때 `auto`처럼 취급하고 결과 container size로 최종 resolve한다. 같은 절은 `fit-content(limit)`을 `max(auto minimum, min(limit, max-content))`로 정의한다. Chrome 156은 고정 비교 oracle이며 기대값은 Ahem metrics와 규범 sizing 순서에서 독립 계산한다.

| Scene | Root sizing | Track | limit basis | Columns | Rows | Root | Item |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| `cyclic-fit-content-calc-column` | width `max-content` | Columns `fit-content(calc(50% + 4px))`; Rows `auto` | `50×50%+4=29` | `[29]` | `[20]` | `50×20` | `29×20` |
| `cyclic-fit-content-calc-row` | width `20px`, height `max-content` | Columns `20px`; Rows `fit-content(calc(50% + 2px))` | `40×50%+2=22` | `[20]` | `[22]` | `20×40` | `20×22` |
| `definite-fit-content-calc-column` | width `40px` | Columns `fit-content(calc(50% + 4px))`; Rows `auto` | `40×50%+4=24` | `[24]` | `[20]` | `40×20` | `24×20` |
| `definite-fit-content-calc-small-limit` | width `10px` | Columns `fit-content(calc(50% + 4px))`; Rows `auto` | `10×50%+4=9` | `[9]` | `[20]` | `10×20` | `9×20` |

열 cyclic scene의 Ahem `aa aa` contribution은 min-content 20px, max-content 50px이다. intrinsic root width 50px에서 최종 limit 29px가 두 contribution 사이에 놓인다. 행 cyclic scene의 `a a a a`는 20px inline width에서 네 줄, intrinsic height 40px를 만든 뒤 최종 limit 22px을 사용한다. definite controls는 각각 limit 24px과 9px을 확인한다. 작은 definite control의 `min-width:0`은 9px item frame을 허용하고, Ahem 첫 단어의 20px glyph run은 root와 item 경계를 넘는다.

이 기대값은 [독립 기대값 JSON](../spec/internal/evidence/c11-4-cyclic-fit-content-calc-precomparison-2026-10-22/spec-derived-expectations.json)과 별도 Agent Browser observation을 대조했다. Chrome fixture의 page y 좌표는 document flow에 따라 달라지므로 판정은 root 기준 frame과 used track list에 둔다.

## 구현 및 판정 관문

1. 네 scene ID와 DOM·style allowlist만 기존 C11.4 opt-in profile에 연결한다. `ComputedCssMath`부터 `CalcId`, typed `FitContent` 입력, Taffy Grid track까지 percentage와 absolute length가 보존되는지 확인한다. 기존 generic CSS admission이나 layout 알고리즘은 바꾸지 않는다. generic path가 규범 결과와 다를 때만 원인을 좁혀 수정한다.
2. Rust style-to-layout/runtime 검사는 최종 Columns·Rows, root/item frame, typed calc basis와 축별 trace를 검사한다. 미등록 scene, DOM shape, 다른 track input과 unsupported expression이 fail-closed인지 확인한다.
3. small-limit control의 root 너비는 10 CSS px, item 너비는 9 CSS px이다. root 기준 `(15, 5)` 표본이 root와 item 양쪽 경계 밖에서 `#e5484d`인지 확인한다. `(5, 5)` 내부 control과 `(25, 5)` 글리프 바깥 control도 함께 비교한다. 이는 text/item overflow 표본이며 border stroke acceptance는 아니다.
4. 기본 제품 FFI archive에서 신규 fixture symbol·scene ID·payload가 빠지고 fixture-test positive control에만 포함되는지 비교한다. 관련 Rust tests, fixture boundary, `cargo fmt`, Clippy와 Android/iOS fixture build를 실제 실행하고 로그를 연결한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene 전부 frame·track trace·draw/present를 확인한다. Android PixelCopy와 iOS compositor 제출 전 Metal drawable 표본을 기록한다. 실기기는 사용하지 않는다.
6. 계획 검토와 다른 실패 관점 20개로 구현/runtime을 검토한다. PR diff는 별도 30개 실패 관점으로 병합 전에 검토한다. 발견한 결함은 수정 후 영향받은 matrix를 재실행한다.
7. PR 본문에는 시뮬레이터 화면과 fixed-fixture 한계를 적고 C11.4 완료를 주장하지 않는다. 구현/runtime 검토와 별도로 PR diff 실패 관점 30개를 기록한다. GitHub required checks와 merge state를 live 확인한 뒤 Rebase and merge한다.

## 구현 진행 결과

- 네 scene 모두 기존 `c04-runtime-gpu-fixture-test` 안에서만 생성한다. runtime fixture의 기존 Ahem text layout을 GPU render snapshot에 연결해 small-limit scene의 text overflow를 실제 색상 픽셀로 확인한다. 일반 CSS admission과 계산 profile은 바꾸지 않았다.
- Android API 37 ARM64 AVD `emulator-5558`와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 네 scene의 Columns/Rows track, root/item frame, draw/present를 통과했다. Android PixelCopy와 iOS 제출 전 Metal drawable은 각 9/9 표본이 exact다.
- 실행 로그, scene별 화면, 기본/fixture-test FFI artifact 대조 및 범위 한계는 [구현 실행 근거](../spec/internal/evidence/c11-4-cyclic-fit-content-calc-runtime-2026-10-22/README.md)에 둔다. C11.4 전체와 공식 완료 수는 미완료로 유지한다.

## 범위 밖

`minmax(calc())`, `fit-content()` 밖의 여러 calc track, 두 축 동시 `fit-content(calc())`, 복수 item, nested Grid, gap/border/writing-mode 변형, WPT, 일반 CSS admission, Android 실기기, hardware GPU, glyph equality, CSS border stroke, iOS compositor 이후 pixels와 전체 C11.4는 검증하지 않는다. Chrome 156 한 버전의 사전 관측만으로 다른 UA parity를 주장하지 않는다.

## 근거와 검토

- [Chrome 156 입력·관측·overflow 표본](../spec/internal/evidence/c11-4-cyclic-fit-content-calc-precomparison-2026-10-22/README.md)
- [고정 HTML fixture](../tests/fixtures/css/c11/grid-cyclic-fit-content-calc-c114-precomparison.html)
- [독립 기대값과 Chrome observation](../spec/internal/evidence/c11-4-cyclic-fit-content-calc-precomparison-2026-10-22/spec-derived-expectations.json)
- [계획 실패 관점 검토 20개](./c11-4-cyclic-fit-content-calc-plan-review-20.md)

## 판정

이 계획은 네 개 고정 scene의 `fit-content(calc())` cyclic sizing과 한 개 small-limit text overflow 표본에만 적용한다. simulator 결과와 후속 구현 PR이 통과해도 C11.4 checkbox나 공식 완료 수를 올리지 않는다.

# C11 parsed Root HTML Grid typed math gap

## 목표

기본 parsed Root HTML Grid 경로에서 node/property-bound typed CSS math `row-gap`과 `column-gap`을 실제 Grid sizing에 연결한다. 이미 투영된 `LayoutCssMath` 트리와 계산 evaluator를 재사용해 `calc()`의 산술, `min()`, `max()`, 세 인수 `clamp()`를 지원한다. 일반 CSS author admission, Flex의 기존 math gap, C11.1/C11 완료로 승격하지 않는다.

이 계획은 기존 Root HTML percentage/px gap 범위를 연장한다. 기본 explicit track Grid, 기존 Root HTML implicit-row, column-flow, 고정 named-area 경로가 받는 math gap을 같은 사용값으로 계산한다. 별도 CSS grammar parser, task-specific fixture FFI, runtime profile은 추가하지 않는다.

## 현행 경계

- Stylo cascade는 gap math를 `ComputedCssMath`로 보존하고 `parse_gap`은 `LayoutLengthPercentage::Calc(LayoutCalcId)` 및 property-bound `LayoutCssMathValue`를 만든다.
- layout 입력 검증은 계산 ID 중복, node/property 소유자 불일치, 누락, 계산 트리 depth/node/arity 한도를 검사한다. Flex의 math gap resolver와 일반 Taffy calc callback은 이미 이 owner-bound 식을 사용할 수 있다.
- Root HTML Grid admission은 `layout_math_values`에 `row-gap` 또는 `column-gap`이 있으면 projection에서 거부한다. 별도 Root HTML gap helper는 현재 normal/px/percentage만 해석하므로 implicit row와 column-flow 검사도 math 사용값을 계산하지 못한다.
- `min()`/`max()`/`clamp()` 및 `calc()`의 산술 연산은 현재 typed AST 표현 범위 안에서만 다룬다. `round()`/`mod()`/`rem()`/삼각 함수, 임의 CSS 함수, container query 등 새 계산 함수는 열지 않는다.

## 규범 기준과 범위

- [CSS Gaps Level 1](https://www.w3.org/TR/css-gaps-1/) §2.1은 `row-gap`/`column-gap`을 `normal` 또는 `[0,∞]` 범위 `<length-percentage>`로 정의한다. §2.2의 `gap`은 두 longhand의 shorthand이고, §2.3의 percentage는 대응하는 content box 축을 기준으로 한다. §2.4는 `grid-gap`을 `gap`의 legacy alias로 정의한다.
- [CSS Values and Units Level 4](https://www.w3.org/TR/css-values-4/) §10.9–10.12의 top-level calculation range checking에 따라 결과는 속성 범위 안으로 clamp/censor한다. Root HTML math gap은 existing `LayoutCssMathValue::resolve` 및 `RowGap`/`ColumnGap` nonnegative 정책을 따라 음수/NaN을 0으로, 양의 무한대를 현재 layout math 숫자 상한으로 제한한다.
- percentage를 포함하는 식은 기존 percentage gap과 같은 축의 definite finite nonnegative CSS-pixel basis를 요구한다. `row-gap`은 height, `column-gap`은 width를 쓴다. 해당 축의 기존 min/max 제약을 그대로 적용한다. percentage가 없는 pure length math는 percentage basis를 요구하지 않는다.
- cyclic/indefinite percentage 수식, cyclic intrinsic contribution 단계, 불명확한 size basis, 임의 min/max 조합은 지원하지 않는다. 수치 경계 스트레스는 finite output와 원자적 실패만 확인하고, Chrome의 극한 track 축소·좌표 clip을 일반 geometry parity로 주장하지 않는다.

## 기준 구현과 독립 기대값

Agent Browser `HeadlessChrome/156.0.0.0`에서 [고정 입력 및 원 관측](../spec/internal/evidence/c11-root-html-grid-math-gap-2026-11-03/README.md)을 작성했다. 19 scene HTML은 여러 Grid/named-area를 포함한 브라우저 oracle이며, named-area 제품 경로가 visible Grid 하나만 받으므로 통째로 제품 입력에 재사용하지 않는다. 별도 단일 Grid runtime fixture를 Android/iOS 양쪽에 동일 hash로 공급한다. 입력, 관측 JSON, 화면 hash는 evidence README에 기록했다. 브라우저 사용 후 Agent Browser 세션 0개와 browser daemon 미존재를 확인했다. expected 값은 CSS 규칙의 축별 percentage 및 Grid gutter arithmetic으로 별도 계산했고 Chrome 사용값은 oracle로 분리했다.

| 장면 | 독립 기대값 |
|---|---|
| `calc-mixed-column`, `calc-mixed-row` | 5%/10%의 해당 content-box 축 기준 계산에 2px를 더해 각각 12px, 두 번째 item x/y=22px |
| `calc-absolute`, `calc-divide` | 산술 결과 12px, column item x=22px |
| `calc-min`, `calc-max`, `calc-clamp` | 사용 gap 20/30/12px, item x=30/40/22px |
| `calc-clamp-min-wins` | `clamp(30px, 7px, 10px)`은 min > max일 때 min 우선; gap 30px, item x=40px |
| `calc-product` | 5%×2로 cascade 정리된 10%는 200px width에서 20px, item x=30px |
| `calc-zero-basis` | definite width 0px에서 75%+2px는 2px, 두 zero-width item x=0/2px |
| `calc-negative`, `calc-nan` | top-level nonnegative range check 결과 0px, item x=10px |
| `calc-over-100` | 120%+1px는 121px로 clamp하지 않고 둘째 item x=131px, scroll width 141px |
| `calc-infinity` | 수치 상한까지 유한하게 제한한다. Chrome track 축소와 좌표 clip은 수치 스트레스 기록으로만 보존한다. |
| `calc-implicit-row` | row gap 7px, explicit/implicit rows `[20,25]`, 다음 row y=27px |
| `calc-column-flow` | column gap 12px, 50% implicit column 100px, 다음 column x=52px |
| `calc-named-area` | column/row gaps 8/5.5px, spanning header width 128px, lower row y=25.5px, next column x=48px |
| `calc-shorthand`, `calc-legacy-alias` | `gap`과 `grid-gap` 모두 row/column 7/12px, item origins `(0,0)`, `(22,0)`, `(0,17)`, `(22,17)` |

## 구현 요구

1. Root HTML projection은 node/property-bound typed math가 있는 gap을 받아 `LayoutLengthPercentage::Calc`와 동일 ID의 CSS math record를 보존한다. 일반 px, percentage, normal 경로는 그대로 유지한다.
2. 표준 Root HTML explicit track Grid의 calc callback에서 `RowGap`/`ColumnGap` owner와 실제 axis basis가 맞는지 확인하고, used gap을 해당 property의 nonnegative 범위 정책으로 resolve한다. math AST가 percentage를 포함하지 않으면 resolver에 0 basis를 전달해 pure-length gap이 불필요하게 definite axis를 요구하지 않게 한다.
3. Root HTML implicit-row, column-flow, named-area 경로는 각각 같은 expression을 해당 축 basis로 검사한다. gap은 row/column의 명시·암시 track extent와 area span 계산에서 한 번만 반영한다.
4. percentage가 포함된 식에서 indefinite, auto 또는 음수/non-finite basis, 축을 잘못 바꾼 basis, 기존 min/max 제약 위반, node/property/ID 불일치를 reject한다. pure length calc는 percentage basis를 요구하지 않는다. 이 basis 예외는 gap property에만 적용하고 다른 CSS math property의 resolver 의미는 바꾸지 않는다.
5. 실제 layout 전에 resolved gap과 모든 적용 gutter 수를 곱한 aggregate extent, explicit/implicit track 및 area span과의 합, f32/작업 상한을 검사한다. gap 수식 오류·resource cap·범위 초과 시 전체 layout 결과를 폐기하고 부분 frame을 발행하지 않는다.
6. 기존 math tree 최대 깊이 32, 노드 256, 함수 인자 64 제한을 유지한다. unsupported 계산 트리, invalid dimension, missing owner는 exact `row-gap`/`column-gap` property로 fail closed한다.
7. Flex profile 및 Root HTML Grid가 아닌 profile에 math gap admission을 전파하지 않는다. `gap` shorthand와 `grid-gap` alias는 computed longhand의 동일 property binding을 공유한다.

## 검증

- cascade/projection: 19 scene browser oracle의 각 declaration을 isolated style/layout unit case로 만들고 math AST, longhand axis, shorthand/alias, zero basis, 기존 invalid literal declaration의 cascade 보존을 검사한다. 19-Grid HTML 전체를 단일 Root HTML product admission에 통과시키려 하지 않는다. layout rejection은 error 시 부분 frame을 발행하지 않는지 별도로 확인한다.
- layout: mixed percentage/length, pure length, min/max/clamp/product/divide, negative/NaN/large finite/infinity censor 경계, implicit row/column, named-area span, >100% overflow를 exact frame 및 finite extent로 검사한다.
- adversarial ownership: missing/duplicate calc ID, wrong node/property, row/column swapped basis, wrong percentage axis, indefinite basis, nonfinite basis, zero basis, negative result, invalid expression type, depth/node/arity caps, aggregate extent overflow, output atomicity를 확인한다.
- [`grid-math-gap-runtime-v1.html`](../tests/fixtures/css/c11/grid-math-gap-runtime-v1.html)의 단일 Grid fixture hash를 기본 parsed Root HTML 제품 경로에서 Android API 37 ARM64 AVD와 iOS Simulator에 똑같이 실행한다. `layout=ready`, expected frame, draw/present와 visible overflow를 기록한다. 19 scene browser oracle의 전체 hash는 앱 입력 hash와 혼동하지 않는다. screenshot은 시각 smoke이며 native surface readback 또는 Chrome pixel equality로 부르지 않는다.
- scoped focused Rust tests, 해당 crate Clippy, `cargo fmt --all -- --check`, `git diff --check`를 수행한다. 전체 workspace 결과는 별도 기록하고 집중 pass로 일반화하지 않는다.
- 구현 후 새로운 20개 implementation/runtime 실패 관점과 PR 변경분 30개 검토를 수행한다. plan review 20개는 재사용하지 않는다.

## 제외 범위

- cyclic/indefinite percentage gap 및 intrinsic contribution, 임의 root sizing/minmax, writing mode, RTL, fragmentation, alignment-added gap.
- `round()` 계열 새 함수, arbitrary typed math expansion, 일반 CSS 지원 또는 WPT conformance 주장, Flex gap 의미 변경.
- 새 app loader, CSS parser 또는 FFI/fixture adapter, hardware GPU/실기기 및 iOS post-compositor pixel equality.
- C11.1/C11 완료 체크, `spec/STATUS.md` 공식 수치 증가 또는 내부 계약 0059 숫자 버전 변경. 숫자 버전은 출시 전 `0.1.0` 유지.

## 완료 조건

- 본 계획은 구현 전에 별도의 20개 실패 관점으로 검토한다. 그 검토는 구현/runtime 20개와 PR delta 30개 검토에 재사용하지 않는다.
- 사전 비교 입력 hash 및 Agent Browser Chrome 156 관측을 고정한다. Independent expected와 Chrome observation을 구분한다.
- focused test, scoped lint/format, Android/iOS simulator, failure injection 및 frame/overflow 결과를 원본 log/JSON/hash에 연결한다.
- 구현 PR 병합 전 필요한 CI/status와 별도 30-point review를 확인한다. 완료 표시는 검증된 병합 뒤에만 수행하고, 이 한정 기능만으로 C11.1/C11 완료를 표시하지 않는다.
- plan PR은 사용자 요청에 따라 같은 checkout에서 순차 처리한다. worktree를 만들지 않는다.

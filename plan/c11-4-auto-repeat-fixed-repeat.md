# C11.4 · fixed-repeat와 auto-repeat 혼합

**상태:** 계획 및 Chrome 사전 비교만 완료. 구현/runtime은 시작 전이다. C11.4 checkbox와 공식 완료 수 `72/213`은 그대로 유지한다.

## 기존 동작과 결손

CSS Grid 2025 CRD §7.2.3.1은 같은 track list에 `auto-repeat` 하나와 `<fixed-repeat>` 하나 이상이 함께 있을 수 있다고 정의한다. 현재 Spinon의 제한 auto-repeat profile은 [cascade adapter](../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs)의 `computed_auto_repeat_tracks()`가 `RepeatCount::Number(_)`를 발견하면 거부한다. 결과적으로 CSS 문법상 유효한 `repeat(2, 20px 10px) repeat(auto-fill, minmax(32px, 1fr)) 16px` 형태가 현재 제한 profile에서 fail-closed된다.

이 계획은 CSS 표준 전체 repeat grammar를 구현하지 않는다. 기존 auto-repeat의 definite-axis 계산, 허용 sizing function, node/property 및 calc binding, Taffy placement·collapse 동작을 유지하고, 정수 반복 track 하나를 한 auto-repeat의 앞 또는 뒤에서 제한적으로 전개하는 방안을 검증한다.

## 목표와 제외

- `grid-template-columns` 또는 `grid-template-rows` 중 한 축에 auto-repeat 하나와 fixed-repeat 하나를 함께 둔다. 한 fixture grid에서 두 축을 동시에 확장하지 않는다.
- fixed-repeat은 auto-repeat 앞이나 뒤에 하나만 둔다. 횟수는 양의 정수이며 fixed-repeat 안은 line name 없이 직접 finite nonnegative `px` breadth 1~2개로 제한한다. 예: `repeat(2, 20px 10px)`.
- auto-repeat pattern은 현재 지원 profile을 그대로 쓴다. 사전 비교는 `minmax(32px, 1fr)` 하나로 고정한다. auto-repeat 최대/최소 `calc()`, percentage, intrinsic 및 flexible 조합의 whitelist는 넓히지 않는다.
- fixed-repeat 외부 track도 현재 auto-repeat neighbor validator를 그대로 통과해야 한다. named/repeated line, `grid-template-areas`, 두 번째 fixed-repeat, 복수 auto-repeat, nested repeat, unsupported neighbor나 새로운 placement 계약은 제외한다.
- expanded authored track descriptor 수는 현재 cascade `MAX_COMPUTED_GRID_TRACKS` 한도 10,000 이내여야 한다. checked multiply/add와 fallible reservation을 사용하고 allocation·limit 실패에서 일부 track을 반환하지 않는다. auto-repeat을 실제 확장한 결과는 layout의 기존 `GRID_MAX_TRACKS` 한도를 따른다.
- 이 fixture-only profile은 공개 CSS admission이나 전체 C11.4 지원을 뜻하지 않는다. 내부 계약 `0059`의 숫자 버전 `0.1.0`을 유지한다.

## 기준과 사전 판정

- 규범 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.1–7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation). 같은 list에 fixed-repeat을 허용하고, auto-repeat은 한 번만 허용한다. definite axis에서는 non-repeated track·repeated minimum·전체 gap을 합산해 overflow 없는 최대 양의 repetition count를 정한다. `auto-fit`은 placement 이후 empty repeated track을 collapse한다.
- 고정 oracle: [Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/README.md), [입력 fixture](../tests/fixtures/css/c11/grid-auto-repeat-fixed-repeat-c114-precomparison.html), [관찰 JSON](../spec/internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/chrome156-observation.json).
- Chrome은 macOS 26.5.1 ARM64, HeadlessChrome `156.0.0.0`, viewport `1280×633`, DPR 1에서 실행했다. 기준 screenshot과 입력 SHA-256은 사전 비교 evidence에 기록했다.

| Case | 축·순서·모드 | 사전 기대/Chrome used tracks | 배치 관찰 |
| --- | --- | --- | --- |
| `fill-prefix-empty` | columns, fixed prefix, auto-fill | `20 10 20 10 38 38 16px` | 반복 2개, 각 flex track 38px |
| `fill-suffix-empty` | columns, fixed suffix, auto-fill | `16 38 38 20 10 20 10px` | suffix fixed-repeat의 순서 보존 |
| `fit-prefix-empty` | columns, fixed prefix, auto-fit | `20 10 20 10 0 0 16px` | 반복 횟수 2 산정 후 두 track collapse |
| `fit-prefix-occupied` | columns, fixed prefix, auto-fit | `20 10 20 10 84 0 16px` | column 5 item x=92px; occupied track·repeat boundary gutter 유지 |
| `row-fill-prefix-empty` | rows, fixed prefix, auto-fill | `20 10 20 10 38 38 16px` | columns와 같은 독립 definite-height 산식 |
| `row-fill-suffix-empty` | rows, fixed suffix, auto-fill | `16 38 38 20 10 20 10px` | suffix fixed-repeat의 순서 보존 |
| `row-fit-prefix-empty` | rows, fixed prefix, auto-fit | `20 10 20 10 0 0 16px` | 두 repeated row collapse |
| `row-fit-prefix-occupied` | rows, fixed prefix, auto-fit | `20 10 20 10 84 0 16px` | row 5 item y=92px; occupied track·repeat boundary gutter 유지 |

독립 반복 수 계산은 두 축 모두 같다. fixed-repeat과 suffix를 포함한 non-repeated breadth 합은 76px, non-repeated track 5개, 각 auto track의 minimum은 32px, gap은 8px다. `n`회 반복 시 필요한 최소 폭/높이는 `76 + 32n + 8(4+n) = 108 + 40n`이므로 200px axis에는 2회가 들어가고 3회는 넘친다. 두 `1fr`의 사용 크기는 각각 38px다. occupied auto-fit은 collapse된 두 번째 track의 boundary 처리까지 검증하며, 기준값 84px와 item의 relative position 92px를 함께 비교한다. Chrome CSSOM fractional size 허용 오차는 1/64 CSS px, integer track과 item/root geometry는 정확 일치다.

## 구현 순서

1. 구현 전에 Stylo computed AST가 같은 list의 `RepeatCount::Number`와 `AutoFill`/`AutoFit`을 어떤 순서·track-size·line-name 형태로 내보내는지 cascade fixture에서 확인한다. 실제 AST가 고정 oracle·계획과 다르면 구현 전에 이 계획과 독립 기대값을 다시 검토한다.
2. auto-repeat list parser는 정확히 하나의 auto-repeat, 최대 하나의 integer repeat, 지정된 prefix/suffix 순서, 빈 line-name groups, 그리고 direct nonnegative `LengthPx` fixed-repeat fragment만 받아들인다. 다른 RepeatCount, named line, 두 번째 repeat group, overflow·non-finite 값은 node/property 맥락이 있는 기존 fail-closed 오류로 거부한다.
3. integer group의 positive count와 pattern 길이를 checked arithmetic으로 전개해 auto-repeat pattern 앞/뒤 track 목록에 추가한다. auto-repeat `start/end`는 전개된 track index를 가리키도록 조정한다. empty line-name metadata는 확장된 component 경계 `N+1`을 보존한다. 기존 auto-repeat line metadata, calc ID/node/property binding, neighbor validation은 변경하지 않는다.
4. 단위·cascade 검증은 prefix/suffix, columns/rows, typed track order와 auto-repeat index, empty/nonempty line groups, fixed repeat의 unsupported breadth/name, 두 repeat group, multiple auto-repeat, 10,000 descriptor 경계, checked overflow, reservation failure/no partial output을 다룬다.
5. layout 검증은 `GRID_MAX_TRACKS`, fixed-repeat의 track 수·순서·used size, 모든 active gutter, auto-fill placement, auto-fit empty/occupied collapse와 prefix/suffix item 위치를 독립 산식 및 8-case oracle과 비교한다. unsupported input은 부분 layout을 반환하지 않아야 한다.
6. opt-in runtime fixture는 8개 고정 scene을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 실행한다. frame, per-axis track trace, layout revision, draw/present를 검사하고 simulator 화면 표본을 저장한다. AVD software renderer 및 iOS compositor 제출 전 sample로 한정한다.
7. 기본 product artifact에는 fixture id/symbol/payload가 빠지고 opt-in fixture-test positive control에는 들어가는지 비교한다. static archive 결과를 APK/IPA 전체 제외 근거로 확대하지 않는다.

## 통과 조건과 별도 검토

- 이 계획을 구현하기 전에 서로 다른 실패 관점 20개로 [계획 검토](./c11-4-auto-repeat-fixed-repeat-plan-review-20.md)를 완료한다. 아래 관점은 계획 구현 검토에서 재사용하지 않는다.
- 구현 뒤 새 code/runtime 실패 관점 20개를 작성하고, 이 계획 및 기존 Chrome 관찰과 독립적으로 검토한다.
- 병합 전 PR diff 전용 실패 관점 30개를 작성한다. 이 계획-only PR은 [계획 PR diff 검토 30개](./c11-4-auto-repeat-fixed-repeat-pr-diff-review-30.md)에 기록했다. 구현 PR에는 `spec/STATUS.md`, 내부 계약 `0059`, public roadmap 및 근거 링크를 함께 갱신하고 PR template를 채운다.
- 기존 CSS Grid/API unrelated tests나 전체 WPT를 이 slice의 성공으로 세지 않는다. 실행한 scoped tests/build/lints와 Android/iOS Simulator 결과만 구분해 보고한다.
- 모든 단계에서 C11.4 checkbox를 미완료로 두고 공식 완료 수 `72/213`을 유지한다.

## 미검증 및 남은 범위

여러 fixed-repeat, line-name merge/collision, fixed-size `minmax()`/percentage/calc, flex/intrinsic fixed-repeat track, simultaneous auto-repeat on both axes, template areas, general placement/span/gutter cross-product, arbitrary CSS admission, WPT, Android physical device/hardware GPU, iOS device/compositor-after pixels 및 전체 C11.4는 이 계획으로 닫지 않는다.

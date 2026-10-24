# C11.4 · auto-repeat 앞뒤 `calc()` 이웃 track 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [percentage neighbor 선행 slice](./c11-4-auto-repeat-percentage-neighbors.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태 · 2026-10-24:** Agent Browser의 Chrome 156 사전 비교와 계획 실패 관점 20개 검토를 마쳤다. 현재 미커밋 소스는 cascade에서 representable track shape를 보존하고, `spinon-style-to-layout` projection 경계에서 제한 typed AST를 검사한다. subset 밖 shape는 node/property가 포함된 `UnsupportedComputedValue`로 거부한다. 14-case opt-in FFI fixture와 Android/iOS runner가 연결됐다. focused Rust `auto_repeat` suites는 `spinon-layout` 51/51, `spinon-style` 19/19, `spinon-style-to-layout` 15/15 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 현재 소스의 14-scene matrix가 각각 14/14를 통과했다. 각 플랫폼에서 11 positive scene의 frame·tracks/collapse·trace·draw/present, 3 rejection control의 typed diagnostic 및 frame 부재를 확인했다. PixelCopy와 Metal drawable 표본은 각 24/24 exact이며 overflow scene 표본은 각 5/5 exact다. `InvalidGridTrackList`의 generic `layout_failed` mapping을 실제 AVD에서 찾아 typed `invalid_grid_track_list` code/node/property mapping과 회귀 test를 추가했다. [현재 소스 실행·hash·screens](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-projection-pass03-2026-10-24/README.md) · [초기 capture flag 누락](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-projection-pass01-2026-10-24/README.md) · [runtime error mapping 진단](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-projection-pass02-2026-10-24/README.md). product `c04-runtime-gpu` release FFI archive에서는 14 scene ID와 C11.4 fixture FFI symbol이 없고, `c04-runtime-gpu-fixture-test` positive-control archive에는 14/14 scene ID와 symbol이 있다. 이는 macOS ARM64 static archive feature 경계 근거이며 최종 APK/IPA exclusion을 증명하지 않는다. [artifact 빌드·검사 근거](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-artifact-2026-10-24/README.md). 새 구현/runtime 실패 관점 20개 검토를 별도 기록했다. 다음 gate는 PR 범위를 고정한 뒤 수행할 서로 다른 30개 diff 실패 관점 검토다. C11.4 상위 체크는 미완료이고 공식 집계 `72/213`을 유지한다.

## 코드 경계와 목적

PR [#580](https://github.com/ohah/spinon/pull/580)은 definite positive content axis에서 유한·0 이상 `LengthPx`/`Percentage` neighbor를 제한 auto-repeat profile에 연결했다. 현재 미커밋 [Stylo gate](../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs)와 [layout gate](../crates/spinon-layout/src/grid.rs)는 computed `Sum` AST에 정확히 두 항이 있는 calc neighbor를 제한 지원한다. 항은 `LengthPx` 또는 단일 `Negate(LengthPx)` signed term 하나와 유한 0 이상 `Percentage` 하나이며 순서는 양쪽을 허용한다. Layout은 unique calc ID와 같은 node/property binding, definite positive content basis를 요구하고 basis 곱과 aggregate overflow를 검사한다. 기존 auto-repeat track 내부 sizing과 바깥 neighbor의 `calc()`는 별도 경계다.

이 계획은 fragment 앞·뒤에 쓰는 제한 2-term `calc(length + percentage)` 하나 이상을 위 neighbor 입력에 추가한다. 일반적인 CSS math 평가기를 새로 만들지 않고, Stylo가 보존한 `ComputedCssMath::Sum`의 정확히 두 항인 `LengthPx`와 `Percentage` 조합만 쓴다. 합의 항 순서는 어느 쪽이든 허용한다. Length 항은 유한 signed 값이며 Percentage 항은 유한 0 이상 값으로 제한한다. 다만 지정 축 basis에서 최종 계산값은 CSS range 규칙에 맞춰 0 이상으로 clamp한다.

## 규범 기준, oracle와 재현

- [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1, §7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat): auto-repeat의 fixed-size 조건, column inline/row block percentage basis, definite content box·non-repeated tracks·gutter를 포함한 반복 횟수, 최소 1회 fallback 및 auto-fit collapse.
- [CSS Values and Units Level 4 §10.12](https://www.w3.org/TR/css-values/#range-checking): calc 최종값을 허용 범위에 clamp한다. Grid `<length-percentage [0,∞]>` neighbor의 음수 최종 결과는 0px로 사용한다. CSS-valid calc 선언 자체를 invalid syntax로 취급하지 않는다.
- 기준 브라우저는 Agent Browser 0.31.1의 Chrome 156.0.8078.48, macOS ARM64, 1280×633, DPR 1이다. [14-scene fixture](../tests/fixtures/css/c11/grid-auto-repeat-calc-neighbors-c114-precomparison.html), [독립 계산과 Chrome 관측](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/README.md), [fixture observation](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/chrome156-observation.json), [validator](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/validate-precomparison.py)가 재현 입력과 관측값을 고정한다.
- 재현 명령은 `python3 spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-precomparison-2026-10-24/validate-precomparison.py`다. 14 scene의 규범 계산과 Chrome CSSOM은 1/64 CSS px 허용 범위에서 통과했다. 이 validator는 제품 runtime 검증이 아니다.

## 고정 장면과 기대값

각 positive scene의 반복 track minimum은 40px, gap은 4px이다. 일반 definite content axis는 240px이고 border-box scene의 content basis는 `136 - 16 padding - 4 border = 116px`이다. 독립 계산은 fixture에서 Chrome 출력과 분리해 생성한다.

| 장면 | normative track 기대값 | 경계 |
|---|---|---|
| `col-fill-both`, `row-fill-both` | count 4, `[22,47,47,47,47,10]` | 양쪽 calc neighbor, 두 axis |
| `col-fill-prefix` | count 4, `[22,50.5,50.5,50.5,50.5]` | prefix만 |
| `row-fill-suffix` | count 5, `[42,42,42,42,42,10]` | suffix만, row basis |
| `col-fit-first` | count 4, `[22,200,0,0,0,10]` | 점유 repeat 1개, 나머지 collapse |
| `col-fit-empty` | count 4, `[22,0,0,0,0,10]` | 모든 auto-fit repeat collapse |
| `col-fill-overflow` | count 1, `[196,40,74]`; 78px track/gap overflow | 최소 1회 규칙; item은 grid border-box 오른쪽 4px 밖에 놓음 |
| `col-fill-border-box` | basis 116, count 2, `[9.8,44.15,44.15,5.9]` | border/padding 제외 후 percentage basis |
| `col-fill-negative-term` | count 5, `[10,42,42,42,42,42]` | signed length term, 최종 10px |
| `col-fill-negative-final` | count 5, `[0,44,44,44,44,44]` | calc 최종값 음수, 0px clamp |
| `col-fill-negative-percentage-term` | count 5, `[8,42.4,42.4,42.4,42.4,42.4]` | CSS-valid negative control; negative percentage term은 AST whitelist 밖 |
| `col-fill-extra-term` | count 4, `[22,50.5,50.5,50.5,50.5]` | CSS-valid negative control; `Sum` 안의 nested `clamp()` 항은 whitelist 밖 |
| `col-max-content` | repeat count 1 | indefinite/cyclic CSS-valid negative control |
| `col-fill-fixed-control` | count 4, `[22,47,47,47,47,10]` | PR #580 fixed-px neighbor 회귀 |

14 scene 중 11개는 구현 대상, `col-fill-negative-percentage-term`·`col-fill-extra-term`·`col-max-content`는 CSS-valid negative control이다. Negative control 3개는 값이 유효하더라도 이 profile의 typed AST/basis 경계 밖이라 runtime에서 정확한 fail-closed error와 partial output 부재를 검사한다. `col-fill-extra-term`은 `calc(10px + 5% + clamp(0px, 0%, 1px))`를 쓴다. Stylo는 호환되는 단순 길이 항끼리 computed typed AST에서 합칠 수 있어 `calc(10px + 5% + 1px)`는 `[LengthPx(11), Percentage(0.05)]`로 정규화된 뒤 whitelist에 들어온다. 정책 경계는 원문 산술 항 개수가 아니라 projected typed AST이며, nested `clamp()`는 그 AST에 남아 거부된다. Chrome CSSOM observation은 control 선언도 CSS-valid임을 보존한다.

## 구현 진행과 남은 경계

1. **작업 트리 구현 완료:** Cascade projection은 auto-repeat 앞뒤의 representable track shape를 보존한다. `spinon-style-to-layout`은 top-level repeat neighbor 위치를 확인하고 computed typed AST의 두 항 `LengthPx` 또는 단일 `Negate(LengthPx)` + `Percentage` 조합만 허용한다. 항 순서는 양쪽을 허용하고, Stylo가 단순 길이 항을 정규화한 결과 AST를 기준으로 판정한다. 지원하지 않는 shape는 node/property가 포함된 `UnsupportedComputedValue`로 거부한다.
2. **작업 트리 구현 완료:** Layout은 고유한 calc ID와 같은 node/property binding을 요구한다. Percentage는 definite positive content size에 resolve하고 `length + percentage × basis` 표현 가능성·aggregate overflow를 확인한다. 기존 resolver가 CSS range clamp를 적용하며 최종 음수는 0px이 된다. 세부 재현 테스트는 `spinon-layout` 3/3 통과했다.
3. **작업 트리 구현 완료:** calc neighbor는 Taffy의 일반 track 입력으로 전달되며 auto-repeat count에서도 definite basis와 함께 계산된다. 반복 fragment의 sizing·auto-fit 점유 판정·alignment는 변경하지 않았다. 별도로 Taffy의 collapsed auto-fit run은 각 빈 track 뒤 gutter를 접고 run 경계의 shared gutter 하나를 남기도록 바로잡았다. Stylo typed-AST 회귀 테스트는 2/2 통과했다.
4. **현재 앱 실행 완료:** Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 현재 소스의 11 positive + 3 rejection scene을 각각 14/14로 확인했다. PixelCopy/Metal drawable 표본은 각 24/24 exact다. iOS 표본은 compositor 제출 전이며 Android는 AVD 한정이다.
5. **artifact 경계 확인 완료:** product와 fixture-test FFI static archive를 각각 release로 빌드하고 scene ID·fixture symbol의 부재/positive control을 확인했다. 근거는 [artifact 기록](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-artifact-2026-10-24/README.md)이다. 최종 Android APK/iOS IPA exclusion은 확인하지 않았다. 일반 CSS admission, WPT, 실기기·hardware GPU, iOS compositor 이후 pixels와 전체 C11.4는 범위 밖이다.

## 오류·실패 조건과 제외 범위

- Indefinite/cyclic percentage basis, non-positive root content size, unresolved axis, missing/mismatched calc ID/node/property, non-finite terms/intermediate/sum, f32 representability overflow는 `LayoutError` 또는 기존 precise cascade error로 거부한다. partial `LayoutOutput`은 만들지 않는다.
- Projected typed AST의 percentage term이 negative거나 length term 이외의 중첩·product/min/max/clamp, 또는 2-term 외 shape이면 현재 scope에서 거부한다. Stylo computed AST에서 호환되는 raw authored sum 항이 합쳐질 수 있으므로 이 경계는 authored 항 수가 아니라 projection 결과에 적용한다. CSS 자체가 유효하더라도 이 opt-in Spinon profile은 unsupported 입력을 조용히 0 또는 fixed 값으로 바꾸지 않는다. 최종 계산값이 음수인 지원 shape만 CSS 규범에 따라 0으로 clamp한다.
- Intrinsic/flexible neighbor, arbitrary author CSS admission, 복수 auto-repeat, 일반 Taffy percentage resolver 변경, WPT 전체, 실기기/hardware GPU, iOS compositor 이후 픽셀, 전체 C11.4/CSS Grid 적합성은 범위 밖이다.
- C11.4 checkbox와 공식 `72/213`은 유지한다. Runtime 통과만으로 C11.4 상위 완료를 주장하지 않는다.

## 구현 전 review gate

계획 자체의 20개 독립 실패 관점 검토는 완료했다. 현재 구현·simulator 확인·두 FFI archive feature-boundary 검증도 끝났다. 새 구현/runtime 20개 실패 관점 검토를 [별도 기록](../spec/internal/evidence/c11-4-auto-repeat-calc-neighbors-implementation-review-20-2026-10-24.md)했고 차단 결함은 찾지 못했다. 다음은 PR 준비 후 별도의 서로 다른 30개 diff 실패 관점 검토다.

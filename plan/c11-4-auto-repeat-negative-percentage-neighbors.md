# C11.4 · auto-repeat 이웃 `calc()`의 음수 percentage 항 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [calc 이웃 선행 구현](./c11-4-auto-repeat-calc-neighbors.md) · [percentage 이웃 계약](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**현재 단계:** 계획 PR [#584](https://github.com/ohah/spinon/pull/584)은 2026-10-24 05:41:39 UTC에 Rebase and merge되어 `411bda24e2525b55d48a6c14ec484aec6ef8e97a`에 병합됐다. 구현 PR [#585](https://github.com/ohah/spinon/pull/585)은 2026-10-24 06:25:51 UTC에 Rebase and merge되어 `e42b1573e710c0c36e9c507790902d4a35a53121`에 병합됐다. `gh pr checks 585 --required`는 이 브랜치에 보고된 check이 없다고 반환했다. exact 2-term signed percentage subset, 신규 9 scene과 회귀 2 scene은 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 11/11 통과했고 PixelCopy/Metal 표본은 각 22/22 exact다. 기본/fixture-test FFI archive 경계도 각각 negative/positive control로 확인했다. C11.4와 공식 완료 집계 `72/213`은 미완료 상태다.

## 기존 결손과 목표

지원 중인 definite-axis calc 이웃은 CSS Values 수학식 중 signed length 항을 보존하지만 signed percentage 항은 거부한다. 그 결과 열·행 auto-repeat 앞뒤의 `calc(length - percentage)`가 CSS Grid `<length-percentage [0,∞]>` 규칙을 만족해도 runtime profile에서 지원되지 않는다.

새 제한 profile은 Stylo computed typed AST의 정확히 두 항인 signed length 하나와 signed percentage 하나를 column/row auto-repeat neighbor로 허용한다. Percentage는 finite 부호 값 자체 또는 finite percentage 하나를 감싼 단일 `Negate`로 보존될 수 있으며 두 항의 순서는 모두 허용한다. 음수 percentage 자체는 유효한 계산식 항이며, 선택한 definite content basis에서 top-level 계산 결과를 property의 nonnegative range에 clamp한다. profile의 기존 finite·layout-representable 경계 안에서 결과가 양수이면 그 used size를 반복 횟수 계산에 쓰고, 음수이면 0px 이웃 track으로 취급한다.

이 계획은 일반 CSS math evaluator나 전체 Grid 지원을 확장하지 않는다. 제안 범위 밖 typed AST는 현재처럼 property·node 정보를 보존해 fail-closed한다. 공식 C11.4 완료 checkbox와 `72/213`을 올리지 않는다.

## 규범 기준과 고정 oracle

- [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)는 track breadth를 nonnegative `<length-percentage [0,∞]>`로 정의한다. Column percentage basis는 inner inline size, row basis는 inner block size다.
- [CSS Grid 2025 CRD §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat)는 definite axis의 반복 횟수를 고정 sizing function과 gap에 따라 정하고, auto-fit의 빈 반복 track/gutter collapse를 auto-fill과 구분한다.
- [CSS Values and Units Level 4 §10.12](https://www.w3.org/TR/css-values-4/#calc-range)는 math function 내부 값의 parse-time range rejection을 금지하고 top-level result를 target context의 닫힌 범위로 clamp한다.
- Agent Browser 0.31.1의 Chrome 156.0.8078.48, macOS ARM64, 1280×633, DPR 1에서 [9-scene fixture](../tests/fixtures/css/c11/grid-auto-repeat-negative-percentage-neighbors-c114-precomparison.html)를 관찰했다. 독립 기대값, CSSOM, frame은 [사전 비교 보고서](../spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.md), [observation JSON](../spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-observation.json), [전체 화면](../spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/chrome156-full.png)에 둔다.
- `python3 spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/validate-precomparison.py`는 Chrome CSSOM을 독립적인 40px minimum/4px gap 계산과 1/64 CSS px 범위에서 대조한다. 이 결과는 Chrome oracle이며 Spinon 실행 결과가 아니다.

## 변경 범위

1. Stylo cascade는 기존 `ComputedCssGridTrackSize::Calc`의 computed math tree를 보존한다. 이미 보존 가능한 형태를 raw authored token 수로 재작성하지 않는다.
2. `spinon-style-to-layout`은 top-level repeat neighbor에서 정확히 두 항인 `Sum`만 검사한다. 한 항은 기존 signed length shape(`LengthPx` 또는 단일 `Negate(LengthPx)`), 다른 항은 finite signed `Percentage` 또는 단일 `Negate(Percentage)`로 제한한다. 항 순서는 둘 다 허용한다. 실제 Stylo AST가 이 계획 shape와 다르면 구현 전에 계획과 oracle을 재검토한다.
3. `spinon-layout`은 기존 calc ID, 같은 node/property binding, definite positive content-axis basis 및 overflow 검사를 유지한다. Percentage 부호를 보존해 resolver에 전달하고, `GridTemplateColumns`/`GridTemplateRows`의 nonnegative property range에서 final value를 0 이상으로 clamp한다. Positive overflow·nonfinite 입력, 부정확한 binding은 계속 거부한다.
4. Taffy auto-repeat count, repeated track minimum/flex sizing, prefix/suffix gap 산정, auto-fit collapse 알고리즘은 변경하지 않는다. 계산된 neighbor used size만 기존 경로에 전달한다.
5. opt-in fixture-test에 Chrome scene을 연결하되 fixture ABI·입력·화면·문자열은 제품 빌드에서 빠져야 한다. 일반 release 앱의 CSS admission을 넓히지 않는다.

## 검증 관문

### 계획 PR

- 계획 자체 실패 관점 20개를 [별도 기록](./c11-4-auto-repeat-negative-percentage-neighbors-plan-review-20.md)하고 서로 다른 기준으로 통과시킨다.
- 이 계획 변경만 대상으로 PR diff 실패 관점 30개를 기록한다.
- 계획 PR은 Rebase and merge하며 완료 checkbox는 바꾸지 않는다.

### 구현 PR

- Stylo 실제 computed AST를 단위/cascade test에서 고정해 `calc(20px - 5%)`가 예상한 2-term signed shape인지 확인한다. Positive/negative sign, term order, non-finite value, extra/nested AST, raw authored sum canonicalization을 각각 대조한다.
- Layout tests는 column/row, prefix/suffix, positive final/negative final, border-box content basis, auto-fill/auto-fit, matching/missing/duplicate/wrong-node/wrong-property calc ID, definite/indefinite/cyclic basis, aggregate overflow를 포함한다. 음수 최종값은 해당 property range에 따라 0px로 clamp하고 부분 output을 내지 않는다.
- [고정 9-scene Chrome oracle](../spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/README.md)을 현재-source Stylo→layout→FFI 앱 경로에 연결한다. Chrome geometry는 track order/count/used size를 기준으로 1/64 CSS px 이내 비교한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 runtime, PixelCopy/Metal drawable 표본, 경계 negative controls를 각각 기록한다. AVD는 software renderer이고 iOS drawable은 compositor 제출 전으로 한정한다.
- 기본 product archive와 opt-in fixture-test archive를 비교해 fixture ID/symbol/payload가 제품 artifact에서 제외되고 fixture-test positive control에는 들어가는지 확인한다. 해당 archive 결과를 APK/IPA 전체 제외 근거로 확대하지 않는다.
- 구현/runtime 실패 관점 20개와 이 변경에 한정한 PR diff 실패 관점 30개를 plan review와 새로 작성한다. 결함이 발견되면 수정 후 해당 관점을 다시 확인한다.
- 같은 PR에서 내부 계약 [0059](../spec/internal/0059-c11-grid-layout.md)의 제한 profile, 오류·basis·clamp 동작과 `spec/STATUS.md`의 근거를 갱신한다. 계약 numeric version `0.1.0`을 유지하며 상위 C11.4 체크와 완료 집계 `72/213`은 그대로 둔다.

## 오류와 제외 범위

- Definite positive basis가 없거나 axis/property binding이 다르면 기존 fail-closed cascade/layout error와 부분 output 부재를 유지한다.
- 정확히 두 항이 아니거나 추가 math operator·nested `min()`/`max()`/`clamp()`/product/invert가 남은 AST는 지원하지 않는다. Stylo typed AST에서 실제로 canonicalize된 결과가 profile shape와 다르면 author syntax만 보고 억지로 수용하지 않는다.
- Nonfinite term/intermediate, profile의 finite·layout-representable 경계를 넘는 값, neighbor aggregate overflow, duplicate/missing calc ID, 다른 node/property 소유 값은 거부한다. 이 경계를 넘는 값까지 CSS range clamp가 구현됐다고 주장하지 않는다. 지원 값 안에서 negative final value의 0 clamp와 overflow rejection은 서로 다른 경계다.
- Intrinsic/flexible neighbor, multiple auto-repeat, zero/indefinite/cyclic basis, general author CSS, arbitrary CSS math, full WPT, 실기기/hardware GPU, iOS compositor 후 surface와 전체 C11.4/CSS Grid 적합성은 이 slice에 포함하지 않는다.

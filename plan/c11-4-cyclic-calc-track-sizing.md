# C11.4 · cyclic percentage가 포함된 `calc()` track sizing 계획

**상위 원본:** [C11.4 track sizing 계획](./c11-4-grid-track-sizing.md) · [Grid 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [CSS Grid 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)

**상태:** Chrome 156 사전 비교와 독립 기대값을 네 fixed scene으로 고정했고, 계획 실패 관점 검토 20개와 구현/runtime 검토 20개를 마쳤다. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene과 overflow surface 표본을 확인했다. 이는 고정 fixture 결과다. C11.4 공식 checkbox와 완료 수 `72/213`은 유지한다.

## 목적과 제한 범위

기존 C11.4 cyclic percentage fixture는 bare percentage, percentage `minmax()`, `fit-content(percentage)`, 두 축 다중 percentage track을 확인했다. 이번 slice는 현재 typed CSS math 경로에 이미 보존되는 `calc(percentage + length)` track을 같은 intrinsic/final-size 경계에 연결한다. 일반 CSS author admission이나 새 공개 API를 추가하지 않는다.

허용 장면은 HTML `div` root와 단일 `div` item, 두 고정 `calc()` expression, `max-content` intrinsic sizing, 네 scene ID뿐이다. 등록되지 않은 scene·DOM shape·style input은 기존 fail-closed gate를 유지한다.

## 입력과 독립 기대값

사전 비교 입력은 [`precomparison evidence`](../spec/internal/evidence/c11-4-cyclic-calc-track-sizing-precomparison-2026-10-22/README.md)와 [Chrome fixture](../tests/fixtures/css/c11/grid-cyclic-calc-tracks-c114-precomparison.html)에 고정했다. item은 `24×16px`, column은 `calc(50% + 4px)`, row는 `calc(50% + 2px)`다.

CSS Grid 2025 CRD §7.2.1·§11.1에 따라 track percentage는 cyclic container intrinsic size 산정 때 `auto`로 취급하고, 산출된 inner size로 최종 layout에서 resolve한다. 독립 기대값은 cyclic column `16px`, cyclic row `10px`, simultaneous axes `16×10px`, definite `40×20px` control `24×12px`다. 네 장면 모두 Chrome 156의 used track, root/item frame과 일치한다.

| Scene | Root | Columns | Rows | Item |
| --- | ---: | ---: | ---: | ---: |
| `cyclic-calc-column` | 24×16 | 16 | 16 | 24×16 |
| `cyclic-calc-row` | 24×16 | 24 | 10 | 24×16 |
| `cyclic-calc-two-axis` | 24×16 | 16 | 10 | 24×16 |
| `definite-calc-control` | 40×20 | 24 | 12 | 24×16 |

## 구현과 검증 관문

1. 기존 C11.4 typed runtime profile에서만 네 scene ID와 고정 style/DOM shape를 등록한다. CSS cascade의 `ComputedCssMath`부터 layout `CalcId`, Taffy Grid track까지 `grid-template-columns`/`rows` 값이 보존되는지 확인한다.
2. 독립 Rust style-to-layout/runtime test에서 final track, root/item frame, calc percentage basis와 축별 trace를 검사한다. unsupported expression, 다른 DOM shape, 누락 ID는 실패해야 한다.
3. 기본 artifact와 opt-in fixture-test positive control을 비교해 신규 scene route/payload가 기본 제품 artifact에 들어가지 않는지 확인한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS Simulator에서 네 scene 모두 layout·draw/present를 확인한다. cyclic item overflow가 track 경계를 넘어 칠해지는 표면 좌표를 Android PixelCopy와 iOS Metal drawable에서 각각 exact 대조한다. 실기기는 연결하지 않는다.
5. 계획 검토와 다른 실패 관점 20개로 구현/runtime을 재검토한다. PR 변경은 별도 적대 검토로 저장소 규칙상 30개 관점을 검토한다.

## 범위 밖과 판정

`minmax(calc())`, `fit-content(calc())`, 복수 `calc()` track, nested Grid, border/gap 변형, WPT, CSS author support 및 전체 C11.4는 범위 밖이다. 이 fixed fixture의 통과만으로 일반 `calc()` track sizing 또는 C11.4 전체를 완료 처리하지 않는다. 구현 변경에서는 C11.4 checkbox와 공식 완료 수를 그대로 둔다.

## 검토와 근거

- [계획 실패 관점 검토 20개](./c11-4-cyclic-calc-track-sizing-plan-review-20.md)
- [사전 비교와 독립 기대값](../spec/internal/evidence/c11-4-cyclic-calc-track-sizing-precomparison-2026-10-22/README.md)
- [구현·Simulator·artifact 결과](../spec/internal/evidence/c11-4-cyclic-calc-track-sizing-2026-10-22/README.md)
- [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-4-cyclic-calc-track-sizing-2026-10-22/implementation-review-20.md)
- [PR diff 실패 관점 검토 30개](../spec/internal/evidence/c11-4-cyclic-calc-track-sizing-2026-10-22/pr-diff-review-30.md)
- [기본·opt-in artifact 경계](../spec/internal/evidence/c11-4-cyclic-calc-track-sizing-2026-10-22/artifact-boundary.json)

## 구현 결과 (2026-10-22)

네 scene을 기존 opt-in C11.4 CSS→Taffy→native fixture 경로에 연결했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 `4/4` scene의 root/item frame, 양 축 track trace, draw/present와 PixelCopy·Metal drawable sample `8/8` exact를 확인했다. 공통 `-two-axis` suffix 처리에서 발견한 iOS 기대 frame 결함을 `nested-depth-*` ID로 한정해 수정하고, 기존 nested-depth min/max-content two-axis 회귀 여섯 scene을 두 플랫폼에서 각각 `6/6` 재실행했다. 기본 release `.rlib`에는 fixture symbol/scene string이 없고 fixture-test positive control에는 포함된다. 수정 전 harness 실패와 로그는 실행 evidence에 남겼다.

검증하지 않은 범위는 일반 CSS author admission, `minmax(calc())`, `fit-content(calc())`, 복수 calc track, nested Grid, WPT, Android 실기기, hardware GPU, iOS compositor 이후 pixel, 전체 C11.4다.

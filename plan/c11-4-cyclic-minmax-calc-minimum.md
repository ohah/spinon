# C11.4 · cyclic percentage `minmax(calc())` 최솟값 track 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [직전 `minmax(calc())` 최댓값 계획](./c11-4-cyclic-minmax-calc.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 네 fixed scene의 opt-in runtime 구현, Rust 회귀, Android API 37 AVD와 iOS 26.2 Simulator 검증을 마쳤다. 각 플랫폼에서 4/4 scene과 20/20 표면 표본이 통과했다. 고정 fixture 범위만 다루며 C11.4 공식 checkbox와 완료 수 `72/213`은 유지한다. 구현 근거는 [runtime evidence](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-runtime-2026-10-22/README.md)에 기록했다.

## 목적과 범위

직전 slice는 `minmax(0px, calc(50% + length))`의 최댓값 함수를 확인했다. 이번 계획은 cyclic percentage가 들어간 `minmax()`의 **최솟값 함수**가 intrinsic container sizing 단계와 최종 track sizing 단계에서 올바르게 분리되는지 확인한다. `minmax(calc(200% + 4px), max-content)` 열 scene과 `minmax(calc(200% + 2px), max-content)` 행 scene을 cyclic 조건과 definite-size control에서 각각 실행한다.

최솟값을 `200% + length`로 잡아 최종 값이 Ahem 한 글자의 `max-content` 기여도 10px보다 커지게 한다. 따라서 `calc()` 최솟값이 누락되거나 최댓값에 잘못 전달되면 expected track 24px/22px과 달라진다. 각 root의 content box는 10px이고 2px solid border가 있어 outer border box는 14px이다. percentage basis는 border-box가 아니라 content box다.

고정 입력은 HTML `div` Grid root와 direct text item 한 개, Ahem `10px/10px`, 한 explicit track, 한 글자 `a`, gap 0, padding 0이다. 열/행 두 cyclic scene, 열/행 definite control 네 장면만 opt-in C11.4 fixture에 등록한다. 일반 CSS author admission, 여러 item·track, nested Grid, 다른 calc 부호·계수, WPT, 내부 계약 버전 변경은 포함하지 않는다.

## 규범 기준과 독립 기대값

입력은 [사전 비교 fixture](../tests/fixtures/css/c11/grid-cyclic-minmax-calc-minimum-c114-precomparison.html)에 고정했다. CSS Grid Layout Module Level 1 2025 CRD [§7.2.1 Track Sizes](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)는 `minmax(min,max)`의 최댓값이 최솟값보다 작으면 최댓값을 최솟값까지 올린다고 정의한다. [§10.6 Grid Layout Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing)은 cyclic track percentage를 grid container 크기 산정 시 `auto`처럼 취급하고, 그 크기를 구한 뒤 track percentage를 다시 resolve한다.

독립 계산은 규범 순서와 Ahem 기여도에서 도출했다. root content size 10px은 한 글자의 intrinsic contribution이다. cyclic 단계에서 percentage minimum을 auto처럼 취급하면 intrinsic root 크기는 10px로 유지된다. 최종 sizing에서 `calc(200% + length)`는 content box 10px을 basis로 해 column `24px`, row `22px`이 된다. `max-content` 최대 10px은 최솟값보다 작으므로 규범상 각각 24px·22px으로 올라간다. border 2px씩을 더한 root border box는 `14×14px`이다.

| 장면 | Root content sizing | Track | 독립 기대 Columns / Rows | Root border box | Item, root-relative |
| --- | --- | --- | --- | --- | --- |
| `cyclic-minmax-calc-min-column` | width `max-content` | Columns `minmax(calc(200% + 4px), max-content)` | `[24]` / `[10]` | `14×14` | `(2,2) 24×10` |
| `cyclic-minmax-calc-min-row` | height `max-content` | Rows `minmax(calc(200% + 2px), max-content)` | `[10]` / `[22]` | `14×14` | `(2,2) 10×22` |
| `definite-minmax-calc-min-column` | width `10px` | Columns `minmax(calc(200% + 4px), max-content)` | `[24]` / `[10]` | `14×14` | `(2,2) 24×10` |
| `definite-minmax-calc-min-row` | height `10px` | Rows `minmax(calc(200% + 2px), max-content)` | `[10]` / `[22]` | `14×14` | `(2,2) 10×22` |

전체 독립 계산과 브라우저 관측은 [JSON](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/spec-derived-expectations.json), 입력·스크린샷·색상 표본은 [사전 비교 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/README.md)에 둔다.

## 검증 관문

1. 계획 자체를 기존 계획 검토와 다른 20개 실패 관점으로 검토하고 입력, 독립 계산, border/overflow 판정과 한계를 반영한다.
2. style projection부터 typed `GridTrackMinSizingFunction::Calc` 또는 동등한 `LayoutCalcId`까지 minimum calc의 percentage와 length가 보존되는지 확인한다. direct layout regression은 cyclic percentage basis, final minimum/maximum, 두 축 track, root/item frame과 stage trace를 비교한다.
3. 네 ID·DOM shape·style만 허용하는 opt-in runtime adapter를 추가한다. 변형된 expression, node 순서/수, ID, CSS math handle의 owner/axis가 다른 입력은 부분 layout 없이 거부한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 네 장면의 track trace, root/item frame, draw/present를 확인한다. 기본 product archive에는 신규 fixture symbol·ID·payload가 없고 fixture-test positive control에는 있는지 따로 검사한다.
5. CSS root의 `2px solid #f0c674`와 Grid item background는 실제 runtime CSS/layout/paint 경로에서 생성한다. 테스트 전용 overlay로 border를 그리지 않는다. Android PixelCopy와 iOS compositor 제출 전 Metal drawable에서 border 안쪽 control, track/item 내부, border가 item overflow에 덮이는 점, border 바깥이지만 item 안인 점, item 바깥 점을 각각 기록한다. item이 root의 오른쪽/아래 경계를 넘는 화면을 simulator 캡처로 보존한다.
6. 계획 검토와 별도의 구현/runtime 실패 관점 20개를 기록하고, PR diff는 별도 30개 실패 관점으로 검토한다. 발견한 결함은 수정 후 해당 Rust·artifact·simulator·surface 표본을 다시 실행한다.
7. PR은 고정 fixture 구현임을 설명하고 C11.4 완료로 승격하지 않는다. live required-check 및 merge 상태를 확인하고 Rebase and merge를 사용한다. PR에 넣는 simulator 화면은 Agent Browser로 로드 여부를 확인한 뒤 해당 Agent Browser session을 종료한다.

## 구현 결과

Style projection은 min-side calc를 axis-scoped `LayoutCalcId`로 보존하고, 기존 `Calc`→Taffy `GridTrackMinSizingFunction::Calc` 연결을 네 장면의 regression으로 고정했다. 별도 runtime scene matrix는 root content-box sizing·2px border·exact track formula·직접 Ahem text를 검사한다. Android PixelCopy와 iOS Metal drawable은 border control, item 내부, border overpaint, root 밖 item overflow와 item 바깥을 각 scene에서 확인했다.

첫 Android 실행에서 FFI/native allowlist와 별개인 runtime scene registry의 누락을 재현해 보정했고, 실패 로그를 evidence에 보존했다. 이후 최신 source diff hash 기준으로 두 플랫폼 앱을 다시 빌드·캡처했다. 구현 실패 관점 20개는 [별도 검토](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-runtime-2026-10-22/implementation-review-20.md)에서 다뤘다. PR diff 검토, 병합 상태와 Tailnet 로드맵 동기화는 PR 진행과 함께 기록한다.

## 범위 밖과 판정

다중 track/item, span, nested·anonymous·replaced item, negative calc term, `minmax()` 바깥의 calc 함수, gap·writing-mode 변형, auto-repeat, 일반 CSS admission, WPT, Android 실기기, hardware GPU, iOS compositor 이후 pixel 및 전체 C11.4는 다루지 않는다. 네 fixed scene의 성공만으로 전체 CSS Grid나 C11.4를 지원한다고 표시하지 않는다. C11.4 checkbox와 공식 `72/213`은 유지한다.

## 근거와 검토

- [Chrome 156 사전 비교·border/overflow 표본](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/README.md)
- [독립 기대값과 Chrome 관측](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/spec-derived-expectations.json)
- [계획 실패 관점 검토 20개](./c11-4-cyclic-minmax-calc-minimum-plan-review-20.md)
- [계획 PR diff 검토 30개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/plan-pr-diff-review-30.md)
- [구현/runtime 근거와 실패 관점 검토 20개](../spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-runtime-2026-10-22/README.md)
- [CSS Grid 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes) · [§10.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing)

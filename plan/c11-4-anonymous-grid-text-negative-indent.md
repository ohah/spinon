# C11.4 anonymous Grid text negative `text-indent` 제한 계획

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [직전 nonnegative `text-indent` slice](./c11-4-anonymous-grid-text-computed-text-indent.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 12개 음수 장면 구현·simulator 검증·구현 diff 30관점 검토를 마쳤다. 이전 구현 PR #930의 18개 `0px`/`5px`/`10px` scene과 합쳐 30개 matrix를 확인했으며 구현 PR 생성 전 상태다. 이 제한 opt-in slice는 C11.4/C11 완료나 일반 CSS 지원을 뜻하지 않는다.

**사전 비교:** [CSS Text 3·CSS Sizing·Grid 규범 및 독립 기대값, Chrome 156 Agent Browser 관측](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-precomparison-2026-11-01/README.md) · [Chrome 156 observation JSON](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-precomparison-2026-11-01/chrome156-observation.json) · [화면](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-precomparison-2026-11-01/chrome156-agent-browser.png)

**계획 검토:** [서로 다른 실패 관점 20개](./c11-4-anonymous-grid-text-negative-indent-plan-review-20.md). 이 20개 검토는 계획 PR diff 검토와 분리한다.

**계획 PR diff 검토:** [서로 다른 실패 관점 30개](./c11-4-anonymous-grid-text-negative-indent-plan-pr-diff-review-30.md).

**구현/runtime 검토:** [새 실패 관점 20개](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-runtime-2026-11-01/implementation-runtime-review-20.md). 계획 검토와 별도다.

**구현 PR diff 검토:** [서로 다른 실패 관점 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-runtime-2026-11-01/pr-diff-review-30.md). Android max-content line advance 기대값 결함을 수정한 뒤 static source test·fixture build·Android 30-scene capture를 다시 실행했다.

## 목표와 고정 범위

기존 `RuntimeBlockFlexGridTrackSizingV1` opt-in anonymous direct-text fixture의 computed `text-indent` allowlist에 `-5px`와 `-10px`만 추가한다. 고정 Ahem 10px/400, `line-height:10px`, `white-space:normal`, `horizontal-tb`, LTR, 한 direct anonymous text source, no edges/gap, 하나의 `min-content` 또는 `max-content` column을 사용한다.

새 scene은 세 텍스트 `XXX X`, `X XXX`, `XX`와 두 음수 값 `-5px`, `-10px`를 각 column function에 적용한 12개다. 기존 PR #930의 18개 `0px`/`5px`/`10px` scene은 전체 regression matrix에 남겨 합계 30개 scene을 검증한다. Validator는 scene ID·computed CSS value·typed `ParagraphStyle.text_indent`·source `HostDocument` revision을 모두 일치시킨다.

이번 profile은 arbitrary negative length를 허용하지 않는다. `-7px`, `-30px`, 미등록 scene/value, percentage, `each-line`, `hanging`, stale revision과 profile conflict는 layout publication 전에 거부한다. 특히 `-30px`은 Chrome oracle-only 경계로 남긴다. `X XXX`의 Chrome `min-content`와 `max-content` Grid track은 모두 30px인데, max-content의 단순 `50 + d` advance는 20px이어서 track base floor와 `TextIntrinsicInlineSizes`의 min≤max 불변 조건을 함께 다뤄야 한다. 이 계획은 그 engine 의미 변경을 포함하지 않는다.

## 규범과 독립 기대값

[CSS Text Level 3 §8.1](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#text-indent-property)의 `<length>` 값은 음수가 될 수 있으며 indent는 첫 formatted line의 start edge에 적용된다. Percentage는 intrinsic size 계산에서 0으로 취급되지만 이 fixture에서는 계속 거부한다. `hanging`과 `each-line`은 대상 line set을 바꾸므로 제외한다. [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes) 및 [CSS Grid Level 1 §11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing) 절차와 고정 Ahem glyph metrics로 기대값을 Chrome 관측과 독립 계산했다.

고정 Ahem 10px에서 `X`와 U+0020 space advance는 각각 10px이다. `d`는 -5 또는 -10이다.

- `XXX X`: min-content `max(30 + d, 10)`, max-content `50 + d` → `-5px`: 25/45px, `-10px`: 20/40px.
- `X XXX`: min-content `max(10 + d, 30)`, max-content `50 + d` → `-5px`: 30/45px, `-10px`: 30/40px.
- `XX`: 단어 분리점이 없어 min/max 모두 `20 + d` → `-5px`: 15px, `-10px`: 10px.

두 단어 입력의 min-content row는 두 줄/20px, max-content row는 한 줄/10px이다. 한 단어는 min/max 모두 한 줄/10px이다. 전체 expected matrix와 `-30px` observation-only boundary는 [expected JSON](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-precomparison-2026-11-01/expected.json)에 고정했다.

## 구현 및 검증 순서

1. 이 계획을 구현 전에 독립 실패 관점 20개로 검토한다. 계획 PR diff에는 별도의 PR 규칙 검토 30개를 수행한다. 두 문서는 한 검토를 다른 횟수로 재사용하지 않는다.
2. `computed_text_indent.rs`의 fixture scene allowlist에 고정 12개 negative scene을 추가한다. 제품 runtime이나 기본 CSS admission은 바꾸지 않는다.
3. 기존 computed `text-indent` projection을 재사용하되 같은 root/style snapshot의 exact computed negative px가 expected scene의 값과 일치하는지 검사한다. `ParagraphStyle`은 이미 finite `text_indent`를 받고 Parley layout에 넘기므로 negative 입력의 intrinsic 결과가 독립 기대값과 일치하지 않으면 allowlist를 열기 전에 원인을 조사하고 계획을 수정한다.
4. Rust expected matrix, computed CSS text, scene/source identity, revision, `ParagraphStyle.text_indent`, min/max contribution, line count/advance, item frame, Grid track과 draw/present를 12개 신규 scene에 대해 strict 비교한다. #930의 양수 18-scene regression도 함께 통과시킨다.
5. Negative controls에서 `-7px`, `-30px`, percentage, `hanging`/`each-line`, unlisted scene, computed/typed mismatch와 stale revision을 각각 거부하고 실패 시 부분 frame/trace가 없음을 확인한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최종 소스 30-scene matrix를 실행한다. AVD software-renderer와 iOS drawable의 compositor 전 경계를 그대로 기록하고 glyph pixel 또는 실기기 근거로 확대 해석하지 않는다.
7. `spinon-ffi` 기본 release archive와 fixture-test positive control을 비교해 fixture-only scene/symbol/payload가 기본 archive에 빠지고 positive control에 들어가는지 확인한다. 이 결과를 APK/IPA 전체 bundle exclusion이라고 부르지 않는다.
8. 관련 `spinon-text-layout`, `spinon-style-to-layout`, `spinon-runtime`, `spinon-ffi` focused tests, fixture boundary, fmt, scoped Clippy 및 Android/iOS Simulator app builds를 실행한다. 미실행 gate는 통과로 쓰지 않는다.
9. 구현 후 새 코드/runtime 실패 관점 20개와 실제 PR diff 실패 관점 30개를 별도로 검토하고, 결함 수정 뒤 영향받은 검토를 반복한다.
10. 구현 PR 본문에는 Chrome 기준 화면과 가능하면 Android/iOS 화면 PNG를 연결하고 image link를 확인한다. PR check이 보고되지 않으면 CI 통과로 쓰지 않으며, user-authorized Rebase and merge 뒤 최종 SHA를 기록한다.
11. 병합 후 내부 계약 0059, `spec/STATUS.md`, C11.4 부모 계획에 고정 subset·근거·제한을 기록한다. C11.4/C11 checkboxes 및 공식 `72/213`은 그대로 둔다.

## 완료 조건과 한계

- 12개 신규 fixture scene의 browser values, independent Ahem calculation, typed text indent, intrinsic sizes, line metrics, frames와 Grid tracks가 일치해야 한다.
- 기존 positive 18-scene regression을 유지하고 모든 unlisted/unsupported inputs를 fail-closed한다.
- Android AVD와 iOS Simulator 전체 30 scene, default/opt-in artifact boundary, focused Rust checks, implementation review 20개와 PR diff review 30개가 각각 통과해야 한다.
- 이 slice는 임의 음수값, `-30px` boundary, percentage, `hanging`/`each-line`, 일반 CSS admission, arbitrary fonts/scripts, glyph paint, WPT, 실기기/hardware GPU, 전체 C11.4 또는 C11을 지원/완료로 만들지 않는다.

## 구현 후보 결과

Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 양수 18개와 새 음수 12개가 각각 30/30 통과했다. Rust focused tests, text-layout tests, fixture module boundary Node tests, formatter, runtime/FFI scoped Clippy 및 두 platform app build도 통과했다. 실행 증거와 상세 한계는 [implementation evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-runtime-2026-11-01/README.md)에 기록한다. 공식 C11.4/C11 checkbox와 완료 수 `72/213`은 유지한다.

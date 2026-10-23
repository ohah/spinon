# C11.4 · zero-length percentage `calc()` auto-repeat minimum 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** [독립 규범 계산](../spec/internal/evidence/c11-4-auto-repeat-zero-percent-calc-precomparison-2026-10-23/spec-derived-expectations.json), Chrome 156의 8개 비교 장면, [계획 실패 관점 20개](./c11-4-auto-repeat-zero-percent-calc-plan-review-20.md), [계획 PR 변경 검토 20개](../spec/internal/evidence/c11-4-auto-repeat-zero-percent-calc-precomparison-2026-10-23/plan-pr-diff-review-20.md)를 기록했다. 아직 구현/runtime 결과가 아니며 C11.4를 완료 처리하지 않는다.

## 결손과 목표

직전 [definite percentage `calc()` minimum slice](./c11-4-auto-repeat-percentage-calc-minimum.md)는 computed sum의 length 항이 양수여야 한다. 따라서 `calc(0px + 10%)`가 definite positive basis에서 12px로 resolve되는 경우도 cascade가 거부한다. CSS Grid 2025 CRD §7.2.1의 `<length-percentage [0,∞]>`는 음이 아닌 값을 허용하며, §7.2.3.1의 `<fixed-size>`가 `minmax()` minimum으로 사용할 수 있다. §7.2.3.2는 definite preferred/max axis에서 content box와 gap을 사용해 가능한 가장 큰 양의 반복 횟수를 정하고, auto-fit은 placement 뒤 비어 있는 반복 track을 접는다.

목표는 기존 C11.4 opt-in `RuntimeBlockFlexGridTrackSizingV1`에 한해 `minmax(calc(0px + 10%),1fr)`의 definite 120px columns/rows auto-fill/auto-fit을 연결한다. CSS Grid CRD와 Chrome 156의 사전 비교 및 독립 계산은 [precomparison evidence](../spec/internal/evidence/c11-4-auto-repeat-zero-percent-calc-precomparison-2026-10-23/README.md)다.

## 고정 판정과 범위

120px content inline/block size에서 minimum은 `0 + 0.10×120 = 12px`이고 repeat count는 `floor(120/12)=10`이다. auto-fill의 expected tracks는 `[12]×10`; auto-fit은 item placement 후 `[120,0×9]`이며 단일 item도 120px이다. 두 축 모두 numeric `12px` control과 같아야 한다. Chrome의 원본 값은 [observation JSON](../spec/internal/evidence/c11-4-auto-repeat-zero-percent-calc-precomparison-2026-10-23/chrome-observation.json)에 보존했다.

구현은 정확히 두 항인 finite `LengthPx(0) + Percentage(>0)` calc minimum을 제한적으로 허용하고 Stylo의 실제 computed AST를 검사한다. fixture cascade test가 `Calc(Sum(LengthPx(0), Percentage(>0)))`를 보존하는지 먼저 확인한다. Stylo가 zero length 항을 제거해 bare `Percentage`로 정규화하면 직접 percentage admission이 필요한지 별도 계획을 검토하기 전까지 구현을 멈춘다. length 항이 양수인 기존 subset은 회귀 없이 유지한다. zero-size definite basis에서 Taffy auto-repeat 분모가 0이 되지 않도록, 해당 zero-length 형태는 명시적 양수 fixed axis와 zero padding/border에서만 layout에 전달하고 그 외에는 typed error로 fail-closed 한다. vendored Taffy의 [repeat-count 계산](../vendor/taffy/src/compute/grid/explicit_grid.rs)에서 0 분모는 큰 repetition 값이 되고 [track 상한](../vendor/taffy/src/compute/grid/mod.rs)으로 잘린다. 이 동작을 CSS의 zero-size floor 의미로 간주하지 않는다. `calc(0px + 0%)`, 음수/추가 calc 항, 다른 함수, cyclic/indefinite basis, 양수 외부 border/padding, nonzero gap, 여러 auto-repeat, areas/named-line/implicit-placement 결합과 일반 CSS admission은 제외한다.

## 구현과 검증 단계

1. cascade unit test에서 columns/rows와 auto-fill/auto-fit의 zero-length calc computed value를 확인하고, `0%`, 음수, 추가 항과 `max()`는 거부한다. 계산 basis zero와 indefinite는 layout 진입 전에 실패하는지 별도 negative case로 확인한다.
2. 기존 auto-repeat calc fixture catalog에 calc/control 각 4 scene을 더한다. 기대값은 10개 12px track 또는 auto-fit `[120,0×9]`, item/root frame, track trace와 collapse 상태다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 실제 V8-authored HTML → HostDocument → Stylo → typed layout input → Taffy → WGPU scene 경로에서 8개를 실행한다. screenshots는 box layout 보조 근거이며, PixelCopy/Metal pixel equality를 검사하지 않으면 주장하지 않는다.
4. 해당 Rust focused tests, fixture catalog/scene validator, `cargo fmt --all -- --check`, workspace Clippy `-D warnings`, Android/iOS simulator build를 기록한다. default와 opt-in static FFI archive marker 경계를 대조한다. APK/IPA 전체 exclusion은 별도 검사 없이는 주장하지 않는다.
5. 계획과 겹치지 않는 구현/runtime 실패 관점 20개 및 PR diff 실패 관점 20개를 각각 작성하고, 문제를 수정하면 영향받은 관점을 다시 판정한다. PR에 가능한 simulator 화면을 첨부하고 GitHub 표시를 Agent Browser로 확인한 뒤 그 브라우저 session/process를 종료한다.
6. Rebase and merge 후 실제 SHA와 check 상태를 다시 조회해 공식 `spec/STATUS.md`의 근거를 갱신한다. Tailnet preview 파일을 hash manifest와 함께 동기화하고 HTTP/Agent Browser에서 최신 화면을 확인한다.

전체 CSS Grid, 공개 CSS 지원, WPT conformance, Android 실기기, hardware GPU와 iOS compositor 이후 pixels는 이 계획에서 판정하지 않는다. 내부 계약 `0059`는 `0.1.0`으로 유지하며 C11.4 checkbox와 공식 완료 수 `72/213`은 바꾸지 않는다.

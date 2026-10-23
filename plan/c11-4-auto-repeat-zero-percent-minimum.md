# C11.4 · zero-percent auto-repeat minimum 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [기존 definite positive percentage minimum slice](./c11-4-auto-repeat-positive-percent-minimum.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 제한 구현과 Android/iOS Simulator runtime 13/13 검증을 마쳤다. Stylo cascade와 layout preflight가 finite `Percentage(0)`을 기존 gate 안에서 허용하고, vendored Taffy는 repeat count에만 track별 UA 1px floor를 적용한다. 실제 zero minimum과 auto-fit collapse는 유지한다. 구현/runtime 적대 검토는 [20개 관점](../spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/implementation-runtime-review-20.md)에 별도 기록했으며 계획 검토를 재사용하지 않았다. PR diff 검토와 병합은 후속 관문이며 C11.4 전체는 미완료다.

## 결손과 목표

직전 [definite positive percentage minimum slice](./c11-4-auto-repeat-positive-percent-minimum.md)는 computed percentage minimum이 양수일 때만 opt-in auto-repeat에 전달한다. 이에 따라 `repeat(auto-fill, minmax(0%, 1fr))`와 `auto-fit`은 현재 fail-closed다. CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.2는 repeat count를 정할 때 division by zero를 피하도록 각 repeated track의 count용 size를 UA 지정 값으로 floor하도록 요구하고 1px를 제안한다. 이 floor는 repeat count에만 사용한다. 실제 zero percentage minimum과 auto-fit collapse는 0px 그대로여야 한다. [W3C §7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-syntax) · [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/README.md).

목표는 기존 `RuntimeBlockFlexGridTrackSizingV1` opt-in profile에서 finite computed `Percentage(0)` minimum을 definite positive content axis의 auto-repeat fragment에 연결하고, Taffy의 repeat-count 계산이 repeated track당 1px UA floor를 적용하게 하는 것이다. 반복 track의 실제 sizing function, prefix/suffix track, gutter, item layout 또는 public CSS admission을 floor로 바꾸지 않는다.

## 규범 판정과 고정 모델

- Authored minimum은 `0%`이며 Stylo computed value가 typed `Percentage(0.0)`인지 cascade와 layout boundary에서 확인한다. Stylo가 `calc(0px + 0%)`를 같은 typed `Percentage(0.0)`으로 정규화하면 동일한 경로로 처리한다. source CSS 재파싱이나 새 math evaluator를 추가하지 않으며 다른 calc 표현은 기존 gate를 따른다.
- 120px definite content axis의 `minmax(0%, 1fr)`는 gap 0에서 120회, gap 4px에서 24회 반복한다. percentage basis는 columns의 inner inline size, rows의 inner block size다.
- Repeat-count floor는 각 반복 track contribution을 계산할 때 1px 이상으로 만든다. definite max sizing function이 선택되는 조건은 유지하고, 해당 selected track size에 floor를 적용한다. track sizing / initialize / flex distribution에 floor 값을 전달하지 않는다.
- gaps는 track floor와 별도로 count formula에 포함한다. repeated fragment 밖의 explicit 0.5px prefix/suffix는 floor하지 않는다. border-box axis는 padding/border를 뺀 content size를 basis로 사용한다.
- `auto-fit`은 auto-fill과 같은 반복 수를 계산하고 item placement 이후 빈 repeated track과 양옆 gutter를 collapse한다. 빈 track의 0px sizing function을 1px로 바꾸지 않는다.
- Product preflight는 양수 definite content axis, min-size 0, max-size auto와 기존 supported maximum 조건을 요구한다. zero content basis, indefinite/cyclic basis, non-finite percentage, negative percentage, typed zero로 정규화되지 않는 추가 track/calc grammar와 axis overflow는 계속 typed error로 fail-closed 한다.

## 13-scene 사전 비교

판정 표본과 Chrome 관측은 [사전 비교 기록](../spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-precomparison-2026-10-24/README.md)에 고정했다. 런타임 fixture는 이 13개의 독립 경로를 그대로 사용한다.

| 장면 | 축·입력 | 기대 repeat 수 | 기대 used result |
|---|---|---:|---|
| `fill-col-zero-gap` | columns 120px, `0%`/`1fr`, `auto-fill`, gap 0 | 120 | flex 사용값 1px × 120 |
| `fill-col-zero-px-gap` | 동일, `0px` control | 120 | `0%` 장면과 동일 |
| `fill-col-gap-four` | columns 120px, `0%`/`1fr`, gap 4px | 24 | 독립 flex 사용값 28/24px |
| `fill-col-half-px-gap-zero` | `0.5px` minimum control, gap 0 | 120 | count floor는 1px, flex 사용값 1px |
| `fill-col-half-px-gap-four` | `0.5px` minimum control, gap 4px | 24 | 독립 flex 사용값 28/24px |
| `fill-col-half-percent-gap-four` | `0.5%` minimum control, gap 4px | 24 | minimum 0.6px, count floor 1px, 28/24px used |
| `fill-col-zero-neighbors-subpixel` | 0.5px explicit prefix/suffix, repeated `0%`, gap 4px | 23 | 2 explicit neighbor + 23 repeat = 25 tracks |
| `fit-col-empty` | columns 120px, `0%`/`1fr`, `auto-fit`, item 없음 | 120 | placement 후 120개 모두 0px collapse |
| `fit-col-first` | 같은 `auto-fit`, item 하나 | 120 | 첫 track 120px, 나머지 119개 0px |
| `fit-col-span-two` | 같은 `auto-fit`, item이 2 track span, gap 4px | 24 | 점유 track 58px × 2, 빈 22개 collapse |
| `fill-row-gap-four` | rows 120px, `0%`/`1fr`, gap 4px | 24 | columns와 같은 block-axis 계산 |
| `fit-row-first` | rows 120px, `auto-fit`, item 하나 | 120 | 첫 row 120px, 빈 119개 collapse |
| `border-box-col` | 136px border-box, 좌우 padding 8px·border 2px 각각, gap 4px | 24 | content basis 116px, 1px floor + gutter가 정확히 116px |

## 구현·검증 관문

1. `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs`와 `crates/spinon-layout/src/grid.rs`에서 `Percentage(0)`을 허용하되 finite·nonnegative check, definite positive content axis, supported maximum, min-size 0/max-size auto gate를 유지한다. negative/non-finite, zero/indefinite/cyclic basis는 거부한다.
2. `vendor/taffy/src/compute/grid/explicit_grid.rs`에서 auto-repeat count에 사용되는 각 repeated track의 definite selected size만 UA floor 1px로 올린다. fixed prefix/suffix는 floor하지 않고 gap 계산은 별도로 유지한다. 이 값을 실제 track sizing, flex expansion, auto-fit collapse에 전달하지 않는다.
3. Taffy unit tests는 zero minimum/gap 0 → 120, zero minimum/gap 4 → 24, 120px·gap 0의 `minmax(0%, 1fr) 0.5px` two-track group → 60 repetitions(각 repeated track을 개별 floor), explicit subpixel neighbors, positive maximum selection, auto-fit pre-placement count와 placement 후 collapse를 검사한다. 0 denominator 및 NaN/Infinity clamp 경로가 다시 생기지 않아야 한다.
4. layout/preflight tests는 13 scene의 columns·rows content basis, repeat count, used track histogram, item size, collapse/trace를 독립 기대값과 대조한다. count·histogram·collapse는 exact, fraction track/frame은 1/64 CSS px 이내다. zero/indefinite basis와 invalid percentage controls는 기존 typed error로 거부한다.
5. opt-in C11.4 runtime fixture를 V8-authored HTML → HostDocument → Stylo → typed layout input → Taffy → WGPU scene/draw/present 경로에 연결한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 13/13 scene을 실행한다. 화면은 scene/frame 근거이며, 별도 readback 없는 pixel equality나 hardware GPU 결과를 주장하지 않는다.
6. default/product FFI artifact와 opt-in fixture-test positive control에서 scene marker, payload, host symbol 경계를 비교한다. 정적 archive 결과를 APK/IPA 전체 fixture 제외로 확대하지 않는다.
7. 내부 계약 [0059](../spec/internal/0059-c11-grid-layout.md)에 이 제한 slice의 `0%` admission gate, count-only 1px floor, unchanged actual sizing, auto-fit collapse 및 fail-closed 입력을 기록한다. 계약 숫자 버전 `0.1.0`은 유지한다.
8. 관련 Rust/FFI tests, fixture/catalog validator, `cargo fmt --all -- --check`, workspace Clippy `-D warnings`, Android/iOS Simulator app builds를 수행해 기록한다. 구현 전 비교와 겹치지 않는 새 구현/runtime 실패 관점 20개를 검토한다.
9. 구현 PR의 변경 범위를 대상으로 구현/runtime 20개 검토와 겹치지 않는 PR diff 실패 관점 30개를 검토하고 발견 사항을 수정한 뒤 영향을 받은 관점을 다시 확인한다. [PR diff 검토](../spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/pr-diff-review-30.md)에 결과를 남긴다. simulator 화면을 PR 본문에 넣고 GitHub 표시를 Agent Browser로 확인한 뒤 해당 browser session/process만 종료한다.
10. Rebase and merge 후 live merge SHA/check state를 조회한다. `spec/STATUS.md`와 공개 roadmap에는 C11.4 미완료·72/213 유지와 제한 slice 결과를 반영하고, Tailnet preview에 선택한 문서만 backup + SHA manifest로 동기화한 뒤 HTTP 200/content hash/Agent Browser 화면을 확인한다.

전체 CSS Grid, 일반 authored CSS admission, WPT, 실기기, hardware GPU, iOS compositor 이후 pixels와 C11.4 완료는 범위 밖이다. 내부 계약 0059는 `0.1.0`, C11.4 체크와 공식 완료 수는 `72/213`으로 유지한다.

실행 결과·환경·제한·scene 및 static archive 경계는 [runtime 근거](../spec/internal/evidence/c11-4-auto-repeat-zero-percent-minimum-runtime-2026-10-24/README.md)에 연결했다. 이 제한 slice는 C11.4 부모 체크박스나 공식 완료 수를 변경하지 않는다.

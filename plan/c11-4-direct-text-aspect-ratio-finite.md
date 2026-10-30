# C11.4 direct-text finite-positive preferred aspect-ratio 확장 계획

**상태:** 구현 계획 PR 준비. 아직 runtime 변경이나 C11.4 완료 표시는 없다.

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [기존 direct-text ratio slice](./c11-4-direct-text-aspect-ratio.md) · [C11 Grid 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md)

## 문제와 목표

PR #834의 `RuntimeBlockFlexGridTrackSizingV1` direct-text collector는 `layout_aspect_ratio == 2.0`과 computed string `2 / 1`만 허용한다. Stylo는 nondegenerate CSS ratio를 finite-positive typed `f32`로 보존하고 `auto <ratio>`는 현재 Taffy mapping이 content-box semantics를 보존하지 못해 거부한다. Stylo bridge는 degenerate ratio를 `None`으로 투영하며, CSS Sizing Level 4 §4.1에 따라 그 값은 `auto`처럼 동작한다. 이 계획은 그 typed 값 제한을 제거해 같은 Ahem direct-text profile에서 **유한한 양수 preferred ratio**를 실제 intrinsic track sizing에 전달하되, computed property serialization과 typed snapshot이 모순되는 입력은 허용하지 않는다.

기준은 [CSS Grid Layout 2025 CRD §6.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing)와 [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing), [CSS Sizing Level 3 §2.2](https://www.w3.org/TR/css-sizing-3/#intrinsic-size-contributions), [CSS Sizing Level 4 2026-09-04 WD §4.1](https://www.w3.org/TR/2026/WD-css-sizing-4-20260904/#aspect-ratio)다. Sizing Level 4 문서는 Working Draft이며, 그 §4.1에서 ratio 방향은 width/height이고 degenerate ratio는 `auto`처럼 동작한다고 규정한다. 초안 상태와 Chrome 관측·Spinon 결과를 구별한다. Non-replaced text item은 Grid §6.6의 replaced-item transferred-size-suggestion shortcut에 넣지 않는다.

## 사전 기준

13개 고정 장면과 독립 계산 및 Agent Browser HeadlessChrome 156 관측은 [사전 비교](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-range-precomparison-2026-10-30/README.md)에 있다. 새 matrix는 `1/2`, `4/3`, `3/2`, `5/2` 네 ratio와 no-ratio controls만 포함하며 `2/1` scene은 포함하지 않는다. 기존 `2/1` 네 회귀 장면은 PR #834 구현의 별도 회귀 집합으로 유지한다. Row 방향은 item block contribution으로 auto row가 결정되고, column 방향은 definite row에서 inline max-content track과 item ratio box를 검사한다. `1/2` column scene은 text max-content track 40px과 ratio-preserving item 40×80px, 60px root row 아래 20px overflow를 고정한다.

## 제한된 입력 계약

- 기존 opt-in `c11-4-grid-track-sizing-fixture-test`의 direct-text ratio route를 사용한다. 일반 CSS admission이나 product profile은 열지 않는다.
- Root는 하나의 HTML Grid이며 정확히 두 in-flow block direct child를 가진다. 각 item은 의미 있는 direct Text node 하나만 소유하고 입력 글자는 ASCII `A–Z`와 U+0020뿐이다. Anonymous/nested text, replaced element, span, explicit placement, nonzero `order`, 추가 item은 계속 거부한다.
- Font는 저장소 Ahem 10px, line-height 10px, horizontal-tb/LTR, nowrap, overflow visible, zero margin/padding/border, auto width/height/min-size, no max-size다. Root는 100px definite inline size, `max-content max-content` columns, 0 gap, normal self alignment다.
- 최대 한 item에만 computed preferred ratio를 허용한다. Ratio는 Stylo snapshot의 typed `layout_aspect_ratio`에서 가져오며 `is_finite() && ratio > 0.0`이어야 한다. Author source, scene ID, fixture label은 ratio 값을 제공하지 않는다.
- `Some(ratio)`이면 computed `aspect-ratio` serialization도 nondegenerate 숫자 ratio여야 하며, 두 구성 요소로 정규화한 값이 typed `f32`와 일치해야 한다. `None`인 no-ratio item은 computed serialization이 `auto`여야 한다. 따라서 Stylo가 `None`으로 투영하는 degenerate numeric ratio는 이 fixture profile에서 no-ratio control로 오인하지 않고 거부한다. `auto <ratio>`는 cascade 단계 rejection을 유지한다.
- 둘 이상의 ratio item, ratio를 가진 `A` item, malformed 또는 revision 불일치 snapshot, unsupported style/structure는 layout 전에 fail closed한다. Ratio로부터 유도되는 모든 layout dimension도 유한하고 해당 `f32` layout domain에서 표현 가능해야 한다. 비정상적으로 작거나 큰 finite ratio가 derived geometry overflow를 만들면 Taffy layout을 시작하기 전에 거부한다.
- 반드시 보존할 regressions는 PR #834의 네 2/1 scene, no-ratio controls, legacy one-item fallback, 일반 C11.4 intrinsic-text route다.

## 구현과 판정 범위

1. `generic_direct_text.rs`의 positive ratio 검사와 paragraph style 경로에서 `2.0` 고정값을 없애고, typed finite-positive value의 실제 전파를 보장한다. Ratio를 `property string` 또는 scene ID에서 추론하지 않는다.
2. 13개 새 Chrome input scene과 기존 2/1 네 scene을 runtime fixture registry에 연결한다. 모든 case에서 computed input, ratio owner, Columns/Rows trace, root/item frame을 고정 기대값과 대조한다.
3. Positive matrix는 네 row ratio와 row control, 네 definite-row column ratio와 같은 높이의 네 no-ratio control이다. `column-1-2`의 track 40px·item 40×80px·root 100×60px overflow를 별도로 검사한다.
4. Adversarial tests는 ratio-less `auto` controls, degenerate numeric ratio가 `None`이 되어도 no-ratio로 오인되지 않는지, ratio를 `A`에 둔 case, 두 ratio item, `auto <ratio>`, unsupported shape, style revision mismatch, non-finite/non-positive synthetic typed input, `f32::from_bits(1)`과 `f32::MAX`의 derived-dimension overflow, failure atomicity를 포함한다. 두 extreme ratio는 각각 `40px / ratio`와 `60px × ratio`에 대해 Taffy 호출 전에 거부되는지 확인한다. Invalid CSS declaration이 computed `auto`가 되는 경우를 non-finite positive ratio로 잘못 취급하지 않는다.
5. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 신규 scene과 기존 #834 회귀를 검사한다. Record source/artifact SHA, HostDocument/Stylo route, frame/track trace, PixelCopy/Metal drawable surface samples와 screenshot. AVD는 hardware GPU가 아니며 iOS drawable은 OS compositor 제출 전이다.
6. 기본 product와 fixture-test static archive를 각각 빌드해 fixture FFI/payload absence와 positive-control presence를 확인한다. Focused Rust/runtime/FFI tests, relevant crates, rustfmt, feature Clippy를 실행한다.
7. 계획 자체를 서로 다른 실패 관점 20개로 검토한 뒤 계획 PR을 병합한다. 구현/runtime review 20개와 별도 PR diff review를 프로젝트 PR 규칙에 따라 30개로 마친다. 결함 수정 뒤 영향받은 항목을 재검토한다.
8. `spec/internal/0059-c11-grid-layout.md`, `spec/STATUS.md`, `plan/c11-4-grid-track-sizing.md`, public roadmap을 같은 범위로 동기화한다. C11.4/C11 checkbox와 `72/213` 집계는 그대로 둔다. 병합 후 exact merge SHA와 Tailnet rendered page를 확인한다.

## 제외 범위

일반 CSS admission, 임의 font/script, `auto <ratio>`, degenerate numeric ratio, replaced image, multiple ratio items, ratio가 있는 short text, min/max/edge transfer, wrapping/multiline, span, percentage/calc, vertical writing mode, Grid baseline alignment, full WPT runner, real-device/hardware GPU, iOS compositor-after pixel, C11.4/C11 완료는 이 slice에 포함되지 않는다. Ratio는 finite-positive typed input 중 derived geometry가 표현 가능한 값을 fixture profile에서 허용하지만, 이 계획의 정량 Chrome matrix는 지정된 네 ratio 값의 oracle 범위로 한정한다.

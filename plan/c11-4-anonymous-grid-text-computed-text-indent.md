# C11.4 anonymous Grid text computed `text-indent` 계획

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태 대장](../spec/STATUS.md)

**상태:** 제한 opt-in runtime slice 구현 PR [#930](https://github.com/ohah/spinon/pull/930)이 2026-10-31T16:37:17Z Rebase and merge로 병합됐다(merge SHA `b1b20e30cc8d36e90d87e902668eb95339698c25`). 18개 고정 scene의 구현·Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 실행 및 검토 근거는 아래 계획 범위와 [runtime evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-runtime-2026-11-01/README.md)에 연결한다. 이 계획은 C11.4 전체 완료나 일반 CSS 지원을 뜻하지 않는다.

**사전 비교:** [CSS 규범·독립 기대값·Chrome 156 Agent Browser 관측](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-precomparison-2026-11-01/README.md) · [계획 실패 관점 검토 20개](./c11-4-anonymous-grid-text-computed-text-indent-plan-review-20.md) · [계획 PR 변경 검토 30개](./c11-4-anonymous-grid-text-computed-text-indent-plan-pr-diff-review-30.md)

## 목표와 고정 범위

기존 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture에서 같은 HostDocument/style snapshot의 computed `text-indent`를 anonymous direct-text `ParagraphStyle`과 min-/max-content sizing에 전달한다. Ahem 10px/400, line-height 10px, horizontal-tb/LTR, normal whitespace, normal word-break, single anonymous text source, 0 gap/edge와 한 개의 min-content 또는 max-content column만 허용한다. scene matrix는 `text-indent:0px`, `5px`, `10px`과 `XX`, `XX XX`, `XX   XX XX  `의 조합으로 고정한다(18 scene).

이 profile은 computed CSS-px 값 0·5·10만 정확한 scene allowlist로 허용한다. 미등록 scene/value, 음수, 퍼센트, `each-line`/`hanging`, 다른 writing mode·whitespace·word-break·font·font size, stale revision 및 unexpected source는 layout publication 전에 구체 진단으로 거부한다. 계산 결과가 같은 authored expression의 출처를 판정하지 않는다. 일반 CSS admission, text-indent 전체 grammar, styled inline runs, arbitrary font/shaping, WPT 전체, 실제 glyph/surface paint 및 전체 C11.4/C11은 범위 밖이다. 기본 text/layout profile의 indent는 0이며 내부 계약 숫자 버전 `0.1.0`을 유지한다.

## 규범과 독립 기대값

[CSS Text Level 3 §8.1](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#text-indent-property)은 `text-indent`를 block container의 첫 formatted line에 적용하고 `each-line` 및 `hanging` 동작을 별도로 정의한다. [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes)의 min-/max-content 기준과 [CSS Grid Level 1 §11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)의 track contribution에 따라 각 고정 scene을 판단한다. 사전 비교는 Chrome 수치를 expected 값으로 복사하지 않고 Ahem font table 및 규범식으로 독립 계산했다.

Ahem 10px에서 `X`와 U+0020의 advance는 각각 10px이다. normal whitespace collapse 뒤 단어 폭을 `wᵢ`, 남은 separator 수를 `s`, indent를 `d ≥ 0`이라 할 때 첫 line이 가장 긴 단어를 포함하는 matrix의 독립 기대값은 다음과 같다.

- min-content inline contribution: `max(w₁ + d, max(wᵢ for i > 1, default 0))`; 이 matrix는 단어 길이가 같아 `20px + d`다.
- max-content inline contribution: `Σ(wᵢ) + s × 10px + d`.
- `XX`: min/max 모두 `20px + d`.
- `XX XX`: min `20px + d`, max `50px + d`.
- `XX   XX XX  `: collapse 후 세 단어와 두 separator가 남으므로 min `20px + d`, max `80px + d`.
- min-content row line count는 각각 1·2·3이고 max-content row는 한 줄이다. line-height는 10px이다.

각 값의 Chrome 156 Grid/Block observation과 음수·percentage·indent keyword 경계는 사전 비교 JSON에 고정했다. Chrome observation만으로 Spinon 지원을 선언하지 않는다.

## 구현 및 검증 순서

1. 이 계획을 별도로 서로 다른 실패 관점 20개에서 검토한다. 계획 PR 변경은 그 검토와 별개인 30개 실패 관점으로 검토한다. 두 review 결과가 통과한 뒤 계획 PR을 `Rebase and merge`한다.
2. `spinon-text-layout::ParagraphStyle`에 기본값 0인 `text_indent` CSS-px 값을 추가하고 finite 검증을 둔다. 고정 Parley 0.12.0의 `Layout::set_text_indent(value, IndentOptions::default())`를 line breaking 전에 적용한다. 기존 line/letter/word spacing 보정과 empty paragraph, default text profile을 회귀 검사한다.
3. C11.4 anonymous Grid fixture adapter에서 scene allowlist와 같은 HostDocument revision의 computed `text-indent`를 대조한다. exact `0px`/`5px`/`10px`만 연결하며 negative/percentage/keyword/missing property/scene mismatch 및 stale source를 거부한다.
4. 18개 unique fixed scene을 existing opt-in FFI route에 연결한다. Rust expected JSON, computed property, source revision, paragraph indent, min-/max-content, line count/advance, item frame, Columns/Rows track과 draw/present trace를 1:1 대조한다. 표준 Ahem glyph paint를 측정하지 않는다.
5. Chrome 156 사전 비교값과 독립 font-table 기대값을 모두 만족한다. `text-indent`가 없는 existing scenes의 golden 결과가 보존되고, default profile이 zero indent를 유지해야 한다. 음수 경계에서 `text-indent`를 단순히 모든 min-content line에 더하는 구현은 거부한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 18/18 scene을 실행하고 설치 artifact hash와 scene trace를 연결한다. 사용자 요청은 simulator 검증을 허용하며 실기기를 요구하지 않는다. AVD software rendering, iOS compositor 전 readback, black renderer screenshot을 glyph paint/pixel equality로 확대 해석하지 않는다.
7. `spinon-ffi` 기본 release static archive와 fixture-test positive control을 따로 빌드해 scene payload·adapter/validator symbol이 default에서 0이고 positive control에는 예상한 symbol·18 scene이 있음을 확인한다. archive 결과는 APK/IPA 전체 경계로 확대하지 않는다.
8. 관련 `spinon-text-layout`, `spinon-style-to-layout`, `spinon-runtime`, `spinon-ffi` tests, CSS fixture boundary, fmt, scoped Clippy, Android/iOS simulator app builds를 실행한다. 변경 전 baseline과 새 결과를 분리하고 미실행 항목은 통과로 보고하지 않는다.
9. 구현 후 code/runtime을 계획 review와 겹치지 않는 새로운 실패 관점 20개로 검토한다. implementation PR diff도 별도의 30개 관점으로 검토하고, 발견 사항 수정 뒤 영향 관점을 다시 확인한다.
10. implementation PR 본문에는 가능한 Android/iOS 동작 화면을 실제 저장된 PNG 링크로 연결한다. 링크가 PR에서 열리고 image content가 로드되는지 확인한다. CI check 부재는 성공으로 기록하지 않으며, merge 전 required checks와 Rebase and merge 결과를 다시 확인한다.
11. 병합 뒤 내부 계약 0059, `spec/STATUS.md`, C11.4/상위 계획과 공개 roadmap에 제한 profile·실행 증거·미지원 범위를 기록한다. C11.4/C11 체크와 official `72/213`은 그대로 둔다.

## 완료 조건과 한계

이 계획의 구현 완료 조건은 정확히 18 fixed opt-in scenes, independent oracle, fixture value/source/revision validation, Rust regressions, Android/iOS Simulator runtime, default/positive-control artifact boundary, implementation review 20개 및 PR diff review 30개다. 완료는 full text indentation, all author CSS, WPT conformance, actual device/GPU rendering, glyph pixels, C11.4 또는 C11 완료를 뜻하지 않는다.

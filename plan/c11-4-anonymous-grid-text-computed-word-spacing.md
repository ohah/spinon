# C11.4 anonymous Grid text computed `word-spacing` 계획·구현 결과

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 결과:** 구현 PR [#924](https://github.com/ohah/spinon/pull/924)은 2026-10-31T12:30:41Z Rebase and merge됐고 merge SHA는 `93aba4eff3998c54c595b661578b339aeda89d87`이다. 이 문서는 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1` fixture에서 computed 값을 `ParagraphStyle`과 intrinsic 측정에 전달한 고정 24-scene slice의 범위·근거를 보존한다. 일반 CSS admission이나 C11.4 완료를 선언하지 않는다.

**사전 비교:** [CSS 규범·Ahem 독립 계산·Chrome 156 관측 24/24](../spec/internal/evidence/c11-4-anonymous-grid-text-word-spacing-precomparison-2026-10-31/README.md) · [fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-word-spacing.html) · [계획 실패 관점 검토 20개](./c11-4-anonymous-grid-text-computed-word-spacing-plan-review-20.md) · [계획 PR diff 검토 20개](./c11-4-anonymous-grid-text-computed-word-spacing-plan-pr-diff-review-20.md)

## 목표와 범위

Stylo가 계산한 `word-spacing`을 anonymous direct-text paragraph의 `ParagraphStyle`과 Parley layout에 보존하고, inline-axis min-/max-content와 그에 따른 row/frame/track 결과를 고정 scene에서 검증한다. `normal`과 authored `0px`는 computed `0px`, 추가 길이 `-1px`와 `1px`은 해당 절대 CSS px로 처리한다. 값은 scene allowlist에 정확히 연결하고 finite `px` 이외의 입력은 fixture adapter가 거부한다.

범위는 Ahem 10px/400, line-height 10px, horizontal-tb/LTR, direct anonymous text source 하나, `white-space:normal`, `word-break:normal`, 한 개의 `min-content` 또는 `max-content` column, auto row, 0 gap·edge로 고정한다. scene 내용은 `XX XX`, 반복·후행 공백이 있는 `XX   XX XX  `, separator가 없는 `XX` 세 가지다. 정상 whitespace collapse 후 남은 separator의 수, min-content에서 가장 긴 단어, separator가 없는 대조군을 각각 확인한다.

지원 범위는 제한 fixture/test profile이다. 새 앱 작성 API, 일반 author CSS property 지원, 전체 anonymous text style cascade, nested/inline styled run, 임의 글꼴·shaping, word justification, RTL/writing mode, arbitrary CSS length expression은 추가하지 않는다. 내부 인터페이스 숫자 버전 `0.1.0`을 유지한다. C11.4/C11 체크 및 공식 완료 수 `72/213`을 바꾸지 않는다.

## 규범과 독립 기대값

[CSS Text Level 3 §7.1](https://www.w3.org/TR/css-text-3/#word-spacing-property)은 추가 간격을 whitespace 처리 뒤 남은 word separator에 적용하고 `normal`의 추가 간격을 0으로 정의한다. [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes)에서 min-content는 zero available space, max-content는 무한 available space 기준이다. [CSS Grid Level 1 §11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)은 item intrinsic contribution을 해당 intrinsic track sizing에 사용한다.

Ahem의 `X`와 U+0020 advance는 각각 1000/1000 units per em이다. Ahem 10px에서는 문자와 기본 공백 하나의 advance가 각각 10px이다. normal whitespace 뒤 단어 길이를 `wᵢ`, 남은 separator 수를 `s`, computed spacing을 `d`라 하면 독립 계산은 다음과 같다.

- min-content inline contribution = `10px × max(wᵢ)`
- max-content inline contribution = `10px × Σ(wᵢ) + s × (10px + d)`
- min-content row height = 단어 수 × 10px; max-content row height = 10px
- 단어가 하나면 `word-spacing`에 관계 없이 inline contribution은 20px이다.

고정 Chrome 156 oracle은 `normal`과 `0px`의 computed serialization, 24 scene의 grid track/frame을 관측한다. 판정 기대값은 Chrome 수치를 복사하지 않고 Ahem font table과 위 규범식에서 생성한다. 관측과 식의 일치는 브라우저 비교이지 Spinon 동작 증거가 아니다.

## 구현 순서와 판정 관문

1. 구현 전에 계획과 위 24-scene oracle을 별도의 실패 관점 20개로 검토한다. 발견한 누락은 계획·fixture·독립 기대값에서 먼저 수정한다.
2. 계획 PR 본문에 oracle screenshot을 첨부하고, 계획 PR diff는 계획 검토와 독립된 20개 관점으로 검토한 뒤 `Rebase and merge`한다.
3. 최신 `main`에서 `ParagraphStyle.word_spacing`을 추가한다. default는 0이며 non-finite 값은 거부한다. Parley `StyleProperty::WordSpacing`에 전달한다. 기존 line/letter spacing의 edge 보정과 line breaking을 바꾸지 않고, 실제 Parley metrics가 spacing을 반영하는지 text-layout 회귀로 확인한다.
4. C11.4 opt-in anonymous-text adapter에서 computed CSSOM serialization을 검증하고 scene에 고정한 값만 통과시킨다. `normal`과 `0px`의 Stylo 직렬화 차이, signed zero, 비정상/미등록 scene, 다른 node와 다른 property values를 negative control로 둔다.
5. 24개의 unique fixed scene을 `RuntimeBlockFlexGridTrackSizingV1` runtime fixture에 연결한다. 각 scene에서 computed paragraph property, HostDocument revision, text intrinsic min/max, line advance/count, item frame, Columns/Rows final track과 draw/present trace를 expected JSON과 1:1 비교한다. min/max geometry의 정수 CSS px는 exact로 비교한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 24/24 scene, installed artifact hash, route/status를 확인한다. 앱 screenshot은 text renderer의 glyph pixels가 확인된 경우에만 paint 근거로 사용한다. black renderer, simulator capture 또는 trace만으로 glyph paint·PixelCopy·Metal drawable·compositor 이후 surface equality를 주장하지 않는다. 이 slice는 실기기를 요구하지 않는다.
7. 기본 product artifact와 opt-in fixture-test artifact를 따로 빌드한다. 신규 scene/adaptor/validator payload와 symbol은 default archive에서 빠지고 fixture positive control에는 예상 수만 들어야 한다. static archive 증거를 전체 모바일 release bundle 검사로 확대하지 않는다.
8. 관련 Rust unit/integration tests, CSS fixture boundary 검사, Android/iOS builds, `cargo fmt --all -- --check`와 scoped Clippy를 실행한다. workspace baseline failure는 수정 전 기준과 변경 결과를 분리한다.
9. 구현과 runtime 결과를 계획 검토에서 재사용하지 않은 새로운 실패 관점 20개로 검토한다. implementation PR diff도 별도의 20개 관점으로 검토하며 결함 수정 뒤 영향을 받은 항목을 다시 확인한다. PR마다 20개 관점을 채우기 위해 동의어 반복을 하지 않는다.
10. PR 본문에 Android/iOS simulator 화면을 가능한 경우 각각 첨부하고, black renderer 등 화면이 증명하지 못하는 범위를 명시한다. 병합 전에 현재 required checks·CI rollup·merge 상태를 다시 조회하고 `Rebase and merge`한다. check 부재는 CI 통과로 기록하지 않는다.
11. 구현 PR #924 병합 뒤 내부 계약 0059, 공식 상태 대장, C11.4 상위 계획과 공개 로드맵에 결과 및 남은 한계를 기록한다. 이 문서 갱신으로 공식 C11.4/C11 check와 `72/213`은 그대로 둔다.

## 구현 및 병합 결과

Chrome 156/Ahem 독립 기대값과 fixed fixture 24/24가 일치했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator는 각각 24/24를 통과했다. 관련 Rust suites는 text-layout 74, runtime fixture 352 passed·3 ignored, FFI fixture 175 passed·2 ignored였고 CSS fixture boundary 1/1, formatting, scoped Clippy 및 기본/fixture-test static archive 경계도 확인했다. 실행 로그와 실제 한계는 [runtime evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-word-spacing-runtime-2026-10-31/README.md)에 둔다.

PR #924의 check 결과는 보고되지 않았고 `main` required status checks도 설정되어 있지 않았다. 따라서 CI 통과로 기록하지 않는다. Simulator 캡처의 WGPU 영역은 검게 나와 glyph paint 증거가 아니다. 일반 CSS author admission, 임의 길이·글꼴, WPT, 실기기·hardware GPU, glyph/surface pixel equality와 전체 C11.4/C11은 확인하지 않았다. 공식 C11.4/C11 check는 미완료, 전체 집계는 `72/213` 그대로다.

## 완료 판정과 제외 범위

이 계획의 완료는 computed word-spacing이 연결된 24개 고정 opt-in scene만 의미한다. Chrome oracle, 독립 기대값, text-layout tests, Stylo property capture, runtime validation, Android/iOS simulator matrix, default/fixture artifact 분리, code/runtime review와 PR diff review를 모두 확인해야 한다.

일반 HTML/CSS admission, arbitrary `word-spacing` values/calc/units, full whitespace and writing-mode cross product, inline run inheritance, actual glyph/compositor pixels, WPT suite, physical device, hardware GPU, 전체 C11.4/C11은 완료 기준에 포함되지 않는다.

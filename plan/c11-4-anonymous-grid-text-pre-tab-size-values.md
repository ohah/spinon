# C11.4 anonymous Grid text `tab-size` 전체 computed value 제한 profile

**Parent:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [internal Grid contract 0059](../spec/internal/0059-c11-grid-layout.md) · [inline text contract 0070](../spec/internal/0070-c15-1-inline-text-layout.md) · [CSS Text Level 3 §4.1.2/§4.2](https://www.w3.org/TR/css-text-3/#tab-size-property)

**Status:** 계획 자체 실패 관점 20/20과 별도 계획 PR diff 20/20 검토 완료. 계획 PR이 main에 병합되기 전까지 구현은 시작하지 않는다. [계획 검토](./c11-4-anonymous-grid-text-pre-tab-size-values-plan-review-20.md) · [계획 PR diff 검토](../spec/internal/evidence/c11-4-pre-tab-size-values-precomparison-2026-10-31/plan-pr-diff-review-20.md) · [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-pre-tab-size-values-precomparison-2026-10-31/README.md)

## 문제와 목표

현재 C11.4 anonymous Grid `white-space:pre` 경로는 computed unitless `tab-size` `1`, `2`, `4`, `8`만 받는다. `TabSize::spaces()`와 텍스트 정규화기는 tab을 정수 개수의 U+0020으로 바꾸므로 CSS의 fractional number, `<length>`, zero를 표현하지 못한다. 이 확장은 그 allowlist를 늘리는 데 그치지 않고, 이미 구현한 fixture 경로가 CSS Text Level 3의 전체 non-negative number/length value model을 처리하도록 바꾼다.

지원값은 computed `<number [0,∞]>`와 computed absolute `<length [0,∞]>` 전체다. unitless value는 tab의 nearest block container에서 쓰이는 U+0020 advance의 배수이며, length는 계산된 CSS px 길이다. 기존 제한 fixture에서 Ahem 10px/10px, U+0020 advance 10px, `1ch` 10px, zero letter/word spacing을 유지한다. 계산 결과가 text/layout backend의 유한 CSS px 범위를 넘으면 partial geometry를 내지 않고 기존 오류 경계로 닫는다. 특정 정수 목록이나 임의의 작은 numeric range를 새 지원 경계로 만들지 않는다.

CSS Text Level 3 §4.1.2에 따라 zero-width tab은 렌더하지 않는다. 양수 interval에서는 tab stop을 해당 줄 content origin 기준으로 계산하고, 다음 stop까지 거리가 `0.5ch`보다 작으면 그 다음 stop으로 이동한다. forced LF/CRLF 뒤에는 tab position을 0으로 reset한다. Blink source의 [`ShapeResult::CreateForTabulationCharacters`](https://chromium.googlesource.com/chromium/src/+/HEAD/third_party/blink/renderer/platform/fonts/shaping/shape_result.cc#1844)는 연속 tab run의 첫 tab만 현재 위치에 따른 폭을 계산하고 후속 tab에 base interval을 더한다. 이 관측을 고정 expected model에 포함한다.

## 구현 경계

- 경로는 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1` anonymous-text fixture다. 일반 CSS admission, 새 공개 API, 제품 전체 텍스트 지원, root HTML loader 변경을 추가하지 않는다.
- collector는 **computed-style snapshot의 `tab-size`** 직렬화만 읽는다. 숫자는 finite, non-negative number로, length는 computed absolute `px`로 parse한다. `em`/`calc()`는 upstream computed style이 resolve한 값을 사용하며 authored declaration string, source text, scene ID로 값을 추론하지 않는다. Scene ID allowlist는 기대값 불일치 검증에만 사용한다.
- `TabSize`는 고정 enum에서 typed number/absolute-px value로 바꾼다. 유한성·음수·overflow·잘못된 단위를 검사하고, 누락/stale snapshot이나 기대값 불일치에서는 layout·frame·trace·draw를 publication하지 않는다.
- tab은 normalized text에 여러 U+0020으로 대체하지 않는다. 원본 tab의 node/UTF-16 source range를 보존하는 위치 anchor와 계산된 inline advance를 전달해 sub-space/fractional 간격을 표현한다. zero tab은 zero advance이며 glyph 또는 양수 inline box를 만들지 않는다. 모든 glyph는 원래 text source에 계속 귀속한다.
- `pre` text normalizer와 Parley build path는 tab advance를 layout 및 intrinsic measurement에 반영한다. consecutive tabs, current inline position, line reset, leading/interior/trailing tabs, min-content/max-content, `white-space:pre`의 soft-wrap 금지와 좁은 fixed track overflow를 함께 검증한다.
- 현재의 고정 text profile(Ahem 10px, ASCII `X`·space·tab·segment break, horizontal-tb/LTR, zero spacing, inline ancestor 없음)은 유지한다. 지원값 model을 좁히는 범위 제한은 추가하지 않는다.
- C11.4 checker, C11 overall checkbox와 공식 `72/213` 집계는 구현 증거가 parent acceptance를 만족하기 전까지 바꾸지 않는다.

## 고정 입력과 사전 기준

입력·실행 환경·observations·hash·독립 산술은 [precomparison README](../spec/internal/evidence/c11-4-pre-tab-size-values-precomparison-2026-10-31/README.md), [HTML fixture](../tests/fixtures/css/c11/anonymous-grid-text-pre-tab-size-values-precomparison.html), [expected-values.json](../spec/internal/evidence/c11-4-pre-tab-size-values-precomparison-2026-10-31/expected-values.json), [chrome-observation.json](../spec/internal/evidence/c11-4-pre-tab-size-values-precomparison-2026-10-31/chrome-observation.json)에 고정한다. 14 computed inputs × 10 text/layout cases = 140 positive browser observations이며, 값마다 추가한 tab-only control 14개를 합쳐 전체 154개를 계측한다. 입력값은 `0`, fractional numbers `0.25/0.3/0.5/1.5/2.5/3.25`, `0px/0.3px/2px/5px/12px`, `1.5em`, `calc(2px + 3px)`다. 비이진 소수 `0.3`과 `0.3px`은 backend 변환 정밀도도 확인한다. CSS Grid는 공백만 있는 anonymous text sequence를 item으로 만들지 않으므로 tab-only control은 0×0 ignored-item 경계 대조군이며 text-layout 양성 장면이 아니다. CSSOM supports/computed string, 모든 장면의 frame, 140개 양성 장면의 관측 가능한 line advance를 독립 기대값과 비교한다.

구현 positive matrix는 140개의 text/layout 양성 입력을 각각 opt-in runtime scene으로 연결한다. tab-only control은 anonymous Grid item 생성 전에 억제되므로 runtime scene 수에 포함하지 않는다. 별도 negative controls는 malformed/negative/비유한/레이아웃 표현 불가 fixture snapshot, absent computed property, stale revision, scene expected value와 computed value 불일치를 확인한다. 이 입력들은 Chrome의 CSS cascade 결과와 fixture FFI의 raw computed-style boundary를 구분한다. negative authored CSS 자체는 browser cascade에서 invalid declaration이므로 raw negative computed value와 혼동하지 않는다. zero tab은 `\tX` 같은 양성 text item의 leading/interior/trailing 위치에서 glyph advance가 0인지 확인한다.

## 검증 기준

1. 계획 자체의 서로 다른 20개 실패 관점 검토와 별도 계획 PR diff 20개 검토를 기록하고 계획 PR을 main에 rebase-merge한 다음 구현을 시작한다.
2. `spinon-text-layout` 단위 검사에서 number/length parse, mixed-text zero-tab suppression, `0.5ch` 미만·정확·초과, consecutive runs, LF/CRLF reset, source provenance, min/max-content, trailing tab, `0.3`/`0.3px` fractional precision을 CSS Text 독립 계산과 검증한다. Chrome CSSOM은 exact computed serialization으로, Chrome geometry는 Blink 1/64px outward quantization으로 별도 비교하며 이 양자화를 Spinon의 CSS 의미 규칙으로 만들지 않는다. 기존 `Pre`, `PreWithTabSize(1/2/4/8)`, `PreWrap`, `BreakSpaces` 결과를 회귀 확인한다. Grid 경계 검사는 tab-only anonymous text가 collector 입력이 되지 않는다는 기존 Grid 수집 계약을 별도로 확인한다.
3. C11.4 runtime positive 140 scenes와 negative controls를 computed string, typed value, normalized anchor/source map, per-line advance, intrinsic contribution, Grid track/frame, revision, error-before-publication, draw/present/retry로 확인한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 55-scene 회귀와 신규 140-scene matrix를 각각 실행한다. 앱 설치/build hash를 연결하고 가능한 동작 화면을 PR 본문에 넣는다. AVD software renderer와 iOS compositor 제출 전 drawable이라는 한계를 유지한다.
5. default/product archive에는 fixture ABI·scene markers·payload가 없고 fixture-test opt-in positive control에만 있는지 재검증한다. focused Rust/FFI/runtime tests, fixture module boundary, relevant Clippy, rustfmt를 실행한다. CI check rollup이 비어 있으면 CI 성공으로 쓰지 않는다.
6. 구현/runtime 동작 20개 관점과 독립적인 PR diff 20개 관점을 각각 기록한다. 계획 검토를 구현 리뷰로 재사용하지 않는다.
7. implementation merge 뒤 `0059`, `0070`, 이 계획과 `spec/STATUS.md`를 실제 코드와 simulator 근거에 동기화한다. 내부 계약 숫자 버전 `0.1.0`을 유지하고 C11.4/C11을 완료로 표시하지 않는다.

## 남는 상위 범위

이 value-profile 확장만으로 일반 CSS cascade, arbitrary fonts/spacing/scripts, bidi/vertical writing, `pre-wrap`/`break-spaces` tab semantics, HTML-root-to-runtime public style mapping, glyph-ink equality, Spinon WPT conformance, physical-device/hardware-GPU proof, C11.4 전체 또는 C11 전체를 완료로 표시하지 않는다. CSS Text 3 CRD에서 `<length>` 값은 at-risk로 표시되므로 이 slice는 Chrome reference에 해당 기능이 있음을 고정하되 그 상태를 안정된 표준 지원으로 확대하지 않는다.

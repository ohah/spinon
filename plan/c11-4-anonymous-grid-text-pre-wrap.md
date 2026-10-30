# C11.4 anonymous Grid text `white-space:pre-wrap` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [CSS 적합성 계약](../spec/0001-conformance.md)

**상태:** 계획 PR [#869](https://github.com/ohah/spinon/pull/869)은 2026-10-30에 `47ffeb1216a1085cb40c505f312108dea66c600c`로 병합됐다. PR [#870](https://github.com/ohah/spinon/pull/870)은 2026-10-30T18:51:40Z에 Rebase and merge로 병합됐고 merge SHA는 `5265793d71a02a6206a09b9bbbc9658696dfd992`이다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 고정 16-scene 행렬을 각각 16/16 통과했다. 구현/runtime 적대 검토 20개와 macOS ARM64 static archive의 product/fixture 경계 검사는 끝났다. 현재 소스 후보 diff에 대한 별도 30관점 적대 검토도 완료했다. 전체 workspace test는 변경 밖 `spinon-style-to-layout` C07.3 고정 문자열 비교에서 실패했으며 결과와 범위는 [runtime 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/README.md)에 기록했다. 최종 PR head `bf817aff25c5d39dcec45a744102cac9a0d9d0d7`의 변경 patch는 로컬 검토본과 일치했다. PR checks 결과는 보고되지 않았고 main required checks도 설정되지 않아 CI 통과로 기록하지 않는다. 일반 CSS 지원은 아니며 C11.4/C11 체크와 공식 완료 `72/213`은 변경하지 않는다. [사전 비교·hash·화면](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31/README.md) · [runtime 결과](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/README.md)

## 목적과 계약 경계

CSS Text Level 3 §3의 `pre-wrap`은 공백을 보존하고 soft wrapping을 허용한다. 줄 끝의 보존 공백은 hanging 처리되며 `break-spaces`와 동작이 다르다. 기존 C11.4 anonymous Grid text fixture는 `Normal | NoWrap | PreLine`만 허용한다. 이 변경은 `PreWrap`을 text layout model에 추가하고, 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1` fixture에서 anonymous Grid item의 intrinsic sizing과 사용 크기까지 전달한다.

허용 범위는 단일 direct-text anonymous Grid item, Ahem 10px/10px, horizontal-tb/LTR, `order:0`, CSS initial `tab-size:8`을 전제한 고정 tab input `XX\tXX`, 고정된 명시적 track 및 지정된 `pre-wrap` 장면이다. 다른 tab source나 inline ancestor가 있으면 거부한다. 앱 16-scene 행렬에는 tab 문자가 없고 runtime style projection은 `tab-size`를 노출하거나 검증하지 않는다. tab expansion은 고정 입력의 Rust unit test에서만 검증한다. 일반 author CSS cascade, arbitrary HTML document, root-HTML 기본 렌더러의 white-space mapping, authored `tab-size` 변경, CSS shorthand parser, 공개 지원 계약, nested Grid, inline element, 다른 writing mode, `white-space:pre`, `break-spaces`, `text-wrap:balance`, WPT runner와 C11.4 전체 완료는 포함하지 않는다. 이 계획은 사용자의 별도 root-HTML parity 요청을 완료한 것으로 표시하지 않는다.

## 사전 비교와 독립 기대값

[고정 HTML fixture](../tests/fixtures/css/c11/anonymous-grid-text-pre-wrap-precomparison.html)의 열두 장면을 Agent Browser 0.31.1 / HeadlessChrome 156, viewport `1280×633`, DPR 1에서 관찰했다. Ahem font가 `loaded`이고 `document.fonts.check('10px Ahem')`이 true인 것을 확인했다. [CSS Text Level 3 §3](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#white-space-property)는 `pre-wrap`을 preserve + wrap으로 정의한다. [CSS Grid Level 1 §6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-items)는 white-space만 포함한 direct text sequence를 렌더링하지 않아 Grid item이나 auto-placement cell을 만들지 않는다고 정의하며, §§7.2.1·11.5.1이 track sizing 문맥을 정한다.

독립 계산과 Chrome observation은 각각 [expected-values.json](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31/expected-values.json), [Agent Browser observation](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31/chrome-observation-agent-browser.json), [screenshot](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-precomparison-2026-10-31/chrome-156-agent-browser.png)에 있다.

| 장면 | 입력·track | 독립 기대값 | Chrome 156 |
| --- | --- | --- | --- |
| `normal-max-content` | `XX  XX`, `max-content` | normal collapses spaces: column 50px, row 10px | `[50]`, `[10]`, root `50×10` |
| `pre-wrap-max-content` | 같은 source, `max-content` | 두 공백 보존: column 60px, row 10px | `[60]`, `[10]`, root `60×10` |
| `pre-wrap-comment-split-max-content` | `XX <!-- split --> XX`, source nodes `"XX "` + `" XX"` | comment는 box가 아니므로 하나의 anonymous item, 두 공백 보존 | `[60]`, `[10]`, root `60×10` |
| `pre-wrap-wrap` | `XX XX`, definite width/track 25px | separator 뒤 wrap, 끝 공백 hanging, row 20px | `[25]`, `[20]`, root `25×20` |
| `pre-wrap-forced-break-max-content` | `XX\nXX`, `max-content` | LF forced break, column 20px, row 20px | `[20]`, `[20]`, root `20×20` |
| `normal-min-content` | `XX  XX`, `min-content` | 두 단어 경계로 column 20px, 두 줄 | `[20]`, `[20]`, root `20×20` |
| `pre-wrap-min-content` | 같은 source, `min-content` | hanging preserved spaces가 20px track을 늘리지 않음 | `[20]`, `[20]`, root `20×20` |
| `pre-wrap-whitespace-only-item` | 공백 두 개만 있는 direct text | Grid item 없음, zero-sized max-content tracks | `[0]`, `[0]`, root `0×0` |
| `pre-wrap-leading-spaces-max-content` | `  XX`, `max-content` | 앞 공백 20px과 text 20px 보존 | `[40]`, `[10]`, root `40×10` |
| `pre-wrap-trailing-spaces-max-content` | `XX  `, `max-content` | max-content line에서 뒤 공백 두 개 보존 | `[40]`, `[10]`, root `40×10` |
| `pre-wrap-tab-max-content` | `XX\tXX`, `max-content` | tab-size 8, tab은 x=20에서 x=80까지 전진 | `[100]`, `[10]`, root `100×10` |
| `pre-wrap-whitespace-run-before-block` | 공백 두 개 뒤의 10×10 block | 공백 sequence가 cell을 소비하지 않아 block은 첫 cell | columns `[20,20]`, marker `(0,0)` |

### 앱 fixture 판정값

앱 harness의 text item 장면은 root width 20px인 overflow 관측 profile을 사용한다. 이 값은 위 max-content oracle의 root가 max-content인 CSS precomparison과 구분한다.

| 신규 fixture scene | Root frame | Columns / Rows | Text item frame / 특이점 |
| --- | --- | --- | --- |
| `anonymous-normal-max-content-spaces` | `20×10` | `[50]` / `[10]` | `50×10`, max-content contribution 50 |
| `anonymous-pre-wrap-max-content-spaces` | `20×10` | `[60]` / `[10]` | `60×10`, max-content contribution 60 |
| `anonymous-pre-wrap-comment-split-max-content` | `20×10` | `[60]` / `[10]` | `60×10`, 두 Text node의 source mapping은 각 원본 node 보존 |
| `anonymous-pre-wrap-min-content-spaces` | `20×20` | `[20]` / `[20]` | `20×20`, two lines; min-content contribution 20 |
| `anonymous-pre-wrap-soft-wrap` | `25×20` | `[25]` / `[20]` | `25×20`, final separator may hang past line edge |
| `anonymous-pre-wrap-forced-break` | `20×20` | `[20]` / `[20]` | `20×20`, LF creates two forced lines |
| `anonymous-pre-wrap-whitespace-before-block` | `40×10` | `[20,20]` / `[10]` | marker `[0,0,10,10]`; whitespace-only text consumes no cell |

각 앱 scene은 CSS computed values와 typed `ParagraphStyle`을 따로 기록한다. Range fragment, item border box, grid track contribution과 surface sample은 서로 다른 관측층이며 한 층의 결과를 다른 층의 증거로 확대하지 않는다.

## 계획된 구현과 검증

1. `crates/spinon-text-layout/src/model.rs`의 `WhiteSpaceMode`에 `PreWrap`을 추가한다. `crates/spinon-text-layout/src/normalize.rs`는 PreWrap source의 연속·앞·뒤 공백을 접거나 제거하지 않고 보존하며, LF/CR/CRLF/form-feed segment break는 정규화된 LF로 전달한다. CRLF 두 source code unit은 한 output line-break range에 모두 provenance를 유지한다. Normal/NoWrap/PreLine 정규화는 그대로 둔다.
2. `crates/spinon-text-layout/src/engine/intrinsic.rs`에서 PreWrap을 `TextWrapMode::Wrap` + `WhiteSpaceCollapse::Preserve`로 전달한다. normal/no-wrap Ahem fast path에는 PreWrap을 포함하지 않는다. Parley 0.12의 현재 `WhiteSpaceCollapse::Preserve` mapping에는 별도 `tab-size` property가 보이지 않으므로 고정 Ahem 입력 `XX\tXX`에서만 default `tab-size:8` 확장을 수행하고 source provenance를 보존한다. 다른 tab source나 inline ancestor는 실패해야 한다. unmodified tab passthrough가 100px을 내는 것으로 가정하지 않는다. `spinon-text-layout` unit tests는 repeated/leading/trailing spaces, 8-space tab stop, LF와 CRLF source mapping, min-/max-content size, 25px soft wrap 및 hanging space를 각기 검증한다. tab 입력은 앱 runtime 행렬에는 포함하지 않는다.
3. C11.4 anonymous Grid paragraph allowlist에만 `WhiteSpaceMode::PreWrap`을 추가하고 오류 설명을 갱신한다. `crates/spinon-layout/src/host_document_tests/c11_4_anonymous_text/`에서 여섯 text paragraph scene의 intrinsic values, root/item frames, source/revision provenance와 Columns/Rows contribution trace를 확인한다. HTML comment가 남기는 두 Text source boundary와 동등한 두 direct Text source를 고정 앱 장면에서 하나의 anonymous item으로 묶고, 각 원본 source provenance를 보존하는지 확인한다. 이 앱 입력은 Comment node 자체를 만들지 않는다. 일곱 번째 `whitespace-before-block` 장면은 공백-only sequence가 paragraph/item을 생성하지 않고 다음 block을 첫 auto-placement cell에 배치하는 Grid 경계 검사다.
4. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items.rs`의 opt-in scene validator/source를 확장한다. 신규 고정 scene은 `normal repeated-space max-content`, `pre-wrap repeated-space max-content`, `pre-wrap comment-split max-content`, `pre-wrap min-content`, `pre-wrap soft-wrap`, `pre-wrap forced-break max-content`, `pre-wrap whitespace-before-block`이다. Runtime validator는 PreWrap 장면의 computed `white-space-collapse:preserve;text-wrap-mode:wrap`을 확인한다. `tab-size`는 runtime style projection이 노출하지 않아 앱에서 확인하지 않았고, tab-width 구현은 CSS initial value 8을 전제로 한 고정 profile Rust unit test에서만 검증한다. 일곱 scene은 기존 9 scene을 포함한 고정 16-scene matrix에서 exact match해야 한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator fixture harness에 일곱 scene을 연결한다. 각 플랫폼에서 16/16 scene, 신규 scene의 root/item frame, intrinsic contribution, Columns/Rows trace, source revision, draw/present를 확인한다. 이 행렬은 [runtime 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/README.md)에 기록했다. 화면 캡처는 app fixture와 box 영역만 증명하며 glyph ink를 자동으로 증명하지 않는다. Android GPU/backend와 iOS compositor 경계는 실제 실행 근거대로 기록한다.
6. 기존 fixture 경계 검사와 Android/Swift scene allowlist를 함께 갱신한다. 기본 product artifact에서 새 FFI marker/source payload가 빠지고 fixture-test positive control에만 포함되는지 확인한다. 실행할 focused tests, workspace Clippy, `cargo fmt --all -- --check`, Android/iOS simulator builds 및 scene captures는 최신 source revision과 연결한다.
7. 구현/runtime 검토 20개는 계획 검토와 별도로 수행했고, 현재 후보 source diff에도 서로 다른 30개 실패 관점을 적용했다([구현 검토](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/implementation-review-20.md), [diff 검토](../spec/internal/evidence/c11-4-anonymous-grid-text-pre-wrap-runtime-2026-10-31/pr-diff-review-30.md)). PR #870 최종 head를 검토본과 대조한 뒤 저장소 정책에 따라 `Rebase and merge`했다.

## 실패 기준과 제외 범위

- `normal-max-content`가 50px이 아니거나 `pre-wrap-max-content`가 60px이 아니면 실패다.
- soft-wrap scene이 25px track에서 20px 두 줄로 배치되지 않거나 line-end preserved space가 track contribution에 잘못 포함되면 실패다.
- forced break가 제거·collapse되거나 CRLF source provenance가 누락되면 실패다.
- `pre-wrap-min-content`가 20px min-content track과 두 줄을 만들지 못하면 실패다.
- whitespace-only sequence가 anonymous text item/paragraph를 만들거나 다음 block의 auto-placement cell을 이동시키면 실패다.
- PreWrap이 opt-in C11.4 경계 밖, `order != 0`, unsupported paragraph profile, 또는 일반 root HTML/CSS cascade에서 새로 허용되면 실패다.
- Android/iOS의 final-source frame·intrinsic·trace 결과가 독립 계산과 다르거나 default artifact에 fixture marker가 들어가면 실패다.

이 제한 slice는 WPT 적합성, 임의 font/script, 일반 authored CSS 지원, root HTML parity, GPU glyph pixels, 실기기·hardware renderer, iOS compositor 이후 pixels 또는 C11/C11.4 공식 완료를 입증하지 않는다. 내부 계약 0059는 `0.1.0`을 유지한다.

# C11.4 · 직교 Ahem text intrinsic 입력 일반화 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [직전 직교 text/image slice](./c11-4-orthogonal-row-intrinsic-contribution.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 PR #629 병합됨. 제한 구현 PR [#630](https://github.com/ohah/spinon/pull/630)은 2026-10-25에 Rebase and merge로 병합됐고 merge SHA는 `6ac7cb1a7ea2826f29905eb2caa71080ed85e4a9`다. Android/iOS Simulator 각 22/22 실행, source-bound static archive 기본·fixture-test 양성/음성 대조, 구현/runtime 실패 관점 검토 20개가 끝났다. 이 제한 fixture는 공개 CSS 지원이나 C11.4 전체 완료를 의미하지 않으며, C11.4와 공식 완료 수 `72/213`은 그대로다.

PR #627은 `vertical-rl` Ahem intrinsic 입력을 특정 문자열 `XX XXXXX` 하나로 제한했다. 이 계획은 같은 안전한 shaping profile 안에서 alphanumeric word sequence와 collapsible U+0020을 여러 길이로 허용하고, logical min/max contribution을 Chrome 156 및 독립 계산과 대조한다. 공개 CSS 지원, 전체 vertical text shaping, C11.4 완료는 선언하지 않는다.

## 목적과 계획 작성 당시 코드 경계

이 계획을 작성할 당시 `TextLayoutEngine::intrinsic_logical_sizes`는 `horizontal-tb/mixed` 입력을 일반 경로로 측정했지만 `vertical-rl/mixed`는 단일 Ahem 10px 문자열 `XX XXXXX`에서만 허용했다. `LayoutGridItemIntrinsicInput::contribution_input` 및 C11.4 projection도 horizontal parent와 vertical child의 제한 조합만 받았다. [PR #627](https://github.com/ohah/spinon/pull/627)은 네 장면에서 `XX XXXXX`의 logical inline min/max `50px/80px`이 horizontal parent의 row track에 전달되는 경로를 확인했다. 이번 계획의 구현은 이 guard를 아래의 검증 가능한 제한 집합으로 확장했다.

이번 작업은 text-layout에서 특정 문장만 받는 guard를 다음의 검증 가능한 제한 집합으로 일반화한다.

- `vertical-rl` / `text-orientation:mixed` paragraph와 `horizontal-tb` Grid parent
- Ahem 10px, weight 400, normal style, 명시 line-height 10px, normal white-space/word-break/line-break, LTR, normal ligatures
- source 하나, inline ancestor/style/run style 없음
- 내용은 비어 있지 않은 ASCII 영숫자와 일반 공백 U+0020만 사용한다. 공백만 있는 문자열도 허용해 collapse 뒤 zero intrinsic contribution과 replaced-item floor를 확인한다. 반복·leading/trailing 공백은 CSS normal whitespace collapsing을 따른다.

이 subset은 Ahem 영숫자의 advance와 단어 경계가 고정되어 있어 Parley의 logical intrinsic 계산을 독립 수치와 비교할 수 있다. `vertical-lr`, upright/CJK/emoji, 다른 글꼴·타이포그래피, punctuation/hyphen line breaking, RTL/bidi, inline styling, 여러 source node, anonymous text item은 계속 거부한다. 일반 vertical glyph shaping이나 glyph painting을 주장하지 않는다.

## Chrome oracle와 독립 기대값

Self-contained 입력은 [fixture](../tests/fixtures/css/c11/grid-orthogonal-text-generalization-c114-precomparison.html), Agent Browser observation과 화면은 [사전 비교 evidence](../spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/README.md)에 고정했다. Agent Browser 0.31.1이 연 `HeadlessChrome/156.0.0.0`은 viewport `1280×633`, DPR 1에서 Ahem font load와 image decode 뒤 10개 target과 2개 image 없는 whitespace control을 관측했다.

CSS Grid Level 1 2025 CRD [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content)과 [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)는 column sizing 이후 row contribution 및 intrinsic track base 결정을 정의한다. CSS Writing Modes Level 3 [§6.1](https://www.w3.org/TR/css-writing-modes-3/#abstract-dimensions)과 [§7.3](https://www.w3.org/TR/css-writing-modes-3/#orthogonal-flows)은 horizontal parent의 block axis와 `vertical-rl` child의 inline axis가 직교 flow에서 대응하는 축임을 정한다. CSS Text Level 3 [§3](https://www.w3.org/TR/css-text-3/#white-space-property)는 normal whitespace collapsing을, CSS Box Sizing Level 3 [§2.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes)는 min/max-content size를 정의한다.

저장소 Ahem font의 `unitsPerEm`은 1000이며 fixture에 쓰인 `A`–`F`, `9`, `Z`, 공백의 advance는 모두 1000 units다. 따라서 10px Ahem에서 각 non-collapsed glyph/space advance는 10px이다. 공백을 CSS normal white-space 규칙대로 하나로 collapse한 문자열을 `s`, 공백으로 나눈 단어를 `words(s)`라 하면 target scene의 독립 기대값은 다음과 같다.

- text min-content inline contribution = `10px × max(word length)`
- text max-content inline contribution = `10px × collapsed string length`
- natural `24×16px` image는 다른 column의 같은 row에 있어 row contribution은 `max(text contribution, 16px)`
- fixed columns `[50px, 24px]`, root width `74px`, zero gap/edges를 유지한다.

| Scene ID | Content | Track | `TextIntrinsicLogicalSizes` inline/block pair (px) | Expected row (px) |
| --- | --- | --- | ---: | ---: |
| `single-token-min-content` | `ABC` | min-content | `30/10` | 30 |
| `single-token-max-content` | `ABC` | max-content | `30/10` | 30 |
| `two-words-min-content` | `A BC` | min-content | `20/20` | 20 |
| `two-words-max-content` | `A BC` | max-content | `40/10` | 40 |
| `three-words-min-content` | `AB CDE F` | min-content | `30/30` | 30 |
| `three-words-max-content` | `AB CDE F` | max-content | `80/10` | 80 |
| `collapsed-whitespace-min-content` | `  A   9Z  ` | min-content | `20/20` | 20 |
| `collapsed-whitespace-max-content` | `  A   9Z  ` | max-content | `40/10` | 40 |
| `spaces-only-min-content` | `   ` | min-content | `0/0` | 16 |
| `spaces-only-max-content` | `   ` | max-content | `0/0` | 16 |
| `spaces-only-no-image-min-content` | `   ` | min-content | `0/0` | 0 |
| `spaces-only-no-image-max-content` | `   ` | max-content | `0/0` | 0 |

위 논리 pair의 block 값은 현재 `TextIntrinsicLogicalSizes` API 회귀값이다. 현 구현은 min pair의 block 값을 `break_all_lines(Some(0px))`에서 nonempty line의 line-height 합으로, max pair를 unconstrained layout height로 계산한다. 이는 CSS Box Sizing §2.1의 block-container `min-content block size`와 동일하다는 주장이 아니다. 그 규범 개념은 max-content block size와 동등하다. 이 slice는 기존 API 값을 유지하고 CSS Grid row contribution에는 inline component만 사용한다. Min pair의 기존 0px probe에서 각 공백 분리 단어가 한 line으로 계산되므로 block 회귀값은 nonempty word 수 × 10px이고, max pair는 한 line이라 10px이다. 공백만 있는 text는 normalization 뒤 빈 text가 돼 API pair `0/0`을 반환한다. Image가 있는 두 scene에서는 natural image가 row를 16px로 유지하며, 별도 image 없는 min/max control은 Chrome row `0px`로 text contribution 0을 직접 대조한다.

각 target scene의 Chrome computed row, root height, item frame이 이 계산과 일치한다. Chrome은 logical pair를 직접 관측하지 않으며, 그 pair는 위에서 설명한 현재 text-layout API 회귀 기대값이다. 두 image 없는 whitespace control을 포함한 Chrome 12/12 관측은 exact scene ID·content·문자 집합·좌표 확인과 함께 `validate-precomparison.py`로 재현한다. Chrome observation은 기준 구현의 관측일 뿐 규범 기대값을 만들지 않는다.

## 구현 경계와 검증

1. `supports_vertical_intrinsic_profile`을 정확한 문자열 비교 대신 위 입력 profile 검사로 바꾼다. 비어 있지 않은 ASCII 영숫자와 U+0020만 허용하고 unsupported input은 기존 `UnsupportedVerticalIntrinsicProfile`로 fail closed 한다. 표준 논리축 mapping, source/style revision과 font registry identity는 그대로 보존한다.
2. `spinon-text-layout` 회귀에서 다섯 문자열의 min/max logical inline/block size, font registry revision/token과 두 번의 line-break 결과를 확인한다. non-ASCII, punctuation/hyphen, tab/newline, RTL, 잘못된 font·line-break, inline style·ancestor negative control을 거부한다. `LineBreakMode`에는 CSS `auto` variant가 없으므로 text-layout API에서는 strict 입력을 negative control로 쓰고, computed CSS `line-break:auto`의 허용 여부는 projection/runtime 경계에서 검사한다.
3. Chrome 표에서 image가 있는 열 target scene만 기존 opt-in C11.4 Android/iOS runtime fixture에 추가한다. 두 image 없는 spaces-only row control은 Chrome oracle 전용이며 모바일 runtime scene 수에 넣지 않는다. 열 target ID는 runtime·FFI mapping과 Android/iOS allowlist에서 같은 입력을 가리켜야 한다. 신규 10개 target의 FFI/runtime HTML text style은 `line-break:normal`을 명시하고 해당 route validator가 computed value도 `normal`로 확인한다. 기존 4개 orthogonal 회귀 scene은 이전 profile의 `auto` 값을 별도 exact match로 유지한다. 각 source revision에서 target logical pair, contribution provenance, `Columns → Rows` trace와 row를 root/item frame과 함께 대조한다. 기존 #611/#620의 8 direct text/image scene도 양 플랫폼에서 재실행한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 성공·실패 장면을 확인한다. 실행 binary/app hash를 최종 source에 연결한다. 시뮬레이터 결과는 실기기나 hardware GPU 증거로 확대하지 않는다.
5. fixture route/symbol/payload의 기본 제품 artifact 부재와 opt-in positive control을 확인한다. 관련 focused Rust/runtime/FFI tests, `cargo fmt`, workspace all-target Clippy `-D warnings`, Android/iOS fixture app builds를 기록한다. 실행하지 않은 항목은 통과로 표시하지 않는다.
6. 구현/runtime 동작은 계획 검토와 다른 실패 경로 20개로 검토한다. 수정된 결함의 영향 검토를 반복한다. PR diff는 계획 및 구현 검토와 별개인 30개 관점으로 확인한다.
7. `spec/internal/0059-c11-grid-layout.md`, C11.4 계획과 공개 roadmap에 이 slice의 입력 경계·근거·남은 범위를 기록한다. C11.4 checkbox 및 공식 완료 집계 `72/213`을 유지한다.

## 병렬 작업 경계

계획 PR 병합 뒤 target 10개 ID와 기대값을 공통 runtime 계약으로 고정하고, 아래 경로를 서로 다른 소유자에게 배정한다. 같은 파일을 두 소유자가 수정하지 않으며, 공유 입력 계약은 이 계획의 target 표로 고정한다.

| Lane | 소유·통합 파일 | 범위 / 상태 |
| --- | --- | --- |
| 1. text-layout | `crates/spinon-text-layout/src/engine/intrinsic.rs`, `crates/spinon-text-layout/src/tests.rs` | vertical intrinsic profile guard 및 unit 회귀 — 구현 완료, 단위 검사 통과 |
| 2. layout/runtime bridge | `crates/spinon-style-to-layout/src/projection.rs`, `crates/spinon-style-to-layout/src/c11_4_track_sizing_areas_tests/orthogonal_writing_mode.rs`, `crates/spinon-layout/src/intrinsic_measurement.rs`, `crates/spinon-layout/src/calc_tree/text.rs`, `crates/spinon-layout/src/calc_tree/intrinsic_measurements.rs`, `crates/spinon-layout/src/grid_text_stretch_tests.rs`, `crates/spinon-layout/src/c15_text_layout_tests.rs`, `crates/spinon-layout/src/intrinsic_size_tests.rs`, `crates/spinon-layout/src/lib.rs`, `crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs`, `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_mixed/orthogonal_rows/` 하위 파일, `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/orthogonal_rows.rs`와 `orthogonal_rows/` 하위 파일 | computed-style gate, axis mapping, intrinsic identity 전달, non-Copy 진단·기존 테스트 caller 조정, runtime bridge 및 회귀 — 구현 완료, focused 검사 통과 |
| 3. FFI/platform | `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs`, 신규 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/orthogonal_text_intrinsic.rs`, `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs`, `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java`, `platforms/ios/Sources/C0410RuntimeGpuDemo.swift` | scene dispatcher, source-identity trace validation, Android/iOS route와 allowlist — 구현 및 시뮬레이터 검증 완료 |

실제 변경 경로는 위 lane의 통합 작업을 따라 25개다. layout/runtime lane의 re-export·clone·기존 test pattern 조정은 opaque font-registry identity를 전달하면서 생긴 필수 compile/API 지원 변경이며, 별도 제품 범위를 추가하지 않는다. 이 목록은 수정하지 않은 runtime fixture 등록 경로를 변경 파일인 것처럼 세지 않는다.

Image 없는 두 Chrome control은 모바일 route에 추가하지 않는다. Runtime bridge와 모바일 route의 ID·문자열·기대값은 target 표에서만 가져온다. 전체 runtime fixture 실행과 simulator 검증은 세 lane을 통합한 뒤 순차 수행하며, text-layout guard와 layout/runtime bridge가 통합되기 전에는 runtime 성공을 주장하지 않는다.

## 범위 밖과 완료 판정

이 slice의 성공은 ASCII 영숫자와 U+0020으로 된 Ahem paragraph의 vertical-rl logical intrinsic measurement subset만 넓힌다. Other fonts/scripts, vertical-lr/upright orientation, line-break punctuation, multi-source/inline descendants, nested/anonymous general item matrix, arbitrary Grid author CSS, WPT conformance, glyph/PNG pixel painting, Android physical/hardware GPU, iOS compositor-after readback, 전체 C11.4/CSS Grid 완료를 뜻하지 않는다.

완료에는 다섯 target 문자열 각각의 Chrome 156 관측·독립 Ahem 계산, image 없는 whitespace 0px controls, 정확한 scene ID 매핑, text-layout/Rust 회귀, opt-in fixture에서의 source-bound logical inline contribution과 trace, unsupported input 거부, 양 platform의 신규 10+회귀 12 scene, HTML style의 `line-break:normal` parity, artifact 경계, 구현 검토 20개 및 별도 PR diff 검토 30개가 필요하다.

## 사전 비교 자료

- [Chrome fixture](../tests/fixtures/css/c11/grid-orthogonal-text-generalization-c114-precomparison.html)
- [Agent Browser observation, screenshot, hashes, 종료 기록](../spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/README.md)
- [독립 계산 검증기](../spec/internal/evidence/c11-4-orthogonal-text-generalization-precomparison-2026-10-25/validate-precomparison.py)
- [계획 실패 관점 검토 20개](./c11-4-orthogonal-text-intrinsic-generalization-plan-review-20.md)

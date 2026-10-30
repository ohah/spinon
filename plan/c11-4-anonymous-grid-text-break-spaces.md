# C11.4 anonymous Grid text `white-space:break-spaces` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [CSS 적합성 계약](../spec/0001-conformance.md)

**상태:** 계획 PR [#872](https://github.com/ohah/spinon/pull/872)은 2026-10-30T19:22:58Z Rebase and merge됐고 merge SHA는 `908e35f0e480f42fb87b75550866c62999ad6642`다. 계획대로 구현 후보를 같은 checkout에서 순차 진행했다. 현재 제한 `BreakSpaces` 후보는 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 26/26씩 통과했고 구현·runtime 검토를 마쳤다. 구현/runtime 실패 관점 20개와 최종 source diff 실패 관점 30개를 마쳤다. 구현 PR 생성과 병합은 이어서 진행한다. C11.4/C11 공식 완료와 `72/213`은 미완료로 유지한다.

**계획 검토:** 서로 다른 실패 경로 20개는 [계획 검토 기록](./c11-4-anonymous-grid-text-break-spaces-plan-review-20.md)에, 계획 PR diff 30개는 [PR diff 검토 기록](./c11-4-anonymous-grid-text-break-spaces-plan-pr-diff-review-30.md)에 별도로 남긴다.

## 목적과 계약 경계

CSS Text 4의 `break-spaces`는 보존 공백을 줄 끝에서 hang하지 않고 공백마다 soft-wrap 기회를 둔다. 이는 기존 `PreWrap`의 조건부/무조건 hanging 동작과 다르며, `XX  XX` 고정 입력에서 max-content는 60px, min-content는 30px이다. 기존 `PreWrap` min-content control은 20px이다. 독립 기대값과 Agent Browser 관측은 [사전 비교 자료](../spec/internal/evidence/c11-4-anonymous-grid-text-break-spaces-precomparison-2026-10-31/README.md)에 따로 기록했다.

이 제한 구현은 현재 opt-in `RuntimeBlockFlexGridTrackSizingV1` C11.4 anonymous-text fixture에서만 `WhiteSpaceMode::BreakSpaces`를 추가한다. 대상은 단일 paragraph의 direct anonymous Grid text, Ahem 10px/10px, weight 400, horizontal-tb/LTR, `order:0`, normal ligatures와 고정된 columns/rows다. runtime scene source는 `white-space-collapse:break-spaces;text-wrap-mode:wrap`을 계산 스타일로 제공하고 validator가 두 값을 확인한다. text 입력은 ASCII `X`, U+0020, LF 및 계획에 열거한 source split만 받는다. tab, inline ancestor/run, 다른 writing mode, author `tab-size`, 임의 글꼴·문자열, 일반 CSS cascade/root HTML parser는 이 slice에서 열지 않으며 지원 profile 밖 입력은 거부한다.

구현은 `spinon-text-layout`의 기존 source-preserving segment-break normalization을 `PreWrap | BreakSpaces` 공통 경로로 정리하고, Parley 0.12의 `WhiteSpaceCollapse::BreakSpaces`를 사용해 non-hanging/break-after-each-space 동작을 그대로 보존한다. PreWrap tab expansion이나 normal/nowrap fast path는 BreakSpaces에 적용하지 않는다. tab 입력은 해당 mode에서 fail-closed한다. 새 mode가 기존 Normal/NoWrap/PreLine/PreWrap 입력과 provenance, error 경계를 바꾸지 않아야 한다.

## 고정 scene과 독립 기대값

Agent Browser/W3C 기준 12개 장면은 [expected-values.json](../spec/internal/evidence/c11-4-anonymous-grid-text-break-spaces-precomparison-2026-10-31/expected-values.json)에 있다. 앱 matrix에는 다음 10개 `break-spaces` 장면을 추가해 기존 16개와 함께 총 26개를 검사한다.

| Runtime scene | 입력·track | 기대 결과와 실패 조건 |
| --- | --- | --- |
| `anonymous-break-spaces-max-content` | `XX  XX`, `max-content` | min/max intrinsic `[30,60]`; columns `[60]`, rows `[10]`; source에 두 공백 보존 |
| `anonymous-break-spaces-comment-split-max-content` | `"XX "` + `" XX"` source 두 개 | anonymous item 한 개, 두 `TextSource` identity 유지, columns `[60]` |
| `anonymous-break-spaces-min-content` | `XX  XX`, `min-content` | min-content 30; columns `[30]`, rows `[20]` |
| `anonymous-break-spaces-soft-wrap-25` | `XX  XX`, fixed 25px column | columns `[25]`, rows `[20]`; 30px 두 line advance, hanging advance 0 |
| `anonymous-break-spaces-forced-break-max-content` | `XX  \nXX`, `max-content` | 강제 break 전 공백 두 개 non-hanging; columns `[40]`, rows `[20]` |
| `anonymous-break-spaces-forced-break-min-content` | `XX  \nXX`, `min-content` | columns `[30]`, rows `[30]`; 세 line과 마지막 forced-break source provenance |
| `anonymous-break-spaces-trailing-soft-wrap-25` | `XX  `, fixed 25px column | trailing spaces가 hang하지 않고 wrap; columns `[25]`, rows `[20]` |
| `anonymous-break-spaces-trailing-max-content` | `XX  `, `max-content` | 뒤 공백 두 개 포함 columns `[40]`, rows `[10]` |
| `anonymous-break-spaces-whitespace-only` | 공백 두 개만 있는 direct text | anonymous text item과 paragraph 없음, 0×0 content 결과 |
| `anonymous-break-spaces-whitespace-before-block` | 공백 두 개 뒤 10×10 block | 공백은 cell을 소비하지 않고 marker가 첫 cell `(0,0)`에 배치 |

모든 장면에서 HostDocument source/revision identity, intrinsic contribution, Columns/Rows track trace, root/item frame, draw/present와 retry 0을 검사한다. Chrome `Range` fragment는 line grouping 보조 자료이며 raster/pixel 증거가 아니다.

## 구현·검증 순서

1. 계획 문서를 20개 서로 다른 실패 관점으로 검토하고, 계획 PR 자체는 별도 30개 diff 관점으로 검토한다.
2. 계획 PR 병합 후 같은 checkout에서 구현 branch를 순차로 만든다. `WhiteSpaceMode` 및 exhaustive mapping, normalized text/provenance, Parley mode, anonymous Grid profile gate에 새 mode를 추가한다. 새 Rust test module은 `crates/spinon-text-layout/src/tests/`와 `crates/spinon-layout/src/host_document_tests/c11_4_anonymous_text/`에 둔다.
3. FFI scene source, runtime computed-style validator, fixed scene matrix, `capture-c11-4-auto-repeat-neighbors-simulator.mjs` expected geometry/trace validator와 `fixture-module-boundary.test.mjs` allowlist를 함께 갱신한다. 기존 C11.4 scene ID count와 default/fixture-test artifact 경계를 다시 확인한다.
4. focused text-layout, layout HostDocument, runtime fixture, FFI fixture tests와 CSS fixture-module boundary 검사를 실행하고 Clippy/rustfmt를 확인한다. 실패는 초기/수정/최종 결과와 정확한 filter 및 count로 구분한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 새 10개 및 기존 16개를 각각 26/26 실행한다. build/app hash가 일치하고 root/item frame·track/contribution trace·draw/present가 맞으며 retry가 없어야 한다. PR 본문에는 앱 결과 screenshot을 연결할 수 있으면 Android와 iOS 각 한 장을 포함한다. 시뮬레이터 결과를 실기기, hardware GPU, glyph pixel, iOS compositor-after 증거로 확대하지 않는다.
6. product/fixture-test static archive에서 새 scene marker·fixture source/FFI export가 기본 artifact에서 빠지고 fixture-test positive control에만 포함되는지 확인한다.
7. 구현/runtime 동작을 계획 리뷰와 다른 20개 실패 관점으로 검토하고, PR 최종 diff를 별도 30개 관점으로 검토한다. 수정이 있으면 영향을 받은 검토 관점을 반복한다. Rebase and merge 전 PR head patch와 검토 대상 diff가 일치하는지 확인한다.

## 제외 범위와 완료 판정

일반 authored CSS admission, computed shorthand parser, root HTML parity, `tab-size`, tabs, arbitrary text/font/script, inline formatting contexts, WPT 실행, glyph ink equality, physical Android/iOS, Android hardware GPU, iOS compositor 이후 pixels, 전체 C11.4/C11은 제외한다. 공개 API 명세가 없는 이 고정 fixture slice만으로 C11.4 또는 C11 checkbox를 체크하지 않으며 공식 완료 `72/213`도 올리지 않는다. 구현과 명세 범위가 다르면 fail-closed를 유지하고 새 계약을 임의로 만들지 않는다.

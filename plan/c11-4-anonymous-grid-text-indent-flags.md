# C11.4 anonymous Grid text-indent `hanging`·`each-line` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** Agent Browser Chrome 156의 20-scene 관측, Ahem advance에서 도출한 독립 기대값, 계획 실패 관점 검토 20개와 구현 실패 관점 검토 20개를 고정했다. Stylo flags fallback이 예상 진단 없이도 허용되던 경계를 찾아 exact diagnostic gate로 좁혔고 runtime 9/9·text-layout 1/1·FFI fixture source 2/2 및 세 패키지 feature-scoped Clippy `-D warnings`를 통과했다. 최종 Rust 코드의 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 검증은 신규 flags 20/20, 기존 positive `text-indent` 18/18, positive 회귀를 포함한 negative matrix 30/30이 각각 통과했다. 초기 runner가 layout summary에서 수집을 끝내던 문제를 고쳐 여섯 행렬 전부에서 scene별 예상 root frame draw/present, Surface 크기, generation과 retry 수를 기록했다. 여섯 행렬 136개 scene의 superseded retry는 0이었다. iOS 강제 줄바꿈의 기대 `maxContentHeight`와 negative matrix의 max-content `lineCount` 생성 오류도 수정했다. Product archive에는 flags scene ID·C11.4 host export 0개, fixture-test positive control에는 20개 scene ID와 export 2개가 확인됐다. 최종 PR diff 실패 관점 30개를 마쳤다([기록](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/pr-diff-review-30.md)). 내부 계약 0059에 고정 지원 profile을 명세했다. 구현 PR [#959](https://github.com/ohah/spinon/pull/959)은 2026-11-01T02:06:32Z Rebase and merge로 병합됐고 merge SHA는 `590f5946e97e0252113abf34b23ae31b13819acf`다. PR check rollup은 비어 있어 CI 통과를 주장하지 않는다. C11.4/C11 checkbox와 공식 프로젝트 집계 `72/213`은 그대로 둔다. [계획 검토 20개](./c11-4-anonymous-grid-text-indent-flags-plan-review-20.md) · [구현 검토 20개](./c11-4-anonymous-grid-text-indent-flags-implementation-review-20.md) · [사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-precomparison-2026-11-01/README.md) · [신규 실행·archive evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-runtime-2026-11-01/README.md) · [positive 실행 evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-runtime-2026-11-01/README.md) · [negative 실행 evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-negative-indent-runtime-2026-11-01/README.md)

## 결손과 목표

C11.4 anonymous direct-text Grid opt-in adapter는 computed `text-indent`의 길이만 `ParagraphStyle.text_indent`에 전달한다. 이 adapter는 `hanging`과 `each-line`이 포함된 computed serialization을 거부한다. Text layout도 `Parley::Layout::set_text_indent(value, IndentOptions::default())`를 호출하고, min/max-content 계산은 현재 line index가 0인지로만 들여쓰기를 더한다. 따라서 첫 formatted line, 강제 줄바꿈 뒤, soft wrap 뒤에 indent가 달라지는 유효 CSS 동작을 이 profile에서 실행하지 못한다.

CSS Text Level 3 §8.1은 `each-line`이 첫 줄과 forced line break 뒤의 줄에 indent를 적용하고 soft wrap 뒤에는 적용하지 않는다고 정의한다. `hanging`은 적용되는 line 집합을 뒤집는다. Parley 0.12.0은 `IndentOptions { each_line, hanging }`와 line별 실제 `LineMetrics.indent`를 제공한다. 이 계획은 해당 computed flags를 같은 computed-style snapshot에서 읽어 text layout 및 Grid intrinsic contribution까지 연결한다.

## 고정 규범 입력과 독립 기대값

기준은 [CSS Text Level 3 §8.1](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#text-indent-property), [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes), [CSS Grid Level 1 §11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)이다. Ahem 10px에서 `X`와 U+0020 공백의 advance는 각 10px이다. 입력은 LTR·horizontal-tb·Ahem 10px/400·line-height 10px·zero gap/edge로 고정한다.

| 입력 | `white-space` | Track | `0px` | `10px` | `10px each-line` | `10px hanging` | `10px hanging each-line` |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| `XXX X` | normal | min-content | 30 | 40 | 40 | 30 | 30 |
| `XXX X` | normal | max-content | 50 | 60 | 60 | 50 | 50 |
| `XXX\nX` | pre-line | min-content | 30 | 40 | 40 | 30 | 30 |
| `XXX\nX` | pre-line | max-content | 30 | 40 | 40 | 30 | 30 |

모든 수치는 CSS px이다. Soft-wrap case의 min-content text advances는 `[30,10]px`, max-content는 `50px`이다. Forced-break case의 line advances는 `[30,10]px`이며 두 줄의 y origin은 `[0,10]px`이다. 각 flag 값이 line별 x origin과 maximum line contribution에 미치는 영향은 기계 판독 [독립 기대값](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-precomparison-2026-11-01/expected-values.json)에 case별로 고정했다. Chrome CSSOM/Range 관측은 기대값 산출식과 분리했다.

## 제한 fixture 범위

- 기존 `RuntimeBlockFlexGridTrackSizingV1` anonymous direct-text opt-in route만 확장한다. HTML `div` Grid root의 direct anonymous text 단일 source, 0 edge/gap, Ahem 10px/400, LTR·horizontal-tb만 허용한다.
- `text-indent` computed value는 정확히 `0px`, `10px`, `10px each-line`, `10px hanging`, `10px hanging each-line`인 5개 profile만 허용한다. `hanging`과 `each-line` 순서는 computed CSSOM serialization을 확인하며 scene ID로 flags를 추측하지 않는다.
- `XXX X`와 `XXX\nX`, `white-space:normal|pre-line`, `min-content|max-content`를 조합한 20개 scene을 기존 Android/iOS opt-in matrix에 추가한다. Computed style, typed flags, HostDocument/style revision, line indent/origin/advance, intrinsic contributions, Grid tracks, root/item frames 및 draw/present를 동일 scene ID에 연결한다.
- `ParagraphStyle`은 길이와 두 boolean을 검증 가능한 typed value로 보존하고 default는 기존처럼 `0px`, 두 flag false다. Text builder는 Parley 0.12.0 `IndentOptions`로 값을 전달한다. intrinsic min/max 계산은 `line_index == 0`만으로 indent를 덧붙이지 않고, 실제 line별 `LineMetrics.indent`와 content advance를 각각 합산해 track contribution을 계산한다. `LineMetrics.advance`에 indent가 이미 포함됐다고 중복 합산하지 않는다.
- 기존 `text-indent` 0/5/10px 18 scene과 -5/-10px 12 scene, 일반 text layout, `pre-line`, normal/nowrap/pre-wrap/break-spaces, letter/word spacing 및 line-break 회귀를 보존한다. Negative indent는 별도 기존 profile로 계속 검증한다.
- Computed property 누락/stale revision, 등록되지 않은 scene, 부정확한 keyword/value 조합, 바뀐 text/white-space/tree/track/axis는 layout publication 전에 거부하며 frame·trace·draw를 남기지 않는다.

## 실행 및 적대 검토 관문

1. 고정 HTML fixture, Chrome 156 관측, Ahem/font hash와 규범 기반 독립 기대값을 검증한다. `node spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-precomparison-2026-11-01/verify-observation.mjs`는 20개 case의 computed serialization, track/root 크기, line origin·advance를 비교한다.
2. 구현 변경 뒤 text-layout 단위 검사에서 `IndentOptions`의 first/forced/soft-wrap line semantics와 `LineMetrics.indent`가 intrinsic min/max contribution에 정확히 한 번 들어가는지 확인한다. 기존 양수·음수 indent, 빈 text, line-height 및 whitespace regressions를 포함한다.
3. C11.4 fixture validator는 exact root/item/source identity, computed serialization, flags, white-space, axis, sizing function과 revision을 검증한다. Unknown flag, declaration/value mismatch, stale style, mutated tree 및 미등록 ID의 failure atomicity를 확인한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 20개 신규 장면과 기존 `text-indent`/anonymous text 회귀를 최종 source hash 기준으로 실행한다. Track, line trace, root/item frame, draw/present, retry count 및 설치/build hash를 보존한다. AVD software renderer와 iOS compositor 제출 전 drawable 증거를 한계를 적어 분리한다.
5. 기본 product/static archive에서 fixture symbols·scene IDs·payload가 빠지고 fixture-test opt-in positive control에 들어가는지 확인한다. Archive 결과를 전체 APK/IPA 제외로 확대하지 않는다.
6. 계획 자체 실패 관점 20개는 별도 문서에서 유지한다. 구현 뒤에는 겹치지 않는 신규 runtime/code 실패 관점 20개, 최종 구현 PR diff에는 저장소 규칙의 서로 다른 실패 관점 30개를 적용한다. 발견된 결함은 수정 후 영향받은 관점을 다시 검증한다.
7. 계획 PR과 구현 PR은 분리하고 각 PR의 최신 head, check report, merge state를 확인한다. 체크가 없으면 CI 통과로 쓰지 않는다. 구현 merge 뒤에만 내부 계약 0059, `spec/STATUS.md`, 상위 계획 및 공개/Tailnet roadmap을 실제 결과·남은 한계와 동기화한다.

## 범위 밖과 완료 판정

`<length-percentage>` 일반 grammar, arbitrary value, `-30px`, percentage intrinsic sizing, inline runs/multiple text sources, bidi/RTL·vertical writing, 다른 fonts/scripts, general HTML/CSS admission, public CSS support, glyph raster equality, full WPT, 실기기·hardware GPU, iOS compositor 이후 pixel, 전체 C11.4 또는 C11 완료는 이 계획에 포함되지 않는다. 이 제한 slice의 성공만으로 parent checkbox 또는 공식 `72/213`을 올리지 않는다.

## 기준 자료

- [Chrome 156 precomparison·독립 기대값·해시·화면](../spec/internal/evidence/c11-4-anonymous-grid-text-indent-flags-precomparison-2026-11-01/README.md)
- [고정 HTML 입력](../tests/fixtures/css/c11/c11-4-anonymous-grid-text-indent-flags-precomparison.html)
- [Parley 0.12.0 `IndentOptions`](https://docs.rs/parley/0.12.0/parley/layout/struct.IndentOptions.html)
- [CSS Text Level 3 §8.1](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#text-indent-property)

## 계획 검토

구현 전에 서로 다른 20개 실패 관점의 검토를 마쳤다([검토 기록](./c11-4-anonymous-grid-text-indent-flags-plan-review-20.md)). 이 검토는 구현/runtime 20개 및 구현 PR diff 30개를 대신하지 않는다.

## 계획 PR 변경 검토

계획과 문서 PR 변경분을 대상으로 서로 다른 실패 관점 30개를 확인했다([검토 기록](./c11-4-anonymous-grid-text-indent-flags-plan-pr-diff-review-30.md)). 이는 제품 구현 정확성이나 simulator 결과를 증명하지 않는다.

# C11.4 anonymous Grid text computed `line-break` 제한 profile

**Parent:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [inline text 계약 0070](../spec/internal/0070-c15-1-inline-text-layout.md) · [CSS Text Level 3 §5.2](https://www.w3.org/TR/css-text-3/#line-break-property)

**상태:** 제한 opt-in fixture 구현 PR [#945](https://github.com/ohah/spinon/pull/945)이 2026-10-31T21:11:12Z Rebase and merge됐다(merge SHA `9eb8cea570fce5a0eb2da423ef37eac2e4ed3c1d`). Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 고정 5개 scene이 각각 5/5 통과했다. [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-runtime-2026-11-01/implementation-runtime-review-20.md), [PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-runtime-2026-11-01/pr-diff-review-30.md), [실행 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-runtime-2026-11-01/README.md). 제한 fixture 결과이며 C11.4/C11 전체 지원이 아니다. 공식 완료 `72/213`은 유지한다.

## 문제와 목표

C11.4 opt-in anonymous Grid direct-text adapter는 최근 computed `word-break`를 연결했지만 해당 경로에서 computed `line-break`는 여전히 `auto`로 고정하고 `LineBreakMode::Strict`를 사용한다. 일반 root HTML style adapter는 `normal`과 `strict`를 이미 typed `LineBreakMode`로 변환하지만, anonymous text fixture allowlist는 이를 다루지 않는다. 이 계획은 기존 fixture 경계에서 computed `line-break:normal|strict`가 같은 style owner의 language metadata와 함께 Parley text intrinsic sizing 및 Grid track 계산에 도달하는지 검증한다.

검증용 입력은 `lang="ja"`인 direct-text Grid의 `文〜文`이다. CSS Text Level 3은 `line-break:normal`에서 중국어·일본어 writing system의 U+301C WAVE DASH 앞 줄바꿈을 허용하고, 그 밖의 writing system에서는 금지한다. `strict`는 해당 기회를 금지한다. Chrome 156은 Noto Sans KR 10px에서 일본어 `normal` min-content column 10px/auto row 30px, `strict` 20px/20px를 관찰했다. 같은 문자열의 `lang="en"; line-break:normal`은 20px/20px다. 이는 locale 전달 여부와 min-content sizing을 동시에 구분한다.

구현은 이 고정 profile과 fixture-test opt-in 경로에 한정한다. 일반 author CSS admission, 제품 HTML의 language 상속, 일반 `line-break` 호환, 전체 C11.4/C11 완료로 확대하지 않는다. 현재 `LineBreakMode::{Normal, Strict}`, Parley 0.12.0의 locale별 segmentation, 고정 Noto Sans KR font bytes를 재사용한다. 출시 전 내부 계약 숫자 버전 `0.1.0`과 공식 완료 `72/213`을 유지한다.

## 규범과 독립 기대값

[CSS Text Level 3 §5.2](https://www.w3.org/TR/css-text-3/#line-break-property)는 `normal`을 가장 보편적인 줄바꿈 규칙으로 정의하고, U+301C 앞 break를 중국어·일본어 writing system에만 허용한다. `strict`는 더 엄격한 규칙을 사용한다. [CSS Sizing Level 3 §5.1](https://www.w3.org/TR/css-sizing-3/#intrinsic-sizes)의 min-content contribution은 허용된 soft-wrap opportunity로 분리되지 않는 가장 긴 segment의 폭이다. [CSS Grid Level 1 §11](https://www.w3.org/TR/css-grid-1/#algo-track-sizing)은 이를 `min-content` track sizing에 사용한다.

Chrome 156 관찰, 입력 CSS, 글꼴 hash, 고정 기대값은 [사전 비교 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-precomparison-2026-11-01/README.md)에 있다. `NotoSansKR-VF.ttf` SHA-256은 `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76`이다. U+6587 `文`과 U+301C `〜`의 advance는 각각 1000/1000 em이고 10px/400 profile에서 각 글리프는 10px다. max-content는 30px다.

| Scene | `lang` | computed `line-break` | Grid column | expected column track | expected row track | root frame |
| --- | --- | --- | --- | ---: | ---: | ---: |
| `line-break-ja-normal-min-content` | `ja` | `normal` | `min-content` | 10px | 30px | 120×30px |
| `line-break-ja-normal-max-content` | `ja` | `normal` | `max-content` | 30px | 10px | 120×10px |
| `line-break-ja-strict-min-content` | `ja` | `strict` | `min-content` | 20px | 20px | 120×20px |
| `line-break-ja-strict-max-content` | `ja` | `strict` | `max-content` | 30px | 10px | 120×10px |
| `line-break-en-normal-min-content` | `en` | `normal` | `min-content` | 20px | 20px | 120×20px |

Text intrinsic expectations at 10px line-height are, respectively: Japanese normal min/max inline sizes 10/30px with min/max block size 30/10px; Japanese strict 20/30px with 20/10px; English normal 20/30px with 20/10px. The direct anonymous text item occupies the selected column track; its wrapping determines the `auto` row. Computed keyword and column/row tracks must match exactly; text metrics/frame comparisons use a 0.05 CSS px tolerance for the existing font/layout conversion boundary.

## 고정 입력과 fixture 경계

- Runtime source contains a `div` Grid style owner with one direct Text child `文〜文`, `lang="ja"` (or `en` for the negative tailoring control), `font-family:"Noto Sans KR"`, 10px/400 font, 10px line-height, horizontal-tb/LTR, `white-space-collapse:collapse`, `text-wrap-mode:wrap`, `word-break:normal`, `overflow-wrap:normal`, zero letter/word spacing and zero text indent.
- The Grid has width 120px, auto height, zero margin/padding/border/gap, one `min-content` or `max-content` column, and an auto row. The fixture contains no child element, inline element, second text node, anonymous-block split or nested Grid.
- The exact Grid style owner has one unnamespaced HTML `lang` attribute whose value matches the scene. XML `lang`, missing/empty/invalid language, a language found only on an ancestor, duplicate or conflicting language metadata and owner substitution are rejected. This is an explicit fixture input rule, not a claim that HTML language inheritance is unsupported by the browser.
- The computed style snapshot and HostDocument snapshot must share the current document/style revision. `line-break`, language, text, font, track and scene ID must match the fixed record before any frame/layout trace/draw is published.
- Existing `line-break:auto` scenes retain their current `LineBreakMode::Strict` mapping. New scenes accept only their exact computed `normal` or `strict` value; `loose`, `anywhere`, missing values and scene/property mismatches fail closed. Do not change generic style parsing or map CSS `auto` globally.
- The fixed Noto Sans KR bytes are registered only when these or existing word-break scenes are active under `c11-4-grid-track-sizing-fixture-test`. No new product FFI route, general font fallback, system font dependency or public API is added.

## 구현과 검증 순서

1. 계획과 oracle를 서로 다른 실패 관점 20개로 검토한다. 누락을 발견하면 입력·기대값·범위를 구현 전에 보정한다. 계획 PR diff는 별도의 20개 관점으로 검토한다.
2. 계획 PR을 `Rebase and merge`한 뒤 같은 checkout에서 최신 `main`을 기준으로 순차적으로 구현 branch를 만든다. 병렬 구현/worktree는 사용하지 않는다.
3. `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/anonymous_text_items/` 아래 computed `line-break` source module에 5개 unique scene을 추가한다. source는 root owner `lang`, 정확한 property 값, Noto font, track keyword, text와 unique root id를 직렬화한다. 기존 word-break scene 및 source generator 경계를 회귀 검사한다.
4. `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_anonymous_text/`에 별도 `line_break` scene/validation module을 둔다. exact owner language와 computed `line-break`, scene ID, text/font/track, style revision을 검사해 `LineBreakMode`와 `ParagraphStyle.locale`을 같은 anonymous paragraph에 전달한다. 기존 word-break locale 검증과 자동 언어 추정을 변경하지 않는다.
5. anonymous text `prepare`에서 새 scene allowlist를 인식하고 기존 Noto font bytes를 등록한다. text profile validator가 기존 scene에는 `auto`, 새 scene에는 scene별 `normal|strict`만 기대하게 한다. `ParagraphStyle`의 다른 defaults, line-break auto behavior, word-break, line-height, white-space wrapping을 보존한다.
6. `spinon-text-layout`에 Noto Sans KR과 `lang=ja`를 쓰는 focused regression을 추가한다. Japanese normal/strict 및 English normal의 intrinsic min/max inline/block sizes를 독립 기대값과 비교한다. Malformed locale는 기존 validation에서 거부되어야 한다. Parley 0.12.0의 `LineBreak`와 `Locale` 처리로 예상한 break opportunity를 source와 고정 테스트에서 대조한다.
7. Layout/runtime/FFI tests에서 다섯 장면 모두 computed property, style owner/revision, text intrinsic sizes, Grid column/row tracks 및 root/item frame을 1:1로 검사한다. `loose`, `anywhere`, auto, language mismatch, missing/ancestor-only/XML `lang`, wrong owner/value/text/font/track, stale revision, hidden/extra child와 unknown scene mutation은 부분 layout/frame/draw 없이 거부한다.
8. fixture source boundary, default product archive와 fixture-test positive control을 별도로 확인한다. 제품 archive에 새 scene marker/fixture export/font payload가 없어야 하며 fixture-test archive에는 예상 scene과 이미 사용 중인 고정 font만 있어야 한다.
9. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 5/5 scene을 실행한다. frame/track/trace/draw/present, 설치 APK와 source artifact hash, 실패 scene의 publication 거부를 기록한다. Android software renderer와 iOS compositor 제출 전 Metal drawable 표본은 실제 기기/hardware GPU/compositor 이후 pixel 증거와 구분한다. Simulator 화면이 glyph pixels를 보여주지 않으면 이를 그대로 기록한다.
10. `cargo fmt --all -- --check`, 해당 `spinon-text-layout`/runtime/FFI tests, feature-scoped Clippy, fixture module-boundary 검사와 Android/iOS simulator builds를 실행한다. 실행하지 않은 범위는 통과로 기록하지 않는다.
11. 구현 뒤 코드와 실제 동작을 계획 검토와 겹치지 않는 새 실패 관점 20개로 검토한다. 구현 PR diff는 저장소 규칙에 따라 별도 30개 관점으로 검토한다. 결함을 수정하면 영향을 받은 관점을 다시 확인한다.
12. 구현 PR에는 Chrome 사전 비교 이미지와 가능한 Android/iOS 동작 화면을 포함한다. PR body의 이미지 URL이 실제로 열리는지 확인하고, simulator 화면이 glyph paint/compositor 픽셀을 증명하지 않는 경우 그 제한을 적는다. 현재 PR check rollup과 required check 설정을 조회한 뒤 Rebase and merge 상태, merge SHA와 최종 source를 검증한다.
13. 구현 후 내부 계약 0059에 고정 fixture profile과 failure boundary를 기록한다. 구현 PR #945 병합 뒤 `spec/STATUS.md`, 이 parent plan 및 공개 roadmap을 실제 상태에 맞추되 C11.4/C11 checkbox나 공식 `72/213`을 승격하지 않는다.

## 완료 판정

이 계획의 완료는 정확한 다섯 opt-in anonymous direct-text scenes에서 computed `line-break:normal|strict`, direct owner language, Parley locale, intrinsic min/max sizing, Grid row/column tracks 및 frame이 Chrome/CSS 규범 기반 독립 기대값과 일치하고, 거부 경계·기본/fixture artifact 분리·Android/iOS Simulator 근거·분리된 계획/구현/PR 검토가 모두 확인된 상태다. 일반 CSS admission, 임의 언어/글꼴/line-break 조합, WPT, glyph/surface paint 또는 전체 C11.4/C11 지원 완료는 포함하지 않는다.

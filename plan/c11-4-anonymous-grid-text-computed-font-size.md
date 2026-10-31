# C11.4 anonymous Grid text computed `font-size` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [CSS 적합성 계약](../spec/0001-conformance.md)

**상태:** 현재 C11.4 anonymous direct-text fixture는 computed `line-height`를 `ParagraphStyle`에 전달하지만 `font-size:10px`과 `ParagraphStyle.font_size:10.0`은 고정한다. Chrome 156 사전 비교와 Ahem font-table 산식을 고정했다. [비교 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-font-size-precomparison-2026-10-31/README.md). 구현과 Android/iOS Simulator 재검증은 아직 시작하지 않았다. 계획 PR은 C11.4/C11 체크와 공식 `72/213`을 바꾸지 않는다.

## 목적과 경계

같은 `RuntimeBlockFlexGridTrackSizingV1` computed-style snapshot의 root `font-size`를 anonymous direct-text Grid item의 `ParagraphStyle`에 연결해 inline min/max-content 기여가 지정된 computed font 크기에 따라 변하는지 확인한다. 현재 validator는 Grid root `font-size:10px`만 허용하고 `anonymous_paragraph_style`은 `font_size:10.0`을 고정한다. 이는 CSS font-size cascade나 일반 text style 지원이 아니라 기존 opt-in fixture의 두 입력 사이 연결 누락이다.

고정 profile은 HTML `div` Grid root, direct anonymous text `XX XX`, Ahem 400, horizontal-tb/LTR, `line-height:10px`, normal whitespace collapse/wrap, zero spacing·border·gap, auto row, auto placement/order 0이다. 5px, 7.5px, 10px, 12.5px, 20px의 positive finite computed px 다섯 값과 `min-content`/`max-content` 단일 column을 조합한 10개 scene만 허용한다. `line-height`는 absolute 10px으로 고정해 line-height 계산 변경과 분리한다.

0px, 음수, non-finite/overflow 입력, 임의 font family·weight/style, arbitrary text, child inline element, nested Grid, public CSS admission, general inheritance, font fallback, glyph paint, WPT 적합성 또는 전체 C11.4/C11 완료는 이 계획에 포함하지 않는다. 이 제한은 CSS 전 범위에서 0px font-size가 invalid라고 주장하지 않으며 현재 text model의 양수 font-size 계약 밖 입력을 해당 fixture에서 fail-closed한다.

## 사전 비교와 독립 기대값

[Chrome 156 Agent Browser observation](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-font-size-precomparison-2026-10-31/chrome/chrome156-observation.json)은 pinned `Ahem.ttf`, `XX XX`, computed font size 다섯 값, 두 intrinsic track function을 고정했다. Chrome `gridTemplateColumns`/`gridTemplateRows`는 track oracle로 기록하고 root frame은 별도 관찰값으로 둔다. Range rectangles는 track 크기의 대용으로 사용하지 않는다.

독립 기대값은 Chrome 출력에서 복사하지 않고 pinned Ahem font table을 확인해 계산한다. `unitsPerEm=1000`, glyph `X`와 `space` advance 각각 1000 units라서 각 글자/공백 advance가 computed font-size 1em이다. `XX XX`의 min-content inline size는 두 글자의 한 단어 `2em`; max-content inline size는 공백을 포함한 `5em`이다. min-content track은 두 단어로 줄바꿈되어 auto row 20px을 차지하고 max-content track은 한 줄로 10px을 차지한다. 기대 track은 아래 표의 `min/max content × font-size` 식과 같다.

| computed `font-size` | min-content column / row | max-content column / row |
| ---: | ---: | ---: |
| 5px | 10px / 20px | 25px / 10px |
| 7.5px | 15px / 20px | 37.5px / 10px |
| 10px | 20px / 20px | 50px / 10px |
| 12.5px | 25px / 20px | 62.5px / 10px |
| 20px | 40px / 20px | 100px / 10px |

CSS Grid Level 1 §11.5의 intrinsic track sizing과 CSS Sizing Level 3의 min/max-content sizing 규칙에 기대값을 연결한다. [CSS Fonts Level 4 §2.5](https://www.w3.org/TR/css-fonts-4/#font-size-prop)의 computed `font-size`는 resolved length이며 fixture는 source CSS를 재해석하지 않고 Stylo의 computed-style value만 사용한다. 자세한 브라우저·font hash·원시 관찰은 [사전 비교 README](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-font-size-precomparison-2026-10-31/README.md)에 둔다.

## 계획된 변경

1. 기존 anonymous text fixture generator에 10개의 고유 scene ID를 추가한다. 각 HTML root의 computed `font-size`와 `grid-template-columns`는 표와 같고 direct text는 `XX XX`, line-height는 10px, row는 auto다. scene 선택만으로 paragraph font size를 정하지 않는다.
2. fixture validator는 정확한 root ID·tag·direct-child topology·text·track·computed `font-size` property와 typed `font_size_css_px`가 allowlist case와 모두 일치하는지 확인한다. grammar와 serializer는 5, 7.5, 10, 12.5, 20px의 finite positive `px` value만 받는다. 0, 음수, non-finite, suffix/number format mismatch, 다른 scene/style는 layout publication 전에 거부한다.
3. anonymous paragraph style은 검증된 동일 root/style revision의 `font_size_css_px`를 사용한다. line-height, font family/weight/style, writing mode, white-space, tab-size 및 direction은 기존 고정값을 유지한다. item identity·source order·style revision provenance도 유지한다.
4. Rust tests는 열 개 scene 각각에 대해 `ParagraphStyle.font_size`가 computed root와 일치하는지, Ahem intrinsic min/max inline size가 `2em`/`5em`인지, line-height 10px에서 min-content/ max-content auto row가 각각 20px/10px인지 확인한다. root/item frame과 Columns/Rows contribution/final-track trace를 함께 고정한다. 기존 55 anonymous text scenes와 140 full-tab-value scenes는 regression으로 유지한다.
5. Runtime negative controls는 allowlist font-size와 computed CSS property 불일치, typed float 불일치, 0px/비유한 값, unlisted fractional value, 잘못된 root/child/text/track, stale revision을 각각 거부한다. 실패 시 부분 frame·trace가 나오지 않아야 한다.
6. 변경 후 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator fixture route에서 10개 신규 scene 및 기존 anonymous-text 회귀를 실행한다. 화면·computed style·ParagraphStyle/font registry·intrinsic sources·track traces·root/item frames·draw/present·revision을 비교하고 대표 overflow 화면을 보존한다. Android AVD는 software renderer이며 iOS drawable 결과는 compositor 제출 전임을 유지한다.
7. default/product와 `c11-4-grid-track-sizing-fixture-test` opt-in archive를 따로 검사한다. 새 scene IDs와 dedicated fixture ABI/payload는 기본 archive에서 없어야 하며 opt-in positive control에는 있어야 한다. 전체 app bundle exclusion으로 확대하지 않는다.
8. 관련 Rust focused tests, fmt, targeted Clippy, Android/iOS Simulator builds, fixture/module boundary checks를 실행한다. 최종 코드/runtime에는 구현 실패 관점 20개를 별도로 검토하고 PR diff에는 저장소 규칙의 독립 실패 관점 30개를 적용한다. plan review 20개와 plan PR diff review 30개도 구현 review로 재사용하지 않는다.
9. 내부 계약 [0059](../spec/internal/0059-c11-grid-layout.md)에 fixture-only `font-size` input·scene allowlist·error boundary·runtime evidence를 기록하고 버전 `0.1.0`을 유지한다. C11.4 및 C11 공식 checkbox, 전체 완료 집계 `72/213`은 변경하지 않는다.

## 실패 기준과 남은 한계

- Stylo의 computed font-size와 Parley `ParagraphStyle.font_size`가 다르거나 unlisted style이 허용되면 실패다.
- 10개 scene 중 하나라도 font-table 산식의 track width, auto row, intrinsic input, trace, frame 또는 revision과 다르면 실패다.
- min-content/max-content 폭을 root width나 Range rect로 대체하거나 fractional CSS px를 정수로 반올림해 비교하면 실패다.
- unsupported input이 partial frame/trace를 남기거나 scene ID로 computed style 값을 숨기면 실패다.
- 기본 archive에 fixture-only symbol/payload가 포함되거나 opt-in positive control에서 빠지면 실패다.
- 이 slice는 CSS `font-size` 전체 문법/cascade, 0px text rendering, arbitrary fonts/scripts, glyph ink exactness, WPT, 실기기/hardware GPU, iOS compositor 이후 pixels, 전체 C11.4 또는 C11 완료를 증명하지 않는다.

## 계획 검토

별도 [계획 실패 관점 20개](./c11-4-anonymous-grid-text-computed-font-size-plan-review-20.md)와 계획 PR의 [diff 실패 관점 30개](./c11-4-anonymous-grid-text-computed-font-size-plan-pr-diff-review-30.md)는 이 계획에서만 다룬다.

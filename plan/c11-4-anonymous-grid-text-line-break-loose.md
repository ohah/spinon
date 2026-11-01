# C11.4 anonymous Grid text computed `line-break: loose` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [CSS Text Level 3 §5.2](https://www.w3.org/TR/css-text-3/#line-break-property) · [Unicode UAX #14](https://www.unicode.org/reports/tr14/)

**상태:** 계획 실패 관점 [20개 검토](./c11-4-anonymous-grid-text-line-break-loose-plan-review-20.md)와 계획 PR diff [30개 검토](./c11-4-anonymous-grid-text-line-break-loose-plan-pr-diff-review-30.md)를 완료했다. Chrome 156 Agent Browser와 고정 Noto Sans KR 입력에서 일본어 `line-break: loose`의 min-content track·auto row가 영어 locale 또는 일본어 `normal` control과 달라지는 네 장면을 고정했다. 규범 입력·독립 계산·관찰은 [사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-precomparison-2026-11-01/README.md)에 있다. 계획 PR 병합 후 구현은 같은 checkout에서 순차 진행한다. 이는 제한 opt-in fixture 후보이며 C11.4/C11 전체 완료나 일반 CSS line-break 지원을 뜻하지 않는다. 공식 `72/213`은 유지한다.

## 목적과 미지원 경계

현재 `LineBreakMode`는 `Normal | Strict`만 보유한다. C11.4 anonymous Grid direct-text fixture는 새 `line-break:loose` 값을 닫아 두며, 기존 computed `normal|strict`, `auto` fallback, 다른 text properties와 언어 metadata의 경계를 바꾸지 않는다. 이 계획은 한 고정 Noto Sans KR 10px/400 LTR profile에서 exact Grid owner의 computed `line-break:loose`를 `ParagraphStyle`과 Parley 0.12.0 intrinsic measurement에 전달한다.

입력 문자열은 `文＄文`이다. 세 문자의 각 advance는 고정 Noto Sans KR font에서 10px이다. U+FF04는 East Asian Fullwidth `PR` prefix다. UAX #14 LB25의 기본 `PR × ID` no-break 규칙 아래 첫 `文＄` 경계는 break 가능하지만 `＄文`은 20px unbreakable segment다. CSS Text §5.2는 `loose`에서 중국어·일본어 writing system이면 이 prefix 뒤 break를 허용한다. 따라서 `lang=ja; line-break:loose`의 독립 min-content는 세 10px segment의 최댓값인 10px이고, `lang=en; loose`와 `lang=ja; normal`은 20px다. 같은 조건의 max-content는 30px이다. Chrome은 이를 [네 장면](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-precomparison-2026-11-01/expected-values.json)에서 관찰했다.

이번 범위는 fixture-test opt-in C11.4 anonymous direct-text adapter 한 곳에 한정한다. 일반 author CSS admission, 제품 HTML의 일반 computed-style mapping, `line-break:auto|anywhere`, 전체 line-break keyword profile, 임의 font·언어·문자열, inline runs, script tailoring, WPT suite, glyph painting, 실기기·hardware GPU, iOS compositor 이후 pixels 및 전체 C11.4/C11 지원은 포함하지 않는다. 내부 계약 `0.1.0`과 공식 완료 `72/213`을 유지한다.

## 사전 비교와 독립 기대값

- Browser: Agent Browser `HeadlessChrome/156.0.0.0`.
- Font: `tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf`, SHA-256 `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76`; browser `document.fonts.check("10px NotoFixture")` 결과는 true다.
- Fixture: [`anonymous-grid-line-break-loose-precomparison.html`](../tests/fixtures/css/c11/anonymous-grid-line-break-loose-precomparison.html).
- 관측한 DOMRect·used track: [`chrome-observation.json`](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-precomparison-2026-11-01/chrome-observation.json).
- CSS Text 규칙·font advance 독립 계산: [`spec-derived-expectations.json`](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-precomparison-2026-11-01/spec-derived-expectations.json).
- 화면: [Chrome 156 일본어 loose min-content](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-loose-precomparison-2026-11-01/chrome156-ja-loose-min-content.png).

| 장면 | `lang` | `line-break` | 열 | used column | auto row | 구분 |
| --- | --- | --- | --- | ---: | ---: | --- |
| ja-loose-min | `ja` | `loose` | `min-content` | 10px | 30px | 세 문자 경계에서 분리 가능 |
| ja-loose-max | `ja` | `loose` | `max-content` | 30px | 10px | 최대 내용 폭에서 줄바꿈하지 않음 |
| en-loose-min | `en` | `loose` | `min-content` | 20px | 20px | 일본어 tailoring 없음 |
| ja-normal-min | `ja` | `normal` | `min-content` | 20px | 20px | prefix 뒤 break를 추가하지 않음 |

Grid root content width는 120px이며 border·padding·gap은 0이고 열만 intrinsic sizing을 사용한다. 결과 줄마다 10px row height를 기대한다. 구현 전에 입력, font hash, browser 관찰 및 독립 기대값 JSON을 고정했다.

## 구현 및 검증 범위

1. `spinon-text-layout::LineBreakMode`에 `Loose`를 추가하고 Parley 0.12.0 `LineBreak::Loose`로 정확히 매핑한다. 기존 `Normal`, `Strict`, default 및 `auto` fallback 결과는 보존한다.
2. Anonymous Grid fixture adapter는 네 신규 scene에서만 computed `line-break:loose`를 허용한다. 동일한 Grid style owner, document/style revision, 직접 지정한 `lang`, text `文＄文`, font/weight/size, white-space, word-break, overflow-wrap, text-indent 및 track sizing 조건을 검증한다.
3. Text-layout unit tests에서 locale별 break segmentation과 min/max inline·block contribution을 독립 기대값에 대조한다. Runtime·FFI validator는 computed value, owner/revision, exact source string, ParagraphStyle locale/mode, intrinsic input/output, 10/20/30px track, row height, frame과 draw/present를 검사한다.
4. 잘못된 locale, scene과 다른 `normal`/`strict`/`auto`, unknown scene, 누락/오류 Grid owner, stale revision, 변형 text/font/track, unsupported white-space/word-break 및 malformed locale은 layout/frame/draw 발행 전에 거부한다. 기존 장면의 computed-style gate는 그대로 유지한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 신규 장면과 기존 anonymous-text line-break matrix를 같은 최종 source hash로 실행한다. geometry/track trace, draw/present, retry count, screenshot과 설치 artifact hash를 기록한다. Screenshot이 glyph ink를 보여 주지 않으면 glyph pixel 근거로 해석하지 않는다.
6. 기본/product static archive에는 새 fixture marker·scene ID·FFI entry·payload가 없어야 하고 fixture-test positive control에는 포함되어야 한다. Static archive 결과를 APK/IPA 전체 fixture 제외로 확대하지 않는다.
7. 계획 검토와 겹치지 않는 구현/runtime 실패 관점 20개를 별도로 수행한다. 구현 PR diff는 30개 관점으로 검사하고, 결함을 고치면 영향받은 runtime와 검토 항목을 다시 확인한다. PR 본문에는 Chrome screenshot과 가능한 simulator 동작 화면을 넣고 Agent Browser로 이미지 표시를 확인한다.
8. 병합 뒤 계획, 내부 계약 0059, `spec/STATUS.md`, parent track-sizing 계획과 public roadmap을 결과 및 제외 범위에 맞춘다. C11.4/C11 checkbox와 공식 `72/213`은 올리지 않는다.

## 완료 판정

네 장면의 used column, auto row 및 source-order break 위치가 CSS Text Level 3와 독립 glyph advance 계산에 맞고, locale/mode/owner/revision 불일치가 부분 출력을 만들지 않아야 한다. Text-layout/Rust 집중 검사, feature-scoped Clippy, fixture 경계, Android/iOS Simulator 결과, 구현/runtime 20개 검토와 별도 PR diff 30개 검토도 모두 통과해야 제한 fixture slice를 완료로 기록할 수 있다. 이 조건은 일반 CSS admission, WPT 적합성, glyph painting, 물리 기기 표면 또는 전체 C11.4/C11 지원을 선언하지 않는다.

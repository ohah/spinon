# C11.4 anonymous Grid text `white-space:pre` 제한 slice

**상위:** [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [공식 상태](../spec/STATUS.md#c11-css-grid) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [내부 text 계약 0070](../spec/internal/0070-c15-1-inline-text-layout.md)

**상태:** 계획 실패 관점 20개 검토와 Chrome 156 사전 비교를 마쳤다. `WhiteSpaceMode::Pre`, HostDocument profile gate, root HTML computed-style mapping, 5개 opt-in scene 구현과 26개 기존 회귀 scene을 합친 Android/iOS Simulator 31/31 행렬이 통과했다. Text/layout/runtime/FFI focused tests, 59개 text-layout unit tests, fixture module boundary 24/24, rustfmt, workspace Clippy, 기본/fixture archive 경계 검사와 별도 구현/runtime 20관점 검토 및 별도 PR diff 20관점 검토도 통과했다. 구현 PR [#880](https://github.com/ohah/spinon/pull/880)은 2026-10-30T23:06:20Z Rebase and merge됐고 merge SHA는 `c592212bbf858cda0f310bb5885e864687960d06`이다. PR check rollup은 비어 있었고 main required status checks도 설정되지 않았다. 일반 C11.4/C11 완료나 넓은 CSS admission은 주장하지 않는다.

## 기준 규칙과 독립 oracle

- CSS Text Level 3 `white-space:pre`는 공백을 보존하고 soft wrap을 금지한다. forced segment break는 줄을 나눈다. [`white-space` 규범](https://www.w3.org/TR/css-text-3/#white-space-property)
- [CSS Grid Layout 2025 CRD의 intrinsic track sizing 규칙](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)과 CSS Text의 보존·줄바꿈 규칙을 독립 산술과 함께 사용한다. Chrome 출력은 교차 관찰이며 기대값의 유일한 출처가 아니다.
- 사전 비교 입력은 저장소의 고정 Ahem `tests/fixtures/resources/c15/Ahem.ttf` (`b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`)이며 브라우저는 Agent Browser HeadlessChrome `156.0.8078.76`이다. `document.fonts.check("10px Ahem")`가 true였다.
- [비교 fixture](../tests/fixtures/css/c11/anonymous-grid-text-pre-precomparison.html) 및 [관측 기록·screenshot](../spec/internal/evidence/c11-4-anonymous-text-pre-precomparison-2026-10-31/README.md)을 사전 기준으로 고정했다.

## 고정 scene과 기대값

| Scene | 고정 입력 | 기대 Grid track·root | 기대 text line |
|---|---|---|---|
| `normal-max-content` | Ahem `XX  XX`, normal, max-content | 50px, 50×10px | 50×10px; 두 공백은 하나로 collapse |
| `pre-max-content` | 같은 source, pre, max-content | 60px, 60×10px | 60×10px; 두 공백 모두 보존 |
| `normal-min-content` | Ahem `XX XX`, normal, min-content | 20px, 20×20px | 20×10px 두 줄 |
| `pre-min-content` | 같은 source, pre, min-content | 50px, 50×10px | 50×10px 한 줄; space 경계에서 soft wrap 금지 |
| `pre-forced-break` | Ahem `XX\nXX`, pre, max-content | 20px, 20×20px | 20×10px 두 줄; LF는 forced break |
| `normal-fixed-width` | Ahem `XX XX`, normal, 25px root | 25px, 25×20px | 20×10px 두 줄 |
| `pre-fixed-width` | 같은 source, pre, 25px root | 25px, 25×10px | 50×10px 한 줄; root 밖 25px overflow |

## 구현 범위

1. `WhiteSpaceMode::Pre`를 `white-space-collapse:preserve` + `text-wrap-mode:nowrap` 의미로 text layout에 추가한다. 기존 `PreWrap`, `BreakSpaces`, `NoWrap`, `PreLine`은 별도 동작을 유지한다.
2. pre normalization은 반복 공백과 forced segment break를 보존한다. tab은 이 slice에서 지원하지 않으며 입력에 tab이 오면 구체적인 오류로 거부하고 부분 결과를 내지 않는다.
3. HostDocument paragraph validation, root HTML computed-style 어댑터의 `preserve + nowrap` mapping과 opt-in C11.4 anonymous-text runtime fixture allowlist를 일관되게 연결한다. 고정 Ahem ASCII scene 밖의 anonymous Grid topology·inline descendants·system font·임의 CSS는 허용하지 않는다.
4. 내부 C15 text 계약과 C11.4 계약에 실제 동작 및 tab·CSS admission 제한을 적는다. 출시 전 숫자 계약 버전 `0.1.0`은 유지한다.
5. 같은 앱 소스 revision으로 Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 신규 5 scene 및 기존 26 scene을 모두 실행한다. scene frame·track·text contribution·provenance·draw/present를 확인한다. Android software renderer와 iOS compositor 이전 sample 한계를 명시한다. Fixed-width scene은 25px Grid item frame과 50px text line advance를 따로 검사한다.
6. fixture-test artifact의 scene marker/fixture ABI가 기본 artifact에서 빠지고 opt-in positive control에 있는지 검사한다. PR 본문에는 Android/iOS 실행 log와 geometry 결과를 연결한다. 현재 runtime fixture는 C15.3 glyph paint가 꺼져 있어 글자 화면 캡처를 동작 화면으로 제시하지 않는다.

## 제외 범위와 완료 기준

- tab-size, tab 확장, arbitrary Unicode/font/script, trailing/leading-space matrix, inline formatting contexts, nested Grid, general author CSS admission, public WPT support, physical device, hardware GPU, iOS compositor 이후 pixels, glyph ink equality 및 전체 C11.4/C11은 제외한다.
- Text engine focused tests와 anonymous Grid host-document tests, runtime scene matrix, Android/iOS simulator builds, Clippy/rustfmt, artifact boundary, implementation/runtime 20관점 검토, 최종 PR diff 20관점 검토가 모두 통과해야 한다. PR마다 별도의 20개 관점을 남긴다.
- 공식 C11.4/C11 체크와 전체 완료 집계 `72/213`은 그대로 유지한다.

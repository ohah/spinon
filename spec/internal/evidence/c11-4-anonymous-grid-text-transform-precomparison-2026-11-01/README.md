# C11.4 anonymous Grid text `text-transform` 사전 비교

Chrome 156의 Agent Browser가 고정 HTML fixture에서 Noto Sans KR 로컬 font를 불러온 뒤 직접 anonymous Grid text의 computed `text-transform`과 min-/max-content track을 관찰했다. 입력은 Grid root의 직접 Text node `IIII ii`, direct `lang="en"`, `font-size:10px`, `font-weight:400`, `line-height:10px`, `white-space:normal`, LTR 및 zero grid spacing으로 고정했다.

CSS Text Level 3 2026-08-14 Candidate Recommendation Draft §2.1은 `text-transform`을 text에 적용되는 상속 속성으로 정의하고 `none`, `uppercase`, `lowercase`, `capitalize`, `full-width`, `full-size-kana`를 열거한다. 이번 계획은 `none|uppercase|lowercase` 중 ASCII 입력만 다루고 나머지 값을 명시적으로 거부한다. DOM source는 `IIII ii`로 보존하며 변환 문자열은 layout용으로만 사용한다. [규범 출처](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#text-transform-property).

Noto Sans KR variable font의 `wght=400`, `lang=en`, Latin shaping에서 고정 advance는 `I=293`, `i=275`, space `=224` units / 1000 UPEM이다. 이 값을 직접 합산하면 min-content 폭은 `none|uppercase` 11.72px, `lowercase` 11px이고 max-content 폭은 각각 19.46px, 19.82px, 18.74px다. CSS Grid CSSOM 값은 Chrome 1/64px layout quantization을 포함해 [기대 JSON](./expected.json)에 고정했으며 독립 합산과의 허용 오차는 0.03125 CSS px다.

Chrome 156.0.0.0 / macOS ARM64, DPR 1, viewport 1280×633에서 font-loaded와 6/6 scenes를 확인했다. `none`, `uppercase`, `lowercase`에서 source string은 같고 transformed layout glyph와 min-/max-content contribution은 다르다. Text Range rectangles는 anonymous item box나 glyph ink의 대용값이 아니다. 기계 판독 결과는 [Chrome 관측](./chrome156-observations.json), 화면은 [Agent Browser screenshot](./chrome156-text-transform-grid.png), 독립 metric 및 scene 값은 [expected.json](./expected.json)에 있다.

## SHA-256

| 입력 | SHA-256 |
| --- | --- |
| HTML fixture | `98ab0ae80ff2f1ca79dd34713d82103c7e31883ff740fc4a12092ca49bb3fa11` |
| Noto Sans KR variable font | `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76` |
| Chrome observation JSON | `fe73ded71b9cc25b0cf889ecb581f4f45cac34171d8dce9696e663971b943a5a` |
| expected JSON | `3dd5d56cad6205958744e850321e3c75e79f1de291cd53748c785b4e8b3ba82e` |
| Chrome screenshot | `ca4be56c62a370c23c773ff8aefc303ab95e56d94280bd4ee0dae5475bc88a42` |

Chrome is a browser oracle for these six fixed scenes, not Spinon runtime, general CSS support, WPT conformance, mobile behavior, glyph painting, or surface pixel parity.

The selected Rspress 2.0.22 documentation build generated the plan, review, contract, status, evidence, and public roadmap routes. Agent Browser confirmed the candidate card at desktop 1280px and mobile 390px without horizontal overflow. This selected-root build does not run the repository-wide dead-link check and is not a Tailnet deployment. [Build and viewport record](./selected-docs-build-plan-pr951.json).

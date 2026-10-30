# C11.4 direct-text preferred aspect-ratio range 사전 비교

**기준 시점:** 2026-10-30, 구현 계획 PR 전
**기준 브라우저:** Agent Browser / HeadlessChrome 156.0.0.0, macOS, viewport 1280×633 CSS px, DPR 1
**기준 문서:** [CSS Grid Layout 2025 CRD §6.2, §11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing)

이 비교는 기존 #834 fixture가 `aspect-ratio:2/1` 하나만 허용하는 경계를 확장할지 판단하기 위해 독립 입력으로 수행했다. Chrome 관측은 명세 기대값과 구별한다. 이는 Spinon 실행, WPT 결과, 일반 CSS 지원 선언이 아니다.

## 고정 입력

[fixture](../../../../tests/fixtures/css/c11/grid-direct-text-aspect-ratio-range.html)는 두 HTML block Grid item의 direct Ahem text를 쓴다. `A`는 10px, `AAAA`는 40px inline advance와 10px line-height다. Root는 definite 100px width, `max-content max-content` columns, 0 gap, `align-items:normal`, `justify-items:normal`이다. Row 장면은 auto row이고 column 장면은 각각 definite 60·45·30·20px row다. 각 ratio 장면은 `AAAA` 하나에만 `aspect-ratio`를 적용한다. Controls는 해당 row 높이에서 ratio 선언만 뺀다.

Agent Browser가 사용한 Chrome은 `Ahem` face를 loaded로 보고했고 `document.fonts.check('10px Ahem')`도 true였다. Fixture와 Ahem TTF를 HTTP 200으로 받았다. 캡처 뒤 이 작업의 Agent Browser 세션과 임시 HTTP server를 종료했다.

## 독립 기대값과 Chrome 관측

CSS Grid 2025 CRD §6.2에 따르면 `normal` 정렬의 auto-sized item은 preferred aspect ratio가 있을 때 일반 block-level 크기 계산을 따르고, ratio가 없는 non-replaced item은 stretch한다. §11.5의 intrinsic contributions와 max-content tracks를 함께 고려한다. Ahem direct text의 예상 inline advance에 ratio를 적용해 row·column 결과를 독립 계산했다. 이 항목은 replaced-item transferred-size-suggestion 규칙(§6.6)을 재사용하지 않는다.

| 장면 | 독립 계산 | Chrome 156 관측 |
|---|---|---|
| `row-1-2` | `40 / 0.5 = 80px` row | columns `[10,40]`, row `80px`, root `100×80`, `AAAA` `40×80` |
| `row-4-3` | `40 / (4/3) = 30px` row | columns `[10,40]`, row `30px`, root `100×30`, `AAAA` `40×30` |
| `row-3-2` | `40 / 1.5 = 26.666…px`; used geometry는 1/64 CSS px quantization 허용 | row `26.65625px`, `AAAA` `40×26.65625` |
| `row-5-2` | `40 / 2.5 = 16px` row | row `16px`, `AAAA` `40×16` |
| `row-control` | ratio 없음, 10px line-height | row `10px`, `AAAA` `40×10` |
| `column-1-2` | row 60px와 1/2 ratio의 preferred width 30px보다 text max-content 40px track이 큼. 40px width를 ratio에 적용한 item block-size는 80px | columns `[40,10]`, grid `100×60`, `AAAA` `40×80`; root row 아래 20px overflow |
| `column-4-3` | `45 × (4/3) = 60px` max-content column | columns `[60,10]`, `AAAA` `60×45` |
| `column-3-2` | `30 × 1.5 = 45px` max-content column | columns `[45,10]`, `AAAA` `45×30` |
| `column-5-2` | `20 × 2.5 = 50px` max-content column | columns `[50,10]`, `AAAA` `50×20` |
| `column-control-60` | ratio 없음, text max-content width 40px | columns `[40,10]`, `AAAA` `40×60` |
| `column-control-45` | ratio 없음, text max-content width 40px | columns `[40,10]`, `AAAA` `40×45` |
| `column-control-30` | ratio 없음, text max-content width 40px | columns `[40,10]`, `AAAA` `40×30` |
| `column-control-20` | ratio 없음, text max-content width 40px | columns `[40,10]`, `AAAA` `40×20` |

`column-1-2`는 track과 item box를 별도로 검사해야 하는 대조 사례다. Max-content text track은 40px을 유지하고 ratio가 item box를 40×80으로 만든다. Root가 60px row에서 멈춰도 item은 잘리지 않으며 20px overflow한다. Browser 결과 JSON은 scene별 computed tracks와 document-coordinate DOMRect를 보존하고, 독립 계산에서는 root-relative frame으로 정규화한다.

## 산출물 및 재현성

- Chrome 관측: [`chrome-observation-agent-browser.json`](./chrome-observation-agent-browser.json)
- Browser/version/font/viewport: [`browser-run-metadata.json`](./browser-run-metadata.json)
- 전체 장면 캡처: [`chrome-156-range.png`](./chrome-156-range.png), 1280×908 RGB PNG
- 입력 fixture SHA-256: `fd43945d6200d36524c07c539fdf1e3193157e4df4ad13f12243937e42c771d0`
- Ahem font SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- Chrome 관측 JSON SHA-256: `218fad4c8f0da207d9e45f2757fcdcacbdafede42e6518652002357fc0c5ad7e`
- Screenshot SHA-256: `092f8eee17e51c2dd43de046d8d9b1bbcfa0b14e1ee9fb9dbcfed9239784f6b7`
- Roadmap 카드 렌더: [`roadmap-agent-browser.png`](./roadmap-agent-browser.png), 새 계획 카드가 포함된 로컬 정적 로드맵의 Agent Browser 캡처, 1280×633 RGB PNG. 실행 세부와 범위는 [`roadmap-render-metadata.json`](./roadmap-render-metadata.json)에 기록했다. SHA-256: `3b29aca427eb77f9b86f06d06505d9b0fdf86ca41475afad3820a1f3778ba82a`.

JavaScript page error는 없었다. Local HTTP server의 `/spinon/icon.svg` 요청은 404였으며 로드맵 카드와 본문은 렌더링됐다. Fixture와 Ahem asset은 각각 200이다. Roadmap 캡처는 저장소의 정적 HTML을 로컬에서 렌더링한 증거이며 배포/Tailnet 링크 동작을 검증하지 않는다. Chrome 장면 screenshot은 layout box/배경 확인용이며 glyph raster parity나 OS compositor 이후 픽셀을 주장하지 않는다.

이 계획의 후속 구현은 기존 C11.4 opt-in fixture-test profile 안에서 computed finite-positive ratio 값을 수용하고, 2/1 기존 scene 및 ratio 없는 control을 유지한다. `auto <ratio>`, non-finite/degenerate ratio, 둘 이상의 ratio item, unsupported structure/style, 일반 author CSS admission, WPT 전체 실행과 C11.4/C11 완료는 별도 범위다.

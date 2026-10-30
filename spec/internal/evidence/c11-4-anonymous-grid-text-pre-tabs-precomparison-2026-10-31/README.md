# C11.4 `white-space:pre` preserved tab 사전 비교

## 기준과 범위

CSS Text Module Level 3 [§4.1.2](https://www.w3.org/TR/css-text-3/#white-space-phase-2) 및 [§4.2 `tab-size`](https://www.w3.org/TR/css-text-3/#tab-size-property)를 기준으로 삼았다. `tab-size`의 초기값은 8이며, 숫자 값은 보존된 탭이 속한 가장 가까운 block container ancestor의 U+0020 advance 배수다. 보존된 탭은 그 ancestor의 inline content 시작점에서 tab-size 간격의 다음 stop으로 전진한다. 현재 비교 fixture는 Ahem `10px/10px`, `horizontal-tb`, LTR, zero margin/padding/border의 고정 입력이며 ancestor와 Grid content 시작점이 모두 x=0이다. 각 Ahem `X`와 공백은 10px이므로 tab stop 간격은 80px이다.

구현 대상 후보는 `white-space:pre`의 기본 `tab-size:8` 동작에 한정한다. 이 자료는 non-default `tab-size`, 임의 폰트·spacing·writing mode 또는 offset ancestor 지원을 제안하지 않는다. CSS 계산의 독립 기대값과 Chrome 결과를 분리해 [expected-values.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31/expected-values.json)에 저장했다.

## 관찰 환경

- Browser: Agent Browser / HeadlessChrome `156.0.0.0`; 실제 사용 agent-browser 세션을 닫았다.
- Font: 저장소 Ahem; SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`.
- Fixture: [anonymous-grid-text-pre-tabs-precomparison.html](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/anonymous-grid-text-pre-tabs-precomparison.html), HTML SHA-256 `f0242ae68ea5558af70a160d2abaf364a34001aa576bf6c450b7c954a680f995`.
- 원본 관측 JSON: [chrome-observation.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31/chrome-observation.json); user agent/font 확인: [browser-environment.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31/browser-environment.json).
- Chrome 화면: [chrome-156-agent-browser.png](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31/chrome-156-agent-browser.png). 이는 Chrome oracle이며 Spinon 앱 화면이 아니다.
- 규범 텍스트를 Agent Browser에서 읽은 관찰: [css-text-spec-observation.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-anonymous-grid-text-pre-tabs-precomparison-2026-10-31/css-text-spec-observation.json).

## 판정 요약

| 장면 | 독립 기대값 및 Chrome 관찰 |
|---|---|
| `XX\tXX` | 2 columns 뒤 탭은 x=80으로 이동, max-content track/text line 100px |
| `\tX` | 선행 탭은 x=80으로 이동, line 90px |
| `XXXXXXXX\tX` | x=80의 정확한 stop에서는 다음 stop x=160을 사용, line 170px |
| `XX\t\tX` | 연속 탭은 x=80과 x=160으로 이동, line 170px |
| `XX\tXX\nX\tX` | 강제 줄바꿈 뒤 두 번째 줄 tab position은 x=0부터 다시 계산, lines 100px/90px, row 20px |
| `XX\t` | 끝의 보존 탭 advance를 유지, line/track 80px |
| min-content `XX\tXX` | `pre`에서 soft wrap 없이 min-content track 100px, 1 line |
| 25px fixed track `XX\tXX` | Grid track/root 25px, unwrapped text line 100px |
| oracle-only `tab-size:2` control | Chrome track 60px; 기본값 8만 받는 제한 profile의 범위 밖 |

Chrome oracle는 화면 좌표가 아닌 computed track, root frame, text Range line fragment를 관찰했다. Range rectangles는 glyph ink raster의 pixel equality 증거가 아니다.

# C11.4 Grid track sizing 사전 비교

**상태:** Chrome 기준 fixture를 수집했다. Spinon 계산기·runtime·WPT 실행 자료가 아니다.
**고정 case 수:** 26.

## 입력과 실행 환경

- Fixture: [`grid-track-sizing-c114-precomparison.html`](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-track-sizing-c114-precomparison.html), SHA-256 `f2bfd4fc295988c9550026627c8d83f9b2daa04c52633e4a4c10d2dc2e9bc8a6`.
- 기준 구현: Agent Browser를 통한 Google Chrome `155.0.8059.85`, HeadlessChrome reduced UA `155.0.0.0`, macOS `26.5.1` ARM, locale `ko-KR`, DPR 1. Full/high-entropy UA와 platform metadata는 [`browser-metadata.json`](chrome/browser-metadata.json)에 있다.
- Browser metadata SHA-256: `6d1f44c5ad3a769726234fe4899e1ff91a647cf7dfa26c9cb6e8aa054aa00c8e`.
- Image 입력: [`c14-1-png-basic.png`](https://github.com/ohah/spinon/blob/main/tests/fixtures/resources/c14-1-png-basic.png), 137 bytes, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`, natural `24×16 CSS px`.
- Text 입력: [`Ahem.ttf`](https://github.com/ohah/spinon/blob/main/tests/fixtures/resources/c15/Ahem.ttf), SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`; `document.fonts.check("10px Ahem")` true. Text/image collection awaited `document.images[].decode()` and font readiness before reading geometry.
- 첫 image decode와 경합해 나온 0×0 측정은 폐기했다. 아래 JSON은 완전히 decode된 이미지·로드된 font에서 수집한 값이다.

## 고정 oracle

원본: [`chrome155-observation.json`](chrome/chrome155-observation.json), SHA-256 `c7453b69936c734d110671d9fcb2525c733f0ac42be1c91b0c783084d7f498ce`.
화면: [`chrome155-dpr1.png`](chrome/chrome155-dpr1.png), full-page `1280×1363` PNG, SHA-256 `f1cadfd00aeb934e58205d4cd21d4341b8ec48111a6f808d24df1a80ea3a5662`.
Agent Browser raw output: [`chrome155-agent-browser-eval.raw.txt`](chrome/chrome155-agent-browser-eval.raw.txt), SHA-256 `c7453b69936c734d110671d9fcb2525c733f0ac42be1c91b0c783084d7f498ce`.

모든 frame 값은 CSS px다. 기준 JSON에는 grid/item frame, used track, image natural size, text content와 item `scrollWidth`/`scrollHeight`가 있다. 기본 geometry gate는 각 numeric field당 `0.05 CSS px` 이내다. Chrome의 auto-repeat 분수 track 값은 1/64 CSS px layout quantization도 보고 exact-oracle와 허용오차 결과를 각각 남긴다.

| ID | CSS 입력 요점 | used column track | grid 크기 | item 크기 |
| --- | --- | ---: | ---: | ---: |
| `auto-image-definite` | definite 100px grid, `auto`, natural image | 24px | 100×40 | 24×16 |
| `minmax-auto-image` | `minmax(0, auto)`, natural image | 24px | 100×40 | 24×16 |
| `minmax-min-max-image` | `minmax(min-content, max-content)` | 24px | 100×40 | 24×16 |
| `fit-content-below-minimum` | `fit-content(12px)`, image min 24px | 24px | 100×40 | 24×16 |
| `fit-content-above-intrinsic` | `fit-content(40px)`, image max 24px | 24px | 100×40 | 24×16 |
| `fit-content-percent` | definite 100px grid, `fit-content(50%)` | 24px | 100×40 | 24×16 |
| `fit-content-calc-limit` | `fit-content(calc(10% + 5px))`, Ahem `XXXXXXXX` with `break-all` | 15px | 100×120 | 15×80 |
| `fixed-small-track-overflow` | fixed 12px track, natural image, visible overflow | 12px | 12×40 | 24×16 |
| `spanning-minmax-items` | 70px item, two `minmax(0,auto)` tracks, gap 4px | 33px 33px | 100×30 | 70×10 |
| `spanning-auto-min-flex-tracks` | 200px unbreakable Ahem word spans two `minmax(0,1fr)` tracks, gap 4px | 48px 48px | 100×30 | 100×10 |
| `spanning-min-content-items` | 24px image spans two min-content tracks, gap 4px | 10px 10px | 100×30 | 24×16 |
| `min-content-row-image` | 100px column, natural image, `min-content` row | 100px | 100×80 | 24×16 |
| `auto-fill-definite` | 100px definite, `repeat(auto-fill,minmax(24px,1fr))`, gap 4px | 30.6562px 30.6719px 30.6719px | 100×30 | 30.65625×20 |
| `auto-fit-definite` | same repeated minimum, one item in first track | 100px 0px 0px | 100×30 | 100×20 |
| `auto-fit-all-empty` | auto-fit, no placed items | 0px 0px 0px | 100×30 | 없음 |
| `indefinite-percent-track` | max-content grid, 50% track, natural image | 12px | 24×16 | 24×16 |
| `auto-fill-indefinite` | max-content grid, auto-fill fixed 24px | 24px | 24×0 | 없음 |
| `auto-text-max-content` | Ahem `XX XXXXXXX`, auto track | 100px | 120×40 | 100×10 |
| `min-content-text-normal` | same text, normal word breaks | 70px | 120×40 | 70×20 |
| `max-content-text-normal` | same text, max-content | 100px | 120×40 | 100×10 |
| `fit-content-text-minimum` | `fit-content(40px)`, text min-content 70px | 70px | 120×40 | 70×20 |
| `min-content-text-edge` | text min-content 70px, 4px inline padding and 1px border per side | 80px | 120×50 | 80×20 |
| `min-content-text-min-width` | text min-content 70px, `min-width:90px` | 90px | 120×50 | 90×20 |
| `min-content-text-max-width` | text min-content 70px, `max-width:40px` | 40px | 120×50 | 40×20 |
| `min-content-text-break-all` | Ahem `XXXXXXXX`, `word-break:break-all` | 10px | 120×40 | 10×80 |
| `min-content-text-nowrap` | Ahem `XX XXXXXXX`, `white-space:nowrap` | 100px | 120×40 | 100×10 |

추가 geometry: `indefinite-percent-track`의 image 오른쪽은 grid 경계보다 12px 바깥이다. `fixed-small-track-overflow`는 12px grid 밖으로 12px 더 그릴 frame을 가진다. `min-content-text-break-all`은 40px grid 높이를 넘어 80px item 높이를 가진다. `fit-content-calc-limit`에서 definite 100px basis에 `calc(10% + 5px)`는 15px limit이고, min-content 10px/max-content 80px text의 track은 15px이다. `min-content-text-edge`의 inline padding/border는 content contribution에 10px을 더하며, `min-content-text-min-width`와 `max-width`는 각각 90px·40px을 반영한다. `spanning-auto-min-flex-tracks`에서 두 track은 48px씩이고 span item box는 100px이지만 Ahem `scrollWidth`는 200px로 보인다. auto-minimum 예외가 긴 콘텐츠를 track에 강제해 넣지 않는지 구현 때 이 차이를 확인한다. Android/iOS 표면 검증은 아직 없으므로 실제 native paint 여부는 이 oracle로 주장하지 않는다.

## WPT 출처 고정

읽기 전용 checkout `/Users/yoonhb/Documents/workspace/spinon/build/wpt-c124-source`, revision `c999c58338ee1d223df5ad62e48be1202ced0d37`에서 고정했다. 아래 파일은 검토 후보이며 여기서 실행하지 않았다.

| 경로 | SHA-256 | 검토할 assertion |
| --- | --- | --- |
| `css/css-grid/grid-definition/fit-content-track-sizing.html` | `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736` | growth cap/floor, auto-track stretch 제외, percent argument, implicit column |
| `css/css-grid/grid-definition/grid-layout-auto-tracks.html` | `37b73eb2a4234fcc7901a6a3ea3f42415c4e941969e76a9187ea373d2c28d698` | auto track과 item intrinsic contribution |
| `css/css-grid/grid-definition/grid-change-intrinsic-size-with-auto-repeat-tracks-001.html` | `b61b3b92114e58f2a7f3e563ed886e6ea39f20925b84b6defa7e111d7730420d` | intrinsic size 변경 뒤 반복 수·placement 갱신 |
| `css/css-grid/grid-definition/grid-auto-repeat-intrinsic-001.html` | `e6b83286b5f1cf4a46de70cd0dec292eff88d3bd24639570ca812a9ff13eb7c1` | intrinsic axis의 auto-fill/auto-fit 계산 |
| `css/css-grid/grid-definition/grid-auto-fit-columns-001.html` | `11410caa92c400b653ac781ef7f1103c871cebb108c52b75f2e8308b2531ba63` | definite auto-fit, minmax, line names 및 repeat 주변 track |

Chromium에서 fixture가 관찰값을 낸 것은 upstream WPT pass도, Spinon WPT pass도 아니다. Spinon 공식 WPT runner는 현재 연결되어 있지 않다.

## 실행 원본과 한계

- `chrome155-agent-browser-metadata.raw.txt`는 high-entropy UA 요청 결과의 원시 stdout이다.
- Screenshot은 oracle page 전체를 보여주는 Chrome 화면이며 Spinon 결과가 아니다.
- 이 26개는 계획 전 비교 집합이다. implementation-only controls, full WPT, CSS Grid 전체, 글리프 픽셀 동등성, Android/iOS runtime 및 실기기 결과를 포함하지 않는다.
- Agent Browser session 및 localhost fixture server를 종료했다. 후속 확인에서도 Agent Browser·Terminal Browser process나 fixture server listener가 남아 있지 않았다.

이 구현 계획은 [C11.4 세부 계획](https://github.com/ohah/spinon/blob/main/plan/c11-4-grid-track-sizing.md), 계획 검토는 [20개 실패 관점](plan-review-20.md), 계획 PR 변경 검토는 [30개 관점](pr-delta-review-30.md)에 있다.

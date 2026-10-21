# Nested Grid + replaced item 사전 비교

## 목적

일반 nested Grid 한 단계 안에서 Ahem text와 자연 크기 HTML image가 각각 내부 track과 parent Grid contribution에 반영되는 고정 입력을 만들었다. 기존 nested text fixture의 두 번째 text leaf를 `<img>`로 교체한다. 결과는 fixture planning oracle이며 public CSS Grid support 주장이 아니다.

## 환경과 입력

- 기준 구현: Agent Browser가 실행한 `HeadlessChrome/156.0.0.0` (navigator user agent는 `chrome156-observation.json` 참조)
- viewport: Agent Browser 기본 headless viewport; 장면은 0, 28, 56 CSS px y 위치에 세로 배치
- CSS: outer gap 4px, inner grid `min-content max-content` / gap 4px, Ahem 10px/10px, fixed `XX XXXXXXX`, natural-size `<img>`
- assets: 저장소의 `Ahem.ttf`와 24×16 PNG를 data URI로 fixture HTML에 고정
- 판정: CSS Grid 2 2025 CRD §§3.4, 5.2, 6.6, 12.5, 12.5.1에 따른 계산을 먼저 하고 Chrome 관찰을 대조한다.
- 입력 SHA-256: `d18ed142b8ff7f4c1ed864178ccf9d34d90b27c023dbfb6645aa27d7b3cf1605`
- 관찰 JSON SHA-256: `cf72cb89cd7d84102d191fa30fa3bcea11551872dcae0793d440aed8c8e4d07a`
- 화면 SHA-256: `b97133e07382458db9931de9f25da070e73a9ebab23964a80ee645ef540e8f34`

## 독립 계산

Ahem의 `XX XXXXXXX`는 10px glyph advances에서 min-content 70px, max-content 100px다. image natural size는 별도 PNG decode input에서 24×16px다. inner gap 4px를 포함하면 inner Grid의 inline min-content는 `70 + 4 + 24 = 98px`, max-content는 `100 + 4 + 24 = 128px`다. outer spanning item을 두 min-content track에 놓으면 4px outer gap을 제외한 94px를 두 track이 나눠 `47/47px`가 되고, one-track scene에서는 `98/0px`, root width `102px`가 된다. definite 120px flex scene의 two-track flex factor는 각 1이므로 `58/58px`이며, 두 flexible tracks를 span하는 item의 §6.6 automatic minimum은 0px다.

## 결과

| scene | outer frame / tracks | nested frame / tracks | text frame | image frame / natural |
| --- | --- | --- | --- | --- |
| span-two | `98×20`; `47/47px` | `98×20`; `70/24px` | `70×20` | `(74,0) 24×16`; `24×16` |
| one-track | `102×20`; `98/0px` | `(0,28) 98×20`; `70/24px` | `(0,28) 70×20` | `(74,28) 24×16`; `24×16` |
| definite-flex-control | `120×20`; `58/58px` | `120×20`; `70/24px` | `(0,56) 70×20` | `(74,56) 24×16`; `24×16` |

Ahem font load 및 각 PNG `complete` 값이 true였고, 모든 관찰값이 독립 계산과 일치했다. Chrome은 비교 oracle이지 WPT 결과나 Spinon 적합성 판정이 아니다. 이 단계에서는 앱 runtime/trace를 아직 검사하지 않았다.

# C14.2 replaced image sizing 사전 비교 · 2026-10-14

## 환경과 입력

- agent-browser `0.31.1`이 연 로컬 fixture의 Chrome Headless `155.0.8059.40`, macOS `26.5.1`, viewport `1280×633` CSS px, DPR `1`에서 DOM 측정값을 읽었다. 터미널 브라우저는 사용하지 않았다.
- Fixture는 [replaced-ratio-sizing-precomparison.html](../../../../tests/fixtures/css/c14/replaced-ratio-sizing-precomparison.html)이다. PNG source는 [c14-1-png-basic.png](../../../../tests/fixtures/resources/c14-1-png-basic.png), SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`; Chrome `naturalWidth × naturalHeight`는 모든 PNG case에서 `24×16`이었다.
- `complete`, natural width/height, 계산된 width/height 및 min/max, box-sizing, aspect-ratio, DOM rect를 [machine-readable 관측](chrome155-observation.json)에 저장했다. [전체 fixture 캡처](chrome155.png)는 브라우저 화면 근거다. 지원하지 않는 SVG·누락 src·깨진 PNG 행은 비교 경계 확인용이지 이번 PNG layout 성공 입력이 아니다.
- inventory에는 27개 browser row와 natural PNG가 있는 22개 row가 있다. C14.2 구현 비교는 구현·Rust oracle 행렬에 포함된 21개 case이며 0.05 CSS px tolerance를 적용한다. `aspect-ratio-auto-override`는 `aspect-ratio:auto <ratio>`를 사용해 C14.2 계획의 fail-closed 경계이므로 PNG row여도 구현 분모에서 제외한다. Chrome은 `max-width:20px` 경계에서 `19.984375×13.328125`를 보고했다. CSS 2.2 식의 `20×13⅓`과 작은 subpixel 차이가 있어 이 허용 오차를 사용한다.
- 별도 [Flex fixture](../../../../tests/fixtures/css/c14/replaced-ratio-sizing-flex-precomparison.html)와 [inventory](../../../../tests/fixtures/css/c14/replaced-ratio-sizing-flex-inventory.json)는 flex auto-basis, 명시 축, definite cross-axis와 stretch case 8개를 기록한다. [Flex 관측](chrome155-flex-observation.json)은 같은 Chrome 155, macOS, viewport, DPR 및 24×16 PNG에서 `48×20`, `30×24`, `48×32`, `24×16`, `60×40`, `30×20`, `24×16`, `36×24`를 확인했다.

## 고정 입력 SHA-256

| 파일 | SHA-256 |
| --- | --- |
| 기본 sizing fixture | `83745105e026b5d16394977ef153e43e53dd8be0533877c4c100d4bb6a72b4e6` |
| 기본 sizing inventory | `ca85885d37ee1d1f847bcaa4266ac96f25f38fa2e1b09431e8b7ccdf4b7326ab` |
| Chrome 기본 관측 | `3728e9d3e940b9249d64a2714c44894867a8cd52148ce06e1d7ad336afc4c1bd` |
| Flex sizing fixture | `b7187b5d726d8608a430805e0e38d686cf73461b27907ed2afa51c3561d0f4b8` |
| Flex sizing inventory | `656f86e2fba233504a86fd4af91271b71a3ef4e997fbb6ef48ce24c88dee6fd5` |
| Chrome Flex 관측 | `e0e8340fc726f70b1671d1b9c14b91324f2c7dc546493a4f11250e9c1fe7282c` |

## 주요 관측

| 경계 | Chrome frame (CSS px) |
| --- | ---: |
| natural `auto/auto` | `24×16` |
| `width:48` 또는 `height:24` ratio transfer | `48×32`, `36×24` |
| `min-width:48`, `min-height:32` | 둘 다 `48×32` |
| `max-height:10`, `max-width:18;max-height:10` | 둘 다 `15×10` |
| both-above-max, width limiting (`max-width:20;max-height:14`) | `19.984375×13.328125` |
| both-above-max, height limiting (`max-width:22;max-height:12`) | `18×12` |
| both-below-min branches | `33×22`, `36×24` |
| cross-axis min/max violations | `30×10`, `20×20` |
| `min-width:48;max-height:24`, `min-width:48;max-width:30` | `48×24`, `48×32` |
| definite width + auto-axis `max-height:20`, definite height + auto-axis `max-width:30` | `48×20`, `30×24` |
| border-box width `48`, padding `2`, border `1` | `48×34` |
| numeric `aspect-ratio:1/1` with width `48` | `48×48` |
| border-box width `48`, padding `2`, border `1`, numeric `aspect-ratio:3/2` | `48×32` |

Flex fixture에서는 고정 너비와 `max-height`, 고정 높이와 `max-width`, 고정 너비와 `min-height`, auto basis, definite cross-axis stretch, `max-height`가 있는 stretch, stretch 없는 definite cross-axis, 고정 cross-axis 크기를 각각 확인했다. 관측 크기는 차례대로 `48×20`, `30×24`, `48×32`, `24×16`, `60×40`, `30×20`, `24×16`, `36×24`다.

## 해석 범위

이 기록은 고정된 Chromium DOM oracle이다. browser observation 자체가 Spinon 적합성이나 이미지 디코드 성공을 증명하지 않는다. 성공했으나 자연 metadata가 빈 entry의 `300×150` fallback은 Rust synthetic snapshot fixture에서 확인한다. snapshot entry 누락·decode 실패는 이 빈 entry와 구분해 레이아웃에서 거부해야 한다. SVG natural sizing, image error icon, loader/cache/replacement, 앱 runtime, WGPU 표시, 실기기는 이 캡처 범위 밖이다.

자연 ratio는 content-box dimensions에 적용한다. 수치 `aspect-ratio`는 `box-sizing`이 지정한 box dimensions를 사용한다. 두 규칙의 구분은 natural ratio border-box `48×34`와 authored border-box ratio `48×32` 행에서 관찰했다.

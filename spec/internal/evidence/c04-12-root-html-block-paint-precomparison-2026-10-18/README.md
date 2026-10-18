# C04.12 루트 HTML Block scene 사전 비교

## 기준과 재현 입력

- 기준: Agent Browser에서 실행한 Headless Chrome `155.0.8059.85`, macOS `26.5.1`.
- 고정 뷰포트: CSS `320×240`, DPR `1`; light와 dark `prefers-color-scheme`을 각각 캡처했다.
- 입력: `tests/fixtures/css/c04/root-html-block-paint.html`, SHA-256 `39dabf47aa5e3ff279421d6d399ea0246399b067e5290162805a24dd8826e899`.
- 렌더링 확인: [light 캡처](chrome-155-light.png), SHA-256 `47c4722f3825325deb236d9fa455518b58412ec6615833a0196ffb7a2cbc8ec7`; [dark 캡처](chrome-155-dark.png), SHA-256 `ae20b3a7d11cb19cba815a8c4597574b23bba41c5e5654b51ad3bc189c40d935`. 두 파일은 뷰포트 크기 그대로 저장했다.

## 관찰값

| 노드 | display | x/y/width/height (CSS px) | light background | dark background |
| --- | --- | --- | --- | --- |
| `html` | `block` | `0/0/320/240` | `rgb(16,24,39)` | `rgb(16,24,39)` |
| `head` | `none` | `0/0/0/0` | transparent | transparent |
| `body` | `block` | `0/0/320/200` | transparent | transparent |
| `hero` (`main`) | `block` | `0/0/160/32` | `rgb(51,102,255)` | `rgb(51,102,255)` |
| `group` | `block` | `0/32/320/24` | transparent | transparent |
| `hidden` | `none` | `0/0/0/0` | computed value만 `rgb(255,0,0)` | 동일 |
| `hidden-child` | 숨긴 조상 아래의 `block` | `0/0/0/0` | computed value만 `rgb(0,255,0)` | 동일 |
| `child` | `block` | `0/32/120/24` | `rgb(64,160,96)` | `rgb(160,92,44)` |

HTML UA stylesheet가 `head`를 숨겨 layout box를 만들지 않는다. 숨긴 조상의 computed `display`가 `none`이면 자식 자신의 computed `display`가 `block`이어도 높이와 픽셀에 영향을 주지 않는다. 뷰포트 픽셀 표본은 다음과 같다.

| CSS 좌표 | light 예상값 | dark 예상값 | 의미 |
| --- | --- | --- | --- |
| `(10,10)` | `(51,102,255,255)` | same | hero background |
| `(10,40)` | `(64,160,96,255)` | `(160,92,44,255)` | body child and scheme media rule |
| `(10,60)` | `(16,24,39,255)` | same | root background after child bottom |
| `(200,40)` | `(16,24,39,255)` | same | transparent block exposes root background |
| `(10,210)` | `(16,24,39,255)` | same | root background outside the body box |

## 판정과 범위

이 oracle은 텍스트가 없는 정적 문서 트리 하나의 Block geometry, 숨긴 하위 트리, sRGB 배경, `prefers-color-scheme` 규칙을 보여준다. 일반 HTML/CSS 적합성, 텍스트 intrinsic sizing, body 배경 전파, 임의 UA 기본값, 반응형 viewport, clipping, 고 DPR projection, WGPU 출력, Android/iOS 동작을 증명하지 않는다. 구현 계획은 computed value와 frame을 native surface 표본과 별도로 비교한다. 시뮬레이터의 PASS 보고 화면만으로 픽셀을 판정하지 않는다.

화면은 Agent Browser로 캡처했다. 브라우저 console과 page error 목록은 비어 있었다. CSSOM에서 위 값들을 별도로 읽었고, 픽셀 표본은 캡처한 PNG 파일에서 읽었다. Terminal Browser와 실기기는 사용하지 않았다.

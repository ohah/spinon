# C13.1 overflow clip 사전 비교 · 2026-10-14

이 관찰은 C13.1 runtime 코드를 바꾸기 전에 고정한 Chrome 기준이다. 기존 [C13 overflow 사전 관찰](../c13-overflow-precomparison-2026-10-14/README.md)의 기본 scroll/clip·float 측정을 확장해 clip 축, clip margin visual box, 실제 `elementFromPoint()` 경계, 중첩 ancestor와 containing-block 예외를 구분한다.

## 환경과 입력

- agent-browser `0.31.1`, Google Chrome `155.0.8059.40` 공식 arm64 빌드, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, V8 `15.5.35.20`.
- macOS `26.5.1 (25F80)`, viewport `1440×1100 CSS px`, DPR `1`; Chrome은 `--headless=new --hide-scrollbars`로 실행했다.
- Chrome user agent는 `HeadlessChrome/155.0.0.0`이다. 정확한 버전·revision은 agent-browser에서 `chrome://version`을 열어 확인했다.
- Chrome 실행 파일 SHA-256: `428dccc1a45819d23c5db4fb37388883f8d9ba64611cbd6c42395eac4a18b6a4`.
- fixture [원본 HTML](../../../../tests/fixtures/css/c13/overflow-clip-c13-1.html), SHA-256 `b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a`.
- [computed value·frame·scroll·hit range 원본 JSON](chrome155-observation.json), SHA-256 `fd6d9e66825bc453609245950635f4fc56054ae9011b14d2f8901d490ecde4b5`.
- [agent-browser 전체 화면 캡처](chrome155.png), SHA-256 `f55b9432abcac47d1e3865ac8d618387ef2fde5c83965da4fa1fff3dbab5ce2b`.
- 브라우저 console/error 출력은 비어 있다.

## 관찰과 차이 분류

- `visible`, x-only `clip`, y-only `clip`, 양축 `clip`, 혼합축 정규화 및 clip-margin의 `padding-box`, `content-box`, `border-box` 계산값은 JSON에 보존했다. 각 frame과 hit 범위는 fixture 요소 rect에서 계산하며 고정 화면 좌표를 CSS 입력에 넣지 않았다.
- PNG는 축별 clip, clip-margin 확장, 중첩 clip 교차, positioned child와 static clip ancestor의 containing-block 예외를 보여준다. 화면에 그려진 픽셀과 DOM `elementFromPoint()` hit 결과는 별도 관찰이다. 예를 들어 x-only clip target의 hit 범위는 x=`232..337`까지여서 오른쪽 border strip까지 hit되지만, PNG의 파란 target은 padding edge에서 잘린다. `content-box 8px`은 PNG의 paint clip과 hit range도 일치하지 않는다. runtime 비교에서 paint와 hit-test를 하나의 결과로 합치지 않는다.
- nested fixture에서 안쪽 clip은 x축만 제한하고 바깥 clip은 양축을 제한한다. Chrome의 target hit 범위는 x=`253..323`, y=`961..1015`이며 두 ancestor의 교집합에서 잘린다.
- containing-block 대조에서 positioned clip ancestor 안의 absolute child는 clip되고, positioned containing block이 더 바깥에 있는 static clip ancestor 사례는 그 ancestor를 벗어나서 paint/hit된다. 이 결과는 DOM descendant 전부에 clip을 무조건 적용하면 안 된다는 fixture다.
- float border-box 높이는 `hidden=64px`, `clip=28px`, `clip + flow-root=64px`다. `clip`은 scroll setter를 받아들이지 않아 `scrollTop/Left=0`이고, `hidden` 및 mixed `hidden/auto` 대조는 요청한 `20px`을 반영했다.
- Chrome은 `CSS.supports('overflow-clip-margin', '-1px') == true`와 computed `-1px`를 반환했다. 고정한 [CSS Overflow Level 3 WD](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/#overflow-clip-margin)는 음수 길이를 invalid로 규정한다. 이는 Chrome oracle과 문법 기준의 차이다. 이 단일 관찰을 표준 적합 판정으로 취급하지 않으며, 구현 비교에서 그대로 드러낸다.
- `overflow-clip-margin:border-box 8px`은 `overflow:hidden`과 혼합 `hidden/auto`에도 computed value로 남지만 clip 확장 효과는 없다. scrollable overflow 값에 margin이 적용되지 않는 control이다.

## 구현 전 판정 기준

1. fixture의 typed `overflow-x/y` 및 `overflow-clip-margin` computed 값은 고정 Chrome JSON과 정확히 비교한다. 음수 margin은 Chrome과 W3C 문법의 차이를 별도 기록하고 임의로 숨기지 않는다.
2. 지원 node별 border/frame geometry는 각 축 최대 절대 오차 `0.5 CSS px` 이하로 비교한다. 누락 node와 틀린 clip ancestor는 평균 오차에 포함하지 않고 실패 처리한다.
3. PNG의 paint clip 표본과 JSON의 정수 CSS px hit range는 각각 비교한다. 이 둘을 서로 대신하는 증거로 쓰지 않는다. visible target은 clip 바깥 control에서 hit되고, clip 안의 target paint 누수·clip 밖의 hit 누수는 pinned Chrome 결과와 다르면 실패한다.
4. 중첩 clip은 ancestor별 축과 clip edge의 교집합을 비교한다. positioned child는 containing-block chain에 있는 clip만 적용하고, 바깥 containing block으로 escape하는 사례를 그대로 보존한다.
5. `clip`은 20px scroll 요청 뒤 offset 0, `hidden`/scrollable mixed control은 20px을 보인다. float 높이 관계는 `hidden == clip + flow-root > clip`이어야 한다.
6. 이 브라우저 fixture는 rectangle과 horizontal-tb/LTR만 포함한다. border radius, transform, scroll gesture, public CSSOM API, simulator surface는 이후 단계의 별도 gate이며 이 캡처로 지원을 주장하지 않는다.

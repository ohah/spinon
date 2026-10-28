# C11.4 `fit-content()` automatic-minimum floor 사전 비교

## 기준 입력

Pinned WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 `css/css-grid/grid-definition/fit-content-track-sizing.html` 세 번째 assertion을 대상으로 한다. 고정 source SHA-256은 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`이며, 2026-10-28에 확인한 `wpt.live` 파일 hash도 같다. 대상 DOM은 [31–34행](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L31-L34), assertion은 [90–92행](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L90-L92)이다.

Agent Browser가 HeadlessChrome 156에서 upstream WPT harness `OK`, 13개 발견, 13 Pass를 보였다. 대상 root의 used height는 `300px`, `grid-template-rows`는 `10px 100px`이다. 두 item의 computed `min-height`는 `auto`, nested child heights는 `10px`과 `100px`이었다. [전체 WPT 화면](./chrome156-fit-content-auto-min-floor-wpt.png)은 Agent Browser session으로 캡처했다. 이 결과는 upstream Chrome 실행이며 Spinon WPT pass가 아니다.

## CSS Grid 독립 계산

CSS Grid 2025 CRD [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms)는 fit-content track을 `max(minimum, min(limit, max-content))`로 정의하며, 그 minimum은 auto minimum이다. [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content)는 auto min track의 non-spanning item minimum contribution을 사용한다. WPT item의 `min-height:auto`와 `overflow:visible`은 scroll container가 아닌 grid item의 auto minimum을 유지한다 ([§6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)).

| 행 | minimum contribution | max-content contribution | limit | 독립 기대값 |
| --- | ---: | ---: | ---: | ---: |
| 1 | 10px | 10px | 40px | `max(10, min(40, 10)) = 10px` |
| 2 | 100px | 100px | 40px | `max(100, min(40, 100)) = 100px` |

결과는 root `60×300px`, Rows `[10,100]px`이다. `min-height:0`을 두 번째 item에 지정하는 differential control에서는 minimum contribution이 0px이 되고 두 번째 row가 `40px` limit으로 내려온다. 그 입력은 WPT의 `contentLargerThanArgument` assertion ([36–38행](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L36-L38), [94–96행](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L94-L96))을 근거로 삼되, 이번 main assertion과 별도 대조군으로 유지한다.

## 범위

이 파일은 Spinon layout/runtime 관찰을 포함하지 않는다. WPT의 auto outer width는 fixture에서 60px로 고정할 계획이며, 비교는 block-axis row sizing과 60px explicit column을 유지하는 제한 입력으로 한정한다. 나머지 WPT 12개 assertion, 일반 CSS admission, Android 실기기/hardware GPU, iOS compositor-after pixels와 전체 C11.4는 검증하지 않았다. 전체 기계 판독 기준은 [`expected.json`](./expected.json)에 있다.

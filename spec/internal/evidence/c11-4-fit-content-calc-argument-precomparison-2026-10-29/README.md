# C11.4 definite calc `fit-content()` WPT 사전 비교

## 고정 출처

Pinned WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 `css/css-grid/grid-definition/fit-content-track-sizing.html`에서 `calculatedArgument` 장면과 assertion을 확인했다. 저장소 원본 SHA-256은 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`이다. 입력은 definite `height:300px`, `grid-auto-rows:fit-content(calc(10% + 5px))`, `min-height:0`인 item 및 400px 자식이다.

## Agent Browser 관측

2026-10-29 Agent Browser 0.31.1의 HeadlessChrome 156.0.0.0에서 `https://wpt.live/css/css-grid/grid-definition/fit-content-track-sizing.html`를 열었다. Viewport는 1280×633 CSS px, DPR 1이며 WPT harness는 `OK`, 13 tests / 13 Pass였다. Target의 computed `grid-template-rows`는 `35px`, root height는 `300px`, item height는 `35px`, item `min-height`는 `0px`, child computed height는 `400px`이다. WPT 페이지의 auto root width는 1264px로 관측했다.

전체 페이지 캡처는 직전 percentage assertion 검증의 동일한 WPT 화면과 SHA-256이 같아 복사본을 저장하지 않았다. 보존된 [Agent Browser 화면](../c11-4-fit-content-percentage-argument-precomparison-2026-10-29/wpt-fit-content-percentage-argument.png)은 SHA-256 `a7eaaf9fd1d68755272c044c519d277546ec6e070861c0cf663dfa3698c77849`이다. 이 캡처와 13/13 결과는 upstream Chrome만 나타내며 Spinon WPT 통과를 뜻하지 않는다. 브라우저 세션은 종료했고, 기존 agent-browser PID 66774만 남아 있음을 확인했다.

기계 판독 관측과 독립 기대값은 [expected.json](./expected.json)에 기록했다.

## 규범 기반 기대값

CSS Grid 2025 CRD §7.2.1에서 row percentage basis는 grid container의 inner block size다. definite 300px basis에서 10%는 30px이며 길이 항 5px을 더한 calc limit은 35px이다. §11.2의 `fit-content()` 공식은 `max(minimum, min(limit, max-content))`이다. Item의 `min-height:0`은 이 장면에서 automatic minimum을 0으로 만들고, 자식의 400px block size는 max-content contribution 400px을 만든다. 따라서 expected row는 `max(0, min(35, 400)) = 35px`이다. 기대값은 Chrome 결과에서 역산하지 않았다.

이 계획은 cyclic percentage sizing과 definite percentage basis를 분리한다. intrinsic sizing 단계에서 percentage를 auto로 취급하는 순환 경로는 입력에서 발생하지 않는다.

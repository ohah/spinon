# Root HTML column-flow implicit-column 사전 비교

이 비교는 parsed Root HTML Grid 경로의 좁은 후속 작업에서 사용할 기대 geometry를 고정한다. fixture는 명시 2열×2행 Grid, `grid-auto-flow: column`, 두 값짜리 고정 px `grid-auto-columns` pattern, 고정 px 너비·높이를 가진 빈 direct `<div>` 12개를 사용한다.

CSS Grid 명세는 auto-track 크기가 여러 개면 필요한 만큼 순환하고, column flow는 각 열을 채운 다음 열을 추가한다고 규정한다. [CSS Grid Level 2 implicit tracks](https://www.w3.org/TR/css-grid-2/#auto-tracks), [`grid-auto-flow`](https://www.w3.org/TR/css-grid-2/#grid-auto-flow-property), [자동 배치 알고리즘](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)을 참조한다. 독립 기대값은 `expected.json`에 기록했으며 browser 출력에서 복사하지 않았다.

## 관측

Agent Browser에서 DPR 1, Chrome 156.0.8078.80로 열었다. 계산된 explicit+implicit columns는 `30, 40, 15, 25, 15, 25` CSS px이고 rows는 `20, 40` CSS px였다. direct child 12개가 column-major 순서로 예상 셀에 배치됐고, 15/25px auto-column 목록은 두 번 반복됐으며 두 gap도 적용됐다. 최신 DOMRect 값은 `chrome156-observation-pass03.json`, screenshot은 `chrome156-agent-browser-pass03.png`에 있다.

pass02는 runtime host가 지원하는 동등한 `background-color` 선언으로 fixture의 `background` shorthand를 바꿨다. pass03은 runtime fixture에서 필요 없는 `font` 및 border/box alignment shorthand를 제거해 runtime host가 파싱할 수 있는 최소 CSS로 만들었다. CSS geometry, 색상 및 screenshot pixels는 달라지지 않았다. 이전 pass01/pass02 원본과 관측값은 보존한다.

최신 fixture SHA-256: `432991cb1edf2a48186339431a5a4504441b0e787449baa4f8ee2703011df87a`. 최신 screenshot SHA-256: `ea1cf2aa7c5ae19d2a2d23b0eddad58d198a675bcde98cc3caa199a5e61e228c`.

`expected.json`의 px 숫자 배열과 Chrome의 computed-style px 문자열을 숫자 CSS px로 정규화해 비교했다. [pass03 비교 결과](comparison-pass03.json)에서 root geometry·computed track과 12개 DOMRect가 모두 정확히 일치했다. 첫 비교 helper는 numeric gap과 `5px` 문자열을 같은 단위로 바꾸지 않은 verifier 오류가 있어 [실패 산출물](comparison-pass03-checker-pass01.json)로 보존했다. 수정 helper는 px 문자열을 숫자로 정규화하고 통과했다. 이 자료는 해당 fixture에 대한 browser oracle evidence다. Spinon 동작, 일반 CSS Grid 지원, WPT 적합성, paint readback 또는 플랫폼 runtime 정확성을 입증하지 않는다.

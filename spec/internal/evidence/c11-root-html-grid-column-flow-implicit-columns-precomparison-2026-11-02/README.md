# Root HTML column-flow implicit-column 사전 비교

이 비교는 parsed Root HTML Grid 경로의 좁은 후속 작업에서 사용할 기대 geometry를 고정한다. fixture는 명시 2열×2행 Grid, `grid-auto-flow: column`, 두 값짜리 고정 px `grid-auto-columns` pattern, 고정 px 너비·높이를 가진 빈 direct `<div>` 12개를 사용한다.

CSS Grid 명세는 auto-track 크기가 여러 개면 필요한 만큼 순환하고, column flow는 각 열을 채운 다음 열을 추가한다고 규정한다. [CSS Grid Level 2 implicit tracks](https://www.w3.org/TR/css-grid-2/#auto-tracks), [`grid-auto-flow`](https://www.w3.org/TR/css-grid-2/#grid-auto-flow-property), [자동 배치 알고리즘](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)을 참조한다. 독립 기대값은 `expected.json`에 기록했으며 browser 출력에서 복사하지 않았다.

## 관측

Agent Browser에서 DPR 1, Chrome 156.0.8078.80로 열었다. 계산된 explicit+implicit columns는 `30, 40, 15, 25, 15, 25` CSS px이고 rows는 `20, 40` CSS px였다. direct child 12개가 column-major 순서로 예상 셀에 배치됐고, 15/25px auto-column 목록은 두 번 반복됐으며 두 gap도 적용됐다. DOMRect 전체 값은 `chrome156-observation.json`, screenshot은 `chrome156-agent-browser.png`에 있다.

fixture SHA-256: `cd9b085c028371ae71d66a033a9042d1ddc1467781a1b08cb2c358e9893ebf4a`. screenshot SHA-256: `ea1cf2aa7c5ae19d2a2d23b0eddad58d198a675bcde98cc3caa199a5e61e228c`.

machine-readable 기대값과 Chrome 관측값은 정확히 일치한다. 이 자료는 해당 fixture에 대한 browser oracle evidence다. Spinon 동작, 일반 CSS Grid 지원, WPT 적합성, paint readback 또는 플랫폼 runtime 정확성을 입증하지 않는다.

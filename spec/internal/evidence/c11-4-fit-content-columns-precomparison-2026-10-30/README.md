# C11.4 `fit-content()` column precomparison

이 자료는 기존 WPT 기반 `fit-content()` limit 경계의 열 축 mirror 입력을 고정한다. 기준 브라우저는 Agent Browser 0.31.1의 HeadlessChrome 156.0.0.0, viewport 1280×633 CSS px, DPR 1이다. 고정 HTML과 화면 hash는 [expected.json](./expected.json)에 있다. 이 Chrome 실행은 upstream oracle 관찰이며 Spinon WPT 통과나 앱 runtime 결과가 아니다.

CSS Grid 2025 CRD §11.2의 fit-content 함수는 `max(minimum, min(limit, max-content))`로 계산한다. Child width는 100px이므로 세 지원 장면의 max-content contribution은 100px이다. `min-width:0`에서 minimum contribution은 0px이며, 30/40/200px 한계의 독립 used-column 기대값은 각각 30/40/100px이다. Differential control은 `min-width:auto`를 유지하므로 100px automatic minimum이 40px limit보다 크고 used column은 100px이어야 한다.

첫 브라우저 실행에서 200px limit 장면의 root width를 60px로 두었더니 Chrome used column이 60px로 제한됐다. 이는 cap 경계를 분리하지 못하는 입력이었다. 최종 fixture는 그 장면의 definite root width를 300px로 넓혀 content max 100px과 200px limit을 비교하며, 초기 관찰은 최종 oracle 표본에 포함하지 않았다.

최종 고정 장면 네 개 모두 `grid-template-columns`, `grid-template-rows`, root/item/child frames와 overflow-visible 입력을 저장했다. Browser session을 닫았고 `ps`에서 `agent-browser` process가 남지 않은 것을 확인했다. Screenshot은 표시 영역 기록이며 pixel readback 또는 OS compositor 검증이 아니다.

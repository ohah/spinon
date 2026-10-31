# C11.3.3 template-area typed auto-track Chrome 사전 비교

## 범위

Spinon template-area profile에서 거부하던 두 CSS Grid 조합을 분리했다.

- 확정 크기 `100×90px` Grid에서 명시 `10px` 행 뒤에 `grid-auto-rows: minmax(20px, 1fr)`로 area 생성 행 두 개를 채운다.
- 확정 크기 `100×20px` Grid에서 명시 `10px` 열 뒤에 `grid-auto-columns: 25%`로 area 생성 열 두 개를 채운다.

독립 기대 행 크기는 `10px 40px 40px`이다. 두 implicit track은 `20px` 최소 크기를 갖고 남은 확정 높이 `80px`을 절반씩 나눈다. 기대 열 크기는 `10px 25px 25px`이며 각 implicit track은 확정 `100px` inline size의 25%다. Gap은 0이고 정렬은 start다.

## 캡처와 수정 이력

- 브라우저: `HeadlessChrome/156.0.0.0`, DPR 1, viewport `1280×633`.
- 도구: Agent Browser CLI 0.31.1. Terminal Browser는 사용하지 않았다.
- fixture: [`grid-template-areas-typed-auto-track-sizing-c1133-runtime.html`](../../../../../tests/fixtures/css/c11/grid-template-areas-typed-auto-track-sizing-c1133-runtime.html).
- CSSOM track 값과 root-relative frame, 현재 fixture hash 및 screenshot hash는 [observation.json](./observation.json)에 기록했다.
- 현재 기준 화면: [fixture.png](./fixture.png).
- 첫 Android 실행에서 fixture의 불필요한 `margin:4px`가 app root 좌표를 `(4,4)`로 이동시켜 scene-local `(0,0)` oracle 비교가 실패했다. margin을 0으로 정리하고 Chrome 기준 화면을 다시 캡처했다.
- 최초 여백이 있던 화면은 [fixture-before-margin-fix.png](./fixture-before-margin-fix.png)로 보존했다. SHA-256은 `fc5dff9517534b510205f3a6004866abb935558c6ed54d4a4a3a11f6dcf2f929`이다.
- 재캡처 후 Agent Browser 세션을 닫고 임시 HTTP server를 종료했다. 세션 목록은 비어 있다.

## 한계

이는 Chrome 156 oracle 관찰이며 CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.3, 7.5–7.6, 11.4–11.8과 대조한 값이다. Spinon runtime 동작이나 WPT 결과는 아니다. `fit-content()` cap, signed/leading implicit tracks, 일반 author CSS, 기기 렌더링 및 전체 CSS Grid 적합성은 검증하지 않는다.

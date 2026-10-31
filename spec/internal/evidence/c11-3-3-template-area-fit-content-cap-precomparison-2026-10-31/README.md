# C11.3.3 area-backed fit-content cap 사전 비교

이 디렉터리는 `grid-auto-columns: fit-content(40px)`가 `grid-template-areas`가 만든 explicit grid column에서 limit을 실제로 자르는지를 확인하기 위한 Chrome 기준 입력과 관찰을 보관한다. 구현·Spinon runtime·시뮬레이터 결과는 아직 없다.

- 고정 HTML: [grid-template-area-fit-content-cap.html](grid-template-area-fit-content-cap.html)
- 독립 기대값: [expected-values.json](expected-values.json)
- Chrome 실행 조건·관찰: [chrome/README.md](chrome/README.md), [observation.json](chrome/observation.json)
- Agent Browser screenshot: [chrome-156-agent-browser.png](chrome/chrome-156-agent-browser.png)
- 계획 및 별도 계획 검토: [C11.3.3 후속 계획](../../../../plan/c11-3-3-template-area-fit-content-cap.md), [계획 실패 관점 20개](plan-review-20.md)

Chrome 156/Ahem 10px에서 min-content 20px, max-content 110px이고, 40px cap 적용 뒤 computed columns는 `20px 40px`, item frame은 `[20,0,40,60]`이다. screenshot `(59,30)`은 cap 안쪽의 초록, `(60,30)`은 cap 바깥 root 배경이다. 이는 CSS Grid 전체나 Spinon 동작의 적합성 결과가 아니다.

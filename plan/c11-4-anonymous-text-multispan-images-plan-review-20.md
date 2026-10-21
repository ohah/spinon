# C11.4 anonymous text + 복수 spanning image 계획 실패 관점 검토 20개

이 검토는 구현 전 계획의 독립 oracle, scene topology, revision/resource 경계와 검증 판정만 반박한다. 구현 코드나 PR 변경분 검토로 대체하지 않는다.

1. **브라우저 실행 버전이 흔들릴 수 있음** — Chrome for Testing `155.0.8059.85`를 Agent Browser에서 실행했고 UA full version, platform, DPR, viewport를 JSON에 보존했다.
2. **Agent Browser가 설치된 최신 Chrome을 조용히 쓸 수 있음** — 156 세션을 닫고 task-local Chrome 155 바이너리를 `AGENT_BROWSER_EXECUTABLE_PATH`로 지정해 observation을 다시 만들었다.
3. **font fallback이 text 기여를 바꿀 수 있음** — Ahem load check와 asset SHA-256을 보존하고 CSS는 `10px/10px Ahem`으로 고정했다.
4. **image decode race가 natural size를 0으로 만들 수 있음** — 네 scene의 전체 image decode promise 완료와 각 PNG natural size `24×16`을 observation 조건으로 둔다.
5. **scene별 CSS 입력 차이가 비교를 오염시킬 수 있음** — root size, 4개 min-content columns, 3개 fixed rows, gap/alignment/font와 image source를 고정한다.
6. **anonymous text가 직접 element처럼 placement될 수 있음** — fixture는 direct Text node `XX`를 쓰고 computed placement가 불가능한 anonymous item임을 검증한다.
7. **auto-placement가 어느 column을 고르는지 모를 수 있음** — 명시 배치 image가 차지한 cells 뒤 row-sparse 첫 빈 칸을 관찰하고 네 scene 모두 row 1, column 3의 local x를 비교한다.
8. **DOM 순서 변화가 geometry를 바꿀 수 있음** — 2-image와 3-image topology 각각에 reverse-order 대조가 있고 기대 track과 모든 item frame을 검사한다.
9. **expected track이 Chrome 결과에서 역으로 만들어질 수 있음** — 기대값은 §11.4 non-spanning base와 §11.5.1 span distribution에서 별도 유도하고 Chrome CSSOM은 다른 열에 기록했다.
10. **min-content와 automatic minimum을 혼동할 수 있음** — min-content track에는 `auto` min track이 없어 automatic minimum이 발동하지 않는 조건을 명시했다.
11. **replaced image의 opposite-axis ratio가 빠질 수 있음** — natural ratio와 heights 16/32/48에서 min-content widths 24/48/72를 각 item의 input으로 고정한다.
12. **single-span anonymous base가 span group 계산에 누락될 수 있음** — `XX`가 third column에 주는 20px base/growth limit부터 계산에 포함한다.
13. **span-2 contribution을 단순 평균으로 오산할 수 있음** — second image의 track 3 limit을 반영해 남은 contribution이 column 2로 가는 단계를 계획에 기록한다.
14. **span-3 처리 순서가 생략될 수 있음** — 72px item은 기존 span-2 결과 뒤 12px beyond-limit 분배를 적용한다.
15. **두 scene의 row sizing이 content 영향을 숨길 수 있음** — 세 60px fixed rows로 16/32/48px image의 cross-axis contribution과 overlap을 분리한다.
16. **Range text 위치를 anonymous box frame으로 오인할 수 있음** — CSS Range는 glyph-fragment 관찰에 한정하고 app pass는 anchor와 item frame을 별도로 읽는다.
17. **반복 이미지 URL이 image completion을 덮어쓸 수 있음** — 구현 관문은 ID-keyed completions와 natural-size/revision per image를 각각 확인하도록 한다.
18. **두 번째 decode 실패가 부분 snapshot을 남길 수 있음** — decode all을 한 revision에 구성하고 누락/실패에는 partial snapshot/layout 금지 조건을 넣었다.
19. **물리 Android 검증이 사용자 선택과 다를 수 있음** — Android API 37 AVD와 iOS 26.2 Simulator만 gate로 고정하고 실기기 근거는 요구하지 않는다.
20. **제한 slice가 전체 Grid 완료로 오해될 수 있음** — 다른 item topology, WPT, painting과 compositor 한계를 적고 C11.4 checkbox 및 공식 완료 수를 유지한다.

검토 결과 필요한 계획 관문이 빠진 부분은 발견되지 않았다. Chrome 155 사전 observation과 independent rational expectations를 PR diff 검토에서 다시 비교한다.

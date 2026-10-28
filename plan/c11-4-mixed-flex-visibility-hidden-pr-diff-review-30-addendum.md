# C11.4 `visibility:hidden` 계획 PR diff · 이미지 근거 후속 검토

이 addendum은 [계획 PR 변경 검토 30개](./c11-4-mixed-flex-visibility-hidden-pr-diff-review-30.md) 이후 추가한 Agent Browser PR image-load record와 manifest/lifecycle 동기화만 검토한다.

| # | 변경 후속 실패 관점 | 재검토 결과 |
| ---: | --- | --- |
| 1 | PR 본문 이미지가 merge commit 전에는 존재하지 않는 경로를 가리킴 | Raw URL은 PR head full SHA `b71a668dbf313fb20e7f10c87018e366178106a6`와 staged image 경로를 사용하며, PR 화면에서 `complete=true`, `naturalWidth=1280`, `naturalHeight=633`을 Agent Browser로 확인했다. 통과. |
| 2 | 이미지 확인을 단순 DOM 참조로 오해 | [검증 JSON](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/plan-pr-image-agent-browser-pass01.json)은 rendered PR page의 실제 `img.currentSrc`, complete state와 intrinsic dimensions를 기록한다. 통과. |
| 3 | 두 번째 Agent Browser session을 종료하지 않음 | [수명주기 기록](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/agent-browser-lifecycle.json)의 두 run 모두 active session 0과 process 0이다. 종료 후 `session list`와 process scan에서도 비어 있음을 확인했다. 통과. |
| 4 | 새 browser JSON이 hash manifest에서 빠지거나 README와 불일치 | README가 JSON을 연결하고, 새 digest를 `SHA256SUMS.txt`에 넣은 뒤 전체 `shasum -c`를 다시 실행한다. 통과. |
| 5 | browser 확인을 Spinon 앱 runtime 결과로 잘못 기술 | PR body는 screenshot을 Chrome 156 기준으로 명시하고 앱/Simulator 결과가 아니라고 적는다. 계획의 구현·runtime 게이트도 미완료로 유지한다. 통과. |

**판정:** 변경된 PR payload 관점 PASS 5/5, 새 blocker 0개. 구현 검토가 아니며 계획 PR의 범위·공식 상태는 변하지 않았다.

# C11.5 precomparison link fix and Tailnet sync PR diff · 실패 관점 20개

검토 대상은 link-fix PR 전체 diff, Tailnet 동기화, status link, HTTP/hash 확인 및 Agent Browser 캡처다. 계획 PR #1130의 계획 검토와 diff 검토를 대체하지 않는다.

| # | 실패 관점 | 결과 및 방어 |
|---:|---|---|
| 1 | link-fix PR에 runtime 구현이 섞임 | **통과.** diff는 evidence markdown/JSON/PNG와 상태 링크만 수정한다. |
| 2 | fixture가 Rspress root 밖 relative route로 남음 | **통과.** fixture source는 GitHub main 링크로 바꿨다. |
| 3 | JSON이 `.json.html` dead route로 변환 | **통과.** Chrome observation과 expected JSON은 GitHub source links다. |
| 4 | PNG normal link가 `.png.html` route로 변환 | **통과.** screenshot은 Markdown image asset syntax를 사용한다. |
| 5 | image asset 경로가 evidence folder 밖을 가리킴 | **통과.** `./chrome156.png`는 같은 디렉터리이며 tracked 파일이다. |
| 6 | 수정으로 Chrome screenshot/source SHA가 손상 | **통과.** fixture와 Chrome screenshot SHA는 manifest와 기존 evidence 값 그대로다. |
| 7 | Tailnet preview가 source보다 앞선/다른 HTML을 제공 | **통과.** source와 HTTPS response는 551498 bytes, SHA-256 `0301e2c299cbebd31811f405381883cec6dbdf124b1dd3a346643829e455e87e`로 일치한다. |
| 8 | 중간에 끊긴 쓰기가 공개 page를 손상 | **통과.** 기존 파일은 backup 후 같은 filesystem temp file에서 atomic replace 했다. |
| 9 | 교체 전 Tailnet 페이지가 보존되지 않음 | **통과.** backup 경로와 기존 SHA-256을 sync manifest/README에 기록했다. |
| 10 | 확인하지 않은 Tailscale/Funnel route를 기록 | **통과.** `tailscale serve status`에서 hostname→`127.0.0.1:8768` 경로를 확인했다. |
| 11 | live route가 error page 또는 stale HTML을 줌 | **통과.** HTTPS GET은 200을 반환했고 body SHA가 local source와 일치했다. |
| 12 | Tailnet screenshot이 로컬 page를 캡처 | **통과.** Agent Browser URL은 실제 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`이다. |
| 13 | 화면에서 계획 상태와 공식 count를 확인하지 않음 | **통과.** 캡처 전 DOM 검사에서 C11.5 계획 카드, `runtime 구현은 없다`, `72/213`을 읽었다. |
| 14 | 계획 및 사전 비교 링크가 잘못됨 | **통과.** Agent Browser DOM의 두 href가 main에 병합된 계획과 precomparison README 경로다. |
| 15 | screenshot viewport나 DPR이 누락/오표기 | **통과.** Agent Browser는 1280×633 CSS px, DPR 1을 기록했고 PNG는 1280×633이다. |
| 16 | Agent Browser 세션/daemon이 남음 | **통과.** browser session을 닫고 종료 뒤 agent-browser process가 없는 것을 확인했다. |
| 17 | 실행 중이던 Tailnet server를 재시작/종료 | **통과.** PID 28040은 보존했고 파일만 교체했다. |
| 18 | `STATUS.md` checkbox/공식 count가 증가 | **통과.** 계획 상태에 evidence link만 추가하며 C11 checkbox와 `72/213`을 유지한다. |
| 19 | 전체 Rspress build가 성공했다고 과장 | **통과.** build exit 1과 pre-existing dead-link blockers를 기록하고 이 작업 경로가 error log에 없음을 확인했다. |
| 20 | STATUS.html도 새 상태로 게시됐다고 과장 | **통과.** sync 범위는 정적 roadmap.html이며 README는 STATUS.html을 재생성하지 않았다고 명시한다. |

## 결론

PR은 precomparison links를 Rspress-safe 형태로 고치고 요청된 Tailnet roadmap route만 백업·hash 확인 후 갱신한다. 전체 docs build와 `STATUS.html`은 갱신된 것으로 주장하지 않는다.

# C11.4 roadmap 상태·화면 실패 관점 검토 20개

대상은 PR #1284 병합 뒤의 공식 상태, 칸반 메모, Tailnet desktop/mobile 화면이다. 이 검토는 제품 runtime 검증과 구분한다.

| # | 독립 실패 관점 | 판정과 근거 |
| --- | --- | --- |
| 1 | 이미 끝난 24순서 PR #1280을 새 진행 작업처럼 유지하는 경우 | 통과. 고정된 `Grid 24순서 구현`을 제거하고 다음 후보 텍스트로 교체했다. |
| 2 | 병합된 PR #1284가 공개 로드맵에 빠지는 경우 | 통과. 최신 #1284와 merge SHA `f2f9312e...`를 hero history와 문서에 표시했다. |
| 3 | 공식 상태 대장이 C11.4를 완료했다고 오해시키는 경우 | 통과. C11.4/C11 checkboxes는 미완료다. |
| 4 | 제한 slice를 공식 완료 수에 반영하는 경우 | 통과. 공식 `78/214`를 그대로 둔다. |
| 5 | 로드맵에서 이전 slice 수치를 새 후보 검증값으로 섞는 경우 | 통과. #1284의 4/4 및 새 계획의 120/120·9,480 checks를 별도 카드로 분리했다. |
| 6 | 미구현 계획을 실제 구현 중이라고 표시하는 경우 | 통과. 30-order tail 후보는 계획·사전 비교 완료 후 `다음` 열로 표시하고, 코드는 시작 전이라고 쓴다. |
| 7 | 30개 unique order가 이미 제품에서 지원된다고 주장하는 경우 | 통과. 링크 제목과 문구가 후보/계획임을 밝히며 PR #1284의 한 order만 현재 구현으로 기록한다. |
| 8 | 계획의 120 scene이 runtime 검증이나 Simulator 결과처럼 보이는 경우 | 통과. Chrome precomparison이라고 표시하고 제품 코드와 분리했다. |
| 9 | fixed-px precomparison이 arbitrary min/max content parity로 과대해석되는 경우 | 통과. 고정 payload의 min/max 한계를 roadmap plan과 README에 남긴다. |
| 10 | workflow/status가 비었는데 CI green이라고 게시하는 경우 | 통과. PR #1284 workflow/status 미보고를 명시한다. |
| 11 | OS screenshot을 PixelCopy/Metal surface readback으로 표현하는 경우 | 통과. PR #1284 이미지가 visual smoke임을 유지한다. |
| 12 | Tailnet 서버를 재시작하거나 사용자 설정을 지우는 경우 | 통과. 게시 절차는 PID 28040과 연결을 유지하고 삭제/재시작을 금지한다. |
| 13 | Preview 파일을 백업 없이 덮어쓰거나 부분 쓰기로 노출하는 경우 | 통과. pass02에서 17개 교체 파일을 사전 hash allowlist와 대조하고 백업 후 같은 디렉터리에서 원자 rename했다. backup/source hash는 [`selected-publication-result-pass02.json`](./selected-publication-result-pass02.json)에 있다. |
| 14 | 기존 browser reader가 열린 파일의 내용을 갑자기 바꾸는 경우 | 통과. preflight에서 17개 교체 경로 모두 열린 reader PID가 없음을 확인했고 atomic rename을 사용했다. 새 파일 hash는 게시 직후 재검증했다. |
| 15 | 전체 docs tree의 대형 untracked evidence를 잘못 게시하는 경우 | 통과. 이전 selected root와 명시된 route 목록만 사용한다. |
| 16 | 새 plan 또는 precomparison route가 Tailnet에서 404가 되는 경우 | 통과. 로컬/Tailnet에서 선택 문서·이미지 경로와 생성 asset 68개가 HTTP 200이고 manifest SHA와 일치했다. [`served-verification-pass02.json`](./served-verification-pass02.json). |
| 17 | 로드맵 내부 anchor/image가 잘못된 경로로 이어지는 경우 | 통과. Agent Browser로 다음 계획, 사전 비교 README, 1440×2196 PNG를 열었다. 이미지 완료 상태와 `image/png`도 확인했다. |
| 18 | 모바일 viewport가 비어 있는 active 열에 고정돼 다음 후보를 숨기는 경우 | active task가 없을 때 `할 일` 열에서 시작하도록 로직을 바꿨다. |
| 19 | 390px 화면에서 카드나 문서가 가로로 잘리거나 overflow하는 경우 | 통과. mobile viewport와 document `scrollWidth`가 390px로 같고 `할 일` lane의 다음 후보 카드가 첫 화면에 보인다. desktop document `scrollWidth`도 1440px다. |
| 20 | browser를 종료하지 않거나 tmux/server를 같이 종료하는 경우 | 통과. Agent Browser 세션을 닫은 뒤 Chrome/Agent Browser process가 0개다. Tailscale `Running`, preview PID 28040, tmux `spinon` 세션은 유지했다. [`agent-browser-lifecycle-pass02.json`](./agent-browser-lifecycle-pass02.json). |

## 결론

실제 Tailnet 화면은 PR #1284 병합 전 카드와 24순서 상태를 보여줬다. 동기화된 화면은 최신 merge 사실을 반영하고 계획 완료된 30-order slice를 `할 일`로 공개하며 진행 중 수는 `0/2`다. 20개 실패 관점은 게시 preflight/result, 68개 route·asset 검증과 Agent Browser desktop/mobile evidence를 대조해 모두 통과했다. 이는 CSS Grid 동작의 구현 완료를 의미하지 않는다.

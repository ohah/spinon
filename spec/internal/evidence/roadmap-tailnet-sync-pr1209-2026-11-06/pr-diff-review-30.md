# PR #1209 Tailnet roadmap sync evidence diff 적대 검토 30개

기준: `spec/STATUS.md`의 sync evidence link와 `spec/internal/evidence/roadmap-tailnet-sync-pr1209-2026-11-06/`의 README, JSON, screenshots, hashes. Tailnet 제공본은 PR #1209 merge SHA `710b359a7e57ce59110f384a1b7b458f0cafc95f` source에서 갱신했다.

| # | 독립 실패 관점 | 판정 | 확인 |
|---:|---|---|---|
| 1 | 사용자가 정한 Tailnet URL을 그대로 확인했는가 | PASS | `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`에서 읽었다. |
| 2 | 로컬 preview와 Tailnet response가 동일한가 | PASS | 두 GET body 모두 593,059 bytes와 같은 SHA-256이다. |
| 3 | 실제 route의 제공 파일을 추측으로 골랐는가 | PASS | 교체 전 body hash가 preview root `roadmap.html`과 byte 단위로 일치했다. |
| 4 | canonical source를 명확히 기록했는가 | PASS | `spec/public/roadmap.html`의 main merge 후 hash와 byte 수를 기록했다. |
| 5 | target이 merge된 canonical source와 정확히 일치하는가 | PASS | atomic replace 후 target/source hash가 `ed76e6d0…`로 같다. |
| 6 | 교체 전 content를 잃지 않았는가 | PASS | 이전 590,947-byte 파일을 같은 preview backup tree에 보존했다. |
| 7 | backup의 길이와 hash가 일치하는가 | PASS | backup SHA가 교체 전 served/root SHA `02c61ff2…`와 같다. |
| 8 | backup path collision 가능성을 확인했는가 | PASS | unique PR-specific `pass01` path가 비어 있는지 확인했다. |
| 9 | 기존 target의 open handle을 고려했는가 | PASS | `lsof`에서 target file handle이 없음을 확인했다. |
| 10 | 파일이 부분적으로 노출될 수 있는가 | PASS | 동일 디렉터리 temp file 후 atomic `os.replace`를 사용했다. |
| 11 | preview server를 임의로 중단·재시작했는가 | PASS | PID 28040을 유지했고 재시작하지 않았다. |
| 12 | Tailnet backend/Funnel 상태가 확인됐는가 | PASS | Tailscale status는 Running이고 macstudio Funnel이 켜져 있었다. |
| 13 | GitHub Pages를 배포했는가 | PASS | 자동/수동 Pages 배포를 실행하지 않았다. |
| 14 | 공식 완료 수가 잘못 증가했는가 | PASS | 78/214 표기를 유지한다. |
| 15 | C11 상위 checkbox가 완료 처리됐는가 | PASS | 공식 C11 checkbox는 미완료 상태다. |
| 16 | C11.4 상위 checkbox가 완료 처리됐는가 | PASS | 공식 C11.4 checkbox는 미완료 상태다. |
| 17 | 표시된 PR merge id가 잘못됐는가 | PASS | PR #1209와 merge SHA `710b359a…`가 일치한다. |
| 18 | 로드맵 카드의 PR #1208 merge 내용이 최신 source에 있는가 | PASS | Agent Browser DOM에 #1208 카드와 merge SHA가 나타난다. |
| 19 | Agent Browser를 사용해 실제 제공 페이지를 확인했는가 | PASS | 실제 Tailnet URL을 열어 desktop/mobile viewport를 확인했다. |
| 20 | Terminal Browser를 사용했는가 | PASS | 사용하지 않았다. |
| 21 | desktop screenshot viewport와 기록이 일치하는가 | PASS | 1440×1000 viewport 및 1440×1000 PNG dimensions다. |
| 22 | desktop 문서가 수평 overflow되는가 | PASS | document width 1440px로 viewport와 일치한다. |
| 23 | desktop PR #1208 카드가 화면에 보이는가 | PASS | card x=90px, width=928px, top≈80px다. |
| 24 | mobile screenshot viewport와 기록이 일치하는가 | PASS | 390×844 viewport 및 390×844 PNG dimensions다. |
| 25 | mobile 문서가 수평 overflow되는가 | PASS | document width 390px로 viewport와 일치한다. |
| 26 | mobile PR #1208 카드가 화면에 보이는가 | PASS | card x=14px, width=362px, top≈70px다. |
| 27 | screenshot/manifest SHA가 파일과 일치하는가 | PASS | desktop/mobile PNG와 canonical source를 manifest에 고정했다. |
| 28 | PR·계획·evidence·화면 링크를 카드에서 확인했는가 | PASS | DOM link list에 target URL들이 표시된다. |
| 29 | Agent Browser가 끝난 뒤 browser process가 남았는가 | PASS | 세션을 닫았고 `pgrep -x agent-browser` 결과가 비었다. |
| 30 | 무관한 process나 user data를 정리했는가 | PASS | preview server 및 기타 user data는 건드리지 않았다. |

**결과:** 30/30 PASS. 변경은 상태/evidence 문서와 Agent Browser screenshots뿐이며 제품 코드를 바꾸지 않는다.

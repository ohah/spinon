# PR #1214 Tailnet roadmap 검증 기록 · PR diff 적대 검토 30개

검토 대상은 `docs/roadmap-tailnet-sync-pr1213-2026-11-06`의 PR #1214 전체 diff다. 저장소의 PR 기준에 따라 서로 다른 실패 경로와 경계 조건을 확인했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 증거의 원본 docs PR이 실제 병합 PR과 일치하지 않을 수 있음 | PASS — #1213은 `cf3b29cab58f66a8993845123b0873e1d414a5cd`로 병합됐고 evidence의 source commit과 일치한다. |
| 2 | source 경로가 다른 roadmap 산출물을 가리킬 수 있음 | PASS — source는 공식 `spec/public/roadmap.html`로 특정한다. |
| 3 | source byte 수가 실제 저장소 파일과 다를 수 있음 | PASS — 595,374 bytes이며 현재 파일과 기록된 값이 일치한다. |
| 4 | source SHA가 실제 파일 hash와 다를 수 있음 | PASS — 현재 SHA-256은 `0544cb8053d1abf6ba073d64ae53de090d07c4604b20915ae8caf96a3d41ce44`로 증거와 일치한다. |
| 5 | 동기화 전 제공본의 식별값이 빠져 이전·새 콘텐츠를 구별하지 못할 수 있음 | PASS — 이전 파일의 593,059 bytes와 SHA를 기록했다. |
| 6 | backup 경로가 기존 serving directory와 섞이거나 불명확할 수 있음 | PASS — serving 경로와 별도의 timestamped backup 경로를 기록했다. |
| 7 | backup 내용이 이전 제공본과 동일하지 않을 수 있음 | PASS — `byte_identical_to_previous_copy`가 true이고 backup SHA가 이전 제공본 SHA와 일치한다. |
| 8 | 교체 시점에 기존 파일을 읽는 process가 있을 수 있음 | PASS — 교체 전 open file handle이 없었다고 preflight에 기록했다. |
| 9 | 임시 복사본이 source와 다른 채로 publish될 수 있음 | PASS — temporary copy SHA와 source SHA가 일치한다. |
| 10 | 대상 파일이 중간 상태로 노출될 수 있음 | PASS — 같은 디렉터리 임시 파일에서 atomic rename으로 교체했다고 기록했다. |
| 11 | 동기화가 preview server 재시작을 유발해 다른 작업에 영향을 줄 수 있음 | PASS — server 재시작은 없었고 `process_unchanged`가 true다. |
| 12 | 보고된 서버가 실제로 기대한 local preview process가 아닐 수 있음 | PASS — PID, command와 `127.0.0.1:8768` listen endpoint를 기록했다. |
| 13 | Tailscale 상태가 내려간 채 원격 확인만 통과한 것으로 오인될 수 있음 | PASS — backend running, TUN, Funnel endpoint 상태를 기록했다. |
| 14 | 로컬 URL이 다른 파일을 반환할 수 있음 | PASS — local URL은 `/spinon/roadmap.html`의 local preview endpoint로 고정했다. |
| 15 | 원격 검증이 HTTP 대신 다른 protocol 또는 다른 host를 검사할 수 있음 | PASS — 실제 Tailnet HTTPS URL을 기록했다. |
| 16 | local endpoint가 오류 응답을 돌려줄 수 있음 | PASS — local HTTP status는 200으로 기록됐다. |
| 17 | Tailnet endpoint가 오류 응답을 돌려줄 수 있음 | PASS — Tailnet HTTPS status는 200으로 기록됐다. |
| 18 | HTTP 응답 길이가 source와 다를 수 있음 | PASS — local·Tailnet 응답 모두 595,374 bytes다. |
| 19 | 바이트 수만 같고 콘텐츠가 바뀌었을 수 있음 | PASS — source, local body, Tailnet body의 SHA-256이 같다. |
| 20 | 문서 화면이 PR #1212 대신 이전 카드나 잘못된 카드를 보여줄 수 있음 | PASS — desktop/mobile DOM evidence가 모두 PR #1212 카드를 특정한다. |
| 21 | 화면에 표시된 완료 수가 source의 공식 집계와 어긋날 수 있음 | PASS — `78/214`가 보이며 이 문서 PR은 집계 값을 수정하지 않는다. |
| 22 | 제한 slice를 C11.4/C11 완료로 승격해 표시할 수 있음 | PASS — C11.4와 C11 모두 미완료라는 상태를 명시한다. |
| 23 | desktop viewport가 기록과 다르거나 문서 폭이 viewport를 넘을 수 있음 | PASS — 1440×1000 CSS px와 document width 1440, horizontal overflow false를 기록했다. |
| 24 | desktop 카드가 화면 바깥에 위치할 수 있음 | PASS — 카드 x=90, width=928이 viewport 안에 있다. |
| 25 | desktop 이미지가 DOM geometry와 다른 캡처일 수 있음 | PASS — PNG는 manifest에 포함되고 실제 SHA-256이 Agent Browser evidence와 일치한다. |
| 26 | mobile viewport가 기록과 다르거나 문서 폭이 viewport를 넘을 수 있음 | PASS — 390×844 CSS px와 document width 390, horizontal overflow false를 기록했다. |
| 27 | mobile 카드가 화면 밖으로 밀리거나 너비가 viewport를 넘을 수 있음 | PASS — 카드 x=14, width=362로 390px viewport 안에 있다. |
| 28 | mobile 카드 콘텐츠가 desktop 전용 상태로 잘못 표시될 수 있음 | PASS — mobile DOM evidence가 같은 PR #1212 카드와 별도 mobile geometry를 기록한다. |
| 29 | mobile screenshot이 해시 불일치 또는 누락으로 다른 파일을 가리킬 수 있음 | PASS — PNG 파일 hash가 evidence JSON과 일치하고 manifest 검증 대상이다. |
| 30 | Agent Browser 또는 preview server 종료 상태를 혼동해 사용자의 process를 남기거나 종료할 수 있음 | PASS — Agent Browser 종료를 확인했고 preview server PID `28040`은 유지했다. |

## 최종 판정

검토에서 수정이 필요한 불일치는 발견하지 못했다. 이 PR은 이미 수행한 Tailnet roadmap 동기화와 화면 확인을 기록하며, 새 C11 구현·공개 CSS 지원·GitHub Pages 배포·공식 완료 수 변경을 포함하지 않는다. GitHub PR check rollup은 비어 있어 CI 성공으로 해석하지 않는다.

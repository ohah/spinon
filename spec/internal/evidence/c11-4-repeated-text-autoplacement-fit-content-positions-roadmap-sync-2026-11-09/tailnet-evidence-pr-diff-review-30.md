# PR #1304 Tailnet 제공본·Agent Browser 근거 diff 검토 30개

검토 대상은 Tailnet roadmap 제공본 갱신 script, preflight/copy reports, Agent Browser DOM 결과와 screenshots다. 각 항목은 제공 대상·교체·해시·화면 근거에서 서로 다른 실패 가능성을 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | private preview 대신 GitHub Pages를 배포한다. | PASS — Pages 배포가 없고 report에 `not requested`로 남는다. |
| 2 | merge 전 문서를 preview에 복사한다. | PASS — source HEAD `181192fb716e3283ccac6f52c5afef642ff9924b`는 PR #1303 merge 뒤 `origin/main`과 일치했다. |
| 3 | 잘못된 roadmap 파일을 source로 사용한다. | PASS — source는 `spec/public/roadmap.html`이다. |
| 4 | source bytes/hash가 branch blob과 다르다. | PASS — 647,470 bytes와 `99e83769170774a79eea06d7cf9742731289ca0fa46230b42cb8889f7a4e7a09`로 고정했다. |
| 5 | backup 경로가 기존 자료와 충돌한다. | PASS — 새 날짜·PR 전용 경로를 만들고 overwrite 전에 존재 여부를 확인한다. |
| 6 | 기존 preview가 보존되지 않는다. | PASS — 교체 전 bytes를 별도 backup에 복사했다. |
| 7 | backup이 기존 제공본과 다른 바이트를 가진다. | PASS — 이전 645,982 bytes와 SHA-256 `f5e5e9c5d4e3e2c228876185a5687817066c3d27ca12da8ee77a3355c1a3e16e`가 일치한다. |
| 8 | 이전 target과 Tailnet 응답이 이미 불일치한다. | PASS — 교체 전에 loopback과 HTTPS 모두 이전 SHA에 일치했다. |
| 9 | Tailscale backend가 중지된 상태에서 sync를 시작한다. | PASS — preflight에서 `Running`을 요구했다. |
| 10 | port 8768의 다른 서비스에 파일을 제공하게 한다. | PASS — listener PID 28040과 `tailnet-preview-server.mjs` 명령을 확인했다. |
| 11 | listener가 loopback 밖에 노출된다. | PASS — `127.0.0.1:8768` listener 주소를 확인했다. |
| 12 | 기존 서버가 target을 열고 있어 rename 동작이 불확실하다. | PASS — target 열린 file handle 0개를 확인했다. |
| 13 | 서버 재시작으로 다른 미리보기 연결을 끊는다. | PASS — preview server를 재시작하지 않았다. |
| 14 | 임시 파일을 다른 filesystem에 만들어 rename이 atomic하지 않다. | PASS — target과 같은 디렉터리에 임시 파일을 기록했다. |
| 15 | atomic rename 전에 file data를 flush하지 않는다. | PASS — temporary file을 `fsync`한 뒤 rename했다. |
| 16 | partial write가 최종 roadmap route로 노출된다. | PASS — 기존 파일은 완성된 임시 파일 rename 뒤 교체됐다. |
| 17 | 사후 제공 hash가 source와 다르지만 그대로 둔다. | PASS — mismatch는 실패 처리되고 기존 backup으로 rollback한다. |
| 18 | 이전 제공본 backup이 target과 다르지만 증거를 통과 처리한다. | PASS — backup SHA를 이전 target SHA와 비교했다. |
| 19 | loopback HTTP가 이전 bytes를 캐시해 전달한다. | PASS — `no-cache` preview와 fresh fetch로 새 source SHA를 확인했다. |
| 20 | Tailnet HTTPS가 이전 bytes를 캐시해 전달한다. | PASS — HTTPS 200 및 새 source SHA를 확인했다. |
| 21 | 두 제공 route 중 하나만 성공한다. | PASS — loopback과 Tailnet 두 응답을 각각 기록했다. |
| 22 | Desktop screenshot이 모바일 viewport로 저장된다. | PASS — JSON과 PNG는 1440×1000 viewport다. |
| 23 | Mobile screenshot이 desktop viewport로 저장된다. | PASS — JSON과 PNG는 390×844 viewport다. |
| 24 | Desktop page가 가로 overflow하지만 놓친다. | PASS — document/body width가 1440px이다. |
| 25 | Mobile page가 가로 overflow하지만 놓친다. | PASS — document/body width가 390px이다. |
| 26 | 캡처 페이지에서 PR #1302 최신 카드가 빠졌다. | PASS — DOM 결과와 screenshots에 카드 제목·merge SHA가 있다. |
| 27 | 로드맵 상단의 완료 수가 공식 상태와 다르다. | PASS — Agent Browser가 `78/214`를 읽었다. |
| 28 | C11.4를 제한 구현 병합만으로 완료 표시한다. | PASS — 카드가 C11.4/C11 미완료를 표시한다. |
| 29 | 다음 slice가 미정인데 완료된 계획으로 표시한다. | PASS — “후속 bounded slice 후보 미선정”을 표시한다. |
| 30 | 캡처 뒤 Agent Browser daemon을 남긴다. | PASS — 전용 session을 닫고 그 daemon/browser process가 종료된 것을 확인한다. |

검토는 지정된 roadmap 제공 route만 다룬다. 전체 문서 사이트 link audit, GitHub Pages, 실기기 및 C11.4 전체 완료를 뜻하지 않는다.

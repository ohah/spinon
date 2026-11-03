# Tailscale C11.1 로드맵 게시 근거 PR diff 검토 · 30개 관점

이 검토는 PR #1119 병합 후 생성된 preflight/copy/verify manifest, Agent Browser 화면과 동기화 설명이 서로 맞는지 살핀다. 공개 페이지와 동기화 코드의 앞선 PR diff 검토와 다른 사후 증거 경로에 초점을 맞췄다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 게시한 HTML이 아직 미병합 후보에서 옴 | 보고서 source commit은 병합된 `origin/main` `c1c8eae0f52f1d7a0358a813c32b879fc4879692`다. PASS |
| 2 | PR #1119의 merge SHA 또는 시각 불일치 | `c1c8eae0f52f1d7a0358a813c32b879fc4879692`, `2026-11-03T13:13:23Z`가 GitHub와 일치한다. PASS |
| 3 | preview 대상 이전값을 preflight에서 누락 | 기존 `545235` bytes와 이전 SHA를 preflight에 보존했다. PASS |
| 4 | 이전 page SHA가 실제 보존 파일과 다름 | backup SHA `e715…23f0`가 preflight의 기존 SHA와 같다. PASS |
| 5 | backup 경로가 다른 preview 버전을 덮음 | 전용 `roadmap-tailnet-sync-2026-11-03/roadmap.html` 경로를 사용했다. PASS |
| 6 | 복사 후 로컬 파일 길이가 원본과 다름 | 원본과 local preview가 모두 `546771` bytes다. PASS |
| 7 | 로컬 복사본에 동일 길이의 다른 내용이 있음 | local SHA `c275…f2df3`가 merged source와 같다. PASS |
| 8 | Tailnet 응답이 다른 길이나 SHA를 서빙 | remote `546771` bytes와 SHA가 source 및 local preview와 일치한다. PASS |
| 9 | remote route가 성공 응답 대신 오류를 반환 | HTTPS 검증 응답은 HTTP 200이다. PASS |
| 10 | browser cache가 이전 응답을 반환 | 검증 요청에 고정 cache-busting query와 `no-cache` header를 썼다. PASS |
| 11 | HTTPS가 아닌 origin이나 다른 host에 게시 | 스크립트는 HTTPS 및 `macstudio.tailed42f2.ts.net` hostname을 강제한다. PASS |
| 12 | 다른 public route까지 함께 바뀜 | preflight/copy/verify 모두 `roadmap.html` 한 route만 기록한다. PASS |
| 13 | 동기화가 기존 file을 삭제 | copy report와 target 검토에서 삭제는 없었다. PASS |
| 14 | 이전 HTML이 backup 전에 교체 | backup을 먼저 만들고 hash를 확인한 후 atomic rename했다. PASS |
| 15 | backup이 원본과 다름 | backup bytes와 이전 target hash가 일치한다. PASS |
| 16 | 대상이 preflight 후 바뀌었는데 덮어씀 | copy 모드가 target의 존재·크기·SHA를 재검사한다. PASS |
| 17 | 저장소에서 달라진 HTML이 실수로 게시 | 스크립트가 local bytes를 `origin/main` blob과 대조한다. PASS |
| 18 | backup 경로 기존 파일이 조용히 덮임 | `wx`와 고정 경로 선검사로 재사용을 거부한다. PASS |
| 19 | 임시 파일이 target route에 부분 노출 | 임시 파일을 같은 directory에 기록하고 atomic rename했다. PASS |
| 20 | copy 성공 보고가 실제 target hash 전 생성 | target을 다시 읽어 source hash와 대조한 뒤 `copy.json`을 썼다. PASS |
| 21 | verify가 copy 단계 없이 성공 | verify는 일치하는 `copy.json`을 먼저 요구한다. PASS |
| 22 | #1117 카드가 live DOM에 없음 | Agent Browser에서 PR #1117 카드와 merge SHA가 보인다. PASS |
| 23 | 후속 문서 PR 링크가 일반 텍스트로 표시 | PR #1118은 실제 anchor link로 DOM에 존재한다. PASS |
| 24 | official count 또는 완료 상태가 바뀜 | live DOM에 `72/213` 및 C11 미완료가 보인다. PASS |
| 25 | desktop viewport에서 page가 수평 overflow | 1280px에서 `documentWidth=1280`, overflow=false다. PASS |
| 26 | mobile viewport에서 page/card가 수평 overflow | 390px에서 `documentWidth=390`, card bounds 14–376px, overflow=false다. PASS |
| 27 | desktop screenshot이 다른 크기로 저장 | 증거 PNG는 1280×720이고 SHA-256을 기록했다. PASS |
| 28 | mobile screenshot이 다른 크기로 저장 | 증거 PNG는 390×844이고 SHA-256을 기록했다. PASS |
| 29 | browser 또는 temporary server가 작업 후 남음 | Agent Browser와 port 8877 preview server 모두 종료를 확인했다. PASS |
| 30 | PR check가 없는데 CI 성공으로 기록 | #1119 check rollup은 비어 있어 CI 통과를 주장하지 않는다. PASS |

30개 사후 증거 관점의 수치·경로·상태가 일치했다. 공개 페이지는 C11.1/C11 완료를 표시하지 않는다.

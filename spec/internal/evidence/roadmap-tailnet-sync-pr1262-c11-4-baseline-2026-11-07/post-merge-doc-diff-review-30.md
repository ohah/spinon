# PR #1262 병합 후 roadmap·Tailnet 기록 문서 diff 검토 30개

구현 PR #1262의 병합 상태, plan, 공식 집계, Tailnet 제공본, Agent Browser 화면과 종료 기록을 서로 대조했다. 코드 구현/runtime 검토와 PR #1262 diff 리뷰에 기대지 않는 문서·배포 기록 실패 관점이다.

| # | 독립 검토 관점 | 판정 |
|---:|---|---|
| 1 | 구현 PR이 열린 상태 또는 draft로 남았는가 | PR #1262는 merged다. 통과. |
| 2 | plan의 병합 시각이 GitHub `mergedAt`과 일치하는가 | `2026-11-07T09:46:46Z`를 기록했다. 통과. |
| 3 | plan/README의 merge SHA가 GitHub와 같은가 | `e275882ecceb1a2e2545eb827cdcde3673d8efd2`와 일치한다. 통과. |
| 4 | Rebase and merge 결과를 다른 merge 방식으로 적었는가 | Rebase and merge로 기록했다. 통과. |
| 5 | PR status check가 없는 것을 CI green으로 표현했는가 | PR check rollup이 비었다는 기존 PR 근거를 유지하고 CI 통과를 주장하지 않는다. 통과. |
| 6 | merge 뒤 계획에 아직 PR 병합 전이라고 남았는가 | 계획 상단을 merge SHA와 후속 evidence 링크로 갱신했다. 통과. |
| 7 | 제한 slice 병합을 C11.4 전체 완료로 확대했는가 | bounded baseline fallback slice만 병합됐다고 구분한다. 통과. |
| 8 | 상위 C11 완료 상태를 바꿨는가 | C11.4/C11 미완료를 명시한다. 통과. |
| 9 | 공식 완료 수가 잘못 증가했는가 | `78/214`를 유지한다. 통과. |
| 10 | `spec/STATUS.md`를 수정하지 않고 갱신했다고 주장하는가 | 상태 대장은 변경하지 않았고 미완료 판정을 유지한다고 기록한다. 통과. |
| 11 | 내부 계약 숫자 버전이 후속 동기화에서 바뀌었다고 썼는가 | 계약 버전에 대한 변경 주장이 없다. 통과. |
| 12 | nested Grid·일반 CSS·WPT·실기기 지원으로 오해시키는가 | README에서 이를 근거 범위 밖으로 제한한다. 통과. |
| 13 | Tailnet이 제공하는 실제 원본 경로를 잘못 적었는가 | standalone route와 `spec/public/roadmap.html` 직접 제공 경로를 적었다. 통과. |
| 14 | source HEAD가 구현 PR merge revision과 일치하는가 | preflight source HEAD는 merged main `e275882e…`다. 통과. |
| 15 | source byte count/hash가 preflight와 현재 파일에서 일치하는가 | 623,127 bytes / `32650007…`이며 served verification과 일치한다. 통과. |
| 16 | 교체 전 Tailnet 파일의 hash가 확인됐는가 | old route는 622,905 bytes / `980c92d9…`로 기록했다. 통과. |
| 17 | 기존 제공본 backup이 빠졌거나 다른 내용인가 | backup은 622,905 bytes이고 이전 served hash와 byte-identical이다. 통과. |
| 18 | 기존 backup을 덮어썼는가 | 고유한 PR #1262 경로를 쓰고 preflight에서 collision을 거부한다. 통과. |
| 19 | 선택 범위를 넘어 파일을 복사·삭제했는가 | `roadmap.html` 하나만 교체했고 삭제는 없다. 통과. |
| 20 | 임시 파일 교체 방식이 안전하지 않은가 | same-directory temporary file과 atomic rename을 사용한다. 통과. |
| 21 | Tailscale backend가 Running인지 확인했는가 | preflight에서 `Running`을 단언했다. 통과. |
| 22 | 다른 preview server를 대상으로 삼았는가 | PID 28040의 command와 preview root/cwd를 대조했다. 통과. |
| 23 | 교체 중 대상 파일이 열려 있었는가 | preflight와 copy에서 열린 handle 수는 0이다. 통과. |
| 24 | local HTTP가 오류 또는 다른 내용을 제공하는가 | localhost route는 HTTP 200 및 source SHA를 반환한다. 통과. |
| 25 | Tailnet HTTPS가 오류 또는 다른 내용을 제공하는가 | Tailnet route는 HTTP 200 및 source SHA를 반환한다. 통과. |
| 26 | 배포 확인을 위해 server restart나 Pages deployment가 필요한가 | server를 재시작하지 않았고 Pages도 배포하지 않았다. 통과. |
| 27 | desktop 캡처가 다른 viewport 또는 URL인가 | Agent Browser URL과 viewport는 Tailnet 1440×1000 CSS px다. 통과. |
| 28 | mobile 캡처가 active card를 보이지 않거나 page overflow가 있는가 | 390×844에서 `scrollLeft=272`로 active 칸을 보며 document/body 폭은 390이다. 통과. |
| 29 | 칸반 상태/공식 count가 로드맵 DOM과 맞는가 | `진행 중 1/2`, `C11.4 · ACTIVE`, 완료 78과 `78/214`를 화면에서 확인했다. 통과. |
| 30 | 브라우저 프로세스 종료, 화면 hash 또는 evidence link를 빠뜨렸는가 | desktop/mobile PNG hash와 lifecycle JSON을 보존했고 종료 후 일치 프로세스가 없다. 통과. |

30개 검토에서 문서·배포 evidence blocker를 찾지 못했다. README는 direct roadmap route 확인만 주장하며 전체 문서 site 배포로 확대하지 않는다.

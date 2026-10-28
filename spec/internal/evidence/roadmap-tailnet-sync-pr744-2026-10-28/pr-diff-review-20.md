# PR #744 Tailnet 배포 근거 PR diff 적대적 검토 · 20개 관점

검토 범위는 PR #744 병합 후 Tailnet에 동기화한 로드맵 파일의 원본·백업·HTTP·Agent Browser 근거다. 배포 명령을 다시 실행한 것으로 간주하지 않고 README/JSON/PNG가 같은 관측을 설명하는지 확인했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 증거가 다른 저장소 revision에서 나온다 | README와 JSON 모두 source commit `15c253c6…`을 고정한다. |
| 2 | 저장소 HTML 크기가 배포 결과와 다르다 | source·target·HTTP response는 모두 365,344 bytes다. |
| 3 | source와 served body의 내용이 다르다 | source, server file, HTTP body의 SHA-256은 모두 `e1ec9b89…`다. |
| 4 | 교체 전 파일 백업이 누락되거나 바뀐다 | pass01 backup 경로가 있고 교체 전 server SHA-256과 일치한다. |
| 5 | backup이 기존 자료를 덮어쓴다 | 새 PR #744 전용 경로만 사용했고, 교체 전에 경로가 없음을 확인했다. |
| 6 | atomic replace가 같은 파일시스템 밖에서 실행된다 | target과 임시 파일을 같은 directory에 두었다. |
| 7 | replace 전에 target이 열린 채 변경된다 | preflight에서 target file의 open handle이 0이었다. |
| 8 | 잘못된 파일 종류를 교체한다 | source와 target은 regular file이며 target은 symlink가 아니었다. |
| 9 | 파일 권한이 공개 서버 접근을 막는다 | 최종 mode는 `0644`다. |
| 10 | preview server가 재시작되거나 교체된다 | listener PID `28040` / port 8768이 유지됐다. |
| 11 | 다른 listener를 실수로 건드린다 | PID `33319` / port 4174도 그대로 살아 있었다. |
| 12 | Tailscale이 offline인데 cached localhost만 검사한다 | Tailscale backend `Running`, self `Online`이고 Tailnet HTTPS URL을 직접 요청했다. |
| 13 | Tailnet endpoint가 redirect/error를 숨긴다 | before/after 모두 Tailnet URL에서 HTTP 200을 받았다. |
| 14 | stale cache가 새 HTML 확인을 가린다 | 고유 query string을 사용했고 response는 `Cache-Control: no-cache`였다. |
| 15 | 응답 content type이 HTML이 아니다 | HTTP `Content-Type`은 `text/html; charset=utf-8`이다. |
| 16 | 새 PR #743 card가 page DOM에 없거나 숨겨졌다 | Agent Browser DOM에서 card를 찾고 `scrollY`/card bounds를 기록한 뒤 card가 보이는 screenshot을 저장했다. |
| 17 | 공식 완료 수가 source/서버에서 달라진다 | live card와 evidence 모두 C11.4 미완료 및 `72/213`을 표시한다. |
| 18 | PR 또는 evidence 링크가 틀리거나 dead다 | Agent Browser에서 PR #743과 source recovery README를 각각 열었다. |
| 19 | 이미지 근거가 잘렸거나 다른 카드 위치를 담는다 | 최종 screenshot을 육안 확인했으며 1280×633 PNG와 SHA-256을 기록했다. |
| 20 | 브라우저 프로세스나 runtime 검증을 종료/실행했다는 주장이 과장된다 | Agent Browser 세션 종료 뒤 관련 process가 없음을 확인했다. 앱/runtime은 재실행하지 않았다고 README에 명시했다. |

**검토 결과:** 정적 Tailnet preview가 PR #744 merge tree와 byte-exact하게 맞고, 백업·실행 상태·브라우저 관측을 서로 다른 증거 경계로 남긴다. 이 기록은 외부 독립 reviewer의 승인을 의미하지 않는다.

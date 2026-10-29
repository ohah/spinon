# C11.4 Tailnet sync pass02 문서 변경 실패 관점 30개

2026-10-29 PR #799 변경은 C11.4 계획 상태를 갱신하고 PR #798 이후 Tailnet 배포·Agent Browser 확인 근거를 추가한다. 검토는 계획의 동기화 상태, 외부 preview 경계, 브라우저·이미지 관찰, 공식 완료 상태 보존을 나눠 확인했다.

| # | 실패 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 계획이 동기화 전 단계 PR #797 상태를 가리키는가? | PR #798 merge SHA와 pass02 소스 revision을 기록한다. |
| 2 | PR #798이 실제로 병합되지 않았는데 병합으로 서술하는가? | GitHub의 Rebase and merge timestamp와 merge SHA를 기록한다. |
| 3 | 동기화 원본 branch가 PR merge commit과 다른가? | source HEAD를 95e77ceb44dce8029baa8c2b3533e38e61ab0f18로 고정한다. |
| 4 | bytes 또는 SHA가 배포 후 다른 파일을 가리키는가? | 원본·설치 파일 모두 393072 byte와 동일 SHA-256이다. |
| 5 | 이전 Tailnet 내용이 바뀌기 전에 보존되지 않았는가? | 이전 파일의 byte-exact backup hash를 기록한다. |
| 6 | 사전 상태가 이미 새 파일인데 교체를 수행했는가? | 교체 전 로컬·Tailnet 모두 PR #797 SHA를 반환했다. |
| 7 | Tailscale가 disconnected인데 성공으로 기록하는가? | 동기화 전 backend state가 Running이었다. |
| 8 | 미리보기 서버를 확인하지 않고 다른 프로세스를 덮어쓰는가? | 정식 preview listener와 기존 server command를 확인했다. |
| 9 | 열린 reader가 있는 상태에서 파일을 바꾸는가? | 사전 검사에서 대상의 외부 open handle은 없었다. |
| 10 | target이 symlink 또는 특수 파일인데 교체하는가? | 대상은 일반 파일이고 symlink가 아님을 확인했다. |
| 11 | preview root나 backup root가 예상 밖 symlink인가? | 두 directory의 실제 타입과 canonical path를 확인했다. |
| 12 | 기존 backup 디렉터리를 재사용해 덮어쓰는가? | pass02 backup root가 없음을 먼저 확인하고 신규 생성했다. |
| 13 | 다른 preview 파일도 함께 수정하는가? | roadmap.html 단일 경로만 교체했고 다른 route 변경은 없었다. |
| 14 | server restart가 배포 근거를 혼동시키는가? | PID는 전후 동일하고 server를 재시작하지 않았다. |
| 15 | partial write가 서비스에 보이는가? | 완성된 same-directory temporary file을 fsync 후 원자 rename했다. |
| 16 | 디렉터리 변경이 durable하지 않은가? | rename 뒤 parent directory를 fsync했다. |
| 17 | 파일 mode가 바뀌는가? | 교체 전 mode를 temporary file에 복사했다. |
| 18 | Tailnet이 stale cache를 반환하는데 성공으로 기록하는가? | Cache-Control no-cache 요청의 본문 SHA를 원본과 비교했다. |
| 19 | 로컬 route만 확인하고 Tailnet route를 추정하는가? | 두 route를 각각 HTTP GET해 status, byte 수, SHA를 비교했다. |
| 20 | 404 또는 다른 HTML 오류 응답을 정상 파일로 오인하는가? | 두 route status 200과 원본 byte/hash 일치를 요구했다. |
| 21 | PR #796 구현 SHA가 링크 수정 중 변형되는가? | 로드맵에 PR #796 merge SHA를 그대로 보존했다. |
| 22 | 49/49 scene 결과를 일반 CSS 지원으로 확대하는가? | 고정 fixture와 제한 opt-in 범위 및 미검증 한계를 유지한다. |
| 23 | pixel 표본 수가 달라지거나 합쳐져 보이는가? | 104/104를 유지하고 Android/iOS 경계를 설명한다. |
| 24 | 공식 완료 집계가 PR #798 때문에 증가하는가? | 계획과 로드맵은 72/213, C11.4 unchecked를 유지한다. |
| 25 | Agent Browser가 아닌 터미널 브라우저로 확인했다고 기록하는가? | Agent Browser 사용 경로를 명시한다. |
| 26 | 브라우저 page title 또는 C11.4 row를 확인하지 않았는가? | page title, merge SHA, scene/sample 수와 row text를 확인했다. |
| 27 | 여섯 개 수정 링크 중 하나가 아직 끊기는가? | 계획, 사전 비교, runtime, 두 캡처, status 목적지를 각각 열었다. |
| 28 | 이미지 링크가 HTML 문서나 깨진 thumbnail을 가리키는가? | 두 PNG 원본 화면과 natural dimensions를 확인한다. |
| 29 | screenshot artifact가 PR에 없는 로컬 임시 파일인가? | 화면을 같은 evidence directory에 저장하고 PR 변경에 포함한다. |
| 30 | 확인 후 사용자가 요청한 브라우저 process가 남는가? | Agent Browser session을 닫고 전용 process가 없는 것을 확인했다. |

**결과:** 계획의 Tailnet pending 문구를 동기화 완료로 바꾸고, 배포 hash·route 결과·Agent Browser 링크 검증과 화면을 연결한다. C11.4 및 공식 완료 상태는 승격하지 않는다.

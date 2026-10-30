# C11.4 PR #861 Tailnet 동기화 PR diff 적대 검토 30개

**검토 범위:** merged main `9450dca8754f7bf63ae271c86e40d5f815c35d2c`에서 만든 선택 publication, preflight·copy·backup·HTTP·Agent Browser receipts, 재현 스크립트와 이 README.

| # | 독립 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | 배포 원본이 병합 전 브랜치일 수 있음 | PASS — staging script가 `origin/main` 및 source HEAD를 `9450dca`로 고정한다. |
| 2 | 문서 source가 dirty working tree에서 섞임 | PASS — root script가 각 source를 `git show <head>:<path>`로 읽는다. |
| 3 | 오래된 source 목록이 들어감 | PASS — 0059, STATUS, public roadmap, C11.4 계획과 nowrap 증거 경로만 선택한다. |
| 4 | 실수로 sync evidence 전체를 재귀 복사함 | PASS — sync review 문서만 exact source로 허용하고 나머지 운영 receipt는 publication에 넣지 않는다. |
| 5 | symbolic link를 따라가 외부 파일을 복사함 | PASS — source·generated tree walker가 symlink를 거부한다. |
| 6 | 다른 RSPress 버전으로 재현됨 | PASS — package lock/source의 RSPress 2.0.22를 유지하고 로그가 해당 버전을 출력한다. |
| 7 | 선택 build를 전체 문서 빌드로 과장함 | PASS — README와 manifest가 partial publication 및 disabled dead-link/image checks를 기록한다. |
| 8 | 선택된 page route가 빠짐 | PASS — publication script가 roadmap, STATUS, C11.4 plan, contract, evidence와 review route를 assert한다. |
| 9 | Tailnet roadmap이 PR #861 이전 내용임 | PASS — raw roadmap marker에 PR #861 Rebase and merge가 있다. |
| 10 | PR #861 merge SHA가 틀림 | PASS — source card와 rendered page에서 `9b7c3a357d94db0e47f4397b73d3376a35de4439`를 확인한다. |
| 11 | 구현 slice가 C11.4 완료로 표시됨 | PASS — roadmap 및 STATUS에서 C11.4 미완료다. |
| 12 | C11 상위가 완료로 표시됨 | PASS — roadmap card와 STATUS의 C11 master checkbox가 미완료다. |
| 13 | 72/213을 C11 전용 수치로 해석함 | PASS — 전체 공식 완료 집계로 문구화했다. |
| 14 | nowrap 장면의 normal 대조군을 누락함 | PASS — 20×20 root, 20px track, 두 줄과 nowrap 기대값을 card·runtime 페이지로 확인한다. |
| 15 | nowrap geometry의 root/item을 혼동함 | PASS — 20×10 root, 50px track, 50×10 item 및 30px overflow를 분리한다. |
| 16 | surface draw/present를 glyph pixel 검증으로 확대함 | PASS — 문구와 runtime 근거가 glyph pixel 동일성을 주장하지 않는다. |
| 17 | Android simulator를 hardware GPU/실기기로 확대함 | PASS — Android API 37 ARM64 AVD 범위로 표시한다. |
| 18 | iOS drawable을 compositor 이후 검증으로 확대함 | PASS — iPhone 17 Pro/iOS 26.2 Simulator 범위를 표시한다. |
| 19 | preflight 전에 Tailnet을 덮어씀 | PASS — preflight가 162 target의 이전·새 hash를 먼저 기록한다. |
| 20 | 기존 Tailnet 파일을 잃음 | PASS — 교체 15개, 5,776,631 bytes를 task-specific backup root에 보존한다. |
| 21 | 새 publication route 외의 파일을 삭제함 | PASS — copy/verify receipt의 `no_deletes`가 true다. |
| 22 | 복사 도중 source가 바뀜 | PASS — copy가 preflight source byte 수·SHA-256 재검사를 요구한다. |
| 23 | 복사 전 Tailnet 대상이 바뀜 | PASS — copy가 target 존재·이전 byte 수·SHA-256을 preflight와 대조한다. |
| 24 | target 파일을 부분 작성한 상태로 공개함 | PASS — 임시 파일 fsync 뒤 atomic rename을 사용한다. |
| 25 | 다른 process의 열린 파일을 덮어씀 | PASS — open handle 소유자를 확인하며 이 실행은 비-preview open handle이 0개였다. |
| 26 | preview server를 재시작하거나 잘못된 서버에 복사함 | PASS — PID 28040과 `tailnet-preview-server.mjs`를 확인했고 서버를 유지했다. |
| 27 | local copy가 source와 다름 | PASS — local verification은 target 162/162 및 backup 15/15 hash가 같다. |
| 28 | HTTPS route가 stale cache/다른 파일을 반환함 | PASS — 162/162 HTTP 200이 source byte 수와 SHA-256에 일치한다. |
| 29 | Browser 화면이 다른 카드/상태를 보여줌 | PASS — Agent Browser에서 첫 카드의 PR #861, merge SHA, `72/213`, 미완료 문구와 STATUS checkbox를 읽었다. |
| 30 | screenshot link가 깨지거나 browser가 남음 | PASS — Android 1080×2400, iOS 1206×2622 이미지를 열었다. Browser를 닫고 daemon이 없음을 확인했으며 terminal-browser는 사용하지 않았다. |

검토는 Tailnet 선택 publication과 증거 경계를 대상으로 한다. 전체 사이트, 전체 WPT, 일반 CSS 지원, 실기기 또는 GitHub Pages 배포를 증명하지 않는다.

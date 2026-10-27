# Tailnet 로드맵 동기화 PR 변경 검토 30개

**검토 대상:** 선택 Rspress build와 route manifest, Tailnet preflight·copy·HTTPS 검증 스크립트 및 로그, Agent Browser 화면·관측, 동기화 README

**기준:** `docs/project-rules.md`, PR #718 병합 뒤 `main` HEAD `ad0106b6b1a6160bb601b4a407c377c8046e9415`, PR #717/#718의 GitHub 병합 메타데이터, Tailnet 제공 응답

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 동기화 원본이 PR #718 병합 전일 수 있음 | source HEAD는 병합 main `ad0106b6b1a6160bb601b4a407c377c8046e9415`다. |
| 2 | `origin/main`과 로컬 source가 달라질 수 있음 | root staging에서 HEAD와 `origin/main` 일치를 요구했다. |
| 3 | tree가 같은 source head에 속하지 않을 수 있음 | selected root/publication/preflight/copy/verify의 HEAD와 tree가 같다. |
| 4 | C11.4 상태를 잘못된 PR과 묶을 수 있음 | #717 구현 merge SHA `c29482aa8fc3ac39cdc342eae138c3ad10a59683`를 사용했다. |
| 5 | 문서 동기화 PR의 merge SHA를 혼동할 수 있음 | README는 #718의 `ad0106b6b1a6160bb601b4a407c377c8046e9415`를 기록한다. |
| 6 | 빈 check rollup을 CI 성공으로 볼 수 있음 | #717/#718 checks는 미보고로 기록했다. |
| 7 | test-only slice를 완료 기능으로 표시할 수 있음 | C11.4 checkbox를 미완료로 유지했다. |
| 8 | 공식 집계가 바뀔 수 있음 | 상태 자료와 제공 페이지에서 `72/213`을 확인했다. |
| 9 | 전체 문서 build를 주장할 수 있음 | 선택 build라고 한정하고 full-doc validation을 `false`로 기록했다. |
| 10 | Rspress 버전이 프로젝트 고정 버전과 달라질 수 있음 | build log는 고정 버전 `2.0.22`를 보고한다. |
| 11 | source selection이 tracked HEAD 밖의 파일을 포함할 수 있음 | prepare script는 `git ls-tree`와 `git show HEAD:path`로 source를 고정했다. |
| 12 | 새 roadmap note의 링크를 검사하지 않을 수 있음 | 새 #717 paragraph 안의 Tailnet 로컬 링크 6개가 publication에 있는지 검사했다. |
| 13 | 한 줄에 붙은 이웃 roadmap 카드 링크를 새 note로 오인할 수 있음 | 초기 selector 오류를 발견해 `<p>` 경계로 #717 paragraph만 분리했다. |
| 14 | Markdown link text가 HTML에서 평문으로 남는다고 가정할 수 있음 | STATUS/계획 검사는 문구 대신 PR href, merge SHA, count를 확인한다. |
| 15 | plan/status html이 stale일 수 있음 | 생성한 집중 계획·상위 계획·STATUS 페이지를 Tailnet에서 다시 읽었다. |
| 16 | raw Markdown route만 올리고 generated HTML을 빼먹을 수 있음 | raw source aliases와 Rspress HTML routes를 함께 publication에 포함했다. |
| 17 | 새 리뷰 링크가 빠질 수 있음 | post-merge review HTML route가 publication에 포함됐다. |
| 18 | 사전 비교 이미지 route가 끊길 수 있음 | PNG route와 signature가 publication에 포함됐다. |
| 19 | route path traversal이나 symlink가 섞일 수 있음 | route component와 staged/build/target symlink를 검사했다. |
| 20 | 기존 선택 root·publication을 덮어쓸 수 있음 | task-specific build 경로가 비어 있음을 먼저 확인했고, 이후 publication 재실행은 그 경로의 일반 디렉터리 안에서만 같은 source를 재생성했다. |
| 21 | 동기화 root가 사용자 서버의 실제 serving directory와 다를 수 있음 | `tailnet-preview-server.mjs`가 `/Users/yoonhb/Library/Application Support/SpinonPreview`를 serving root로 사용함을 확인했다. |
| 22 | 서버가 이미 내려가 있을 수 있음 | preflight와 postflight에서 Tailscale backend `Running`, Mac Studio `online`을 확인했다. |
| 23 | 이전 로드맵 응답이 이미 달라졌을 수 있음 | HTTP 200 응답의 크기와 hash를 사전 기록하고 local target과 대조했다. |
| 24 | 교체 중인 target을 서버가 열고 있을 수 있음 | 12개 교체 대상 모두 preflight와 쓰기 직전 open-handle 검사가 통과했다. |
| 25 | 기존 파일을 잃을 수 있음 | 교체 전 12개, `4,115,888` bytes를 별도 backup root에 byte-exact 보존했다. |
| 26 | 동기화가 대상 외 경로를 삭제할 수 있음 | copy report는 create/replace/same만 기록하고 route deletion은 하지 않았다. |
| 27 | temp-file write가 부분 파일을 노출할 수 있음 | 같은 directory의 temp file을 fsync 후 rename하는 절차를 사용했다. |
| 28 | 로컬 파일만 갱신되고 Tailnet route는 stale일 수 있음 | 96/96 HTTPS response의 status, byte length, SHA-256을 publication manifest와 대조했다. |
| 29 | browser evidence가 Terminal Browser이거나 다른 화면일 수 있음 | Agent Browser DOM으로 #717 note·링크·계획·STATUS를 확인했다. pass01의 잘못된 scroll 화면은 보존하고 pass02만 최종 화면 근거로 썼다. |
| 30 | 검증 뒤 Agent Browser가 살아 있을 수 있음 | `close` 성공, 빈 session list, 존재하지 않는 보고 PID와 session Chrome process 없음으로 종료 상태를 확인했다. |

**판정:** 초기 검토에서 roadmap paragraph selector, Markdown 링크 확인 조건, Agent Browser smooth-scroll 화면 위치 문제를 발견해 각각 보정했다. 최종 publication/served hashes와 표시 화면을 다시 확인했으며, 선택 정적 문서 범위 밖의 제품 runtime·전체 문서 적합성은 주장하지 않는다.

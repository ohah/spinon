# PR #1145 Tailnet 동기화 근거 문서 diff 적대 검토 30개

검토 대상은 이 evidence 디렉터리의 README, selected-root/publication 처리 스크립트, Tailnet preflight/copy/verify 스크립트, manifest·보고서와 Agent Browser 캡처다. 원본은 PR #1144 병합 뒤의 `origin/main`이다. 이 PR은 코드 runtime이나 공식 상태를 바꾸지 않는다.

| # | 적대 관점 | 확인 근거와 판정 |
| --- | --- | --- |
| 1 | 동기화 source SHA가 PR #1144 병합 전 브랜치 SHA인가 | source SHA는 main의 #1144 merge SHA `a2e90a84fabcb73f8836eeb412df1a9ac86755fc`다 — 통과. |
| 2 | 선택 root의 source tree가 다른 main 상태인가 | manifest tree `809e0510b9f142facbb372fecc53d7396597a7c6`와 merge commit tree가 일치한다 — 통과. |
| 3 | 추적 중인 변경이 남은 상태를 source로 사용했는가 | tracked diff SHA는 빈 diff 값을 기록한다 — 통과. |
| 4 | 문서 생성기 pin이 바뀌었는가 | 빌드 출력은 저장소 규칙의 RSPress `2.0.22`를 사용한다 — 통과. |
| 5 | 선택 config가 저장소 기본 config나 공개 사이트 배포 설정을 덮는가 | 전용 config를 새 evidence 경로에 두고 output은 고유 `build/spinon/...` 경로로 보낸다 — 통과. |
| 6 | 선택 source에 untracked 임시물이 섞이는가 | tracked tree 복사는 `git ls-files`만 사용하고 새 PR evidence 파일은 복사하지 않는다 — 통과. |
| 7 | source symlink가 publication 경계를 벗어나는가 | 각 경로 요소의 symlink를 거부하고 regular file만 선택한다 — 통과. |
| 8 | 악성 route나 `..` 경로가 root 밖으로 쓰이는가 | source/publication route 검사에서 절대 경로, 역슬래시, 빈·dot·dotdot segment를 거부한다 — 통과. |
| 9 | 이전 build 또는 backup 증거를 덮어쓰는가 | 고유 `pr1144-2026-11-04/pass01` 경로와 `wx` 생성으로 기존 산출물을 보존한다 — 통과. |
| 10 | script가 credential이나 비공개 token을 기록하는가 | Tailnet URL 외 인증 비밀을 포함하지 않으며 process args에도 credential을 넣지 않는다 — 통과. |
| 11 | 선택된 roadmap가 다른 PR 카드를 기준으로 삼는가 | 단일 `#1142 Rebase and merge` marker와 merge SHA `0f5506423a1150c80e5684fd087b948923fdbcb7`을 확인한다 — 통과. |
| 12 | C11 제한 구현을 전체 구현 완료로 표시하는가 | 카드, 상태와 보고서가 C11 미완료를 유지한다 — 통과. |
| 13 | 공식 완료 수가 올라갔는가 | 선택 root·publication 모두 `72/213`을 고정 확인한다 — 통과. |
| 14 | 카드 local route를 일부만 검사하는가 | manifest가 카드의 local route 10개 전부와 route hash를 기록한다 — 통과. |
| 15 | 필수 plan/contract/evidence/screenshot route가 빠지는가 | stage script가 14개 핵심 route와 C11 fragment를 확인한다 — 통과. |
| 16 | 상태 fragment anchor가 빌드 과정에서 사라지는가 | 생성된 `STATUS.html`의 `id="c11-css-grid"`와 72/213을 검사한다 — 통과. |
| 17 | selected source hash와 publication hash가 다르게 기록되는가 | 각 staged 파일과 generated route에서 SHA-256을 계산하고 manifest에 보존한다 — 통과. |
| 18 | 전체 docs link graph 검증을 했다고 확대 주장하는가 | report와 README는 selected route만 검증했다고 명시하고 full docs link validation은 false다 — 통과. |
| 19 | preview PID가 예상 서버가 아닌가 | Tailscale 상태, 8768 listener, process args를 검사했고 PID는 28040이다 — 통과. |
| 20 | sync 직전 local server가 응답하지 않는가 | local roadmap fetch는 HTTP 200이며 server PID를 보고서에 남긴다 — 통과. |
| 21 | open-handle 예외가 임의 프로세스나 쓰기 핸들을 허용하는가 | 허용 조건은 동일 preview PID, `node`, regular file, read-only `r`, 동일 inode와 경로 모두다 — 통과. |
| 22 | 열린 read handle 중 다른 사용자의 inode를 덮어쓰는가 | 원본 inode는 유지하고 임시 파일 rename만 사용해 기존 reader를 보존한다 — 통과. |
| 23 | 기존 target 내용이 백업 없이 사라지는가 | 교체 대상 26개를 바꾸기 전에 전용 경로에 복사하고 이전 SHA-256을 확인한다 — 통과. |
| 24 | create/replace/same 분류 합계가 publication 수와 다르는가 | 119+26+28은 173이며 copy 결과와 일치한다 — 통과. |
| 25 | 선택 publication 파일을 지우는 경로가 있는가 | sync는 copy/rename만 하며 report의 `noDeletes=true`다 — 통과. |
| 26 | 복사 후 local serving root가 source hash와 다른가 | 173개 local route의 size와 SHA-256을 대조했다 — 통과. |
| 27 | public HTTPS가 stale 또는 404 route를 제공하는가 | 173개 HTTPS route 모두 HTTP 200 및 manifest SHA-256 일치다 — 통과. |
| 28 | desktop 확인에서 카드 또는 수평 overflow를 놓치는가 | Agent Browser 1280×1000에서 카드 1개, local link 10개, 폭 1280/1280을 확인했다 — 통과. |
| 29 | mobile 확인이 desktop viewport를 재사용하거나 잘림을 숨기는가 | 390×844 mobile 화면과 390/390 문서 폭을 기록했다. 인접 overview SVG의 우측 잘림도 README와 캡처에 명시한다 — 통과. |
| 30 | C11 딥링크나 검토 page route가 publication 뒤에 깨지는가 | Agent Browser에서 C11 fragment 이동과 30행 PR #1144 검토 페이지를 확인했다. 종료 후 Agent Browser를 닫고 PID 28040은 유지했다 — 통과. |

## 판정

선택 발행, 백업, 원자 교체, local/HTTPS hash 확인을 지정 범위에서 수행했다. 전체 문서 링크 검증, C11 완료, Android 실기기, 앱 runtime test 또는 GitHub Pages 배포를 주장하지 않는다. 모바일 overview SVG의 우측 잘림은 별도 시각 표시 한계로 남아 있다.

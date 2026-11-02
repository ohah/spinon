# Tailnet 로드맵 동기화 PR 변경 실패 관점 검토 30개

검토 대상은 이 폴더의 동기화 도구, 범위 제한 publication, 실행 기록과 화면 자료다. 제품 코드나 C11 구현 완료 상태를 검토·변경하지 않는다. 각 항목은 서로 다른 경계 또는 실패 경로다.

| # | 실패 관점 | 결과와 근거 |
|---:|---|---|
| 1 | 다른 checkout/commit의 문서를 게시할 위험 | `selected-root-pass05.json`의 HEAD와 tree가 각각 `00230cda…` / `6d0e2bf0…`이며 main의 #1039 merge 이후 소스와 일치한다. |
| 2 | 전체 `72/213`을 C11 진행률로 오해할 위험 | README와 browser record에서 전체 집계로만 표기한다. C11 비율로 환산하지 않는다. |
| 3 | integer `repeat()` subset을 전체 C11 완료로 오해할 위험 | roadmap 카드와 공식 STATUS 모두 C11 미완료를 유지하고, publication 검증에서 `C11.3`·`C11.4`를 확인한다. |
| 4 | build 실패를 숨기고 전체 docs build 성공이라 보고할 위험 | README/빌드 결과가 selected-root build 성공만 기록하고 repository-wide build를 성공으로 부르지 않는다. |
| 5 | 선택 build가 전역 dead-link 검사를 대신하는 위험 | config는 selected-root에서만 검사를 끈다. README는 PR #1038의 새 local 링크 여덟 개만 별도 검증했다고 범위를 한정한다. |
| 6 | untracked/임시 evidence가 publication 입력에 섞일 위험 | `prepare-selected-root.mjs`는 evidence tree를 `git ls-files`에서 가져오고 script 확장자는 제외한다. |
| 7 | source symlink가 미리보기 root 밖으로 빠질 위험 | source 경로 각 segment의 `lstat`에서 symlink를 거부한다. |
| 8 | 기존 build snapshot을 덮어쓸 위험 | 선택 root 생성은 pass05 경로와 manifest가 없을 때만 진행한다. 이전 pass 산출물은 그대로 남는다. |
| 9 | 문서 root가 예상 밖 경로를 읽을 위험 | RSPress config의 root/outDir는 이 작업의 pass05 디렉터리로 고정한다. |
| 10 | favicon/logo 누락으로 문서가 깨질 위험 | public icon/logo/roadmap SVG를 선택 root에 포함하고 성공한 RSPress 출력에 icon.svg를 생성했다. |
| 11 | 문서 도구 버전을 바꾸거나 최신 버전에 기대는 위험 | 빌드 결과는 저장소 설치 RSPress `2.0.22`를 기록하며 package/lockfile 변경은 없다. |
| 12 | 처리되지 않은 `public/roadmap.html` 대신 generated route를 게시할 위험 | publication은 main의 raw roadmap을 `roadmap.html` 및 기존 두 alias로 복사한다. |
| 13 | 현재 PR 카드의 merge SHA가 빠지거나 stale해질 위험 | staging은 카드 안의 `b3ad473d4bfef4bb2f54a61603d498e620c03b0a`를 검사하고 Agent Browser에서도 읽었다. |
| 14 | 카드가 가리키는 local HTML/Markdown/PNG 중 하나가 누락될 위험 | 카드의 내부 link 여덟 개가 모두 publication manifest에 있어야 staging이 끝난다. |
| 15 | GitHub PR 링크를 Tailnet local route로 잘못 해석할 위험 | staging 검사는 http/https/mailto를 제외하고 `/spinon/` local link만 route로 검사한다. |
| 16 | URL encoded traversal이 route 검사에서 통과할 위험 | 카드 URL을 decode한 뒤 route segment의 빈 값, `.`, `..` 및 역슬래시를 거부한다. |
| 17 | 같은 route를 다른 바이트로 두 번 stage할 위험 | 중복 route는 SHA-256이 같을 때만 수용한다. |
| 18 | 일부 publication 파일만 누락된 채 copy 단계로 넘어갈 위험 | publication tree 전체와 manifest route 수·각 파일 크기·SHA를 preflight에서 대조한다. |
| 19 | publication에서 이름이 manifest에 없거나 그 반대인 위험 | 서로 다른 전체 route 목록을 비교하고 하나라도 다르면 preflight를 중단한다. |
| 20 | 같은 이름의 backup을 덮어써 이전 Tailnet 자료를 잃을 위험 | preflight와 copy 모두 고유 pass01 backup 경로가 존재하면 중단한다. |
| 21 | 다른 프로세스가 `8768`을 사용하거나 잘못된 server를 건드릴 위험 | listener는 하나이며 PID 28040의 command가 `tailnet-preview-server.mjs`임을 확인했다. |
| 22 | Tailscale이 꺼졌는데 로컬 결과만 보고 게시할 위험 | preflight에서 Tailscale `Running`과 self `Online=true`를 요구한다. |
| 23 | preview root alias/symlink에 잘못 게시할 위험 | preview root의 `lstat`와 `realpath`를 검사해 실제 예상 경로만 허용한다. |
| 24 | 교체 대상이 현재 열려 있어 reader가 부분 파일을 볼 위험 | 교체 15개 각각의 `lsof` 결과가 0인 경우에만 copy를 허용한다. |
| 25 | 이전 파일을 백업하기 전에 새 내용으로 교체할 위험 | 모든 replacement 파일을 전용 backup에 복사하고 이전 SHA를 검증한 뒤 route 교체를 시작한다. |
| 26 | copy 전에 preflight 이후 바뀐 대상 파일을 덮을 위험 | copy 직전 기존 존재 여부와 이전 SHA를 다시 비교한다. |
| 27 | 교체 중 부분 파일을 노출할 위험 | 임시 파일을 destination과 같은 디렉터리에 쓰고 chmod 후 `renameSync`로 교체한다. |
| 28 | 신규 route 부모가 symlink이거나 일반 파일인 위험 | 부모 디렉터리는 segment별 `lstat`로 실제 directory임을 확인하며, 없는 폴더만 만든다. |
| 29 | 서버 재시작 또는 무관 route 삭제가 부수효과로 생길 위험 | 동기화는 선택 파일 생성/교체만 수행했고 `noDeletes=true`, `previewServerRestarted=false`다. |
| 30 | 게시 후 서버가 이전/stale 바이트를 돌려주는 위험 | local preview와 HTTPS에서 124/124 HTTP 200 및 SHA 일치, replacement backup 15/15 이전 SHA, Agent Browser 페이지·Android/iOS 이미지 로드를 확인했다. |

남은 검증 한계: 이 PR은 Tailnet 미리보기의 선택 route 갱신 근거다. 전체 RSPress dead-link pass, GitHub Pages, 제품 release, C11 전체 지원, 실기기 GPU 검증을 주장하지 않는다.

# C11.4 cyclic gap Tailnet 동기화 실패 관점 검토 20개

**대상:** PR #359 뒤 선택 문서 빌드, Tailnet 복사, 실제 응답, Agent Browser 표시. 각 관점은 서로 다른 경계나 실패 원인을 확인한다.

| # | 적대 질문 | 확인 방법과 결과 |
| ---: | --- | --- |
| 1 | 이전 PR이나 stale branch를 기준으로 갱신하지 않았나? | 기준은 병합 SHA `7a364befe44ba56903be7c24df036ce89d252156`다. pass10 preflight는 `docs/c11-4-cyclic-grid-gap-tailnet-20261020`와 같은 HEAD를 기록했다. |
| 2 | roadmap만 갱신하고 상태 대장·집중 계획·근거를 놓치지 않았나? | 282개 선택 파일에 `roadmap.html`, `STATUS.html`, C11.4 계획, Chrome 근거 및 기존 fixed-gutter 근거가 포함됐다. Agent Browser에서 네 핵심 route를 열었다. |
| 3 | 한정된 cyclic gap fixture를 C11.4 전체 완료로 표시하지 않았나? | Tailnet STATUS의 C11.4 checkbox는 `false`였고, 변경 diff는 C11.4 아래 설명만 추가했다. |
| 4 | 확정되지 않은 WPT·실기기·전체 CSS Grid 결과를 완료 사실처럼 쓰지 않았나? | roadmap과 근거 페이지는 WPT·실기기·전체 Grid 및 iOS compositor-after 결과를 미완료로 표기한다. |
| 5 | STATUS에서 계획 링크가 선택 Tailnet route로 연결되나? | Agent Browser에서 `/spinon/plan/c11-4-grid-track-sizing.html` 및 새 근거 route 링크를 확인했다. |
| 6 | 선택되지 않은 저장소 문서를 존재하지 않는 Tailnet 경로에 연결하지 않나? | 36개 HTML route의 same-origin out-of-base 링크 수는 0이다. 비선택 상대 링크는 GitHub `main`으로 변환됐다. |
| 7 | 선택된 Markdown의 이미지가 Tailnet에서 실제 제공되나? | pass10에서 Chrome·Android·iOS 이미지 3종을 확인했다. pass13은 보고서의 네 Browser 캡처가 selection에서 빠져 GitHub main으로 가는 결함을 찾았고, pass14에 네 PNG를 명시적으로 넣어 4/4 load 및 hash를 확인한다. |
| 8 | 긴 경로 prefix가 있을 때 잘못된 짧은 mapping을 고르지 않나? | adapter가 경로 prefix를 길이 내림차순으로 비교한다. 실제 계획 페이지의 모든 새 C11.4 링크가 `/spinon/internal/evidence/...` route로 연결됐다. |
| 9 | 상대 URL의 query·fragment와 인코딩을 잃지 않나? | 구현은 suffix를 분리해 보존하고 path segment를 encode한다. pass10 HTML에는 same-origin base 이탈이 없었다. 별도 query-case unit test는 만들지 않았다. |
| 10 | root 밖으로 탈출하는 `../` 링크가 Tailnet 내부에 남지 않나? | adapter가 root 밖 정규화 경로를 거부한다. 생성된 36개 HTML에서 same-origin out-of-base 링크는 0개다. |
| 11 | Markdown source나 staging root를 복사하지 않나? | helper는 sanitized site output `build/spinon/c11-4-cyclic-grid-gap-tailnet-site-pass10`만 허용했고, preflight도 이 상대 경로를 기록했다. |
| 12 | 작업이 Rspress 버전이나 lockfile을 바꾸지 않았나? | `@rspress/core` 2.0.22를 사용했다. `bun.lock`과 package manifests diff는 없다. |
| 13 | 복사 대상이 사용자 미리보기 루트 밖으로 바뀌지 않나? | helper가 target을 기존 `SpinonPreview` 경로로 고정했다. 잘못된 target/backup 경로는 거부하도록 코드에서 검사한다. |
| 14 | backup 경로를 기존 실행과 재사용하지 않나? | pass10·pass11·pass12는 각각 이 작업 전용 `pass10`, `pass11`, `pass12` backup 하위 경로를 쓴다. 앞선 실행의 backup과 겹치지 않는다. |
| 15 | symlink가 사용자 파일 경계를 우회하지 않나? | build source·target·backup 구성요소와 source/destination entry의 symlink를 검사해 거부한다. pass10 입력에서 symlink 오류는 없었다. |
| 16 | preflight 후 source/target 변경을 탐지하나? | copy helper가 file set, source bytes/hash, target 존재·bytes/hash를 재검증했다. pass10의 282개 copy 전부 검증됐다. |
| 17 | 선택 범위 밖의 파일을 삭제하거나 교체하지 않나? | `no_deletes=true`; 46 create, 31 replace, 205 same만 적용했다. |
| 18 | 이전 파일 교체 전 복구 사본을 검증하나? | 교체 31건 모두 backup bytes와 SHA-256이 preflight 기존 값과 일치했다. |
| 19 | 열린 파일 때문에 preview server나 tmux를 종료하지 않나? | 교체 경로 31개 모두 `lsof` 열린 handle 0개였다. PID 1191은 계속 8768에서 listen했고, tmux 명령은 실행하지 않았다. |
| 20 | 복사 성공만으로 제공 완료를 주장하거나 browser를 남기지 않나? | pass10·pass11·pass12의 선택 URL 전체를 HTTP·byte·SHA로 검증한다. 각 Agent Browser 실행 뒤 전용 session은 active=false/pid=null이고 전용 Chrome이 사라졌음을 종료 JSON에 기록한다. 기존 PID 74457은 보존한다. |

## 후속 게시

pass11은 296개 선택 파일을 게시해 전체 HTTP·byte·SHA가 일치했다. Agent Browser에서 발견한 공백 경로 표시 문제는 문서 문구를 고쳐 pass12에 반영했고, pass13은 문서 보고서의 네 캡처 누락을 추가로 찾아냈다. pass14 선택 목록에 그 네 PNG를 넣어 같은 no-delete, backup, atomic rename, hash 검증 절차로 복구한다.

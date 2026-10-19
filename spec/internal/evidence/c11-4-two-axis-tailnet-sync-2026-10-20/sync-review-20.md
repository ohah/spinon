# C11.4 PR #368 Tailnet 미리보기 동기화 실패 관점 검토 22개

**범위:** 선택 root 생성, 기존 preview 파일 보호, 실제 HTTP 제공, Agent Browser 화면과 종료를 각각 검토했다.

| # | 실패 질문 | 확인 결과 |
| ---: | --- | --- |
| 1 | 원래 checkout의 staged/unstaged 작업이 새 sync branch에 섞이나? | 새 branch는 merge된 `origin/main` `c8aa0939`에서 만들었다. 원래 checkout은 수정하지 않았다. |
| 2 | Tailnet server document root를 잘못 선택했나? | 실행 중 server script의 root는 `~/Library/Application Support/SpinonPreview`, listen 주소는 `127.0.0.1:8768`, route prefix는 `/spinon/`이다. Funnel 443 proxy도 이 주소를 가리킨다. |
| 3 | preview 갱신 중 기존 Tailnet server를 중지하나? | PID 1191을 계속 두었다. 동기화 중·후에도 프로세스가 실행 중이었다. |
| 4 | 전체 저장소 문서를 무심코 공개하나? | 선택 root는 88개 명시 source 항목에서 1,790개 staging 파일과 71 HTML route를 만들었다. Tailnet에는 preflight에 열거된 1,522개 파일만 다뤘고 삭제는 없다. pass별 create/replace/same 수치는 각 `preflight-pass*.json`에 남겼다. |
| 5 | source에 private host/worktree 경로가 남나? | selected-root staging에서 199개 파일을 정리했고 생성 산출물에서 1개를 추가 정리했다. 검사된 결과에는 host path가 남지 않았다. |
| 6 | 새 roadmap·STATUS·plan·evidence route가 base 밖을 가리키나? | Agent Browser로 확인한 페이지의 same-origin out-of-base 링크는 0개이며, 새 C11.4 관련 Tailnet route 8/8이 HTTP 200이다. |
| 7 | 문서 링크 대상이 선택 build에서 빠졌나? | 추가된 계획, Chrome 사전 비교, 실행 근거, 구현 검토, 상태 대장 및 내부 index의 대상 route를 확인했다. |
| 8 | broader link scan이 모두 해결됐다고 과장하나? | 69개 HTML broader scan은 선택하지 않은 이전 문서 대상 10개가 현재 preview에 없음을 기록한다. 새 C11.4 route와 구분했고 이 sync는 이를 삭제하거나 수정하지 않았다. |
| 9 | 초기에 실패한 링크 검사 수를 숨기나? | build output만 검사했던 초안은 기존 preview target을 포함하지 않아 305개를 누락으로 잘못 셌다. 초기 결과를 보존하고 실제 served root를 기준으로 다시 검사했다. |
| 10 | Chrome oracle 관찰을 Spinon 표준 적합성으로 부르나? | 문서에는 Chrome 155 사전 비교라고 적고 전체 WPT/CSS Grid parity를 주장하지 않는다. |
| 11 | Simulator 캡처를 실기기나 compositor-after 결과로 넓히나? | Agent Browser에서 Android 1080×2400, iOS 1206×2622 캡처가 로드됐다. 원 근거는 AVD와 iOS Simulator이며 실기기·hardware GPU·compositor-after 결과는 미완료로 둔다. |
| 12 | PR #368 merge SHA 또는 merge method를 잘못 썼나? | PR #368은 Rebase and merge됐으며 SHA `c8aa093981a1b0d435347d9757b1fd0146a1f7db`로 plan, STATUS, roadmap을 대조했다. |
| 13 | required check 미설정을 CI 통과로 표현하나? | GitHub에 required status check가 설정되어 있지 않아 CI 통과라고 쓰지 않았다. |
| 14 | C11.4 완료나 공개 완료 집계를 올리나? | roadmap UI는 `72/212`, C11.4 checkbox는 unchecked다. 공식 C11.4 체크와 완료 집계를 수정하지 않았다. |
| 15 | Rspress나 lockfile을 업데이트하나? | 고정된 로컬 `@rspress/core 2.0.22` 실행 파일로 빌드했고 package/lockfile 변경은 없다. 도움말용 `bunx`가 별도 1.x를 받았지만 빌드에 쓰지 않았다. |
| 16 | arbitrary source/target path 또는 symlink를 따라 쓰나? | helper는 sanitized pass root, 고정 `SpinonPreview` target/operation backup path, source/target/backup 구성 요소 symlink를 확인한다. |
| 17 | 낡은 preflight 뒤 source나 target이 바뀌어도 계속 쓰나? | copy 전에 source file set/hash 및 기존 target 존재·bytes/hash를 preflight와 다시 대조한다. |
| 18 | 기존 target을 backup 없이 직접 덮거나 열린 파일을 바꾸나? | pass12 교체 52개는 열린 파일 0개와 backup 52/52를 확인했다. pass13 두 차례에서 PID 1191이 연 HTML 두 개와 추가 세 개는 현재 bytes를 staging에 보존했다. 최종 copy 직전 열린 교체 대상은 0개였다. pass13 final preflight의 backup count와 SHA 확인은 `copy-and-backup-pass13.json`에 기록했다. |
| 19 | 일부 local copy만으로 live Tailnet 성공을 주장하나? | pass12 1,458/1,458 및 최종 pass13 1,522/1,522 파일 모두 HTTP 200, byte 수와 SHA-256이 일치했다. 삭제는 없다. |
| 20 | 화면 확인 뒤 browser/process나 tmux를 남기거나 끄나? | 최종 Tailnet 문서 확인 뒤 Agent Browser session을 닫고 session inactive 및 process 0개를 확인했다. Tailnet PID 1191과 13개 tmux session은 유지했다. |
| 21 | pass13 입력 manifest가 없는 경로를 참조하나? | 첫 생성 시 누락 경로를 확인했다. pass12 manifest를 보존하고 pass13별 manifest를 생성하도록 helper를 고쳤으며 최초 오류와 조치를 JSON으로 남겼다. |
| 22 | 열린 파일을 강제로 닫거나 서버를 중단해야 하나? | PID 1191이 두 조사 시점에서 열어 둔 HTML 2개와 추가 3개의 기존 bytes/SHA를 staging에 보존했다. 대상 파일은 쓰지 않았고 나머지만 교체하도록 preflight를 다시 만들었다([초기 handle](./open-handles-initial-pass13.json), [handle 재검사](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-two-axis-tailnet-sync-2026-10-20/open-handles-pass13.json), [보존 hash](./preserved-open-targets-pass13.json)). |

## 발견과 조치

- 첫 route checker가 Rspress build만 검사해, 선택하지 않은 기존 preview paths를 없는 파일로 계산했다. 그 결과를 별도 JSON으로 보존하고 실제 `SpinonPreview` directory를 기준으로 확인 로직을 고쳤다.
- broader route audit에서 확인된 10개 대상은 새 C11.4 links가 아니며 이번 no-delete sync가 만든 결손이 아니다. 이 제한을 README와 route JSON에 남겼다.

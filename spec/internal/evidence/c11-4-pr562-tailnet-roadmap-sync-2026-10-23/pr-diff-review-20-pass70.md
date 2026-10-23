# C11.4 Tailnet roadmap sync pass70 PR diff 검토 20개

PR #562 병합 뒤 공개 roadmap 변경과 선택 RSPress site의 Tailnet 동기화 근거를 서로 다른 실패 관점으로 확인했다. 경로·source hash, 기존 미리보기 파일 보존, 공식 집계와 browser 종료 기록을 점검했다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | merge된 main과 선택 빌드 branch source를 혼동하는가 | origin/main `22eafb6`와 경로 수정 source `10ae04a`를 별도 기록하고 선택 HTML의 hash를 branch source와 비교했다. |
| 2 | root preparation이 이미 존재하는 선택 root를 덮어쓰는가 | 이전 pass69 root를 hardlink로 새 경로에 복제하고 기존 대상이 있으면 중단한다. |
| 3 | 이전 root의 hardlink 파일 내용을 새 overlay가 직접 변경하는가 | overlay 대상은 selected root에서 unlink 후 새 파일로 기록해 이전 root inode와 분리한다. |
| 4 | plan/spec source에 symlink나 비정규 entry가 섞이는가 | selected root traversal 및 Tailnet sync가 symlink와 지원하지 않는 entry에서 실패한다. |
| 5 | 문서 변경에 삭제가 포함되어 선택 root에서 파일이 누락되는가 | base-to-main 변경은 429개 A/M만이며 삭제 0건이다. 기존 Tailnet 파일도 지우지 않았다. |
| 6 | 로컬 미추적 파일이 main source evidence에 섞이는가 | 변경 문서는 `git show origin/main:<path>`에서 가져오고 branch override는 공개 roadmap 한 파일로 제한한다. |
| 7 | private 절대 경로가 문서 출력에 남는가 | 선택 root에서 기존 sanitizer를 적용하고 PNG는 byte-for-byte 보존한다. |
| 8 | Tailnet helper config가 문서 route로 빌드되는가 | selected root 안의 `rspress*.config.ts`를 복사본에서만 제거했고 원본 파일은 유지했다. |
| 9 | 상대 Markdown image URL 정규화가 파일을 불필요하게 수정하는가 | pass70 정규화 검사에서 변경 0개를 기록했다. |
| 10 | 참조된 로컬 이미지가 선택 root에 빠지는가 | 314 Markdown image 참조, workspace 누락 0개, 추가 import 0개를 기록했다. |
| 11 | 선택 문서 빌드를 전체 저장소 dead-link 검사로 과장하는가 | RSPress `checkDeadLinks`와 `checkDeadImages`를 비활성화했고 실제로 확인한 route만 기록한다. |
| 12 | raw HTML roadmap이 RSPress output에 포함되지 않아 오래된 경로가 제공되는가 | 공개 branch source를 `roadmap.html`로 atomic stage하고 SHA-256이 candidate file과 일치함을 확인했다. |
| 13 | public roadmap evidence 링크가 없는 `/spec/internal/` route를 가리키는가 | 새 카드의 내부 경로는 `/spinon/internal/...`으로 정정했고 선택 build 및 Agent Browser에서 route를 확인했다. |
| 14 | C11.4를 완료 처리하거나 72/213을 변경하는가 | public roadmap DOM은 checkbox 213개 중 72 checked였고 C11.4 미완료, `72/213`을 표시했다. |
| 15 | status anchor가 존재하지 않거나 C11.4 상태가 checked로 렌더되는가 | Agent Browser에서 `#css-구현-체크리스트`를 찾았고 C11.4 checkbox `checked=false`였다. |
| 16 | Android/iOS 화면 route가 다른 기기나 누락된 이미지로 연결되는가 | Tailnet에서 Android 1080×2400 및 iOS 1206×2622 screenshot을 직접 열었다. |
| 17 | preview 대상의 이전 파일이 backup 없이 교체되는가 | 732개 교체 파일을 같은 볼륨 hardlink로 보존하고 각 backup SHA를 검증했다. |
| 18 | 파일 복사 중 server가 부분 파일을 읽거나 unrelated preview file이 삭제되는가 | unique sibling temporary file 후 atomic rename을 사용했고 sync는 추가·교체만 수행했다. 삭제는 0건이다. |
| 19 | local files가 맞아도 Tailnet이 오래된 response를 제공하는가 | 변경 파일 1,699개와 remote route/asset 32개 모두 HTTP 200 및 source SHA-256 일치다. |
| 20 | browser 종료가 preview server나 사용자의 다른 Chrome을 함께 종료하는가 | Agent Browser 전용 session만 닫고 PID 1191 / port 8768은 유지했으며 browser process 목록에서 해당 session이 사라진 것을 확인했다. |

**검토 결과:** Tailnet 변경은 선택 문서 미리보기로 한정하고, 앱 runtime·production 배포·전체 C11.4 완료로 확대하지 않았다. 미해결 diff 결함은 발견되지 않았다.

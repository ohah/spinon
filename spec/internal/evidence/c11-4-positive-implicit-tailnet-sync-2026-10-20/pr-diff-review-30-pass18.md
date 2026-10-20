# C11.4 positive implicit Tailnet sync PR diff 검토 30관점

검토 대상은 PR #377의 roadmap-fragment 수정, 선택 build/sync helpers, sync 결과 및 Agent Browser evidence다. 기존 PR #375 구현 diff와 PR #376 문서 diff 검토를 재사용하지 않고 이 PR 변경의 실패 경로를 각각 확인했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | PR 기준이 PR #376 merge 전으로 돌아감 | pass18은 PR #376 merge SHA `98d4910539f0cc69ee8065f266038f1e996c03ce`의 후속 commit에서 빌드했다. |
| 2 | 로드맵 PR #375 설명이 이전 구현 상태로 남음 | 이전 “구현 전” 문구를 구현 PR #375 병합 사실로 교체했다. |
| 3 | 새 note에 제품 지원 또는 C11.4 전체 완료를 암시 | typed positive implicit list와 미완료 경계를 함께 표시한다. |
| 4 | official 완료 수를 수동 증가 | STATUS/roadmap checkbox를 그대로 두었고 browser는 72/212·unchecked를 확인했다. |
| 5 | 계획 link가 선택 route에 없음 | `/spinon/plan/c11-4-positive-implicit-track-sizing.html`이 pass18 serve manifest의 HTTP 200 항목이다. |
| 6 | contract href의 heading slug가 틀림 | pass16의 실패를 수정했고 pass17 Agent Browser가 실제 DOM ID에서 `anchor_found=true`를 확인했다. |
| 7 | 구현 근거 경로가 404 또는 이전 evidence로 연결 | roadmap의 README route가 HTTP 200·동일 hash이며 browser title이 C11.4 implementation evidence와 일치한다. |
| 8 | pass17/18 source commit이 기록과 불일치 | prepare/preflight JSON의 commit과 branch name을 evidence에 보존했다. |
| 9 | 선택 root manifest가 URL 출력 경로를 잘못 계산 | 131 destination mapping이 3,413 staging files로 출력됐고 74 unique source file groups의 manifest와 대조했다. |
| 10 | 전체 Rspress root 대신 선택 root 외 경로까지 읽음 | pass별 config는 unique root/outDir를 사용한다. |
| 11 | package install이나 Rspress upgrade가 잠입 | lockfile 또는 package 변경 없이 고정 `2.0.22` workspace binary를 썼다. |
| 12 | dead-link 검사 비활성화를 전체 링크 통과로 해석 | README는 선택 build의 dead-link 검사를 껐고 지정 route/이미지만 별도로 검사했다고 한정한다. |
| 13 | 본문·generated JS에서 개인 경로가 누출 | staging·sanitizer 결과 remaining host/worktree path는 0이다. |
| 14 | 새 pass 디렉터리가 기존 build를 덮어씀 | pass16/17/18는 각각 새 output root와 site root를 사용했다. |
| 15 | 잘못된 branch/seed가 roadmap-only 자료를 제외 | source seed와 prepared JSON의 branch·commit·selected mapping을 보존했다. |
| 16 | preflight 이후 source file set이 바뀜 | copy helper는 preflight file set·bytes·hash를 다시 대조하며 pass18 결과 모두 일치했다. |
| 17 | target이 preflight 후 외부 변경됨 | copy helper는 기존 preview bytes/hash를 다시 비교했다. |
| 18 | 같은 pass backup이 이전 백업을 조용히 덮음 | 각 실행에 새 pass 경로를 쓰고 existing backup은 expected prior hash와 같을 때만 허용한다. |
| 19 | 열린 HTML에 대한 backup 없이 교체 | PID 1191의 열린 두 HTML 이전 bytes/hash를 pass18 backup에 기록했다. |
| 20 | 실패한 pass16 링크가 교정 없이 남음 | pass16 false 결과와 pass17 true 결과를 같이 보존하고 pass18 sync review 문구도 최종 사실로 갱신했다. |
| 21 | copy가 site 밖 파일을 건드리거나 기존 자산 삭제 | helper는 `SpinonPreview` 내 selected path만 create/replace하며 no_deletes=true를 기록했다. |
| 22 | 생성·교체 수와 file total이 맞지 않음 | 103 + 102 + 2,610 = 2,815이다. |
| 23 | backup 개수 또는 old hash 검증이 누락 | 102/102 교체 backup이 expected prior SHA-256과 일치한다. |
| 24 | Copy output이 target에서 실제 제공되지 않음 | Tailnet HTTPS의 모든 2,815 path에서 HTTP 200·length·SHA가 source와 일치한다. |
| 25 | roadmap screenshot이 오래된 pass16 화면을 사용 | PR 이미지는 pass17 screenshot이며 source URL/viewport와 browser JSON이 함께 기록된다. |
| 26 | screenshot을 앱 화면으로 오해 | README와 PR 본문은 이를 Tailnet 문서 UI로, Android/iOS PNG는 별도 앱 evidence로 구분한다. |
| 27 | Android/iOS PNG 링크가 깨짐 | Agent Browser image load가 Android 1080×2400, iOS 1206×2622 둘 다 성공했다. |
| 28 | 상태 페이지 checkbox가 완료로 표시 | Tailnet STATUS의 C11.4 checkbox는 unchecked로 확인했다. |
| 29 | 계획 페이지 merge SHA/남은 scope가 오래됨 | browser가 PR #375 SHA, negative implicit/auto-repeat/WPT 미완료 문구를 확인했다. |
| 30 | 실행 후 브라우저나 shared preview process를 잘못 종료 | pass18 isolated session은 inactive/pid null이다. 공유 Agent Browser PID 67317과 Tailnet server PID 1191은 중지하지 않았다. |

PR merge 이후 공식 count, C11.4 전체 status, 앱 코드와 GitHub Pages는 이 변경에서 바꾸지 않는다.

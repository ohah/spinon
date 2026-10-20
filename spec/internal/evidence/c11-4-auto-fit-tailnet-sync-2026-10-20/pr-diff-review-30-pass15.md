# C11.4 post-merge Tailnet 문서 PR diff 검토 30개

**검토 대상:** PR #372 병합 상태를 공식 대장·계획·공개 roadmap에 연결하고 Tailnet preview/evidence를 추가한 diff. PR #372 runtime 구현 검토와 구분한다.

| # | 독립 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | merge SHA가 GitHub 실제 merge commit과 다른가? | **통과.** gh pr view 372에서 9996ee2089eaa4c0e86506af7fcea8f0cec3491c을 확인했고 대장·계획·roadmap이 같다. |
| 2 | 열려 있거나 닫힌 PR을 병합으로 표현하는가? | **통과.** GitHub PR state는 MERGED, mergedAt은 2026-10-20T01:18:53Z다. |
| 3 | check rollup이 비었는데 CI 통과를 주장하는가? | **통과.** statusCheckRollup은 비어 있고 GitHub required check 미설정을 명시했다. |
| 4 | 완료 체크가 실수로 켜지는가? | **통과.** 공식 C11.4 checkbox를 false로 두고 변경 diff도 설명 문구만 바꿨다. |
| 5 | C11 parent를 제한 slice 하나로 완료 처리하는가? | **통과.** 상위와 하위 완료 체크는 변경하지 않았다. |
| 6 | 상위 구현 완료 숫자를 수동으로 올리는가? | **통과.** roadmap summary note만 추가했고 done task source나 status parsing 로직은 수정하지 않았다. |
| 7 | 공개 칸반이 실제 상태와 다른 수를 표시하는가? | **통과.** Agent Browser SVG의 완료 값 72가 공식 STATUS-derived task state와 맞고, 관련 체크 추가는 없다. |
| 8 | 계획에 PR이 여전히 submitted라고 남는가? | **통과.** plan과 STATUS 문구 모두 Rebase and merge 및 SHA를 기록한다. |
| 9 | Rebase merge에서 원본 commit SHA를 merge SHA로 오기하는가? | **통과.** mergeCommit.oid를 GitHub에서 재조회해 썼다. |
| 10 | 53 scene을 PR #372의 53 신규 scene으로 과장하는가? | **통과.** README/roadmap은 각 플랫폼 total 53 scene이며 신규 추가분 20개로 분리한다. |
| 11 | 4/4 표본을 전체 surface pixel equality로 일반화하는가? | **통과.** Surface/Metal 표본의 제한된 네 점만 적고 compositor-after 범위를 남긴다. |
| 12 | Simulator 결과를 실기기 결과로 소개하는가? | **통과.** Android는 API 37 ARM64 AVD, iOS는 iPhone 17 Pro Simulator라고 적고 physical device를 제외한다. |
| 13 | Android AVD renderer를 hardware GPU라고 부르는가? | **통과.** README의 runtime 한계에서 hardware GPU를 미검증으로 남긴다. |
| 14 | iOS drawable 표본을 compositor 후 표시 픽셀이라고 하는가? | **통과.** iOS compositor 이후 픽셀은 남은 범위에 포함한다. |
| 15 | Chrome 155 154/162px를 제품 기대값으로 채택하는가? | **통과.** 규범 기대 162/170px와 관찰값을 대조한다. |
| 16 | out-of-flow item을 runtime 지원으로 셈하는가? | **통과.** C11.1 거부를 기록하고 in-flow second-repeat evidence를 연결했다. |
| 17 | PR #372 evidence README 링크가 빠지거나 경로가 틀리는가? | **통과.** 구현 README route를 Tailnet에서 열고 overflow screenshot 두 장이 로드됐다. |
| 18 | road map의 신규 local link가 선택 build에 빠지는가? | **통과.** plan, implementation evidence, sync README route가 모두 Agent Browser에서 열렸다. |
| 19 | Korean 사용자에게 문서가 다른 언어로 바뀌는가? | **통과.** project language rule과 기존 한글 문체를 유지했다. |
| 20 | PR #372 browser screenshot을 앱 screenshot으로 재사용하는가? | **통과.** simulator 화면과 Tailnet browser screenshot을 별도 파일·설명으로 보관한다. |
| 21 | README에 post-merge 자료를 넣으며 기존 pass14 evidence를 덮어쓰는가? | **통과.** 기존 기록 뒤에 pass15 section을 prepend했고 pass14 결과 파일은 변경하지 않았다. |
| 22 | sync evidence에 root path나 사용자 절대 경로가 들어가는가? | **통과.** 결과 경로는 repo-relative이며 산출물 host-path scan이 0개 남음을 기록한다. |
| 23 | 임시 의존성 설치로 bun.lock/package version을 바꾸는가? | **통과.** 기존 Rspress 2.0.22 workspace dependency를 재사용했다. lockfile·package version diff는 없다. |
| 24 | build root가 전체 저장소라 불필요한 문서·자료를 공개하는가? | **통과.** prior selected root mapping을 재사용하고 최신 C11.4 계획·근거만 추가해 선택 root를 만들었다. |
| 25 | build HTML에 host/worktree path가 남는가? | **통과.** sanitizer 기록은 redacted 1개 asset, remaining_host_path_files 0개다. |
| 26 | 기존 미리보기 파일을 선택 목록 밖에서도 삭제하는가? | **통과.** copy는 create/replace만 하고 no_deletes=true, 삭제 0개로 끝났다. |
| 27 | 교체 전에 복원 가능한 원본이 남지 않는가? | **통과.** 대상 76개를 지정 backup root에 저장하고 전부 SHA-256을 대조했다. |
| 28 | Tailnet server나 사용 중 tmux가 sync 과정에서 종료되는가? | **통과.** PID 1191은 전후 유지됐고 tmux에는 신호를 보내지 않았다. |
| 29 | Agent Browser 확인 뒤 Chrome daemon이 남아 있는가? | **통과.** 이번 고유 session은 active=false, pid=null로 닫았고 기존 default session은 보존했다. |
| 30 | 이 문서 sync가 Pages 또는 앱 배포로 잘못 설명되는가? | **통과.** Tailnet static preview만 갱신했고 GitHub Pages workflow와 mobile app을 실행하지 않았다. |

30개 관점은 서로 다른 상태 원본, 링크, 증거 경계, 동기화 내구성, 프로세스 수명 실패 조건을 확인한다. 발견은 source path 오기와 초기 임시 CLI 사용이었으며 수정 뒤 final build/sync를 다시 확인했다.

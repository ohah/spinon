# C11.4 PR #391 Tailnet 문서 PR 변경분 실패 관점 검토 30개

**검토 대상:** PR #391 병합 이후 `spec/STATUS.md`, 내부 인덱스, C11.4 계획/evidence와 공개 HTML roadmap의 문서 변경 및 Tailnet 선택 동기화. 아래 검토는 PR #391 구현 review가 아니라 이번 문서 변경의 별도 검토다.

| # | 실패 조건 | 확인 사항 |
| ---: | --- | --- |
| 1 | PR #391이 merge 전인데 문서가 병합으로 기록된다. | GitHub PR state 및 merge commit을 다시 조회한다. |
| 2 | 문서에 다른 merge SHA가 적힌다. | PR #391 merge commit `e3119ea21bdd6196f334c104edb3bb33dfe70b0c`와 맞춘다. |
| 3 | required status check이 없는데 CI 통과로 보고한다. | GitHub check rollup을 보고된 상태 그대로 적고 빈 rollup을 통과로 해석하지 않는다. |
| 4 | 공식 상태 대장의 C11.4 checkbox가 조기에 켜진다. | task ID checkbox diff를 확인해 미완료를 유지한다. |
| 5 | 공식 완료 수가 증가한다. | 구현 상태 수치의 before/after를 비교한다. |
| 6 | 제한 구현이 전체 CSS Grid로 확대 서술된다. | `RuntimeBlockFlexGridTrackSizingV1` 범위와 미지원 grammar를 보존한다. |
| 7 | simulator 결과를 실기기 결과로 표현한다. | Android API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator로 명시한다. |
| 8 | iOS drawable 표본을 compositor 이후 픽셀로 표현한다. | edge 표본이 root background이고 compositor 이전 readback임을 유지한다. |
| 9 | Android API 37 AVD를 hardware GPU 증거로 표현한다. | software/SwiftShader AVD 한계를 유지한다. |
| 10 | 138 scene이 신규 138개로 오해된다. | 기존 124개 + 신규 14개인 matrix 구성을 명시한다. |
| 11 | 한 플랫폼의 scene 일부만 통과했는데 전체 성공이라 쓴다. | Android와 iOS를 각각 138/138로 분리 확인한다. |
| 12 | 비교 지표 개수가 틀린다. | cross-platform JSON에서 frame scalar 960개, used-track size 1,019개를 확인한다. |
| 13 | 비교 오차가 0이 아닌데 exact라고 적는다. | 동일 scene의 frame/track 최대 차이가 0 CSS px인 JSON을 확인한다. |
| 14 | surface sample 24/24가 24개 scene으로 오해된다. | 표본이 5개 scene에 분포하며 기존 auto-fill scene은 4/4임을 쓴다. |
| 15 | root edge sample을 CSS border stroke 검사로 말한다. | 가장자리 표본은 root background로 제한한다. |
| 16 | WPT 또는 전체 적합성 완료를 암시한다. | WPT와 전체 CSS Grid 범위가 미완료임을 명시한다. |
| 17 | 테스트 수가 PR #391 실행 근거와 달라진다. | workspace, runtime fixture, Clippy, rustfmt, fixture boundary 수치는 근거 README와 대조한다. |
| 18 | source evidence README가 PR #391 merge 상태와 모순된다. | 같은 README의 기존 PR #385 기록과 후속 PR #391 기록을 구별한다. |
| 19 | plan의 구현 상태가 PR #391 open 상태로 남는다. | 병합 상태 및 SHA 설명을 확인한다. |
| 20 | 내부 인덱스와 공개 roadmap 설명이 서로 다르다. | PR, 범위, 138 scene 및 미완료 상태의 표현을 비교한다. |
| 21 | 새 markdown 링크가 선택 root에서 빠진다. | prepare mapping이 plan, evidence README, review 및 JSON의 route를 포함하는지 확인한다. |
| 22 | 이미지가 HTML page route에서 404가 된다. | evidence image는 raw path와 public static route에 모두 복사하고 Tailnet HTTP 상태를 확인한다. |
| 23 | 상대 markdown 링크가 이전 worktree 또는 로컬 파일을 가리킨다. | 선택 mapping은 `/spinon/` route로 rewrite하고 외부 경로만 GitHub fallback으로 둔다. |
| 24 | 전체 docs build의 기존 dead link를 이번 변경 회귀로 오인하거나 감춘다. | 변경 전 보고된 642개 그룹과 새 link 결과를 구분하며 선택 root build를 별도로 기록한다. |
| 25 | GitHub Pages 또는 `gh-pages`로 preview를 배포한다. | 이번 작업은 사용자가 지정한 Tailnet static root만 동기화하고 Pages workflow를 실행하지 않는다. |
| 26 | Tailnet sync 중 서버 또는 tmux를 종료한다. | PID 1191 및 기존 tmux 세션 상태를 보존한다. |
| 27 | 새 worktree를 만들어 다른 checkout을 수정한다. | `git rev-parse --show-toplevel`이 primary checkout이며 현재 작업은 그 checkout 안의 문서 branch에서만 한다. |
| 28 | unrelated untracked evidence를 PR에 포함한다. | `git add`는 검토한 task 문서와 새 report 디렉터리만 명시한다. |
| 29 | browser snapshot만 성공했는데 raw screenshot을 첨부했다고 한다. | 캡처 파일 존재/크기와 PR raw image URL 표시를 각각 검증한다. |
| 30 | browser 확인 후 Agent Browser process가 남는다. | 이번 실행에서 시작한 named session을 닫고 session 목록을 확인한다. |

## 결과

30개 별도 실패 관점을 최종 문서 diff와 실행 근거에 대조했다. pass16 Agent Browser에서 실행 evidence 페이지의 새 Simulator 이미지 3개가 Tailnet이 아닌 GitHub blob 경로로 나가는 결함을 찾았다. 선택 빌더에 Android·iOS negative/positive implicit track 이미지와 iOS typed-auto 이미지를 추가했다. Rspress는 처음 `bun x`가 최신 1.47.2를 받아 저장소 테마 alias를 찾지 못했으나, 저장소 고정 `packages/docs/node_modules/.bin/rspress` 2.0.22로 빌드해 해결했다. 빌드가 만든 FlexSearch worker의 `/Users/...` file URL도 정적 preview worker asset으로 바꾸고 host/worktree path가 산출물에 남지 않는지 재검사했다.

pass17 전체 1,612개 파일은 HTTP 200과 byte/SHA-256으로 모두 맞았고, backup 96/96 및 열린 교체 파일 0개였다. Agent Browser에서는 PR #391 merge SHA, C11.4 checkbox `false`, 계획의 미완료 경계와 runtime evidence 이미지 3개 로드를 확인했다. pass18 최종 정적 build/served 결과는 pass18 JSON과 최종 PR 상태에 연결한다. 공식 완료 수 및 C11.4 checkbox는 변경하지 않으며, GitHub Pages·앱·simulator를 이번 Tailnet 문서 갱신에서 실행하지 않는다.

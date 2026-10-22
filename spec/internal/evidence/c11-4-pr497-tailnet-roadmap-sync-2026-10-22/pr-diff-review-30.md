# PR #498 Tailnet preview 변경 diff 검토 · 30개

PR diff의 문서와 sync 증거를 구현 계약, 게시 경로, 복구 가능성, 실제 화면 관점에서 다시 검토했다.

| # | 검토 관점 | 결과 |
|---:|---|---|
| 1 | 추가된 scope가 PR #497 구현을 재수정하는가 | 통과. PR #498은 동기화 설명·검증 산출물만 보탠다. |
| 2 | 기준 merge SHA가 다른 PR을 가리키는가 | 통과. #497 Rebase and merge 결과 `7de0bc1983a5c824468c9994b9ee6404dadef7ad`를 기록한다. |
| 3 | 현재 완료 수가 오래된 72/213인가 | 통과. 새 노트는 현재 roadmap parser 집계 `77/213`을 사용한다. |
| 4 | C11.4를 전체 완료로 표기하는가 | 통과. 로드맵과 계획에서 C11.4 전체를 미완료로 적는다. |
| 5 | `spec/STATUS.md` official item을 무단 체크하는가 | 통과. 이 PR은 STATUS 원문을 수정하지 않는다. |
| 6 | 선택 fixture를 일반 CSS 지원으로 확대 해석하는가 | 통과. 정확히 네 fixed scene의 제한 결과라고 명시한다. |
| 7 | AVD 결과를 실기기 Android 근거로 쓰는가 | 통과. API 37 ARM64 AVD와 physical device를 구분한다. |
| 8 | iOS drawable 표본을 compositor 이후 색상으로 주장하는가 | 통과. 제출 전 Metal drawable 결과로 한계를 유지한다. |
| 9 | WPT나 전체 CSS Grid 적합성으로 과장하는가 | 통과. WPT와 일반 CSS admission 미검증을 명시한다. |
| 10 | GitHub Pages를 배포한 것처럼 보이는가 | 통과. Tailnet preview만 갱신했고 Pages/gh-pages는 변경하지 않았다. |
| 11 | `@rspress/core` 고정 버전 정책을 위반하는가 | 통과. pinned RSPress `2.0.22`를 유지한다. |
| 12 | 전체 문서 사이트 build 성공으로 과장하는가 | 통과. 별도 selected Tailnet root/output build라고 적는다. |
| 13 | dead image 검사 비활성화가 공개 build 설정으로 유입되는가 | 통과. preview 전용 config에서만 기존 기록 이미지 검사 옵션을 끈다. |
| 14 | source root가 이전 기록을 파괴하는가 | 통과. pass24를 복사한 새 pass25 root에서 선택 문서만 overlay했다. |
| 15 | 계획·contract·runtime evidence 중 하나가 누락되는가 | 통과. 새 roadmap entry에서 plan, precomparison, run evidence를 연결한다. |
| 16 | preview URL이 로컬 파일 경로로 바뀌는가 | 통과. Tailnet `/spinon/` base URL을 Agent Browser에서 직접 열었다. |
| 17 | 새 내부 README 항목에서 evidence page가 끊기는가 | 통과. 내부 README와 sync evidence route가 실제 Tailnet에서 열린다. |
| 18 | PR #497 screenshot raw URL이 삭제 branch SHA에 묶이는가 | 통과. PR body 이미지 링크는 merge SHA tree를 가리키고 Agent Browser에서 다시 로드했다. |
| 19 | Tailnet screenshot에 최신 roadmap 항목이 없는가 | 통과. 로드맵 screenshot에는 77/213 및 PR #497 entry가 보인다. |
| 20 | 계획 화면이 다른 작업 문서로 바뀌는가 | 통과. browser title과 `[29,29]`, `[12,12]` track 값이 맞다. |
| 21 | runtime README가 Android 캡처만 보여주는가 | 통과. Android 1080×2400과 iOS 1206×2622 이미지를 모두 읽었다. |
| 22 | native surface black을 browser page white로 섞는가 | 통과. runtime evidence는 Chrome page backdrop과 native surface clear를 구분한다. |
| 23 | STATUS page에서 C11.4가 완료로 보이는가 | 통과. Agent Browser에서 official C11.4 description의 미완료 상태를 확인했다. |
| 24 | 초기 대상 집합의 파일 수·생성·교체·동일 수가 맞지 않는가 | 통과. 초기 1,746개 집합과 create/replace/same 수를 preflight/copy manifest에 기록했다. |
| 25 | 교체 전 파일 백업 수가 변경 수보다 적은가 | 통과. 각 단계에서 98개와 103개 backup을 별도 root에 보존하고 hash 검증했다. |
| 26 | final copy가 사용 중 파일 삭제나 덮어쓰기를 했는가 | 통과. 삭제는 없고 최종 open-handle 관찰도 없었다. |
| 27 | Tailnet 서버를 종료해 preview를 끊었는가 | 통과. PID 1191과 `127.0.0.1:8768` listener가 계속 살아 있다. |
| 28 | Agent Browser 종료가 기존 사용자 Chrome을 닫는가 | 통과. 전용 session 종료 뒤 browser 관련 process는 0개이며 일반 Chrome PID 20311은 유지됐다. |
| 29 | 증거 파일이 review 이후 변했는데 manifest가 오래됐는가 | 통과. 로그 끝 공백을 정규화한 뒤 SHA256SUMS.txt를 다시 만들고 모든 항목의 체크섬이 일치했다. |
| 30 | 이전 개발 작업물이나 untracked evidence를 함께 stage하는가 | 통과. staged path는 spec/internal/README.md와 이번 Tailnet sync evidence 폴더 26개 파일뿐이다. |

### PR #498 화면·병합 전 상태

PR #498 본문에 Tailnet 로드맵과 runtime evidence screenshot을 올렸다. Agent Browser에서 두 raw GitHub 이미지가 `complete=true`, 각각 1680×1100으로 로드되는 것을 확인했다. `gh pr view`의 merge state는 `CLEAN`, status rollup은 비어 있고 `gh pr checks --required`는 이 branch에 보고된 required checks가 없다고 반환했다. 이 응답은 `pr-498-agent-browser-verification.json`에 저장했다. Rebase merge 뒤 PR 본문 image URL은 최종 merge SHA로 바꾸고 다시 확인한다.

최종 manifest는 로그 공백 정규화 뒤 재생성하고 모든 checksum을 확인했다. PR diff는 `spec/internal/README.md`와 Tailnet sync evidence 폴더만 포함한다.

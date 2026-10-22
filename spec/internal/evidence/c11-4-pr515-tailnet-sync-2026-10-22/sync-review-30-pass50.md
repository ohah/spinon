# Tailnet pass50 동기화 실패 관점 검토 30개

검토 대상은 선택된 RSPress root, preview target·backup, route hash verifier 및 Agent Browser 확인 절차다. `SpinonPreview`에서 파일을 삭제하지 않고 기존 server를 유지한다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 잘못된 이전 build를 바탕으로 새 상태를 만든다 | pass49 selected root를 복제하고 source revision 및 갱신 문서 hash를 preparation manifest에 기록했다. 통과. |
| 2 | workspace의 unrelated untracked evidence를 통째로 공개한다 | PR #515 최종 README, 2개 full result JSON, manifests, 대표 logs, watchdog log 및 화면만 curated list에 넣었다. 통과. |
| 3 | pass50 경로가 이미 있어 다른 결과물을 덮어쓴다 | selected root와 output 경로는 pass50 전용이며 생성 스크립트는 기존 root가 있으면 거부한다. 통과. |
| 4 | 선택 문서 안에 repository 절대 경로가 남는다 | preview 복사본에서 workspace, Android SDK, home prefix를 치환한다. PNG는 byte 보존한다. 통과. |
| 5 | selected root의 symlink가 예상 밖 경로 파일을 끌어온다 | 모든 source tree entry를 재귀 열거하며 symlink와 일반 파일·directory 외 항목을 거부했다. 통과. |
| 6 | 기존 preview root나 중간 target path가 symlink다 | preflight는 root와 각 path segment를 `lstat`로 검사한다. 통과. |
| 7 | directory 위치에 파일이 있거나 반대 유형이 존재한다 | target traversal이 중간 non-directory와 끝 non-file을 거부한다. 통과. |
| 8 | 다른 RSPress 버전으로 빌드되어 asset graph가 흔들린다 | `@rspress/core` `2.0.22` pin을 사용했다. 통과. |
| 9 | file count가 같지만 일부 byte가 달라졌는지 놓친다 | site tree hash는 정렬된 route, file size와 SHA-256을 포함한다. 7,033 files, tree hash `53bffe…ace5d`. 통과. |
| 10 | roadmap/status/plan/evidence route 누락을 발견하지 못한다 | build summary에서 11개 required route와 두 image file 존재를 확인했다. 통과. |
| 11 | 기존 route hydration이 stale roadmap 내용으로 남는다 | 전체 RSPress site를 다시 빌드했고 Agent Browser DOM에서 fixed-slice runtime result를 읽었다. 통과. |
| 12 | C11.4 체크박스가 구현 slice만으로 완료 처리된다 | Agent Browser에서 공식 C11.4 parent checkbox `checked=false`를 확인했다. 통과. |
| 13 | 완료 집계가 `77/213`에서 오르거나 오래된 값으로 돌아간다 | build·Agent Browser STATUS와 roadmap에서 `77/213`을 확인했다. 통과. |
| 14 | 계획이 runtime 미시작이라고 계속 보고한다 | plan route에 263/263 양 플랫폼 draw/present와 219/219 pixel sample 요약이 보인다. 통과. |
| 15 | 내부 계약에 이전 AVD 실패 상태가 남는다 | contract 0059와 public status를 같은 source로 선택 root에 overlay하고 route를 생성했다. 통과. |
| 16 | 실행 근거의 새 link가 preview에서 404다 | PR #515 evidence README route가 생성되고 browser에서 열렸다. 통과. |
| 17 | 대표 runtime log가 evidence README link와 다르다 | Android/iOS intrinsic·definite logs를 README와 같은 directory 아래로 복사했다. 통과. |
| 18 | 263 scene 결과를 일부만 포함하거나 표본 수를 오계산한다 | source matrix JSON을 263 observations로 검사하고 219/219·63 scene은 runtime logs를 재집계했다. 통과. |
| 19 | PR screenshot이 잘못되었거나 빈 파일이다 | Agent Browser에서 Android `1080×2400`, iOS `1206×2622` image route를 열었다. 통과. |
| 20 | 제한된 fixture 결과를 전체 CSS 증거로 보인다 | README, plan, STATUS와 roadmap에 simulator, glyph, WPT, 일반 CSS 경계를 명시했다. 통과. |
| 21 | full test run의 민감하지 않은 오류 자료가 누락된다 | anonymous fixture routing 실패 전·후 로그와 SwiftShader watchdog context를 포함했다. 통과. |
| 22 | create/replace/same 분류가 복사 전에 변한다 | preflight 결과는 create 485, replace 459, same 6,089며 copy 직전에 존재·size·hash를 다시 확인했다. 통과. |
| 23 | preview 내 관련 없는 파일을 정리하면서 삭제한다 | copy script는 selected output만 create/replace하고 어떠한 delete 작업도 하지 않는다. 통과. |
| 24 | 지난 pass의 backup directory와 섞어 복구 불가능해진다 | 새 backup root를 사용하고 존재하면 실행을 중단한다. 통과. |
| 25 | replacement backup이 생성됐지만 byte가 달라진다 | 459개 backup의 원래 size와 SHA-256을 확인했다. 통과. |
| 26 | copy 중 기존 target state가 바뀐 것을 모르고 덮어쓴다 | copy loop가 preflight existence·size·hash를 파일별로 재검증한다. 통과. |
| 27 | 파일을 직접 truncate해 동시 reader가 부분 output을 본다 | source bytes를 unique temp file에 쓰고 rename으로 적용한다. 통과. |
| 28 | copy가 성공했지만 local preview hash가 다르다 | local final verify에서 7,033/7,033 target, 459/459 backup, 실패 0이다. 통과. |
| 29 | local copy는 맞지만 Tailnet server가 stale/다른 파일을 응답한다 | origin의 7,033 routes가 모두 HTTP 200이며 source bytes와 SHA-256이 일치했다. 통과. |
| 30 | browser check 과정에서 사용자 browser, preview server 또는 tmux를 닫는다 | isolated Agent Browser session만 사용하고 종료 후 session/process를 확인한다. PID 79812와 tmux·user Chrome을 보존한다. 통과. |

최종 sync 수치는 [README-pass50](./README-pass50.md), [preflight summary](./preflight-summary-pass50.json), [local verify](./final-verification-pass50.json), [Tailnet verify](./remote-verification-pass50.json) 및 [Agent Browser 결과](./agent-browser-verification-pass50.json)에 있다.

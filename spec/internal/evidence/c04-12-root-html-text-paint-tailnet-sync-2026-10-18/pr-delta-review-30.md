# Tailnet 로드맵 동기화 PR diff 실패 관점 검토 30개

검토 대상은 C04.12 direct text 상태 링크, 계획 상태, public roadmap 설명, Tailnet 동기화 근거와 화면 캡처다. 코드는 바꾸지 않았다.

| # | 독립 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | C04.12 상위 항목이 제한 fixture의 성공만으로 완료 처리되지 않는가 | 통과: 상위 체크는 미완료다. |
| 2 | C15 상위 항목이 direct text slice 때문에 완료 처리되지 않는가 | 통과: C15 미완료 문구가 유지된다. |
| 3 | direct text 행의 `[x]`를 이 문서 PR에서 새로 추가하지 않았는가 | 통과: 기존 병합 행에 PR 링크만 보탠다. |
| 4 | 기본 `RuntimeSession`의 HTML text 지원을 잘못 암시하지 않는가 | 통과: 기본 경로 미연결을 명시한다. |
| 5 | opt-in fixture와 제품 기본 경로가 구분되는가 | 통과: fixture 전용이며 기본 화면 경로가 아니라고 쓴다. |
| 6 | supported text profile을 임의 HTML/CSS로 일반화하지 않는가 | 통과: `XX`/Ahem과 전체 지원의 한계를 적었다. |
| 7 | font loading이 구현됐다고 오해할 수 있는가 | 통과: font loading은 미지원으로 남긴다. |
| 8 | nested inline layout을 구현 범위에 포함시키는가 | 통과: 지원 완료가 아니라고 명시한다. |
| 9 | PR #317의 구현 상태가 올바른가 | 통과: GitHub PR #317 링크로 Rebase and merge 상태를 연결한다. |
| 10 | PR #318의 re-export 보완 상태가 올바른가 | 통과: GitHub PR #318 링크로 별도 병합을 연결한다. |
| 11 | PR #318이 구현 기능 범위를 넓힌 것으로 표현되는가 | 통과: public re-export 보완으로 구분한다. |
| 12 | C04.12 Canvas 배경 이력의 74/207이 현행 수로 오인되는가 | 통과: 당시 수라고 표시하고 현재 수는 하드코딩하지 않는다. |
| 13 | 공식 집계 숫자가 `roadmap.html`에 고정되는가 | 통과: 페이지 스크립트가 공식 STATUS를 읽는다. |
| 14 | 동적 집계의 현 시점 수가 브라우저에서 확인됐는가 | 통과: Agent Browser가 76/209를 표시했다. |
| 15 | 76/209를 원시 Markdown 체크박스 개수와 혼동하는가 | 통과: 로드맵 동기화 메모의 78+129+2 공식 분류를 기록한다. |
| 16 | 로드맵 direct PNG 링크가 실제 Tailnet에서 열리는가 | 초기 검증에서 404를 재현해 링크를 제거했다. |
| 17 | 깨진 링크 제거 뒤 읽을 수 있는 Simulator 증거가 남는가 | 통과: 근거 README 링크를 유지하고 Android/iOS 이미지 natural size를 확인했다. |
| 18 | Android 그림이 원본 표면 크기로 제공되는가 | 통과: Agent Browser에서 840×630으로 로드됐다. |
| 19 | iOS 그림이 원본 캡처 크기로 제공되는가 | 통과: Agent Browser에서 1206×2622로 로드됐다. |
| 20 | 두 simulator 결과를 실기기 결과로 넓히는가 | 통과: AVD와 Simulator로 표기하고 실기기 한계를 남겼다. |
| 21 | Android software renderer가 hardware GPU로 잘못 설명되는가 | 통과: AVD/SwiftShader 경계와 hardware GPU 미검증을 유지한다. |
| 22 | iOS drawable readback이 compositor 이후 pixel로 잘못 표현되는가 | 통과: Metal drawable 및 compositor 전 한계를 유지한다. |
| 23 | 선택 빌드를 전체 docs build로 과장하는가 | 통과: 선택 RSPress build로 기록한다. |
| 24 | dead-link 검사를 실행했다고 주장하는가 | 통과: 선택 build에서 해당 검사를 껐고 전체 링크 감사 미실행을 명시한다. |
| 25 | RSPress 버전이 고정 정책을 위반하는가 | 통과: 2.0.22를 사용했고 package/lock 변경은 없다. |
| 26 | plan·contract·implementation evidence Tailnet route가 실제로 있는가 | 통과: Agent Browser에서 세 경로를 열었다. |
| 27 | Tailnet과 local preview가 같은 최종 산출물을 제공하는가 | 통과: 최종 88개 파일 모두 두 origin에서 바이트와 SHA-256가 일치했다. |
| 28 | preview의 기존 파일이 백업 없이 덮이거나 삭제되는가 | 통과: 최종 88개 manifest 중 기존 64개를 대조·백업하고 신규 24개 경로를 추가했으며 삭제는 0개다. |
| 29 | preview listener나 기존 tmux 세션을 종료/재시작하는가 | 통과: PID 1191 유지; tmux에 명령을 보내지 않았다. |
| 30 | 340개 기존 변경 경로를 가진 공유 checkout에 문서 변경이 섞이는가 | 통과: 전용 worktree에서만 편집했고 공유 checkout의 변경 수는 340으로 유지됐다. |

검토 범위 밖: 전체 문서 build, 모든 site 링크, 제품 runtime 동작, 실기기·hardware GPU, GitHub Pages 배포.

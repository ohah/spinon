# C11.3.3 병합 후 공식 문서 PR 변경 검토 30개

검토 대상은 PR #970 병합 사실을 공식 상태에 반영하고 raw roadmap에 제한 slice를 추가한 문서 변경이다. 검토 기준은 현재 `origin/main`의 상태 대장과 GitHub PR merge metadata이며 Tailnet 배포 결과는 다음 별도 단계에서 검증한다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | merge 전에 후속 문서가 병합 완료를 주장하는가 | GitHub에서 PR #970 `MERGED`와 merge SHA를 조회한 뒤 반영했다. PASS. |
| 2 | raw roadmap 날짜가 로컬 시각으로 바뀌었는가 | card에 GitHub `mergedAt` UTC `2026-11-01T05:59:35Z`를 쓴다. PASS. |
| 3 | merge SHA와 PR head SHA를 혼동하는가 | card와 STATUS는 `dd178e7...` merge SHA를 기록하고 head는 merge evidence에서 따로 보존한다. PASS. |
| 4 | merge strategy를 잘못 표기하는가 | GitHub merge result는 Rebase and merge다. PASS. |
| 5 | check 미보고를 CI 성공으로 설명하는가 | `gh pr checks` 결과는 no checks reported이며 docs는 성공으로 세지 않는다. PASS. |
| 6 | top-level C11이 완료로 바뀌는가 | `spec/STATUS.md`의 C11 checkbox는 계속 미완료다. PASS. |
| 7 | 중간 C11.3 parent가 완료되는가 | C11.3 checkbox는 계속 미완료다. PASS. |
| 8 | C11.3.3 template-area candidate가 완료되는가 | 해당 candidate checkbox는 계속 미완료다. PASS. |
| 9 | 전체 공식 수가 slice merge만큼 증가하는가 | 상태 대장과 roadmap은 `72/213`을 그대로 유지한다. PASS. |
| 10 | C11 progress card가 완료 수 집계의 근거로 오인되는가 | card는 limited slice라며 checkbox/count와 별도 의미를 적는다. PASS. |
| 11 | internal contract numeric version이 불필요하게 증가하는가 | 숫자 버전 `0.1.0`은 그대로다. PASS. |
| 12 | #967 one-axis behavior가 양 축으로 대체됐다고 오해하는가 | card와 contract는 기존 one-axis mode와 별도 typed mode라고 구분한다. PASS. |
| 13 | area template 및 일반 CSS admission을 새로 지원한다고 읽히는가 | 제한 범위를 no-area opt-in fixture로 한정한다. PASS. |
| 14 | auto sizing과 fixed pattern 계산 범위를 넓혀 표현하는가 | 두 축 `auto`/fixed-px pattern controls와 equal/reversed endpoints만 요약한다. PASS. |
| 15 | test 수가 전부 green이라고 읽히는가 | 330 layout, 19 CSS reference, 30 fixture boundary를 별도 적고 style suite의 실패를 남긴다. PASS. |
| 16 | C07.3 실패를 숨기거나 candidate failure로 돌리는가 | 276 passed·1 unrelated float serialization assertion failure로 적는다. PASS. |
| 17 | 최신 Simulator run ID가 기존 capture와 뒤섞이는가 | Android `055432072Z-227c6067`, iOS `055554422Z-f9734042` links를 exact 경로로 둔다. PASS. |
| 18 | surface sample이 geometry 전체의 proof로 과장되는가 | 6/6 pixel samples와 13 element frames를 구별한다. PASS. |
| 19 | 기존 one-axis regression 또는 full matrix 범위를 착각하는가 | #967 one-axis와 기존 39-scene geometry를 regression으로 표기한다. PASS. |
| 20 | WPT conformance로 표현되는가 | Spinon WPT는 명시적으로 미검증이다. PASS. |
| 21 | SwiftShader를 hardware GPU로 오해하는가 | Android AVD/SwiftShader로 명시한다. PASS. |
| 22 | iOS compositor 이후 pixels로 오해하는가 | Metal drawable comparison이 compositor 제출 전이라고 명시한다. PASS. |
| 23 | fixture marker archive 검사 범위를 앱 전체로 읽는가 | macOS static archive 한계와 APK/IPA 미검증을 적는다. PASS. |
| 24 | style fallback 결함 및 수정 이력이 사라지는가 | root README와 review link가 auto-size rejection 수정 및 focused regression을 기록한다. PASS. |
| 25 | plan 문서의 상태가 merge 전으로 남는가 | plan에 PR #970 merge time/SHA를 반영한다. PASS. |
| 26 | internal contract 제목이 unmerged라고 남는가 | 제목을 merged limited slice로 바꾸고 parent C11 incomplete를 유지한다. PASS. |
| 27 | merge evidence route를 raw roadmap에서 찾을 수 없는가 | card는 `merge-PR970.md` route를 연결한다. PASS. |
| 28 | PR review와 post-merge review 링크가 혼동되는가 | 구현 PR diff 30과 병합 후 상태 대조 30은 별도 경로다. PASS. |
| 29 | 새 card HTML이 다른 C11.3.3 history를 대체하거나 중복 ID를 만드는가 | 새 card를 최신 progress item으로 앞에 추가하고 #967 history를 보존한다. PASS. |
| 30 | Tailnet preview가 아직 이전 버전인데 동기화했다고 주장하는가 | source 변경만 이 PR에 기록하며 Tailnet sync와 Agent Browser 결과는 이후 단계로 남긴다. PASS. |

최종 대조는 merge SHA, 날짜, count, checkbox와 링크를 확인했다. GitHub Pages 배포는 포함하지 않는다. Tailnet preview의 선택 파일 복사·hash·화면 확인은 별도 evidence로 수행한다.

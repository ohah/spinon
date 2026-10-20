# C11.4 PR #385 Tailnet 동기화 PR diff 적대 검토 30개

검토 대상은 PR #386 병합 뒤 생성한 Tailnet sync PR 전체 diff다. 저장소의 문서 상태, 선택 build, preflight, backup, served-hash 검증, Agent Browser 화면과 cleanup 기록을 대조했다.

| # | 독립 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | sync PR에 제품 소스나 무관한 로드맵 변경이 섞이는가 | 변경은 이번 Tailnet sync의 reproducible helper와 실행·검토 증거에 한정한다. 제품 소스 수정은 없다. 통과. |
| 2 | sync가 별도 worktree에서 main 문서를 섞거나 dirty untracked 증거를 몰래 선택하는가 | 기본 spinon checkout 안의 docs/c11-4-pr385-tailnet-sync-20261020 브랜치에서 작업했다. Preflight dirty 상태는 보존한 untracked simulator/C12.4 증거와 이 실행 기록 때문이며 README와 선택 source manifest에 이를 구분했다. 통과. |
| 3 | 미리보기 source commit이 방금 병합한 문서 PR보다 오래됐는가 | source commit a623819ff9148f12a1d784eaa9739043618ddf1f가 PR #386 merge commit과 일치한다. 통과. |
| 4 | 구현 원본 PR #385 merge SHA가 다른 구현을 가리키는가 | evidence와 페이지에서 078f5708a5c13d956150b311d392490cc4faa311을 사용한다. 통과. |
| 5 | staging source와 output을 혼동해 작업 폴더를 미리보기로 덮는가 | 고유 build/spinon/c11-4-pr385-tailnet-root/site-pass16 디렉터리와 승인된 SpinonPreview 대상만 사용한다. 통과. |
| 6 | 백업 위치가 기존 pass나 다른 사용자 자료를 덮는가 | 고유 SpinonPreviewBackups/c11-4-pr385-tailnet-sync-2026-10-20/pass16 경로를 사용한다. 통과. |
| 7 | Rspress version, build status 또는 기록 log가 부정확한가 | 이미 설치된 Rspress 2.0.22 CLI로 build exit 0을 확인했다. log의 table row trailing whitespace와 EOF blank line만 정리했고 package/lockfile은 바꾸지 않았다. 통과. |
| 8 | link rewrite가 절대 URL, anchor 또는 기존 외부 참조를 잘못 바꾸는가 | rewrite는 선택 Markdown의 상대 링크만 처리한다. 새 roadmap plan anchor는 GitHub canonical URL을 유지한다. 통과. |
| 9 | Tailnet에서 plan·contract·evidence route가 빌드 산출물에 빠지는가 | 모든 target route와 대표 asset이 선택 build에 존재하고 Agent Browser에서 로드됐다. 통과. |
| 10 | public roadmap의 새 note가 다른 C11.4 항목이나 링크를 덮는가 | 새 PR #385 note를 최신 위치에 표시했고 기존 항목들도 남아 있다. 통과. |
| 11 | C11.4 부모 checkbox 또는 완료 수가 바뀌는가 | Agent Browser에서 C11.4 checked=false를 관찰했다. 체크 항목과 공식 완료 수를 변경하지 않는다. 통과. |
| 12 | C11.4 summary가 제한 slice를 전체 Grid 지원으로 확대하는가 | full grammar, WPT와 C11.4/C11 미완료 경계를 화면과 source 문서에 유지했다. 통과. |
| 13 | 제한 grammar의 축별 repeat 개수를 잘못 표현하는가 | 각 축의 단일 제한 auto-repeat 범위와 unsupported grammar를 source evidence에 맞춘다. 통과. |
| 14 | numeric placement가 expanded repeat count 대신 authored count 기준으로 소개되는가 | 새 roadmap 및 계약 route에서 expanded-count numeric placement라고 기록한다. 통과. |
| 15 | leading grid-auto cycle 방향이나 허용 값이 과장되는가 | leading reverse cycle과 finite non-negative px subset을 문서가 분리해 명시한다. 통과. |
| 16 | auto-fit collapse가 repeat count 감소로 읽히는가 | count는 유지하고 placement 뒤 empty track 및 인접 gutter를 접는다고 적었다. 통과. |
| 17 | Chrome 관찰과 독립 규범 기대가 하나의 oracle로 합쳐지는가 | source evidence의 auto-fit gutter 차이 분리와 normative 기대를 유지한다. 통과. |
| 18 | simulator 실행 수가 기존 106과 추가 18의 합계와 어긋나는가 | route에서 Android/iOS 각 124/124를 확인했다. 통과. |
| 19 | trace chunks와 draw/present가 서로 바뀌는가 | Android 676/iOS 1,284 trace chunks와 각 124/124 draw/present를 구분한다. 통과. |
| 20 | simulator 이름이 실제 기기 또는 물리 성능을 암시하는가 | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator만 기록한다. 통과. |
| 21 | overflow sample이 CSS border stroke까지 검증했다고 과장되는가 | source README가 border stroke 미검증 및 compositor 시점을 별도 제한한다. 통과. |
| 22 | Android 결과가 hardware GPU, iOS 결과가 compositor 이후 pixels로 읽히는가 | AVD software renderer 및 iOS compositor 전 Metal readback 한계를 유지한다. 통과. |
| 23 | 90개 교체 파일의 기존 bytes를 되돌릴 수 없게 덮는가 | 교체 전 backup 90개를 각각 기존 크기와 SHA로 검증한다. 통과. |
| 24 | 서버가 열어 둔 fixture HTML을 truncate하거나 이전 열린 fd와 경합하는가 | 열린 파일 한 개를 기록하고 same-directory temp plus atomic rename을 사용했다. 제공 후 SHA가 일치한다. 통과. |
| 25 | preflight 뒤 source 또는 target이 달라졌는데 조용히 계속 복사하는가 | copy script가 source와 target byte length/SHA를 preflight 값과 파일마다 재검사한다. 통과. |
| 26 | 비선택 파일이나 기존 preview 파일을 삭제하는가 | preflight와 copy 결과 모두 no_deletes=true, 삭제 0이다. 통과. |
| 27 | 새 preview bytes가 서버에 올라오지 않았는데 copy 성공만 보고하는가 | 1,611개 경로를 HTTPS로 받아 모두 HTTP 200과 로컬 SHA를 비교했다. 통과. |
| 28 | 잘못된 화면을 보거나 핵심 plan link가 GitHub에서 끊기는가 | Agent Browser에서 roadmap, STATUS, plan, contract, evidence, docs-sync evidence와 GitHub plan URL을 열었다. 통과. |
| 29 | PR evidence images가 외부 화면에 표시되지 않거나 잘못된 파일을 가리키는가 | Agent Browser screenshot 세 장과 Android/iOS evidence image natural dimensions를 확인했다. 통과. |
| 30 | 사용자 지시 뒤 Agent Browser daemon 또는 tmux/server가 잘못 종료되는가 | 모든 Agent Browser session을 닫고 PID 77078 부재를 확인했다. Tailnet server와 tmux session은 중단하지 않았다. 통과. |

## 최종 판정

30개 독립 관점에서 sync 범위, reversible backup, route/hash 정확도와 종료 절차를 확인했다. 최종 diff에서 preflight dirty 상태 설명과 전체 evidence 연결을 확인했고 evidence manifest 24개 항목의 SHA-256도 검증했다. 병합 blocker는 발견하지 않았다. Rspress build, preview sync 및 Agent Browser 화면 검증을 구분하며 앱/WPT 통과를 주장하지 않는다.

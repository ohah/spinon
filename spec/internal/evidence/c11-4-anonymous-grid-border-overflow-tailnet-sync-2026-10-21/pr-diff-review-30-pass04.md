# C11.4 roadmap·Tailnet 재동기화 PR diff 실패 관점 검토 30개

**검토 대상:** PR #436 병합 상태를 반영한 `spec/STATUS.md`, C11.4 계획 두 문서, `spec/public/roadmap.html`, simulator 실행 README screenshot table, pass04 selected RSPress 빌드·Tailnet 동기화 도구와 Agent Browser/HTTP evidence.

| # | 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | PR #436 병합이 미완료 작업으로 기록되는가? | GitHub merge state와 merge SHA `e0ac9aaf1681aeb56850bf749ec099cf1737d44f`를 대조하고 공식 상태·계획에 기록했다. 통과. |
| 2 | 공식 완료 항목 수가 잘못 증가하는가? | STATUS의 C11 checkbox는 unchecked이며 `72/213`을 유지한다. Agent Browser DOM에서도 count 문구와 checkbox 상태를 확인했다. 통과. |
| 3 | merge SHA가 상태·계획·공개 roadmap에서 다른가? | 세 문서 모두 같은 PR #436 SHA를 사용하고 선택 빌드 route 검증을 통과했다. 통과. |
| 4 | Chrome 156 관측이 CSS Grid normative 기대값으로 승격되는가? | roadmap·plan·status는 Chrome `[16,32,24,20]`과 CRD 계산 `[18,34,20,20]`을 구분한다. 통과. |
| 5 | 새 expected track 또는 row 값이 달라지는가? | 계획·simulator result JSON의 columns `[18,34,20,20]`, rows `[60,60,60]`을 대조했다. 통과. |
| 6 | 세 번째 image overflow를 content-box로 잘못 설명하는가? | image right edge x=74와 root border-box edge x=40의 차이 34 CSS px을 보존한다. 통과. |
| 7 | surface sample이 border 안쪽인데 밖이라고 보고되는가? | 새 image 표본 `(62,128)`은 root-local 오른쪽 border edge보다 22 CSS px 바깥이다. 통과. |
| 8 | source order reverse 화면이 누락되는가? | Android와 iOS forward/reversed 네 screenshot을 README와 공개 roadmap link에서 확인했다. 통과. |
| 9 | screenshot URL이 과거 실행 결과를 가리키는가? | PR #436 hash manifest와 일치하는 최신 final simulator run 경로를 사용했다. 통과. |
| 10 | iOS Metal drawable readback을 compositor 이후 픽셀로 부르는가? | plan, STATUS, evidence README가 compositor 전 한계를 명시한다. 통과. |
| 11 | AVD를 실기기 또는 hardware GPU로 설명하는가? | 새 항목은 Android API 37 ARM64 AVD 및 iOS Simulator 결과로 한정하고 physical device claim을 넣지 않았다. 통과. |
| 12 | 명시적 `overflow:visible` 선언이 지원된다고 오해하는가? | runtime profile 선언 파싱 미지원과 CSS 초기값 visible 사용을 plan/status에 남겼다. 통과. |
| 13 | 고정 fixture가 full CSS Grid/WPT 적합성을 뜻하는가? | WPT, arbitrary CSS와 full C11.4 미검증을 명시했다. 통과. |
| 14 | 실제 Tailnet roadmap에만 변경되고 공식 상태는 낡는가? | `spec/public/roadmap.html`, `spec/STATUS.md`, 세부 plan과 simulator README 모두 업데이트했다. 통과. |
| 15 | 선택 build가 이전 source commit을 사용해 새 변경을 놓치는가? | source `4592bd77...` 및 tree `02c6ae07...`을 selected-root preparation JSON에 고정했다. 통과. |
| 16 | selected build가 작업 copy를 원본처럼 사용해 재현이 불가능한가? | preparation은 branch HEAD 경로와 `git show HEAD:path` bytes를 비교했고 worktree의 tracked plan/spec가 clean한지 확인한다. 통과. |
| 17 | pass04가 pass03 build 경로를 덮어쓰는가? | 선택 root, site root, config와 sync backup에 별도 pass04 경로를 쓴다. 기존 pass03 자료는 그대로다. 통과. |
| 18 | 잘못된 RSPress version이 사용되는가? | 설치된 `@rspress/core 2.0.22`가 build log와 summary에 기록됐다. 통과. |
| 19 | 선택 build를 전체 문서 사이트 통과로 확대하는가? | dead-link/dead-image 전체 검사를 끈 선택 route build이며 README에 범위를 한정했다. 통과. |
| 20 | roadmap/plan/scene asset route 중 하나가 선택 결과에서 빠지는가? | summary script는 roadmap, root/spec STATUS, 새 plan, simulator README와 네 PNG를 필수 route로 확인한다. 통과. |
| 21 | source path mapping이 public page와 evidence를 잘못 복사하는가? | prep script의 `spec/public`, `spec/STATUS`, `plan`, `spec/internal/evidence` mapping이 원본 bytes와 target bytes를 비교한다. 통과. |
| 22 | symlink나 parent traversal이 선택 build에 들어오는가? | source·target은 plain tree로 검사하며 symlink, 절대경로와 `..` 경로를 거부한다. 통과. |
| 23 | Tailnet 동기화가 이전 파일을 삭제하는가? | pass04 preflight/copy/verify는 삭제 0개를 명시하고 source path 집합이 동일할 때만 복사한다. 통과. |
| 24 | 원본 target 파일이 백업 없이 교체되는가? | 교체 전 각 179개 이전 파일을 전용 pass04 backup에 저장하고 SHA/bytes를 확인했다. 통과. |
| 25 | 백업은 있으나 실제 복사 파일이 손상되는가? | local verification에서 target 3,164/3,164와 backup 179/179의 byte/hash가 모두 일치했다. 통과. |
| 26 | active client가 열린 파일을 원자 rename해 중단되는가? | sync tool은 rename 직전 `lsof`로 검사하고 유일한 127.0.0.1:8768 idle preview listener만 열었을 때 진행한다. 통과. |
| 27 | preview process가 다른 서비스로 잘못 식별되는가? | PID 1191의 command line은 `tailnet-preview-server.mjs`, listener는 `127.0.0.1:8768` 하나다. 통과. |
| 28 | 공개 Tailnet origin이 실제 새 content를 제공하지 않는가? | 원격 확인은 HTTP 200 3,164/3,164, source SHA 일치 3,164/3,164, 실패 0건이다. 통과. |
| 29 | 정적 HTML이 맞아도 Agent Browser에서 페이지나 image가 깨지는가? | roadmap·STATUS·plan DOM과 simulator README를 열었고 네 image가 `complete=true`, Android 1080×2400, iOS 1206×2622다. 통과. |
| 30 | 검증 후 browser/server/tmux나 무관한 checkout 상태가 남아 영향을 주는가? | Agent Browser/HeadlessChrome 최종 process 목록은 비었고 preview PID 1191과 tmux는 유지했다. staging은 PR 관련 경로만 포함한다. 통과. |

추가 blocker는 발견되지 않았다. pass04 실행은 PR #436 병합 뒤의 72/213 상태를 Tailnet roadmap과 상세 문서에 반영했고, 변경 대상/백업/원격 파일 hash와 브라우저 화면을 각각 확인했다.

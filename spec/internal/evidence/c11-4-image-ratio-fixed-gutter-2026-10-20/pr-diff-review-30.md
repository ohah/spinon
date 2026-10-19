# C11.4 fixed-gutter PR diff 적대 검토 · 30개 실패 관점

**대상:** `origin/main` 기준 C11.4 구현 후보, 앱 fixture 연결, 계획·계약·상태 문서와 실행 근거. 앞선 계획 20개 및 구현 20개 표와 다른 PR 통합·배포·증거 경계를 검토했다. GitHub PR은 아직 생성 전이므로 원격 required checks와 merge state는 별도 관문이다.

| # | 실패 관점 | 확인 결과 | 판정 |
| ---: | --- | --- | --- |
| 1 | C11.4 diff가 다른 작업 브랜치의 커밋을 끌어오는가 | 후보 branch base는 `origin/main` `54a6523deaf49e59c22b56207408a2c4bf1d1267`이다. | PASS |
| 2 | tmux가 사용하는 C11.3 checkout에 C11.4 변경이 섞이는가 | C11.4 수정은 전용 worktree에서 진행하며 원래 checkout은 `feat/c11-3-template-repeat-named-lines-areas`로 남아 있다. | PASS |
| 3 | 이전 Simulator 진단 run을 성공 근거로 함께 게시하는가 | PR에는 성공한 `c114-fg-20261019T153833Z`만 넣었다. 앞선 진단 run은 별도 임시 백업에 보존했고 manifest에 넣지 않았다. | PASS |
| 4 | C11.4 전체 항목을 작은 slice만으로 완료 표시하는가 | `spec/STATUS.md`의 C11.4 체크와 완료 수는 바뀌지 않았다. | PASS |
| 5 | 계획·README가 공개 CSS 지원을 암시하는가 | 제한 구현 후보이며 WPT·전체 Grid 미지원임을 각 문서에 명시한다. | PASS |
| 6 | 내부 계약의 숫자 버전을 근거 없이 올리는가 | 기존 0059 계약에 후보 설명만 추가했고 버전은 올리지 않았다. | PASS |
| 7 | 규범 근거 링크가 검색 결과나 비공식 요약으로 향하는가 | 계획·계약·근거 README가 W3C CSS Grid Level 1 §6.6 원문을 직접 가리킨다. | PASS |
| 8 | Chrome 관찰값을 규범 기대값이나 구현 성공으로 혼동하는가 | 사전 Chrome 155 frame 관찰과 §6.6 기반 Rust 기여값을 분리하고 둘의 차이를 보존한다. | PASS |
| 9 | 구현 검토 20개를 PR 검토 30개로 재사용하는가 | 이 표는 계획·구현 표와 별도로 PR 통합·artifact·simulator·문서 경계를 확인한다. | PASS |
| 10 | 구현·계획·PR 검토 중 하나가 문서 링크만 있고 파일은 없는가 | 계획 수정 20개, 구현 20개, 이 PR diff 30개 보고서 파일을 각각 둔다. | PASS |
| 11 | README의 test count가 현재 보고된 실행 결과와 다른가 | layout 181/181, focused 8/8, runtime 2/2, FFI 1/1, Clippy를 기록한다. | PASS |
| 12 | 컴파일 성공을 전체 문서 사이트 성공으로 확대 해석하는가 | 전체 RSPress build의 기존 dead-link 실패를 별도 한계로 남겼고 성공으로 기록하지 않는다. | PASS |
| 13 | evidence 파일 변경 뒤 무결성 목록이 낡거나 빠지는가 | evidence 폴더의 `SHA256SUMS`를 생성해 README·검토·빌드 로그·스크립트·성공 run을 고정한다. | PASS |
| 14 | fixture가 runtime 제품 모듈에 무조건 포함되는가 | C11.4 adapter는 `fixtures` 경계와 `c04-runtime-gpu-fixture-test` feature 아래 있다. | PASS |
| 15 | Android/iOS native define와 Cargo feature가 엇갈리는가 | 각 플랫폼의 fixture build option은 같은 C11.4 FFI test path를 켜며 기본 feature는 꺼져 있다. | PASS |
| 16 | default static/Rust artifact에 신규 fixed-gutter scene 문자열이 포함되는가 | 기본 `.a`와 `.rlib` 문자열 scan에서 두 scene ID가 없음을 확인했다. | PASS |
| 17 | FFI에서 임의 scene 이름이 실행 가능한가 | scene allowlist와 미등록 입력의 오류 test를 확인했다. | PASS |
| 18 | Android run ID와 scene ID가 서로 다른 실행의 로그를 합치는가 | runtime assertion과 PixelCopy summary가 동일 scene/run ID를 요구한다. | PASS |
| 19 | iOS run ID와 scene ID가 서로 다른 실행의 로그를 합치는가 | Metal readback argument·runtime trace가 같은 scene/run ID로 연결된다. | PASS |
| 20 | 신규 Rust 테스트 파일이 프로젝트의 권장 크기를 크게 넘는가 | 처음 599줄이던 matrix를 공통 helper 272줄과 case 하위 모듈 331줄로 나눴다. rustfmt와 focused 8/8이 통과했다. | FIXED |
| 21 | overflow sample이 grid 바깥 좌표가 아닌 item 안쪽에서 선택되는가 | item origin과 각 축의 grid 폭·높이를 대조해 오른쪽·아래 sample이 grid 경계 밖임을 확인했다. | PASS |
| 22 | PNG 화면만 보고 exact pixel 결과라고 주장하는가 | Android PixelCopy와 iOS Metal drawable readback를 exact color 근거로 사용하고 PNG는 시각 자료로 한정한다. | PASS |
| 23 | 색 검사가 alpha 또는 border/item 식별을 생략하는가 | scene별 녹색 border와 빨간 item sample은 RGBA 4채널 기대값을 비교한다. | PASS |
| 24 | screenshot이 다른 scene/device의 결과를 가리키는가 | 네 PNG는 run별 Android/iOS, row/column 경로에 저장되고 README에서 각각 연결한다. | PASS |
| 25 | Simulator 기록에 target 식별 정보가 없어 재현 대상이 모호한가 | 성공 run에 AVD/API/ABI와 iOS runtime·UDID·model identifier를 남긴다. | PASS |
| 26 | 재현 스크립트가 API 37의 임의 AVD를 받아들이는가 | 검토 중 발견했다. 정확한 `spinon_api37_1_compat16k` AVD 검사로 고쳤고 실제 AVD 이름으로 preflight를 다시 확인했다. | FIXED |
| 27 | 재현 스크립트가 임의 iOS runtime 또는 기종을 허용하는가 | iPhone 17 Pro UDID, iOS 26.2 booted list, model identifier `iPhone18,1`을 모두 확인하도록 보강하고 현재 Simulator에서 검증했다. | FIXED |
| 28 | 실행 중인 사용자 앱을 종료하거나 tmux pane을 바꾸는가 | 스크립트는 대상 앱이 이미 실행 중이면 중단하고, 자신이 시작한 앱만 종료한다. tmux 조작은 하지 않는다. | PASS |
| 29 | 에뮬레이터 검증이 실기기 검증으로 보고되는가 | Android의 emulator 속성과 iOS Simulator UDID를 확인하며 README에 실기기 미검증을 명시한다. | PASS |
| 30 | GitHub label·required check·merge 방식 없이 완료 처리하는가 | 기존 `bug`·`area: layout`을 붙일 예정이며 PR 생성 뒤 live required checks 확인 후 rebase merge만 진행한다. 아직 생성 전이므로 merge 완료 주장은 없다. | PASS |

## 발견 및 재확인

- 재현 스크립트가 실제 근거와 다른 Simulator 대상을 허용하던 점, 신규 테스트 파일이 599줄로 프로젝트 권장 범위를 넘던 점을 발견했다. 정확한 AVD·iOS runtime·iPhone 모델 guard를 추가하고 테스트 helper와 입력 matrix를 272줄·331줄 하위 모듈로 나눴다.
- 수정 뒤 `bash -n`, 현재 Android AVD/API/ABI와 iOS runtime/UDID/model preflight, rustfmt, focused Rust 8/8, `git diff --check`를 통과시켰다. 앱을 다시 실행하지 않았다.
- 다른 blocking diff 결함은 찾지 못했다. 원격 PR required checks, merge state, Tailnet preview는 이 보고서 작성 시점 이후 확인할 항목이다.

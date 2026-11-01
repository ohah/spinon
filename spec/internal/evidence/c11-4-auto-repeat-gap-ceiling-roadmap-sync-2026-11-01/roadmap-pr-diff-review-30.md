# C11.4 positive-gap auto-repeat ceiling 로드맵 PR diff 적대 검토 30개

검토 대상은 `spec/public/roadmap.html`의 PR #985 진행 기록과 Tailnet 동기화·Agent Browser 캡처 근거다. 공식 구현 상태는 `spec/STATUS.md`, 입력·기대값은 계획과 실행 README 및 Rust test를 기준으로 대조했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR 링크가 다른 변경을 가리키는가 | 첫 C11.4 카드의 링크는 #985를 가리킨다. 통과. |
| 2 | 계획 링크의 slug가 실제 계획과 다른가 | `c11-4-auto-repeat-gap-ceiling` 경로가 선택 publication에 존재하고 HTTPS 200/hash 검증을 통과했다. 통과. |
| 3 | runtime README 링크가 잘못되었는가 | `c11-4-auto-repeat-gap-ceiling-runtime-2026-11-01/README.html`을 Tailnet critical route에서 200/hash 검증했다. 통과. |
| 4 | 상태 문서 링크가 누락되거나 다른 경로인가 | 공개 상태 anchor가 `STATUS.html#css-구현-체크리스트`로 연결된다. 통과. |
| 5 | C11.4 또는 C11 상위 항목을 완료로 표시하는가 | 카드가 둘 다 미완료라고 명시하며 checkbox를 변경하지 않는다. 통과. |
| 6 | 공식 완료 수가 부분 slice로 증가하는가 | 공식 `72/213`을 유지한다고 명시한다. 통과. |
| 7 | test-only 결과가 공개 CSS 지원처럼 읽히는가 | 공개 CSS admission 미완료를 밝힌다. 통과. |
| 8 | WPT 적합성을 주장하는가 | WPT 미완료로 한정한다. 통과. |
| 9 | 상한이 CSS 명세의 보편적인 제한처럼 오해되는가 | 고정 vendored Taffy의 internal 10,000-track cap이라고 적는다. 통과. |
| 10 | gap 크기가 일반화되는가 | 시험한 값은 `1px`이라고 명시한다. 통과. |
| 11 | repeat pattern 범위가 확장되는가 | 두-track pattern으로 한정해 설명한다. 통과. |
| 12 | exact-fit과 cap 입력이 혼동되는가 | `25,001px` exact-fit과 `25,006px` cap basis를 따로 적는다. 통과. |
| 13 | 두 축 결과가 한 축만 시험한 것으로 보이는가 | Columns/Rows 모두 네 scene의 일부라고 명시한다. 통과. |
| 14 | 4/4가 행렬의 정의 없이 제시되는가 | 두 basis × 두 축으로 4개 실행임을 주변 설명 및 계획 링크에서 확인한다. 통과. |
| 15 | track 기대 수가 잘못 제시되는가 | 10,000-track sequence를 실행 README의 기대값과 대조했다. 통과. |
| 16 | named-line 결과를 CSS 일반 보장으로 읽는가 | 고정 trace의 item frame 관찰로 설명하고 일반 admission은 주장하지 않는다. 통과. |
| 17 | Android/iOS 앱 화면이 있는 것처럼 서술하는가 | 앱 surface가 없는 pure Rust trace이며 simulator screenshot 대상이 아니라고 쓴다. 통과. |
| 18 | Agent Browser 화면을 제품 동작 증거로 오해하는가 | 동기화 요약에서 문서 캡처로 한정하고 앱/pixel 근거를 부인한다. 통과. |
| 19 | 데스크톱 캡처가 다른 viewport 크기로 설명되는가 | 파일을 1440 × 1000로 캡처했고 요약도 같은 크기를 기록한다. 통과. |
| 20 | 모바일 전체 페이지 overflow를 카드 너비만으로 추정하는가 | document/body `scrollWidth`와 `clientWidth`를 390px로 직접 기록하고 카드 너비도 362px로 별도 기록한다. 통과. |
| 21 | 캡처에서 현재 카드나 완료 수가 보이지 않는가 | 화면 및 DOM에서 첫 C11.4 카드, PR #985, 공식 `72/213`을 확인했다. 통과. |
| 22 | Tailnet 데이터가 sync 전 publication일 수 있는가 | source HEAD/tree와 tracked diff hash를 publication/sync manifest와 연결했다. 통과. |
| 23 | 동기화가 일부 route만 검증됐는데 전체라고 쓰는가 | 1,029/1,029 selected routes의 hash 검증 수를 명시한다. 통과. |
| 24 | 핵심 문서가 HTTP 상태만 확인하고 content mismatch를 놓치는가 | `roadmap.html`, `STATUS.html`, 계획/실행 문서에서 200과 hash 일치를 모두 확인했다. 통과. |
| 25 | 공개 후 기존 파일 backup이 빠지는가 | 59개 교체 파일의 backup 59/59 hash 검증을 기록한다. 통과. |
| 26 | 동기화 도중 파일 handle 경합이 있었는가 | 사전 점검의 open handle 0개를 기록한다. 통과. |
| 27 | sync가 기존 route를 삭제하거나 서버를 재시작하는가 | 삭제 0, 서버 재시작 없음으로 기록한다. 통과. |
| 28 | 문서 build 결과를 full-site dead-link 검사로 부풀리는가 | full docs dead-link 검사가 비활성화되었다고 명시한다. 통과. |
| 29 | Agent Browser를 남겨둔 채 종료했다고 보고하는가 | `close --all` 뒤 browser/headless Chromium process 부재를 확인했다고 기록하고 Tailnet server는 유지한다. 통과. |
| 30 | PR에 문서 변경 검토 근거 또는 screenshot 링크가 빠지는가 | 이 30개 리뷰와 데스크톱·모바일 캡처를 PR의 문서 검토 근거로 연결한다. 통과. |

## 판정

30개 관점에서 상태 과장, 상위 완료 승격, 경로/viewport 착오, 배포 hash·backup·프로세스 경계를 확인했다. 발견한 차단 결함은 없다. 이 검토는 Rust 구현/runtime 20개 검토 및 계획 20개 검토와 별도다.

# C11 계획 PR #864 Tailnet 동기화 PR diff 적대 검토 30개

**검토 범위:** merged main `cf217d8bab6ea4b5f27e43f4be48d62c8b8272ca`의 변경 문서만 렌더링한 pass03, Tailnet 사전 비교·복사·backup·HTTPS receipts, Agent Browser 화면과 재현 스크립트.

| # | 독립 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | 병합 전 브랜치의 문서를 게시할 수 있음 | PASS — selector와 stager가 `origin/main` 및 `cf217d8` source HEAD를 고정한다. |
| 2 | dirty working tree 내용이 빌드에 섞일 수 있음 | PASS — pass02 stager는 source를 고정된 HEAD의 `git show`로 읽는다. |
| 3 | PR #864 변경 범위가 아닌 roadmap/status를 잘못 갱신할 수 있음 | PASS — PR #864의 변경은 상위 계획, 상세 계획, diff 검토 3개뿐이며 README가 공개 roadmap/status를 다시 게시하지 않았다고 기록한다. |
| 4 | 모든 내부 실행 로그와 화면을 미리보기로 재복사할 수 있음 | PASS — 738-file 후보는 복사하지 않고 변경 페이지 6 route와 참조 자원만 pass03에 포함했다. |
| 5 | 후보 publication을 줄인 뒤에도 route 수가 예상보다 커질 수 있음 | PASS — pass03 manifest와 preflight가 21 route, 4,723,849 bytes를 고정한다. |
| 6 | 이전 stage 실패의 원인을 감추거나 실패 파일을 게시할 수 있음 | PASS — 없는 경로를 지정한 pass01 실패 로그를 보존했고 pass02 source 목록에서 제거했다. |
| 7 | 현재 문서 사이트 전체를 검증했다고 과장할 수 있음 | PASS — README와 manifest는 dead-link/dead-image 검사 및 전체 docs 검증을 완료하지 않았다고 적는다. |
| 8 | 바뀐 페이지가 빠진 채 배포될 수 있음 | PASS — pass03 page allowlist에 상위 계획, C11.4 계획, PR #864 diff 검토의 두 route alias씩 들어 있다. |
| 9 | 두 계획 문서가 root alias만 갱신되고 `/spec/` alias는 stale일 수 있음 | PASS — 여섯 페이지를 명시적으로 선택하고 각각 pass03 preflight에 포함했다. |
| 10 | 새 PR diff 검토 페이지가 빠지거나 이전 문서와 충돌할 수 있음 | PASS — `c11-roadmap-sync-pr864.../pr-diff-review-30.html`과 `/spec/` alias가 신규 route다. |
| 11 | HTML이 참조하는 새 route JS/CSS가 누락될 수 있음 | PASS — selector가 각 page의 `/spinon/` 자원을 추출해 21-route manifest에 넣었다. |
| 12 | HTML 자원 경로를 따라 publication 바깥 파일을 읽을 수 있음 | PASS — decoded route를 복사 전에 `safeRoute`로 검사하며 `..`, 빈 segment, 역슬래시를 거부한다. |
| 13 | symlink를 통해 선택 build 바깥으로 나갈 수 있음 | PASS — selector는 선택된 각 source를 `lstat`해 일반 파일만 복사한다. |
| 14 | asset 링크가 외부 URL이나 페이지 링크를 로컬 파일로 취급할 수 있음 | PASS — `/spinon/` 아래의 존재하는 비-HTML 자원만 복사하고 HTML navigation route는 제외한다. |
| 15 | 검색 결과에 업데이트된 계획 내용이 나타나지 않을 수 있음 | PASS — RSPress 검색 색인 `static/search_index.ko.ed3e171c.json`을 명시적으로 포함한다. |
| 16 | PR #861 정보를 C11 전체 완료로 표시할 수 있음 | PASS — 두 계획 화면과 README가 `72/213`, C11/C11.4 미완료 및 남은 범위를 유지한다. |
| 17 | `72/213`을 C11만의 완료 건수로 오해할 수 있음 | PASS — 문서에서 공식 전체 진행 집계로 표현하고 C11 완료 수치로 쓰지 않는다. |
| 18 | PR #850의 scene/sample 수치가 변경될 수 있음 | PASS — 상위 계획의 Agent Browser 화면에서 42/42 scene 및 129/129 sample 문구를 확인했다. |
| 19 | nowrap geometry에서 root와 item/track을 혼동할 수 있음 | PASS — 상세 계획은 root `20×10px`, track `50px`, item `50×10px`와 30px overflow를 구분한다. |
| 20 | Android AVD 결과를 실기기 또는 hardware GPU 결과로 확대할 수 있음 | PASS — 동기화한 계획은 API 37 ARM64 AVD를 명시하고 실기기 증거라고 하지 않는다. |
| 21 | iOS drawable 확인을 compositor 이후 pixel 검증으로 확대할 수 있음 | PASS — 계획은 iOS Simulator 범위와 drawable sample 경계를 구분하며 해당 PR 화면은 glyph pixel 검증을 주장하지 않는다. |
| 22 | preflight 전에 Tailnet 파일을 덮어쓸 수 있음 | PASS — pass03 copy는 pass03 preflight JSON을 필수 입력으로 받는다. |
| 23 | 교체 중 기존 미리보기 파일을 잃을 수 있음 | PASS — 교체 4개, 795,225 bytes가 task-specific backup 경로에 보관됐다. |
| 24 | source가 preflight 뒤 바뀌어 검증되지 않은 내용이 복사될 수 있음 | PASS — copy는 source byte 수와 SHA-256을 preflight 예상값과 비교한다. |
| 25 | 대상 파일이 preflight 뒤 바뀌어 덮어쓸 수 있음 | PASS — copy는 기존 대상의 존재 상태, byte 수, SHA-256을 재검사한다. |
| 26 | 선택한 파일 이외의 대상 파일을 삭제할 수 있음 | PASS — preflight/copy receipt는 `no_deletes: true`이며 삭제 단계가 없다. |
| 27 | 다른 preview server 또는 잘못된 포트로 게시할 수 있음 | PASS — 실행 당시 PID `28040`이 `127.0.0.1:8768` listener였고 `tailnet-preview-server.mjs`로 식별됐다. 복사 script는 단일 listener와 그 process 명령을 검사한다. |
| 28 | HTTPS가 stale 또는 다른 파일을 반환할 수 있음 | PASS — `served-verification-pass03.json`은 21/21 HTTP 200 및 byte/hash 일치를 기록한다. |
| 29 | 브라우저 snapshot이 실제 페이지가 아닌 다른 route를 확인할 수 있음 | PASS — 세 Agent Browser receipt에 각 Tailnet URL이 기록되고 두 계획의 #850/#861/72/213 및 geometry 문구를 읽었다. |
| 30 | 사용자가 요청한 브라우저 확인 뒤 Agent Browser가 실행 상태로 남을 수 있음 | PASS — Agent Browser 창을 닫았고 후속 process scan에서 Agent Browser·전용 Chrome이 없었다. terminal-browser는 사용하지 않았다. |

이 검토는 변경된 Tailnet 문서 route와 evidence 경계만 판정한다. 저장소 전체 링크, 전체 WPT, 일반 CSS 지원, C11/C11.4 완료, 실기기 검증, GitHub Pages 배포는 증명하지 않는다.

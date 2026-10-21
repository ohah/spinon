# C11.4 cyclic `calc()` PR #480 Tailnet roadmap 동기화 · pass41

PR [#480](https://github.com/ohah/spinon/pull/480)의 four-scene fixed fixture 결과를 공식 상태와 공개 HTML roadmap에 반영한 PR [#481](https://github.com/ohah/spinon/pull/481)이 Rebase and merge된 뒤, 그 `main` source tree로 Tailnet 미리보기를 갱신했다. PR #480 merge SHA는 `e8c7eb06c66fc7424002eaca76af081ecd72a24f`, PR #481 merge SHA는 `0791d3105268fe343426e04c7cfdda27e5e7bf3b`다. C11.4는 미완료이고 공식 집계는 `72/213`이다.

## 선택 source와 build

- `origin/main` head `0791d3105268fe343426e04c7cfdda27e5e7bf3b`, tree `25f28e3eb3e61f1cdbf182e4dda6d33ea436c6f4`를 기준으로 선택 root를 만들었다. 선택에 쓴 workspace HEAD는 rebase 전 docs branch commit `edc12522316210e085df65504ae34d2836141532`였지만 Git tree가 merge된 `origin/main`과 정확히 같다. [선택 root 복사·source hash](./selected-root-preparation-pass41.json).
- 이전 pass40 selected root를 보존 복사하고 현재 status, public roadmap, C11.4 계획·검토·fixture 및 네 관련 evidence directory만 갱신했다. 기존 root/site를 덮어쓰거나 지우지 않았다.
- RSPress `2.0.22`로 새 site root를 빌드했다. 결과는 `3,027` files, `128,044,998` bytes, tree SHA-256 `2756922db15d74032c1413b81b17080e3cc478e0f74af7b13cafd597541bfaf0`이다. 저장한 RSPress 로그에서는 표 정렬용 후행 공백만 제거했다. [build log](./selected-build-pass41.log) · [build summary와 required routes](./selected-build-summary-pass41.json).
- roadmap, status, 집중 계획, Chrome 사전 비교, simulator 결과, implementation review, PR diff review, Android/iOS two-axis screenshots와 HTML fixture 등 필수 route `15/15`가 build output에 있다.

첫 selected-root 준비 실행은 스크립트의 clean-tree assertion이 Node `null`을 `undefined`와 비교해 중단됐다. 이 실행은 preview에 쓰지 않았고, assertion을 `git status --porcelain --untracked-files=no`로 고친 뒤 새 pass41 root에서 source-tree 동일성을 다시 확인했다.

## Preview 복사와 검증

- 대상은 `/Users/yoonhb/Library/Application Support/SpinonPreview`, Tailnet URL [`/spinon/`](https://macstudio.tailed42f2.ts.net/spinon/)이다. listener는 기존 `tailnet-preview-server.mjs` PID `1191`, `127.0.0.1:8768`이었고 동기화 뒤에도 그대로 유지했다.
- Preflight는 file `3,027`개를 create `298`, replace `165`, same `2,564`로 분류했다. 교체 전 원본 `31,832,668` bytes를 별도 pass41 backup root에 보존한다. 삭제는 없다. [Preflight](./preflight-pass41.json).
- Copy 결과에서 target `3,027/3,027`, backup `165/165`가 각각 source/previous SHA-256과 일치한다. 교체 대상 파일을 여는 preview 외 process는 없었고, server는 단일 listen socket만 보유했다. [Copy·backup 기록](./copy-and-backup-pass41.json) · [로컬 검증](./local-verification-pass41.json).
- Tailnet URL에서 `3,027/3,027` 파일 모두 HTTP `200`, `3,027/3,027` hash exact, failure `0`이다. [remote 결과](./served-verification-pass41.json) · [진행 요약](./remote-verification-pass41.log).

## Agent Browser 확인과 종료

- 공개 [roadmap](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)에 PR #480 merge 기록, Android/iOS 결과와 `72/213`이 보인다. [DOM 결과](./roadmap-dom-pass41.json) · [화면](./tailnet-roadmap-agent-browser-pass41.png).
- 공식 [status](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html)에서 C11.4 미완료, PR #480, merge SHA 및 `72/213`을 확인했다. [DOM 결과](./status-dom-pass41.json) · [C11.4 entry 화면](./tailnet-status-c11-4-agent-browser-pass41.png).
- [계획](https://macstudio.tailed42f2.ts.net/spinon/plan/c11-4-cyclic-calc-track-sizing.html)의 네 scene과 root/Columns/Rows/item 결과가 표시된다. [DOM 결과](./plan-dom-pass41.json) · [화면](./tailnet-plan-agent-browser-pass41.png).
- [Simulator evidence](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c11-4-cyclic-calc-track-sizing-2026-10-22/README.html)에 네 scene 행과 8개 screenshot 링크가 보인다. Android two-axis PNG는 `1080×2400`, iOS PNG는 `1206×2622`이며 둘 다 `complete=true`로 열렸다. [Evidence DOM](./evidence-dom-pass41.json) · [Android image DOM](./android-image-dom-pass41.json) · [iOS image DOM](./ios-image-dom-pass41.json) · [Evidence 화면](./tailnet-evidence-agent-browser-pass41.png) · [Android 화면](./tailnet-android-image-agent-browser-pass41.png) · [iOS 화면](./tailnet-ios-image-agent-browser-pass41.png).
- 병합된 [PR #480](https://github.com/ohah/spinon/pull/480) 페이지에서 `Merged`와 두 body image `complete=true`를 확인했다. PR 본문의 Android/iOS image도 각각 `1080×2400`, `1206×2622`이다. [PR DOM](./pr-dom-after-merge-pass41.json) · [PR 화면](./pr-agent-browser-after-merge-pass41.png).
- Agent Browser 전용 `spinon-tailnet-pass41` session을 닫았다. 종료 직후 Agent Browser·Chrome process scan은 비어 있었고 Tailnet server PID `1191`은 보존했다. [종료 상태](./browser-close-pass41.json).

## 범위

이 결과는 사설 Tailnet 개발 미리보기와 build output의 byte/hash 검증이다. production website 배포가 아니며 C11.4·일반 CSS 지원·WPT·Android 실기기 또는 iOS compositor 이후 화면의 완료 근거로 쓰지 않는다. PR #481은 merge 후 30 관점으로 회고 검토했으며, 이를 merge 전 검토 수행으로 소급 표기하지 않는다. [PR #481 회고적 diff 검토 30개](./pr-481-retrospective-diff-review-30.md).

이 Tailnet 동기화 증거 PR 자체의 사전 적대적 검토는 [pass41 diff 검토 30개](./tailnet-sync-pr-diff-review-30-pass41.md)에 기록했다.

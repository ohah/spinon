# C11.1 PR #1015 병합 후 상태·로드맵 근거

PR [#1015](https://github.com/ohah/spinon/pull/1015)은 2026-11-02T00:39:29Z Rebase and merge됐고 merge SHA는 `015ff0e9ac959a269b60728a1ac3a6ab962791ea`다. 구현 PR check rollup은 비어 있었으며 CI 통과로 기록하지 않는다.

이 문서 PR은 공식 상태 ledger, 내부 계약 0059, C11 계획과 공개 roadmap source를 merge 결과에 맞췄다. C11 checkbox는 미완료로 두고 C11.1 전체도 미완료로 유지하며 전체 공식 완료 수 `72/213`도 바꾸지 않는다. Root HTML direct finite non-negative px/fr는 bounded implementation slice로만 설명하며, 일반 CSS admission·intrinsic sizing·WPT 또는 전체 C11 완료로 확대하지 않는다.

Agent Browser에서 PR #1016 병합 전 로컬 공개 roadmap source를 열고 PR #1015 카드와 merge SHA, `72/213`, C11 미완료와 C11.1 전체 미완료 표기를 확인했다. [화면](../c11-root-html-grid-fr-track-runtime-2026-11-02/roadmap-source-agent-browser.png) · [DOM observation](../c11-root-html-grid-fr-track-runtime-2026-11-02/roadmap-source-agent-browser.json). 이 local-source capture만으로 Tailnet 게재를 주장하지 않는다.

이 문서 변경의 별도 [PR diff 실패 관점 검토 30개](./pr-diff-review-30.md)를 포함한다.

화면 캡처는 1440×1000이며 observation의 `scrollWidth`는 1440이다. PNG SHA-256은 `2305b34937f92c0d8e3c33548550a4b499ae1eb489af9bb7fd69a2c475259e84`다. 확인 뒤 `agent-browser close --all`을 실행했고 브라우저 프로세스가 남지 않았음을 확인했다.

## PR #1016 병합 후 Tailnet 동기화

문서 PR [#1016](https://github.com/ohah/spinon/pull/1016)은 2026-11-02T00:43:58Z Rebase and merge됐고 merge SHA는 `3c009d316f270fbfbca55fc8813a050d1921a000`이다. 병합된 `origin/main`에서 선택 route를 다시 렌더링해 기존 Tailnet 미리보기 서버 PID `28040`에 게시했다. 서버 프로세스는 교체하거나 종료하지 않았다.

선택한 112 route, 13,213,819 bytes 중 새 route 84개, 교체 15개, 동일 13개였다. 교체한 기존 파일 15개를 사용자 지원 백업 경로에 이전 SHA-256과 대조해 보존했다. 게시 전 경로는 91,454개였고 게시 후 91,538개이며 기준 경로 삭제는 0개다. 전체 4.1 GB 프리뷰 미러를 다시 만들거나 변경하지 않았다. 선택 route 묶음 SHA-256은 `cede280fa5673be10cc00965b9d1b9de2e3c44fa7e824ff18fbd0a6cfcea0388`이다.

로드맵, 공식 상태 HTML/Markdown, C11 계획, 내부 계약, Android/iOS 실행 README와 화면을 포함한 11개 필수 route를 localhost와 Tailnet HTTPS에서 요청했다. 22개 응답이 모두 HTTP 200이고, 원본·로컬·Tailnet의 크기와 SHA-256이 일치한다. [선택 route manifest](./route-publication-manifest.json) · [게시 사전 점검](./tailnet-preflight.json) · [백업·복사 결과](./tailnet-copy.json) · [HTTP 및 경로 검증](./tailnet-served-verification.json).

Agent Browser에서 실제 Tailnet 로드맵의 PR #1015 카드, merge SHA와 `72/213`을 확인했다. 데스크톱 1280×633에서 `scrollWidth=1280`, 모바일 390×844에서 `scrollWidth=390`이다. [데스크톱 캡처](./tailnet-roadmap-desktop-agent-browser-pass02.png) · [모바일 캡처](./tailnet-roadmap-mobile-agent-browser-pass02.png) · [브라우저 검증 기록](./agent-browser-verification.json). 캡처 후 `agent-browser close --all`을 실행했고 `agent-browser-darwin-arm64`와 `chrome-headless-shell` 프로세스는 남지 않았다.

새 Tailnet 게시 변경은 별도 [실패 관점 검토 30개](./tailnet-sync-review-30.md)에 기록했다. HTTP 상태와 선택 route hash는 실제 게시를 검증하지만, 이 검사는 기존 전체 미러의 모든 91,454개 파일 내용을 재해시한 검사는 아니다. C11 전체 및 공식 완료 수 `72/213`에는 변화가 없다.

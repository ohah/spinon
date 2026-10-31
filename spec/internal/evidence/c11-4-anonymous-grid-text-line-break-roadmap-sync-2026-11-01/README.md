# C11.4 `line-break` 병합 후 상태·Tailnet 로드맵 동기화

## 범위

구현 PR [#945](https://github.com/ohah/spinon/pull/945)은 2026-10-31T21:11:12Z Rebase and merge됐다. merge SHA는 `9eb8cea570fce5a0eb2da423ef37eac2e4ed3c1d`다. 이 문서 변경은 merged slice의 상태를 `spec/STATUS.md`, C11/C11.4 계획과 `spec/public/roadmap.html`에 반영한다. C11 및 C11.4 체크는 미완료로 두고 공식 `72/213`을 유지한다.

## 선택 문서 빌드

RSPress `2.0.22`로 roadmap에 연결된 계획, 내부 계약, 사전 비교와 실행 근거만 별도 root에서 빌드했다. 결과는 20개 생성 HTML, 110개 선택 게시 경로, 11,058,719 bytes다. 전체 문서 dead-link 검사는 실행하지 않았다. 사용한 선택 root·route·SHA-256은 [selected-root manifest](selected-root.json), [선택 RSPress 로그](selected-rspress-build.log), [publication manifest](selected-publication.json)에 있다.

## Tailnet 동기화와 응답 확인

Tailnet 미리보기 서버는 PID `28040`, `127.0.0.1:8768`, root `/Users/yoonhb/Library/Application Support/SpinonPreview`에서 실행 중이었다. 기존 원격 roadmap도 HTTP 200이었고 로컬 사본과 SHA-256이 일치했다. 사전 점검은 대상 110개 중 신규 86, 교체 9, 동일 15를 기록했다. 열려 있던 22개 경로는 모두 변경 전후 hash가 같았고, 교체 경로 9개의 이전 bytes는 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/c11-4-anonymous-grid-text-line-break-roadmap-sync-2026-11-01`에 별도로 보존했다. 파일 삭제는 없었다.

동기화 후 전체 110 route의 로컬 SHA-256이 publication manifest와 일치했다. 로드맵, 공식 상태, C11/C11.4 계획, 내부 계약 0059, Chrome 사전 비교와 Android/iOS 실행 근거 페이지 및 두 화면 PNG는 로컬 preview와 `https://macstudio.tailed42f2.ts.net/spinon/`에서 HTTP 200을 반환했고 served hashes가 publication manifest와 일치했다. 결과는 [preflight](preflight.json), [copy 및 backup 기록](copy-and-backup.json), [served verification](served-verification.json)에 있다.

## Agent Browser 확인

Agent Browser `0.31.1`에서 공개 roadmap을 확인했다. Desktop 1280×900에서 문서 너비는 1280px이고 대상 카드 폭은 908px이었다. Mobile 390×844에서 문서 너비는 390px이며 카드 폭 362px, 좌우 여백은 각각 14px이라 가로 넘침이 없었다. 계획, 내부 계약, Chrome 사전 비교와 runtime evidence 네 route의 제목과 내용을 직접 열어 확인했다. [Desktop 화면](roadmap-desktop-agent-browser.png) · [Mobile 화면](roadmap-mobile-agent-browser.png) · [측정·route 결과](browser-verification.json).

Agent Browser 세션을 닫고 실행 프로세스가 남지 않았음을 확인했다. Tailnet preview 서버는 계속 실행 중이다.

이번 상태·로드맵 PR 변경의 별도 [30관점 diff 검토](pr-diff-review-30.md)를 기록했다.

## 남은 경계

이번 동기화는 preview 문서 게시다. 전체 docs dead-link 통과, CSS/WPT 적합성, 실기기·hardware GPU, glyph paint 또는 C11.4/C11 완료를 주장하지 않는다. 공식 상태 수와 체크박스는 바꾸지 않았다.

# C11.4 공개 로드맵과 Tailnet 미리보기 동기화

## 원본 로드맵 보정

`spec/STATUS.md`는 구현 PR #959 `text-indent: hanging`/`each-line`과 PR #962 `line-break: loose`를 기록했지만, 공개 `spec/public/roadmap.html`에는 text-indent가 계획 후보로 남고 line-break는 PR #945까지만 보였다. 두 카드의 merge metadata·scope·근거 링크를 갱신하고 최신 병합순으로 배치했다. 공식 `72/213`과 C11/C11.4 미완료 상태는 유지한다. [변경 기록](public-roadmap-reconcile.json) · [공개 카드 diff 검토 30개](pr-diff-review-30.md).

## 선택 문서 빌드와 미리보기 게시

전체 문서 RSPress build는 저장소 전체의 dead links 때문에 종료 코드 1로 끝났다. 이 결과는 전체 문서 link 검증 통과로 취급하지 않는다. 별도 selected root의 434개 파일을 고정 RSPress 2.0.22로 빌드했다. 선택 설정은 dead-link/image 검사를 끄므로 새 공개 카드의 내부 route는 별도 publication manifest에서 검사했다. [전체 build 결과](full-docs-build-summary.json) · [성공한 selected build 로그](selected-rspress-build-pass02.log) · [선택 root manifest](selected-root.json) · [845-route publication manifest](selected-publication.json).

Tailscale backend는 `Running`, preview server는 PID 28040의 기존 `tailnet-preview-server.mjs`였다. 서버는 `127.0.0.1:8768`과 `/Users/yoonhb/Library/Application Support/SpinonPreview`에서 계속 실행했고 재시작하지 않았다. 사전 검사에서 선택 route 845개를 만들기 298, 교체 55, 동일 492로 분류했고 열려 있는 변경 대상 파일은 0개였다. copy 뒤 생성 298·교체 55·동일 492를 확인했다. 바뀐 기존 55개 파일은 별도 backup root에 보존했으며 55/55 hash 검증이 통과했다. 삭제는 없다. [사전 검사](tailnet-preflight.json) · [copy 및 backup](tailnet-copy-and-backup.json) · [served 검증](tailnet-served-verification.json) · [이전 preview hash와의 차이](target-drift-reconciliation.json).

이전 기록된 PR #956 publication보다 앞서 Tailnet root 일부 파일이 이미 갱신되어 있었다. PR #956 manifest와 다른 route 38개의 기존 bytes는 backup에 보존했다. raw 상태·계획 파일 중 확인 가능한 내용은 저장소의 과거 병합 commit 버전과 일치했다. 새 publication은 현재 `main`의 계획·상태와 이번 공개 roadmap 후보를 포함하며 경로 삭제 없이 추가·교체만 했다.

## 실제 페이지 확인

Agent Browser 0.31.1로 공개 Tailnet URL을 열었다. desktop 1280×900에서 문서 너비가 1280px, mobile 390×844에서 문서 너비가 390px이고 두 신규 카드 너비는 362px이었다. PR #962와 #959 카드, 공식 `72/213`, 교체된 text-indent 상태를 확인하고 두 계획 route를 열었다. 845개 전체 route는 HTTPS HTTP 200 및 publication SHA-256과 일치한다. [브라우저 확인 결과](browser-verification.json) · [desktop 화면](roadmap-desktop-agent-browser.png) · [mobile 화면](roadmap-mobile-agent-browser.png).

Agent Browser 세션과 프로세스를 종료했다. Tailnet preview server는 사용자 미리보기를 계속 제공하도록 실행 상태를 유지했다.

## 적대적 검토와 남은 한계

- 공개 로드맵 문구·merge 상태·제한 범위 diff: [30개 검토](pr-diff-review-30.md).
- 선택 publication·backup·hash·live route 검토: [30개 검토](tailnet-publication-review-30.md).
- 전체 문서 dead-link 통과, CSS/WPT 적합성, glyph paint, hardware GPU 및 전체 C11.4/C11 완료는 주장하지 않는다.

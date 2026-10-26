# Tailnet 로드맵 미리보기 동기화 · 2026-10-26

PR [#653](https://github.com/ohah/spinon/pull/653)이 `fb2671fbc81a65d686f8eae2c069cc86298cfdb7`에 Rebase and merge된 뒤, 해당 `origin/main` tree `50de6ebd3bccf9580f878dc17256196b651d96db`의 C11.4 계획·검토·실행 근거를 Tailnet 미리보기에 반영했다. 공식 `spec/STATUS.md`와 공개 `roadmap.html`은 바꾸지 않았다. C11.4는 미완료이고 공식 완료 집계 `72/213`은 유지한다.

## 빌드와 선택 범위

- 전체 `spec/` root를 바로 Rspress에 전달한 첫 시도는 내부 evidence에 보관된 옛 `rspress*.config.ts` 파일을 route source로 수집해 실패했다. 그 시도와 출력은 [full-root config](./rspress-full-root-attempt-pass01.config.ts), [정규화 log](./rspress-build-full-root-attempt-pass01.log), [원본 출력](./rspress-build-full-root-attempt-pass01.log.raw.gz)에 보존했다.
- 수정 실행은 이전 pass08-corrected selected root를 읽기 전용 hardlink base로 사용하고, `origin/main`의 현재 상태·로드맵·0059 계약·C11.4 부모/세부 계획·새 계획 검토 및 실행 근거를 byte-exact 확인 후 overlay했다. source commit은 `fb2671f...`, tree는 `50de6e...`다. 준비 근거는 [selected root manifest](./selected-root-pass01.json)와 [준비 스크립트](./prepare-selected-root-pass01.mjs)에 있다.
- Rspress `2.0.22`의 선택 build는 통과했다. [정규화 build log](./rspress-build-selected-pass01.log), [원본 출력](./rspress-build-selected-pass01.log.raw.gz), [선택 build manifest](./selected-build-pass01.json)에 16개 생성 문서 route 및 전체 78개 전달 route가 기록돼 있다. 정규화는 표시용 복사본의 trailing whitespace만 제거했으며 압축 파일은 원본 출력이다. 이 config는 dead-link와 dead-image 검사를 끄므로 전체 docs build나 저장소 전체 링크 검사를 의미하지 않는다.
- 전달 route에는 root `roadmap.html`과 `STATUS.md`, C11.4 부모/세부 계획, 0059 문서, 새 focused test 근거와 관련 raw 로그가 포함된다. overview page는 기존 부모 계획 링크를 유지하고, 새 §11.8 test-only slice와 PASS 상태는 부모 계획에서 연결한다.

## Tailnet 반영과 검증

- 사전 확인 때 Tailscale backend는 Running, Mac Studio는 online, preview server PID `28040`은 `127.0.0.1:8768`에서 수신했고 Tailnet roadmap은 HTTP 200이었다. 서버 재시작은 하지 않았다. [preflight](./tailnet-preflight-pass01.json)와 [교체 대상 open-handle 확인](./target-open-handles-pass01.json)을 보라.
- 선택한 78개 route 가운데 56개를 생성하고 15개를 byte-exact backup 후 교체했으며 7개는 이미 일치했다. 대체 전 총 `4,289,067` bytes를 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-26/pass01`에 보존했다. 파일 삭제는 없었다. [copy report](./tailnet-copy-pass01.json)와 [실행 출력](./tailnet-copy-run-pass01.log)에 기록했다.
- 동기화 후 로컬 route, 15개 backup, Tailnet HTTP 200 response body의 SHA-256이 모두 선택 build manifest와 일치했다: local `78/78`, backup `15/15`, remote `78/78`. [최종 검증](./tailnet-verify-pass01.json)과 [검증 실행 출력](./tailnet-verification-run-pass01.log)을 보라.

## Agent Browser 확인

- 터미널 브라우저를 사용하지 않았다. Agent Browser에서 [공개 roadmap](./tailnet-roadmap-agent-browser-pass01.json)을 확인해 `72/213` 표시, C11.4 미완료 checkbox와 부모 계획 링크를 보았다.
- [부모 계획](./tailnet-parent-plan-agent-browser-pass01.json)에서는 PR #653 diff review PASS 30/30, `72/213`, 세부 계획 링크를 확인했다.
- [새 §11.8 계획](./tailnet-stretch-plan-agent-browser-pass01.json)에서는 `StretchAutoTracks`, 예상 rows `[32.5,42.5,5]`, PASS 30/30과 `72/213`을 확인했다. [Agent Browser 화면](./tailnet-stretch-plan-agent-browser-pass01.png)은 해당 페이지의 첫 화면이다.
- 이 검증 후 Agent Browser session을 닫고 프로세스가 남지 않았는지 확인했다. [lifecycle 기록](./agent-browser-lifecycle-pass01.json).

## 범위

이 동기화는 선택한 78개 Tailnet 문서 route의 게시·hash 일치만을 확인한다. 앱 코드, simulator, CSS/WPT 지원 범위, C11.4 공식 상태 또는 전체 `docs:build` 성공을 주장하지 않는다. Android/iOS Simulator가 필요 없는 문서 게시 작업이며 PR #653 자체의 Rust 검증 로그는 새 계획 evidence에 연결돼 있다.

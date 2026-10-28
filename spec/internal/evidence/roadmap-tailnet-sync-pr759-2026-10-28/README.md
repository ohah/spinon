# PR #759 이후 공개 로드맵과 Tailnet 동기화

2026-10-28에 C11.4 two-track span overflow automatic minimum runtime slice의 공개 상태를 갱신하고 Tailnet 미리보기 `/spinon/roadmap.html`에 병합된 공개 HTML 한 파일을 반영했다.

## 소스와 범위

- 구현 PR [#759](https://github.com/ohah/spinon/pull/759)은 2026-10-28T13:20:23Z에 Rebase and merge됐고 merge SHA는 `9937a510439e46c5a6374a6ab373edc556afe2b0`이다. 상태 대장 반영 PR [#760](https://github.com/ohah/spinon/pull/760)은 2026-10-28T13:21:48Z에 merge SHA `b860322cd01d24dae051be5bb9f3d8edbe7ff697`로, 공개 로드맵 수정 PR [#761](https://github.com/ohah/spinon/pull/761)은 2026-10-28T13:26:56Z에 merge SHA `3210face8b77dd921c403188fe73f2c6993c53c2`로 Rebase and merge됐다.
- 공개 출처는 `origin/main`의 merge SHA `3210face8b77dd921c403188fe73f2c6993c53c2`에 있는 `spec/public/roadmap.html`이다. 파일은 374,155 bytes, SHA-256 `d24e7dfc498cd33ecdaeedee897b902f826103b9d9b1688ed50ccd1cfa4f070c`다.
- 교체한 대상은 `/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html` 하나뿐이다. 업데이트 전 파일 372,287 bytes, SHA-256 `faff974c222573d5c7ae8683a3a2f2072540dd2723b2d9263a39e38736ab26a3`는 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr759-2026-10-28/pass01/roadmap.html`에 바이트 그대로 백업했다.
- 같은 디렉터리 임시 파일을 만든 후 원자적으로 교체했다. 다른 route는 바꾸지 않았고 파일은 삭제하지 않았다.

## 사전·사후 검증

- Tailscale backend는 `Running`이었다. 미리보기 서버 PID `28040`이 `127.0.0.1:8768`에서 수신했고 작업 전후 같은 PID를 유지했다. 서버를 재시작하지 않았다.
- 작업 전 local과 Tailnet URL은 모두 HTTP 200, 372,287 bytes, 이전 파일의 SHA-256을 반환했다. 작업 뒤 local과 Tailnet URL은 모두 HTTP 200, 374,155 bytes, 병합된 출처와 같은 SHA-256을 반환했다.
- atomic replace 전후에 대상 파일 open handle이 없고 서버 TCP 상태가 listener-only가 된 것을 확인했다. 요청 직후 잠시 열린 loopback 연결은 최대 15초간 기다려 닫힌 뒤에만 교체했다.
- [preflight.json](preflight.json), [sync-result.json](sync-result.json), [동기화 스크립트](sync-roadmap.py)는 source/target hash와 경로, 백업, listener 및 HTTP 결과를 보존한다. 서로 다른 실패 경로 30개를 대조한 [publication diff 검토](tailnet-publication-pr-diff-review-30.md)와 [SHA-256 manifest](SHA256SUMS)도 포함한다.

## Agent Browser 검증과 종료

- Terminal Browser를 사용하지 않았다. Agent Browser로 [Tailnet 로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)을 열어 #759 merge SHA, Android/iOS 각각 5/5, 범위 한계, C11.4 미완료와 `72/213` 표시를 확인했다.
- [화면 캡처](tailnet-roadmap-agent-browser.png)는 1280×633이다. 페이지 title과 9개 근거 링크를 확인했고 capture SHA-256은 `agent-browser-verification.json`에 있다.
- Agent Browser `close` 명령은 성공했고 후속 session 정보는 `active: false`, `pid: null`을 반환했다. [관측 기록](agent-browser-verification.json) · [종료 기록](browser-close.json).

이 동기화는 정적 `roadmap.html` 단일 route 변경이다. 전체 문서 사이트 publication이나 Android/iOS 앱 재실행을 뜻하지 않는다. C11.4 전체는 미완료이며 공식 완료 집계 `72/213`은 유지된다.

# PR #675 공개 roadmap Tailnet 동기화

구현 PR [#675](https://github.com/ohah/spinon/pull/675)과 상태/roadmap PR [#676](https://github.com/ohah/spinon/pull/676)이 병합된 뒤, `spec/public/roadmap.html` 하나만 Tailnet 미리보기 경로에 동기화했다. PR #676 merge commit은 `d298fd61aca3afe795623d4356010c3c2003fb92`다.

## 교체와 복구 지점

- 원본 `spec/public/roadmap.html`: 336,731 bytes, SHA-256 `4a721943e14fb742459925f0eadbdff4c57d0f312282042ce9a1ed09f8f8b9a7`.
- 교체 전 Tailnet 파일: 333,951 bytes, SHA-256 `47f5c3429e08773ce5caa0824618f26e93e7dfd49a6dafce181e52ca4a5383b0`.
- byte-exact backup: `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr675-2026-10-27/pass01/roadmap.html`.
- 새 파일은 같은 디렉터리 임시 파일을 `roadmap.html`로 atomic rename했다. 다른 route를 바꾸거나 파일을 지우지 않았고 preview server PID 28040도 재시작하지 않았다.
- 교체 전 `lsof`에서 preview server가 이전 inode의 읽기 descriptor 하나를 보유했지만, 서버에는 listener 외 활성 TCP 연결이 없었다. `tailnet-preview-server.mjs`는 매 GET마다 경로를 다시 `stat`하고 stream을 열므로, 같은 디렉터리 atomic rename 뒤 다음 요청부터 새 inode를 읽는 동작을 확인했다.
- 사전·교체·사후 수치는 [`preflight.json`](preflight.json)과 [`sync-result.json`](sync-result.json)에, 재현 가능한 단일 route guard는 [`sync-roadmap-only.mjs`](sync-roadmap-only.mjs)에 기록했다.

## HTTP 및 Agent Browser 확인

- 교체 전 local route와 Tailnet route 모두 HTTP 200, 333,951 bytes, 이전 hash였다.
- 교체 후 `http://127.0.0.1:8768/spinon/roadmap.html` 및 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html` 모두 HTTP 200, 336,731 bytes, 원본과 같은 SHA-256을 반환했다.
- Agent Browser에서 Tailnet page title, PR #675 카드, merge SHA `96a392d32f69fae6453aecd897aa6d77c8b4aced`, 공식 `72/213`을 확인했다. 화면과 브라우저 종료 상태는 [`agent-browser-observation.json`](agent-browser-observation.json)과 아래 캡처에 남겼다.
- Agent Browser `close` 후 이 검증에서 연 Agent Browser 및 Chrome profile 프로세스가 최종 확인에서 사라졌다. 중간 목록에 보인 별도 HWP 세션은 종료 대상으로 삼지 않았다. 최종 확인에서는 8769 임시 미리보기 listener도 없고 기존 Tailnet server PID 28040만 계속 실행 중이었다.

![Tailnet Agent Browser에서 확인한 PR #675 roadmap 항목](tailnet-roadmap-pr675-agent-browser.png)

## 범위

선택한 정적 roadmap route만 동기화했다. Rspress 전체 build, GitHub Pages 배포, 앱·Simulator 실행, CSS 지원 범위 및 공식 완료 상태 변경은 포함하지 않는다. C11/C11.4는 미완료이고 `72/213`을 유지한다. 파일 hash와 screenshot hash는 [`SHA256SUMS`](SHA256SUMS)에 있다.

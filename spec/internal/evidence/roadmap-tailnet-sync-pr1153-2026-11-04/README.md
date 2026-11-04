# PR #1153 병합 후 Tailnet 로드맵 동기화

## 상태와 원본

구현 PR [#1152](https://github.com/ohah/spinon/pull/1152)은 Rebase and merge됐고 merge SHA는 `6fe4980533f3da1e7250ff00fdb4d21e541a3a55`다. 상태·계획·공개 roadmap source를 동기화한 PR [#1153](https://github.com/ohah/spinon/pull/1153)은 Rebase and merge됐고 source commit은 `287fd4872f64e62eb00584eea70e5a1645cc270c`다. 공식 완료는 `72/213`, C11.1/C11은 미완료다.

이번 게시에서 변경한 route는 `/spinon/roadmap.html` 하나다. 새 카드에 C11.1 혼합 교차축 제한 구현과 PR #1152 merge SHA를 적고, Android software renderer·iOS compositor 이전 sample 한계와 CI 상태를 함께 표시했다.

## 서버 점검과 파일 교체

Tailscale backend는 `Running`, host는 `macstudio.tailed42f2.ts.net`이다. 기존 preview server PID `28040`은 예상된 `tailnet-preview-server.mjs`였고 `127.0.0.1:8768`에 계속 listen했다. 로컬과 Tailnet route는 사전 점검에서 HTTP 200을 반환했다. preview server를 재시작하지 않았다.

Source `spec/public/roadmap.html`은 `origin/main` commit `287fd4872f64e62eb00584eea70e5a1645cc270c`와 byte-for-byte 일치했다. 변경 전 source는 559,228 bytes / SHA-256 `dc5c76792a21ce4edaa1ab0a4430c762a31a9c6d38bd91323b0812aa0c192157`다. 기존 preview는 557,742 bytes / SHA-256 `ab7f36660bcf6817bdabef61839ff56a95b78eb815e2f75590640319a02aedab`였다. 대상에는 open handle이 없었다.

대상 파일을 고유 백업 경로 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1153-2026-11-04/roadmap.html`에 먼저 복사해 이전 hash를 확인한 뒤 같은 디렉터리의 임시 파일을 atomic rename했다. 삭제는 없었다. 최종 local preview와 merged source SHA-256은 `dc5c76792a21ce4edaa1ab0a4430c762a31a9c6d38bd91323b0812aa0c192157`로 같다.

첫 copy 단계의 응답 기록은 같은 고정 `?sync=pr1153-preflight` URL을 재사용해 이전 remote body를 반환했다. 이를 성공 결과로 세지 않았다. 최종 검증은 고유 cache-busting query와 `Cache-Control: no-cache`로 다시 실행해 HTTP 200, 559,228 bytes 및 source와 동일한 hash를 확인했다. 그 뒤 사용자가 제시한 query 없는 Tailnet URL도 no-cache 요청으로 읽어 같은 HTTP 200/hash를 확인했다. 원본 기록은 [preflight](./preflight.json), [copy](./copy.json), [verify](./verify.json)에 있다.

## Agent Browser 확인과 종료

Agent Browser에서 cache-busted Tailnet route를 열어 page title `스피논 구현 로드맵`, 새 PR #1152 카드 1개, C11 미완료 문구 및 공식 `72/213`을 확인했다. Desktop `1280×1000`과 mobile `390×844` 모두 `clientWidth === scrollWidth`였고 각각 화면을 저장했다. [검증 JSON](./agent-browser-verification.json) · [Desktop](./tailnet-roadmap-agent-browser-desktop.png) · [Mobile](./tailnet-roadmap-agent-browser-mobile.png).

Agent Browser 세션을 종료했고 후속 확인에서 active session 0개 및 일치하는 browser process 0개였다. Preview server PID `28040`은 사용자가 접근하는 Tailnet 미리보기를 위해 계속 실행 중이다.

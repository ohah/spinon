# C11.4 fit-content 목록 · 병합 후 상태와 Tailnet roadmap 동기화

## 병합 상태

구현 PR [#1270](https://github.com/ohah/spinon/pull/1270)은 `2026-11-07T18:38:06Z` Rebase and merge됐고 merge SHA는 `de34c3c8f2b85826c61b6224f260f380ea972085`다. GitHub main은 같은 SHA를 가리킨다. PR check rollup과 workflow run은 비어 있고 main ruleset은 없으며 branch protection은 필수 status check나 approving review를 요구하지 않는다. 이는 CI 통과 결과가 아니다.

`spec/STATUS.md`, 내부 계약 0059, 두 계획 파일과 공개 roadmap을 merge 결과에 맞췄다. 최근 C11.4 slice를 PR #1270으로 표시하고 공식 C11.4/C11 완료 체크와 `78/214`는 그대로 뒀다. 내부 숫자 계약 `0.1.0`도 유지한다.

## Tailnet 제공본

공개 roadmap source `spec/public/roadmap.html` 628,254 bytes를 private preview의 `/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html`로 동기화했다. Source, target, loopback HTTP와 Tailnet HTTPS response의 SHA-256은 모두 `307632e0d8de1ccfc7441291e9625ff9ba79a7eb8cb48e11655d74f55d3f359f`다. 두 HTTP response 모두 status 200이다.

| 확인 | bytes | SHA-256 |
| --- | ---: | --- |
| 기존 Tailnet 제공본 | 628,186 | `505a4d247661e25bc64fd97467677daf3be6206e2257a4ed6b3b53a20c889578` |
| 새 source / preview / local HTTP / Tailnet HTTPS | 628,254 | `307632e0d8de1ccfc7441291e9625ff9ba79a7eb8cb48e11655d74f55d3f359f` |

기존 제공본은 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-c11-4-fit-content-track-list-sync-2026-11-08/pass01/roadmap.html`에 보존했다. 교체 스크립트는 target과 같은 디렉터리의 새 파일에 기록하고 `fsync` 후 atomic rename했다. Tailscale backend는 `Running`이고 preview server PID `28040`은 계속 `127.0.0.1:8768`에서 제공했다. 서버를 재시작하지 않았고 GitHub Pages도 배포하지 않았다. 사전 검사, 복사·backup manifest 및 사용한 script는 이 디렉터리에 있다.

## Agent Browser 확인

[Tailnet roadmap URL](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)에서 데스크톱 1440×1000과 모바일 390×844를 확인했다. 두 화면 모두 진행 카드 제목 `fit-content 목록`, `C11.4 · ACTIVE`, 공식 `78/214`와 PR #1270 merge SHA를 표시했다. `documentElement.scrollWidth`와 `body.scrollWidth`는 각 viewport 폭과 같았다.

- [Desktop DOM 확인](./agent-browser-tailnet-final-desktop.json) · [Desktop 화면](./agent-browser-final-desktop.png)
- [Mobile DOM 확인](./agent-browser-tailnet-final-mobile.json) · [Mobile 화면](./agent-browser-final-mobile.png)
- [Agent Browser 수명 종료 확인](./agent-browser-final-lifecycle.json)

Agent Browser 0.31.1 session을 종료한 뒤 `agent-browser` 프로세스가 남지 않았음을 확인했다. screenshot은 각각 1440×1000 및 390×844이며 hash는 [SHA256SUMS](./SHA256SUMS)에 있다.

## 범위

Tailnet은 저장소의 공개 roadmap preview일 뿐 GitHub Pages 배포가 아니다. 이 bounded slice는 C11.4/C11 완료를 뜻하지 않고 공식 `78/214`를 변경하지 않는다. 병합 후 문서 diff는 별도 30개 실패 관점으로 확인했다: [검토 기록](./post-merge-doc-pr-diff-review-30.md).

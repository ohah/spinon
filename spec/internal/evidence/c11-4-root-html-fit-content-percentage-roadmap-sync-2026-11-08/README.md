# C11.4 percentage fit-content · 병합 후 상태·Tailnet roadmap 동기화 · 2026-11-08

## 상태와 원인

구현 PR [#1268](https://github.com/ohah/spinon/pull/1268)은 `2026-11-07T16:22:21Z` Rebase and merge됐고 merge SHA는 `818d83b800d86c09bb379a93605aa9d5fc50aaa9`다. C11과 C11.4는 여전히 미완료이며 공식 집계는 `78/214`다.

동기화 전 Agent Browser는 실제로 `진행 중 1/2`와 `C11.4 · ACTIVE`를 보였다. 다만 제목 `intrinsic track sizing`이 일반적이라 어떤 slice인지 구분되지 않았다. 당시 화면은 구현 근거 폴더의 [`roadmap-before-sync.png`](../c11-4-root-html-fit-content-percentage-auto-rows-runtime-2026-11-08/roadmap-before-sync.png)다. 새 보드 제목은 `fit-content %`로 줄여 카드에서 잘리지 않게 했고, 바로 아래 최근 진행 설명에 전체 항목명 `Root HTML fit-content percentage implicit rows`와 PR #1268을 기록했다.

## Tailnet 제공본

`spec/public/roadmap.html`만 private preview의 `/Users/yoonhb/Library/Application Support/SpinonPreview/roadmap.html`에 복사했다. 세 pass 중 첫 두 제목은 카드에서 줄임표로 잘려 다시 고쳤으며, 각 직전 제공본은 다음 pass의 backup으로 보존했다. 최종 pass03 source는 `628186` bytes, SHA-256 `505a4d247661e25bc64fd97467677daf3be6206e2257a4ed6b3b53a20c889578`다. Local HTTP와 Tailnet HTTPS 모두 HTTP 200, 동일한 byte count와 SHA-256을 반환한다.

| 제공 확인 | source SHA-256 | 결과 |
| --- | --- | --- |
| pass01 | `170fc9e3dafbec4247d75ce7ac49960b39eafc74d23c9ea86fa5304699885496` | 제목이 카드 폭을 넘겨 pass02에서 수정 |
| pass02 | `3b74d4925f49d03e9575c3e0b1834e793b4f6c063bc6e75fef139a5e21c762f3` | 여전히 일부 잘려 pass03에서 짧게 수정 |
| pass03 최종 | `505a4d247661e25bc64fd97467677daf3be6206e2257a4ed6b3b53a20c889578` | local HTTP·Tailnet HTTPS 모두 일치 |

- [pass01 사전 확인](./preflight-pass01.json) · [pass01 교체·backup](./copy-and-backup-pass01.json)
- [pass02 사전 확인](./preflight-pass02.json) · [pass02 교체·backup](./copy-and-backup-pass02.json)
- [pass03 사전 확인](./preflight-pass03.json) · [pass03 교체·backup](./copy-and-backup-pass03.json)
- [동기화 스크립트](./sync-tailnet-roadmap.mjs) · [최종 파일 SHA-256 목록](./SHA256SUMS)

교체는 같은 디렉터리의 임시 파일에 기록하고 `fsync` 후 atomic rename했다. Tailscale backend는 `Running`; preview server PID `28040`이 `127.0.0.1:8768`에서 계속 제공 중이었다. 해당 프로세스를 재시작하지 않았고 GitHub Pages도 배포하지 않았다. 변경 파일은 `/spinon/roadmap.html` 하나다.

## Agent Browser desktop/mobile 확인

최종 Tailnet URL `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`에서 두 화면 모두 공식 `78/214`, `진행 중 1/2`, `fit-content %`, `C11.4 · ACTIVE`를 확인했다. Desktop `1440×1000`, mobile `390×844`에서 document/body 가로 폭은 각각 viewport와 같다. 전체 세부 항목명과 병합 결과는 상단 최신 진행 설명에 보인다.

- [Desktop DOM 결과](./agent-browser-tailnet-final-desktop.json) · [Desktop 화면](./agent-browser-final-desktop.png)
- [Mobile DOM 결과](./agent-browser-tailnet-final-mobile.json) · [Mobile 화면](./agent-browser-final-mobile.png)
- [Agent Browser 종료 결과](./agent-browser-final-lifecycle.json)

두 화면의 원본 크기는 각각 `1440×1000`과 `390×844`다. Agent Browser session 종료 후 활성 session이 없고 session info는 `active=false`, `pid=null`이다.

## 검토

병합 후 `spec/STATUS.md`, 내부 계약 0059, 계획 문서와 공개 로드맵 변경분을 별도 30개 관점으로 검토했다: [post-merge docs PR diff review](./post-merge-doc-pr-diff-review-30.md). C11/C11.4 완료 체크와 공식 `78/214`는 바꾸지 않았다.

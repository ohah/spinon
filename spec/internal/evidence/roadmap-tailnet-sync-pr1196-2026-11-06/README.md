# PR #1196 병합 후 Tailnet roadmap 제공본 확인

PR [#1196](https://github.com/ohah/spinon/pull/1196)은 2026-11-05T21:41:05Z Rebase and merge됐고 merge SHA는 `b746dae9ba662da65b0ca2867934362ee9a40d09`다. 이 기록은 해당 `main`의 C11.4 PR #1195 문서 변경을 사용자가 지정한 Tailnet 미리보기 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`에 반영하고 제공 상태를 확인한 결과다. GitHub Pages는 배포하지 않았다.

## 선택 파일 동기화

- 기준 `sourceHead`: `b746dae9ba662da65b0ca2867934362ee9a40d09`; source tree: `b70142d90f663302f3e3cf88b40d48a35a3c7684`.
- Tailscale backend는 `Running`; 기존 preview server PID `28040`의 `/Users/yoonhb/Library/Application Support/SpinonPreview/tailnet-preview-server.mjs`가 `127.0.0.1:8768`에서 수신 중이었다. 서버를 재시작하거나 종료하지 않았다.
- source `spec/public/roadmap.html`은 584,210 bytes / SHA-256 `42369b6d07a4aad15b523e5e636de470f7126cac788361e1cb690300228c1255`다.
- 교체 전 제공 파일은 581,932 bytes / SHA-256 `4cf96a01cf0ecfe0c71a1fd453410541f752abc432135fa66b6d3d10f2fe4f54`였다. 정확한 교체 직전에도 대상 파일을 여는 process가 없었다.
- 교체 전 파일은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1196-2026-11-06/pass01/roadmap.html`에 새 backup으로 보관했다. 새 파일은 임시 파일을 쓴 뒤 atomic rename으로 반영했다. 삭제 route는 없고 기존 preview server를 유지했다.
- Local HTTP와 Tailnet HTTPS가 모두 HTTP 200, 584,210 bytes였고 source/local/remote SHA-256이 `42369b6d07a4aad15b523e5e636de470f7126cac788361e1cb690300228c1255`로 일치한다. 상세 preflight, backup·copy 및 served verification 결과는 같은 directory의 `preflight.json`, `copy-and-backup.json`, `served-verification.json`에 있다.

## Agent Browser 화면 확인

Agent Browser에서 실제 Tailnet URL을 열고 live counter, 새 PR #1195 카드, card links와 desktop/mobile geometry를 확인했다.

- Desktop 1440×1000: document/body width 1440px, C11.4 PR #1195 카드 x=90px·width=928px·right=1018px. 화면은 `tailnet-roadmap-agent-browser-desktop.png`, DOM·link·geometry는 `tailnet-agent-browser-desktop.json`에 기록했다.
- Mobile 390×844: document/body width 390px이며 horizontal overflow가 없다. C11.4 카드 x=14px·width=362px·right=376px. 화면은 `tailnet-roadmap-agent-browser-mobile.png`, DOM·link·geometry는 `tailnet-agent-browser-mobile.json`에 기록했다.
- 두 viewport에서 live counter는 `214개 표시 · 공식 완료 78/214`, 동기화 note는 `상위 78개 + 세부 ID 134개 + ID 없는 하위 체크 2개 · 완료 78/214개`를 표시했다. 상단 counter 화면은 `tailnet-roadmap-agent-browser-counter.png`다.
- 카드에는 제한된 explicit intrinsic column + direct-text implicit min-content row 범위, fixture hash, Android/iOS simulator 한계와 C11.4/C11 미완료가 보인다. Android/iOS screenshot 링크는 PR #1195 merge SHA를 가리키며 PR #1196 본문에서 두 이미지가 각각 1080×2400 및 1206×2622로 정상 로드되는 것을 Agent Browser로 확인했다.
- 확인 뒤 `agent-browser close --all`을 실행했다. 후속 process inspection에서 Agent Browser 및 Chrome for Testing process는 보이지 않았다. Tailnet preview server PID 28040은 그대로 유지 중이다.

## 검증 한계

전체 `mise exec -- bun run docs:build`는 여러 internal evidence 문서의 dead-link 검사에서 exit 1이었으며, 변경 전 checkout과 비교하지 않아 문제 원인을 기존 상태로 분류하지 않는다. 이번 확인은 선택된 정적 roadmap 파일의 local/remote SHA, HTTP 응답, 화면 표시와 responsive geometry를 증명한다. 전체 RSPress link validation, CI 성공, GitHub Pages 배포 또는 추가 CSS 동작 검증을 뜻하지 않는다.

이 제공·화면 evidence의 PR 변경분 검토 30개: [review](./post-merge-evidence-pr-diff-review-30.md).

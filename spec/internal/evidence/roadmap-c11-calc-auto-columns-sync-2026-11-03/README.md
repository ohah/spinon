# C11.1 calc 열 변경 Tailnet roadmap 동기화 근거 · 2026-11-03

2026-11-03에 병합된 구현 PR [#1097](https://github.com/ohah/spinon/pull/1097)과 병합 후 roadmap 문서 PR [#1098](https://github.com/ohah/spinon/pull/1098)을 공개 Tailnet 미리보기와 맞췄다. 이 동기화는 C11.1/C11 완료나 공식 구현 수 변경을 뜻하지 않는다. 공식 완료 `72/213`과 C11 미완료 표시는 그대로다.

## 입력과 제한 빌드

- 문서 source HEAD: `f2cf4bb7fe19f30d5e927966aa79921f3583f093`; source tree: `82688b49d18630737af206543ee5e586545026f0`; checkout은 `origin/main`과 tree-identical이고 tracked diff SHA-256은 빈 diff `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`다.
- 선택 입력은 git index에 있는 C11 계획·상태·내부 계약·calc precomparison/runtime·roadmap 변경 검토의 tracked 파일 52개, 3,338,782 bytes다. symlink는 포함하지 않았다.
- RSPress 2.0.22 선택 빌드에서 dead-link/dead-image 검사를 끄고 107 route, 12,623,481 bytes를 staging했다. 이것은 선택 publication이며 전체 문서 링크 검사나 전체 저장소 문서 빌드의 성공이 아니다.
- 일반 전체 문서 빌드 두 시도는 기존 R05 dead-link 및 전체 tree의 Shiki `rust,ignore`/asset-resolution 문제로 실패했다. 해당 범위는 이 C11 변경에 속하지 않는다. Tailnet에 올린 선택 빌드 자체는 성공했다.
- [선택 입력 manifest](./selected-root-pass04.json) · [route·size·SHA-256 manifest](./selected-publication-pass04.json).

## Tailnet 동기화

- 실제 preview root는 `/Users/yoonhb/Library/Application Support/SpinonPreview`, server PID `28040`, `127.0.0.1:8768`이었다. `tailscale status --json`에서 backend `Running`, `tailscale serve status --json`에서 HTTPS 443 → `127.0.0.1:8768`을 확인했다.
- 기존 Tailnet/static preview server를 재시작하지 않았다. 기존 roadmap의 local/HTTPS 응답은 모두 HTTP 200, 536,480 bytes, SHA-256 `fe4b1e131dd576fe4d52f27a6f14903695898be718ca2b517ddb75747e0f5300`이었다.
- 선택 107 route 중 76개를 생성, 9개를 교체, 22개는 동일하다고 확인했다. 삭제는 없었다. 교체 전 9개 파일을 `~/Library/Application Support/SpinonPreviewBackups/roadmap-c11-calc-auto-columns-sync-2026-11-03/pass04`에 저장했고 hash를 9/9 검증했다.
- 최종 local disk와 HTTPS 응답은 107/107에서 publication manifest와 SHA-256이 일치하고 HTTP 200이었다. 핵심 roadmap·STATUS·0059·C11 plan·calc plan·runtime README·두 review route는 local/HTTPS 각각 8/8로 추가 확인했다. 상세 입력은 [preflight](./preflight-pass04.json), [backup/copy](./copy-and-backup-pass04.json), [served verification](./served-verification-pass04.json)에 있다.
- 최종 `/spinon/roadmap.html`: 538,151 bytes, SHA-256 `2af02d414f0a442b18157acb391db50dcd315a7d1133671b13af363bfb03568e`. 공개 카드에는 PR #1097, merge SHA, 제한 calc 범위, C11 미완료 및 `72/213`이 표시된다.

## Agent Browser 확인과 종료

`https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`의 새 C11.1 카드와 local link 다섯 개를 확인했다. Desktop viewport 1280×633에서 document width 1280px, mobile viewport 390×844에서 document width 390px였다. mobile에서 새 카드 bounds는 x=14px, width=362px로 viewport 안에 들었다. [desktop 캡처](./tailnet-roadmap-desktop.png) · [mobile 캡처](./tailnet-roadmap-mobile.png).

Agent Browser에서 calc 계획, runtime README 및 Android `1080×2400`/iOS `1206×2622` screenshots를 열었다. 이 확인 후 모든 작업 세션을 닫고 `agent-browser session list`에서 active session이 없음을 확인했다. `pgrep`에서도 agent-browser/Chromium 프로세스가 없었다. 기존 Tailnet과 static preview server는 계속 실행 중이다.

## 범위와 해석

- 페이지는 bounded parsed Root HTML sparse column-flow `grid-auto-columns: calc(px + %)` slice만 설명한다. C11.1/C11 official checkbox와 `72/213`은 변경하지 않았다.
- Simulator screenshots는 visual smoke이며 Android PixelCopy, iOS Metal drawable readback, compositor 이후 pixel equality를 뜻하지 않는다.
- GitHub CI/check rollup은 비어 있고 main required status checks도 설정되지 않았다. 로컬 검증·선택 빌드·Tailnet served hash 확인을 CI로 확대하지 않는다.
- [로드맵 source 변경 검토 30개](./roadmap-pr-diff-review-30.md) · [Tailnet sync PR 변경 검토 30개](./tailnet-sync-pr-review-30.md) · [artifact hashes](./artifact-hashes.sha256).

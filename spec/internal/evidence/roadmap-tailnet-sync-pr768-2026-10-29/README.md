# C11.4 PR #768 Tailnet roadmap sync

구현 PR [#768](https://github.com/ohah/spinon/pull/768)은 2026-10-28T18:41:57Z Rebase and merge됐고 merge SHA는 `7c004e8f5ed3197d696abcc8f84bcace83b97fe2`다. PR check rollup과 required-check 조회는 비어 있었고 main에는 required status checks가 설정돼 있지 않았다. CI 통과로 서술하지 않는다. 공식 C11.4 상태는 미완료, 완료 집계는 `72/213`으로 유지한다.

## 대상과 경계

- Tailnet route: [공개 로드맵 미리보기](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)
- Preview root: `/Users/yoonhb/Library/Application Support/SpinonPreview`
- Active preview server: `tailnet-preview-server.mjs`, `127.0.0.1:8768`; 현재 Tailscale Serve host route가 이 listener를 가리킨다.
- Source page: [`spec/public/roadmap.html`](https://github.com/ohah/spinon/blob/main/spec/public/roadmap.html)

선택한 문서는 public roadmap과 이 slice에서 연결하는 status, plan, 내부 Grid 계약, runtime evidence 및 screenshot asset이다. 선택하지 않은 preview 파일은 제거하지 않았다. 대상 교체 전 파일을 pass별 backup에 보존했다.

## 동기화와 제공 응답

최종 manifest는 preview root 아래 170개 파일을 열거한다. 각 대상의 생성·교체·동일 결과와 사전 SHA-256은 manifest에 기록한다. 추가로 manifest의 internal/spec 두 route를 검사해 총 172개 route의 HTTP 200, response byte 수와 SHA-256이 일치하는지 확인했다. 불일치 0건이며 deletion path는 없다. 교체 전 파일은 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr768-2026-10-29/pass01`과 `pass02`에 보존했다. manifest 자체와 served-hash report는 자기 참조를 피하도록 target 목록에서 제외하고 manifest route를 별도 검사했다.

선택한 docs root는 Rspress 2.0.22 build를 통과했다. 전체 docs root build는 기존 dead link와 Shiki language/image 문제로 끝나지 않아 이를 전체 사이트 build 성공으로 확대하지 않는다.

- [선택 파일/served route SHA-256 manifest](./tailnet-sync-manifest.json)
- [served response 검증 결과](./tailnet-served-hash-check.json)
- [Agent Browser route·상태·종료 기록](./agent-browser-verification.json)
- [Tailnet roadmap의 PR #768 화면](./tailnet-roadmap-agent-browser.png)
- [동기화 실패 관점 검토 30개](./docs-sync-pr-diff-review-30.md)

Agent Browser에서 roadmap의 PR #768 카드가 보이고 status의 `C11.4 intrinsic track sizing` checkbox가 미완료인 것을 확인했다. status와 roadmap은 공식 `72/213`을 표시한다. 검증 뒤 Agent Browser session을 닫았고 active session과 Chrome process는 남지 않았다.

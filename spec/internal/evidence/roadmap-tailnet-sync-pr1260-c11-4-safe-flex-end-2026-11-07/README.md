# PR #1260 병합 후 Tailnet 로드맵 동기화 · 2026-11-07

## 상태 변경

PR [#1260](https://github.com/ohah/spinon/pull/1260)은 Rebase and merge로 병합됐고 merge SHA는 57f5b9103e919a8dc7f9378dc78f984b3101f1f1이다. C11.4/C11 완료 표시는 유지하고 공식 집계는 78/214로 둔다. 칸반의 실제 진행 카드는 이제 “후속 slice 계획 중”으로 표시한다. 이 카드는 다음 C11.4 범위를 정리하는 단계이며, 새 구현이 이미 진행 중이라는 뜻은 아니다.

## Tailnet 제공본

PR #1260 병합 이후 작업 브랜치 docs/c11-root-html-safe-flex-end-postmerge에서 갱신한 spec/public/roadmap.html을 Tailnet preview에 반영했다. 교체 전 제공본 622,565 bytes, SHA-256 a291392b7c7be1e68d9e1dabca5fe9e5d91f034e7d338fe2f014e22b5ca5586d는 pass03 backup으로 보존했다. pass04에서는 중간 제공본 622,945 bytes, SHA-256 28ad80d6eb2b1e6b58fb2f03349d9d713e6b4a67cc3b06c9c014b788763def55도 백업했다. 최종 source와 local HTTP 및 Tailnet HTTPS 제공본은 모두 622,905 bytes, SHA-256 980c92d9360b39a5ccc319aede900108c68ceb1fcc519c4936fae528b78d2494다. 두 URL은 HTTP 200을 반환했다.

- pass03 backup: /Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-c11-safe-flex-end-in-progress-2026-11-07/pass03/roadmap.html
- pass04 backup: /Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-c11-safe-flex-end-in-progress-2026-11-07/pass04/roadmap.html
- [pass03 preflight](./preflight-pass03.json) · [pass03 copy/backup](./copy-and-backup-pass03.json)
- [pass04 preflight](./preflight-pass04.json) · [pass04 copy/backup](./copy-and-backup-pass04.json)
- [동기화 스크립트](./sync-tailnet-roadmap.mjs)
- [병합 후 문서 diff 실패 검토 30개](./post-merge-doc-diff-review-30.md)

교체는 same-directory 임시 파일, fsync, atomic rename을 사용했다. Tailscale backend는 Running이었다. preview server PID 28040을 재시작하지 않았고 GitHub Pages도 배포하지 않았다. 새 로드맵 항목의 계획·실행 근거 링크는 Tailnet 정적 미리보기에서 아직 생성되지 않은 페이지를 가리키지 않도록 GitHub main으로 연결했다.

## Agent Browser 확인

최종 Tailnet URL을 Agent Browser로 확인했다. Desktop 1440×1000과 mobile 390×844에서 진행 중 1/2, “후속 slice 계획 중”, C11.4 ACTIVE, 공식 78/214를 확인했다. 두 viewport 모두 document/body scroll width가 viewport와 같았다.

- [Desktop DOM 기록](./agent-browser-tailnet-final-desktop.json) · [Desktop 화면](./agent-browser-final-desktop.png)
- [Mobile DOM 기록](./agent-browser-tailnet-final-mobile.json) · [Mobile 화면](./agent-browser-final-mobile.png)
- [Agent Browser 종료 기록](./agent-browser-final-lifecycle.json)

Desktop screenshot SHA-256: 1bbefbbc21671766c78ba3a6f01d33acff2600a874878ba18d412fb2c7f317ae
Mobile screenshot SHA-256: 0f2ef408b1ce70e3a5ea36a31aacb859353e6d1079cf088bf4dbee03fee5f90d

Agent Browser session을 닫았고 종료 뒤 Agent Browser/Chrome for Testing 프로세스가 남지 않았다.

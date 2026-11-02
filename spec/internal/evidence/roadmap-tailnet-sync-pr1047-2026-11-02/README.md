# PR #1047 이후 C11 Root HTML percentage auto-row Tailnet 동기화 · 2026-11-02

## 범위

구현 PR [#1047](https://github.com/ohah/spinon/pull/1047) 병합으로 저장소의 `spec/public/roadmap.html`, C11 계획, 전용 percentage `grid-auto-rows` 계획 및 evidence가 갱신됐다. Tailscale preview server는 저장소 파일을 자동으로 제공하지 않으므로 최신 `main` source를 기반으로 선택 문서 route를 빌드하고 기존 `/Users/yoonhb/Library/Application Support/SpinonPreview`에 반영했다.

C11.1과 C11은 미완료이며 공식 완료 수는 `72/213`으로 유지된다. 선택한 C11 route만 갱신했다. 전체 문서 사이트의 모든 링크를 검사한 것은 아니다.

## Source 및 실행 결과

| 확인 | 결과 |
| --- | --- |
| source `main` commit | `444304968ff930d23d9c73075db83d0ca97cccb8` |
| source tree | `943843d78d1b8ff7f17f8cf278c2cbbf6fe52294` |
| Rspress | 2.0.22, 선택 문서 build 성공 |
| Tailscale backend | `Running` |
| preview server | PID `28040`, `127.0.0.1:8768`; 재시작하지 않음 |
| preview root | `/Users/yoonhb/Library/Application Support/SpinonPreview` |
| 선택 publication | 188 routes, 8,908,454 bytes |
| 변경 분류 | 95 created, 13 replaced, 80 identical |
| 교체 전 백업 | 13 files, `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1047-2026-11-02/pass01` |
| 바꿀 파일 open handles | 0 |
| 로컬 및 HTTPS 검증 | local SHA-256 188/188, HTTPS HTTP 200 + SHA-256 188/188 |
| 백업 검증 | SHA-256 13/13 |
| 삭제 / server restart | 0 / 하지 않음 |
| 공식 상태 | `72/213`, C11 unchecked |

[선택 route manifest](./selected-publication.json)는 각 source, byte 크기와 SHA-256을 담는다. [preflight](./preflight.json)는 교체 전 대상·백업·open-handle 상태를, [copy report](./copy-and-backup.json)는 파일 생성·교체와 백업을, [served verification](./served-verification.json)은 로컬·HTTPS hash를 기록한다. `prepare-site.mjs`, `build-publication.mjs`, `sync-tailnet.mjs`는 같은 절차의 재현 파일이다. 이전 파일은 변경 전에 복사했고, atomic rename을 사용했으며 route 삭제는 하지 않았다.

## Agent Browser 확인

Tailnet `/spinon/roadmap.html`에서 PR #1047 card, merge SHA, `72/213`, C11 미완료 문구를 확인했다. C11 percentage plan, runtime README, C11 overview, `STATUS.html`도 열어 merge/status markers를 확인했다. 세로 화면 폭 390px에서 문서 `scrollWidth`는 390px이며 새 card는 362px 너비로 viewport 안에 놓였다. 하단 overview SVG는 기존처럼 오른쪽이 잘려 보인다. 이는 page-level horizontal overflow는 아니지만 해당 도식의 mobile 표시 한계다.

- [Desktop 1280×900 screenshot](./tailnet-roadmap-agent-browser-desktop-pass01.png), SHA-256 `ddf7d721f35c57857273cd4a4c0fa77f638e1eb8bd31b89c1ff92d99ce31b282`
- [Mobile 390×844 screenshot](./tailnet-roadmap-agent-browser-mobile-pass01.png), SHA-256 `05b8503d4b25a80f28bf61101650981808073c495c3e9d02e30407037ef5bb43`
- 기계 판독 결과는 [agent-browser-verification.json](./agent-browser-verification.json)에 있다.

`agent-browser close --all` 후 browser session과 process를 종료했으며, 후속 process scan에서 Agent Browser/Chrome process가 없었다.

## 한계

이 배포는 Tailnet preview의 선택 문서 및 evidence route만 갱신한다. WPT 적합성, 일반 CSS 지원, native app pixel parity 또는 C11/C11.1 완료를 증명하지 않는다. 전체 docs link validation도 수행하지 않았다. [Tailnet 변경 30개 관점 검토](./tailnet-pr-diff-review-30.md).

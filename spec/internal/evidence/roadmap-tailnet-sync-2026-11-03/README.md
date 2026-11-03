# Tailscale 공개 로드맵 C11.1 동기화 · 2026-11-03

`spec/public/roadmap.html`의 C11.1 parsed Root HTML dense auto-placement 기록을 최신 병합 PR #1117로 갱신했다. C11 checkbox와 공식 `72/213`은 그대로다. 공개 페이지 변경은 PR #1119로 병합한 뒤 Tailnet preview의 `/spinon/roadmap.html`에 반영했다.

## 게시 결과

PR [#1119](https://github.com/ohah/spinon/pull/1119)은 `2026-11-03T13:13:23Z` Rebase and merge됐고 merge SHA는 `c1c8eae0f52f1d7a0358a813c32b879fc4879692`다. PR check rollup은 비어 있어 CI 통과로 세지 않는다.

동기화 스크립트는 origin/main의 `spec/public/roadmap.html`과 byte-for-byte 일치하는 파일만 게시한다. 변경 대상은 `roadmap.html` 하나이며 삭제는 없었다.

| 대상 | Bytes | SHA-256 |
|---|---:|---|
| 병합된 원본 | 546,771 | `c275d6c1bee8a1c58ab061be4236eeab331112f095c9feb5a818ab4f248f2df3` |
| 동기화 전 preview | 545,235 | `e71529145854fbe23a548f3759cbe85f7bf6e0ab10185548efa3de58dfca23f0` |
| 보존 backup | 545,235 | `e71529145854fbe23a548f3759cbe85f7bf6e0ab10185548efa3de58dfca23f0` |
| 동기화 후 local preview | 546,771 | `c275d6c1bee8a1c58ab061be4236eeab331112f095c9feb5a818ab4f248f2df3` |
| Tailnet HTTPS 응답 | 546,771 | `c275d6c1bee8a1c58ab061be4236eeab331112f095c9feb5a818ab4f248f2df3` |

Tailnet route는 HTTP 200을 반환했고, merged source·local preview·remote response SHA-256이 모두 일치했다. 이전 파일은 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-11-03/roadmap.html`에 보존했다.

## Agent Browser 확인

Live page title은 `스피논 구현 로드맵`이다. 최신 C11.1 PR #1117 카드와 PR #1118 문서 링크가 보이며 C11 미완료와 공식 `72/213`을 유지한다. 1280×720 및 390×844 viewport에서 페이지 가로 넘침은 없었다.

- [Desktop 1280×720](./tailnet-roadmap-desktop-1280.png)
- [Mobile 390×844](./tailnet-roadmap-mobile-390.png)
- [viewport·DOM 확인값](./agent-browser-verification.json)

확인 후 Agent Browser daemon과 임시 local preview server를 종료했다. [preflight](./preflight.json) · [copy](./copy.json) · [remote verify](./verify.json)

## 남은 범위

이 페이지 업데이트는 parsed Root HTML dense-placement 제한 구현의 진행 기록만 갱신한다. C11.1/C11 checkbox와 `72/213`은 바꾸지 않는다. 전체 WPT/CSS Grid 적합성, 실기기/hardware GPU, iOS compositor 이후 pixel parity는 확인하지 않았다.

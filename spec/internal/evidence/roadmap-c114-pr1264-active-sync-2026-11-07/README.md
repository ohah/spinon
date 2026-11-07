# C11.4 PR #1264 활성 로드맵 동기화 · 2026-11-07

## 결과

- `spec/public/roadmap.html`을 `~/Library/Application Support/SpinonPreview/roadmap.html`에 원자적으로 교체했다. 변경·삭제된 route는 `roadmap.html` 하나이며 preview 서버 재시작은 없다.
- 기존 제공본 SHA-256 `e5b48551412e60cb0dde646929a65e18c61fe11d3ef81665619e5246714a3400`은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-c114-pr1264-active-sync-2026-11-07/roadmap.html`에 보존했다.
- 새 로컬 HTML, preview 파일, `127.0.0.1:8768` 제공본과 Tailnet 제공본 모두 SHA-256 `7f0207249be2d1dbdcac89321faca459069289c4f820ce6fa696d7f183ca1eb4`, 626304 bytes다. 서버 PID는 28040으로 동일하다.
- Agent Browser 0.31.1의 Chrome 157에서 Tailnet 로드맵을 확인했다. 데스크톱 1440×1000과 모바일 390×844 화면에 `C11.4 · ACTIVE` 카드가 표시되고 공식 상태는 `78/214`다. 모바일 캡처는 활성 열이 보이도록 가로 스크롤된 상태다.
- PR [#1264](https://github.com/ohah/spinon/pull/1264) 본문 Android/iOS 이미지를 Agent Browser로 확인했다. raw 이미지가 각각 1080×2400 및 1206×2622 픽셀로 정상 렌더됐다.

## 근거 파일

- `preflight.json`, `copy-and-backup.json`, `served-verification.json`: 교체 전 해시, 백업 및 파일/local/Tailnet 제공본 검증
- `tailnet-roadmap-agent-browser-desktop.png`, `tailnet-roadmap-agent-browser-mobile.png`: Agent Browser 실제 화면
- `pr-image-agent-browser-check.json`: PR 본문 이미지 URL과 브라우저 natural dimensions
- `sync-tailnet.mjs`: 경로·서버 PID·기존 해시를 확인하고 대상 HTML 한 개만 교체한 스크립트

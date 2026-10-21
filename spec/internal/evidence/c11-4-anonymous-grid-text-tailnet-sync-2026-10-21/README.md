# Tailnet 로드맵 동기화 근거 · pass38

2026-10-21 C11.4 anonymous Grid text 제한 구현 PR [#422](https://github.com/ohah/spinon/pull/422)와 최신 Simulator 결과를 Tailnet 미리보기 `/spinon/`에 반영했다. 로드맵의 현재 공식 진행 집계는 **72/213**이다. 일부 과거 항목 본문에 남은 `61/197`은 이전 스냅샷이며 현재 집계는 `spec/STATUS.md`에서 동적으로 읽는다.

## 선택 빌드

- RSPress `2.0.22`가 2,725개 파일, 113,284,825 bytes를 빌드했다. [`selected-build-summary-pass38.json`](./selected-build-summary-pass38.json)과 [`selected-build-pass38.log`](./selected-build-pass38.log)에 결과를 보존했다.
- 선택 root는 pass37 root에서 복사한 뒤 현재 `spec/STATUS.md`, `spec/public/roadmap.html`, C11.4 계획·계약 및 두 evidence 폴더를 작업 tree와 byte 단위로 맞췄다. [root 준비 기록](./selected-root-preparation-pass38.json).

## Tailnet 반영과 검증

- 대상은 `/Users/yoonhb/Library/Application Support/SpinonPreview`, 서버는 PID `1191`의 `tailnet-preview-server.mjs` (`127.0.0.1:8768`)였다. 서버는 그대로 두었다.
- Preflight 결과는 create 198, replace 156, same 2,371, delete 0이다. 교체 전 파일 156개, 16,012,513 bytes를 pass38 backup root에 복사하고 SHA-256을 검증했다. [preflight](./preflight-pass38.json) · [반영·백업 결과](./copy-and-backup-pass38.json).
- 로컬 target 2,725/2,725개와 backup 156/156개가 SHA-256까지 일치했다 ([로컬 검증](./local-verification-pass38.json)). Tailnet URL에서도 2,725/2,725개 모두 HTTP 200 및 기대 SHA-256과 일치했다 ([원격 검증](./served-verification-pass38.json), [실행 요약](./remote-verification-pass38.log)).

## Agent Browser 화면 확인

- [로드맵 화면](./tailnet-roadmap-agent-browser-pass38.png)에서 C11.4 최신 note와 PR #422 링크를 확인했다. DOM에는 현재 공식 집계 `72/213`이 표시됐다 ([DOM 결과](./roadmap-dom-pass38.json)).
- [실행 근거 페이지](./tailnet-evidence-agent-browser-pass38.png)의 iOS·Android screenshot은 각각 `1206×2622`, `1080×2400`으로 로드됐다 ([DOM 결과](./evidence-dom-pass38.json)).
- [PR #422 화면](./pr-agent-browser-pass38.png)에서 본문에 넣은 Android·iOS 이미지도 raw URL에서 각각 `1080×2400`, `1206×2622`로 로드됐다 ([DOM 결과](./pr-dom-pass38.json)).
- Agent Browser를 닫은 뒤 세션 `active=false`, `pid=null`이며 Agent Browser/Chrome 프로세스가 남지 않은 것을 확인했다 ([종료 기록](./browser-close-pass38.json)). Tailnet preview server는 계속 실행 중이다.

저장소 공백 검사를 유지하기 위해 캡처한 CLI·build 로그에서는 행 끝 공백과 파일 끝 빈 줄만 정규화했다. 표 정렬 외 명령 출력 내용은 바꾸지 않았다.

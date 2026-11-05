# PR #1176 병합 후 Tailnet 로드맵 동기화

이 근거는 PR [#1176](https://github.com/ohah/spinon/pull/1176) 병합 직후의 C11 상태 문서와 공개 로드맵을 사용자가 정한 Tailnet 미리보기에 반영한 기록이다. GitHub Pages는 배포하지 않았다.

## 입력과 범위

- 기준 `HEAD`: `da67a18032f449d8477cdd5d7ea0aba02e691b0b` (`origin/main`과 일치)
- 선택 문서 변경 SHA-256: `be1e439f3c490efd848103eab307fad3497fa09506ccb1cb1590ffcbf49de8d3`
- PR #1176 merge SHA: `da67a18032f449d8477cdd5d7ea0aba02e691b0b`
- 공식 로드맵 집계: `78/214`; C11은 미완료로 유지
- RSPress `2.0.22`에서 선택된 C11 문서와 evidence route만 빌드했다. 이 빌드는 전체 문서 링크 검증이 아니며 dead-link/dead-image 검사는 꺼져 있다.
- 선별 route 목록·크기·SHA-256은 `selected-publication*.json`에, 복사 전 조건과 이전 파일 hash는 `preflight*.json`에, 백업·복사 결과는 `copy-and-backup*.json`에, local/HTTPS 응답 검증은 `served-verification*.json`에 기록했다.

## 동기화 결과

Tailscale backend가 `Running`이고 기존 preview server PID `28040`이 `127.0.0.1:8768`에서 응답하는 것을 확인했다. 변경 전 대상 파일을 `lsof`로 확인했으며 교체 route에 열린 파일 handle은 없었다. 교체 파일은 고유한 backup directory에 SHA-256 검증 후 보관하고 임시 파일을 만든 뒤 atomic rename으로 반영했다. 어떤 기존 route도 삭제하지 않았고 preview server를 재시작하지 않았다.

선택된 모든 route는 local preview와 HTTPS Tailnet URL에서 HTTP 200 및 publication SHA-256이 일치하는지 확인했다. 기존 preview server PID를 유지한 상태에서 확인했다. PR #1176 카드에는 제한 입력 범위, merge metadata, 공식 `78/214`, C11 미완료 및 시뮬레이터 증거의 한계를 표시한다.

## Agent Browser 확인

사용자가 지정한 Agent Browser에서 [Tailnet 로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)을 열어 PR #1176 카드, merge SHA, 공식 완료 수를 확인했다. 데스크톱 뷰포트는 1440×1000이며 가로 넘침이 없었다. 모바일은 390×844에서 카드 폭 362px, 좌우 14px 여백, 문서 폭 390px로 가로 넘침이 없었다. 실제 화면은 [desktop 캡처](./tailnet-roadmap-agent-browser-desktop.png)와 [mobile 캡처](./tailnet-roadmap-agent-browser-mobile.png)에 보존했다. viewport와 카드 bounds는 [검증 JSON](./agent-browser-verification.json)에 기록했다.

확인 뒤 `agent-browser close --all`로 default session을 닫았다. Agent Browser daemon PID `59128`은 종료되어 후속 process inspection에서 보이지 않았고, Chrome for Testing 프로세스도 남지 않았다. 종료 확인은 [lifecycle JSON](./agent-browser-lifecycle.json)에 기록했다. tmux 상태는 건드리지 않았다.

## 검증 한계

이 게시물은 선택한 문서 route와 자산만 포함한다. 전체 RSPress 링크 검증, CI 성공, GitHub Pages 게시를 뜻하지 않는다. 카드에 있는 simulator 캡처는 visual smoke이며 Android PixelCopy, iOS Metal drawable readback, 실기기/hardware GPU, iOS compositor 이후 pixels 또는 WPT 적합성 증거가 아니다.

## 재현·검토 파일

같은 evidence directory의 `prepare-selected-root-pass02.mjs`, `rspress-pass02.config.ts`, `stage-publication-pass02.mjs`, `sync-tailnet-pass02.mjs`가 최종 route 목록을 준비하고 검증한다. 저장소의 PR diff에서 이 경로를 열어 재현할 수 있다. [병합 후 문서와 동기화 PR 변경 검토 30개](./pr-diff-review-30.md)

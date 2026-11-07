# C11.4 진행 상태 로드맵 동기화 · 2026-11-08

## 변경 이유

공개 Tailnet 로드맵의 마지막 진행 카드는 이미 병합된 PR #1270 mixed track-size list를 가리켰다. 그 다음 `auto` list-member 구현은 로컬 feature branch에서 시작됐지만 진행 카드가 없어 현재 상태를 찾기 어려웠다. 공식 상태는 완료 판정 원본이므로 `spec/STATUS.md`와 C11.4 체크는 바꾸지 않고, 계획 및 공개 로드맵의 진행 문구만 맞췄다.

작업 계획은 이제 제한 구현 진행 중으로 표시한다. 로드맵 카드에는 Chrome 157 사전 관측 3 scene과 독립 기대값, 계획 실패 관점 20개 검토 완료, style projection/layout admission 시작, RuntimeSession·거부/복구·Android/iOS Simulator·구현/runtime 검토 미완료를 명시했다. 미병합 상태 및 공식 `78/214`도 표시한다.

## Tailnet 제공본 동기화

Tailscale backend는 `Running`, `macstudio.tailed42f2.ts.net` preview는 기존 PID `28040`으로 동작했다. 변경 대상은 `/spinon/roadmap.html` 한 경로뿐이었다. 교체 전에 해당 파일의 열린 handle이 0개인지 확인하고 기존 파일을 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-c114-auto-track-progress-2026-11-08/pass01/roadmap.html`에 hash 검증과 함께 백업했다. 소스·제공본 SHA-256은 `7144c930d7c41a5e5614a3aa7e2db4a20a7c273b25bb76a129e36c1ab3e8ede5`로 일치한다. 파일 삭제와 preview server 재시작은 없었다.

시각 확인은 Agent Browser로 실제 Tailnet URL을 열어 수행했다. 데스크톱 1280px에서 viewport/document 폭이 일치하고, 모바일 390px에서도 document/body 폭이 390px로 일치해 가로 넘침이 없었다. 두 화면에서 진행 카드와 Kanban의 `진행 중` 열에 `fit-content 목록`이 표시됐다. 확인 후 해당 Agent Browser session을 닫았고, Agent Browser/Chrome 프로세스가 남지 않았음을 확인했다.

## 근거

- [사전 교체 검사](./preflight.json)
- [백업 및 복사 결과](./copy-and-backup.json)
- [Agent Browser DOM·viewport·화면 hash](./agent-browser-verification.json)
- [데스크톱 화면](./tailnet-agent-browser-desktop.png)
- [모바일 화면](./tailnet-agent-browser-mobile.png)
- [동기화 스크립트](./sync-tailnet-roadmap.mjs)

이 로드맵 동기화는 구현 완료, PR 병합, Android/iOS runtime 검증 또는 C11.4 완료를 뜻하지 않는다.

## pass02 · focused Rust 검증 반영

이후 Layout track-list 2/2, style projection 1/1, RuntimeSession auto-track 2/2 및 기존 singleton 회귀 2/2, 세 변경 crate all-target Clippy와 rustfmt가 통과했다. 로드맵 진행 카드를 이 결과로 갱신했다. Android API 37 AVD·iPhone 17 Pro/iOS 26.2 Simulator 및 별도의 구현/runtime 검토 20개, PR diff 검토 30개는 여전히 남아 있다.

두 번째 단일-route 교체 전후에도 Tailscale backend `Running`, 같은 preview PID `28040`, target open handle 0개를 확인했다. pass01 제공본을 pass02 backup에 보존했고, 최신 source/preview SHA-256은 `22e2502b9316e7e66e04d232ce98bf87d622106fef9787f861aa9e9986ef220c`로 일치한다. 삭제·서버 재시작은 없었다. Agent Browser에서 데스크톱 1280×1200과 모바일 390×844를 확인했고, 모바일 document/body 폭은 390px로 일치했다. session을 닫은 뒤 Agent Browser/Chrome process 0개를 확인했다.

- [pass02 사전 교체 검사](./preflight-pass02.json)
- [pass02 백업 및 복사 결과](./copy-and-backup-pass02.json)
- [pass02 Agent Browser DOM·viewport·화면 hash](./agent-browser-verification-pass02.json)
- [pass02 데스크톱 화면](./tailnet-agent-browser-desktop-current-status-pass02.png)
- [pass02 모바일 화면](./tailnet-agent-browser-mobile-current-status-pass02.png)
- [pass02 동기화 스크립트](./sync-tailnet-roadmap-pass02.mjs)

## pass03 · Android/iOS Simulator와 구현/runtime 검토 반영

Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기본 Root HTML route `layout=ready boxes=12` 및 fixture hash를 확인했고, 구현/runtime 실패 관점 20개 검토를 끝낸 뒤 공개 진행 카드를 갱신했다. 공식 C11.4/C11 체크와 완료 `78/214`는 유지했다.

Tailnet preview는 Tailscale `Running`, 기존 PID `28040`으로 계속 제공됐다. `/spinon/roadmap.html` 한 파일만 갱신했으며 pass02 제공본은 [`pass03 backup`](</Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-c114-auto-track-progress-2026-11-08/pass03/roadmap.html>)에 보존했다. 이전 제공본 SHA-256 `22e2502b9316e7e66e04d232ce98bf87d622106fef9787f861aa9e9986ef220c`, 새 source/served hash `8099e56cb3905b3ad32dafbb90cb0250eef5945446207838b8ad920a0c35e3ce`가 일치한다. 서버 재시작은 없었다.

Agent Browser로 실제 Tailnet URL을 확인했다. 데스크톱 1280×1200에서 document/body 폭이 1280px이고 진행 카드 text가 최신 상태를 표시한다. 모바일 390×844에서는 document/body 폭이 390px이며, 진행 카드가 화면에 보이고 가로 overflow가 없다. 캡처 뒤 세션을 닫았으며 Agent Browser/Chrome for Testing/Chromium/headless_shell process가 남지 않았다.

- [동기화·backup hash](./tailnet-sync-pass03.txt)
- [desktop DOM/width](./agent-browser-desktop-current-status-pass03.json) · [desktop screenshot](./tailnet-agent-browser-desktop-current-status-pass03.png)
- [mobile DOM/width](./agent-browser-progress-card-mobile-pass03.json) · [mobile current-card screenshot](./tailnet-agent-browser-mobile-progress-card-pass03.png)
- 앞선 pass03 responsive viewport 캡처: [desktop](./tailnet-agent-browser-desktop-c114-auto-track-progress-pass03.png) · [mobile](./tailnet-agent-browser-mobile-c114-auto-track-progress-pass03.png)


## pass04/pass05 · PR diff 검토 완료 상태와 짧은 진행 카드

PR diff 실패 관점 30개 검토 완료 상태를 진행 문구에 반영했다. pass04의 카드 제목 `fit-content + auto`는 칸반에서 말줄임표 처리되어 실제 slice가 잘 보이지 않아 pass05에서 `auto member`로 줄였다. 현재 보드의 진행 열은 `auto member`와 `C11.4 · ACTIVE`를 표시하고, 상단 기록은 PR 생성·CI/check 결과 확인·merge가 남았다고 설명한다. 공식 완료는 `78/214` 그대로다.

Tailnet은 `Running`, preview PID `28040` 그대로였고 `/spinon/roadmap.html`만 교체했다. pass04 source/served SHA `0e63ba843a98768d04b7bd3dc963e9dddddbc6d416a81f20c8cd12f5c2630d95`; pass05 최종 source/served SHA `1d858acee89e244073e50e2d40bf12ca1146cca2563fd7b280a07382910c0d59`. 이전 제공본은 각각 `pass04` 및 `pass05` backup에 보존했으며 preview 재시작이나 삭제는 없었다.

Agent Browser에서 실제 Tailnet desktop 1280×1200 및 mobile 390×844를 확인했다. Desktop body/document 폭은 1280px, mobile body/document 폭은 390px이고 mobile kanban은 `진행 중` 열에 맞춰졌다. 세션을 닫은 뒤 session `active=false`, pid `null`, agent-browser CLI process 없음으로 확인했다. 사용자 소유/기존 Chrome 프로세스는 그대로 보존했다.

- [pass04 사전 검사](./preflight-pass04.json) · [백업/동기화](./copy-and-backup-pass04.json) · [칸반 화면](./tailnet-agent-browser-desktop-current-status-pass04.png)
- [pass05 사전 검사](./preflight-pass05.json) · [백업/동기화](./copy-and-backup-pass05.json) · [desktop](./tailnet-agent-browser-desktop-current-status-pass05.png) · [mobile](./tailnet-agent-browser-mobile-current-status-pass05.png)
- [pass05 DOM·폭 확인](./agent-browser-desktop-current-status-pass05.json) · [mobile 확인](./agent-browser-mobile-current-status-pass05.json) · [process 상태](./agent-browser-state-pass05.txt)
- [pass04 sync script](./sync-tailnet-roadmap-pass04.mjs) · [pass05 sync script](./sync-tailnet-roadmap-pass05.mjs)

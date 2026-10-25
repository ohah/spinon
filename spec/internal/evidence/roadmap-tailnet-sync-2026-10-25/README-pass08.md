# Tailnet roadmap sync · pass08

2026-10-25에 PR [#624](https://github.com/ohah/spinon/pull/624) 병합 후의 `origin/main` (`f197239b0168e2356f4d0ece03f5d2e4c40278fe`)을 기준으로 Tailnet 미리보기를 갱신했다. 공식 C11.4 checkbox는 미완료이고 완료 집계는 `72/213`이다.

## 선택 build와 route

- Rspress pass08 selected build는 custom config로 완료됐다. dead-link/image 검사는 끈 선택 build라 전체 `docs:build` 성공으로 해석하지 않는다. 전체 문서 build의 이전 실패 원인은 저장소 전역 dead-link 검사다.
- pass07 선택 source root를 hardlink한 뒤 현재 main의 STATUS, roadmap, C11.4 계획과 cyclic runtime evidence를 overlay했다. 입력·overlay hash는 [source manifest](selected-root-pass08.json)에 있다.
- 성공한 stage는 24개 생성 페이지를 포함해 262 routes, 25,971,098 bytes를 선택했다. Route별 크기/SHA-256은 [selected build manifest](selected-build-verification-pass08-retry03.json)에 있다. 기존 route를 보존했고 삭제는 없다.
- Marker 검증을 보정하면서 만든 부분 선택 output은 그대로 보존했다. 최종 선택 root는 `build/spinon/roadmap-tailnet-sync-20261025-site-pass08-selected-retry03-retry02`다. 선택 script와 결과 manifest가 성공 기준이다.

## Tailnet 반영과 검증

- Preflight 시 Tailscale backend는 `Running`, Mac Studio는 online, 미리보기는 `127.0.0.1:8768`에서 수신했고 Tailnet roadmap HEAD 요청은 HTTP 200이었다. 초기 preflight에서 서버 PID 1180이 교체 대상에 오래된 read handle 하나를 남긴 것을 확인했다. 해당 서버 프로세스만 SIGTERM으로 재시작했고 PID 28040이 같은 포트에서 다시 응답했다. tmux 세션은 건드리지 않았다.
- 262 route 중 171개 생성, 27개 교체, 64개 동일 파일이었다. 교체 전 파일 27개는 [pass08 backup root](</Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-2026-10-25/pass08>)에 byte-exact로 보존했다. 교체 파일은 사전 lsof 확인 뒤 임시 파일 fsync와 원자 rename으로 갱신했다. [preflight](tailnet-preflight-pass08.json) · [교체 대상 handle 검사](target-open-handles-pass08.json) · [sync 결과](tailnet-copy-pass08.json)
- 최종 검사에서 local preview 262/262, backup 27/27, Tailnet HTTP 200 및 response body SHA-256 262/262가 일치했다. [최종 검증 manifest](tailnet-verify-pass08.json)

## Agent Browser 확인

- Agent Browser로 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`을 열었다. DOM에서 PR #623 병합 기록, `72/213`, orthogonal row blocker를 확인했다. 화면 캡처는 [Tailnet roadmap](tailnet-roadmap-agent-browser-pass08.png)이며 핵심 DOM 검사는 [관측 manifest](tailnet-roadmap-agent-browser-pass08.json)에 있다.
- 실행 후 `agent-browser close --all`을 호출했다. 세션 목록은 비었고 Agent Browser process도 남지 않았다. 이 확인은 터미널 브라우저를 사용하지 않았다.

## 범위

- 이 작업은 docs preview sync이며 제품 코드·공식 완료 체크·완료 집계·내부 계약 버전을 바꾸지 않는다.
- Tailnet route hash 일치는 선택한 262개 파일에 대한 확인이다. 선택 build는 저장소 전체 dead-link 검사를 통과하지 않았고, 이번 단계에서 테스트나 simulator 실행을 하지 않았다.

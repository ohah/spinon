# C11.4 two-track ceiling PR #837 Tailnet roadmap sync · 2026-10-30

PR [#837](https://github.com/ohah/spinon/pull/837)와 로드맵 문서 PR [#838](https://github.com/ohah/spinon/pull/838)을 병합한 뒤 공개 로드맵·상태 페이지와 C11.4 계획/evidence 경로를 Spinon Tailnet 미리보기에 동기화했다. GitHub Pages 배포가 아니다.

## 병합 원본과 선택 빌드

- PR #837 test-only 구현 merge SHA: `12c11815165077d570d4d9c1674b0785047cba67` (2026-10-30T03:35:31Z).
- PR #838 공개 로드맵 merge SHA: `e886dfebccc406ac6f7303a462b228ef3f258248` (2026-10-30T03:40:22Z).
- 선택 RSPress 빌드는 `origin/main` `e886dfebccc406ac6f7303a462b228ef3f258248`에서 21개 source와 43개 staged route를 사용했다. RSPress 2.0.22가 성공적으로 102개 publication route, 11,589,066 bytes를 만들었다.
- 이 선택 빌드는 `checkDeadLinks`/`checkDeadImages`를 끈 RSPress 설정을 사용했다. 전체 문서 dead-link 검증 또는 전체 문서 build 성공을 뜻하지 않는다. 선택 publication에서 로드맵, 상태, C11.4 계획과 새 evidence route를 직접 검사했다.
- 처음 stage 시도는 `build/spinon` 부모 디렉터리가 없어 `ENOENT`로 끝났고 새 산출물은 만들지 않았다. base 경로 생성을 재귀로 바꿔 재실행했다. 첫 Bun 호출도 script 인자 구분자를 누락해 help만 출력했다. 올바른 `bun run --filter @spinon/docs docs:build -- --config ...` 호출은 통과했다.
- publication 첫 판정은 현재 문서 head가 PR #837 merge SHA로 기록된다는 점을 놓쳐 잘못된 marker 기대값에서 중단했다. 실패 staging 디렉터리는 보존하고 pass02 publication을 새 경로에 만들었다. 올바른 PR merge SHA marker로 검증했다.

## Tailnet 미리보기 동기화

- 미리보기 서버 PID 28040은 `127.0.0.1:8768`에서 계속 수신 중이었다. 서버를 재시작하거나 종료하지 않았다.
- preflight는 102개 경로를 분류했다: 신규 69, 교체 15, 동일 18. 삭제는 없었다.
- 교체 대상 15개, 기존 5,650,761 bytes는 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr837-2026-10-30/pass02` 아래 byte-exact 백업을 확인했다.
- 복사 직후 target 102/102, backup 15/15 SHA-256 검증이 통과했다. `open_handle_observations`는 비어 있어 교체 전에 비-preview 프로세스가 대상 파일을 열고 있지 않았다.
- HTTPS `https://macstudio.tailed42f2.ts.net/spinon/`에서 선택한 102개 route 모두 HTTP 200이며 응답 크기와 SHA-256이 publication과 일치했다(102/102). 서버는 그대로 실행 중이다.

## Agent Browser 화면 확인

터미널 브라우저를 사용하지 않았다. Agent Browser에서 로드맵, 상태, 새 계획 페이지를 열어 DOM 내용과 링크를 확인했다.

- [로드맵 화면](./tailnet-roadmap-agent-browser-pass02-viewport.png)은 최신 progress card로 PR #837, merge SHA, 두-track 입력, Rust test count와 `72/213`을 표시한다. 계획, 사전 비교, 실행 근거, 공식 상태 링크도 확인했다.
- [C11 상태 화면](./tailnet-status-c11-agent-browser-pass02-viewport.png)과 [상태 평가 결과](./agent-browser-status-pass02.json)는 공식 `72/213`, C11=false, C11.3=false, C11.3.3=false, C11.4=false 및 C11.2.2/C11.2.3=true를 확인한다.
- [로드맵 평가 결과](./agent-browser-roadmap-pass02.json)와 [계획 페이지 확인](./agent-browser-plan-pass02.json)은 문서 제목, PR #837 링크, 계획/evidence route를 기록한다.
- 두 개의 격리 Agent Browser 세션을 종료했다. [종료 결과](./browser-close-pass02.json)는 두 세션 모두 `active=false`, `pid=null`이다. 다른 세션은 닫지 않았다.

## 재현 산출물

- [선택 root 및 hash](./selected-root-pass01.json) · [RSPress 설정](./rspress-pass01.config.ts) · [RSPress build log](./tailnet-build-pass01.log)
- [선택 publication 및 route hash](./selected-publication-pass02.json) · [publication 첫 판정 실패](./publication-summary-pass01.log) · [정정 후 요약](./publication-summary-pass02.log)
- [preflight 및 백업 기준](./preflight-pass02.json) · [copy와 backup 결과](./copy-and-backup-pass02.json) · [로컬 검증](./local-verification-pass02.json)
- [HTTPS served hash 검증](./served-verification-pass02.json) · [요약](./served-verification-summary-pass02.log)
- [pass01 stage script](./stage-root-pass01.mjs) · [pass01 publication script](./select-publication-pass01.mjs) · [성공한 pass02 publication script](./select-publication-pass02.mjs) · [Tailnet copy/verify](./sync-tailnet-pass02.mjs) · [HTTPS verify](./verify-tailnet-pass02.mjs)

각 동기화 스크립트는 source head·preflight hash·target identity를 대조하며 파일 삭제는 수행하지 않는다. pass01 오류 산출물과 pass02 성공 산출물을 모두 보존했다.

[이 Tailnet 동기화 PR의 변경 적대 검토 30개](./tailnet-sync-pr-diff-review-30.md)는 스크립트 안전성, hash 경계, 브라우저 화면 증거와 공식 상태가 서로 일치하는지 별도로 확인한다.

# C11.4 text-transform 병합본 Tailnet 로드맵 동기화

2026-11-01에 Tailnet 정적 미리보기가 PR #951의 계획 전용 화면을 계속 제공하는 것을 Agent Browser로 확인했다. 저장소 `main`에는 text-transform runtime PR #952와 상태·공개 로드맵 PR #953/#954가 병합된 상태였으므로, 현재 `origin/main`의 공개 문서와 추적되는 근거를 다시 렌더링해 미리보기를 갱신했다. 공식 구현 완료는 `72/213`이며 C11.4/C11은 미완료다.

## 동기화 범위와 결과

- 소스 HEAD `43db168f26ecaaf6eb799692359e3f3861dcb546`, tree `cdf6e67755a4361f7d50056dc8259556dcd80f70`에서 Rspress `2.0.22` 선택 문서 빌드를 만들었다. 저장소 전체 dead-link 검사는 실행하지 않았다.
- 기존 미리보기 root는 `/Users/yoonhb/Library/Application Support/SpinonPreview`이며, Tailscale backend `Running`, PID `28040`, listener `127.0.0.1:8768`을 확인했다. 서버는 재시작하지 않았다.
- 선택 publication 306 route, 25,498,629 bytes를 반영했다: 106개 생성, 19개 교체, 181개 동일. 교체 전 19개 파일은 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr954-2026-11-01/pass01` 아래에 보관했다. route 삭제는 없었다.
- 전체 306개 route의 미리보기 파일 hash가 publication manifest와 일치했고, 19/19 backup hash도 원본과 일치했다. Tailnet HTTPS는 306/306 HTTP 200과 SHA-256 일치를 반환했다.
- PR #952 카드의 내부 링크 9개는 Agent Browser에서 모두 HTTP 200을 반환했다. 카드에는 제한 runtime slice, 병합 SHA, Android/iOS simulator 범위, 공식 `72/213`과 미완료 상태가 보이고 `plan-only` 문구는 없다.
- Agent Browser에서 데스크톱 1280×900, 모바일 390×844, 계획 페이지를 확인했다. 두 화면 모두 가로 overflow가 없었다. 확인 후 세션을 닫았고 `agent-browser` 및 그 Chrome 프로세스가 남지 않았다.

이 publication은 미리보기 문서와 근거 파일 동기화다. CSS 전체 적합성, 전체 문서 링크 적합성, GitHub CI 성공 또는 C11 완료를 뜻하지 않는다.

## 원본 기록

- [선택 root manifest](selected-root.json)
- [Rspress build 로그](selected-build.log)
- [선택 publication manifest](selected-publication-tailnet-pass01.json)
- [사전 검사와 열린 파일 검사](tailnet-preflight-pass01.json)
- [복사와 백업 결과](tailnet-copy-and-backup-pass01.json)
- [로컬·Tailnet 전체 hash 검증](tailnet-served-verification-pass01.json)
- [Agent Browser 확인값](agent-browser-verification.json)
- [데스크톱 화면](roadmap-agent-browser-desktop.png)
- [모바일 화면](roadmap-agent-browser-mobile.png)
- [선택 root 구성 스크립트](prepare-selected-root.mjs)
- [publication staging 스크립트](stage-publication.mjs)
- [Tailnet 동기화 스크립트](sync-tailnet.mjs)
- [PR 변경 검토 30개](pr-diff-review-30.md)

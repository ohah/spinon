# C11 통합 계획 Tailnet 동기화 · PR #956 이후

2026-11-01에 PR [#956](https://github.com/ohah/spinon/pull/956) 병합 뒤 `plan/c11-css-grid.md`와 `plan/c11-4-grid-track-sizing.md`를 Tailscale 미리보기와 대조했다. 공식 집계는 `72/213`이며 C11, C11.3, C11.4는 미완료다. C11.2.2와 C11.2.3의 제한 profile 체크 상태는 바꾸지 않았다.

## 동기화 결과

- 소스 `main`은 `00f7f316abf515b21fca3b93ad2360b2c203a98f`, tree는 `358534b97890729a501e35ba08435e181e129f12`였다. Rspress `2.0.22`로 선택 문서를 다시 빌드했다.
- 첫 publication은 342개 route를 갱신했지만, 이후 계획 링크 감사를 통해 PR #948의 overflow-wrap 계획·근거 route가 선택 root에 없음을 발견했다. 페이지 링크를 완전하게 맞추기 위해 해당 계획, precomparison, Android/iOS runtime, artifact-boundary 파일을 모두 선택 root에 추가했다.
- 최종 선택 publication은 529개 route, 39,689,826 bytes다. 직전 publication 대비 187개 추가, 26개 교체, 316개 동일이며 삭제는 없다. Tailnet preflight에서는 63개 생성, 35개 교체, 431개 동일로 분류했다. 변경 전 열려 있는 대상 파일은 없었다.
- 최종 복사에서는 기존 파일 35개를 별도 backup root에 보존했다. local target 529/529 hash, backup 35/35 hash, HTTPS 529/529 HTTP 200 및 route SHA-256 일치가 확인됐다. Tailscale backend는 Running이었고 preview server PID 28040은 재시작하지 않았다.
- pass02 복사는 이미 만들어진 backup 상위 폴더에서 `EEXIST`로 route 복사 전에 멈췄다. pass02와 재시도 pass03 preflight의 529개 대상 존재·hash·action snapshot이 같음을 확인한 뒤, 새 backup 경로를 쓰는 pass03으로 완료했다. 실패 전후 Tailnet 파일 변경은 없었다.

## 화면과 링크 확인

- Agent Browser에서 C11 계획과 C11.4 track sizing 계획을 1280×900 및 390×844로 열었다. 네 화면 모두 document width가 viewport와 같았고 수평 overflow가 없었다.
- 두 페이지에서 `overflow-wrap`, `text-transform`, PR #948/#952, `72/213`, 미완료 상태를 확인했다. C11.4 계획에서 overflow-wrap 계획과 runtime README 링크를 눌러 대상 문서 제목까지 확인했다.
- PR #957 본문에 포함한 C11.4 desktop/mobile image가 GitHub 화면에서 각각 1280×900, 390×844의 natural size로 로드됐다.
- 집중 링크 감사는 두 계획 페이지의 overflow-wrap/text-transform 관련 링크 11개와 C11 overview의 C11.4 상세 계획 링크를 확인했다. PR #948 계획, Chrome precomparison, runtime README, 20/30-point reviews와 PR #952 계획/evidence route가 publication에 들어 있다. 전체 문서 dead-link 검사는 실행하지 않았다.
- 확인 후 Agent Browser 세션을 모두 닫았다. 프로세스 검사에서 `agent-browser` 실행 파일은 남지 않았고, 별도로 실행 중이던 Tailscale preview server는 계속 유지됐다.

이 작업은 문서와 미리보기 동기화다. C11 구현 범위, 명세, `spec/STATUS.md` 체크박스, 공식 완료 수는 변경하지 않았고 GitHub CI 성공을 주장하지 않는다. PR #948/#952 구현 PR의 check rollup은 보고되지 않았다.

## 주요 근거

- [선택 root 구성](prepare-selected-root-pass02.mjs) · [Rspress 설정](rspress-pass02.config.ts) · [build 결과와 생성 page hash](selected-build-result.json)
- [선택 root manifest](selected-root-pass02.json) · [선택 publication manifest](selected-publication-tailnet-pass02.json) · [관련 plan link audit](plan-link-route-audit-pass02.json)
- [최종 Tailnet 사전 검사](tailnet-preflight-pass03.json) · [복사와 backup](tailnet-copy-and-backup-pass03.json) · [전체 hash 검증](tailnet-served-verification-pass03.json)
- [Agent Browser 측정과 링크 탐색](agent-browser-verification.json)
- [PR 변경 30개 검토](pr-diff-review-30.md)
- 화면: [C11 desktop](c11-plan-desktop-agent-browser-pass02.png), [C11 mobile](c11-plan-mobile-agent-browser-pass03.png), [C11.4 desktop](c114-plan-desktop-agent-browser-pass02.png), [C11.4 mobile](c114-plan-mobile-agent-browser-pass02.png)

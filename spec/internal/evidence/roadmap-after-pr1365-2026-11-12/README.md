# PR #1365 이후 C11.4 로드맵 갱신

## 변경 내용

- 공개 로드맵의 최근 병합 요약을 PR #1360에서 PR #1365로 갱신했다.
- six-fit row-span은 RuntimeSession 11,520/11,520 scenes, candidate payload-floor violation 0건, baseline 4,800건으로 제한해 기록한다.
- Android/iOS 기본 Root HTML light/dark smoke는 six-fit fixture geometry 확인이 아니라고 표시한다.
- GitHub checks/workflows 0건, `spinon-layout --lib` 544 passed / 1 unchanged C15 assertion failure를 그대로 노출한다.
- C11.4/C11 미완료와 공식 `78/214`를 유지한다. 다음 bounded feature candidate는 아직 선택하지 않았고, 잔여 Grid scope 검토 뒤 선정하도록 표시한다.
- 순차 작업 계획도 PR #1365 병합 완료로 동기화했다.

## Agent Browser 사전 확인

Candidate branch의 `spec/public/roadmap.html`을 로컬 HTTP origin에서 Agent Browser로 열었다. 데스크톱 1440×1000에서 summary 영역 1068.2px, 전체 document/body 1440px였고, 모바일 390×844에서 전체 document/body 390px였다. 두 화면에서 공식 완료 `78/214`, C11.4/C11 미완료, 다음 candidate 미선정 상태와 보드 카드가 표시됐다. 모바일 칸반은 기존 동작대로 자체 가로 스크롤을 사용한다.

- [`roadmap-desktop-agent-browser.png`](./roadmap-desktop-agent-browser.png), SHA-256 `db0848bc54e11a6c9d15bdd31268a7b5506b92304709821685cfddcbe5dc2ba0`
- [`roadmap-mobile-agent-browser.png`](./roadmap-mobile-agent-browser.png), SHA-256 `e20017d12605beba7e6be75e0ad2edaf55f7636c88b1a5ad288209f9bc6ae7c2`
- [`pr-diff-review-30.md`](./pr-diff-review-30.md)

The Tailnet static preview still requires a post-merge synchronization and verification; it is tracked separately from this candidate-page check.


## PR #1366 병합과 Tailnet 동기화

PR [#1366](https://github.com/ohah/spinon/pull/1366)은 Rebase and merge로 병합됐다. Merge SHA는 `5a7ed74d197c78d287cc12a0823a2abc77050f3f`다. Tailnet server는 `SpinonPreview`의 정적 파일을 서빙하므로 main이 바뀌어도 복사본은 자동 갱신되지 않는다. server PID `90711` / port `8768`은 재시작하지 않고 유지했다.

- 선택 Rspress 2.0.22 빌드: 8개 지정 페이지와 14개 참조 static asset 확인. 전체 docs build와 전체 dead-link 검사는 실행·주장하지 않는다.
- `tailnet-copy-manifest.json`: 89개 경로 확인, 69개 추가·교체(그중 기존 파일 14개를 전용 backup에 보존), 20개 동일, 삭제 0개.
- `served-verification.json`: source/preview/local HTTP/Tailnet HTTPS SHA-256이 89/89 경로에서 일치하고, 두 HTTP 경로 모두 89/89 응답 200이며 backup 14/14 hash가 맞다.
- [실제 Tailnet 화면 검증](./tailnet-agent-browser-verification.json): Agent Browser desktop 1440×1000, mobile 390×844에서 official 78/214, C11.4/C11 미완료, PR #1365 latest slice, next candidate 미선정을 확인했다. 두 viewport에서 page-level horizontal overflow는 없다.
- 실제 반영 화면: [desktop](./tailnet-roadmap-desktop-agent-browser.png), [mobile](./tailnet-roadmap-mobile-agent-browser.png).
- 완료 뒤 task-created Agent Browser 세션과 임시 HTTP server를 종료했다. iOS Simulator에는 booted device가 없었다. 사용자 지정 Tailnet preview server는 계속 실행 중이다. 세부 확인은 [process cleanup record](./browser-session-lifecycle.json)에 있다.

선택 빌드 결과 디렉터리 `build/spinon/roadmap-after-pr1365-2026-11-12`는 Tailnet hash와 화면 검증 후 제거했다. 설정, 입력 manifest, build log, output page hash와 Tailnet backup은 재현·rollback 근거로 보존한다.

- 별도 [Tailnet sync PR diff 검토 30개](./tailnet-sync-pr-diff-review-30.md)에서 실제 복사 변경을 검토했다. [Agent Browser process cleanup](./browser-session-lifecycle.json) 기록도 보존한다.

# Tailnet 로드맵 동기화 · 2026-10-28 · PR #718 이후

## 반영한 상태

문서 동기화 PR [#718](https://github.com/ohah/spinon/pull/718)은 2026-10-27T22:06:31Z에 Rebase and merge됐고 merge SHA는 `ad0106b6b1a6160bb601b4a407c377c8046e9415`다. 이 선택 publication은 그 시점의 `main` HEAD와 tree를 사용한다. C11.4 §11.7 test-only 구현 PR [#717](https://github.com/ohah/spinon/pull/717)은 `c29482aa8fc3ac39cdc342eae138c3ad10a59683`로 병합됐다. 공식 C11.4 checkbox는 미완료이고 집계는 `72/213`이다. 두 PR의 GitHub check rollup은 비어 있어 CI 통과로 기록하지 않는다.

## 선택 빌드와 경로 범위

- 저장소 고정 Rspress `2.0.22`로 관련 계획, 공식 상태, 사전 비교·실행 근거와 검토 문서를 선택해 빌드했다. [공백 정리 build log](rspress-build.log)와 [원본 gzip log](rspress-build.raw.gz)에서 21개 tracked source, 43개 staged source route, 성공한 Rspress 실행을 확인할 수 있다.
- 최종 publication은 생성 페이지·정적 자산과 선택 원본을 합쳐 96개 route, `8,364,860` bytes다. [source manifest](selected-root.json)와 [publication manifest](selected-publication.json)에 `main` SHA/tree, route별 크기와 SHA-256이 있다.
- 새 공개 로드맵 문단의 여섯 Tailnet 로컬 링크는 publication 안의 새 계획·비교·근거·검토 페이지에 도달한다. README의 Rust test source 링크와 Chrome 사전 비교 PNG도 선택했다.
- Rspress의 dead-link/dead-image 검사는 비활성화했다. 전체 문서 빌드나 전체 링크 그래프 검사는 수행하지 않았고 manifest의 `fullDocsLinkValidation`은 `false`다.

## Tailnet 상태와 동기화

- 사전 요청은 HTTP 200, 353,418 bytes, SHA-256 `5adfb59f9c4823e099fa74ad3b276edc06750fd2f4f94cf8aa62bcf278fc3bd8`였다. Tailscale backend는 `Running`, Mac Studio는 `online`이었고 미리보기 서버 PID `28040`이 `127.0.0.1:8768`에서 수신 중이었다. [preflight](tailnet-preflight.json)
- 교체 경로 12개에는 open handle이 없었다. 사전 대조 후 96개 route 중 70개를 만들고 12개를 교체했으며 14개는 이미 같았다. route를 삭제하지 않았다. 교체 전 파일 12개 `4,115,888` bytes는 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr718-2026-10-28/pass01`에 byte-exact 보존했다. [copy report](tailnet-copy.json)
- 게시 뒤 로컬 파일과 HTTPS 응답 모두 96/96개가 HTTP 200 및 manifest의 크기·SHA-256과 일치했고, 12개 백업도 모두 byte-exact로 확인했다. 로드맵, `STATUS.html`, 집중 계획에서 #717, merge SHA와 `72/213` 표식을 확인했다. 서버 PID는 전후 `28040`으로 같아 재시작하지 않았다. [verification report](tailnet-verify.json)

## Agent Browser 확인 및 종료

- Terminal Browser는 사용하지 않았다. 격리된 Agent Browser session에서 Tailnet [로드맵](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html), [집중 계획](https://macstudio.tailed42f2.ts.net/spinon/plan/c11-4-flex-stage-oracle.html), [공식 상태](https://macstudio.tailed42f2.ts.net/spinon/STATUS.html)를 확인했다. 로드맵 DOM에서 새 #717 항목과 로컬 링크 6개를 확인했고, 집중 계획·공식 상태에서 merge SHA와 `72/213`, 미완료 상태를 확인했다. 오래된 PR 생성 대기 문구는 없었다.
- 첫 화면 캡처(pass01)는 DOM을 읽었지만 CSS smooth scroll 때문에 이전 위치에 남아 새 카드 화면을 보여주지 못했다. 원본을 보존하고 scroll behavior를 즉시로 바꾼 뒤 다시 이동해 만든 [pass02 화면](tailnet-roadmap-agent-browser-pass02.png)은 1280×633이며 #713과 새 #717 문단을 함께 보여준다(SHA-256 `4602cb026e78172e141b1d15f84131a38d765ab8bd9f5e8cd100ec0d9daa56eb`). 두 시도와 계획·상태 관측은 각각 [roadmap pass01](tailnet-roadmap-agent-browser-pass01.json), [roadmap pass02](tailnet-roadmap-agent-browser-pass02.json), [계획](tailnet-plan-agent-browser.json), [공식 상태](tailnet-status-agent-browser.json)에 남겼다.
- `close` 명령은 성공했다. 후속 `session info`가 일시적으로 `active: true`, PID `58992`를 보고했지만 그 PID는 프로세스 목록에 없었고 마지막 `session list`도 비었으며 해당 session의 Chrome remote-debugging 프로세스도 없었다. [최종 lifecycle 확인](agent-browser-lifecycle.json)과 이 원시 응답은 [별도 보존](agent-browser-session-info-after-close.json)했다.

## 확인하지 않은 범위

이 작업은 선택한 정적 문서 publication과 Tailnet 제공 경로를 확인했다. 전체 문서 링크, 앱 runtime, Android/iOS simulator·실기기, WPT와 C11.4 전체 동작은 검증하지 않았다. 공식 checkbox와 `72/213`은 그대로 유지한다.

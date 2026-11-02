# PR #1073 merge 후 C11.1 percentage-gap Tailnet roadmap 확인 — 2026-11-03

문서 PR [#1073](https://github.com/ohah/spinon/pull/1073)은 2026-11-02T17:50:09Z Rebase and merge됐으며 merge SHA는 `227011db0df2cf403de779cde4e70e2a06551bb5`다. 이 source revision의 공개 roadmap과 C11 문서를 선택 발행하고 Tailnet 미리보기에서 확인했다. implementation PR #1072의 merge SHA는 `6508a1a91d04c7e83c00b10f230519d52e8ed86b`이다.

## 선택 빌드와 발행

RSPress `2.0.22`로 Git 추적 소스에서만 선택 root 81개 파일을 만들고 120개 route, 12,858,881 byte publication을 생성했다. 선택 root의 symlink는 거부했다. 카드의 내부 링크 8개와 필수 문서·화면 route를 manifest에 대조했다. 선택 build는 선택 밖 문서에 대한 전체 dead-link 검증을 수행하지 않는다.

전체 `bun run docs:build`는 `spec/internal/evidence/**` 여러 문서의 연결되지 않은 JSON·로그·이미지와 test fixture 경로 등 dead link를 보고해 exit 1이었다. 이번 변경의 route에서 발생한 오류라고 단정하지 않으며, 전체 사이트 build/link validation 성공으로 기록하지 않는다. 별도의 선택 RSPress build는 성공했다.

## Tailnet 갱신과 hash 확인

Tailscale backend는 `Running`, 기존 미리보기 서버는 PID `28040`으로 `127.0.0.1:8768`에서 수신 중이었고 갱신 전 로컬 및 Tailnet roadmap 응답은 HTTP 200이었다. 서버를 재시작하거나 종료하지 않았다.

사전 점검은 120 route 중 89개 생성, 13개 교체, 18개 동일을 예상했다. 교체 전 `roadmap.html`과 `STATUS.html`을 서버 PID가 열고 있음을 확인했다. 13개 이전 파일 약 6,354,307 byte를 다음 경로에 백업하고 같은 디렉터리 임시 파일 후 atomic rename으로 바꿨다. 이전 reader는 기존 inode를 계속 사용한다. route 삭제는 없었다.

갱신 후 roadmap, status, C11 계획, 계약 0059, 근거 문서 4개와 Android/iOS 화면을 포함한 10개 주요 route가 local HTTP와 Tailnet HTTPS에서 모두 200을 반환했다. 두 origin의 바이트 수와 SHA-256은 모두 publication manifest와 일치한다. 상세 결과는 [사전 점검](./preflight.json), [복사·백업](./copy-and-backup.json), [served hash 확인](./served-verification.json), [선택 root](./selected-root.json), [선택 publication](./selected-publication.json)에 있다.

## Agent Browser 확인

Agent Browser가 실제 Tailnet URL을 열어 확인했다. 데스크톱 1440×1000에서는 PR #1072 카드와 merge SHA, `72/213`이 보였고 문서 폭은 1440px이었다. 모바일 390×844에서는 카드가 x=14px, 폭 362px로 보였고 document/body 폭이 모두 390px이었다. 가로 넘침은 관찰되지 않았다. 별도 `/STATUS.html` 확인에서 공식 `72/213`이 보이고 C11 상위 체크박스는 disabled·unchecked였다.

- 화면: [데스크톱](./tailnet-roadmap-agent-browser-desktop.png), [모바일](./tailnet-roadmap-agent-browser-mobile.png)
- 측정: [데스크톱](./agent-browser-desktop.json), [모바일](./agent-browser-mobile.json), [상태 대장](./agent-browser-status-desktop.json)
- [Tailnet 발행 PR 변경 적대 검증 30개](./roadmap-pr-diff-review-30.md)

검증 뒤 `agent-browser close --all`을 실행했다. session 목록은 빈 배열이고 Agent Browser/Chrome 자식 프로세스 검색 결과도 비어 있었다. Tailnet preview PID `28040`은 계속 수신 중이다. [종료 확인](./browser-close.json).

화면은 문서 페이지의 responsive 표시 근거다. 앱 simulator 화면과 표면 pixel 검증 근거를 대체하지 않는다. C11.1/C11은 미완료이고 공식 완료 수 `72/213`은 그대로다. GitHub Pages 배포는 하지 않았다.

# PR #939 병합본 Tailnet 로드맵 동기화

## 동기화 기준

- 원격 `main`: `caf827f7652b698d7956ad7d6cc70b654ea23d20`
- 선택 빌드의 source tree와 원격 `main` tree: `6aa4e02f4c1b843a1b84f9e56402d0e61706ce20`
- 원시 로드맵: `spec/public/roadmap.html`, 459,410 bytes, SHA-256 `a10d5161bf634b32e706ab53a307b893fea9312bd6804537efb2f7038884d303`
- 공식 상태는 `72/213`; C11과 C11.4는 미완료로 유지한다.
- 미리보기: `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`

## 동기화와 경로 수정

첫 배포 사전검사에서 미리보기 서버가 Tailscale `Running`, 온라인 상태였고, PID `28040`의 기존 서버가 `127.0.0.1:8768`에서 응답했다. 선택된 58개 경로 중 기존 12개는 변경 전 백업 대상으로, 10개는 동일 hash, 36개는 신규 경로로 판정했다. 기존 CSS 파일 하나를 서버가 열고 있었지만 다음 hash와 같아 변경하지 않았다.

첫 복사 후 HTTP 경로 검증에서 새 디렉터리 아래 10개 파일의 부모 경로가 파일로 생성된 오류를 발견했다. 사전검사 코드가 대상 파일 경로가 아니라 처음 없던 부모 경로를 `targetPath`에 기록한 것이 원인이었다. 오류 파일 3개는 내용을 변경하거나 버리지 않고 다음 백업 위치로 이동해 hash를 보존했다.

`/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr938-2026-11-01/pass01/misplaced-route-parent-files-pass01/`

그 뒤 잘못 놓인 10개 경로를 선택 publication 원본에서 다시 만들어 각 파일의 byte 수와 SHA-256을 확인했다. 이전 미리보기 파일 12개와 잘못 놓였던 3개 파일을 합해 백업 15개도 원본 hash로 확인했다. 기존 경로 삭제는 없었다.

## 최종 검증

- 선택된 58개 경로 전체가 로컬 HTTP와 Tailnet HTTPS에서 각각 200을 반환했고, 각 응답의 byte 수와 SHA-256이 publication manifest와 일치했다.
- 로드맵은 200, 459,410 bytes, 위 SHA-256으로 제공됐다. 본문에 C11 미완료와 `72/213`이 보존됐다.
- 로드맵 카드의 Android AVD와 iOS Simulator 링크도 각각 200 및 기록된 PNG hash와 일치했다.
- Agent Browser로 실제 Tailnet 페이지를 열어 C11.4 음수 `text-indent` 카드, PR #938 요약, 범위 한계와 공식 집계를 화면에서 확인했다. 화면은 [tailnet-roadmap-agent-browser-pass01.png](tailnet-roadmap-agent-browser-pass01.png)에 있다.
- 사용 후 Agent Browser 세션을 닫았고, 프로세스 목록에서 Agent Browser 프로세스가 남지 않은 것을 확인했다. Tailnet 미리보기 서버는 재시작하지 않았다.

`preflight-tailnet-pass01.json`과 `copy-and-backup-tailnet-pass01.json`은 첫 시도의 경로 오류를 포함한 원본 기록이다. 최종 경로와 응답의 판정에는 `tailnet-sync-route-repair-pass01.json` 및 `served-verification-tailnet-pass01.json`을 사용한다.

## 한계

선택 빌드는 저장소 고정 Rspress `2.0.22`로 수행됐고, 전체 문서의 dead-link 검사는 켜지 않았다. 별도 전체 빌드 로그에는 기존 상대 링크를 포함한 1,171개 페이지 수준 dead-link 오류가 기록되어 있다. 이 동기화는 문서 미리보기의 경로·응답·hash를 확인한 것이며, 전체 문서 빌드 성공이나 C11 완료를 뜻하지 않는다.

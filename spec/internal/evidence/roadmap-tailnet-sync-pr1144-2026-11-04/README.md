# PR #1144 병합 후 Tailnet 로드맵 동기화

## 기준과 공식 상태

- Source는 `origin/main`의 `a2e90a84fabcb73f8836eeb412df1a9ac86755fc`이며 tree는 `809e0510b9f142facbb372fecc53d7396597a7c6`이다. tracked diff hash는 빈 diff의 SHA-256 `e3b0c442…b855`다.
- PR #1142 구현, PR #1143 병합 후 상태 문서, PR #1144 공개 로드맵 카드를 모두 Rebase and merge했다. #1144 merge SHA는 `a2e90a84fabcb73f8836eeb412df1a9ac86755fc`다.
- 공식 완료 수는 `72/213`이며 C11/C11.1은 미완료다. 최근 한 축 placement PR은 제한 slice라 완료 항목 수를 바꾸지 않는다.

## 선택 빌드와 publication

- 고정 RSPress `2.0.22`로 선택 root를 빌드했다. 91개 selected-root source, 173개 publication route, 17,825,354 bytes를 준비했다.
- 새 카드 marker는 PR #1142 Rebase and merge, merge SHA `0f5506423a1150c80e5684fd087b948923fdbcb7`, 공식 `72/213`을 확인했다. 10개 내부 route와 렌더된 `#c11-css-grid` 앵커를 검사했다.
- 전체 문서 링크 그래프 검사로 확대 해석하지 않는다. 이번 카드 링크와 지정한 publication route만 확인했다.

## 서버 사전 점검과 교체

- Tailscale backend는 `Running`; 미리보기는 PID `28040`, `127.0.0.1:8768`에서 응답했고 로드맵은 HTTP 200이었다.
- 사전 비교는 173 route 중 create 119, replace 26, same 28이었다. 변경 경로의 교체 전 해시와 열린 파일 상태를 검사했다.
- 변경 경로 중 두 파일에만 preview 서버 PID 28040의 읽기 전용 핸들이 있었다: `internal/0059-c11-grid-layout.html`과 `STATUS.html`. 각 핸들은 해당 기존 inode의 일반 파일을 `r` 모드로 열고 있었다. 다른 프로세스나 쓰기 핸들은 없었다.
- 기존 파일은 전용 백업 경로에 복사한 뒤 임시 파일을 같은 디렉터리에서 원자 rename했다. 열린 이전 inode는 기존 읽기를 위해 보존하고, 후속 요청은 새 경로를 읽는다. PID 28040을 재시작하지 않았다.
- 복사 결과: create 119, replace 26, same 28. 교체본 26개를 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1144-2026-11-04/pass01`에 보존했다. 파일 삭제는 없었다.

세부 자료: [selected root](selected-root.json) · [publication route/hash manifest](selected-publication.json) · [사전 점검](preflight-selected-pass01.json) · [복사 및 백업](copy-and-backup-selected-pass01.json) · [로컬·HTTPS 검증](served-verification-selected-pass01.json).

## Agent Browser 화면 확인

- Desktop 1280×1000: PR #1142 카드 1개와 local links 10개, `clientWidth=scrollWidth=1280`을 확인했다.
- Mobile 390×844: 같은 카드가 보이고 `clientWidth=scrollWidth=390`이었다. 인접한 “여섯 개의 검증 관문” SVG는 좁은 화면에서 오른쪽이 잘려 보인다. 이 표시 한계는 이번 카드나 C11 실행 범위와 별개로 캡처에 기록했다.
- `STATUS.html#c11-css-grid`가 C11 항목으로 이동하고, 문서 PR #1144의 30행 검토 페이지도 열리는 것을 확인했다.
- 화면: [desktop](tailnet-roadmap-desktop-agent-browser.png) · [mobile](tailnet-roadmap-mobile-agent-browser.png).
- Agent Browser는 확인 후 닫았다. preview server는 계속 실행 중이다.

이 동기화는 C11 완료나 전체 문서 링크 검증을 뜻하지 않는다. 공식 완료 수 `72/213`과 C11 미완료 상태는 그대로다.

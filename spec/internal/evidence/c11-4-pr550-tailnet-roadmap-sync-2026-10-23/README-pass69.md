# C11.4 PR #550 Tailnet 문서 동기화 · pass69

PR #550의 문서 정정이 main `82f7514dc51bedd4160f27180145a13bc3df8750`에 병합된 뒤 선택한 문서 루트를 RSPress 2.0.22로 빌드하고 Tailnet 미리보기를 갱신했다. 공식 C11.4 상태 checkbox는 그대로 미완료이며 로드맵 집계는 72/213이다.

## 선택 루트 빌드

- 고정된 저장소 바이너리 `packages/docs/node_modules/.bin/rspress`로 선택 루트를 빌드했다. 종료 코드는 0이고 생성물은 1,255개 / 253,750,035 bytes다. [빌드 요약](./selected-build-summary-pass69.json).
- 전체 저장소의 dirty checkout은 기존 비추적 evidence의 링크 문제로 전체 dead-link 빌드가 통과하지 않았다. 이번 선택 빌드는 구문 변환과 페이지 생성을 확인했으며 RSPress `checkDeadLinks`·`checkDeadImages`를 껐다. 따라서 전체 문서 dead-link 검증으로 확대 해석하지 않는다.
- 공개 `roadmap.html`은 raw HTML 입력이라 RSPress 결과에 포함되지 않는다. main의 `spec/public/roadmap.html`과 Tailnet 파일은 사전 조사 때 이미 267,555 bytes / SHA-256 `d01d12f91617c559772ec61f0fd934994dba22e80f7534d825e483b6ee296f09`로 같았다. Tailnet 파일을 다시 쓰거나 백업하지 않았고, 교차 경로 HTTP 검증용 산출물 디렉터리에만 같은 source를 추가했다. [roadmap 일치 기록](./roadmap-preflight-pass69.json) · [동일성 확인](./roadmap-remote-verification-pass69.json).

## Tailnet 동기화 및 HTTP 검증

- 서버가 제공하는 `/Users/yoonhb/Library/Application Support/SpinonPreview`에서 동기화 전 58,774개 / 2,489,977,392 bytes를 조사했다. 생성된 1,255개 경로와 비교해 동일한 84개는 건너뛰고 새 파일 603개와 바뀐 파일 568개를 반영했다. 568개 이전 버전은 `SpinonPreviewBackups/c11-4-pr550-tailnet-roadmap-sync-2026-10-23/pass69/`에 보존했다.
- 교체는 같은 볼륨 hard link 백업 후 임시 형제 파일을 atomic rename하는 방식으로 적용했다. Tailnet 파일 삭제는 없으며 기존 대상에만 있던 파일도 보존했다. 동기화 뒤 대상은 59,377개다. [사전 검사](./preflight-pass69.json) · [변경 경로와 SHA-256](./delta-manifest-pass69.json) · [복사·백업 결과](./copy-and-backup-pass69.json).
- 변경된 로컬 파일 1,171개와 이전 버전 백업을 모두 해시 검증했다. HTTPS에서 roadmap, STATUS, conformance, 내부 README·0059, C11.4 계획 및 근거 페이지 9개와 이 페이지들이 참조하는 정적 자산 18개를 확인했다. 모두 HTTP 200이며 main 빌드 source와 SHA-256이 일치한다. [원격 검증](./remote-verification-pass69.json).

## Agent Browser 화면 검증

- Agent Browser `spinon-tailnet-pr550-pass69`에서 Tailnet 로드맵 DOM을 확인했다. 표시 checkbox 213개 중 72개 checked이고 전부 disabled다. 본문에 72/213이 있고 77/213은 없으며, PR #549 결과와 C11.4 미완료 문구가 표시된다. [로드맵·공식 상태 DOM과 screenshot hash](./agent-browser-verification-pass69.json).
- STATUS route도 열어 링크 fragment `css-구현-체크리스트`가 실제 제목에 연결되는 것을 확인했다. STATUS 페이지의 원시 checkbox는 226개 중 87개 checked다. 이는 별도 source 범위이며 공식 roadmap의 213개 task counter와 혼합하지 않는다.
- PR #549 카드를 포함한 1440×1200 Agent Browser 화면은 [로드맵 screenshot](./tailnet-roadmap-c114-pr549-agent-browser-pass69.png)이다. 보조 화면은 [로드맵 상단](./tailnet-roadmap-agent-browser-pass69.png)과 [STATUS anchor](./tailnet-status-agent-browser-pass69.png)다.
- 확인 뒤 Agent Browser 창과 세션 데몬을 닫았다. session info는 `active: false`, `pid: null`, 세션 목록과 전용 프로세스는 0개다. Tailnet preview server PID 1191은 유지했다. [브라우저 종료 기록](./browser-close-pass69.json).

## 범위와 남은 검증

이번 변경은 문서 미리보기 동기화다. 앱·Android/iOS Simulator·실기기·WPT 동작 검증이 아니다. C11.4 공식 완료 checkbox는 미완료이고 집계는 72/213으로 유지한다.

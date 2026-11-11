# PR #1363 이후 Tailnet 로드맵 동기화

## 요약

PR #1363 병합 뒤 공식 계획과 읽기 전용 Tailnet 로드맵을 대조했다. 데스크톱에서 칸반 제목이 flex 축소로 눌리고 긴 요약 문자열이 한 줄에 붙는 문제를 발견해 `spec/public/roadmap.html`을 수정했다. 이 변경은 문서 화면만 다루며 구현 상태나 공식 완료 수는 바꾸지 않는다.

- 계획 PR #1363 merge commit: `910abd5c36ee2e7030bcb1200a71160e891c7b3d`
- 화면 수정 commit: `cc944e4e3ff64c78298a5330f8cdb02234376710`
- 변경: `.section-heading h2`에 `flex:none`, 요약 `span`에 `min-width:0`, 줄 높이와 `overflow-wrap:anywhere` 적용
- 현재 공식 상태: `78/214`; C11.4와 C11은 미완료
- 다음 bounded 작업: C11.4 six-fit repeated-text row-span `min-height:auto`; 계획과 Chrome 사전 비교가 끝났고 구현은 시작 전

## 빌드 범위

Rspress 2.0.22로 로드맵과 연결된 문서만 선택한 root에서 빌드했다. 선택 빌드에는 5개 페이지와 연결 자산 11개가 포함된다. 전체 문서 사이트 빌드나 전체 dead-link 검사를 통과했다고 주장하지 않는다. 전체 빌드는 이 변경과 무관한 기존 링크·코드 펜스·자산 문제 때문에 실패했고, 선택 빌드에서는 그 전체 검사를 비활성화했다.

선택 빌드 입력, 설정, 출력 검증은 같은 디렉터리의 `prepare-selected-root.mjs`, `rspress.config.ts`, `selected-root.json`, `selected-site-verification.json`에 기록했다.

## Tailnet 반영

사용자가 선택한 미리보기 서버 PID `90711`과 기존 preview root를 유지했고 서버를 재시작하지 않았다. 2차 복사에서 선택된 36개 경로 중 로드맵 HTML 1개를 교체하고 35개는 그대로 두었다. 교체 전 HTML 한 개를 `backup/roadmap-tailnet-sync-pr1363-c114-sixfit-plan-2026-11-12-pass02/roadmap.html`로 보존하고 사전 해시도 확인했다. 다른 미리보기 파일과 기존 백업은 건드리지 않았다.

`tailnet-copy-manifest-pass02.json`과 `served-verification-pass02.json`은 source/preview/local HTTP/Tailnet HTTP 해시, 36개 경로의 HTTP 200, 백업 검증을 기록한다. 전체 사이트 동기화 증거가 아니다.

## Agent Browser 확인

URL: <https://macstudio.tailed42f2.ts.net/spinon/roadmap.html>

- 데스크톱 1440×1000: document/body 너비 1440px, 제목 171.8125×33.59375px, 요약 1068.1875×70.375px, C11.4 후보 표시
- 모바일 390×844: document/body 너비 390px, C11.4 후보 표시
- 두 viewport 모두 가로 overflow가 없었다.
- 최종 화면: `tailnet-roadmap-agent-browser-desktop.png`, `tailnet-roadmap-agent-browser-mobile.png`
- 수정 전 발견 화면도 `*-before-fix.png`로 보존했다.
- 재확인 세부 값은 `agent-browser-verification-final.json`에 있다. 검증 후 Agent Browser 세션을 닫았고 활성 세션과 Agent Browser/Chrome 프로세스가 남지 않은 것을 확인했다.

## 정리와 한계

사용하지 않는 빈 구현 브랜치 `feat/c114-six-fit-row-span-auto-minimum-20261112`는 main과 같은 커밋을 가리키고 별도 worktree가 없어 제거했다. 선택 빌드 출력은 Tailnet 검증과 PR 생성에 사용한 뒤 제거한다. 과거 증거, 미리보기 백업, 다른 미추적 파일은 삭제하지 않았다.

PR check/workflow가 0개이므로 CI 통과로 기록하지 않는다. 이 문서 변경은 C11.4 구현 증거가 아니며 공식 상태를 승격하지 않는다.

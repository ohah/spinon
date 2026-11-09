# C11.4 PR #1312 병합 후 Tailnet 로드맵 동기화

PR [#1312](https://github.com/ohah/spinon/pull/1312)의 계획과 Chrome 사전 비교를 병합한 뒤 공식 상태 페이지와 사용자 지정 Tailnet 미리보기를 동기화했다. 제품 코드는 변경하지 않았다.

## 병합과 공식 상태

- PR #1312는 2026-11-09T16:14:51Z에 Rebase and merge됐고 merge SHA는 `6439049c16cc107682292dd7c4d6c64dc4c1b413`이다. GitHub check rollup은 비어 있어 CI green으로 기록하지 않는다.
- 게시 원본은 이 merge commit의 `spec/STATUS.md`와 `spec/public/roadmap.html`이다. 다음 계획은 두 `fit-content()` track을 포함한 auto-placement slice이며 구현은 아직 시작하지 않았다.
- 공식 완료 수 `78/214`, C11.4/C11 미완료, 내부 계약 `0.1.0`을 유지한다.

## 빌드와 게시

- Rspress 2.0.22로 `STATUS.html`, 계획·검토 문서, Chrome 사전 비교 문서를 포함한 selected-root를 빌드했다. selected-root의 dead-link 검사는 끈 상태로 렌더링했다. 전체 docs build 성공으로 주장하지 않는다. 기존 전체 빌드는 역사적 evidence의 dead links 때문에 실패한 상태였다.
- 선택된 원본 23개, 8,609,180 bytes의 hash manifest는 [staged-root-manifest.json](./staged-root-manifest.json)에 있다. 게시 대상 전체 53 route, 16,625,924 bytes 중 29개 추가, 11개 교체, 13개 동일이었다. 교체 전 11개는 별도 backup root에 SHA 검증 후 보존했다.
- Tailnet은 `Running`, preview server PID `28040`을 유지했다. 서버를 재시작하거나 파일을 삭제하지 않았다. 새 roadmap은 연결된 페이지를 먼저 게시한 뒤 마지막에 교체했다.
- [게시 전 검증](./publication-preflight.json), [복사·백업 결과](./publication-result.json), [실제 제공본 검증](./served-verification.json)에 경로별 해시를 기록했다. 로컬 preview와 Tailnet 모두 53/53 route가 HTTP 200 및 기대 SHA-256과 일치했다.

## Agent Browser 화면 검증

- Tailnet roadmap desktop `1440×1000`: 새 계획 카드, PR #1311 병합 기록, `78/214`가 보였고 문서 폭은 `1440px`, 깨진 이미지는 없었다. [화면](./tailnet-roadmap-agent-browser-desktop.png)
- Tailnet roadmap mobile `390×844`: 동일한 상태 표시를 확인했다. 문서 전체 폭은 `390px`이며 칸반만 내부 가로 스크롤을 사용한다. [화면](./tailnet-roadmap-agent-browser-mobile.png)
- 새 계획 페이지는 Tailnet에서 열리고 `구현 전` 상태를 유지했다. [화면](./tailnet-plan-agent-browser.png)
- 사전 비교 README 페이지의 Chrome sample image는 `1280×900`으로 로드됐고 깨진 이미지가 없었다.
- 구조화 결과는 [Agent Browser 검증](./agent-browser-verification.json)에 있다.

## 프로세스와 산출물 정리

- 이번 검증에서 사용한 Agent Browser session 세 개를 각각 닫았고 고유 session 이름으로 실행 중인 프로세스가 남지 않은 것을 확인했다. Terminal Browser는 사용하지 않았다.
- iOS/Android simulator는 이번 Tailnet 확인에서 시작하지 않았다. 종료 확인 시점에 부팅된 iOS simulator가 없었으며 S03/Pilot 이름의 simulator profile도 찾지 못했다. 공유 simulator profile은 삭제하지 않았다.
- 게시가 끝난 selected-root build와 일회용 Rspress config 디렉터리는 더 이상 사용하지 않아 제거했다. Tailnet server는 계속 실행 중이다.

## 검토

[병합 후 게시 실패 관점 20개](./postmerge-review-20.md)와 [이 evidence PR diff 실패 관점 20개](./evidence-pr-diff-review-20.md)를 마쳤다. 검토는 해시·백업, 링크 대상, 화면 상태, 반응형 폭, 빌드 범위와 정리 경계를 확인한다.

# Tailnet 동기화 실패 관점 검토 20개

검토 범위는 PR #349 뒤 선택 Rspress 빌드와 정적 Tailnet 파일 동기화다. 앱 구현이나 C11.4 기능 완료 판정은 다루지 않는다.

| # | 실패 가능성 | 확인 결과 |
|---:|---|---|
| 1 | 기준 원본이 PR #349 병합 전 상태임 | source revision은 PR #349 merge commit `6867c2af5de4c3a953c2e846160334fd6d9590ee`를 기준으로 고정했다. |
| 2 | 주 체크아웃의 변경이 문서 브랜치에 섞임 | C11.3 주 체크아웃은 유지했고 문서 작업은 병합 커밋 기반 별도 worktree에서 진행했다. |
| 3 | roadmap이 이전 1-scene 결과를 계속 표시함 | Agent Browser 화면에서 10-scene 요약, PR #349 링크, 미완료 범위를 확인했다. |
| 4 | 측정 결과가 trace byte 단위 exact로 과장됨 | roadmap은 의미상 10/10, 원본 byte 비교 8/10을 구분한다. |
| 5 | PixelCopy·Metal 표본을 전체 화면 일치로 확대함 | 문구는 고정 overflow scene의 표본 각 3/3으로 제한한다. |
| 6 | fixture 화면을 glyph paint 결과로 오해함 | roadmap·계획·근거 페이지에 glyph가 없는 box geometry 캡처라고 적었다. |
| 7 | 시뮬레이터 검증이 실기기 검증으로 읽힘 | 이번 작업이 정적 문서 동기화임을 명시하고 앱은 AVD·Simulator 기존 근거로 분리한다. |
| 8 | C11.4 체크나 공식 완료 수가 바뀜 | 상태 대장의 C11.4 checkbox는 미완료로 유지했고 완료 집계를 수정하지 않았다. |
| 9 | 계획 링크가 실제 Rspress route와 다름 | Tailnet 계획 페이지에서 10-scene 후속 문단과 새 근거 링크를 확인했다. |
| 10 | 상태 대장 URL에 저장소 경로 `spec/`가 남음 | 선택 빌드의 route에 맞춰 `/spinon/STATUS.html`로 고쳤다. |
| 11 | 새 evidence README가 preview에 빠짐 | 선택 사이트에 C11.4 text intrinsic README와 두 review route를 포함했다. |
| 12 | README의 JSON·로그·PNG 링크가 빌드 결과에 누락됨 | 별도 정적 첨부 복사와 HTTP 제공 확인에서 각 경로의 status·byte·SHA-256을 대조했다. |
| 13 | precomparison fixture 링크가 Tailnet에서 404임 | 브라우저에서 최초 404를 재현해 GitHub 저장소 파일 링크로 바꾸고 수정 링크를 재확인했다. |
| 14 | Rspress 버전이 프로젝트 pin과 달라짐 | `@rspress/core` 2.0.22를 사용하고 프로젝트 pin은 변경하지 않았다. |
| 15 | 선택 빌드를 전체 사이트 검증으로 오해함 | 전체 문서 build와 GitHub Pages 배포를 하지 않았다고 README에 남겼다. |
| 16 | dead-link/image 검사가 꺼진 사실이 감춰짐 | 선택 config에서 검사를 끈 점과 실제 브라우저·대상 파일 확인을 별도로 기록했다. |
| 17 | preview 경로가 `/spinon/` prefix와 어긋남 | roadmap, status, 계획, evidence page를 Agent Browser에서 실제 Tailnet URL로 열었다. |
| 18 | 기존 preview 파일을 덮고 백업하지 않음 | preflight에서 교체 경로를 식별하고 각 이전 파일을 별도 pass backup에 저장했다. |
| 19 | 복사 중 파일이 바뀌거나 preview 파일을 삭제함 | source·target SHA-256과 byte 수를 확인하고 동기화는 추가·교체만 수행했다. |
| 20 | 사용 후 Agent Browser나 기존 preview listener가 종료됨 | 브라우저 확인 뒤 Agent Browser만 닫고 기존 Tailnet listener·Tailscale 경로는 유지했다. |

판정: C11.4 진행 상태를 최신 근거에 맞춰 표시하는 선택 정적 미리보기 동기화다. 전체 문서 사이트, GitHub Pages, 앱 경로 또는 전체 C11.4 적합성 판정은 아니다.

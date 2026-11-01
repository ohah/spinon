# C11.4 hyphens:auto Tailnet 로드맵 미리보기 동기화

2026-11-01에 `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`과 연결된 문서를 갱신했다. 소스 변경은 C11.4 제한 `hyphens:auto` PR #982의 현재 병합 상태와 한계를 표시하며 C11.4/C11 공식 완료 수 `72/213`은 유지한다.

## 빌드 범위

- 기준 HEAD: `b53e5f51b86818eaa946e6c9d9a39ea02190e4b6` (PR #983 병합 후 `origin/main`). 선택 문서 overlay의 SHA-256: `5e643605cf82ba0c732cf36a613d4dc1f5a95a183c8fa9efe21913df2479f01f`.
- 전체 문서 빌드는 기존 `spec/internal/evidence` 자료의 dead link 오류로 실패했다. 예를 들어 예전 evidence의 `.log`, `.json`, 이미지 링크가 RSPress HTML 문서로 해석됐다. 관련 없는 문서를 일괄 수정하지 않았다.
- 선택 문서 root에 대해 RSPress 2.0.22 빌드는 통과했다. 이 경로는 기존 선택 root 293개 파일에 현재 C11.4 hyphens 계획·계약·상태·근거를 overlay하고 dead-link 검사와 dead-image 검사는 끈 채 렌더링했다. 따라서 전체 사이트 dead-link 통과로 해석하지 않는다.
- 로드맵 최신 두 카드는 로컬 route 존재 여부와 `PR #982`, 병합 SHA, `72/213`, PR #977의 역사적 범위 표기를 대조했다.

## Tailnet 동기화

- Tailscale backend는 `Running`이었다. 기존 미리보기 서버 PID `28040`과 `127.0.0.1:8768` listener를 유지했다.
- 사전 점검은 931개 선택 route 중 501개(21,120,776 bytes)가 신규 또는 변경 대상이라고 계산했다. 변경할 route의 open handle은 0개였다.
- 원자적 파일 교체로 463개 route를 추가하고 38개를 바꿨으며 430개는 같아서 유지했다. 기존 파일 38개의 백업은 `/Users/yoonhb/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr982-2026-11-01/pass02`에 저장하고 hash를 검증했다. 경로 삭제나 preview server 재시작은 없었다.
- 동기화 뒤 선택 route 931/931의 로컬 파일 hash가 publication과 일치했다. 로드맵, 상태 대장, 계획, 내부 계약, PR #977/#982 실행 근거·검토 페이지와 핵심 표본 파일 등 필수 20 route는 Tailnet HTTPS와 local HTTP에서 모두 200 및 정확한 SHA-256으로 확인했다. 백업 38/38도 원본 hash와 일치했다.
- 검증된 route-set SHA-256: `554a7552556f195f75a40b4c698bb1cbb5c4316d7b435dff27e4d3fb260b15fb`.

## Agent Browser 확인

Agent Browser에서 Tailnet 로드맵, 공식 상태, PR #982 실행 근거 페이지를 열어 `PR #982 Rebase and merge`, `PR #977`의 역사적 설명, 공식 `72/213`을 확인했다. 문서 캡처이며 앱 동작 화면이나 glyph/pixel 근거가 아니다.

- [Desktop 화면 캡처 (1280 × 720)](tailnet-roadmap-agent-browser-desktop.png)
- [Mobile 화면 캡처 (390 × 844)](tailnet-roadmap-agent-browser-mobile.png)

Mobile viewport에서 `documentElement.scrollWidth`와 `body.scrollWidth`는 모두 390px이었다. 상단 내비게이션은 자체 가로 스크롤 영역(`scrollWidth` 479px, `clientWidth` 362px)을 사용해 페이지 전체의 가로 넘침은 없었다. 확인 뒤 Agent Browser 세션을 닫았고 남은 browser process와 active session이 없는 것을 재확인했다.
